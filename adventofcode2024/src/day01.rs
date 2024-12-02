use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn run() {
    let (mut list1, mut list2) = parse_input();
    
    list1.sort();
    list2.sort();

    let total_distance: usize = std::iter::zip(&list1, &list2)
        .map(|(num1, num2)| usize::abs_diff(*num1, *num2) as usize)
        .sum();

    let similarity_score = calculate_similarity(&list1, &list2);

    println!("total distance: {total_distance}, similarity score: {similarity_score}");
}

fn calculate_similarity(list1: &[usize], list2: &[usize]) -> usize {
    let mut similarity: usize = 0;

    let mut i = 0;
    let mut j = 0;
    while i < list1.len() {
        let mut window1 = 1;
        while i+1 < list1.len() && list1[i+1] == list1[i] {
            window1 += 1;
            i += 1;
        }

        j = match list2[j..].iter().position(|num| *num == list1[i]) {
            None => {i += 1; continue},
            Some(k) => k + j,
        };

        let mut window2 = 1;
        while j+1 < list2.len() && list2[j+1] == list2[j] {
            window2 += 1;
            j += 1;
        }

        similarity += list1[i] * window1 * window2;

        i += 1; j += 1;
    }

    similarity
}

fn parse_input() -> (Vec<usize>, Vec<usize>) {
    const INPUT_FILE: &'static str = "inputs/input01";
    let f = File::open(INPUT_FILE).unwrap();
    let reader = BufReader::new(f);
    let mut list1: Vec<usize> = Vec::new();
    let mut list2: Vec<usize> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("read error");
        let mut numbers = line.split_whitespace();
        list1.push(numbers.next().unwrap().parse().unwrap());
        list2.push(numbers.next().unwrap().parse().unwrap());
    }

    (list1, list2)
}