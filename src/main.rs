// #[derive(Debug)]
#![allow(unused)]
use std::env;
mod blog;

fn get_own(s: String) {
    println!("s:{}", s);
}
fn get_own2(s: &str) {
    println!("s:{}", s);
}

fn main() {
    // 正式编码之前
    let test: &str = "字符串";
    // let test: String = "字符串".to_string();
    // get_own(test);
    // println!("test{}", test);
    let test1: String = test.to_string(); // &str -> String
    let test2: &String = &test1; // String -> &String
    let test3: &str = test2.as_str(); // &String -> &str
    println!(
        "test{}, test1{:}, test2{:?}, test3{}",
        test, test1, test2, test3
    );
    // 正式编码

    // 获取整条运行命令内容
    let argument: Vec<String> = env::args().collect();
    // print!("argument{:?}", argument);
    // 获取命令名称
    let command_name = &argument[2];
    match command_name as &str {
        "new" => {
            let file_name = &argument[3];
            blog::new(file_name.to_string()).unwrap();
        }
        "build" => {
            blog::build().unwrap();
        }
        "serve" => blog::serve(),
        _ => println!("啊，不存在的指令!"),
    }
}
