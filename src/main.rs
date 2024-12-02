extern crate core;

mod lists;
mod reactor;

use std::env;
use std::fs;
use crate::lists::get_list_similarities;

fn fact(n: i32) -> i32 {
    if n <= 0 {
        return 1;
    }
    return n * fact(n - 1);
}

fn fib(n:i32) -> i32{
    if n <= 1 {
        return 1;
    }
    return fib(n-1) + fib(n-2);
}

fn bubble_sort(mut arr:&mut[i32]){
    for i in 0..arr.len(){
        for j in i+1..arr.len(){
            if arr[j] < arr[j-1] {
                arr.swap(j, j-1);
            }
        }
    }
}

fn selection_sort(mut arr:&mut [i32]){
    for i in 0..arr.len() {
        for j in i..arr.len() {
            if(arr[j] > arr[i]){
                arr.swap(i, j);
            }
        }
    }
}

fn quick_sort(arr:&mut Vec<i32>){
    if(arr.len() == 1){
        return;
    }
    let pivot:i32 = arr[0];
    let mut arr1:Vec<i32> = Vec::new();
    let mut arr2:Vec<i32> = Vec::new();
    for i in 1..arr.len(){
        if arr[i] >= pivot{
            arr2.push(arr[i]);
        }else{
            arr1.push(arr[i]);
        }
    }
}

fn get_array_distance(arr1:&Vec<i32>, arr2:&Vec<i32>) -> i32{
    let mut distance = 0;
    for i in 0..arr1.len(){
        distance += (arr1[i] - arr2[i]).abs();
    }
    return distance;
}

fn main() {
    /*
    println!("Hello, world!");
    println!("{}", fact(10));
    println!("{}", fib(45));

    let mut array: [i32;5] = [5, 3, 1, 4, 2];
    
    println!("{:?}", array);
    bubble_sort(&mut array);
    println!("{:?}", array);*/


    //lists::lists();
    reactor::print_answer();
}