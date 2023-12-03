use std::vec::IntoIter;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let input = std::fs::read_to_string("src/part-1.input").unwrap();

    let lines: Vec<&str> = input.split("\n").collect();

    let mut sum = 0;
    for line in lines {
        let mut chars = line.chars().collect::<Vec<char>>().into_iter();
        let first_number = find_digit(&mut chars);
        let mut reversed_chars = line.chars().rev().collect::<Vec<char>>().into_iter();
        let last_number = find_digit(&mut reversed_chars);
        let combination = format!("{}{}", first_number, last_number);
        sum += combination.parse::<i32>().unwrap();
    }

    println!("Result: {:?}", sum);
}

fn find_digit(chars: &mut IntoIter<char>) -> u32 {
    let mut number = 0;
    while let Some(char) = chars.next() {
        if let Some(digit) = char.to_digit(10) {
            number = digit;
            break;
        }
    }

    number
}

fn part_2() {
    let lines: Vec<&str> = input.split("\n").collect();

    let mut sum = 0;
    for line in lines {
        let mut chars = line.chars().collect::<Vec<char>>().into_iter();
        let first_number = find_digit_2(&mut chars);
        let mut reversed_chars = line.chars().rev().collect::<Vec<char>>().into_iter();
        let last_number = find_digit_2_rev(&mut reversed_chars);
        let combination = format!("{}{}", first_number, last_number);
        sum += combination.parse::<i32>().unwrap();
    }

    println!("Result: {:?}", sum);
}

fn find_digit_2(chars: &mut IntoIter<char>) -> u32 {
    let mut number = 0;
    let mut previous_chars = Vec::new();
    while let Some(char) = chars.next() {
        if let Some(digit) = char.to_digit(10) {
            number = digit;
            break;
        }
        if char == 'e' {
            if previous_chars.len() >= 2
                && previous_chars[previous_chars.len() - 1] == 'n'
                && previous_chars[previous_chars.len() - 2] == 'o'
            {
                number = 1;
                break;
            }
            if previous_chars.len() >= 4
                && previous_chars[previous_chars.len() - 1] == 'e'
                && previous_chars[previous_chars.len() - 2] == 'r'
                && previous_chars[previous_chars.len() - 3] == 'h'
                && previous_chars[previous_chars.len() - 4] == 't'
            {
                number = 3;
                break;
            }
            if previous_chars.len() >= 3
                && previous_chars[previous_chars.len() - 1] == 'v'
                && previous_chars[previous_chars.len() - 2] == 'i'
                && previous_chars[previous_chars.len() - 3] == 'f'
            {
                number = 5;
                break;
            }
            if previous_chars.len() >= 3
                && previous_chars[previous_chars.len() - 1] == 'n'
                && previous_chars[previous_chars.len() - 2] == 'i'
                && previous_chars[previous_chars.len() - 3] == 'n'
            {
                number = 9;
                break;
            }
        }
        if char == 'o' {
            if previous_chars.len() >= 2
                && previous_chars[previous_chars.len() - 1] == 'w'
                && previous_chars[previous_chars.len() - 2] == 't'
            {
                number = 2;
                break;
            }
        }
        if char == 'r' {
            if previous_chars.len() >= 3
                && previous_chars[previous_chars.len() - 1] == 'u'
                && previous_chars[previous_chars.len() - 2] == 'o'
                && previous_chars[previous_chars.len() - 3] == 'f'
            {
                number = 4;
                break;
            }
        }
        if char == 'x' {
            if previous_chars.len() >= 2
                && previous_chars[previous_chars.len() - 1] == 'i'
                && previous_chars[previous_chars.len() - 2] == 's'
            {
                number = 6;
                break;
            }
        }
        if char == 'n' {
            if previous_chars.len() >= 4
                && previous_chars[previous_chars.len() - 1] == 'e'
                && previous_chars[previous_chars.len() - 2] == 'v'
                && previous_chars[previous_chars.len() - 3] == 'e'
                && previous_chars[previous_chars.len() - 4] == 's'
            {
                number = 7;
                break;
            }
        }
        if char == 't' {
            if previous_chars.len() >= 4
                && previous_chars[previous_chars.len() - 1] == 'h'
                && previous_chars[previous_chars.len() - 2] == 'g'
                && previous_chars[previous_chars.len() - 3] == 'i'
                && previous_chars[previous_chars.len() - 4] == 'e'
            {
                number = 8;
                break;
            }
        }

        previous_chars.push(char);
    }

    number
}

fn find_digit_2_rev(chars: &mut IntoIter<char>) -> u32 {
    let mut number = 0;
    let mut previous_chars = Vec::new();
    while let Some(char) = chars.next() {
        if let Some(digit) = char.to_digit(10) {
            number = digit;
            break;
        }
        if char == 'o' {
            if previous_chars.len() >= 2
                && previous_chars[previous_chars.len() - 1] == 'n'
                && previous_chars[previous_chars.len() - 2] == 'e'
            {
                number = 1;
                break;
            }
        }
        if char == 't' {
            if previous_chars.len() >= 2
                && previous_chars[previous_chars.len() - 1] == 'w'
                && previous_chars[previous_chars.len() - 2] == 'o'
            {
                number = 2;
                break;
            }
            if previous_chars.len() >= 4
                && previous_chars[previous_chars.len() - 1] == 'h'
                && previous_chars[previous_chars.len() - 2] == 'r'
                && previous_chars[previous_chars.len() - 3] == 'e'
                && previous_chars[previous_chars.len() - 4] == 'e'
            {
                number = 3;
                break;
            }
        }
        if char == 'f' {
            if previous_chars.len() >= 3
                && previous_chars[previous_chars.len() - 1] == 'o'
                && previous_chars[previous_chars.len() - 2] == 'u'
                && previous_chars[previous_chars.len() - 3] == 'r'
            {
                number = 4;
                break;
            }
            if previous_chars.len() >= 3
                && previous_chars[previous_chars.len() - 1] == 'i'
                && previous_chars[previous_chars.len() - 2] == 'v'
                && previous_chars[previous_chars.len() - 3] == 'e'
            {
                number = 5;
                break;
            }
        }

        if char == 'n' {
            if previous_chars.len() >= 3
                && previous_chars[previous_chars.len() - 1] == 'i'
                && previous_chars[previous_chars.len() - 2] == 'n'
                && previous_chars[previous_chars.len() - 3] == 'e'
            {
                number = 9;
                break;
            }
        }
        if char == 's' {
            if previous_chars.len() >= 2
                && previous_chars[previous_chars.len() - 1] == 'i'
                && previous_chars[previous_chars.len() - 2] == 'x'
            {
                number = 6;
                break;
            }
            if previous_chars.len() >= 4
                && previous_chars[previous_chars.len() - 1] == 'e'
                && previous_chars[previous_chars.len() - 2] == 'v'
                && previous_chars[previous_chars.len() - 3] == 'e'
                && previous_chars[previous_chars.len() - 4] == 'n'
            {
                number = 7;
                break;
            }
        }
        if char == 'e' {
            if previous_chars.len() >= 4
                && previous_chars[previous_chars.len() - 1] == 'i'
                && previous_chars[previous_chars.len() - 2] == 'g'
                && previous_chars[previous_chars.len() - 3] == 'h'
                && previous_chars[previous_chars.len() - 4] == 't'
            {
                number = 8;
                break;
            }
        }

        previous_chars.push(char);
    }

    number
}
