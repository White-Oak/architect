use git2::{Time};

use ansi_term::Colour::{Green, Red};

pub fn print_stat(stat: &Stat){
    println!("Insertions: {}; Deletions: {}", Green.paint(stat.inserts.to_string()),
        Red.paint(stat.dels.to_string()));
    match stat.message {
        None => println!("STARRING {}", stat.author),
        Some(ref m) => println!("STARRING {}:\n{}", stat.author, m)
    };
    println!("");
}

#[derive(Clone)]
pub struct Stat{
    pub author: String,
    pub inserts: u32,
    pub dels: u32,
    pub time: Time,
    pub message: Option<String>
}
