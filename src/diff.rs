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
    let from = revwalk.nth(0).expect("NEED MORE COMMITS").expect("NEED MORE COMMITS");
    let to = revwalk.nth(1).expect("NEED MORE COMMITS").expect("NEED MORE COMMITS");
    println!("FROM {} TO {}", from, to);
    let tree_from = repo.find_commit(from).unwrap().tree().unwrap();
    let tree_to = repo.find_commit(to).unwrap().tree().unwrap();
    let diff = try!(repo.diff_tree_to_tree(Some(&tree_to),  Some(&tree_from), Some(&mut opts)));

    // Generate simple output
    try!(print_stats(&diff));
    Ok(())
}

fn print_stats(diff: &Diff) -> Result<(), Error> {
    let stats = try!(diff.stats());
    print!("Insertions: {}; Deletions: {}", stats.insertions(), stats.deletions());
    Ok(())
}

fn tree_to_treeish<'a>(repo: &'a Repository, arg: Option<&String>)
                       -> Result<Option<Object<'a>>, Error> {
    let arg = match arg { Some(s) => s, None => return Ok(None) };
    let obj = try!(repo.revparse_single(arg));
    let tree = try!(obj.peel(ObjectType::Tree));
    Ok(Some(tree))
}
