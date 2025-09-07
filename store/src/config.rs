pub struct Config {
    pub db_url: String,
}

impl Default for Config {
    fn default() -> Self {
        let db_url: String = env::var("DATABASE_URL")
            .unwrap_or_else(|_| panic!("please provide the database env variable"));

        Self { db_url }
    }
}
