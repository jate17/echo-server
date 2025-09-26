use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::net::Shutdown;

fn handle(mut stream: TcpStream)  -> std::io::Result<()> {
    let mut buffer = [0u8; 512];
    stream.write_all(b"Helo, ECHO Server By J\n\n");
    loop {

        let n = stream.read(&mut buffer)?;
        if n == 0{
            break;
        }

        let cmd = String::from_utf8_lossy(&buffer[..n]).trim_end().to_string();
        if cmd == "exit"{
            let conn = stream.peer_addr();
            stream.write_all(b"By Dear User\n");
            stream.shutdown(Shutdown::Both)?;
            println!("[DORPCON] {:?}",conn);
        }


        stream.write_all(&buffer[..n])?;
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7")?;



    for stream in listener.incoming() {

        let mut stream = stream?;                // qui estrai il TcpStream dal Result
        let peer = stream.peer_addr()?;          // ora puoi usarlo
        println!("[NEWCON] {:?}", peer);

        handle(stream);
    }




    Ok(())
}
