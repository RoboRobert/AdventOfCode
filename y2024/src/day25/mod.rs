#[derive(Debug)]
struct Key {
    heights: Vec<isize>,
}

#[derive(Debug)]
struct Lock {
    heights: Vec<isize>,
}

fn fits(key: &Key, lock: &Lock) -> bool {
    for i in 0..key.heights.len() {
        if (key.heights[i] + lock.heights[i] > 7) {
            return false;
        }
    }

    return true;
}

pub fn puzzle1(input: &str) -> i128 {
    let mut sum: i128 = 0;

    let mut keys: Vec<Key> = Vec::new();
    let mut locks: Vec<Lock> = Vec::new();

    // Parse the keys and locks
    // First, gather all groups of chars into Strings with all whitespace removed
    let groups: Vec<String> = input
        .split("\n\n")
        .map(|s| s.chars().filter(|c| !c.is_whitespace()).collect())
        .collect();
    for ele in groups {
        let mut height_vec: Vec<isize> = vec![0; 5];
        for character in ele.chars().enumerate() {
            if (character.1 == '#') {
                height_vec[character.0 % 5] += 1;
            }
        }

        if (&ele[0..5] == "#####") {
            locks.push(Lock {
                heights: height_vec,
            });
        } else {
            keys.push(Key {
                heights: height_vec,
            });
        }
    }

    // dbg!(&locks, &keys);

    for key in keys {
        for lock in &locks {
            if (fits(&key, lock)) {
                sum += 1;
            }
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
    fn test_day_25_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 3);
    }

    #[test]
    fn test_day_25_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 2770);
    }

    #[test]
    fn test_day_25_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 23);
    }

    #[test]
    fn test_day_25_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 1710);
    }
}
