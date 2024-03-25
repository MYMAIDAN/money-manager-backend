use std::{env, path::Path};

use config::{Config, ConfigBuilder, ConfigError, File};
use serde::Deserialize;


#[derive(Debug,Deserialize)]
#[allow(unused)]
pub struct Database{
    pub url: String
}

#[derive(Debug,Deserialize)]
#[allow(unused)]
pub struct Jwt{
    pub secret: String,
    pub expired_in: usize,
    pub maxage: usize
}

#[derive(Debug,Deserialize)]
#[allow(unused)]
pub struct Settings{
    pub debug: bool,
    pub database: Database,
    pub jwt: Jwt
}

impl Settings{
    pub fn new(config_folder_path: &str, mode: &str) -> Result<Self, ConfigError>{
        let config = Config::builder()
        .add_source(File::with_name(&format!("{}/default.toml",config_folder_path)))
        .add_source(File::with_name(&format!("{}/{}",config_folder_path,mode)).required(false))
        .build()?;
    
        config.try_deserialize()
    }
}




