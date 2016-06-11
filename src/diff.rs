use std::str;
use git2::{Repository, Error, DiffOptions, Oid, Commit};

pub fn run() -> Result<(), Error> {
    let path = ".";
    let repo = Repository::open(path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    // Prepare our diff options based on the arguments given
    let mut opts = DiffOptions::new();

    let mut from = get_nth_commit(&repo, revwalk.nth(0).unwrap())?;

    for oid in revwalk {
        let to = get_nth_commit(&repo, oid)?;
        println!("FROM {} TO {}", from.id(), to.id());
        let tree_from = from.tree()?;
        let tree_to = to.tree()?;
        let diff = repo.diff_tree_to_tree(Some(&tree_to),  Some(&tree_from), Some(&mut opts))?;

        let stats = diff.stats()?;
        println!("Insertions: {}; Deletions: {}", stats.insertions(), stats.deletions());
        match from.message() {
            None => println!("STARRING {}", from.author()),
            Some(m) => println!("STARRING {}:\n{}", from.author(), m)
        };
        println!("");
        from = to;
    }

    Ok(())
}

fn get_nth_commit(repo: &Repository, res: Result<Oid, Error>) -> Result<Commit, Error> {
    let oid = res.or_else(|_| Err(Error::from_str("NEED MORE COMMITS")))?;
    repo.find_commit(oid)
}
