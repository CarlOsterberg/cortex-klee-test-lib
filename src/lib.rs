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

#[inline(never)]
pub fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[inline(never)]
pub fn selection_sort(arr: &mut [i32])-> &[i32] {
    let len = arr.len();
	// Rust would skip iteration if lower bound >= upper bound.
	// Hence, no need to `len - 1`.
	for i in 0..len {
	    let mut temp = i;
	    for j in (i + 1)..len {
	        if arr[temp] > arr[j] {
	            temp = j;
	        }
	    }
	    arr.swap(i, temp);
	}
    arr
}