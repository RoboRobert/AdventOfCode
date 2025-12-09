use std::collections::HashSet;

use num::pow;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coordinate {
    x: i64,
    y: i64,
    z: i64,
}

impl Coordinate {
    fn distance(&self, other: Coordinate) -> i64 {
        return pow(self.x - other.x, 2) + pow(self.y - other.y, 2) + pow(self.z - other.z, 2);
    }
    fn equals(&self, other: Coordinate) -> bool {
        return self.x == other.x && self.y == other.y && self.z == other.z;
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Edge {
    coord1: Coordinate,
    coord2: Coordinate,
    distance: i64,
}

impl Edge {
    fn connects(&self, other: Edge) -> bool {
        return self.coord1.equals(other.coord1)
            || self.coord1.equals(other.coord2)
            || self.coord2.equals(other.coord1)
            || self.coord2.equals(other.coord2);
    }
}

fn do_puzzle(input: &str, puzzle_num: i64) -> (i64, i64) {
    let mut largest_circuits = 1;
    let mut final_coords: i64 = 0;

    let nodes: Vec<Coordinate> = input
        .lines()
        .map(|line| {
            let mut split = line.split(",");
            return Coordinate {
                x: split.nth(0).unwrap().parse::<i64>().unwrap(),
                y: split.nth(0).unwrap().parse::<i64>().unwrap(),
                z: split.nth(0).unwrap().parse::<i64>().unwrap(),
            };
        })
        .collect();

    let mut circuits: Vec<HashSet<Edge>> = vec![HashSet::new()];
    let mut edges: Vec<Edge> = vec![];

    for i in 0..nodes.len() {
        for j in i + 1..nodes.len() {
            let node1 = nodes[i];
            let node2 = nodes[j];
            edges.push(Edge {
                coord1: node1,
                coord2: node2,
                distance: node1.distance(node2),
            });
        }
    }

    let mut deduped: HashSet<Coordinate> = HashSet::new();

    edges.sort_by(|a, b| b.distance.cmp(&a.distance));

    let mut iterations = 1000;
    if puzzle_num == 2 {
        iterations = edges.len();
    }

    for _ in 0..iterations {
        let mut additions: Vec<usize> = vec![];
        let added_edge = edges.pop().unwrap();
        for j in 0..circuits.len() {
            let disjoint_set = &circuits[j];
            for edge in disjoint_set {
                if added_edge.connects(*edge) {
                    additions.push(j);
                }
            }
        }

        if additions.len() > 0 {
            for k in 0..additions.len() {
                circuits[additions[k]].insert(added_edge);
            }
        }

        if additions.len() > 1 {
            let mut total_set: HashSet<Edge> = HashSet::new();
            for k in 0..additions.len() {
                total_set.extend(circuits[additions[k]].clone().into_iter());
                circuits[additions[k]] = HashSet::new();
            }

            circuits = circuits
                .iter()
                .filter(|item| !item.is_empty())
                .cloned()
                .collect();

            circuits.push(total_set);
        }

        if additions.len() == 0 {
            circuits.push(HashSet::from([added_edge]));
        }

        if puzzle_num == 2 {
            deduped.insert(added_edge.coord1);
            deduped.insert(added_edge.coord2);

            if deduped.len() == nodes.len() {
                final_coords = added_edge.coord1.x * added_edge.coord2.x;
                break;
            }
        }
    }

    if puzzle_num == 1 {
        let mut deduped_sets: Vec<HashSet<Coordinate>> = vec![];
        for i in 0..circuits.len() {
            deduped_sets.push(HashSet::new());
            for edge in &circuits[i] {
                deduped_sets[i].insert(edge.coord1);
                deduped_sets[i].insert(edge.coord2);
            }
        }

        deduped_sets.sort_by(|set1, set2| set1.len().cmp(&set2.len()));

        for _ in 0..3 {
            let ele = deduped_sets.pop().unwrap();
            largest_circuits *= (ele.len()) as i64;
        }
    }

    (largest_circuits, final_coords)
}

pub fn puzzle1(input: &str) -> i64 {
    do_puzzle(input, 1).0
}

pub fn puzzle2(input: &str) -> i64 {
    do_puzzle(input, 2).1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_08_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 0);
    }

    #[test]
    fn test_day_08_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 75680);
    }

    #[test]
    fn test_day_08_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 0);
    }

    #[test]
    fn test_day_08_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 8995844880);
    }
}
