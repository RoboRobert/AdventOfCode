use std::{
    cmp::Ordering,
    collections::{HashMap},
    i128,
};
#[derive(Debug, Eq, Clone)]
struct Node {
    pos: (isize, isize),
    weight: i128,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        (self.weight == other.weight)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.weight.cmp(&other.weight))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

// Returns a hashmap containing the minimum path distances for every node, as well as the previous nodes for each node
// Assumes the start node is initialized with weight 0
fn min_map(weight_map: &HashMap<(isize, isize), Node>, end: (isize, isize)) -> HashMap<(isize, isize), Node> {
    let mut map_mut: HashMap<(isize, isize), Node> = weight_map.clone();
    let mut seen_map: HashMap<(isize, isize), Node> = HashMap::new();

    // Loop until all nodes are visited
    while (map_mut.len() > 0) {
        // Pops the unvisited node with lowest weight
        let current = map_mut.values().min().unwrap().clone();
        let pos = current.pos;

        // Moves current node from visited to unvisited
        map_mut.remove(&pos);
        seen_map.insert(pos, current.clone());

        // Direction vector with up, down, left, right
        let dir_vec: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        // Goes through all neighbors of the current node using the dir_vec
        for new_dir in dir_vec {
            let n_pos = (pos.0 + new_dir.0, pos.1 + new_dir.1);
            let neighbor = map_mut.get(&n_pos);
            match neighbor {
                // If the neighbor isn't in the graph, do nothing
                None => continue,
                _ => {}
            }

            let n_unwrap = neighbor.unwrap();
            let mut n_update = Node {
                pos: n_pos,
                weight: 0,
            };

            n_update.weight += 1;
            n_update.weight += current.weight;

            // If the new weight is the same or lower, replace its weight in the unvisited list
            if (n_update.weight < n_unwrap.weight) {
                map_mut.insert(n_update.pos, n_update);
            }
        }
    }

    return seen_map;
}

// Creates a map of Nodes from a 2D Vector of chars and returns start and end positions
fn create_map(char_vec: &Vec<Vec<char>>) -> (HashMap<(isize, isize), Node>,(isize, isize),(isize, isize)) {
    let mut weight_map: HashMap<(isize, isize), Node> = HashMap::new();
    let mut start: (isize, isize) = (0,0);
    let mut end: (isize, isize) = (0,0);
    for row in char_vec.iter().enumerate() {
        for col in row.1.iter().enumerate() {
            let pos = (row.0 as isize, col.0 as isize);
            let mut node: Node = Node{pos: (pos), weight: 0};

            // Insert everything except the walls
            match *col.1 {
                // Start faces East (0,1)
                'S' => {node = Node{pos: (pos), weight: 0}; start = pos;}
                'E' => {node = Node{pos: (pos), weight: i128::MAX}; end = pos;}
                '.' => node = Node{pos: (pos), weight: i128::MAX},
                '#' => continue,
                _ => {dbg!("WEIRD");}
            }
            weight_map.insert(pos, node);
        }
    }

    return (weight_map, start, end);
}

fn manhattan_distance(p1: (isize, isize), p2: (isize, isize)) -> isize {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn get_cheats(min_map: HashMap<(isize, isize), Node>, max_dist: isize, improvement: i128) -> i128 {
    let mut sum = 0;

    // Look at all points within a manhattan distance less than max_dist
    for node in &min_map {
        for neighbor in min_map.clone() {
            // Only check nodes closer than max_dist
            let dist = manhattan_distance(*node.0, neighbor.0);
            if(dist > max_dist) {
                continue;
            }

            if(neighbor.1.weight-node.1.weight-dist as i128 >= improvement) {
                sum += 1;
            }
        }
    }

    return sum;
}

fn do_puzzle(input: &str, cheat_dist: isize, improvement: i128) -> i128 {
    let char_vec: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let map_tuple = create_map(&char_vec);
    let weight_map = map_tuple.0;
    let end = map_tuple.2;

    let map_min = min_map(&weight_map, end);

    for ele in char_vec.iter().enumerate() {
        for ele1 in ele.1.iter().enumerate() {
            let pos = (ele.0 as isize, ele1.0 as isize);
            let map_get = map_min.get(&pos);
            match map_get {
                None => {
                    let format = format!("{:5}",1000);
                    print!("{format}");
                },
                _ => {
                    let format = format!("{:5}",map_get.unwrap().weight);
                    print!("{format}");
                },
            }
        }
        println!();
    }

    return get_cheats(map_min, cheat_dist,improvement);
}

pub fn puzzle1(input: &str) -> i128 {
    return do_puzzle(input, 2, 100);
}

pub fn puzzle2(input: &str) -> i128 {
    return do_puzzle(input, 20, 100);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_20_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 0);
    }

    #[test]
    fn test_day_20_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 1372);
    }

    #[test]
    fn test_day_20_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 0);
    }

    #[test]
    fn test_day_20_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 979014);
    }
}
