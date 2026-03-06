#[nestforge::dto]
pub struct CreateUserDto {
    #[validate(required)]
    pub name: String,
}
