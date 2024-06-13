use crate::config::{Config, IConfig};
use mongodb::error::Error;
use mongodb::{options::ClientOptions, Client};

pub struct Connection {}

impl Connection {

    pub async fn init() -> Result<Client, Error> {
        let _config: Config = Config {};
        let database_user = _config.get_config_with_key("DATABASE_USER");
        let database_pass = _config.get_config_with_key("DATABASE_PASS");
        let database_host = _config.get_config_with_key("DATABASE_HOST");

        let connect_string = format!(
            "mongodb://{}:{}@{}:27017",
            database_user, database_pass, database_host
        );
        let client_options = ClientOptions::parse(&connect_string).await?;
        Ok(mongodb::Client::with_options(client_options)?)
    }
}
