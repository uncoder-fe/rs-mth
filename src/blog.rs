// #![allow(unused)]
use chrono::{DateTime, Local};
use handlebars::Handlebars;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use pulldown_cmark::{html, Parser};
use serde::{Deserialize, Serialize};
use std::fs::{self, create_dir, read_dir};
use std::io;
use std::io::prelude::*;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use toml;

#[derive(Serialize, Deserialize, Debug)]
struct ArticleMeta {
    title: String,
    author: String,
    date: String,
    tags: Vec<String>,
    categories: Vec<String>,
    slug: String,
}
// 静态文件目录
static STATIC_FOLDER: &str = "src/static";
// 模版文件目录
static TEMPLATE_FOLDER: &str = "src/template";
// 文章文件目录
static MARKDOWN_FOLDER: &str = "markdown";
// 构建目录
static BUILD_FOLDER: &str = "build";

// 复制静态文件到构建目录
fn copy_static_file() -> Result<(), io::Error> {
    let paths = fs::read_dir(STATIC_FOLDER).unwrap();
    for p in paths {
        let path_origin = p.unwrap().path();
        // 获取路径信息
        let path_info = fs::metadata(&path_origin).unwrap();
        // 判定是否是目录
        let is_dir = path_info.is_dir();
        if is_dir {
            let move_paths = fs::read_dir(&path_origin);
            println!("目录{:?}", move_paths);
        } else {
            println!("文件{:?}", path_origin);
            let file_name = path_origin.file_name().unwrap();
            let build_folder = PathBuf::from(BUILD_FOLDER).join(file_name);
            // 复制
            fs::copy(path_origin, build_folder)?;
        }
    }
    Ok(())
}
// 编译.md文件到.html
fn md_to_html(path: PathBuf) {
    // 读取路径下的文件内容
    let file_string = fs::read_to_string(path).unwrap();
    // 拆分头部/内容
    let file_part: Vec<&str> = file_string.split("\n---\n").collect();
    let file_head_string = file_part[0].replace("---", "");
    let mut file_content_str = ""; // 内容
    if file_part.len() > 1 {
        file_content_str = file_part[1];
    }
    // 解析头部toml信息
    let file_head: ArticleMeta = toml::from_str(file_head_string.trim()).unwrap();
    // 解析内容
    let file_content = Parser::new(file_content_str);
    let mut html_content = String::new();
    html::push_html(&mut html_content, file_content);
    // 拼接html
    let temple = Handlebars::new();
    let html_header_template =
        fs::read_to_string(PathBuf::from(TEMPLATE_FOLDER).join("layout/header.html")).unwrap();
    let html_footer_template =
        fs::read_to_string(PathBuf::from(TEMPLATE_FOLDER).join("layout/footer.html")).unwrap();
    let mut html_string = temple
        .render_template(&html_header_template, &file_head)
        .unwrap();
    html_string.push_str(&html_content);
    html_string.push_str(&html_footer_template);
    // render without register
    // println!("{:?}", html_string);
    // 输出
    let new_file_path = PathBuf::from(BUILD_FOLDER).join(file_head.title + ".html");
    fs::write(new_file_path, html_string).expect("构建html失败😵");
    // println!("{:?},{}", file_head, html_buf);
}

// 初始化命令
pub fn init(project_name: String) {
    // 读取md配置文件
    println!("创建项目");
    // 创建一个目录
    fs::create_dir(project_name).expect("创建目录失败😵");
}
// 构建命令
pub fn build() -> Result<(), io::Error> {
    // 创建build目录
    if Path::new(BUILD_FOLDER).exists() {
        // 存在，则删除
        fs::remove_dir_all(BUILD_FOLDER)?;
    }
    fs::create_dir(BUILD_FOLDER)?;
    // 编译markdown
    let paths = read_dir(MARKDOWN_FOLDER).unwrap();
    // read_dir(MARKDOWN_FOLDER) 返回一个Result<ReadDir>
    // read_dir(MARKDOWN_FOLDER).unwrap() 使用Result的unwrap方法返回ReadDir(迭代目录中的条目)
    // println!("paths:{:?}", paths);
    for file in paths {
        // file     返回一个Result<DirEntry>结果
        // file.unwrap()    使用Result的unwrap方法返回DirEntry
        // file.unwrap().path()    返回此条目表示的文件的完整路径PathBuf
        // println!("{:?}", file.unwrap().path());
        let file_path = file.unwrap().path(); // 链式，又不是真正的链式，如果返回一个新的类型，那就不能继续链式了
        match file_path.to_str() {
            Some(_) => md_to_html(file_path),
            None => println!("不是路径哦😵"),
        }
    }
    // 拷贝静态文件
    copy_static_file()?;
    Ok(())
}
// 创建命令
pub fn new(filename: String) -> Result<(), io::Error> {
    // markdown路径存在判定
    if !Path::new(MARKDOWN_FOLDER).exists() {
        print!("没有目标目录，创建新目录");
        create_dir(MARKDOWN_FOLDER)?;
    }
    // 新文件的meta
    let mut new_file_meta = String::from("---\n");
    // 读取md配置文件
    let file_base_dir = PathBuf::from(TEMPLATE_FOLDER).join("file-base.toml");
    let mut file_base_config = fs::File::open(file_base_dir).expect("没找到配置文件");
    let mut content = String::new();
    // 读取配置内容
    file_base_config
        .read_to_string(&mut content)
        .expect("读取配置文件内容失败😵");
    // 解析配置
    let mut article_meta: ArticleMeta = toml::from_str(&content).unwrap();
    // 设置文档创建时间
    let date_now: DateTime<Local> = Local::now();
    let date: String = date_now.to_rfc3339();
    article_meta.date = date; // 日期
    article_meta.title = filename.clone(); // 标题
    // println!("{:?}", article_meta);
    // 重新吐出toml转成的字符串
    let file_meta = toml::to_string(&article_meta).unwrap();
    new_file_meta.push_str(&file_meta);
    new_file_meta.push_str("---");
    // 新文件路径（拼接路径和文件名）
    let new_file_path = PathBuf::from(MARKDOWN_FOLDER).join(filename);
    fs::write(new_file_path, new_file_meta).expect("创建文件失败😵");
    Ok(())
}

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::empty());
    let method = _req.method();
    let uri_path = _req.uri().path();
    match (method, uri_path) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        }
        (&Method::POST, "/echo") => {
            let full_body = hyper::body::to_bytes(_req.into_body()).await?;
            *response.body_mut() = Body::from(full_body);
        }
        _ => {
            println!("404!");
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };
    Ok(response)
}
async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}
// 启动服务器命令
#[tokio::main]
pub async fn serve() {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, hyper::Error>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // And now add a graceful shutdown signal...
    let graceful = server.with_graceful_shutdown(shutdown_signal());
    // Run this server for... forever!
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}
