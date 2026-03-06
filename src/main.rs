mod app_module;
mod users;
mod controllers;
mod dto;
mod guards;
mod interceptors;
mod services;

use app_module::AppModule;
use nestforge::{ NestForgeFactory };

const PORT: u16 = 3000;

async fn bootstrap() -> anyhow::Result<()> {
    NestForgeFactory::<AppModule>::create()?
        .with_global_prefix("api")
        .listen(PORT)
        .await
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    bootstrap().await
}
