use rand::Rng;
use std::time::Instant;

fn main() {
    let vec = create_array(1000);
    
    evaluate_sort("Merge Sort", merge_sort, &vec);
    evaluate_sort("Insertion Sort", insertion_sort, &vec);
}

fn evaluate_sort(name: &str, f: fn(&Vec<i32>) -> Vec<i32>, array: &Vec<i32>) {
    let start = Instant::now();
    let result = f(array);
    let elapsed = start.elapsed();
    
    println!("Sort function: {:?}", name);
    println!("Is sorted ? {}", is_sorted(&result));
    println!("Duration: {:?}\n", elapsed);
}

fn is_sorted(array: &Vec<i32>) -> bool {
    for i in 1..array.len() {
        if array[i-1] > array[i] {
            return false;
        }
    }
    
    return true;
}

fn create_array(size: usize) -> Vec<i32> {
    let mut vec = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..size {
        vec.push(rng.gen::<i32>());
    }
    
    return vec;
}

fn insertion_sort(array: &Vec<i32>) -> Vec<i32> {
    let mut sorted = array.to_vec();
    let length = sorted.len();
    
    for i in 1..length {
        let x = sorted[i];
        
        let mut j = i;
        while j > 0 && sorted[j - 1] > x {
            sorted[j] = sorted[j - 1];
            j -= 1;
        }
        
        sorted[j] = x;
    }
    
    return sorted;
}

fn split(array: &Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let half = array.len() / 2;
    
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    
    for i in 0..half {
        vec1.push(array[i]);
    }
    
    for i in half..array.len() {
        vec2.push(array[i]);
    }
    
    return (vec1, vec2);
}

fn merge_sort(array: &Vec<i32>) -> Vec<i32> {
    let mut sorted = Vec::new();
    let vec1: Vec<i32>;
    let vec2: Vec<i32>;
    
    if array.len() == 1 {
        return array.to_vec();
    } else {
        let splitted = split(array);
        vec1 = merge_sort(&splitted.0);
        vec2 = merge_sort(&splitted.1);
    }

    let mut i = 0;
    let mut j = 0;

    for _ in 0..array.len() {
        if i >= vec1.len() {
            sorted.push(vec2[j]);
            j += 1;
        } else if j >= vec2.len() {
            sorted.push(vec1[i]);
            i += 1;
        } else {
            let x = vec1[i];
            let y = vec2[j];
            
            if x < y {
                sorted.push(x);
                i += 1;
            } else {
                sorted.push(y);
                j += 1;
            }
        }
    }
    
    return sorted;
}
