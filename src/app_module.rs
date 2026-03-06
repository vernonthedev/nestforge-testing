use crate::users::UsersModule;
use nestforge::{module, ConfigModule, ConfigOptions};

use crate::{
    controllers::{AppController, HealthController},
    services::{AppConfig},
};

fn load_app_config() -> anyhow::Result<AppConfig> {
    Ok(ConfigModule::for_root::<AppConfig>(ConfigOptions::new().env_file(".env"))?)
}

#[module(
    imports = [
        /* nestforge:imports */
        UsersModule,
    ],
    controllers = [
        AppController,
        HealthController,
        /* nestforge:controllers */
    ],
    providers = [
        load_app_config()?,
        /* nestforge:providers */
    ],
    exports = []
)]
pub struct AppModule;
