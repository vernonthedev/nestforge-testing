#[nestforge::dto]
pub struct UserDto {
    pub id: u64,
    pub name: String,
}

nestforge::impl_identifiable!(UserDto, id);
