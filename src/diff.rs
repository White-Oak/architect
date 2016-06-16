use git2::*;
use std::io::{stdout, Write};
use std::collections::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::channel;

pub fn gather_stats() -> Result<Vec<Stat>, Error> {
    // Open repo on '.'
    let repo = Repository::open(".")?;
    fn calculate_diff(repo: &Repository, from: &Commit, to: &Commit) -> Result<Stat, Error> {
        // Form two trees and find a diff of them
        let tree_from = from.tree()?;
        let tree_to = to.tree()?;
        let diff = repo.diff_tree_to_tree(Some(&tree_from),  Some(&tree_to), None)?;
        // Get stats from the diff
        let diff = diff.stats()?;
        let author = match to.author().name() {
            Some(x) => x.to_string(),
            None => "Unknown".to_string()
        };
        let email = match to.author().email() {
            Some(x) => x.to_string(),
            None => "unknown@user.com".to_string()
        };
        Ok(Stat{
            author: author,
            email: email,
            inserts: diff.insertions() as u32,
            dels: diff.deletions() as u32,
            time: to.time(),
            message: match to.message() {
                None => None,
                Some(m) => Some(m.to_string())
            }
        })
    }

    let mut stats = Vec::new();
    let visited: BTreeSet<Oid> = BTreeSet::new();
    let visited_arc = Arc::new(Mutex::new(visited));

    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    let total = revwalk.count() - 1;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    let mut commits = Vec::new();
    for commit in revwalk {
        commits.push(commit);
    }
    println!("Total: {}", total);
    print!("0/{}", total);
    stdout().flush().unwrap();

    let curr_arc = visited_arc.clone();
    let (tx, rx) = channel();
    thread::spawn(move || {
        let repo = Repository::open(".").unwrap();
        let mut stats = Vec::new();
        let mut revwalk = repo.revwalk().unwrap();
        revwalk.push_head().unwrap();
        for (_, next) in revwalk.skip(total/2).enumerate() {
            let commit = repo.find_commit(next.unwrap()).unwrap();
            if !curr_arc.lock().unwrap().contains(&commit.id()) {
                for parent in commit.parents() {
                    stats.push(calculate_diff(&repo, &parent, &commit).unwrap());
                    curr_arc.lock().unwrap().insert(parent.id());
                }
            }
        }
        tx.send(stats).unwrap();
    });

    let mut revwalk = repo.revwalk().unwrap();
    revwalk.push_head().unwrap();
    for (_, next) in revwalk.enumerate() {
        let commit = repo.find_commit(next.unwrap()).unwrap();
        if !visited_arc.lock().unwrap().contains(&commit.id()) {
            for parent in commit.parents() {
                stats.push(calculate_diff(&repo, &parent, &commit).unwrap());
                visited_arc.lock().unwrap().insert(parent.id());
            }
        }
    }


    stats.append(&mut rx.recv().unwrap());
    println!("");
    Ok(stats)
}

// Cut the commit hash to 7 symbols
// https://git-scm.com/book/en/v2/Git-Tools-Revision-Selection#Short-SHA-1
fn short_hash(full_hash: Oid) -> String {
    let short_hash = full_hash.to_string();
    short_hash[..7].to_string()
}

#[derive(Clone)]
pub struct Stat{
    pub author: String,
    pub email: String,
    pub inserts: u32,
    pub dels: u32,
    pub time: Time,
    pub message: Option<String>
}
