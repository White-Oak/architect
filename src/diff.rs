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
    let total = revwalk.count() - 1;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    println!("");
    for (i, next) in revwalk.enumerate() {
        print!("\r{}/{}", i, total);
        let commit = repo.find_commit(next?)?;
        for parent in commit.parents() {
            stats.push(calculate_diff(&repo, &parent, &commit)?);
        }
    }
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
