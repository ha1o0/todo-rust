use std::collections::HashMap;

#[derive(Debug)]
#[allow(dead_code)]
struct Todo {
    id: usize,
    content: String,
    done: bool,
}

fn main() {
    println!("Hello, world!");
    let mut todos = HashMap::new();
    let todo1 = Todo {
        id: 1,
        content: "English".to_string(),
        done: false,
    };
    todos.insert(1, todo1);
    let mut next_id = 2;
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let cmd = input.trim();
        let mut parts = cmd.split_whitespace();
        if let Some(cmd) = parts.next() {
            match cmd {
                "LIST" => {
                    println!("{:?}", todos);
                }
                "ADD" => {
                    if let Some(add_content) = parts.next() {
                        let todo2 = Todo {
                            id: next_id,
                            content: add_content.to_string(),
                            done: false,
                        };
                        todos.insert(next_id, todo2);
                        next_id += 1;
                    }
                }
                "GET" => match parts.next() {
                    Some(keystr) => {
                        let key = keystr.parse::<usize>().unwrap_or(0);
                        if let Some(value) = todos.get(&key) {
                            println!("{:?}", value);
                        } else {
                            println!("Error: {} not found", key);
                        }
                    }
                    None => {
                        println!("Error: missing key id");
                    }
                },
                "DEL" => match parts.next() {
                    Some(keystr) => {
                        let key = keystr.parse::<usize>().unwrap_or(0);
                        if let Some(_) = todos.remove(&key) {
                            println!("Delete {} success", key);
                        } else {
                            println!("Error: {} not found", key);
                        }
                    }
                    None => {
                        println!("Error: missing key id");
                    }
                },
                _ => {}
            }
        }
    }
}
