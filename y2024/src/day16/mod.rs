use std::{cmp::Ordering, collections::{HashMap, HashSet}};
#[derive(Debug, Eq, Clone)]
struct Node {
    pos: (isize, isize),
    prevs: HashSet<(isize, isize)>,
    dir: (isize, isize),
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
fn min_map(weight_map: &HashMap<(isize, isize), Node>) -> HashMap<(isize, isize), Node> {
    let mut map_mut: HashMap<(isize, isize), Node> = weight_map.clone();
    let mut seen_map: HashMap<(isize, isize), Node> = HashMap::new();

    // Loop until all nodes are visited
    while(map_mut.len() > 0) {
        // Pops the unvisited node with lowest weight
        let current = map_mut.values().min().unwrap().clone();
        let pos = current.pos;

        // Moves current node from visited to unvisited
        map_mut.remove(&pos);
        seen_map.insert(pos, current.clone());

        // Direction vector with up, down, left, right
        let dir_vec: Vec<(isize, isize)> = vec![(-1, 0),(1,0),(0,-1), (0,1)];
        // Goes through all neighbors of the current node using the dir_vec
        for new_dir in dir_vec {
            let n_pos = (pos.0 + new_dir.0, pos.1 + new_dir.1);
            let neighbor = map_mut.get(&n_pos);
            match neighbor {
                // If the neighbor isn't in the graph, do nothing
                None => continue,
                _ => {},
            }
            
            let n_unwrap = neighbor.unwrap();
            let mut n_update = Node{pos: n_pos, prevs: current.prevs.clone(), dir: new_dir, weight: 0};

            if(new_dir != current.dir) {
                n_update.weight += 1001;
            }
            else {
                n_update.weight += 1;
            }
            n_update.weight += current.weight;

            // If the new weight is the same or lower, replace its weight in the unvisited list
            if(n_update.weight <= n_unwrap.weight) {
                n_update.prevs.insert(current.pos);
                map_mut.insert(n_update.pos, n_update);
            }
        }
    }

    return seen_map;
}

// Creates a map of Nodes from a 2D Vector of chars and returns start and end positions
fn create_map(char_vec: Vec<Vec<char>>) -> (HashMap<(isize, isize), Node>,(isize, isize),(isize, isize)) {
    let mut weight_map: HashMap<(isize, isize), Node> = HashMap::new();
    let mut start: (isize, isize) = (0,0);
    let mut end: (isize, isize) = (0,0);
    for row in char_vec.iter().enumerate() {
        for col in row.1.iter().enumerate() {
            let pos = (row.0 as isize, col.0 as isize);
            let mut node: Node = Node{pos: (pos), prevs: HashSet::new(), dir: (0,0), weight: 0};

            // Insert everything except the walls
            match *col.1 {
                // Start faces East (0,1)
                'S' => {node = Node{pos: (pos), prevs: HashSet::new(), dir: (0,1), weight: 0}; start = pos;}
                'E' => {node = Node{pos: (pos), prevs: HashSet::new(), dir: (0,0), weight: i128::MAX}; end = pos;}
                '.' => node = Node{pos: (pos), prevs: HashSet::new(), dir: (0,0), weight: i128::MAX},
                '#' => continue,
                _ => {dbg!("WEIRD");}
            }
            weight_map.insert(pos, node);
        }
    }

    return (weight_map, start, end);
}

pub fn puzzle1(input: &str) -> i128 {
    let char_vec: Vec<Vec<char>> = input.split("\n\n").nth(0).unwrap().lines().map(|line| line.chars().collect()).collect();

    let map_tuple = create_map(char_vec);
    let weight_map = map_tuple.0;
    let end = map_tuple.2;

    return min_map(&weight_map).get(&end).unwrap().weight;
}

pub fn puzzle2(input: &str) -> i128 {
    let char_vec: Vec<Vec<char>> = input.split("\n\n").nth(0).unwrap().lines().map(|line| line.chars().collect()).collect();

    let map_tuple = create_map(char_vec);
    let weight_map = map_tuple.0;
    let end = map_tuple.2;

    let new_map = min_map(&weight_map);
    let prevs = new_map.get(&end).unwrap().prevs.clone();
    
    dbg!(&prevs);

    return prevs.len() as i128;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_16_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 11048);
    }

    #[test]
    fn test_day_16_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 78428);
    }

    #[test]
    fn test_day_16_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 81);
    }

    #[test]
    fn test_day_16_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 1366);
    }
}
