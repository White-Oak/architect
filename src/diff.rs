use std::str;
use git2::{Repository, Error, Object, ObjectType, DiffOptions, Diff};

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
    let from = repo.find_commit(revwalk.nth(0).expect("NEED MORE COMMITS").expect("NEED MORE COMMITS")).unwrap();
    let to = repo.find_commit(revwalk.nth(1).expect("NEED MORE COMMITS").expect("NEED MORE COMMITS")).unwrap();
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

fn tree_to_treeish<'a>(repo: &'a Repository, arg: Option<&String>)
                       -> Result<Option<Object<'a>>, Error> {
    let arg = match arg { Some(s) => s, None => return Ok(None) };
    let obj = try!(repo.revparse_single(arg));
    let tree = try!(obj.peel(ObjectType::Tree));
    Ok(Some(tree))
}
