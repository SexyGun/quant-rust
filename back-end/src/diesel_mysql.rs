// extern crate diesel; // 这行代码声明 diesel 作为外部 crate，虽然在新版 Rust 中通常不再需要显式声明。
use diesel::prelude::*;
// 导入 Diesel 的预定义功能，例如模型定义、数据库操作等。
use rocket::fairing::AdHoc; // 导入 Rocket 的 AdHoc 类型，用于动态附加配置或功能
use rocket::response::status::Created; // 导入 Rocket 的 Created 响应类型，用于返回 HTTP 201 Created 状态。
use rocket::response::Debug; // 导入 Rocket 的 Debug 类型，用于调试错误响应。
use rocket::serde::{json::Json, Deserialize, Serialize};
// 导入 Rocket 的 Json 类型以及 Deserialize 和 Serialize 特性，用于处理 JSON 数据。
use rocket_db_pools::diesel::prelude::RunQueryDsl; // 导入 RunQueryDsl，用于执行 Diesel 的数据库查询。
use rocket_db_pools::diesel::{AsyncConnection, MysqlPool}; // 导入 AsyncConnection 和 MysqlPool，用于与 MySQL 数据库异步交互。
use rocket_db_pools::{Connection, Database}; // 导入 Rocket 数据库池的 Connection 和 Database 类型。

// 定义一个通用的 Result 类型，默认错误类型为 Debug<diesel::result::Error>，用于处理数据库操作中的错误。
type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[derive(Database)] // 使用 Rocket 的 Database 派生宏为 Db 类型提供数据库连接功能
#[database("diesel_mysql")] // 指定 Rocket 配置文件中数据库池的名称为 diesel_mysql。
struct Db(MysqlPool); // 定义一个名为 Db 的结构体，其中包含一个 MysqlPool 类型的字段，用于数据库连接池。

// 派生 Debug、Clone、Deserialize、Serialize、Queryable 和 Insertable 特性，允许 Post 结构体进行调试、克隆、序列化、反序列化，以及与 Diesel 的数据库交互。
#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")] // 指定 serde 使用 Rocket 自带的 serde 库，而不是默认的 serde。
#[diesel(table_name=posts)] // 指定 Diesel 中表的名称为 posts
pub struct Post {
    // 定义一个 Post 结构体，用于表示数据库中的 posts 表。
    // 包含字段 id、title、text 和 published。
    #[serde(skip_deserializing)] // 使得 id 和 published 字段在反序列化时被跳过（即不从请求体中读取这些字段）。
    id: Option<i64>,
    title: String,
    text: String,
    #[serde(skip_deserializing)]
    published: bool,
}

// 使用 Diesel 的宏定义数据库表结构。这里定义了 posts 表及其列。
diesel::table! {
    posts(id) {
        id -> Nullable<BigInt>,
        title -> Text,
        text -> Text,
        published -> Bool,
    }
}

// 定义一个 POST 路由处理函数，处理路径为根目录 / 的请求，数据类型为 Json<Post>。
#[post("/", data = "<post>")]
/// 定义 create 异步函数，接受数据库连接和 JSON 格式的 Post 数据，
/// 返回一个 Result 类型，其中成功时返回 Created<Json<Post>>，失败时返回 Debug<diesel::result::Error>。
async fn create(mut db: Connection<Db>, mut post: Json<Post>) -> Result<Created<Json<Post>>> {
    // 定义一个 SQL 函数 last_insert_id，用于获取刚插入记录的 ID。
    // last_insert_id 在数据库中有默认的同名函数
    // 这里可以直接进行匹配
    // 但实际上，last_insert_id 可以由我们自定义，前提是要保证你在数据库中也定义了相同的函数名
    diesel::sql_function!(fn last_insert_id() -> BigInt);
    // let post = db.transaction(|mut conn| { ... }).await?; 
    // 开始一个数据库事务，在事务中插入数据并获取插入记录的 ID。
    // Box::pin 用于将异步闭包转为 Pin<Box>，以便在异步环境中使用
    let post = db
        .transaction(|mut conn| {
            Box::pin(async move {
                // 执行插入操作，将 post 数据插入到 posts 表中
                diesel::insert_into(posts::table)
                    .values(&*post)
                    .execute(&mut conn)
                    .await?;
                // 使用 last_insert_id 函数获取刚插入记录的 ID，并将其设置到 post 对象的 id 字段中。
                post.id = Some(
                    posts::table
                        .select(last_insert_id())
                        .first(&mut conn)
                        .await?,
                );
                // 回 HTTP 201 Created 状态码，并将插入的 post 对象作为响应体。
                Ok::<_, diesel::result::Error>(post)
            })
        })
        .await?;

    Ok(Created::new("/").body(post))
}

#[get("/")] 
async fn list(mut db: Connection<Db>) -> Result<Json<Vec<Option<i64>>>> {
    // .load(&mut db) 是一个 Diesel 的方法，用于执行构建的查询并从数据库中加载数据。这里的 db 是一个数据库连接对象，它被用作查询的执行者。
    // &mut db 表示 db 是一个可变引用，因为 Diesel 的一些操作需要对连接进行修改（例如事务处理）。
    let ids = posts::table
        .select(posts::id)
        .load(&mut db)
        .await?;
    Ok(Json(ids))
}

#[get("/byId?<id>")]
async fn read(mut db: Connection<Db>, id: i64) -> Option<Json<Post>> {
    // .first(&mut db) 是 Diesel 的方法，用于从查询结果中获取第一条记录。
    let result = posts::table
        .filter(posts::id.eq(id))
        .first(&mut db)
        .await
        .map(Json)
        .ok();
    println!("{:?}", result);
    result
}
#[get("/delete?<id>")]
async fn delete(mut db: Connection<Db>, id: i64) -> Result<Option<()>> {
    let affected = diesel::delete(posts::table)
        .filter(posts::id.eq(id))
        .execute(&mut db)
        .await?;
    // affected 是删除操作影响的行数。如果 affected == 1，则说明成功删除了一条记录
    // then_some(()) 将 true 转换为 Some(())，将 false 转换为 None。
	// 最终返回 Ok(Some(())) 表示删除成功，返回 Ok(None) 表示没有找到要删除的记录（没有记录被删除）。
    Ok((affected == 1).then_some(()))
}

#[get("/delete/all")]
async fn destroy(mut db: Connection<Db>) -> Result<()>{
    diesel::delete(posts::table).execute(&mut db).await?;
    Ok(())
}

pub fn stage() -> AdHoc {
    // AdHoc::on_ignite 是 Rocket 提供的一种机制，用于在 Rocket 启动时执行自定义的初始化代码。这个方法接受两个参数：
	// 名称：一个字符串，用于标识这个阶段的名称，通常用于日志或调试信息。
	// 初始化闭包：一个异步闭包（async {}），用于执行初始化代码。
    AdHoc::on_ignite("Diesel MySQL Stage", |rocket|async {
        rocket.attach(Db::init())
            .mount("/diesel-async", routes![create, list, read, delete, destroy])
    })
}
