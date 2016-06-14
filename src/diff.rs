use std::collections::*;
use git2::*;
use output::*;

pub fn gather_stats() -> Result<Vec<Stat>, Error> {
    // Open repo on '.'
    let repo = Repository::open(".")?;

    fn diff_commit(repo: &Repository, from: &Commit, to: &Commit, stats: &mut Vec<Stat>,
         visited: &mut BTreeSet<Oid>) -> Result<(), Error>{
        println!("FROM {} TO {}", short_hash(from.id()), short_hash(to.id()));

        // Form two trees and find a diff of them
        let tree_from = from.tree()?;
        let tree_to = to.tree()?;
        let diff = repo.diff_tree_to_tree(Some(&tree_to),  Some(&tree_from), None)?;
        // Get stats from the diff
        let diff = diff.stats()?;
        let new_stat = Stat{
                            author: format!("{}", to.author()),
                            inserts: diff.insertions() as u32,
                            dels: diff.deletions() as u32,
                            time: to.time(),
                            message: match from.message() {
                                None => None,
                                Some(m) => Some(m.to_string())
                            }
                        };
        print_stat(&new_stat);
        stats.push(new_stat);
        if from.parents().count() > 1 {
            println!("{} has {} parents", short_hash(from.id()), from.parents().count());
        }
        for parent in from.parents() {
            if !visited.contains(&parent.id()) {
                visited.insert(parent.id());
                diff_commit(repo, &parent, from, stats, visited)?
            }
        }
        Ok(())
    }

    let mut stats = Vec::new();
    let mut visited = BTreeSet::new();

    // Get HEAD id from a repo. A HEAD should be present so we just unwrap this thing
    let head_oid = repo.head()?.target().unwrap();
    // Our first commit is the one that HEAD points at
    let head = repo.find_commit(head_oid)?;
    // Walking through all commits, skipping first as it's the one HEAD points to
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
