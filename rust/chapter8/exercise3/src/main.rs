/*
Using a hash map and vectors, 
create a text interface to allow a user to add 
employee names to a department in a company. 
For example, “Add Sally to Engineering” or 
“Add Amir to Sales.” Then let the user retrieve 
a list of all people in a department or all people 
in the company by department, sorted alphabetically.
*/
use std::io;
use std::collections::HashMap;

fn main() {
    /*
    NOTE: With VIEW_ALL, I print the entire database
    and, as hashmaps are unsorted by nature, I kept it as is.

    With VIEW, I print each name sorted alphabetically and, 
    if some names are capitalized and some aren't, you might
    get unexpected behaviour.
    */
    let mut database : HashMap<String, Vec<String>> = HashMap::new();
    loop{
        let mut input = String::new();
        println!("Enter department name and employee name separated by a hypen (i.e. engineering-dan)");
        println!("Enter VIEW:<department_name> to view people in that department (i.e. VIEW:engineering)");
        println!("Enter VIEW_ALL to view all people (i.e. VIEW_ALL)");
        io::stdin().read_line(&mut input)
            .expect("ERROR");

        let mut hyphen_index = 0;
        input = input.trim().to_string();
        match input.find("-"){
            Some(i) => {
                hyphen_index = i;
                database.entry(input[0..hyphen_index].to_string())
                    .or_default()
                    .push(input[hyphen_index+1..].to_string());
            },
            None => {
                if input == "VIEW_ALL".to_string(){
                    println!("VIEW_ALL
                    {:?}", database);
                } else if input.contains("VIEW"){
                    for (key, val) in &database{
                        if key == &input[5..].to_string(){
                            let mut sorted_val = val.clone();
                            sorted_val.sort();
                            println!("{:?}", sorted_val);
                        }
                    }
                }
            },
        }
        println!("------------------------------------------------");
    }
}
