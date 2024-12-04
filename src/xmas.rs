use std::fs;
use std::thread::AccessError;

fn get_char_matrix() -> Vec<Vec<char>> {
    let str = fs::read_to_string("xmas.txt").unwrap();
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in str.lines() {
        matrix.push(line.chars().collect());
    }
    return matrix;
}

fn check_vertical(matrix: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for i in 0..(matrix.len() - 3) {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == 'X' && matrix[i + 1][j] == 'M' && matrix[i + 2][j] == 'A' && matrix[i + 3][j] == 'S' {
                count += 1;
            }
            if matrix[i][j] == 'S' && matrix[i + 1][j] == 'A' && matrix[i + 2][j] == 'M' && matrix[i + 3][j] == 'X' {
                count += 1;
            }
        }
    }
    return count;
}
fn check_horizontal(matrix: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for i in 0..matrix.len() {
        for j in 0..(matrix[0].len() - 3) {
            if matrix[i][j] == 'X' && matrix[i][j + 1] == 'M' && matrix[i][j + 2] == 'A' && matrix[i][j + 3] == 'S' {
                count += 1;
            }
            if matrix[i][j] == 'S' && matrix[i][j + 1] == 'A' && matrix[i][j + 2] == 'M' && matrix[i][j + 3] == 'X' {
                count += 1;
            }
        }
    }
    return count;
}
fn check_diag_one(matrix: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for i in 0..(matrix.len() - 3) {
        for j in 0..(matrix[0].len() - 3) {
            if matrix[i][j] == 'X' && matrix[i + 1][j + 1] == 'M' && matrix[i + 2][j + 2] == 'A' && matrix[i + 3][j + 3] == 'S' {
                count += 1;
            }
            if matrix[i][j] == 'S' && matrix[i + 1][j + 1] == 'A' && matrix[i + 2][j + 2] == 'M' && matrix[i + 3][j + 3] == 'X' {
                count += 1;
            }
        }
    }
    return count;
}
fn check_diag_two(matrix: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for i in 0..(matrix.len() - 3) {
        for j in 4..matrix[0].len(){
            if matrix[i][j] == 'X' && matrix[i + 1][j - 1] == 'M' && matrix[i + 2][j - 2] == 'A' && matrix[i + 3][j - 3] == 'S' {
                count += 1;
            }
            if matrix[i][j] == 'S' && matrix[i + 1][j - 1] == 'A' && matrix[i + 2][j - 2] == 'M' && matrix[i + 3][j - 3] == 'X' {
                count += 1;
            }
        }
    }
    return count;
}

fn check_all(matrix: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == 'A' {
                //count += check_around(matrix, i as i32, j as i32);
                count += okay_you_stupid_elf(matrix, i, j);
            }
        }
    }
    return count;
}

fn check_around(matrix: &Vec<Vec<char>>, a:i32, b:i32) -> i32 {
    let mut count = 0;
    for i in 0..=2{
        for j in 0..=2{
            println!("{}{}", i, j);
            if i == 1 && j == 1 {
                continue;
            }
            if a + (i - 1) * 3 < 0 || b + (j - 1) * 3 < 0 || a + (i - 1) * 3 >= matrix.len() as i32 || b + (j - 1) * 3 >= matrix[i as usize].len() as i32{
                continue;
            }
            if(matrix[a as usize][b  as usize] == 'X' && matrix[(a + i - 1) as usize][(b + j - 1) as usize] == 'M' && matrix[(a + (i - 1) * 2) as usize][(b + (j - 1) * 2) as usize] == 'A' && matrix[(a + (i - 1) * 3) as usize][(b + (j - 1) * 3) as usize] == 'S') {
                count += 1;
            }
        }
    }
    return count;
}

fn okay_you_stupid_elf(matrix: &Vec<Vec<char>>, a:usize, b:usize) -> i32{
    if(a < 1 || b < 1 || a >= matrix.len() - 1  || b >= matrix[0].len() - 1){
        return 0;
    }
    if(matrix[a - 1][b - 1] == 'M' && matrix[a + 1][b + 1] == 'S'){
        if(matrix[a - 1][b + 1] == 'M' && matrix[a + 1][b - 1] == 'S'){
            return 1;
        } else if(matrix[a - 1][b + 1] == 'S' && matrix[a + 1][b - 1] == 'M'){
            return 1;
        }
    }
    if(matrix[a + 1][b + 1] == 'M' && matrix[a - 1][b - 1] == 'S'){
        if(matrix[a - 1][b + 1] == 'M' && matrix[a + 1][b - 1] == 'S'){
            return 1;
        } else if(matrix[a - 1][b + 1] == 'S' && matrix[a + 1][b - 1] == 'M'){
            return 1;
        }
    }

    return 0;
}

pub fn print_answer() {
    let matrix = get_char_matrix();
    println!("{}", check_all(&matrix));
}