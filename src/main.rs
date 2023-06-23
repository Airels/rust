mod sorts;
mod utils;

use sorts::*;
use utils::*;

use std::time::Instant;

fn main() {
    let vec = create_array(1000);
    
    evaluate_sort("Merge Sort", merge_sort, &vec);
    evaluate_sort("Insertion Sort", insertion_sort, &vec);
    evaluate_sort("Shaker sort", shaker_sort, &vec);

    println!("End.");
}

fn evaluate_sort(name: &str, f: fn(&Vec<i32>) -> Vec<i32>, array: &Vec<i32>) {
    let start = Instant::now();
    let result = f(array);
    let elapsed = start.elapsed();
    
    println!("Sort function: {:?}", name);
    println!("Is sorted ? {}", is_sorted(&result));
    println!("Duration: {:?}\n", elapsed);
}
