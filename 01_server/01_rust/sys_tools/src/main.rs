// use std::env;
use std::path::Path;

extern crate regex;

use regex::Regex;


fn main() {
    // get_tree("/Users/honghaitao/code/open_project/sys_tools/");

    let name = get_file_name("/Users/honghaitao/code/open_project/sys_tools/02_client/01_angular/sys_tools/src/environments/environment.ts");
    println!("13: {}", name)
    // env::set_var("RUST_BACKTRACE", "full");
    // use std::path::Path;
    // let path = Path::new("/Users/honghaitao/code/open_project/sys_tools/");
    // for entry in path.read_dir().expect("read_dir call failed") {
    //     if let Ok(entry) = entry {
    //         println!("{:?}", entry.path());
    //     }
    // }
    // assert_eq!(, true);
    // println!("{}", Path::new("/Users/honghaitao/code/open_project/sys_tools/01_server/01_rust/sys_tools/Cargo.toml").is_dir())
}

pub fn get_tree(mut path_str: &str) {
    let path = Path::new(&mut path_str);
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let mut str_path = entry.path().display().to_string();
            let re = Regex::new(r"node_modules|\.(git|vscode|vscode|angular|idea)|target/debug").unwrap();
            if entry.path().is_dir() & !re.is_match(&mut str_path) {
                println!("{:?}", entry.path());
                get_tree(&mut str_path);
            }
        }
    }
}

pub fn get_file_name(file_path: &str) -> &str {
    let mut file_name = "";
    for sre_0 in file_path.split("/") {
        file_name = sre_0;
    }
    println!("{}", file_name.replace(".*", ""));
    println!("{}", file_name.replace(".*", ""));
    // println!("{:}",format!(""));
    let str5 = "\t\t\tHello\n\r";
    println!("{:}",str5.trim());
    return file_name;
}
