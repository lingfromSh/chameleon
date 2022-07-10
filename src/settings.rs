/*
 * @Author: stephen.ling lingfromsh@163.com
 * @Date: 2022-07-10 01:55:05
 * @LastEditors: stephen.ling
 * @LastEditTime: 2022-07-10 13:46:19
 * @Description: settings
 */

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Project {
    name: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Server {
    host: String,
    port: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Auth {
    jwt_key: String,
    jwt_salt: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Database {
    conn_url: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Redis {
    conn_url: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    project: Project,
    server: Server,
    auth: Auth,
    database: Database,
    redis: Redis,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mode = env::var("MODE").unwrap_or_else(|_| "development".into());

        let config = Config::builder()
            .add_source(File::with_name("src/config/default.toml"))
            .add_source(File::with_name(&format!("src/config/{}.toml", mode)).required(false))
            .add_source(File::with_name("src/config/local.toml").required(false))
            .add_source(Environment::with_prefix("CHAMELEON"))
            .build()?;
        
        config.try_deserialize()
    }
}
