use rayon::prelude::*;
use regex::Regex;
use std::{
    fs::OpenOptions, io::Write, os::unix::fs::FileExt, ptr::write_unaligned, rc::Rc, time::Duration,
};
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
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(20)
        .build_global()
        .expect("error");

    let mut site = read("output.txt");
    //let mut path = read("test.txt");

    let all = site.into_par_iter().for_each(|(x)| req(&x));
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

fn write_url(fname: &str, line: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(fname)
        .expect("file problem");

    writeln!(&file, "{}", line);
}

fn read(fname: &str) -> Vec<String> {
    let file = std::fs::read_to_string(fname).unwrap();
    let line: Vec<String> = file
        .lines()
        .map(|s| s.replace(r#"""#, "").replace(" ", "").to_string())
        .collect();

    // for li in &line {
    //   println!("{}", li);
    //}
    return line;
}

fn req(url: &str) {
    let mut path = read("test.txt");
    let re = Regex::new(r#"Stable tag:[ .0-9]{6}"#).unwrap();
    for i in path {
        //println!("{}", format!("https://{}{}", url, i));

        let req = reqwest::blocking::get(format!("https://{}/{}/readme.txt", url, i)).unwrap();
        //  let req = reqwest::blocking::get(
        //    "https://www.wpallimport.com//wp-content/plugins/advanced-cron-manager/readme.txt",
        // )
        // .unwrap();
        //println!("test {}", req.text().unwrap());
        if let Some(test) = re.captures(&req.text().unwrap()) {
            println!(
                "{} found {:?}",
                format!("https://{}/{}/readme.txt", url, i),
                test.get(0).unwrap().as_str()
            );
            write_url(
                "hits",
                format!(
                    "https://{}/{}/readme.txt | {}",
                    url,
                    i,
                    test.get(0).unwrap().as_str()
                ),
            );
        }

        //        if let Some(res) = re.captures(&req.text().unwrap()).unwrap().get(1) {
        //          println!("{}", format!("https://{}/{}", url, i));
        //    } else {
        //println!("{}  status code is {}", url, req.status());
        //  }

        //std::thread::sleep(Duration::from_secs(2));
    }
}
