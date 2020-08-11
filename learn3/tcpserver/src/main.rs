
//引用io模块
use std::io::prelude::*;
//引入io库的内容读取BufReader对象
use std::io::BufReader;
//引入io库的内容写对象BufWriter
use std::io::BufWriter;
//引用tcp socket链接对象TcpListener
use std::net::TcpListener;
//引用线程
use std::thread;

//程序入口 main函数
fn main() {

    //创建本地ip127.0.0.1端口8080，作为tcpsocket服务器
    let l = TcpListener::bind("127.0.0.1:8080").unwrap();

    println!("start...");

    //通过for迭代器，获取服务器的连接信息
    for stream in l.incoming() {

        
        //启m动一个线程
        thread::spawn(move || {

            //获取客户端连接流
            let stream = stream.unwrap();
            //创建读对象
            let reader = BufReader::new(&stream);
            //创建写对象
            let mut writer = BufWriter::new(&stream);

            //返回Lines迭代器
            for line in reader.lines() {

                //获取信息中的一行信息
                let line = line.unwrap();

                //打印信息
                println!("{}", "收到客户端请求");
                println!("{}", line);
                //通过write对象向客户端输出echo
                writer.write_all(b"echo\n").unwrap();

                //刷新输出流将所有流提交客户端
                writer.flush().unwrap();
            }
        });
    }
}
