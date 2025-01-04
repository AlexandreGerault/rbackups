use std::sync::Arc;
use mysql::Conn;
use mysql::prelude::Queryable;

// pub enum SourceDriver {
//     Mysql {
//         host: String,
//         port: u16,
//         username: String,
//         password: String,
//         database: String,
//     },
//     Postgres {
//         host: String,
//         port: u16,
//         username: String,
//         password: String,
//         database: String,
//     },
//     Sqlite {
//         host: String,
//         port: u16,
//         username: String,
//         password: String,
//         database: String,
//     }
// }
//
pub struct Source {
    id: String,
    driver: Arc<dyn SourceDumper>,
}

pub struct Mysql {
    connection: Conn,
    database: String,
}

impl Mysql {
    pub fn new(host: &str, port: u16, username: &str, password: &str, database: &str) -> Mysql {
        let url = format!("mysql://{}:{}@{}:{}/{}", username, password, database, host, port);
        let connection = Conn::new(url.as_str()).unwrap();

        Mysql {
            connection,
            database: database.to_owned(),
        }
    }
}

impl Source {
    pub fn new(id: &str, driver: Arc<dyn SourceDumper>) -> Source {
        Source { id: id.to_string(), driver }
    }

    pub fn dump(&mut self) -> String {
        self.driver.dump()
    }
}

pub trait SourceDumper {
    fn dump(&mut self) -> String;
}

impl SourceDumper for Mysql {
    fn dump(&mut self) -> String {
        let query_result = self.connection.query_drop(format!("SELECT  FROM `{}`;", self.database));

        match query_result {
            Ok(res) => res,
            Err(_) => panic!("Failed to dump source"),
        }

        "something".to_string()
    }
}
