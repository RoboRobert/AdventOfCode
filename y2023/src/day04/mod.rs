pub fn puzzle1(input: &str) -> i128 {
    let mut sum: i128 = 0;
    for current in input.lines() {
        //This stores all the winning numbers in a u32 vector
        let winning_vec: Vec<i128> =  current.split("|").collect::<Vec<&str>>()[1].split_whitespace().map(|digit| digit.parse::<i128>().unwrap()).collect();

        //This stores all the held numbers in a u32 vector
        let held_vec: Vec<i128> = current.split("|").collect::<Vec<&str>>()[0].split(":").collect::<Vec<&str>>()[1].split_whitespace().map(|digit| digit.parse::<i128>().unwrap()).collect();

        let vec: Vec<&i128> = held_vec.iter().filter(|num| winning_vec.contains(num)).collect();
        let num_winners: u32 = vec.len() as u32;
        let base: i128 = 2;

        if num_winners > 0 {sum += base.pow(num_winners-1);}
    }

    return sum;
}

pub fn puzzle2(input: &str) -> u32 {
    let mut num_card_type: Vec<u32> = Vec::new();
    let mut num_matches: Vec<u32> = Vec::new();

    for current in input.lines() {
        num_card_type.push(1);

        //This stores all the winning numbers in a u32 vector
        let winning_vec: Vec<u32> =  current.split("|").collect::<Vec<&str>>()[1].split_whitespace().map(|digit| digit.parse::<u32>().unwrap()).collect();

        //This stores all the held numbers in a u32 vector
        let held_vec: Vec<u32> = current.split("|").collect::<Vec<&str>>()[0].split(":").collect::<Vec<&str>>()[1].split_whitespace().map(|digit| digit.parse::<u32>().unwrap()).collect();

        let vec: Vec<&u32> = held_vec.iter().filter(|num| winning_vec.contains(num)).collect();
        let num_winners: u32 = vec.len() as u32;
        //Pushes the number of matches for each card type
        num_matches.push(num_winners);
    }
    
    for (iteration, number) in num_matches.into_iter().enumerate() {
        for inner in (iteration + 1)..=(iteration + number as usize) {
            num_card_type[inner] += num_card_type[inner];
        }
    }

    dbg!(&num_card_type);

    return num_card_type.iter().sum()
}