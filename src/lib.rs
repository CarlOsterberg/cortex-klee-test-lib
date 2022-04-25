#![no_std]
use core::{cmp::Ord};

use heapless::Vec;


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
pub fn do_things(input: &str) -> bool {
    let mut ret = false;
    //let string: String = "hej kom och hjälp mig ".to_string() + input;
    if input.len() > 20 {
        for _ in input.as_bytes() {
            ret = !ret;
        }
        ret
    }
    else {
        input.as_bytes().ends_with(&[0])
    }
}

pub fn is_palindrome_classic(phrase: &str) -> bool {
    if phrase.len() == 0 { return true }

    // need to do this because Rust is rightly trying to force you away from treating strs as
    //  arrays
    let mut xs: Vec<char, 30> = Vec::new();
    
    xs = phrase.chars().collect();

    // start from the beginning
    let mut first_idx = 0;

    // and the end, btw, don't forget the off-by-one b/c of len() is actually past the last index...
    //  this is a classic error avoided implicitly above.
    let mut last_idx = phrase.len() - 1;
    // loop and guard that we don't go too far
    while first_idx < last_idx {
        // filter out non-alphabetics, the += and -= would be something you could accidentally screw up,
        //   avoided in the iterator based impl
        if !xs[first_idx].is_alphabetic() { first_idx += 1; continue }
        if !xs[last_idx].is_alphabetic() { last_idx -= 1; continue }

        // compare the values, did we compare the correct indexes? again avoided in the iterator impl
        if xs[first_idx].to_ascii_lowercase() != xs[last_idx].to_ascii_lowercase() {
            return false;
        }

        // same += and -= potential bug avoided in the iterator impl
        first_idx += 1;
        last_idx -= 1;
    }

    // is this actually simpler or more readable? I don't think so...

    true
}
