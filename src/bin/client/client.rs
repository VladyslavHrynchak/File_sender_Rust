use std::{
    net::TcpStream, 
    io::Write, fs, 
    path::Path, 
    io::Read, 
    error::Error
};
pub struct Client{
    ip: String,
    port: u32,
    stream: Option<TcpStream>,
}

impl Client{
    pub fn new() -> Client{
        Client{ip: String::new(), port: 0, stream: None}
    }

    pub fn start(&mut self, ip: String, port: u32){
        self.ip = ip;
        self.port = port;
        
        let connection_str = format!("{}:{}", self.ip, self.port);

        self.stream = Some(TcpStream::connect(&connection_str).expect(&format!("Client: bind to {} error", connection_str)));

        println!("Client: connection established");
    }

    pub fn send_file(&self, path: &str ) -> Result<(), Box<dyn Error>>{
        let delimiter = '\n';

        let metadata = fs::metadata(path)?;
       
        let file_size = metadata.len().to_string();

        self.stream.as_ref().unwrap().write_all(&file_size.as_bytes())?;
        self.stream.as_ref().unwrap().write_all(&[delimiter as u8])?;

        println!("Client: size of the file: {file_size}");

        let name_of_file = Path::new(path);
        let name_of_file = name_of_file.file_name().unwrap().to_string_lossy().to_string();

        self.stream.as_ref().unwrap().write_all(&name_of_file.as_bytes())?;
        self.stream.as_ref().unwrap().write_all(&[delimiter as u8])?;

        let mut buf_file = Vec::with_capacity(file_size.trim().parse()?);

        let mut file = fs::File::open(path)?;
        file.read_to_end(& mut buf_file)?;

        let chunk_size = 256;

        for chunk in buf_file.chunks(chunk_size){
                self.stream.as_ref().unwrap().write_all(chunk)?;
        }

        Ok(())
    } 

}

