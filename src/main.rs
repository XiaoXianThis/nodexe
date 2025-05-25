use std::env;
use std::env::{args, current_exe};
use std::fs::{remove_file, File};
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;

fn main() {
    // 读取参数
    let args: Vec<String> = args().collect();
    // 如果有参数，打包模式
    if args.len() > 1 {
        package_js(&args[1])
    }
    // 没有参数，运行模式
    else {
        run_self()
    }
}

// 读取自身末尾的 js 文件，并调用 node.js 执行
fn run_self() {
    // 读取自身
    let buffer = read_self();
    // 获取分隔符
    let mark = get_mark_split();
    let target = mark.as_bytes();
    // 反向遍历文件，找到分隔符位置
    let pos = buffer.windows(target.len()).rposition( |window| window == target ).unwrap_or(0);
    if pos > 0 {
        // 分割出 js 部分的 buffer
        let js_part_buffer = &buffer[pos..];
        // 去除分隔符，只保留 js 部分
        let js_buffer = &js_part_buffer[target.len()..];
        // 转为字符串
        let js_str = String::from_utf8_lossy(js_buffer);
        // 保存临时文件到当前路径
        let file_name = "_temp.js";
        let mut js_file = File::create(file_name).unwrap();
        let _ = js_file.write_all(js_buffer);
        // 调用 node 执行 js
        let output = Command::new("node")
            .arg(file_name)
            .output()
            .expect("运行失败：未找到 Node.js，请检查是否安装。");
        // 执行完成，删除文件
        remove_file(file_name).unwrap();
        // 打印错误
        print!("{}", String::from_utf8_lossy(&output.stderr));
        // 打印输出
        print!("{}", String::from_utf8_lossy(&output.stdout));
    }
    else {
        println!("运行失败：程序中未嵌入js");
    }
}

// 打包 js
fn package_js(path: &String) {
    // 如果文件路径存在
    if Path::new(path).exists() {
        // 且是js文件
        if (path.ends_with(".js") || path.ends_with(".mjs")) {
            // 读取自身
            let buffer = read_self();
            // 获取分隔符
            let mark = get_mark_split();
            let mark_buffer = mark.as_bytes();
            // 打开JS
            let mut js_file = File::open(path).unwrap();
            // 读取内容
            let mut js_buffer = Vec::new();
            let _ = js_file.read_to_end(&mut js_buffer);
            // 写文件
            let mut js_file = File::create(format!("{}{}", path.replace(".js", "").replace(".mjs", ""), if env::consts::OS == "windows" { ".exe" } else { "" }  )).unwrap();
            let _ = js_file.write_all( [ &buffer[..], &mark_buffer[..], &js_buffer[..] ].concat().as_slice() );
        } else {
            println!("{} 不是js文件", path);
        }
    } else { 
        println!("文件不存在：{}", path);
    }
}

// 读取自身
fn read_self() -> Vec<u8> {
    // 获取可执行文件路径
    let exe_path = current_exe().unwrap();
    // 打开文件
    let mut file = File::open(exe_path).unwrap();
    // 读取到 buffer
    let mut buffer = Vec::new();
    let _ = file.read_to_end(&mut buffer);
    return buffer;
}

// 获取分隔符
fn get_mark_split() -> String {
    // 拼出分隔符
    let mark_split = "###";
    let mark_name = "JS_START";
    let mark = format!("{}{}{}", mark_split, mark_name, mark_split);
    return mark;
}

