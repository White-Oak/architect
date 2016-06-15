use std::collections::*;
use git2::*;

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
            None => panic!("No author for {}", to.author())
        };
        let email = match to.author().email() {
            Some(x) => x.to_string(),
            None => panic!("No email for {}", to.author())
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

    fn diff_commit(repo: &Repository, from: &Commit, to: &Commit, stats: &mut Vec<Stat>,
         visited: &mut BTreeSet<Oid>) -> Result<(), Error>{
        stats.push(calculate_diff(repo,from,to)?);
        for parent in from.parents() {
            if !visited.contains(&parent.id()) || !visited.contains(&from.id()) {
                diff_commit(repo, &parent, from, stats, visited)?
            }
        }
        visited.insert(from.id());
        Ok(())
    }

    let mut stats = Vec::new();
    let mut visited = BTreeSet::new();

    // Get HEAD id from a repo. A HEAD should be present so we just unwrap this thing
    let head_oid = repo.head()?.target().unwrap();
    // Our first commit is the one that HEAD points at
    let head = repo.find_commit(head_oid)?;
    // Walking through all commits
    for parent in head.parents() {
        diff_commit(&repo, &parent, &head, &mut stats, &mut visited)?
    }
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
