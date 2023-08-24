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
    todos.insert(1, Todo {
        id: 1,
        content: "学英语".to_string(),
        done: false
     });
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
                        todos.insert(next_id, Todo { id: next_id, content: add_content.to_string(), done: false });
                        next_id += 1;
                    }
                }
                "GET" => {
                    match parts.next() {
                        Some(keystr) => {
                            let key = keystr.parse::<usize>().unwrap_or(0);
                            if let Some(value) = todos.get(&key) {
                                println!("{:?}", value);
                            } else {
                                println!("Error: {} not found", key);
                            }
                        },
                        None => {
                            println!("Error: missing key id")
                        }
                    }
                }
                "DEL" => {
                    match parts.next() {
                        Some(keystr) => {
                            let key = keystr.parse::<usize>().unwrap_or(0);
                            if let Some(_) = todos.remove(&key) {
                                println!("Delete {} success", key);
                            } else {
                                println!("Error: {} not found", key);
                            }
                        },
                        None => {
                            println!("Error: missing key id")
                        }
                    }
                }
                _ => {}
            }
         }

     }
}
