pub mod controllers;
pub mod dto;
pub mod services;

use nestforge::module;

use self::controllers::{
    /* nestforge:feature_controllers_use */
    UsersController,
};
use self::services::{
    Service,
    /* nestforge:feature_services_use */
    UsersService,
};

#[module(
    imports = [],
    controllers = [
        /* nestforge:feature_controllers */
        UsersController,
    ],
    providers = [
        Service::new(),
        /* nestforge:feature_providers */
        UsersService::new(),
    ],
    exports = [
        Service,
        /* nestforge:feature_exports */
        UsersService,
    ]
)]
pub struct UsersModule;

// Feature module: users
