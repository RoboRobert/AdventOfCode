use std::fs;

#[derive(Debug)]
struct hand_bid {
    hand: String,
    bid: u64,
}

struct value_tuple(u64, u64);

pub fn puzzle1(input: &str) -> u64 {
    let mut total_winnings = 0;

    let mut hand_vec: Vec<hand_bid> = Vec::new();
    for line in input.lines() {
        let pushed_value: hand_bid = hand_bid { 
            hand: line.split(' ').nth(0).unwrap().to_string(), 
            bid: line.split(' ').nth(1).unwrap().parse::<u64>().unwrap() };
        hand_vec.push(pushed_value);
    }

    //Sorts the hands in order of hand strength
    hand_vec.sort_by(|a, b| find_stronger(&a.hand, &b.hand).0.cmp(&find_stronger(&a.hand, &b.hand).1));

    for iterator in 0..hand_vec.len() {
        let current = (iterator+1) as u64;
        total_winnings+= current*hand_vec[iterator].bid;
    }

    return total_winnings;
}

fn get_type(hand: &str) -> u64 {
    let mut return_value:u64 = 0;
    let mut value_vec: [u64; 15] = [0; 15];

    for character in hand.chars() {
        let iterator: usize = get_value(character) as usize;
        value_vec[iterator] += 1;
    }

    let mut pairs = 0;
    let mut three_of_a_kind = 0;
    for value in value_vec {
        match value {
            2 => {pairs+=1;}
            3 => {three_of_a_kind+=1;}
            4 => {return_value=5;}
            5 => {return_value=6;}
            _ => {}
        }
    }

    //One pair
    if pairs == 1 && three_of_a_kind == 0{
        return_value = 1;
    }

    //Two pair
    else if pairs == 2 {
        return_value = 2;
    }

    //Three of a kind
    else if pairs == 0 && three_of_a_kind == 1{
        return_value = 3;
    }

    //Full house
    else if pairs == 1 && three_of_a_kind == 1{
        return_value = 4;
    }

    return return_value;
}

fn find_stronger(hand_1: &str, hand_2: &str) -> value_tuple {
    let return_value = value_tuple(1,0);

    if get_type(hand_1) > get_type(hand_2) {return value_tuple(1,0);}
    if get_type(hand_2) > get_type(hand_1) {return value_tuple(0,1);}

    let hand_1_vec: Vec<char> = hand_1.chars().collect();
    let hand_2_vec: Vec<char> = hand_2.chars().collect();

    for iterator in 0..hand_1_vec.len() {
        if get_value(hand_1_vec[iterator]) > get_value(hand_2_vec[iterator]) {return value_tuple(1,0);}
        if get_value(hand_2_vec[iterator]) > get_value(hand_1_vec[iterator]) {return value_tuple(0,1);}
    }

    return return_value;
}

fn get_value(card: char) -> u64{
    let mut value = 0;

    if card.is_ascii_digit() {
        return card.to_digit(10).unwrap() as u64;
    }

    match card {
        'T' => {value = 10;}
        'J' => {value = 11;}
        'Q' => {value = 12;}
        'K' => {value = 13;}
        'A' => {value = 14;}
        _ => {println!("Weird!");}
    }

    return value;
}

pub fn puzzle2(input: &str) -> u64 {
    let mut total_winnings = 0;

    let mut hand_vec: Vec<hand_bid> = Vec::new();
    for line in input.lines() {
        let pushed_value: hand_bid = hand_bid { 
            hand: line.split(' ').nth(0).unwrap().to_string(), 
            bid: line.split(' ').nth(1).unwrap().parse::<u64>().unwrap() };
        hand_vec.push(pushed_value);
    }

    //Sorts the hands in order of hand strength
    hand_vec.sort_by(|a, b| find_stronger_2(&a.hand, &b.hand).0.cmp(&find_stronger_2(&a.hand, &b.hand).1));

    for iterator in 0..hand_vec.len() {
        let current = (iterator+1) as u64;
        total_winnings+= current*hand_vec[iterator].bid;
    }

    return total_winnings;
}

fn get_type_2(hand: &str) -> u64 {
    let mut return_value:u64 = 0;
    let mut value_vec: [u64; 15] = [0; 15];

    for character in hand.chars() {
        let iterator: usize = get_value_2(character) as usize;
        value_vec[iterator] += 1;
    }

    //Singles, pairs, three of a kind, four of a kind, five of a kind
    let mut type_array: [u64; 5] = [0; 5];
    for iterator in 2..value_vec.len() {
        let value = value_vec[iterator];
        match value {
            1 => {type_array[0]+=1;}
            2 => {type_array[1]+=1;}
            3 => {type_array[2]+=1;}
            4 => {type_array[3]+=1;}
            5 => {type_array[4]+=1;}
            _ => {}
        }
    }

    //Code that finds where to put the jokers
    let jokers = value_vec[1];
    let mut found: bool = false;
    for iterator in (0..type_array.len()).rev() {
        if jokers == 0 {
            break;
        }
        let jokeri = jokers as usize;
        if type_array[iterator] > 0 {
            type_array[iterator] -=1;
            type_array[iterator + jokeri] += 1;
            found = true;
            break;
        }
    }
    if found == false && jokers > 0 {
        type_array[(jokers-1) as usize] += 1;
    }

    //One pair
    if type_array[1] == 1 {
        return_value = 1;
    }

    //Two pair
    if type_array[1] == 2 {
        return_value = 2;
    }

    //Three of a kind
    if type_array[2] == 1 {
        return_value = 3;
    }

    //Full house
    if type_array[1] == 1 && type_array[2] == 1 {
        return_value = 4;
    }

    //Four of a kind
    if type_array[3] == 1 {
        return_value = 5;
    }

    //Five of a kind
    if type_array[4] == 1 {
        return_value = 6;
    }

    return return_value;
}

fn find_stronger_2(hand_1: &str, hand_2: &str) -> value_tuple {
    let return_value = value_tuple(1,0);

    if get_type_2(hand_1) > get_type_2(hand_2) {return value_tuple(1,0);}
    if get_type_2(hand_2) > get_type_2(hand_1) {return value_tuple(0,1);}

    let hand_1_vec: Vec<char> = hand_1.chars().collect();
    let hand_2_vec: Vec<char> = hand_2.chars().collect();

    for iterator in 0..hand_1_vec.len() {
        if get_value_2(hand_1_vec[iterator]) > get_value_2(hand_2_vec[iterator]) {return value_tuple(1,0);}
        if get_value_2(hand_2_vec[iterator]) > get_value_2(hand_1_vec[iterator]) {return value_tuple(0,1);}
    }

    return return_value;
}

fn get_value_2(card: char) -> u64{
    let mut value = 0;

    if card.is_ascii_digit() {
        return card.to_digit(10).unwrap() as u64;
    }

    match card {
        'J' => {value = 1;}
        'T' => {value = 10;}
        'Q' => {value = 12;}
        'K' => {value = 13;}
        'A' => {value = 14;}
        _ => {println!("Weird!");}
    }

    return value;
}