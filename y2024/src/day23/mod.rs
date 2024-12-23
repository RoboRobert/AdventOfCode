use std::collections::{HashMap, HashSet};

fn choose(x: i128, y: i128) -> i128 {
    if y > x {
        return 0;
    }
    let y = y.min(x - y); // Use the smaller of y and x - y to minimize calculations
    let mut result = 1;
    for i in 0..y {
        result *= x - i;
        result /= i + 1;
    }
    result
}

pub fn puzzle1(input: &str) -> i128 {
    let mut sum: i128 = 0;
    let mut conn_map: HashMap<&str, HashSet<&str>> = HashMap::new();
    for ele in input.lines() {
        let conn: (&str, &str) = (ele.split('-').nth(0).unwrap(), ele.split('-').nth(1).unwrap());
        // If the map already contains what is being connected to, add it to that entry
        if(conn_map.contains_key(conn.1)) {
            conn_map.entry(conn.1).and_modify(|e| {e.insert(conn.0);});
        
        }
        // Otherwise make a new entry and insert
        else {
            conn_map.entry(conn.0).or_default().insert(conn.1);
            conn_map.get_mut(conn.0).unwrap().insert(conn.0);
        }
        
    }

    // dbg!(&conn_map);

    for ele in conn_map.values() {
        if(ele.len() >=3 ) {
            sum += 1;
            dbg!(ele);
        }
    }

    sum
}

pub fn puzzle2(input: &str) -> i128 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_23_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 7);
    }

    #[test]
    fn test_day_23_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 15006633487);
    }

    #[test]
    fn test_day_23_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 23);
    }

    #[test]
    fn test_day_23_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 1710);
    }
}
