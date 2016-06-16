use git2::*;
use std::io::{stdout, Write};
use std::collections::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::channel;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::rc::Rc;

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

    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    let mut commits = Vec::new();
    for commit in revwalk {
        commits.push(commit?);
    }
    let total = commits.len();
    println!("Total: {}", total);
    print!("0/{}", total);
    stdout().flush().unwrap();

    let first = commits.clone().into_iter().skip(total/2);
    let (tx, rx) = channel();
    let current = AtomicUsize::new(0);
    let arc_current = Arc::new(current);
    let local_arc_current = arc_current.clone();
    thread::spawn(move || {
        let repo = Repository::open(".").unwrap();
        let mut stats = Vec::new();
        for next in first {
            let commit = repo.find_commit(next).unwrap();
            for parent in commit.parents() {
                stats.push(calculate_diff(&repo, &parent, &commit).unwrap());
            }
            local_arc_current.fetch_add(1, Ordering::SeqCst);
        }
        tx.send(stats).unwrap();
    });

    let first = commits.clone().into_iter().take(total/2);
    let (tx2, rx2) = channel();
    let local_arc_current = arc_current.clone();
    thread::spawn(move || {
        let repo = Repository::open(".").unwrap();
        let mut stats = Vec::new();
        for (_, next) in first.enumerate() {
            let commit = repo.find_commit(next).unwrap();
            for parent in commit.parents() {
                stats.push(calculate_diff(&repo, &parent, &commit).unwrap());
            }
            local_arc_current.fetch_add(1, Ordering::SeqCst);
        }
        tx2.send(stats).unwrap();
    });

    let mut last_percent = 0;
    loop{
        let counter = arc_current.load(Ordering::Relaxed);
        let of_half_percents = counter * 200 / total;
        if of_half_percents - last_percent >= 1 {
            print!("\r{}/{}", arc_current.load(Ordering::Relaxed), total);
            stdout().flush().unwrap();
            last_percent = of_half_percents;
        }
        if counter >= total {
            break
        }
    }

    stats.append(&mut rx.recv().unwrap());
    stats.append(&mut rx2.recv().unwrap());
    print!("\r{}/{}", total, total);
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
