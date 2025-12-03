fn get_factors(input: i64) -> Vec<i64> {
    let mut factor_vec: Vec<i64> = vec![];
    for i in 1..input {
        if input % i == 0 {
            factor_vec.push(i);
        }
    }

    return factor_vec;
}

pub fn puzzle1(input: &str) -> i64 {
    let mut sum: i64 = 0;
    let ranges: Vec<(i64, i64)> = input
        .split(",")
        .map(|current| {
            let mut split = current.split("-");
            return (
                split.nth(0).unwrap().parse::<i64>().unwrap(),
                split.nth(0).unwrap().parse::<i64>().unwrap(),
            );
        })
        .collect();

    for range in ranges {
        let lower = range.0;
        let upper = range.1;

        // Only look at ranges that contain numbers with an even amount of digits
        if lower.to_string().len() % 2 == 0 || upper.to_string().len() % 2 == 0 {
            for i in lower..=upper {
                let i_as_str = i.to_string();
                let split = i_as_str.split_at(i_as_str.len() / 2);
                if split.0 == split.1 {
                    sum += i;
                }
            }
        }
    }

    sum
}

pub fn puzzle2(input: &str) -> i64 {
    let mut sum: i64 = 0;
    let ranges: Vec<(i64, i64)> = input
        .split(",")
        .map(|current| {
            let mut split = current.split("-");
            return (
                split.nth(0).unwrap().parse::<i64>().unwrap(),
                split.nth(0).unwrap().parse::<i64>().unwrap(),
            );
        })
        .collect();

    for range in ranges {
        let lower = range.0;
        let upper = range.1;

        for i in lower..=upper {
            let mut invalid = false;
            let i_as_str = i.to_string();

            let factor_vec = get_factors(i_as_str.len().try_into().unwrap());
            'outer: for factor in factor_vec {
                let mut chunks: Vec<&str> = vec![];
                let mut cur = i_as_str.as_str();
                while !cur.is_empty() {
                    let (chunk, rest) = cur.split_at(factor.try_into().unwrap());
                    chunks.push(chunk);
                    cur = rest;
                }
                let chunk_1 = chunks[0];
                if chunks.iter().all(|chunk| *chunk == chunk_1) {
                    invalid = true;
                    break 'outer;
                }
            }

            if invalid == true {
                sum += i;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_02_puzzle1_example1() {
        assert_eq!(puzzle1(EXAMPLE), 1227775554);
    }

    #[test]
    fn test_day_02_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 31839939622);
    }

    #[test]
    fn test_day_02_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 4174379265);
    }

    #[test]
    fn test_day_02_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 41662374059);
    }
}
