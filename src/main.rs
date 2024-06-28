use regex::Regex;
use std::{fs::OpenOptions, io::Write, os::unix::fs::FileExt, ptr::write_unaligned};
fn main() {
    /*
    let req=reqwest::blocking::Client::new().get("https://plugins.svn.wordpress.org/").header("User-Agent","Mozilla/5.0 (Linux;Android 13; SM-S908B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36").send().unwrap().text().unwrap();
    let re = Regex::new(r#"href="([^"]+)""#).unwrap();
    // println!("test is {:?}", req);
    let res: Vec<_> = re
        .find_iter(&req)
        .map(|t| {
            let f = t.as_str();
            let url = &f[6..f.len() - 1];
            url.to_string()
        })
        .collect();

    for i in res {
        write_file("test.txt", i);
        // println!("the res is {}", i);
    }
    */
    read();
}
fn write_file(fname: &str, line: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(fname)
        .expect("File problem");
    writeln!(&file, "/wp-content/plugins/{}", line);
}

fn read() -> Vec<String> {
    let file = std::fs::read_to_string("res").unwrap();
    let line: Vec<String> = file
        .lines()
        .map(|s| s.replace(r#"""#, "").to_string())
        .collect();

    for li in &line {
        println!("{}", li);
    }
    return line;
}


fn req(url:&str,path:&str){

    let req=reqwest::blocking::get(format!("https://{}"))
}
