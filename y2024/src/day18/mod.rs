use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
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
fn min_map(weight_map: &HashMap<(isize, isize), Node>) -> HashMap<(isize, isize), Node> {
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

// Creates a map of nodes from a Vec of (isize,isize)
fn create_map(pos_vec: Vec<(isize, isize)>, bounds: (isize, isize)) -> HashMap<(isize, isize), Node> {
    let mut weight_map: HashMap<(isize, isize), Node> = HashMap::new();

    for i in 0..=bounds.0 {
        for j in 0..=bounds.1 {
            let node: Node = Node {
                pos: (i,j),
                weight: i128::MAX-10,
            };
            weight_map.insert((i,j), node);
        }
    }

    // Remove any entries that have a byte on them
    for ele in pos_vec {
        weight_map.remove(&ele);
    }

    return weight_map;
}

pub fn puzzle1(input: &str) -> i128 {
    let byte_vec: Vec<(isize, isize)> = input
        .lines()
        .map(|line| {
            (
                line.split(',').nth(0).unwrap().parse::<isize>().unwrap(),
                line.split(',').nth(1).unwrap().parse::<isize>().unwrap(),
            )
        })
        .collect();

    let start: (isize, isize) = (0,0);

    // Regular stuff
    let end: (isize, isize) = (70,70);
    let small_vec = byte_vec[0..1024].to_vec();

    // Example stuff
    // let end: (isize, isize) = (6,6);
    // let small_vec = byte_vec[0..12].to_vec();

    let mut start_map = create_map(small_vec, end);
    start_map.insert(start, Node{pos:start, weight:0});
    let map_min = min_map(&start_map);

    

    return map_min.get(&end).unwrap().weight as i128;
}

pub fn puzzle2(input: &str) -> i128 {
    let mut byte_vec: Vec<(isize, isize)> = input
        .lines()
        .map(|line| {
            (
                line.split(',').nth(0).unwrap().parse::<isize>().unwrap(),
                line.split(',').nth(1).unwrap().parse::<isize>().unwrap(),
            )
        })
        .collect();

    // Reverse the byte vec so I can just pop off it
    byte_vec.reverse();

    let start: (isize, isize) = (0,0);

    // Regular stuff
    let end: (isize, isize) = (70,70);

    // Example stuff
    // let end: (isize, isize) = (6,6);

    let mut start_map = create_map(Vec::new(), end);
    start_map.insert(start, Node{pos:start, weight:0});
    let mut current_byte = (0,0);
    let mut map_min = min_map(&start_map);
    while(map_min.get(&end).unwrap().weight< 10000000000) {
        current_byte = byte_vec.pop().unwrap();
        start_map.remove(&current_byte);
        // for i in 0..=end.0 {
        //     for j in 0..=end.1 {
        //         if(start_map.get(&(j,i)).is_some()) {
        //             print!(".");
        //         }
        //         else {print!("#");}
        //     }
        //     println!();
        // }
        // println!("\n\n");
        map_min = min_map(&start_map).clone();
        
    }

    dbg!(current_byte);

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_18_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 11048);
    }

    #[test]
    fn test_day_18_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 78428);
    }

    #[test]
    fn test_day_18_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 81);
    }

    #[test]
    fn test_day_18_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 1366);
    }
}
