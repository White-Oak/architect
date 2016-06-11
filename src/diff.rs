use std::str;
use git2::{Repository, Error, DiffOptions, Oid, Time};

pub fn run() -> Result<(), Error> {
    // Open repo on '.'
    let repo = Repository::open(".")?;
    // Create an iterator to 'reverse walk' from HEAD to root of a repo
    let mut revwalk = repo.revwalk()?;
    // Set HEAD as a start of our 'walk'
    revwalk.push_head()?;

    // Prepare our diff options based on the arguments given
    // We set no options
    let mut opts = DiffOptions::new();

    let mut stats_vec = Vec::new();

    // Get HEAD id from a repo. A HEAD should be present so we just unwrap this thing
    let head_oid = repo.head()?.target().unwrap();
    // Our first commit is the one that HEAD points at
    let mut from = repo.find_commit(head_oid)?;

    // Walking through all commits, skipping first as it's the one HEAD points to
    for oid in revwalk.skip(1) {
        let to = repo.find_commit(oid?)?;
        println!("FROM {} TO {}", short_hash(&from.id()), short_hash(&to.id()));
        // Form two trees and find a diff of them
        let tree_from = from.tree()?;
        let tree_to = to.tree()?;
        let diff = repo.diff_tree_to_tree(Some(&tree_to),  Some(&tree_from), Some(&mut opts))?;

        // Get stats from the diff
        let stats = diff.stats()?;
        println!("Insertions: {}; Deletions: {}", stats.insertions(), stats.deletions());
        match from.message() {
            None => println!("STARRING {}", from.author()),
            Some(m) => println!("STARRING {}:\n{}", from.author(), m)
        };
        println!("");
        stats_vec.push(Stat{
                            author: format!("{}", to.author()),
                            inserts: stats.insertions() as u32,
                            dels: stats.deletions() as u32,
                            time: to.time(),
                            message: match from.message() {
                                None => String::new(),
                                Some(m) => m.into()
                            }
                        });

        // Prepare for next iteration
        from = to;
    }

    Ok(())
}

// Cut the commit hash to 7 symbols
// https://git-scm.com/book/en/v2/Git-Tools-Revision-Selection#Short-SHA-1
fn short_hash(full_hash: &Oid) -> String {
    let short_hash = full_hash.to_string();
    short_hash[..7].to_string()
}
struct Stat{
    author: String,
    inserts: u32,
    dels: u32,
    time: Time,
    message: String
}
