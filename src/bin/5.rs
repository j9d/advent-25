use advent_25::get_input;

fn number_in_range(number: u64, range: &(u64, u64)) -> bool {
    number >= range.0 && number <= range.1
}

fn check_food(food: u64, ranges: &Vec<(u64, u64)>) -> bool {
    for range in ranges {
        if number_in_range(food, range) {
            return true
        }
    }
    false
}

fn main() -> Result<(), &'static str> {
    let input: Vec<String> = get_input(5);
    let mut input_ranges: Vec<(u64, u64)> = Vec::new();
    let mut foods: Vec<u64> = Vec::new();
    let mut fresh_ranges: Vec<(u64, u64)> = Vec::new();
    let mut fresh_ingredients: u32 = 0;

    for line in input {
        if line.len() == 0 {
            continue;
        }
        if line.contains("-") {
            let range: Vec<u64> = line.split("-").map(|item| u64::from_str_radix(item, 10).unwrap()).collect();
            input_ranges.push((range[0], range[1]));
        } else {
            foods.push(u64::from_str_radix(&line, 10).unwrap());
        }
    }
    input_ranges.sort_by_key(|item| item.0);

    // Populate fresh ranges
    for range in &input_ranges {
        if fresh_ranges.len() > 0 {
            let &(mut fresh_range) = fresh_ranges.last().unwrap();
            if number_in_range(range.0, &fresh_range) && range.1 >= fresh_range.1 {
                fresh_range.1 = range.1;
                fresh_ranges.pop();
                fresh_ranges.push(fresh_range);
                continue;
            }
            if number_in_range(range.1, &fresh_range) && range.0 <= fresh_range.0 {
                fresh_range.0 = range.0;
                fresh_ranges.pop();
                fresh_ranges.push(fresh_range);
                continue;
            }
            if range.0 >= fresh_range.0 && range.1 <= fresh_range.1 {
                continue;
            }
        }
        fresh_ranges.push(*range);
    }

    // Check which ingredients are fresh
    for food in foods {
        if check_food(food, &fresh_ranges) {
            fresh_ingredients += 1;
        }
    }

    // Tally total fresh ranges
    let mut total_fresh: u64 = 0;
    fresh_ranges.iter().for_each(|item| {
        total_fresh += item.1-item.0 + 1
    });

    println!("1: {fresh_ingredients}");
    println!("2: {total_fresh}");

    Ok(())
}
