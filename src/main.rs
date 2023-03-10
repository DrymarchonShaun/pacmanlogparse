use std::fs;
use std::vec::Vec;
use regex::Regex;


fn main() {
    let file_path = "/var/log/pacman.log";


    let base_log = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut installed: Vec<String> = Vec::new();
    let mut reinstalled: Vec<String> = Vec::new();
    let mut removed: Vec<String> = Vec::new();

    for cap in Regex::new(r"\[\d{4}\-\d{2}\-\d{2}T\d{2}:\d{2}:\d{2}(?:\+|\-)\d{4}\] \[ALPM\] (installed|reinstalled|removed) (\S+\w)").unwrap().captures_iter(&base_log) {
        if cap[1].to_string() == "installed" {
            installed.push(cap[2].to_string());
        } else if cap[1].to_string() == "reinstalled" {
                reinstalled.push(cap[2].to_string()); 
            } else {
                removed.push(cap[2].to_string());
                };
        };
    println!("{:?}", installed);
    println!("{:?}", reinstalled);
    println!("{:?}", removed);
}