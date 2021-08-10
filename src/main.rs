//Using a hash map and vectors, create a text interface to allow a user to add employee names to a 
//department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” 
//Then let the user retrieve a list of all people in a department or all people in the company by department, 
//sorted alphabetically.
use std::io;
use std::collections::hash_map::Entry;
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
            println!("{:?}", employees);
        } else if choice == 2 {
            employees = get_people_departmen(employees);
        } else {
            employees = get_people(employees);
        }
        
    }
}

fn add_employee(mut employees: HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    let mut department = String::new();
    println!("Enter Department: ");
    io::stdin().read_line(&mut department).expect("Failed to read line!");
    department = department.trim().to_string();

    let mut name = String::new();
    println!("Enter name: ");
    io::stdin().read_line(&mut name).expect("Failed to read line!");
    name = name.trim().to_string();

    //Pushing to Vec<String> value in employees Hash Map
    //If a hashmap is not present (Vacant) then make a new one
    //Else (Occupied) then push to the existing Vec<String>.
    match employees.entry(department) {
        Entry::Vacant(names) => {names.insert(vec![name]);},
        Entry::Occupied(mut names) => {names.get_mut().push(name);}
    }

    employees
}

//Make it return it in aplphabetical order! 
fn get_people_departmen(mut employees: HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    let mut department = String::new();
    println!("Enter Department: ");
    io::stdin().read_line(&mut department).expect("Failed to read line!");
    department = department.trim().to_string();

    //clone because sort_department .entry requires a String and not a reference
    employees = sort_department(employees, department.clone()); 

    for (key, value) in &employees {
        if *key == department {
            for i in 0..value.len() {
                println!("{} : {}", value[i], key);
            }
        }
    }

    employees
}

fn sort_department(mut employees: HashMap<String, Vec<String>>, department: String) -> HashMap<String, Vec<String>> {
    //bubble sort for putting names into alphabetical order
    match employees.entry(department) {
        Entry::Vacant(_) => println!("No names in department!"),
        Entry::Occupied(mut names) => {
            let mut swapped = true;
            while swapped {
                swapped = false;
                for i in 1..names.get_mut().len() {
                    if names.get()[i-1] > names.get()[i] {
                        names.get_mut().swap(i-1, i);
                        swapped = true;
                    }
                }
            }
        }
    }
    employees
}

fn get_people(employees: HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    println!("{:?}", employees);
    employees
}



