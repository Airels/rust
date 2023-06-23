pub fn insertion_sort(array: &Vec<i32>) -> Vec<i32> {
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

pub fn merge_sort(array: &Vec<i32>) -> Vec<i32> {
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

pub fn shaker_sort(array: &Vec<i32>) -> Vec<i32> {
    let mut sorted = array.to_vec();
    let mut swapped: bool;
    let end = sorted.len() - 1;

    loop {
        swapped = false;
        for i in 0..end {
            if sorted[i] > sorted[i + 1] {
                swap(&mut sorted, i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            return sorted;
        }

        swapped = false;
        for i in (0..end-1).rev() {
            if sorted[i] > sorted[i + 1] {
                swap(&mut sorted, i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            return sorted;
        }
    }
}

// ***
// Utils functions
// ***

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

fn swap(array: &mut Vec<i32>, i: usize, j: usize) {
    let tmp = array[i];
    array[i] = array[j];
    array[j] = tmp;
}