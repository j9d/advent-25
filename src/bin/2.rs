use advent_25::get_input;


fn main() -> Result<(), &'static str> {
    let input: Vec<String> = get_input(2);
    let ranges: Vec<(u64, u64)> = input[0].split(",").map(|range| {
        let bounds: Vec<&str> = range.split("-").collect();
        (u64::from_str_radix(bounds[0], 10).unwrap(), u64::from_str_radix(bounds[1], 10).unwrap())
    }).collect();

    let mut sum_1: u64 = 0;
    let mut sum_2: u64 = 0;

    for (start, end) in ranges {
        for current in start..=end {
            let current_str = current.to_string();
            let mut current_number_added: bool = false;
            for slice_length in 1..=(current_str.len()/2) {
                let mut this_slice_length_is_good: bool = true;
                if current_str.len() % slice_length != 0 {
                    continue;
                }
                let first_slice = &current_str[0..slice_length];
                for start_point in (slice_length..=current_str.len()-slice_length).step_by(slice_length) {
                    let this_slice = &current_str[start_point..start_point+slice_length];
                    if this_slice != first_slice {
                        this_slice_length_is_good = false;
                        break;
                    }
                }
                if this_slice_length_is_good {
                    if !current_number_added {
                        sum_2 += current;
                        current_number_added = true;
                    }
                    if slice_length == current_str.len()/2 {
                        sum_1 += current;
                    }
                }
            }

        }
    }

    println!("1: {sum_1}");
    println!("2: {sum_2}");

    Ok(())
}
