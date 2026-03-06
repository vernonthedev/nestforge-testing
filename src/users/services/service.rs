use crate::users::dto::UserDto;

#[derive(Clone)]
pub struct Service;

pub type UsersService = nestforge::ResourceService<UserDto>;

impl Service {
    pub fn new() -> Self {
        Self
    }

    pub fn hello(&self) -> String {
        "Hello from users module".to_string()
    }
}
