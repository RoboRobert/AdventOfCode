use regex::{self, Captures, Regex};

pub fn puzzle1(input: &str) -> i128 {
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let muls: Vec<Captures> = re.captures_iter(input).collect();

    return muls.iter().map(|capture| {
        capture.get(0).unwrap().as_str()}).map(|string | {
            let mut split = string.split(['(',',',')']);
            split.nth(1).unwrap().parse::<i128>().unwrap()
            *split.nth(2).unwrap().parse::<i128>().unwrap()}).sum();
}

pub fn puzzle2(input: &str) -> i128 {
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\)").unwrap();
    let muls: Vec<Captures> = re.captures_iter(input).collect();

    let mut doBool: bool = true;
    let mut sum: i128 = 0;

    for ele in muls {
        let current: &str = ele.get(0).unwrap().as_str();
        match(current) {
            "do()" => {doBool = true;}
            "don't()" => {doBool = false;}
            _ => {
                if(doBool) {
                    sum += current.split(['(',',',')']).nth(1).unwrap().parse::<i128>().unwrap()*current.split(['(',',',')']).nth(2).unwrap().parse::<i128>().unwrap();
                }
            }
        }
    }

    return sum;
}