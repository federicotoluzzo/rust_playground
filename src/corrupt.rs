use std::fs;
use regex::Regex;
fn mul(a:i32, b:i32) -> i32 {
    return a * b;
}

fn mul_all(vec:Vec<i32>) -> i32 {
    let mut result = 0;
    for i in (0..vec.len()).step_by(2) {
        result += mul(vec[i], vec[i+1]);
    }
    return result;
}

fn parse(string:String) -> Vec<i32> {
    let exp = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let mut vec:Vec<i32> = Vec::new();
    let mut appropriate_variable_name = String::new();
    let mut can_do = true;
    for c in string.chars() {
        appropriate_variable_name.push(c);
        if(appropriate_variable_name.contains("do()")){
            can_do = true;
            appropriate_variable_name.clear()
        }
        if(appropriate_variable_name.contains("don't()")){
            can_do = false;
            appropriate_variable_name.clear();
        }
        if c == 'm'{
            appropriate_variable_name = "m".to_string();
        }
        if(can_do){
            if exp.is_match(appropriate_variable_name.as_str()) {
                let params = appropriate_variable_name
                    .trim_start_matches("mul(")
                    .trim_end_matches(')')
                    .split(",")
                    .collect::<Vec<&str>>();
                vec.push(params[0].parse::<i32>().unwrap());
                vec.push(params[1].parse::<i32>().unwrap());
                appropriate_variable_name = String::new();
            }
        }
    }
    return vec;
}

pub fn print_answer(){
    let data = fs::read_to_string("corrupt.txt").unwrap();
    println!("{}", mul_all(parse(data)))
}