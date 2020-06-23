use std::convert::TryInto; 

fn main() {

    let integers = vec![8, -3, 0, 8, 18, 2, -5, 8, 18, -5, 8, 18, 8];

    let mean = mean(integers);

    println!("Mean: {}", mean);
}

fn mean(v: Vec<i32>) -> f32 {

    let mut sum : i32 = 0;

    for x in &v {
        sum += x;
    }

    let length : i32 = v.len().try_into().unwrap();

    (sum as f32) / (length as f32)
}

