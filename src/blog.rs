// #![allow(unused)]
use chrono::{DateTime, Local};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper_staticfile::Static;
use pulldown_cmark::Parser; // markdown to html
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::fs::{self, create_dir, read_dir, DirEntry, File};
use std::io;
use std::io::prelude::*;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use tera::Tera; // html template
use toml;

#[derive(Debug, Serialize, Eq, PartialEq, Ord, PartialOrd)]
pub enum PathType {
    Dir,
    SymlinkDir,
    File,
    SymlinkFile,
}

#[derive(Debug, Serialize, Eq, PartialEq, Ord, PartialOrd)]
struct Item {
    path_type: PathType,
    name: String,
    path: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ArticleMeta {
    title: String,
    author: String,
    date: String,
    tags: Vec<String>,
    categories: Vec<String>,
    slogan: String,
}

// 静态文件目录
static STATIC_FOLDER: &str = "src/static";
// 模版文件目录
static TEMPLATE_FOLDER: &str = "src/template";
// 文章文件目录
static MARKDOWN_FOLDER: &str = "markdown";
// 构建目录
static BUILD_FOLDER: &str = "build";

// 解析文件后缀
fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}
// 包含后缀有这些的文件
fn has_include_ext(p: &str) -> bool {
    let matches: [&str; 3] = [".js", ".css", ".html"];
    let mut b: bool = false;
    for elem in matches {
        if p.contains(elem) {
            b = true;
        }
    }
    b
}
// 获取path类型
fn get_type(p: PathBuf) -> PathType {
    p.symlink_metadata()
        .map(|meta| {
            let is_symlink = meta.file_type().is_symlink();
            let _is_dir = p.is_dir();
            match (is_symlink, _is_dir) {
                (true, true) => PathType::SymlinkDir,
                (false, true) => PathType::Dir,
                (true, false) => PathType::SymlinkFile,
                (false, false) => PathType::File,
            }
        })
        .unwrap_or(PathType::File)
}
// 复制静态文件到构建目录
fn copy_static_file() -> Result<(), io::Error> {
    let paths = fs::read_dir(STATIC_FOLDER).unwrap();
    for p in paths {
        let _path: DirEntry = p.unwrap();
        let _path: PathBuf = _path.path();
        // 获取路径信息
        let path_attr: fs::Metadata = fs::metadata(&_path).unwrap();
        // 判定是否是目录
        let _is_dir: bool = path_attr.is_dir();
        if _is_dir {
            // let move_paths = fs::read_dir(&_path);
            println!("跳过目录{:?}", _path);
            continue;
        }
        // Option 是一个枚举类型，用于有 “不存在” 的可能性的情况
        let file_name: Option<&OsStr> = _path.file_name();
        // 使用match显式脱衣, 解包 `Some` 将取出被包装的值
        let file_name_os_str: &OsStr = match file_name {
            // Some找到一个属于 T 类型的元素
            Some(t) => t,
            None => OsStr::new(""), // None 找不到相应元素
        };
        let file_name_op_str: Option<&str> = file_name_os_str.to_str();
        let file_name_str: &str = file_name_op_str.unwrap(); // 使用unwrap隐式脱衣，不推荐写法不安全
        let ext: Option<&str> = get_extension_from_filename(file_name_str);
        if ext == None {
            // 跳过没有文件后缀的，比如.DS_Store
            continue;
        }
        println!("拷贝文件{:?}", _path);
        let build_folder = PathBuf::from(BUILD_FOLDER).join(file_name_str); // 思考题：为啥PathBuf可以join另一种类型OsStr？https://doc.rust-lang.org/stable/std/ffi/struct.OsStr.html, https://kaisery.github.io/trpl-zh-cn/ch10-00-generics.html
                                                                            // 复制
        fs::copy(_path, build_folder)?;
    }
    Ok(())
}
// 编译.md文件到.html
fn md_to_html(_path: PathBuf) {
    // 读取路径下的文件内容
    let file_string: String = fs::read_to_string(_path).unwrap();
    // 根据换行符，拆分头部、内容
    let file_part: Vec<&str> = file_string.split("\n---\n").collect(); // 投机取巧，用\n标识分割线
    let file_head_string: String = file_part[0].replace("---", "");
    let mut file_content_str: &str = ""; // 内容预设空
    if file_part.len() > 1 {
        //len长度大于1，我们认为有内容，赋值
        file_content_str = file_part[1];
    }
    // println!("file_head_string:{:?}", file_head_string);
    // 解析头部toml信息
    let file_head: ArticleMeta = toml::from_str(file_head_string.trim()).unwrap();
    println!("file_head:{:?}", file_head);
    // 解析markdown内容
    let file_content: Parser = Parser::new(file_content_str);
    let mut html_content: String = String::new();
    pulldown_cmark::html::push_html(&mut html_content, file_content);
    // 读取模版，填入内容
    let html_template =
        fs::read_to_string(PathBuf::from(TEMPLATE_FOLDER).join("layout/blog.html")).unwrap();
    let mut page_data = tera::Context::new();
    page_data.insert("file_head", &file_head);
    page_data.insert("file_content", &html_content);
    let html_string = Tera::one_off(&html_template, &page_data, false).unwrap();
    println!("{:?}", html_content);
    // 输出html文件
    let out_file: PathBuf = PathBuf::from(BUILD_FOLDER).join(file_head.title + ".html");
    fs::write(out_file, html_string).expect("构建html失败😵");
}
// 创建命令
pub fn new(filename: String) -> Result<(), io::Error> {
    // markdown路径存在判定
    if !Path::new(MARKDOWN_FOLDER).exists() {
        print!("创建新markdown目录");
        create_dir(MARKDOWN_FOLDER).expect("期望参数是一个字符串路径");
    }
    // 读取md配置文件
    let file_base_path = PathBuf::from(TEMPLATE_FOLDER).join("file-base.toml");
    let mut file_base = fs::File::open(file_base_path)?; // 问号，捕获错误
    let mut file_base_string: String = String::new();
    // 读取配置内容，缓冲读取
    file_base
        .read_to_string(&mut file_base_string)
        .expect("读取配置文件内容失败😵");
    // 解析配置，并且更新标题和日期
    let mut article_meta: ArticleMeta = toml::from_str(&file_base_string).unwrap();
    // 设置文档创建时间
    let date_now: DateTime<Local> = Local::now();
    article_meta.date = date_now.format("%Y-%m-%d %H:%M:%S").to_string(); // 日期
    article_meta.title = filename.clone().replace(".md", ""); // 标题
    let file_meta_toml: String = toml::to_string(&article_meta).unwrap(); // 重新反转为字符串
    let new_file_content: String = ["---\n", &file_meta_toml, "---\n"].join(""); // 新文件的meta
    let new_file_path = PathBuf::from(MARKDOWN_FOLDER).join(filename); // 新文件路径（拼接路径和文件名）
    fs::write(new_file_path, new_file_content).expect("创建文件失败😵");
    Ok(()) // Result结果，不要加分号；，想加分号，使用 return
}
// 构建命令
pub fn build() -> Result<(), io::Error> {
    // 创建build目录
    if Path::new(BUILD_FOLDER).exists() {
        // 存在，则清空构建目录
        fs::remove_dir_all(BUILD_FOLDER)?;
    }
    fs::create_dir(BUILD_FOLDER)?;
    // 编译目录下的所有markdown文件
    let paths = read_dir(MARKDOWN_FOLDER).unwrap();
    // read_dir(MARKDOWN_FOLDER) 返回一个Result<ReadDir>
    // read_dir(MARKDOWN_FOLDER).unwrap() 使用Result的unwrap方法返回ReadDir(迭代目录中的条目)
    // println!("paths:{:?}", paths);
    for file in paths {
        // file     返回一个Result<DirEntry>结果
        // file.unwrap()    使用Result的unwrap方法返回DirEntry
        // file.unwrap().path()    DirEntry有path的方法
        let file_path = file.unwrap().path(); // 链式，像不像jquery
        if file_path.file_name() != None {
            md_to_html(file_path);
        }
    }
    // 拷贝静态文件
    copy_static_file()?;
    Ok(())
}
// web服务入口
async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, std::io::Error> {
    let mut response = Response::new(Body::empty());
    let method = _req.method();
    let uri_path = _req.uri().path();
    if uri_path == "/" {
        // 首页渲染
        let paths = fs::read_dir(BUILD_FOLDER).unwrap();
        // 文件列表，数组未知长度用Vec
        let mut files: Vec<Item> = Vec::new();
        for p in paths {
            let path_origin = p.unwrap().path();
            let file_name = path_origin.file_name().unwrap();
            let file_name_as_str: &str = file_name.to_str().unwrap();
            let file_name_as_string: String = String::from(file_name_as_str);
            // 获取路径信息
            let path_info: fs::Metadata = fs::metadata(&path_origin).unwrap();
            // 判定是否是目录
            let _is_dir = path_info.is_dir();
            // if !_is_dir {
            // println!("文件{:?}", file_name_as_string);
            // if file_name_as_string.contains(".html") {
            let item = Item {
                path_type: get_type(path_origin),
                name: file_name_as_string.clone(),
                path: file_name_as_string.clone(),
            };
            files.push(item);
            // }
            // }
        }
        println!("文件列表{:?}", files);
        let mut page_data = tera::Context::new();
        page_data.insert("dir_name", uri_path);
        page_data.insert("files", &files);
        let html_template: String =
            fs::read_to_string(PathBuf::from(TEMPLATE_FOLDER).join("layout/index.html")).unwrap();
        let res: String = Tera::one_off(&html_template, &page_data, true)
            .unwrap_or_else(|e| format!("500 Internal server error: {}", e));
        *response.body_mut() = Body::from(res);
        Ok(response)
    } else if method == &Method::POST {
        // ajax请求
        let full_body = hyper::body::to_bytes(_req.into_body()).await.unwrap();
        *response.body_mut() = Body::from(full_body);
        Ok(response)
    } else if has_include_ext(uri_path) {
        // 静态文件
        let static_ = Static::new(Path::new(BUILD_FOLDER));
        static_.serve(_req).await
    } else {
        // 404
        *response.status_mut() = StatusCode::NOT_FOUND;
        Ok(response)
    }
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
        Ok::<_, std::io::Error>(service_fn(hello_world))
    });
    let server = Server::bind(&addr).serve(make_svc);
    // And now add a graceful shutdown signal...
    let graceful = server.with_graceful_shutdown(shutdown_signal());
    // Run this server for... forever!
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}
