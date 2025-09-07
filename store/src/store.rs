use crate::config::Config;
use diesel::prelude::*;
use diesel::Connection;
use diesel::ConnectionError;
use diesel::PgConnection;

pub struct Store {
    pub conn: PgConnection,
}

impl Store {
    pub fn default() -> Result<Self, ConnectionError> {
        let config: Config = Config::default();

        let conn: PgConnection = PgConnection::establish(&config.db_url)?;
        Ok(Self { conn })
    }
}
