use std::{collections::HashMap, io};

fn main() {
    println!("#Hash task");

    let depart_flag = "to";
    let show_all_flag = "all";
    let show_from_flag = "from";

    let add_opration = "add";
    let show_operation = "show";

    let mut table: HashMap<String, Vec<String>> = HashMap::new();

    fn get_next_word(words: &Vec<&str>, target: &str) -> String {
        if let Some(index) = words.iter().position(|&word| word == target) {
            if index + 1 < words.len() {
                String::from(words[index + 1])
            } else {
                println!("No word found after '{}'", target);
                String::new()
            }
        } else {
            println!("No word found after '{}'", target);
            String::new()
        }
    }

    loop {
        println!("Please, input Name and department to add");

        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .unwrap_or_else(|_error| panic!("Problem with reading STDIN"));

        let command: String = match input.trim().parse() {
            Ok(command) => command,
            Err(_) => continue,
        };

        let operation = match command.split_whitespace().next() {
            Some(operation) => operation.to_lowercase(),
            None => {
                println!("No operation find :(");
                String::new();
                continue;
            }
        };

        let words: Vec<&str> = command.split_whitespace().collect();

        if operation == add_opration {
            let department: String = get_next_word(&words, &depart_flag);
            let name: String = get_next_word(&words, &add_opration);
            let departament_vec = table.get_mut(&department);

            match departament_vec {
                Some(vec) => {
                    vec.push(name);
                    println!("Работник добавлен в отдел: {}", department);
                },

                None => {
                    let new_vec = vec![name];
                    table.insert(department.clone(), new_vec);
                    println!("Создан новый отдел! Работник добавлен в отдел: {}", department);
                }
            }
        }

        if operation == show_operation {
            let show_kind: String = get_next_word(&words, show_operation);
            if show_kind == show_all_flag {
                let departments: Vec<_> = table.keys().collect();
                for department in departments {
                    let mut names = table.get(department).unwrap().clone();
                    names.sort();

                    println!("Отдел: {}", department);

                    for name in names {
                        println!("{}", name);
                    }
                }
            }

            if show_kind == show_from_flag {
                let from_department: String = get_next_word(&words, show_from_flag);
                let all_names = table.get_mut(&from_department);
                match all_names {
                    Some(names) => {
                        names.sort();
                        println!("Отдел: {}", from_department);
                        for name in names {
                            println!("{}", name);
                        }
                    },
                    None => println!("no employees in the {} department", from_department),
                } 
            }
        }
    }
}
