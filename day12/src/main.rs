use std::{
    collections::{BinaryHeap, HashSet},
    fs,
};

fn main() {
    let file_path = "input.txt";
    let input_file = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let input: Vec<&str> = input_file.split('\n').collect();

    // part 1
    let (start_x, start_y) = input
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            if let Some((x, _)) = row.char_indices().find(|&(_, c)| c == 'S') {
                Some((x, y))
            } else {
                None
            }
        })
        .expect("There's a start");
    let shortest_path = dijkstra(&input, start_x, start_y);
    let part1 = shortest_path;
    println!("The fewest steps required to move from your current position to the location that should get the best signal is {}", part1);

    // part 2
    let part2 = input
        .iter()
        .enumerate()
        .filter_map(|(y, row)| {
            if let Some((x, _)) = row.char_indices().find(|&(_, c)| c == 'a') {
                Some((x, y))
            } else {
                None
            }
        })
        .map(|(x, y)| dijkstra(&input, x, y))
        .min()
        .expect("There are paths");
    println!("The fewest steps required to move starting from any square with elevation a to the location that should get the best signal is {}", part2);
}

fn neighbours<'a>(
    map: &'a [&str],
    visited: &'a HashSet<(usize, usize)>,
    x: usize,
    y: usize,
    length: usize,
) -> impl Iterator<Item = Node> + 'a {
    let width = map[0].len();
    let height = map.len();
    let length = length + 1;
    let c = map[y].chars().nth(x).expect("Coordinates are valid");
    [
        (x.saturating_sub(1), y),
        (x + 1, y),
        (x, y.saturating_sub(1)),
        (x, y + 1),
    ]
    .into_iter()
    .filter(move |&(nx, ny)| nx < width && ny < height && !visited.contains(&(nx, ny)))
    .filter_map(move |(nx, ny)| {
        let d = map[ny].chars().nth(nx).expect("Already filtered");
        if c.can_reach(d) {
            Some(Node {
                x: nx,
                y: ny,
                length,
            })
        } else {
            None
        }
    })
}

/// Length of shortest path to 'E'
fn dijkstra(map: &[&str], start_x: usize, start_y: usize) -> usize {
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    let start = Node {
        x: start_x,
        y: start_y,
        length: 0,
    };
    queue.push(start);
    while let Some(Node { x, y, length }) = queue.pop() {
        if visited.contains(&(x, y)) {
            continue;
        }
        if map[y].chars().nth(x) == Some('E') {
            return length;
        }
        visited.insert((x, y));
        neighbours(map, &visited, x, y, length).for_each(|n| queue.push(n))
    }

    unreachable!("No path up the mountain!");
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    x: usize,
    y: usize,
    length: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match other.length.cmp(&self.length) {
            std::cmp::Ordering::Equal => {}
            cmp => return cmp,
        }
        match self.x.cmp(&other.x) {
            std::cmp::Ordering::Equal => {}
            cmp => return cmp,
        }
        self.y.cmp(&other.y)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

trait Height {
    fn can_reach(&self, destination: Self) -> bool;
}

impl Height for char {
    fn can_reach(&self, destination: Self) -> bool {
        *self == 'S' || *self == 'z' || (97..=(*self as u8 + 1)).contains(&(destination as u8))
    }
}
