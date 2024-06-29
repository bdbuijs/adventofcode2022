use std::collections::HashMap;
use std::fs;

type Graph = HashMap<usize, Path>;

fn children<'a>(graph: &'a mut Graph, id: &usize) -> &'a mut Vec<usize> {
    if let Some(Path::Folder {
        children: ref mut c,
        ..
    }) = graph.get_mut(id)
    {
        c
    } else {
        panic! {"can't get children of folder with id: {}", id};
    }
}

fn path_of(path: &[&str]) -> String {
    let mut full_path = String::new();
    for el in path.iter() {
        full_path.push('>');
        full_path.push_str(el);
    }
    full_path
}

#[derive(Debug)]
enum Path {
    Folder {
        parent: Option<usize>,
        children: Vec<usize>,
    },
    File {
        size: usize,
    },
}

trait DiskSize {
    fn size_on_disk(&self, graph: &Graph) -> usize;
}

impl DiskSize for Path {
    fn size_on_disk(&self, graph: &Graph) -> usize {
        match self {
            Self::Folder { children: c, .. } => {
                let mut total_size = 0;
                for child in c.iter() {
                    let child_size = graph.get(child).unwrap().size_on_disk(graph);
                    total_size += child_size;
                }
                total_size
            }
            Self::File { size: s, .. } => *s,
        }
    }
}

fn main() {
    let file_path = "input.txt";
    let input_file = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let input: Vec<&str> = input_file.split('\n').collect();

    // part 1 & 2
    let mut id = 0;
    let mut ids: HashMap<String, usize> = HashMap::new();
    let mut graph: HashMap<usize, Path> = HashMap::new();
    let mut current_path: Vec<&str> = Vec::new();
    let mut current_id: Option<usize> = None;
    let mut index = 0;
    while index < input.len() {
        let line = input[index];
        let mut words = line.split(' ');
        assert_eq!(words.next().unwrap(), "$");
        match words.next() {
            Some("cd") => {
                let d = words.next().unwrap();
                match d {
                    ".." => {
                        if let Some(Path::Folder { parent: p, .. }) =
                            graph.get(&current_id.expect("attempting to .. from root!"))
                        {
                            current_id = *p;
                            current_path.pop();
                        } else {
                            unreachable!();
                        }
                    }
                    _ => {
                        current_path.push(d);
                        let p = path_of(&current_path);
                        if ids.contains_key(&p) {
                            current_id = ids.get(&p).copied();
                        } else {
                            ids.insert(path_of(&current_path), id);
                            graph.insert(
                                id,
                                Path::Folder {
                                    parent: None,
                                    children: Vec::new(),
                                },
                            );
                            current_id = Some(id);
                            id += 1;
                        }
                    }
                }
            }
            Some("ls") => {
                index += 1;
                while index < input.len() {
                    let mut nxt = input[index].split(' ');
                    let first_part = nxt.next();
                    match first_part {
                        Some("$") => {
                            index -= 1;
                            break;
                        }
                        Some("dir") => {
                            let mut p = path_of(&current_path);
                            p.push('>');
                            p.push_str(nxt.next().expect("name must follow dir"));
                            ids.insert(p, id);
                            graph.insert(
                                id,
                                Path::Folder {
                                    parent: current_id,
                                    children: Vec::new(),
                                },
                            );
                            children(&mut graph, &current_id.unwrap()).push(id);
                            id += 1;
                        }
                        Some(size) => {
                            let size = size.parse::<usize>().expect("must be valid size!");
                            let mut p = path_of(&current_path);
                            p.push('>');
                            p.push_str(nxt.next().expect("name must follow size"));
                            ids.insert(p, id);
                            graph.insert(id, Path::File { size });
                            children(&mut graph, &current_id.unwrap()).push(id);
                            id += 1;
                        }
                        None => {
                            break;
                        }
                    }
                    index += 1;
                }
            }
            Some(&_) => {
                unreachable!();
            }
            None => {
                unreachable!();
            }
        }
        index += 1;
    }

    let filesystem_size = 70_000_000;
    let mut free_space = 0;
    let mut total_size = 0;
    for (k, v) in graph.iter() {
        match v {
            Path::File { .. } => {
                continue;
            }
            Path::Folder { .. } => {
                let s = v.size_on_disk(&graph);
                if k == &0 {
                    free_space = filesystem_size - s;
                }
                if s < 100_001 {
                    total_size += s;
                }
            }
        }
    }
    let part1 = total_size;
    println!(
        "The sum of the total sizes of directories with a total size of at most 100000 is: {}",
        part1
    );

    let mut options = Vec::new();
    let min_size = 30_000_000 - free_space;
    for (_k, v) in graph.iter() {
        match v {
            Path::File { .. } => {
                continue;
            }
            Path::Folder { .. } => {
                let s = v.size_on_disk(&graph);
                if s >= min_size {
                    options.push(s);
                }
            }
        }
    }
    let part2 = options.iter().min().unwrap();
    println!("The total size of the smallest directory that, if deleted, would free up enough space on the filesystem to run the update is {}", part2);
}
