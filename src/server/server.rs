use std::sync::Arc;

use crate::core::{
    cmd::{self, Commands},
    db::DB,
    resp_encoder, resp_parser,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::RwLock,
};

pub struct Server;

impl Server {
    pub async fn run_server(&self) {
        let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
        let db = Arc::new(RwLock::new(DB::new()));
        loop {
            if let Ok((stream, _)) = listener.accept().await {
                println!("Connection established");

                let _ = stream.set_nodelay(true);

                let db = db.clone();
                tokio::spawn(async move {
                    handle_process(stream, db).await;
                });
            }
        }
    }
}

async fn handle_process(mut stream: TcpStream, db: Arc<RwLock<DB>>) {
    let mut buffer = [0; 500];
    loop {
        let n = match stream.read(&mut buffer).await {
            Ok(n) if n == 0 => return, // Connection closed
            Ok(n) => n,
            Err(_) => {
                println!("Error reading from stream");
                return;
            }
        };

        // Read command and arguments from buffer
        println!("Received: {:?}", String::from_utf8_lossy(&buffer[..n]));
        let (_length, length_till) = resp_parser::read_length(&buffer[..n]);
        let (command, command_till) = resp_parser::read_simple_string(&buffer[length_till..n]);

        match command.to_lowercase().as_str() {
            "get" => {
                if let Some(args) = parse_arguments(&buffer, length_till, command_till, n) {
                    let cmd = Commands::new(command, args);
                    let value = cmd.handle_get_command(db.clone()).await;
                    let response = match value {
                        Some(v) => format!("+{}\r\n", v),
                        None => "$-1\r\n".to_string(),
                    };
                    stream.write_all(response.as_bytes()).await.unwrap();
                } else {
                    send_argument_error(&mut stream, "get").await;
                }
            }
            "set" => {
                if let Some(args) = parse_arguments(&buffer, length_till, command_till, n) {
                    if args.len() < 2 {
                        send_argument_error(&mut stream, "set").await;
                        continue;
                    }
                    let cmd = Commands::new(command, args);
                    match cmd.handle_set_command(db.clone()).await {
                        Ok(_) => {}
                        Err(_) => {
                            stream
                                .write_all("-ERR syntax error\r\n".as_bytes())
                                .await
                                .unwrap();
                            continue;
                        }
                    }
                    stream.write_all("+OK\r\n".as_bytes()).await.unwrap();
                } else {
                    send_argument_error(&mut stream, "set").await;
                }
            }
            "del" => {
                if let Some(args) = parse_arguments(&buffer, length_till, command_till, n) {
                    let cmd = Commands::new(command, args);
                    cmd.handle_del_command(db.clone()).await;
                    stream.write_all("+OK\r\n".as_bytes()).await.unwrap();
                } else {
                    send_argument_error(&mut stream, "del").await;
                }
            }
            "expire" => {}
            "exists" => {
                if let Some(key) = parse_arguments(&buffer, length_till, command_till, n) {
                    let cmd = Commands::new(command, key);
                    let value = cmd.handle_exists_command(db.clone()).await;

                    let response = format!(":{}\r\n", value);
                    stream.write_all(response.as_bytes()).await.unwrap();
                } else {
                    send_argument_error(&mut stream, "exists").await;
                }
            }
            "keys" => {
                if let Some(args) = parse_arguments(&buffer, length_till, command_till, n) {
                    let cmd = Commands::new(command, args);
                    let value = cmd.handle_keys_command(db.clone()).await;
                    let response = resp_encoder::encode_array_response(value);
                    stream.write_all(response.as_bytes()).await.unwrap();
                } else {
                    send_argument_error(&mut stream, "keys").await;
                }
            }

            "incr" => {
                if let Some(args) = parse_arguments(&buffer, length_till, command_till, n) {
                    let cmd = Commands::new(command, args);
                    let value = cmd.handle_incr_command(db.clone()).await;
                    let response = match value {
                        Ok(v) => format!(":{}\r\n", v),
                        Err(_) => "-ERR value is not an integer or out of range\r\n".to_string(),
                    };

                    stream.write_all(response.as_bytes()).await.unwrap();
                } else {
                    send_argument_error(&mut stream, "incr").await;
                }
            }
            "decr" => {
                if let Some(args) = parse_arguments(&buffer, length_till, command_till, n) {
                    let cmd = Commands::new(command, args);
                    let value = cmd.handle_decr_command(db.clone()).await;
                    let response = match value {
                        Ok(v) => format!(":{}\r\n", v),
                        Err(_) => "-ERR value is not an integer or out of range\r\n".to_string(),
                    };
                    stream.write_all(response.as_bytes()).await.unwrap();
                } else {
                    send_argument_error(&mut stream, "decr").await;
                }
            }

            "quit" => {
                stream.write_all("+OK\r\n".as_bytes()).await.unwrap();
                return;
            }
            "ping" => {
                let response =
                    if let Some(arg) = parse_arguments(&buffer, length_till, command_till, n) {
                        Commands::new(command, arg).handle_ping_command()
                    } else {
                        Commands::new(command, vec![]).handle_ping_command()
                    };
                stream
                    .write_all(format!("+{}\r\n", response).as_bytes())
                    .await
                    .unwrap();
            }
            "echo" => {
                if let Some(arg) = parse_arguments(&buffer, length_till, command_till, n) {
                    let cmd = Commands::new(command, arg);
                    let value = cmd.handle_ping_command();
                    stream
                        .write_all(format!("+{}\r\n", value).as_bytes())
                        .await
                        .unwrap();
                } else {
                    send_argument_error(&mut stream, "echo").await;
                }
            }
            "ttl" => {}
            "flushall" => {
                let cmd = Commands::new(command, vec![]);
                cmd.handle_flushall_command(db.clone()).await;
                stream.write_all("+OK\r\n".as_bytes()).await.unwrap();
            }
            "command" | "client" => {
                stream.write_all("+OK\r\n".as_bytes()).await.unwrap();
            }
            _ => {
                stream
                    .write_all(
                        format!(
                            "-ERR unknown command `{}`, with args beginning with:{}\r\n",
                            command,
                            String::from_utf8_lossy(&buffer[command_till..n])
                        )
                        .as_bytes(),
                    )
                    .await
                    .unwrap();
            }
        }
    }
}

async fn send_argument_error(stream: &mut TcpStream, command: &str) {
    stream
        .write_all(
            format!(
                "-ERR wrong number of arguments for '{}' command\r\n",
                command
            )
            .as_bytes(),
        )
        .await
        .unwrap();
}

fn parse_arguments(
    buffer: &[u8],
    length_till: usize,
    command_till: usize,
    n: usize,
) -> Option<Vec<String>> {
    let mut args = Vec::new();
    let mut pos = 0;
    while length_till + command_till + pos < n {
        let (arg, arg_till) =
            resp_parser::read_simple_string(&buffer[command_till + length_till + pos..n]);
        args.push(arg);
        pos += arg_till;
    }
    if length_till + command_till + pos == n {
        Some(args)
    } else {
        None
    }
}
