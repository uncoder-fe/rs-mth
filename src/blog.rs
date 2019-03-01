#![allow(unused)]
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs::{create_dir, read_dir, write, File, ReadDir};
use std::io;
use std::io::prelude::*;
use std::io::Result;
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

// 编译md文件
fn md_to_html(file: String) {}
pub fn build() {
    let paths = read_dir("./test").unwrap();
    // read_dir("./test") 返回一个Result<ReadDir>
    // read_dir("./test").unwrap() 使用Result的unwrap方法返回ReadDir(迭代目录中的条目)
    // println!("paths:{:?}", paths);
    for path in paths {
        // path     返回一个Result<DirEntry>结果
        // path.unwrap()    使用Result的unwrap方法返回DirEntry
        // path.unwrap().path()    返回此条目表示的文件的完整路径PathBuf
        println!("{:?}", path.unwrap().path());
    }
}

// 创建服务器
pub fn server() {}
