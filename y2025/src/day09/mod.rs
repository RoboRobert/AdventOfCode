#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    p1: Point,
    p2: Point,
}

#[derive(Debug, Clone)]
struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    fn contains(&self, edge: Edge) -> bool {
        return true;
    }
}

pub fn puzzle1(input: &str) -> i64 {
    let mut max_area = 0;

    let point_vec: Vec<Point> = input
        .lines()
        .map(|line| {
            let x = line.split(",").nth(0).unwrap().parse::<i64>().unwrap();
            let y = line.split(",").nth(1).unwrap().parse::<i64>().unwrap();

            let point: Point = Point { x: x, y: y };

            return point;
        })
        .collect();

    let mut edge_vec: Vec<Edge> = vec![];

    edge_vec.push(Edge {
        p1: point_vec[0],
        p2: *point_vec.last().unwrap(),
    });

    for i in 0..point_vec.len() {
        for j in i + 1..point_vec.len() {
            edge_vec.push(Edge {
                p1: point_vec[i],
                p2: point_vec[j],
            })
        }
    }

    for edge in edge_vec {
        let side_1 = (edge.p1.x.abs() - edge.p2.x.abs()).abs() + 1;
        let side_2 = (edge.p1.y.abs() - edge.p2.y.abs()).abs() + 1;

        let area = side_1 * side_2;
        if area as i64 > max_area {
            max_area = area as i64;
        }
    }

    max_area
}

pub fn puzzle2(input: &str) -> i64 {
    let mut max_area = 0;

    max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_09_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 50);
    }

    #[test]
    fn test_day_09_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 4759531084);
    }

    #[test]
    fn test_day_09_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 24);
    }

    #[test]
    fn test_day_09_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 8995844880);
    }
}
