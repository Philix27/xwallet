pub struct State {
    name: String,
    zip_code: String,
}
pub struct Country {
    name: String,
    phone: String,
    abr: String,
    state: Vec<State>,
}

impl Country {}
