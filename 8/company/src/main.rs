use std::io;
use std::process;
use std::collections::HashMap;

// 加入员工
// Example: Add Ruiheng Zhang to Development
fn add(command: Vec<&str>, company: &mut HashMap<String, Vec<String>>) -> Result<String, String> {
    let mut staff_name = String::new();  // 员工名
    let mut department = String::new();  // 部门
    let mut find_to = false;  // 是否找到了“to”
    for item in command {
        if item == "to" {
            find_to = true;
            continue;
        }

        if item == "Add" {
            continue;
        }

        if find_to {  // 进入部门名
            department.push_str(item);
            department.push(' ');
        } else {  // 员工名
            staff_name.push_str(item);
            staff_name.push(' ');
        }
    }

    if !find_to {  // 未找到“to”
        return Err(String::from("Miss 'to' in Add command."));
    }

    // 去除末尾空格
    let staff_name = staff_name.trim().to_string();
    let department = department.trim().to_string();

    let department_staff = company.entry(department).or_insert(vec![]);
    department_staff.push(staff_name); // 加入员工

    Ok(String::from("Successful."))
}

// 列出公司/部门员工
// Example: List Development
fn list(command: Vec<&str>, company: &HashMap<String, Vec<String>>) -> Result<String, String> {
    if command.len() == 1 {  // 仅 List
        Err("Miss department in List command.".to_string())
    } else {
        // 展示部门员工
        let mut new_command = command.clone();
        new_command.remove(0);

        let department = new_command.join(" ");

        match company.get(&department) {
            Some(staffs_name) => {  // 找到此部门
                let mut staffs_name = staffs_name.clone();
                staffs_name.sort();

                println!("* {department}");

                for name in staffs_name {
                    println!("  - {name}");
                }

                Ok(String::from("Successful."))
            },
            None => Err(format!("Cannot find {department} in the company."))
        }
    }
}

fn quit() {
    println!("Bye👋");
    process::exit(0);  // 成功退出
}

fn parse_command(command: Vec<&str>, company:&mut HashMap<String, Vec<String>>) -> Result<String, String> {
    if command.len() == 0 {
        return Err(String::from("Please enter a command!"));
    }

    match command[0] {
        "Add" => {
            let result = match add(command, company) {
                Ok(msg) => Ok(msg),
                Err(err) => Err(err),
            };
            result
        },
        "List" => {
            let result = match list(command, company) {
                Ok(msg) => Ok(msg),
                Err(err) => Err(err),
            };
            result
        },
        "Quit" => {
            quit();
            Ok(String::from("Bye"))
        },
        _ => {
            Err(format!("Unsupported command: {}", command[0]))
        }
    }
}

fn main() {
    println!("Company Management System Started.");

    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut input_command = String::new();

        io::stdin()
            .read_line(&mut input_command)
            .expect("Failed to read line.");

        let input_command: Vec<&str> = input_command.trim().split(' ').collect();

        let result = parse_command(input_command, &mut company);

        match result {
            Ok(msg) => println!("{msg}"),
            Err(err) => println!("{err}"),
        }
    }
}
