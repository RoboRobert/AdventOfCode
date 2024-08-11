pub fn puzzle1(input: &str) -> u32 {
    let mut i = 0;

    let mut sum = 0;

    let mut characters: Vec<Vec<char>> = Vec::new();

    //Puts the file into a character array.
    for line in input.lines() {
        characters.push(Vec::new());

        for character in line.chars() {
            print!("{}",character);
            characters[i].push(character);
        }

        i += 1;
        println!("");
    }

    let mut row = 0;
    let mut col = 0;
    while row < characters.len() {
        col = 0;
        while col < characters[row].len() {
            
            //This looks at all the symbols and analyzes digits around them.
            if is_symbol(characters[row][col]) {

                //Grabs any integers above the current symbol.
                if characters[row+1][col].is_ascii_digit() {
                    sum += get_integer(&characters, row+1, col);
                }
                else {
                    sum += get_integer(&characters, row+1, col+1);
                    sum += get_integer(&characters, row+1, col-1);
                }

                //Grabs any integers below the current symbol.
                if characters[row-1][col].is_ascii_digit() {
                    sum += get_integer(&characters, row-1, col);
                }
                else {
                    sum += get_integer(&characters, row-1, col+1);
                    sum += get_integer(&characters, row-1, col-1);
                }
            
                //Gets any integers to the sides of the current symbol.
                sum += get_integer(&characters, row, col+1);
                sum += get_integer(&characters, row, col-1);

            }
            col+=1;
        }
        
        row+=1;
    }

    return sum;
}


fn is_symbol(input: char) -> bool {
    if input.is_ascii_digit() {
        return false;
    }
    else if input == '.' {
        return false;
    }

    return true;
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

pub fn puzzle2(input: &str) -> u32 {
    let mut i = 0;

    let mut sum = 0;

    let mut characters: Vec<Vec<char>> = Vec::new();

    //Puts the file into a character array.
    for line in input.lines() {
        characters.push(Vec::new());

        for character in line.chars() {
            print!("{}",character);
            characters[i].push(character);
        }

        i += 1;
        println!("");
    }

    let mut row = 0;
    let mut col = 0;
    while row < characters.len() {
        col = 0;
        while col < characters[row].len() {
            
            //This looks at all the symbols and analyzes digits around them.
            if characters[row][col] == '*' {
                let mut adjacent_values: Vec<u32> = Vec::new();

                dbg!(characters[row][col]);
                
                //Grabs any integers above the current symbol.
                if characters[row+1][col].is_ascii_digit() {
                    adjacent_values.push(get_integer(&characters, row+1, col));
                }
                else {
                    adjacent_values.push(get_integer(&characters, row+1, col+1));
                    adjacent_values.push(get_integer(&characters, row+1, col-1));
                }

                //Grabs any integers below the current symbol.
                if characters[row-1][col].is_ascii_digit() {
                    adjacent_values.push(get_integer(&characters, row-1, col));
                }
                else {
                    adjacent_values.push(get_integer(&characters, row-1, col+1));
                    adjacent_values.push(get_integer(&characters, row-1, col-1));
                }
            
                //Gets any integers to the sides of the current symbol.
                adjacent_values.push(get_integer(&characters, row, col+1));
                adjacent_values.push(get_integer(&characters, row, col-1));

                //For any gear with exactly two adjacencies, multiply them together and
                //add the sum.
                let mut num_adjacent: u32 = 0;
                let mut current: u32 = 1;
                for integer in adjacent_values {
                    if integer > 0 {
                        current *= integer;
                        num_adjacent+= 1;
                    }
                }

                //If the vector only contained two integers, add it to the sum.
                if num_adjacent == 2 {
                    sum += current;
                }

            }
            col+=1;
        }
        
        row+=1;
    }

    return sum;
}