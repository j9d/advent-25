use std::{iter::Rev, str::Chars};

use advent_25::get_input;

fn parse_input(input: Vec<String>) -> Vec<Vec<String>> {
    // Lazily handle the extra empty string at the start because it's 3am
    let mut spaces: Vec<&str> = input[4].split(|item| item == '*' || item == '+').collect();
    spaces.remove(0);
    let mut lengths: Vec<usize> = spaces.iter().map(|item| item.len()).collect();

    // Lazily handle the lack of extra space at the end because it's 3am
    let adjustment = lengths.pop().unwrap() + 1;
    lengths.push(adjustment);

    // Read x chars from each line and store them (skip the extra space between)
    let mut parsed_input: Vec<Vec<String>> = Vec::new();
    for _ in 0..input.len() {
        parsed_input.push(Vec::new());
    }
    let mut running_idx: usize = 0;
    for length in lengths {
        for i in 0..input.len() {
            parsed_input[i].push(input[i][running_idx..running_idx+length].to_string());
        }
        running_idx += length + 1
    }
    parsed_input
}

fn parse_equation_1(inputs: &Vec<&String>) -> Vec<u64> {
    inputs.iter().map(|item| u64::from_str_radix(item.trim(), 10).unwrap()).collect::<Vec<u64>>()
}

fn parse_equation_2(inputs: &Vec<&String>) -> Vec<u64> {
    // Read each line vertically and store as vecs of chars
    let max: usize = *inputs.iter().map(|item| (*item).len()).collect::<Vec<usize>>().iter().max().unwrap();
    let mut figures: Vec<Vec<String>> = Vec::new();
    let mut iters: Vec<Rev<Chars>> = inputs.iter().map(|item| (*item).chars().rev()).collect();
    for i in 0..max {
        figures.push(Vec::new());
        for j in 0..iters.len() {
            figures[i].push(iters[j].next().unwrap().to_string());
        }
    }

    // Parse into ints
    let mut operands: Vec<u64> = Vec::new();
    for figure in figures {
        let filtered_figure: Vec<&String> = figure.iter().filter(|item| *item != " ").collect();
        operands.push(
            u64::from_str_radix(
                filtered_figure
                    .iter()
                    .map(|i| i.as_str())
                    .collect::<Vec<&str>>()
                    .join("")
                    .as_str(),
                10
            )
            .unwrap()
        )
    }

    operands
}

fn main() -> Result<(), &'static str> {
    let input: Vec<String> = get_input(6);
    let equations: Vec<Vec<String>> = parse_input(input);
    let mut total_1: u64 = 0;
    let mut total_2: u64 = 0;

    for i in 0..equations[0].len() {
        let mut current: Vec<&String> = Vec::new();
        for j in 0..=3 {
            current.push(&equations[j][i]);
        }
        let equation_1 = (parse_equation_1(&current), equations[4][i].clone());
        let this_1 = if equation_1.1.trim() == "+" {
            equation_1.0.iter().fold(0, |acc, elem| acc + elem)
        } else {
            equation_1.0.iter().fold(1, |acc, elem| acc * elem)
        };

        let equation_2 = (parse_equation_2(&current), equations[4][i].clone());
        let this_2 = if equation_2.1.trim() == "+" {
            equation_2.0.iter().fold(0, |acc, elem| acc + elem)
        } else {
            equation_2.0.iter().fold(1, |acc, elem| acc * elem)
        };

        total_1 += this_1;
        total_2 += this_2;
    }

    println!("1: {total_1}");
    println!("2: {total_2}");

    Ok(())
}
