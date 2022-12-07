use std::{collections::HashMap, fs};

fn main() {
    let path = "input/input.txt";
    let contents = fs::read_to_string(path).expect("Error when reading the file");

    let file_system = create_file_system(&contents);
    let mut folder_size = HashMap::new();
    calculate_folder_size(&file_system, &"/".to_string(), &mut folder_size);

    let mut result1 = 0;
    for size in folder_size.values() {
        if *size < 100000 {
            result1 += size;
        }
    }

    let massimo = 70000000;
    let richiesto = 30000000;
    let usato = folder_size.get(&"/".to_string()).unwrap();
    let libero = massimo - usato;
    let needed = richiesto - libero;

    let mut pollo = folder_size
        .values()
        .filter(|x| **x > needed)
        .collect::<Vec<&usize>>();

    pollo.sort();
    let result2 = pollo[0];

    println!("Result part 1: {}", result1);
    println!("Result part 2: {}", result2);
}

fn calculate_folder_size(
    file_system: &HashMap<String, Vec<(&str, usize)>>,
    current_path: &String,
    folder_size: &mut HashMap<String, usize>,
) {
    let files = file_system.get(current_path).unwrap();
    let mut size = 0;
    //println!("{} {:?}", current_path, files);
    for (file, file_size) in files {
        if *file_size == 0 {
            let path;
            if current_path == "/" {
                path = format!("{}{}", current_path, file);
            } else {
                path = format!("{}/{}", current_path, file);
            }
            calculate_folder_size(file_system, &path, folder_size);
            size += folder_size.get(&path).unwrap();
        } else {
            size += file_size;
        }
    }
    folder_size.insert(current_path.clone(), size);
}

fn create_file_system(contents: &String) -> HashMap<String, Vec<(&str, usize)>> {
    let mut file_system = HashMap::new();
    let mut current_path = String::new();
    file_system.insert("/".to_string(), Vec::new());
    for line in contents.lines() {
        let mut line = line.split_whitespace();
        let first = line.next().unwrap();
        if first == "$" {
            let command = line.next().unwrap();
            if command == "cd" {
                let path = line.next().unwrap();
                if path == ".." {
                    let last_dir = current_path.split("/").last().unwrap();
                    current_path =
                        (&current_path[..current_path.len() - last_dir.len() - 1]).to_string();
                } else if path == "/" {
                    current_path = "/".to_string();
                } else {
                    if current_path == "/" {
                        current_path = format!("{}{}", current_path, path);
                    } else {
                        current_path = format!("{}/{}", current_path, path);
                    }
                }
            } else {
                //ls command, no action needed
            }
        } else if first == "dir" {
            let folder = line.next().unwrap();
            let full_name;
            if current_path == "/" {
                full_name = format!("{}{}", current_path, folder);
            } else {
                full_name = format!("{}/{}", current_path, folder);
            }
            file_system
                .get_mut(&current_path)
                .unwrap()
                .push((folder, 0));
            file_system.insert(full_name, Vec::new());
        } else {
            let size = first.parse::<usize>().unwrap();
            let file = line.next().unwrap();
            let files = file_system.get_mut(&current_path).unwrap();
            files.push((file, size));
        }
    }
    file_system
}
