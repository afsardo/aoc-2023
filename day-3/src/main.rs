fn main() {
    part_1();
    part_2();
}

#[derive(Debug)]
struct NumberInMatrix {
    digits: Vec<u32>,
    adjacent_chars: Vec<char>,
}

impl NumberInMatrix {
    fn new() -> NumberInMatrix {
        NumberInMatrix {
            digits: Vec::new(),
            adjacent_chars: Vec::new(),
        }
    }

    fn add_digit(&mut self, digit: u32) {
        self.digits.push(digit);
    }

    fn add_adjacent_char(&mut self, adjacent_char: char) {
        if !adjacent_char.is_digit(10) {
            self.adjacent_chars.push(adjacent_char);
        }
    }

    fn to_number(&self) -> u32 {
        let mut number = 0;
        for digit in self.digits.iter() {
            number = number * 10 + digit;
        }

        number
    }

    fn is_empty(&self) -> bool {
        self.digits.len() == 0
    }

    fn is_isolated(&self) -> bool {
        for adjacent_char in self.adjacent_chars.iter() {
            if adjacent_char != &'.' {
                return false;
            }
        }

        true
    }
}

fn part_1() {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let input = std::fs::read_to_string("src/part-1.input").unwrap();
    let lines: Vec<&str> = input.split("\n").collect();
    for (row, line) in lines.iter().enumerate() {
        if matrix.len() <= row {
            matrix.push(Vec::new());
        }
        for character in line.chars() {
            matrix[row].push(character);
        }
    }

    let mut numbers: Vec<NumberInMatrix> = Vec::new();
    for (i, row) in matrix.iter().enumerate() {
        let mut number = NumberInMatrix::new();
        for (j, char) in row.iter().enumerate() {
            if let Some(digit) = char.to_digit(10) {
                number.add_digit(digit);
                if i > 0 {
                    number.add_adjacent_char(matrix[i - 1][j]);
                    if j > 0 {
                        number.add_adjacent_char(matrix[i - 1][j - 1]);
                    }
                    if j < row.len() - 1 {
                        number.add_adjacent_char(matrix[i - 1][j + 1]);
                    }
                }
                if i < matrix.len() - 1 {
                    number.add_adjacent_char(matrix[i + 1][j]);
                    if j > 0 {
                        number.add_adjacent_char(matrix[i + 1][j - 1]);
                    }
                    if j < row.len() - 1 {
                        number.add_adjacent_char(matrix[i + 1][j + 1]);
                    }
                }
                if j > 0 {
                    number.add_adjacent_char(matrix[i][j - 1]);
                }
                if j < row.len() - 1 {
                    number.add_adjacent_char(matrix[i][j + 1]);
                }
            } else {
                if !number.is_empty() {
                    numbers.push(number);
                    number = NumberInMatrix::new();
                }
            }
        }
        if !number.is_empty() {
            numbers.push(number);
        }
    }

    let sum = numbers
        .iter()
        .filter(|n| !n.is_isolated())
        .map(|n| n.to_number())
        .reduce(|a, b| a + b);

    println!("Result: {:?}", sum.unwrap());
}

#[derive(Debug)]
struct NumberInGear {
    digits: Vec<u32>,
    positions: Vec<(u32, u32)>,
}

impl NumberInGear {
    fn new() -> NumberInGear {
        NumberInGear {
            digits: Vec::new(),
            positions: Vec::new(),
        }
    }

    fn add_digit(&mut self, digit: u32, i: usize, j: usize) {
        self.digits.push(digit);
        self.positions.push((i as u32, j as u32));
    }

    fn reverse(&mut self) {
        self.digits.reverse();
    }

    fn to_number(&self) -> u32 {
        let mut number = 0;
        for digit in self.digits.iter() {
            number = number * 10 + digit;
        }

        number
    }

    fn equals(&self, other: &NumberInGear) -> bool {
        if self.digits.len() != other.digits.len() {
            return false;
        }

        for (i, digit) in self.digits.iter().enumerate() {
            if digit != &other.digits[i] {
                return false;
            }
        }

        true
    }
}

fn part_2() {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let input = std::fs::read_to_string("src/part-2.input").unwrap();
    let lines: Vec<&str> = input.split("\n").collect();
    for (row, line) in lines.iter().enumerate() {
        if matrix.len() <= row {
            matrix.push(Vec::new());
        }
        for character in line.chars() {
            matrix[row].push(character);
        }
    }

    let mut sum = 0;
    for (i, row) in matrix.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if char == &'*' {
                let count = count_adjacent_numbers(&matrix, i, j);
                if count >= 2 {
                    let numbers = find_adjacent_numbers(&matrix, i, j);
                    if numbers.len() >= 2 {
                        let ratio = numbers
                            .iter()
                            .map(|n| n.to_number())
                            .reduce(|a, b| a * b)
                            .unwrap();
                        sum += ratio;
                    }
                }
            }
        }
    }

    println!("Result: {:?}", sum);
}

fn count_adjacent_numbers(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    let mut count = 0;
    if i > 0 {
        if matrix[i - 1][j].is_digit(10) {
            count += 1;
        }
        if j > 0 {
            if matrix[i - 1][j - 1].is_digit(10) {
                count += 1;
            }
        }
        if j < matrix[i].len() - 1 {
            if matrix[i - 1][j + 1].is_digit(10) {
                count += 1;
            }
        }
    }
    if i < matrix.len() - 1 {
        if matrix[i + 1][j].is_digit(10) {
            count += 1;
        }
        if j > 0 {
            if matrix[i + 1][j - 1].is_digit(10) {
                count += 1;
            }
        }
        if j < matrix[i].len() - 1 {
            if matrix[i + 1][j + 1].is_digit(10) {
                count += 1;
            }
        }
    }
    if j > 0 {
        if matrix[i][j - 1].is_digit(10) {
            count += 1;
        }
    }
    if j < matrix[i].len() - 1 {
        if matrix[i][j + 1].is_digit(10) {
            count += 1;
        }
    }

    count
}

fn find_adjacent_numbers(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<NumberInGear> {
    let mut numbers: Vec<NumberInGear> = Vec::new();
    if i > 0 {
        if matrix[i - 1][j].is_digit(10) {
            let number = search_number(matrix, i - 1, j);
            if !numbers.iter().any(|n| n.equals(&number)) {
                numbers.push(number);
            }
        }
        if j > 0 {
            if matrix[i - 1][j - 1].is_digit(10) {
                let number = search_number(matrix, i - 1, j - 1);
                if !numbers.iter().any(|n| n.equals(&number)) {
                    numbers.push(number);
                }
            }
        }
        if j < matrix[i].len() - 1 {
            if matrix[i - 1][j + 1].is_digit(10) {
                let number = search_number(matrix, i - 1, j + 1);
                if !numbers.iter().any(|n| n.equals(&number)) {
                    numbers.push(number);
                }
            }
        }
    }
    if i < matrix.len() - 1 {
        if matrix[i + 1][j].is_digit(10) {
            let number = search_number(matrix, i + 1, j);
            if !numbers.iter().any(|n| n.equals(&number)) {
                numbers.push(number);
            }
        }
        if j > 0 {
            if matrix[i + 1][j - 1].is_digit(10) {
                let number = search_number(matrix, i + 1, j - 1);
                if !numbers.iter().any(|n| n.equals(&number)) {
                    numbers.push(number);
                }
            }
        }
        if j < matrix[i].len() - 1 {
            if matrix[i + 1][j + 1].is_digit(10) {
                let number = search_number(matrix, i + 1, j + 1);
                if !numbers.iter().any(|n| n.equals(&number)) {
                    numbers.push(number);
                }
            }
        }
    }
    if j > 0 {
        if matrix[i][j - 1].is_digit(10) {
            let number = search_number(matrix, i, j - 1);
            if !numbers.iter().any(|n| n.equals(&number)) {
                numbers.push(number);
            }
        }
    }
    if j < matrix[i].len() - 1 {
        if matrix[i][j + 1].is_digit(10) {
            let number = search_number(matrix, i, j + 1);
            if !numbers.iter().any(|n| n.equals(&number)) {
                numbers.push(number);
            }
        }
    }

    numbers
}

fn search_number(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> NumberInGear {
    let mut number = NumberInGear::new();

    let mut x = (j as isize) - 1;
    while x >= 0 {
        if let Some(digit) = matrix[i][x as usize].to_digit(10) {
            number.add_digit(digit, i, x as usize)
        } else {
            break;
        }
        x -= 1;
    }

    number.reverse();

    number.add_digit(matrix[i][j].to_digit(10).unwrap(), i, j);

    let mut x = j + 1;
    while x < matrix[i].len() {
        if let Some(digit) = matrix[i][x].to_digit(10) {
            number.add_digit(digit, i, x)
        } else {
            break;
        }
        x += 1;
    }

    number
}
