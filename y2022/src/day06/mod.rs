fn is_marker(packet: &str, length: usize) -> bool {
    if(packet.len() != length) {
        return false;
    }

    for ele in packet.chars() {
        if(packet.matches(ele).count() != 1) {
            return false;
        }
    }

    return true;
}

fn do_puzzle(input: &str, length: usize) -> i128 {
    let mut pos: usize = 0;
    while(pos < input.len()) {
        let packet = &input[pos..pos+length];
        if(is_marker(packet, length)) {
            pos += length;
            break;
        }
        pos += 1;
    }

    return pos as i128;
}

pub fn puzzle1(input: &str) -> i128 {
    do_puzzle(input, 4)
}

pub fn puzzle2(input: &str) -> i128 {
    do_puzzle(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_06_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 11);
    }

    #[test]
    fn test_day_06_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 1034);
    }

    #[test]
    fn test_day_06_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 26);
    }

    #[test]
    fn test_day_06_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 2472);
    }
}
