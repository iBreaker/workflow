use std::{process::{Command, Stdio}, io::Read};
use std::io::{Write, stdin};


fn main(){
    let mut child = Command::new("/bin/bash").stdin(Stdio::piped()).
        spawn().expect("exec error");

    let mut child_stdin = child.stdin.take().unwrap();

    loop {
        let mut buf = [0;10];
        
        if let None = stdin().read(&mut buf[..]).ok(){
            break;
        }

        if let Err(e) = child_stdin.write(&buf){
            println!("{:}", e);
            break;
        }
    }

    let status = child.wait().expect("wait child error");
    println!("{:}", status)
}