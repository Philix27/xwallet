pub struct CreateUserInput {
    first_name: String,
    last_name: String,
    email: String,
    phone: String,
    password: String,
}
pub struct CreateUserResponse {
    message: String,
}
pub struct KycInput {
    first_name: String,
    last_name: String,
    middle_name: String,
    phone: String,
    email: String,
    country: String,
    address: String,
}
