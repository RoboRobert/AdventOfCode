use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn is_on(&self, edge: Edge) -> bool {
        let x_min = edge.p1.x.min(edge.p2.x);
        let x_max = edge.p1.x.max(edge.p2.x);
        let y_min = edge.p1.y.min(edge.p2.y);
        let y_max = edge.p1.y.max(edge.p2.y);

        if self.x >= x_min && self.x <= x_max && self.y >= y_min && self.y <= y_max {
            return true;
        }

        return false;
    }
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    p1: Point,
    p2: Point,
}

impl Edge {
    fn rectangle_area(&self) -> i64 {
        let side_1 = (self.p1.x - self.p2.x).abs() + 1;
        let side_2 = (self.p1.y - self.p2.y).abs() + 1;

        return side_1 * side_2;
    }
}

#[derive(Debug, Clone)]
struct Polygon {
    _points: Vec<Point>,
    edges: Vec<Edge>,
}

impl Polygon {
    fn new(points: &Vec<Point>) -> Polygon {
        let mut edge_vec: Vec<Edge> = vec![];

        edge_vec.push(Edge {
            p1: *points.last().unwrap(),
            p2: *points.first().unwrap(),
        });

        for i in 0..points.len() - 1 {
            edge_vec.push(Edge {
                p1: points[i],
                p2: points[i + 1],
            })
        }

        return Polygon {
            _points: points.to_owned(),
            edges: edge_vec,
        };
    }
    fn contains(&self, point: Point) -> bool {
        let mut num_hits = 0;

        for edge in &self.edges {
            if point.is_on(*edge) {
                return true;
            }
            let x_max = edge.p1.x.max(edge.p2.x);
            let y_max = edge.p1.y.max(edge.p2.y);
            let y_min = edge.p1.y.min(edge.p2.y);
            // Check edges to the left
            if point.x >= x_max && y_min <= point.y && y_max >= point.y {
                num_hits += 1;
            }
        }

        if num_hits % 2 != 0 {
            return true;
        }
        return false;
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

    for i in 0..point_vec.len() {
        for j in i + 1..point_vec.len() {
            edge_vec.push(Edge {
                p1: point_vec[i],
                p2: point_vec[j],
            })
        }
    }

    for edge in edge_vec {
        let area = edge.rectangle_area();
        if area as i64 > max_area {
            max_area = area as i64;
        }
    }

    max_area
}

pub fn puzzle2(input: &str) -> i64 {
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

    let polygon: Polygon = Polygon::new(&point_vec);

    let mut combinations: Vec<Edge> = vec![];
    for i in 0..point_vec.len() {
        for j in i..point_vec.len() {
            combinations.push(Edge {
                p1: point_vec[i],
                p2: point_vec[j],
            })
        }
    }

    let mut contained_set: HashSet<Point> = HashSet::new();

    for rectangle in combinations {
        // Skip smol rectangles.
        let area = rectangle.rectangle_area();
        if area <= max_area {
            continue;
        }

        let point1 = rectangle.p1;
        let point2 = rectangle.p2;

        let point3: Point = Point {
            x: rectangle.p1.x,
            y: rectangle.p2.y,
        };
        let point4: Point = Point {
            x: rectangle.p2.x,
            y: rectangle.p1.y,
        };

        let mut contained: bool = true;

        let rect_edges: Vec<Edge> = vec![
            Edge {
                p1: point1,
                p2: point3,
            },
            Edge {
                p1: point1,
                p2: point4,
            },
            Edge {
                p1: point2,
                p2: point3,
            },
            Edge {
                p1: point2,
                p2: point4,
            },
        ];

        'outer: for edge in rect_edges {
            let x_min = edge.p1.x.min(edge.p2.x);
            let x_max = edge.p1.x.max(edge.p2.x);
            let y_min = edge.p1.y.min(edge.p2.y);
            let y_max = edge.p1.y.max(edge.p2.y);
            for x_test in x_min..=x_max {
                for y_test in y_min..=y_max {
                    // dbg!(x_test, y_test);
                    let test_point = Point {
                        x: x_test,
                        y: y_test,
                    };
                    if contained_set.contains(&test_point) {
                        continue;
                    }
                    if polygon.contains(test_point) {
                        contained_set.insert(test_point);
                    } else {
                        contained = false;
                        break 'outer;
                    }
                }
            }
        }

        if contained {
            dbg!(max_area);
            max_area = area;
        }
    }

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
        assert_eq!(puzzle2(INPUT), 1539238860);
    }
}
