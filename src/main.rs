use std::collections::HashMap;
use std::convert::TryInto;

fn main() {
    let integers = vec![-3, 0, 8, 18, 2, -5, 8, 18, -5, 8, 18, 8, 0, 0, 0];

    println!("Sorting             {:?}", integers);

    let mean = mean(&integers);
    let median = median(&integers);

    let mut mode_vec = Vec::with_capacity(integers.len());
    mode(&integers, &mut mode_vec);

    println!("Mean  : {}", mean);
    println!("Median: {}", median);
    println!("Mode  : {:?}", mode_vec);
}

fn mean(integers: &Vec<i32>) -> f32 {

    let mut sum : i32 = 0;

    for x in integers {
        sum += x;
    }

    let length : u32 = integers.len().try_into().unwrap();

    (sum as f32) / (length as f32)
}

fn median(integers: &Vec<i32>) -> f32 {

    let mut sorted_integers = integers.clone();
    sort_integers(integers, &mut sorted_integers);

    let length = integers.len();
    let median;

    if (length % 2) == 1 {
        let mid_index = (length - 1) / 2;
        median = sorted_integers[mid_index] as f32;
    }
    else {
        let mid_index = length / 2;
        median = (sorted_integers[mid_index-1] as f32
                + sorted_integers[mid_index] as f32)
                / 2.0;
    }

    return median;
}

fn sort_integers(integers: &Vec<i32>, sorted_integers: &mut Vec<i32>) {
    let length = integers.len();

    for x in 0..length {
        let mut min = sorted_integers[x];
        let mut index_of_min = x;

        for y in x+1..length {
            if sorted_integers[y] < min {
                min = sorted_integers[y];
                index_of_min = y;
            }
        }

        sorted_integers[index_of_min] = sorted_integers[x];
        sorted_integers[x] = min;
    }

    println!("Sorted list         {:?}", sorted_integers);
}

fn mode(integers: &Vec<i32>, mode: &mut Vec<i32>) {
    let mut integers_and_counts = HashMap::new();

    for x in integers {
        let count = integers_and_counts.entry(x).or_insert(0);
        *count += 1;
    }

    println!("Hash map of numbers {:?}", integers_and_counts);

    let mut max_count = 0;

    for integer in integers_and_counts.keys() {
        let count = integers_and_counts.get(integer);
        match count {
            None => None,
            Some(i) => if Some(i) > Some(&max_count) { Some(max_count = *i) }
                       else { None },
        };
    }

    for integer in integers_and_counts.keys() {
        let count = integers_and_counts.get(integer);
        match count {
            None => {},
            Some(i) => if Some(i) == Some(&max_count) { mode.push(**integer) },
        };
    }
}

