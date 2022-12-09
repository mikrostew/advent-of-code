use std::collections::HashMap;
use std::collections::VecDeque;
use std::path::Path;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while;
use nom::character::complete::alpha1;
use nom::character::complete::digit1;
use nom::character::is_alphabetic;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::IResult;

use super::{parse_usize, read_file, run_parts};

run_parts!();

#[derive(Clone, Debug)]
enum TermOutput {
    Cmd(Command),
    Output(Entry),
}

#[derive(Clone, Debug)]
enum Command {
    Cd(ChangeDir),
    Ls,
}

#[derive(Clone, Debug)]
enum ChangeDir {
    Root,
    Up,
    ToDir(String),
}

#[derive(Clone, Debug)]
enum Entry {
    File(File),
    Dir(String),
}

fn parse_line(line: &str) -> IResult<&str, TermOutput> {
    alt((cmd, output))(line)
}

fn cmd(input: &str) -> IResult<&str, TermOutput> {
    alt((cd, ls))(input).map(|(next_input, result)| (next_input, TermOutput::Cmd(result)))
}

fn cd(input: &str) -> IResult<&str, Command> {
    preceded(tag("$ cd "), alt((cd_root, cd_up, cd_dir)))(input)
        .map(|(next_input, result)| (next_input, Command::Cd(result)))
}

fn cd_root(input: &str) -> IResult<&str, ChangeDir> {
    tag("/")(input).map(|(next_input, _result)| (next_input, ChangeDir::Root))
}

fn cd_up(input: &str) -> IResult<&str, ChangeDir> {
    tag("..")(input).map(|(next_input, _result)| (next_input, ChangeDir::Up))
}

fn cd_dir(input: &str) -> IResult<&str, ChangeDir> {
    alpha1(input).map(|(next_input, result)| (next_input, ChangeDir::ToDir(result.to_string())))
}

fn ls(input: &str) -> IResult<&str, Command> {
    tag("$ ls")(input).map(|(next_input, _result)| (next_input, Command::Ls))
}

fn output(input: &str) -> IResult<&str, TermOutput> {
    alt((dir, file))(input).map(|(next_input, result)| (next_input, TermOutput::Output(result)))
}

fn dir(input: &str) -> IResult<&str, Entry> {
    preceded(tag("dir "), alpha1)(input)
        .map(|(next_input, result)| (next_input, Entry::Dir(result.to_string())))
}

fn file(input: &str) -> IResult<&str, Entry> {
    separated_pair(digit1, tag(" "), filename)(input).map(|(next_input, (size, name))| {
        (
            next_input,
            Entry::File(File {
                name: name.to_string(),
                size: parse_usize!(size),
            }),
        )
    })
}

fn filename(input: &str) -> IResult<&str, &str> {
    take_while(is_filename)(input)
}

fn is_filename(chr: char) -> bool {
    is_alphabetic(chr as u8) || chr == '.'
}

#[derive(Clone, Debug)]
struct File {
    name: String,
    size: usize,
}

#[derive(Clone, Debug)]
struct Dir {
    name: String,
    dirs: HashMap<String, Dir>,
    files: HashMap<String, File>,
}

impl Dir {
    fn new(name: String) -> Dir {
        Dir {
            name,
            dirs: HashMap::new(),
            files: HashMap::new(),
        }
    }

    // add a child dir if it doesn't already exist
    fn add_child_name(&mut self, name: String) -> () {
        if !self.has_child_dir(&name) {
            self.dirs.insert(name.clone(), Dir::new(name));
        }
    }

    fn add_child_dir(&mut self, d: Dir) -> () {
        // need to check if dir exists?
        self.dirs.insert(d.name.clone(), d);
    }

    fn has_child_dir(&self, name: &String) -> bool {
        self.dirs.contains_key(name)
    }

    // easier to remove & re-add, instead of trying to mutate it behind &
    fn remove_child_dir(&mut self, name: &String) -> Option<Dir> {
        self.dirs.remove(name)
    }

    // add file if it doesn't already exist
    fn add_file(&mut self, f: File) -> () {
        if !self.files.contains_key(&f.name) {
            self.files.insert(f.name.clone(), f);
        }
    }
}

// where to go from the current dir
#[derive(Debug, Eq, PartialEq)]
enum Go {
    Up,
    Root,
    Here,
    Eof, // no more instructions
}

#[derive(Debug)]
struct Filesystem {
    root: Dir,
}

impl Filesystem {
    fn new() -> Filesystem {
        Filesystem {
            root: Dir::new("/".to_string()),
        }
    }

    // build filesystem from the file & dir info in the input
    fn build(&mut self, term_output: VecDeque<TermOutput>) -> () {
        // Note: the first thing in each input is `cd /`
        // (if not this will fail)
        (self.root, _, _) = self.build_from_info(self.root.clone(), term_output);
    }

    fn build_from_info(
        &mut self,
        mut in_dir: Dir,
        mut term_output: VecDeque<TermOutput>,
    ) -> (Dir, VecDeque<TermOutput>, Go) {
        let mut where_to_go = Go::Here;

        while where_to_go == Go::Here || (where_to_go == Go::Root && in_dir.name == "/") {
            if let Some(next_thing) = term_output.pop_front() {
                where_to_go = match next_thing {
                    TermOutput::Cmd(c) => match c {
                        Command::Cd(cd) => {
                            println!("cd {:?}", cd);
                            match cd {
                                ChangeDir::Root => Go::Root,
                                ChangeDir::Up => {
                                    if in_dir.name == "/" {
                                        panic!("In root dir - can't cd out of the filesystem!");
                                    }
                                    Go::Up
                                }
                                ChangeDir::ToDir(d) => {
                                    // get ownership of that dir to change it, then re-add it
                                    let child_dir =
                                        in_dir.remove_child_dir(&d).expect("Dir does not exist!");
                                    let changed_dir;
                                    let go;
                                    (changed_dir, term_output, go) =
                                        self.build_from_info(child_dir, term_output.clone());
                                    in_dir.add_child_dir(changed_dir);
                                    // some things need to be popped up futher
                                    match go {
                                        Go::Root | Go::Eof => go,
                                        _ => Go::Here,
                                    }
                                }
                            }
                        }
                        Command::Ls => {
                            //println!("ls (nothing to do)");
                            Go::Here
                        }
                    },
                    TermOutput::Output(e) => match e {
                        Entry::File(f) => {
                            println!("create file {:?}", f);
                            in_dir.add_file(f);
                            Go::Here
                        }
                        Entry::Dir(d) => {
                            println!("create dir {:?}", d);
                            in_dir.add_child_name(d);
                            Go::Here
                        }
                    },
                };
            } else {
                where_to_go = Go::Eof;
            }
            //println!("go: {:?}", where_to_go);
        }
        (in_dir, term_output, where_to_go)
    }
}

fn part1<P: AsRef<Path>>(path: P) -> () {
    read_file!(file_contents, path);
    //println!("{}", file_contents);

    let mut term_output: VecDeque<TermOutput> = VecDeque::new();

    file_contents.lines().for_each(|line| {
        //println!("{}", line);
        let (leftover, result) = parse_line(line).expect("failed to parse line");
        assert_eq!(leftover, "");
        //println!("result: {:?}", result);
        term_output.push_back(result);
    });

    let mut filesystem = Filesystem::new();
    filesystem.build(term_output);
}

fn part2<P: AsRef<Path>>(path: P) -> () {
    read_file!(file_contents, path);
    println!("{}", file_contents);
}
