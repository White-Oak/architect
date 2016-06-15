use git2::{Time};

#[derive(Clone)]
pub struct Stat{
    pub author: String,
    pub inserts: u32,
    pub dels: u32,
    pub time: Time,
    pub message: Option<String>
}
