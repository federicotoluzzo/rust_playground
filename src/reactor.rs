use std::fs;
pub fn print_answer(){
    println!("{}", count_almost_safe(&get_reports(fs::read_to_string("reactor.txt").unwrap())));
}

fn get_reports(data:String) -> Vec<Vec<i8>>{
    let mut res:Vec<Vec<i8>> = Vec::new();
    for line in data.split("\n"){
        let values = line.split_whitespace().collect::<Vec<&str>>();
        let mut report:Vec<i8> = Vec::new();
        for number in values{
            report.push(number.parse::<i8>().unwrap());
        }
        res.push(report);
    }
    return res;
}

fn count_safe(reports:&Vec<Vec<i8>>)->u16{
    let mut count:u16 = reports.len() as u16;
    for report in reports{
        let is_increasing = report[1] > report[0];
        for i in 0..report.len() - 1{
            if(is_increasing){
                if !((report[i + 1] - report[i]) >= 1 && (report[i + 1] - report[i]) <= 3){
                    count -= 1;
                    break;
                }
            }else{
                if !((report[i + 1] - report[i]) <= -1 && (report[i + 1] - report[i]) >= -3){
                    count -= 1;
                    break;
                }
            }
        }
    }
    return count;
}

fn is_almost_safe(report:Vec<i8>)->bool{
    for i in 0..report.len(){
        let mut clone = report.clone();
        clone.remove(i);
        let is_increasing = clone[1] > clone[0];
        let mut is_safe = true;
        for i in 0..clone.len() - 1{
            if(is_increasing){
                if !(clone[i + 1] - clone[i] >= 1 && clone[i + 1] - clone[i] <= 3){
                    is_safe = false;
                }
            }else{
                if !(clone[i + 1] - clone[i] <= -1 && clone[i + 1] - clone[i] >= -3){
                    is_safe = false;
                }
            }
        }
        if is_safe {
            return true;
        }
    }
    return false;
    /*
    for valuein report{
        let mut clone = report.clone();
        clone.remove(value as usize);
        let is_increasing = value[1] > value[0];
        for i in 0..value.len() - 1{
            if(is_increasing){
                if !((value[i + 1] - value[i]) >= 1 && (value[i + 1] - value[i]) <= 3){
                    count -= 1;
                    break;
                }
            }else{
                if !((value[i + 1] - value[i]) <= -1 && (value[i + 1] - value[i]) >= -3){
                    count -= 1;
                    break;
                }
            }
        }
    }
    return count;*/
}

fn count_almost_safe(reports:&Vec<Vec<i8>>)->u16{
    let mut count:u16 = 0;
    for report in reports{
        if is_almost_safe(report.clone()){
            count += 1;
        }
    }
    return count;
}