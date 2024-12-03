use regex::{self, Captures, Regex};

pub fn puzzle1(input: &str) -> i128 {
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let matches: Vec<Captures> = re.captures_iter(input).collect();

    return matches.iter().map(|capture| {
        capture.get(0).unwrap().as_str()}).map(|string | {
            let split: Vec<&str> = string.split(['(',',',')']).collect();
            split[1].parse::<i128>().unwrap()
            *split[2].parse::<i128>().unwrap()}).sum();
}

pub fn puzzle2(input: &str) -> i128 {
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\)").unwrap();
    let matches: Vec<Captures> = re.captures_iter(input).collect();

    let mut doBool: bool = true;
    let mut sum: i128 = 0;
    for ele in matches {
        let current: &str = ele.get(0).unwrap().as_str();
        match(current) {
            "do()" => {doBool = true;}
            "don't()" => {doBool = false;}
            _ => {
                if(doBool) {
                    let split: Vec<&str> = current.split(['(',',',')']).collect();
                    sum += split[1].parse::<i128>().unwrap()*split[2].parse::<i128>().unwrap();
                }
            }
        }
    }

    return sum;
}