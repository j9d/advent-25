use std::fs;

pub fn get_input(day: u8) -> Vec<String> {
    let input: Vec<String> = fs::read_to_string(format!("input/{}.txt", day))
        .expect("couldn't read input file")
        .lines()
        .map(|i| i.to_owned())
        .collect();

    input
}
