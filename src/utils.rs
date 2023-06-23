use rand::Rng;

pub fn is_sorted(array: &Vec<i32>) -> bool {
    for i in 1..array.len() {
        if array[i-1] > array[i] {
            return false;
        }
    }

    return true;
}

pub fn create_array(size: usize) -> Vec<i32> {
    let mut vec = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..size {
        vec.push(rng.gen::<i32>());
    }

    return vec;
}