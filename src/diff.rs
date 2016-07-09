use git2::*;
use std::io::{stdout, Write};
use std::sync::Arc;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use num_cpus;
use crossbeam::sync::SegQueue;

pub fn gather_stats() -> Result<Vec<Stat>, Error> {
    // Open repo on '.'
    let repo = Repository::open(".")?;
    /// Calculates a diff for two commits
    fn calculate_diff(repo: &Repository, from: &Commit, to: &Commit) -> Result<Stat, Error> {
        // Form two trees and find a diff of them
        let tree_from = from.tree()?;
        let tree_to = to.tree()?;
        let diff = repo.diff_tree_to_tree(Some(&tree_from), Some(&tree_to), None)?;
        // Get stats from the diff
        let diff = diff.stats()?;
        let author = match to.author().name() {
            Some(x) => x.to_string(),
            None => "Unknown".to_string(),
        };
        let email = match to.author().email() {
            Some(x) => x.to_string(),
            None => "unknown@user.com".to_string(),
        };
        Ok(Stat {
            id: to.id(),
            author: author,
            email: email,
            inserts: diff.insertions() as u32,
            dels: diff.deletions() as u32,
            time: to.time(),
            message: match to.message() {
                None => None,
                Some(m) => Some(m.to_string()),
            },
        })
    }

    // Gather all commits into a list
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    let commits = Arc::new(SegQueue::new());
    let mut total = 0;
    for commit in revwalk {
        commits.push(commit?);
        total += 1;
    }

    // Find out parameters
    let threads_num = num_cpus::get();
    println!("Total: {}", total);
    println!("Counting on {} threads", threads_num);
    print!("0/{}", total);
    stdout().flush().unwrap();

    // Counts an amount of commits
    let current = Arc::new(AtomicUsize::new(0));
    let (tx, rx) = channel();
    for _ in 0..threads_num {
        // Local references
        let current = current.clone();
        let commits = commits.clone();
        let tx = tx.clone();
        thread::spawn(move || {
            let repo = Repository::open(".").unwrap();
            let mut stats = Vec::new();
            while let Some(next) = commits.try_pop() {
                let commit = repo.find_commit(next).unwrap();
                let parents = commit.parents();
                // Skip if merge commit
                if parents.len() == 1 {
                    for parent in commit.parents() {
                        stats.push(calculate_diff(&repo, &parent, &commit).unwrap());
                    }
                }
                current.fetch_add(1, Ordering::Relaxed);
            }
            tx.send(stats).unwrap();
        });
    }

    // Checking if there is enough commits to inform user about it
    let mut last_percent = 0;
    loop {
        let counter = current.load(Ordering::Relaxed);
        let of_half_percents = counter * 200 / total;
        if of_half_percents - last_percent >= 1 {
            print!("\r[");
            for i in 0..50 {
                if i < of_half_percents / 4 {
                    print!("■");
                } else {
                    print!(" ");
                }
            }
            print!("] {}/{}", counter, total);
            stdout().flush().unwrap();
            last_percent = of_half_percents;
        }
        if counter >= total {
            break;
        }
        thread::sleep(Duration::from_millis(500));
    }

    // Receiveing data from all streams
    let mut stats = Vec::new();
    for _ in 0..threads_num {
        stats.append(&mut rx.recv().unwrap());
    }
    println!("");
    Ok(stats)
}

#[derive(Clone)]
pub struct Stat {
    pub id: Oid,
    pub author: String,
    pub email: String,
    pub inserts: u32,
    pub dels: u32,
    pub time: Time,
    pub message: Option<String>,
}
