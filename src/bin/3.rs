use advent_25::get_input;

fn get_joltage(bank: String, number_of_batteries: usize) -> u64 {
    let mut joltage: Vec<(String, usize)> = Vec::new();
    for battery_number in 1..=number_of_batteries {
        'outer: for search_digit in (1..=9).rev() {
            let digit_str = search_digit.to_string();
            let start_point: usize = if joltage.len() > 0 {
                let last: usize = joltage.last().unwrap().1.try_into().unwrap();
                last + 1
            } else { 0 };
            let end_point: usize = bank.len() - (number_of_batteries-battery_number);
            for (idx, battery) in bank[start_point..end_point].chars().enumerate() {
                if battery.to_string() == digit_str {
                    joltage.push((digit_str, idx + start_point));
                    break 'outer;
                }
            }
        }
    }
    let joltage_str: String = joltage.iter().map(|item| item.0.as_str()).collect();
    u64::from_str_radix(&joltage_str, 10).unwrap()
}

fn main() -> Result<(), &'static str> {
    let input: Vec<String> = get_input(3);
    let mut total_joltage_1: u64 = 0;
    let mut total_joltage_2: u64 = 0;

    for bank in input {
        total_joltage_1 += get_joltage(bank.clone(), 2);
        total_joltage_2 += get_joltage(bank.clone(), 12);
    }

    println!("1: {total_joltage_1}");
    println!("2: {total_joltage_2}");

    Ok(())
}
