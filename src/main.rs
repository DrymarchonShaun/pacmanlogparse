use regex::Regex;
use std::fs;
use std::vec::Vec;

fn main() {
    let log_path = String::from("/var/log/pacman.log");
    let file_path = String::from("./package_list");

    let base_log = fs::read_to_string(log_path).expect("Should have been able to read the file");
    let mut operation: Vec<String> = Vec::new();
    let mut package: Vec<String> = Vec::new();
    let mut package_list: Vec<String> = Vec::new();

    for cap in Regex::new(r"\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}[+\-]\d{4}] \[ALPM] (installed|removed) (\S+\w)").unwrap().captures_iter(&base_log) {
        operation.push(cap[1].to_string());
        package.push(cap[2].to_string());
    }

    if package.len() == operation.len() {
        let mut i = 0;
        for entry in package {
            if operation.get(i) == Some(&"installed".to_string()) {
                package_list.push(entry);
            } else if package_list.iter().position(|x| x == &entry) != None {
                package_list.remove(package_list.iter().position(|x| x == &entry).unwrap());
            };
            i += 1;
        }
        fs::write(
            file_path,
            format!("paru -S --noconfirm {}", package_list.join(" ")),
        )
        .expect("");
    }
}
