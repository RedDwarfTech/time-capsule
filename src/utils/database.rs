use diesel::PgConnection;
use rust_wheel::config::db::config;

pub fn get_connection() -> PgConnection {
    return config::connection("TIK_DATABASE_URL".to_string());
}

