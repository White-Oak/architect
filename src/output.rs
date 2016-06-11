use git2::{Time};

pub fn print_stat(stat: &Stat){
    println!("Insertions: {}; Deletions: {}", stat.inserts, stat.dels);
    match &stat.message {
        &None => println!("STARRING {}", stat.author),
        &Some(ref m) => println!("STARRING {}:\n{}", stat.author, m)
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
