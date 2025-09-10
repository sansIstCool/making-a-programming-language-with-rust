use std::fs;

fn main() {
    let file_path = "src/data.txt";
    let content = fs::read_to_string(file_path)
        .expect("err: unable to read file");
    for line in content.lines() {
        if line.starts_with("sum(") && line.ends_with(");") {
            let content = line
                .trim_start_matches("sum(")
                .trim_end_matches(");");
            let numbers: Vec<&str> = content.split(',').collect();
            if numbers.len() == 2 {
                let num1_str = numbers[0].trim();
                let num2_str = numbers[1].trim();
                let num1: i32 = num1_str.parse().expect("err: unable to parse first number");
                let num2: i32 = num2_str.parse().expect("err: unable to parse second number");
                let result = num1 + num2;
                println!("{}", result);
            }
        }
        if line.starts_with("div(") && line.ends_with(");") {
            let content = line
                .trim_start_matches("div(")
                .trim_end_matches(");");
            let numbers: Vec<&str> = content.split(',').collect();
            if numbers.len() == 2 {
                let num1_str = numbers[0].trim();
                let num2_str = numbers[1].trim();
                let num1: i32 = num1_str.parse().expect("err: unable to parse first number");
                let num2: i32 = num2_str.parse().expect("err: unable to parse second number");
                if num2 == 0 {
                    println!("err: division by zero");
                } else {
                    let result = num1 / num2;
                    println!("{}", result);
                }
            }
        }
    }
}