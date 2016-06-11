use std::str;
use git2::{Repository, Error, Object, ObjectType, DiffOptions, Diff, Revwalk, Oid, Commit};

#[derive(PartialEq, Eq, Copy, Clone)]
enum Cache { Normal, Only, None }

pub fn run() -> Result<(), Error> {
    let path = ".";
    let repo = try!(Repository::open(path));
    let mut revwalk = try!(repo.revwalk());
    revwalk.push_head();

    // Prepare our diff options based on the arguments given
    let mut opts = DiffOptions::new();

    // Prepare the diff to inspect
    let from = try!(get_nth_commit(&repo, revwalk.nth(0)));
    let to = try!(get_nth_commit(&repo, revwalk.nth(1)));
    println!("FROM {} TO {}", from.id(), to.id());
    let tree_from = from.tree().unwrap();
    let tree_to = to.tree().unwrap();
    let diff = try!(repo.diff_tree_to_tree(Some(&tree_to),  Some(&tree_from), Some(&mut opts)));

    // Generate simple output
    let stats = try!(diff.stats());
    println!("Insertions: {}; Deletions: {}", stats.insertions(), stats.deletions());

    let message = match from.message() {
        None => "...",
        Some(m) => m
    };
    println!("STARRING {}:\n{}", from.author(), message);

    Ok(())
}

fn get_nth_commit<'repo, 'b>(repo: &'repo Repository, inp: Option<Result<Oid, Error>>) -> Result<Commit, Error> {
    let res = try!(inp.ok_or(Error::from_str("NEED MORE COMMITS")));
    let oid = try!(res.or(Err(Error::from_str("NEED MORE COMMITS"))));
    repo.find_commit(oid.clone())
}
