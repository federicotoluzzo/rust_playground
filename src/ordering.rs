use std::collections::{HashMap, HashSet};
use std::fs;
use std::iter::Map;
use rand::prelude::*;

fn bogosort(mut arr: &mut Vec<i32>, rules:HashMap<i32, HashSet<i32>>){
    let mut vec = arr.clone();
    let mut arrcpy = arr.clone();
    while(!is_ordered(vec.clone(), rules.clone())){
        vec.clear();
        arrcpy = arr.clone();
        while !arrcpy.is_empty(){
            let rand = rand::random_range(0..arrcpy.len());
            let num = arrcpy[rand];
            vec.push(num);
            arrcpy.remove(rand);
        }
    }
    arr.clear();
    arr.append(&mut vec);
    println!("SUCCESS");
}

fn is_ordered(vec:Vec<i32>, rules:HashMap<i32, HashSet<i32>>) -> bool{
    for i in 0..vec.len() {
        if rules.contains_key(&vec[i]) {
            for j in 0..i {
                if(rules[&vec[i]].contains(&vec[j])) {
                    return false;
                }
            }
        }
    }
    return true;
}

pub fn print_answer() {
    let mut all:String = fs::read_to_string("ordering.txt").unwrap();

    let mut map:HashMap<i32, HashSet<i32>> = HashMap::new();

    let mut pairs: String = all.split("\n\n").collect::<Vec<&str>>()[0].to_string();
    let mut pages:String = all.split("\n\n").collect::<Vec<&str>>()[1].to_string();

    let mut arr1 :Vec<i32> = Vec::new();
    let mut arr2 :Vec<i32> = Vec::new();

    for line in pairs.lines() {
        let key :i32= line.split("|").collect::<Vec<&str>>()[0].to_string().parse::<i32>().unwrap();
        let val :i32= line.split("|").collect::<Vec<&str>>()[1].to_string().parse::<i32>().unwrap();

        let mut newset;
        if map.contains_key(&key) {
            newset = map.get_mut(&key).unwrap();
            newset.insert(val);
        } else {
            map.insert(key, HashSet::new());
        }
    }

    let mut nums :Vec<Vec<i32>> = Vec::new();

    for line in pages.lines() {
        let mut row = Vec::new();
        for num in line.split(",") {
            row.push(num.parse::<i32>().unwrap());
        }
        nums.push(row);
    }
    let mut valid_pages = Vec::new();
    for something in nums.clone() {
        let mut flag = true;
        for i in 0..something.len() {
            if map.contains_key(&something[i]) {
                for j in 0..i {
                    if(map[&something[i]].contains(&something[j])) {
                        flag = false;
                    }
                }
            }
        }
        if flag {
            valid_pages.push(something);
        }
    }

    let mut sum = 0;
    for something in valid_pages {
        sum += something[something.len() / 2];
    }

    println!("{}", sum);

    let mut sum2 = 0;
    for mut something in nums{
        bogosort(&mut something, map.clone());
        println!("{:?}", something);
        if(something.len()>=3){
            sum2 += something[something.len() / 2];
        }
    }
    println!("{}", sum2);
}
