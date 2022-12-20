use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::VecDeque;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while;
use nom::character::complete::alpha1;
use nom::character::complete::digit1;
use nom::character::is_alphabetic;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::IResult;

use super::expect_usize;
use super::simple_struct;

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
            Entry::File(File::new(name.to_string(), expect_usize!(size))),
        )
    })
}

fn filename(input: &str) -> IResult<&str, &str> {
    take_while(is_filename)(input)
}

fn is_filename(chr: char) -> bool {
    is_alphabetic(chr as u8) || chr == '.'
}

simple_struct!(File; name: String, size: usize);

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

simple_struct!(DirSize; name: String, size: usize);

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
                            //println!("cd {:?}", cd);
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
                            //println!("create file {:?}", f);
                            in_dir.add_file(f);
                            Go::Here
                        }
                        Entry::Dir(d) => {
                            //println!("create dir {:?}", d);
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

    fn find_all_dir_sizes(&self, d: &Dir) -> (Vec<DirSize>, usize) {
        let mut self_size = d.files.iter().map(|(_k, f)| f.size).sum();
        let mut self_and_child_dirs: Vec<DirSize> = vec![];
        println!("file size for {}: {}", d.name, self_size);

        for (_k, dir) in d.dirs.iter() {
            let (mut child_dirs, tot_size) = self.find_all_dir_sizes(dir);
            self_and_child_dirs.append(&mut child_dirs);
            self_size += tot_size;
        }
        self_and_child_dirs.push(DirSize::new(d.name.clone(), self_size));
        (self_and_child_dirs, self_size)
    }
}

pub fn part1(file_contents: String) -> String {
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

    let (all_dirs, size) = filesystem.find_all_dir_sizes(&filesystem.root);
    println!("all dirs: {:?}", all_dirs);
    println!("total size: {}", size);

    // find dirs with size <= 100,000
    let dirs_100k: Vec<&DirSize> = all_dirs.iter().filter(|d| d.size <= 100_000).collect();
    println!("dirs <= 100k: {:?}", dirs_100k);
    let sum_of_sizes: usize = dirs_100k.iter().map(|d| d.size).sum();
    println!("sum of those: {}", sum_of_sizes);
    format!("{}", sum_of_sizes)
}

pub fn part2(file_contents: String) -> String {
    let mut term_output: VecDeque<TermOutput> = VecDeque::new();

    file_contents.lines().for_each(|line| {
        let (leftover, result) = parse_line(line).expect("failed to parse line");
        assert_eq!(leftover, "");
        term_output.push_back(result);
    });

    let mut filesystem = Filesystem::new();
    filesystem.build(term_output);

    let (all_dirs, total_size) = filesystem.find_all_dir_sizes(&filesystem.root);
    println!("all dirs: {:?}", all_dirs);
    println!("total size: {}", total_size);

    // find which dir to delete
    let total_space = 70_000_000;
    let needed_space = 30_000_000;

    let current_unused = total_space - total_size;
    println!("current unused: {}", current_unused);
    let need_to_delete = needed_space - current_unused;
    println!("need to delete: {}", need_to_delete);

    // find dirs of at least that size
    let mut deletion_candidates: Vec<&DirSize> = all_dirs
        .iter()
        .filter(|d| d.size >= need_to_delete)
        .collect();
    println!("candidates for deletion: {:?}", deletion_candidates);
    deletion_candidates.sort_by(|a, b| {
        if a.size < b.size {
            Ordering::Less
        } else if a.size > b.size {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
    println!("sorted: {:?}", deletion_candidates);
    println!(
        "dir to delete: {:?}",
        deletion_candidates.first().expect("no dir??")
    );
    println!(
        "size of that: {}",
        deletion_candidates.first().expect("no dir??").size
    );
    format!("{}", deletion_candidates.first().expect("no dir??").size)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use crate::days::read_input_file;

    #[test]
    fn part1_example() {
        let input = read_input_file("inputs/day7-example.txt");
        assert_eq!(part1(input), "95437".to_string());
    }

    #[test]
    fn part1_input() {
        let input = read_input_file("inputs/day7-input.txt");
        assert_eq!(part1(input), "1582412".to_string());
    }

    #[test]
    fn part2_example() {
        let input = read_input_file("inputs/day7-example.txt");
        assert_eq!(part2(input), "24933642".to_string());
    }

    #[test]
    fn part2_input() {
        let input = read_input_file("inputs/day7-input.txt");
        assert_eq!(part2(input), "3696336".to_string());
    }
}
