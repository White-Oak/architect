use std::str;
use git2;
use git2::{Repository, Error, Object, ObjectType, DiffOptions, Diff};
use git2::{DiffFindOptions, DiffFormat};

const RESET: &'static str = "\u{1b}[m";
const BOLD: &'static str = "\u{1b}[1m";
const RED: &'static str = "\u{1b}[31m";
const GREEN: &'static str = "\u{1b}[32m";
const CYAN: &'static str = "\u{1b}[36m";

#[derive(PartialEq, Eq, Copy, Clone)]
enum Cache { Normal, Only, None }

pub fn run() -> Result<(), Error> {
    let path = ".";
    let repo = try!(Repository::open(path));

    // Prepare our diff options based on the arguments given
    let mut opts = DiffOptions::new();
    // opts.reverse(args.flag_R)
    //     .force_text(args.flag_text)
    //     .ignore_whitespace_eol(args.flag_ignore_space_at_eol)
    //     .ignore_whitespace_change(args.flag_ignore_space_change)
    //     .ignore_whitespace(args.flag_ignore_all_space)
    //     .include_ignored(args.flag_ignored)
    //     .include_untracked(args.flag_untracked)
    //     .patience(args.flag_patience)
    //     .minimal(args.flag_minimal);
    // if let Some(amt) = args.flag_unified { opts.context_lines(amt); }
    // if let Some(amt) = args.flag_inter_hunk_context { opts.interhunk_lines(amt); }
    // if let Some(amt) = args.flag_abbrev { opts.id_abbrev(amt); }
    // if let Some(ref s) = args.flag_src_prefix { opts.old_prefix(&s); }
    // if let Some(ref s) = args.flag_dst_prefix { opts.new_prefix(&s); }
    // if let Some("diff-index") = args.flag_format.as_ref().map(|s| &s[..]) {
    //     opts.id_abbrev(40);
    // }

    // Prepare the diff to inspect
    let t1 = try!(tree_to_treeish(&repo, Some(&"d928d46a7fc0c39afdc584003aa4cd2d2749ddf7".to_string()))).unwrap();
    let head = try!(tree_to_treeish(&repo, Some(&"HEAD".to_string()))).unwrap();
    let diff = try!(repo.diff_tree_to_tree(t1.as_tree(), head.as_tree(), Some(&mut opts)));

    // Generate simple output
    try!(print_stats(&diff));
    // if args.flag_patch || !stats {
    //     if args.color() { print!("{}", RESET); }
    //     let mut last_color = None;
    //     try!(diff.print(args.diff_format(), |_delta, _hunk, line| {
    //         if args.color() {
    //             let next = match line.origin() {
    //                 '+' => Some(GREEN),
    //                 '-' => Some(RED),
    //                 '>' => Some(GREEN),
    //                 '<' => Some(RED),
    //                 'F' => Some(BOLD),
    //                 'H' => Some(CYAN),
    //                 _ => None
    //             };
    //             if args.color() && next != last_color {
    //                 if last_color == Some(BOLD) || next == Some(BOLD) {
    //                     print!("{}", RESET);
    //                 }
    //                 print!("{}", next.unwrap_or(RESET));
    //                 last_color = next;
    //             }
    //         }
    //
    //         match line.origin() {
    //             '+' | '-' | ' ' => print!("{}", line.origin()),
    //             _ => {}
    //         }
    //         print!("{}", str::from_utf8(line.content()).unwrap());
    //         true
    //     }));
    //     if args.color() { print!("{}", RESET); }
    // }

    Ok(())
}

fn print_stats(diff: &Diff) -> Result<(), Error> {
    let stats = try!(diff.stats());
    let mut format = git2::DIFF_STATS_NONE;
    if true {
        format = format | git2::DIFF_STATS_FULL;
    }
    if false {
        format = format | git2::DIFF_STATS_SHORT;
    }
    if true {
        format = format | git2::DIFF_STATS_NUMBER;
    }
    if true {
        format = format | git2::DIFF_STATS_INCLUDE_SUMMARY;
    }
    let buf = try!(stats.to_buf(format, 80));
    print!("{}", str::from_utf8(&*buf).unwrap());
    Ok(())
}

fn tree_to_treeish<'a>(repo: &'a Repository, arg: Option<&String>)
                       -> Result<Option<Object<'a>>, Error> {
    let arg = match arg { Some(s) => s, None => return Ok(None) };
    let obj = try!(repo.revparse_single(arg));
    let tree = try!(obj.peel(ObjectType::Tree));
    Ok(Some(tree))
}
