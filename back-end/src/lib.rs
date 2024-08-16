pub mod db;
pub mod routes;
pub mod stock_lib;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;

use core::fmt;
use core::any::Any;
use pyo3::PyErr;
use rocket::tokio::task::JoinError;

/// 错误处理
#[derive(Debug)]
pub enum AppErrorEnum {
    PythonError(PyErr),
    DieselError(diesel::result::Error),
    IoError(std::io::Error),
    ThreadErr(Box<dyn Any + Send + 'static>),
    JoinErr(JoinError)
    // 可以扩展其他错误类型
}

impl fmt::Display for AppErrorEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppErrorEnum::PythonError(err) => write!(f, "Python error: {}", err),
            AppErrorEnum::DieselError(err) => write!(f, "Diesel error: {}", err),
            AppErrorEnum::IoError(err) => write!(f, "IO error: {}", err),
            AppErrorEnum::ThreadErr(err) => write!(f, "Thread execute error: {:?}", err),
            AppErrorEnum::JoinErr(err) => write!(f, "Thread execute error: {:?}", err),
            // 可以扩展其他错误类型的显示方式
        }
    }
}

impl From<PyErr> for AppErrorEnum {
    fn from(error: PyErr) -> Self {
        AppErrorEnum::PythonError(error)
    }
}

impl From<diesel::result::Error> for AppErrorEnum {
    fn from(error: diesel::result::Error) -> Self {
        AppErrorEnum::DieselError(error)
    }
}

impl From<std::io::Error> for AppErrorEnum {
    fn from(error: std::io::Error) -> Self {
        AppErrorEnum::IoError(error)
    }
}

impl From<Box<dyn Any + Send + 'static>> for AppErrorEnum {
    fn from(error: Box<dyn Any + Send + 'static>) -> Self {
        AppErrorEnum::ThreadErr(error)
    }
}

impl From<JoinError> for AppErrorEnum {
    fn from(error: JoinError) -> Self {
        AppErrorEnum::JoinErr(error)
    }
}
