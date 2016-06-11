use std::str;
use git2::{Repository, Error, Object, ObjectType, DiffOptions, Diff, Revwalk, Oid, Commit};

#[derive(PartialEq, Eq, Copy, Clone)]
enum Cache { Normal, Only, None }

pub fn run() -> Result<(), Error> {
    let path = ".";
    let repo = Repository::open(path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head();

    // Prepare our diff options based on the arguments given
    let mut opts = DiffOptions::new();

    // Prepare the diff to inspect
    let from = get_nth_commit(&repo, revwalk.nth(0))?;
    let to = get_nth_commit(&repo, revwalk.nth(1))?;
    println!("FROM {} TO {}", from.id(), to.id());
    let tree_from = from.tree()?;
    let tree_to = to.tree()?;
    let diff = repo.diff_tree_to_tree(Some(&tree_to),  Some(&tree_from), Some(&mut opts))?;

    // Generate simple output
    let stats = diff.stats()?;
    println!("Insertions: {}; Deletions: {}", stats.insertions(), stats.deletions());

    let message = match from.message() {
        None => "...",
        Some(m) => m
    };
    println!("STARRING {}:\n{}", from.author(), message);

    Ok(())
}

fn get_nth_commit(repo: &Repository, inp: Option<Result<Oid, Error>>) -> Result<Commit, Error> {
    let res = inp.ok_or(Error::from_str("NEED MORE COMMITS"))?;
    let oid = res.or(Err(Error::from_str("NEED MORE COMMITS")))?;
    repo.find_commit(oid.clone())
}
