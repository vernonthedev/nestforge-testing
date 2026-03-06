use nestforge::ResourceService;

use crate::users::dto::UserDto;

pub type UsersService = ResourceService<UserDto>;
