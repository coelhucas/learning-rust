use std::collections::HashMap;

fn main() {
    let mut vec = vec![8, 4, 2, 6, 8, 10, 13, 1, 14, 3, 3, 4, 3];
    println!("Vec mean: {}", mean(&vec));
    println!("Vec mode: {}", mode(&vec));
    println!("Vec median: {}", median(&mut vec));
}

fn mean(vec: &Vec<i32>) -> i32 {
    let mut sum = 0;
    let length = vec.len() as i32;

    for value in vec.iter() {
        sum += value;
    }

    sum / length
}

fn median(vec: &mut Vec<i32>) -> i32 {
    let mut sorted_vec = vec;
    sorted_vec.sort_unstable();

    let middle_index = sorted_vec.len() / 2;

    sorted_vec[middle_index]
}

fn mode(vec: &Vec<i32>) -> i32 {
    let mut numbers = HashMap::new();
    let mut most_occurrences_value = 0;

    for value in vec.iter() {
        let count = numbers.entry(value).or_insert(0);
        *count += 1;
    }

    let mut tmp_occurrences = 0;

    for (key, value) in &numbers {
        if value > &tmp_occurrences {
            tmp_occurrences = *value;
            most_occurrences_value = **key;
        }
    }

    most_occurrences_value
}
