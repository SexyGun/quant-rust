use rocket_db_pools::Database; // 导入 Rocket 数据库池的 Connection 和 Database 类型。
use rocket_db_pools::diesel::MysqlPool; // 导入 AsyncConnection 和 MysqlPool，用于与 MySQL 数据库异步交互。

#[derive(Database)] // 使用 Rocket 的 Database 派生宏为 Db 类型提供数据库连接功能，此时 rocket 会根据 toml 中的数据库地址连接数据库
#[database("diesel_mysql")] // 指定 Rocket 配置文件中数据库池的名称为 diesel_mysql。
pub struct Db(MysqlPool); // 定义一个名为 Db 的结构体，其中包含一个 MysqlPool 类型的字段，用于数据库连接池。