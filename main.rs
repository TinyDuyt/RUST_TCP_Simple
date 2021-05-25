use std::{borrow::Borrow, fs, net::TcpListener};
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
fn main(){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();  //对该地址的请求进行监听

    for stream in listener.incoming(){  //遍历所有对该地址的请求
        let stream = stream.unwrap();               

        thread::spawn(|| {   //多线程
            handle_connection(stream);//调用handle_connection
        }); 


    } 

}


fn handle_connection(mut stream:TcpStream){
    let mut buffer = [0;1024]; //在stack上创建缓存buffer

    stream.read(&mut buffer).unwrap(); //将请求信息写入buffer

    let get= b"GET / HTTP/1.1\r\n";  //b""为字节字符串转换器，将str转换为字节字符串[u8; 16]

    let(status_line,filename) = if buffer.starts_with(get) {  //对请求信息类型及信息地址是否存在进行判断
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    }else{
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    
    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}",status_line,contents);  //拼接

    stream.write(response.as_bytes()).unwrap();  //将响应内容以字节形式写入writer
    

    stream.flush().unwrap();  //将writer中的响应内容发送给浏览器    



       
    //println!("Request: {}",String::from_utf8_lossy(&buffer[..]));

}

