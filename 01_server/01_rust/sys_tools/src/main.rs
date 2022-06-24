// use std::env;
use std::path::Path;

fn get_tree(mut path_str: &str) {
    let path = Path::new(&mut path_str);

    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {

            println!("{:?}", entry.path().is_dir());
            if (entry.path().is_dir())
        }
    }
}

fn main() {
    get_tree("/Users/honghaitao/code/open_project/sys_tools/");
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