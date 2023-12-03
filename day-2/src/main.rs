fn main() {
    part_1();
    part_2();
}

const MAX_PER_COLOR: (i32, i32, i32) = (12, 13, 14);

fn part_1() {
    let input = std::fs::read_to_string("src/part-1.input").unwrap();
    let lines: Vec<&str> = input.split("\n").collect();
    let mut sum = 0;
    for line in &lines {
        let game_parts = line.split(": ").collect::<Vec<&str>>();
        let game_id = game_parts[0].replace("Game ", "").parse::<i32>().unwrap();
        let max_per_color = get_max_per_color(game_parts[1].to_string());
        if (max_per_color.0 <= MAX_PER_COLOR.0)
            && (max_per_color.1 <= MAX_PER_COLOR.1)
            && (max_per_color.2 <= MAX_PER_COLOR.2)
        {
            sum += game_id;
        }
    }

    println!("Result: {:?}", sum);
}

fn get_max_per_color(game: String) -> (i32, i32, i32) {
    let mut max_per_color = (0, 0, 0);

    let set: Vec<&str> = game.split("; ").collect();
    for draw in set {
        let mut colors = (0, 0, 0);
        let colors_draw = draw.split(", ").collect::<Vec<&str>>();
        for color_draw in colors_draw {
            let color_data = color_draw.split(" ").collect::<Vec<&str>>();
            let number = color_data[0].parse::<i32>().unwrap();
            if color_data[1] == "red" {
                colors.0 += number;
            } else if color_data[1] == "green" {
                colors.1 += number;
            } else if color_data[1] == "blue" {
                colors.2 += number;
            }
        }
        if colors.0 > max_per_color.0 {
            max_per_color.0 = colors.0;
        }
        if colors.1 > max_per_color.1 {
            max_per_color.1 = colors.1;
        }
        if colors.2 > max_per_color.2 {
            max_per_color.2 = colors.2;
        }
    }

    max_per_color
}

fn part_2() {
    let input = std::fs::read_to_string("src/part-2.input").unwrap();
    let lines: Vec<&str> = input.split("\n").collect();
    let mut sum = 0;
    for line in &lines {
        let game_parts = line.split(": ").collect::<Vec<&str>>();
        let max_per_color = get_max_per_color(game_parts[1].to_string());
        sum += max_per_color.0 * max_per_color.1 * max_per_color.2;
    }

    println!("Result: {:?}", sum);
}
