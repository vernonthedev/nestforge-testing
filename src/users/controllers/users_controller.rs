use axum::Json;
use nestforge::{controller, routes, ApiResult, Inject, List, OptionHttpExt, Param, ResultHttpExt, ValidatedBody};

use crate::users::dto::{CreateUserDto, UpdateUserDto, UserDto};
use crate::users::services::UsersService;

#[controller("/users")]
pub struct UsersController;

#[routes]
impl UsersController {
    #[nestforge::version("1")]
    #[nestforge::get("/")]
    async fn list(
        service: Inject<UsersService>,
    ) -> ApiResult<List<UserDto>> {
        Ok(Json(service.all()))
    }

    #[nestforge::version("2")]
    #[nestforge::get("/{id}")]
    async fn get_one(
        id: Param<u64>,
        service: Inject<UsersService>,
    ) -> ApiResult<UserDto> {
        let id = id.value();
        let item = service
            .get(id)
            .or_not_found_id("User", id)?;

        Ok(Json(item))
    }

    #[nestforge::version("2")]
    #[nestforge::post("/")]
    async fn create(
        service: Inject<UsersService>,
        body: ValidatedBody<CreateUserDto>,
    ) -> ApiResult<UserDto> {
        let item = service
            .create(body.value())
            .or_bad_request()?;
        Ok(Json(item))
    }

    #[nestforge::version("2")]
    #[nestforge::put("/{id}")]
    async fn update(
        id: Param<u64>,
        service: Inject<UsersService>,
        body: ValidatedBody<UpdateUserDto>,
    ) -> ApiResult<UserDto> {
        let id = id.value();
        let item = service
            .update(id, body.value())
            .or_bad_request()?
            .or_not_found_id("User", id)?;

        Ok(Json(item))
    }

    #[nestforge::version("2")]
    #[nestforge::delete("/{id}")]
    async fn delete(
        id: Param<u64>,
        service: Inject<UsersService>
    ) -> ApiResult<UserDto>{
        let id = id.value();
        let deleted_user = service.delete(id).or_not_found_id("User", id)?;
        Ok(Json(deleted_user))
    }
}
