
use std::fs;
use crate::get_array_distance;

fn get_list_difference(arr1: &mut Vec<i32>, arr2: &mut Vec<i32>) -> i32 {
    quick_sort(arr1, 0, arr1.len());
    quick_sort(arr2, 0, arr2.len());

    let mut diff = 0;
    for i in 0..arr1.len() {
        diff += (arr1[i] - arr2[i]).abs();
    }
    return diff;
}

pub(crate) fn lists(){
    let str = fs::read_to_string("lists.txt").unwrap();
    let mut nums:Vec<&str> = vec![];
    for line in str.lines(){
        nums.push(line);
        println!("{}", line);
    }

    let mut arr1:Vec<i32> = Vec::new();
    let mut arr2:Vec<i32> = Vec::new();
    for i in 0..nums.len(){
        arr1.push(nums[i].split_whitespace().collect::<Vec<&str>>()[0].parse::<i32>().unwrap());
        arr2.push(nums[i].split_whitespace().collect::<Vec<&str>>()[1].parse::<i32>().unwrap());
    }
    arr1.sort();
    arr2.sort();
    println!("{}", get_array_distance(&arr1, &arr2));
    let mut arr:Vec<i32> = vec![10, 69, 420, 104, 123, 4 , 12, 12];
    println!("{:?}",arr);
    quick_sort(&mut arr, 0, 7);
    println!("{:?}",arr);
    println!("{}", get_array_distance(&vec![3, 4, 1, 2, 3, 3], &vec![4, 3, 5, 3, 9, 3]));
}
/*
fn quick_sort(arr: &mut Vec<i16>, begin:i16, end:i16) {
    if(arr.len() <=1){
        return;
    }
    let pivot = arr[0];
    let arr2 = Vec::new();
    let mut min = begin;
    let mut max = end;
    for i in begin..end{
        if(arr[i] < pivot){
            arr2[min] = arr[i];
            min += 1;
        }else{
            arr2[max] = arr[i];
            max -= 1;
        }
    }

    arr2[min] = pivot;
    quick_sort(arr, begin, min);
    quick_sort(arr, min, end);
    arr = arr2;
}*/

// cheated with claude.ai but we'll need this until I can actually learn the language
fn quick_sort(arr: &mut Vec<i32>, begin: usize, end: usize) {
    // Check if the slice is worth sorting
    if begin >= end || end - begin <= 1 {
        return;
    }

    let pivot_idx = partition(arr, begin, end);

    // Sort left partition
    quick_sort(arr, begin, pivot_idx);

    // Sort right partition (pivot_idx + 1 to avoid infinite recursion)
    quick_sort(arr, pivot_idx + 1, end);
}

fn partition(arr: &mut Vec<i32>, begin: usize, end: usize) -> usize {
    let pivot = arr[begin]; // Using first element as pivot
    let mut i = begin + 1;
    let mut j = end - 1;

    loop {
        // Find element greater than pivot from left
        while i <= j && arr[i] <= pivot {
            i += 1;
        }

        // Find element smaller than pivot from right
        while i <= j && arr[j] > pivot {
            j -= 1;
        }

        if i > j {
            break;
        }

        // Swap elements
        arr.swap(i, j);
    }

    // Place pivot in its final position
    arr.swap(begin, j);
    j
}