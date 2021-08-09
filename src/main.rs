//Using a hash map and vectors, create a text interface to allow a user to add employee names to a 
//department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” 
//Then let the user retrieve a list of all people in a department or all people in the company by department, 
//sorted alphabetically.
use std::io;
use std::collections::HashMap;

fn main() {
    let mut employees = HashMap::new();

    loop {
        println!("(1) Add Employee  (2) Get People in department    (3) Get all people");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line!");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Choice: {}", choice);

        if choice == 1 {
            employees = add_employee(employees);
        } else if choice == 2 {
            employees = get_people_departmen(employees);
        } else {
            employees = get_people(employees);
        }
        
    }
}

fn add_employee(mut employees: HashMap<String, String>) -> HashMap<String, String> {
    let mut name = String::new();
    println!("Enter name: ");
    io::stdin().read_line(&mut name).expect("Failed to read line!");
    name = name.trim().to_string();

    let mut department = String::new();
    println!("Enter Department: ");
    io::stdin().read_line(&mut department).expect("Failed to read line!");
    department = department.trim().to_string();

    println!("{} added in: {}", name, department);

    employees.insert(name, department);

    employees
}

fn get_people_departmen(employees: HashMap<String, String>) -> HashMap<String, String> {
    let mut department = String::new();
    println!("Enter Department: ");
    io::stdin().read_line(&mut department).expect("Failed to read line!");
    department = department.trim().to_string();

    for(key, value) in &employees {
        if *value == department {
            println!("{}: {}", key, value);
        }
    }
    employees
}

fn get_people(employees: HashMap<String, String>) -> HashMap<String, String> {
    println!("{:?}", employees);
    employees
}
