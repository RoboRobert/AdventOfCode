#[derive(Debug)]
struct Symbol {
    key: char,
    numbers: Vec<u32>,
}

fn is_symbol(input: char) -> bool {
    if input.is_ascii_digit() || input == '.' {
        return false;
    }

    return true;
}

fn convert_to_characters(input: &str) -> Vec<Vec<char>> {
    let mut i = 0;
    let mut characters: Vec<Vec<char>> = Vec::new();

    //Puts the file into a character array.
    for line in input.lines() {
        characters.push(Vec::new());

        for character in line.chars() {
            characters[i].push(character);
        }

        i += 1;
    }

    return characters;
}

fn get_numbers(input: &Vec<Vec<char>>, row: usize, col: usize) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();

    //Grabs any integers above the current symbol.
    if input[row+1][col].is_ascii_digit() {
        if get_integer(input, row+1, col) > 0 {numbers.push(get_integer(input, row+1, col));}
    }
    else {
        if get_integer(input, row+1, col+1) > 0 {numbers.push(get_integer(input, row+1, col+1));}
        if get_integer(input, row+1, col-1) > 0 {numbers.push(get_integer(input, row+1, col-1));}
    }

    //Grabs any integers below the current symbol.
    if input[row-1][col].is_ascii_digit() {
        if get_integer(input, row-1, col) > 0 {numbers.push(get_integer(input, row-1, col));}
    }
    else {
        if get_integer(input, row-1, col+1) > 0 {numbers.push(get_integer(input, row-1, col+1));}
        if get_integer(input, row-1, col-1) > 0 {numbers.push(get_integer(input, row-1, col-1));}
    }

    //Gets any integers to the sides of the current symbol.
    if get_integer(input, row, col+1) > 0 {numbers.push(get_integer(input, row, col+1));}
    if get_integer(input, row, col-1) > 0 {numbers.push(get_integer(input, row, col-1));}

    return numbers;
}

fn get_integer(input: &Vec<Vec<char>>, row: usize, col: usize) -> u32 {
    //If the character at the input is not a digit, don't continue processing.
    if input[row][col].is_ascii_digit() != true {
        return 0;
    }

    let mut iterator = col;

    let mut return_num: u32 = 0;

    //This positions the iterator at the most significant digit of the integer.
    while iterator >= 0 {
        if input[row][iterator].is_ascii_digit() != true {
            iterator = iterator + 1;
            break;
        }
        if iterator == 0 {
            break;
        }

        iterator -= 1;
    }

    //After the iterator is in the right spot, walks back over and adds the digits.
    while iterator < input.len() {
        if input[row][iterator].is_ascii_digit() != true {
            break;
        }

        return_num *=10;
        return_num = return_num + input[row][iterator].to_digit(10).unwrap();

        iterator += 1;
    }

    return return_num;
}

pub fn puzzle1(input: &str) -> u32 {
    let mut row = 0;
    let mut col = 0;

    let characters = convert_to_characters(input);

    let mut symbol_vec: Vec<Symbol> = Vec::new();
    while row < characters.len() {
        col = 0;
        while col < characters[row].len() {
            //This looks at all the symbols and analyzes digits around them.
            if is_symbol(characters[row][col]) {
                symbol_vec.push(Symbol {key: characters[row][col], numbers: get_numbers(&characters, row, col)});
            }
            col+=1;
        }
        row+=1;
    }

    // Sums all the numbers adjacent to symbols.
    return symbol_vec.iter().map(|symbol| symbol.numbers.clone()).flatten().sum();
}

pub fn puzzle2(input: &str) -> u32 {
    let characters = convert_to_characters(input);

    let mut row = 0;
    let mut col = 0;

    let mut symbol_vec: Vec<Symbol> = Vec::new();
    while row < characters.len() {
        col = 0;
        while col < characters[row].len() {
            //This looks at all the symbols and analyzes digits around them.
            if is_symbol(characters[row][col]) {
                symbol_vec.push(Symbol {key: characters[row][col], numbers: get_numbers(&characters, row, col)});
                dbg!(symbol_vec.last());
            }
            col+=1;
        }
        row+=1;
    }

    return symbol_vec.iter()
        .filter(|symbol| symbol.key.eq(&'*') && symbol.numbers.len().eq(&2))
        .map(|symbol| symbol.numbers.clone())
        .map(|nums| nums[0] * nums[1])
        .sum()
}