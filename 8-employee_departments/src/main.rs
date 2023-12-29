use std::collections::HashMap;
use std::io;

const DISPLAY: &str = "display";
const ADD: &str = "add";
const TO: &str = "to";
const STOP: &str = "stop";

fn main() {
    let mut employee_per_departement: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!("Please input your action");
        let mut line: String = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        // removing \n
        line.truncate(line.len() - 1);
        let collection: Vec<&str> = line.split(" ").collect();
        if collection.len() == 0 {
            println!("Wrong pattern");
            continue;
        }

        // action
        let action = collection[0];
        match action {
            DISPLAY => {
                for (department, employees) in &employee_per_departement {
                    println!("department {}:", department);
                    for employee in employees {
                        println!("\t - {}", employee);
                    }
                }
            }

            ADD => {
                if collection.len() != 4 {
                    println!("Wrong pattern");
                    continue;
                }

                if *collection.get(2).unwrap() != TO {
                    println!("Wrong pattern");
                    continue;
                }

                let department = collection.get(3).unwrap().to_string();
                let employee = collection.get(1).unwrap().to_string();
                if !employee_per_departement.contains_key(&department) {
                    employee_per_departement.insert(department, vec![employee]);
                } else {
                    let employees = employee_per_departement.entry(department).or_default();
                    employees.push(employee);
                }
            }

            STOP => return,

            _ => {
                println!("Wrong pattern");
                continue;
            }
        }
    }
}
