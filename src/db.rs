use sqlx::{migrate::Migrator, PgPool};

use crate::config::Settings;

static MIGRATOR: Migrator = sqlx::migrate!();

pub(crate) async fn create_pool(settings: &Settings) -> PgPool {
    let url = settings.database_url();
    PgPool::connect(url.as_str())
        .await
        .expect("cannot create database pool")
}

pub(crate) async fn init() {
    println!("Init Postgres")
}

pub(crate) async fn migrate(settings: &Settings) {
    let pool = create_pool(settings).await;
    match MIGRATOR.run(&pool).await {
        Ok(_) => println!("✅ database has been migrated"),
        Err(e) => panic!("{}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn migrate_with_environment() {
        let settings = Settings::from_env();
        migrate(&settings).await;
        // Nothing paniced ✅
    }
}
