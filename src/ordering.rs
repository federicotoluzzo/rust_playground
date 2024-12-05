use std::collections::{HashMap, HashSet};
use std::fs;

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
    for something in nums {
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
}
