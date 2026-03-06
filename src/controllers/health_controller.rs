use nestforge::{controller, routes};

#[controller("")]
pub struct HealthController;

#[routes]
impl HealthController {
    #[nestforge::get("/health")]
    async fn health() -> String {
        "OK".to_string()
    }
}
