use log::trace;
use std::env::var;

pub struct Config {
    pub host: String,
    pub port: u32,
}

impl Config {
    pub fn get_server_url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

pub fn from_env() -> Config {
    trace!("constructing config");
    Config {
        host: var("HOST").expect("HOST is not set"),
        port: var("PORT")
            .expect("PORT is not set")
            .parse()
            .expect("PORT is not a number"),
    }
}
