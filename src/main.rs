// #[derive(Debug)]
use std::env;
mod blog;

fn getOwn(s: String) {
    println!("s:{}", s);
}

fn main() {
    // 正式编码之前
    let test = "字符串";
    // let test = "字符串".to_string();
    // getOwn(test);
    // println!("test{}", test);
    let test1 = test.to_string(); // &str -> String
    let test2 = &test1; // String -> &String
    let test3 = test2.as_str(); // &String -> &str
    println!("test{}, test1{}, test2{}, test3{}", test, test1, test2, test3);
    // 正式编码

    // 获取整条运行命令内容
    let argument: Vec<String> = env::args().collect();
    // print!("argument{:?}", argument);
    // 获取命令名称
    let command_name = &argument[2];
    match command_name as &str {
        "init" => {
            let project_name = &argument[3];
            blog::init(project_name.to_string());
        }
        "new" => {
            let file_name = &argument[3];
            blog::new(file_name.to_string());
        }
        "build" => blog::build(),
        "server" => blog::server(),
        _ => println!("something else!"),
    }
}
