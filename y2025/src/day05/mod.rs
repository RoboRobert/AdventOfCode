pub fn puzzle1(input: &str) -> i64 {
    let mut sum = 0;

    let mut range_vec: Vec<(i64, i64)> = vec![];
    let mut ingredients_vec: Vec<i64> = vec![];

    for ele in input.lines() {
        if ele == "" {
            continue;
        }
        let ele_int = ele.parse::<i64>();
        if ele_int.is_ok() {
            ingredients_vec.push(ele_int.unwrap());
        } else {
            let ele_range = (
                ele.split("-").nth(0).unwrap().parse::<i64>().unwrap(),
                ele.split("-").nth(1).unwrap().parse::<i64>().unwrap(),
            );
            range_vec.push(ele_range);
        }
    }

    for ingredient in ingredients_vec {
        for range in &range_vec {
            if range.0 <= ingredient && range.1 >= ingredient {
                sum += 1;
                break;
            }
        }
    }

    sum
}

fn has_overlap(range1: (i64, i64), range2: (i64, i64)) -> bool {
    if range2.0 <= range1.0 && range2.1 >= range1.0 {
        return true;
    }
    if range2.0 <= range1.1 && range2.1 >= range1.1 {
        return true;
    }

    return false;
}

fn combine_ranges(range1: (i64, i64), range2: (i64, i64)) -> (i64, i64) {
    return (range1.0.min(range2.0), range1.1.max(range2.1));
}

pub fn puzzle2(input: &str) -> i64 {
    let mut sum = 0;

    let mut range_vec: Vec<(i64, i64)> = vec![];

    for ele in input.lines() {
        if ele == "" {
            break;
        }
        let ele_range = (
            ele.split("-").nth(0).unwrap().parse::<i64>().unwrap(),
            ele.split("-").nth(1).unwrap().parse::<i64>().unwrap(),
        );
        range_vec.push(ele_range);
    }

    // Iteratively combine ranges until there aren't any left to combine
    loop {
        let mut index_updates: Vec<(usize, usize)> = vec![];
        for i in 0..range_vec.len() {
            for j in 0..range_vec.len() {
                // Skip checking overlap for the same element
                if i == j {
                    continue;
                }

                // Solves the gigachad range issue
                let mut add_update = true;
                index_updates.iter().for_each(|update| {
                    if update.0 == i || update.0 == j || update.1 == i || update.1 == j {
                        add_update = false;
                    }
                });

                let range1 = range_vec[i];
                let range2 = range_vec[j];
                if add_update && has_overlap(range1, range2) {
                    index_updates.push((i, j));
                }
            }
        }
        if index_updates.len() == 0 {
            break;
        }

        // Perform squash, retaining original data for now.
        for update in &index_updates {
            let range1 = range_vec[update.0];
            let range2 = range_vec[update.1];
            range_vec.push(combine_ranges(range1, range2));
        }

        // Perform removal
        for update in &index_updates {
            range_vec[update.0] = (-1, -1);
            range_vec[update.1] = (-1, -1);
        }
        range_vec = range_vec
            .into_iter()
            .filter(|item| item.0 != -1 && item.1 != -1)
            .collect();
    }

    for ele in range_vec {
        sum += ele.1 - ele.0 + 1;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_05_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 3);
    }

    #[test]
    fn test_day_05_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 577);
    }

    #[test]
    fn test_day_05_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 14);
    }

    #[test]
    fn test_day_05_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 350513176552950);
    }
}
