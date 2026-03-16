use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SignInFormRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SignUpFormRequest {
    pub username: String,
    pub password: String,
    pub confirm_password: String,
}
