use advent_25::get_input;

fn main() -> Result<(), &'static str> {
    let input: Vec<String> = get_input(1);
    let mut code: i32 = 50;
    let mut combination_1: i32 = 0;
    let mut combination_2: i32 = 0;

    for (_, i) in input.iter().enumerate() {
        let add: bool = (&i[0..1]) == "R";
        let value: i32 = i[1..].parse::<i32>().unwrap();
        let prev: i32 = code;
        if add {
            code += value;
        } else {
            code -= value;
        }

        combination_2 += i32::abs(code) / 100;
        if code < 0 && !(prev == 0) {
            combination_2 += 1;
        }

        if code == 0 {
            combination_2 += 1;
        }

        code = code.rem_euclid(100);

        if code == 0 {
            combination_1 += 1;
        }

    }
    println!("1: {combination_1}");
    println!("2: {combination_2}");

    Ok(())
}
