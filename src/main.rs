use regex::Regex;
use std::fs;
use std::ops::Add;
use std::vec::Vec;


fn main() {
    let file_path = String::from("/var/log/pacman.log");

    let base_log = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut operation: Vec<String> = Vec::new();
    let mut package: Vec<String> = Vec::new();

    for cap in Regex::new(r"\[\d{4}\-\d{2}\-\d{2}T\d{2}:\d{2}:\d{2}(?:\+|\-)\d{4}\] \[ALPM\] (installed|removed) (\S+\w)").unwrap().captures_iter(&base_log) {
        operation.push(cap[1].to_string());
        package.push(cap[2].to_string());
    }

    fn pkg_mgr(operation: String, package: String) {
        let mut to_install: Vec<String> = Vec::new();
        if operation == "installed" {
            to_install.push(package);
        } else if operation == "removed" {
            let checkpkg = to_install.iter().position(|x| x == &package);
            if checkpkg != None {
                to_install.remove(checkpkg.unwrap());
            };
            };
        }
    if package.len() == operation.len() {
        let mut i = 0;
        while i < package.len() {
         packagelist = pkg_mgr(operation[i].to_string(), package[i].to_string());
            i += 1;
        }
        
    }
    }