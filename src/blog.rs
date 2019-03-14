#![allow(unused)]
use chrono::{DateTime, Local};
use pulldown_cmark::{html, Parser};
use serde::{Deserialize, Serialize};
use std::fs::{create_dir, read_dir, read_to_string, write, File, ReadDir};
use std::io;
use std::io::prelude::*;
use std::io::Result;
use std::path::Path;
use toml;

#[derive(Deserialize, Serialize, Debug)]
struct ArticleHeader {
    title: String,
    author: String,
    date: String,
    tags: Vec<String>,
    categories: Vec<String>,
    slug: String,
}

fn file_config() -> String {
    let mut fb = File::open("./src/template/file-base.toml").expect("没找到配置文件");
    let mut s = String::new();
    // 读取文本
    fb.read_to_string(&mut s)
        .expect("读取配置文件内容失败");
    // 解析预设置，文件头的内容
    let mut article_header: ArticleHeader = toml::from_str(&s).unwrap();
    // 设置文档创建时间
    let date: DateTime<Local> = Local::now();
    let date: String = date.to_rfc3339();
    article_header.date = date;
    // println!("{:?}", article_header);
    // 重新吐出tom转成的字符串
    toml::to_string(&article_header).unwrap()
}

// 初始化项目
pub fn init(projectname: String) {
    // 读取md配置文件
    println!("创建项目");
    // 创建一个目录
    create_dir(projectname).expect("创建目录失败😢");
}

// 创建md文件
pub fn new(filename: String) {
    // 读取md配置文件
    let file_config = file_config();
    // println!("{:?}", file_config);
    let mut new_file_path = String::from("./test/");
    new_file_path += &filename;

    // 新文件的内容
    let mut new_file_header = String::from("---\n");
    new_file_header.push_str(&file_config);
    new_file_header.push_str("---");
    write(new_file_path, new_file_header).expect("写入失败");
}

// 编译md文件到html
fn md_to_html(path: String) {
    let file_string = read_to_string(path).unwrap();
    // 拆分头部/markdown，
    let file_split: Vec<&str> = file_string.split("\n---\n").collect();
    let file_head: String = String::from(file_split[0]).replace("---", "");
    let file_content: String = String::from(file_split[1]);
    // 解析头部toml
    let file_head: ArticleHeader = toml::from_str(&file_head.trim()).unwrap();
    // 解析markdown
    let file_content = Parser::new(&file_content);
    let mut html_buf = String::new();
    html::push_html(&mut html_buf, file_content);  
    // 拼接html

    // 输出
    let mut file_name = String::from(file_head.title);
    file_name.insert_str(0, "./build/");
    file_name.push_str(".html");
    write(file_name, html_buf).expect("写入失败");
    // println!("{:?},{}", file_head, html_buf);
}
pub fn build() {
    let paths = read_dir("./test").unwrap();
    // read_dir("./test") 返回一个Result<ReadDir>
    // read_dir("./test").unwrap() 使用Result的unwrap方法返回ReadDir(迭代目录中的条目)
    // println!("paths:{:?}", paths);
    for file in paths {
        // file     返回一个Result<DirEntry>结果
        // file.unwrap()    使用Result的unwrap方法返回DirEntry
        // file.unwrap().path()    返回此条目表示的文件的完整路径PathBuf
        // println!("{:?}", file.unwrap().path());
        let file_path = file.unwrap().path(); // 链式，又不是真正的链式，如果返回一个新的类型，那就不能继续链式了
        match file_path.to_str() {
            Some(f) => md_to_html(String::from(f)),
            None => println!("error"),
        }
    }
}

// 创建服务器
pub fn server() {}
