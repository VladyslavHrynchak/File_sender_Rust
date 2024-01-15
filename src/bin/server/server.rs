use std::{
        net::{TcpListener, TcpStream}, 
        io::{BufReader, BufRead, Read, Write}, 
        fs,
    };

pub struct Server{
    ip: String,
    port: u32,    
}

impl Server{
    pub fn new() -> Server{
        Server{ip: String::new(), port: 0}
    }

    pub fn start(&mut self, ip: String, port: u32){
        self.ip = ip;
        self.port = port;

        let connection_str = format!("{}:{}", self.ip, self.port);
       
        let listener = TcpListener::bind(&connection_str).expect(&format!("Server: bind to {} error", connection_str));

        println!("Server started at {connection_str}");

        for stream in listener.incoming(){
            let stream: TcpStream = stream.expect("Server: connection established error");
            self.handle_connection(stream);
        }
    }

    fn handle_connection(&self, stream : TcpStream){
        let mut reader = BufReader::new(&stream);

        let mut size_of_file = String::new();
        reader.read_line(&mut size_of_file).unwrap();
        let size: usize = size_of_file.trim().parse().unwrap();
        println!("Server: size of the file: {}", size);
    
        println!("Server: Waiting...");

        let mut file_name = String::new();
        reader.read_line(&mut file_name).unwrap();
        let file_name = file_name.trim_end().to_string();
    

        let mut file_buff: Vec<u8> = Vec::new();

        while file_buff.len() != size{
            reader.read_to_end(&mut file_buff).unwrap();
        }

        let full_path = format!("D://{}", file_name);

        println!("Server: path to the file: {:?}", full_path);

        let mut file = fs::File::create(format!("D://{}", file_name)).unwrap();
        file.write_all(&file_buff).unwrap();

        println!("Server: Done!!!!");

    }
    
}