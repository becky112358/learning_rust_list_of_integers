use std::convert::TryInto; 

fn main() {
    let integers = vec![8, -3, 0, 8, 18, 2, -5, 8, 18, -5, 8, 18, 8, 0, 0, 0];

    println!("Sorting     {:?}", integers);

    let mean = mean(&integers);
    let median = median(&integers);

    println!("Mean  : {}", mean);
    println!("Median: {}", median);
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

    println!("Sorted list {:?}", sorted_integers);

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

