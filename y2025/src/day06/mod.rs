#[derive(Debug, Clone)]
struct Worksheet {
    operator: char,
    operands: Vec<i64>,
}

fn vec_to_i64(char_vec: &Vec<char>) -> i64 {
    let mut sum: i64 = 0;

    for ele in char_vec {
        if ele == &' ' {
            continue;
        }
        sum *= 10;
        sum += ele.to_digit(10).unwrap() as i64;
    }

    sum
}

fn vec_is_empty(char_vec: &Vec<char>) -> bool {
    for ele in char_vec {
        if ele != &' ' {
            return false;
        }
    }

    return true;
}

impl Worksheet {
    fn solve(&self) -> i64 {
        match self.operator {
            '+' => {
                return self.operands.iter().sum::<i64>();
            }
            '*' => {
                return self.operands.iter().product::<i64>();
            }
            _ => return 0,
        }
    }
}

pub fn puzzle1(input: &str) -> i64 {
    let mut sum: i64 = 0;

    let input_vec: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    for i in 0..input_vec[0].len() {
        let mut worksheet: Worksheet = Worksheet {
            operator: ' ',
            operands: vec![],
        };
        for j in 0..input_vec.len() {
            let element = input_vec[j][i].parse::<i64>();
            if element.is_ok() {
                worksheet.operands.push(element.unwrap());
            } else {
                worksheet.operator = input_vec[j][i].chars().nth(0).unwrap();
            }
        }
        sum += worksheet.solve();
    }

    sum
}

pub fn puzzle2(input: &str) -> i64 {
    let mut sum: i64 = 0;

    let input_chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let num_cols = input_chars.len();

    let mut worksheets: Vec<Worksheet> = vec![];
    let mut worksheet: Worksheet = Worksheet {
        operator: '+',
        operands: vec![],
    };
    for i in 0..input_chars[0].len() {
        let operator = input_chars[num_cols - 1][i];
        if operator != ' ' {
            worksheet.operator = operator;
        }
        let mut current_num: Vec<char> = vec![];
        for j in 0..(num_cols - 1) {
            current_num.push(input_chars[j][i]);
        }

        if vec_is_empty(&current_num) {
            worksheets.push(worksheet.clone());
            worksheet.operands = vec![];
        } else {
            worksheet.operands.push(vec_to_i64(&current_num));
        }
    }

    worksheets.push(worksheet);

    for ele in worksheets {
        sum += ele.solve();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_06_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 4277556);
    }

    #[test]
    fn test_day_06_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 3785892992137);
    }

    #[test]
    fn test_day_06_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 3263827);
    }

    #[test]
    fn test_day_06_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 7669802156452);
    }
}
