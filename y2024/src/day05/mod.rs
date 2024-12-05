#[derive(Debug, PartialEq)]
struct Rule {
    first: i128,
    last: i128,
}

pub fn puzzle1(input: &str) -> i128 {
    let mut sum: i128 = 0;

    // Parse out all the rules
    let mut rules_vec: Vec<Rule> = input
        .split("\n\n")
        .nth(0)
        .unwrap()
        .lines()
        .map(|line| Rule {
            first: line.split('|').nth(0).unwrap().parse::<i128>().unwrap(),
            last: line.split('|').nth(1).unwrap().parse::<i128>().unwrap(),
        })
        .collect();

    // Parse out all the updates
    let mut update_vec: Vec<Vec<i128>> = input
        .split("\n\n")
        .nth(1)
        .unwrap()
        .lines()
        .map(|line| {
            line.split(',')
                .map(|split| split.parse::<i128>().unwrap())
                .collect()
        })
        .collect();
        
    for ele in update_vec {
        let mut good = true;
        for rule in &rules_vec {
            let pos1 = ele.iter().position(|&e| e == rule.first);
            let pos2 =  ele.iter().position(|&e| e == rule.last);
            if pos1 == None || pos2 == None {
                continue;
            }
            if(pos1.unwrap() >= pos2.unwrap()) {
                good = false;
                break;
            }
        }

        if(good) {
            sum += ele.get(ele.len()/2).unwrap();
        }
    }

    return sum;
}

pub fn puzzle2(input: &str) -> i128 {
    let mut sum: i128 = 0;

    // Parse out all the rules
    let mut rules_vec: Vec<Rule> = input
        .split("\n\n")
        .nth(0)
        .unwrap()
        .lines()
        .map(|line| Rule {
            first: line.split('|').nth(0).unwrap().parse::<i128>().unwrap(),
            last: line.split('|').nth(1).unwrap().parse::<i128>().unwrap(),
        })
        .collect();

    // Parse out all the updates
    let mut update_vec: Vec<Vec<i128>> = input
        .split("\n\n")
        .nth(1)
        .unwrap()
        .lines()
        .map(|line| {
            line.split(',')
                .map(|split| split.parse::<i128>().unwrap())
                .collect()
        })
        .collect();
        
    for mut ele in update_vec {
        let mut good = true;
        let mut index: usize = 0;
        while index < rules_vec.len() {
            let rule = &rules_vec[index];
            let pos1 = ele.iter().position(|&e| e == rule.first);
            let pos2 =  ele.iter().position(|&e| e == rule.last);
            if pos1 == None || pos2 == None {
                index +=1;
                continue;
            }
            if(pos1.unwrap() >= pos2.unwrap()) {
                good = false;
                // After the swap, all previous rules must be checked again
                ele.swap(pos1.unwrap(), pos2.unwrap());
                index = 0;
            }
            index +=1;
        }
        
        if(!good) {
            sum += ele.get(ele.len()/2).unwrap();
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_05_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 143);
    }

    #[test]
    fn test_day_05_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 6034);
    }

    #[test]
    fn test_day_05_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 123);
    }

    #[test]
    fn test_day_05_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 6305);
    }
}
