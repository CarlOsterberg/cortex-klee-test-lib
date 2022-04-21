#![no_std]
use core::cmp::Ord;

#[inline(never)]
pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    _quick_sort(arr, 0, (len - 1) as isize);
}

fn _quick_sort<T: Ord>(arr: &mut [T], low: isize, high: isize) {
    if low < high {
        let p = partition(arr, low, high);
        _quick_sort(arr, low, p - 1);
        _quick_sort(arr, p + 1, high);
    }
}

fn partition<T: Ord>(arr: &mut [T], low: isize, high: isize) -> isize {
    let pivot = high as usize;
    let mut store_index = low - 1;
    let mut last_index = high;

    loop {
        store_index += 1;
        while arr[store_index as usize] < arr[pivot] {
            store_index += 1;
        }
        last_index -= 1;
        while last_index >= 0 && arr[last_index as usize] > arr[pivot] {
            last_index -= 1;
        }
        if store_index >= last_index {
            break;
        } else {
            arr.swap(store_index as usize, last_index as usize);
        }
    }
    arr.swap(store_index as usize, pivot as usize);
    store_index
}

#[inline(never)]
pub fn bubble_sort(vec: &mut [i32]) {
    loop {
        let mut done = true;
        for i in 0..vec.len()-1 {
            if vec[i+1] < vec[i] {
                done = false;
                let temp = vec[i+1];
                vec[i+1] = vec[i];
                vec[i] = temp;
            }
        }
        if done {
            return;
        }
    }
}

#[inline(never)]
pub fn is_palindrome(phrase: &str) -> bool {
    // get the chars iterator and associated index
    phrase.char_indices().filter(|&(_,c)| c.is_alphabetic())
                // zip with the second half...
                .zip(phrase.char_indices()
                           // which needs to be reversed...
                           .rev()
                           // and filter out bad cars
                           .filter(|&(_,c)| c.is_alphabetic()))
                // accept all input until the indexes have crossed
                .take_while(|&((first_count, _), (last_count, _))| {first_count < last_count})
                // check that all the chars from the begining and end match
                .all(|((_, first_char), (_, last_char))| {
                    first_char.to_ascii_lowercase() == last_char.to_ascii_lowercase()
                })
}