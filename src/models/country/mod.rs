pub mod list;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    pub id: i64,
    pub name: String,
    pub zip_code: String,
    pub country: Country,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Country {
    pub name: list::Countries,
    pub phone: String,
    pub abr: String,
    pub state: Vec<State>,
}

impl Country {
    fn from_state(st: State) -> () {}
    fn from_abr(st: String) -> () {}
    fn from_phone(st: String) -> () {}
    fn from_name(self, name: list::Countries) -> () {
        // match name {
        //     list::Country::Afghanistan | _ => list::Country::Algeria,
        // }
    }
}
