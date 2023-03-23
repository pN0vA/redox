use std::process::{Command,exit};
use std::io::{self, Write, BufReader, BufRead};
use std::net::Ipv4Addr;
use std::mem::drop;
use std::net::{Shutdown, SocketAddrV4, TcpListener, TcpStream};

fn handle_connection(clientsocket: &mut TcpStream, clients: &Arc<Mutex<HashMap<String, TcpStream>>>, port2: &String) {
    println!("New client connected: {}", clientsocket.local_addr().unwrap());
    let clientaddr = clientsocket.peer_addr().unwrap();
    let client_list = clients.lock().unwrap().keys().cloned().collect::<Vec<String>>();
    let num_clients = client_list.len();
    println!(" 

                    █▄─▄▄▀█▄─▄▄─█▄─▄▄▀█─▄▄─█▄─▀─▄█
                    ██─▄─▄██─▄█▀██─██─█─██─██▀─▀██
                    ▀▄▄▀▄▄▀▄▄▄▄▄▀▄▄▄▄▀▀▄▄▄▄▀▄▄█▄▄▀  
        _______________________________________________________________       
        Active:   Server: {} <- Client: {}      
        _______________________________________________________________
        ({:?} Agents in Session List):

        {:?}
        _______________________________________________________________

        Type 'rtfm' for help \n", clientsocket.local_addr().unwrap(),clientaddr,num_clients,client_list,); //,clients.keys(), clientsocket.local_addr().unwrap(),clientaddr); // {:?} inside Map

    loop {
        println!("Enter Command to send: ");
        let mut msg = String::new();
        io::stdin().read_line(&mut msg).expect("String expected");
        if msg.trim().contains("dl"){
            msg.push('\0');
            let mut buffer: Vec<u8> = Vec::new();
            clientsocket.write(msg.as_bytes());
            println!("Sent dl command? {}", &msg);
            println!("Enter url of file to dl: {}", &clientaddr);
            let mut msg = String::new();
            io::stdin().read_line(&mut msg).expect("Url String expected");
            msg.push('\0');
            clientsocket.write(msg.as_bytes());
            println!("Sent url? {}", &msg);
            let mut buffer: Vec<u8> = Vec::new();
            println!("Enter url of filename to write: {}", &clientaddr);
            let mut msg = String::new();
            io::stdin().read_line(&mut msg).expect("String expected");
            msg.push('\0');
            let mut buffer: Vec<u8> = Vec::new();
            clientsocket.write(msg.as_bytes());
            let mut reader = BufReader::new(clientsocket.try_clone().unwrap());
            println!("client {} sent \n{}", clientaddr, String::from_utf8_lossy(&buffer));
        } else if (msg.trim().contains("tx")){ //send files to client
            msg.push('\0');
            let mut buffer: Vec<u8> = Vec::new();
            clientsocket.write(msg.as_bytes());
            println!("Enter name of file to send: {}", &clientaddr);
            let mut msg = String::new();
            io::stdin().read_line(&mut msg).expect("String expected");
            msg.push('\0');
            let mut buffer: Vec<u8> = Vec::new();
            clientsocket.write(msg.as_bytes());
            println!("Sent {}", &msg);
            let clientaddrstr = clientaddr.to_string();
            let clientaddrstr2 = clientaddrstr.split(":").nth(0).unwrap();
            let port2 = "9001".to_string();
            let clientaddrtx = format!("{}:{}",clientaddrstr2,port2);println!("test ip client2: {}", clientaddrtx);
            let mut reader = BufReader::new(clientsocket.try_clone().unwrap());
            send_to_client(&mut TcpStream::connect(clientaddrtx).unwrap(), &mut msg, &port2);
        } else if (msg.trim().contains("rx")){ //receive files from client
            msg.push('\0');
            let mut buffer: Vec<u8> = Vec::new();
            clientsocket.write(msg.as_bytes());
            println!("Sent rx command {}", &msg);
            println!("Enter file name to download: {}", &clientaddr);
            let mut msgfn = String::new();
            io::stdin().read_line(&mut msgfn).expect("String expected");
            msgfn.push('\0');
            let mut buffer: Vec<u8> = Vec::new();
            clientsocket.write(msgfn.as_bytes());
            println!("Sent flnm{}", &msgfn);
            let mut reader = BufReader::new(clientsocket.try_clone().unwrap());
            let p2 = port2.parse::<u16>().unwrap();
            clientrx(&mut msgfn, p2);
        } else if (msg.trim().contains("agents")){
            if let Some(last_client) = client_list.last() {
                println!("The current active Agent is: {}", last_client);
            } else {
                println!("No clients connected");
            }
            println!("We have {} Active agents", num_clients);
            println!("With ID's: {:?}\n", client_list);
            continue;
        } else {
            msg.push('\0');
            let mut buffer: Vec<u8> = Vec::new();
            clientsocket.write(msg.as_bytes());
            println!("Sent {}", &msg);
            let mut reader = BufReader::new(clientsocket.try_clone().unwrap());
        }
        if msg.trim().contains("rtfm"){ 
            println!("THE MANUAL_________________________________________________________________\n");
            if cfg!(windows) {
                println!("Usage: [COMMAND]           Gives result\n");
                println!(" dl,                       Asks for source url and filename to write\n");
                println!(" tx,                       Asks for filename to send from server in current directory\n");
                println!(" rx,                       Asks for filename to receive from client in current directory\n");
                println!(" quit,                     Quits current client connection\n");
                println!(" agents,                   Shows connected devices. Type 'quit' to switch to the next connected client\n");
            } else if cfg!(unix) { 
                println!("Usage: [COMMAND]           Gives result\n");
                println!(" dl,                       Asks for source url and filename to write\n");
                println!(" tx,                       Asks for filename to send from server in current directory\n");
                println!(" rx,                       Asks for filename to receive from client in current directory\n");
                println!(" quit,                     Displays quits current client connection\n");
                println!(" agents,                   Shows connected devices. Type 'quit' to switch to the next connected client\n");
            } else if cfg!(target_os = "macos") {
                println!("Usage: [COMMAND]           Gives result\n");
                println!(" dl,                       Asks for source url and filename to write\n");
                println!(" tx,                       Asks for filename to send from server in current directory\n");
                println!(" rx,                       Asks for filename to receive from client in current directory\n");
                println!(" quit,                     Displays quits current client connection\n");
                println!(" agents,                   Shows connected devices. Type 'quit' to switch to the next connected client\n");
            }
        }
        msg.push('\0');
        let mut buffer: Vec<u8> = Vec::new();
        if msg.trim().contains("quit"){
            println!("shutting down client stream: {}", &clientaddr);
            clientsocket.shutdown(Shutdown::Both);
            println!("end of connections, crtl + c to terminate server program: {}", clientsocket.local_addr().unwrap());
            break;
        } 
        
        let mut reader = BufReader::new(clientsocket.try_clone().unwrap());
        reader.read_until(b'\0', &mut buffer);
        println!("client {} sent \n{}", clientaddr, String::from_utf8_lossy(&buffer));
    }
}

// downloads a file from current working directory located on client machine.
// this cannot take in the filename from but almost works otherwise
use std::io::{Read};
use std::path::Path;
fn handle_client_rx(mut stream: TcpStream){
    let mut filename = [0; 128];
    let bytes_read = stream.read(&mut filename).unwrap();
    let original_filename = std::str::from_utf8(&filename[..bytes_read]).unwrap().trim();
    println!("Received file {}", original_filename);
    let mut newfilname = String::from(original_filename);println!("File {}", newfilname);
    let mut file = std::fs::File::create(original_filename).unwrap();
    let mut buffer = [0; 1024];
    loop {
        let bytes_read = stream.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
        let data = &buffer[..bytes_read];
        file.write_all(data).unwrap();
    }
    println!("Saved file {}", original_filename);
}

 fn clientrx(filenm: &mut String, port2: u16) {
    let clientaddrtx = format!("0.0.0.0:{}",port2);
    let mut listener = TcpListener::bind(clientaddrtx).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| {
                    handle_client_rx(stream);
                });  return drop(listener);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    drop(listener);
}

fn send_to_client(socket:&mut TcpStream, filename: & String, port2: &String) -> std::io::Result<()> {
    let fntext : String = String::from(filename.to_string()).trim_end_matches('\0').replace('\n', "").replace('\r', "");
    let file_path = Path::new(&fntext);
    println!("Opening file {:?}", file_path);
    let mut file = File::open(&file_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents);
    let mut socket = socket;
    // Send file name
    let filename = file_path.file_name().unwrap().to_str().unwrap();
    let filename_bytes = filename.as_bytes();
    let filename_len = filename_bytes.len();
    println!("Sending to socket");
    socket.write_all(&filename_bytes)?;
    // Send file contents
    let contents_len = contents.len();
    socket.write_all(&contents)?;
    println!(
        "Sent file {} containing {} bytes",
        contents_len,
        String::from_utf8_lossy(&filename_bytes)
    );
    Ok(())
}

use std::cmp::min;
use std::fs::{File, OpenOptions};
use std::io::{Seek};
use reqwest::Client;
use indicatif::{ProgressBar, ProgressStyle};
use futures_util::StreamExt;

pub async fn download_file(client: &Client, url: &str, path: &str) -> Result<(), String> {
    let res = client
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.white/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("█  "));
    pb.set_message(&format!("Downloading {}", url));

    let mut file;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();
    
    println!("Seeking in file.");
    if std::path::Path::new(path).exists() {
        println!("File exists. Resuming.");
        file = std::fs::OpenOptions::new()
            .read(true)
            .append(true)
            .open(path)
            .unwrap();

        let file_size = std::fs::metadata(path).unwrap().len();
        file.seek(std::io::SeekFrom::Start(file_size)).unwrap();
        downloaded = file_size;

    } else {
        println!("Fresh file..");
        file = File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
    }

    println!("Commencing transfer");
    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        file.write(&chunk)
            .or(Err(format!("Error while writing to file")))?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(&format!("Downloaded {} to {}", url, path));
    return Ok(());
}

use std::thread;
use tokio::runtime::Runtime;
use tokio::time::{sleep, Duration};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() { 
    // set ip address and port here
    let mut ipaddy = "0.0.0.0".to_string();
    let port = 5358;
    let port2 = "9001".to_string(); // for tx and rx of files
    
    let serveraddress = format!("{}:{}",ipaddy,port);
    let serveraddresstx = format!("{}:{}",ipaddy,port2);
    let ip = ipaddy.parse::<Ipv4Addr>().unwrap();
    let portu16:u16 = port;
    let mut s = SocketAddrV4::new(ip, port);

    println!("IP Address: {}", s.ip());
    println!("Port: {}", s.port());

    let listener = TcpListener::bind(s);
    let listener: () = match listener {
        Ok(l) => {
            println!("Successfully binded to {}", l.local_addr().unwrap());
        }
        Err(e) => {println!("{}",e); exit(0);}
    };
    let listener = TcpListener::bind(serveraddress.to_string()).unwrap();
    let clients: Arc<Mutex<HashMap<String, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));

    // v2 using multi threading
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let clients_clone = clients.clone();
                let port2clone = port2.clone();
                let client_id = stream.peer_addr().unwrap().to_string();
                 // Add the new client to the hashmap
                 clients_clone.lock().unwrap().insert(client_id.clone(), stream.try_clone().unwrap());
                 // Print the list of connected clients
                 let client_list = clients_clone.lock().unwrap()
                 .keys()
                 .map(|s| s.as_ref())
                 .collect::<Vec<&str>>()
                 .iter()
                 .map(|s| s.to_string())
                 .collect::<Vec<String>>();
                // Spawn a new thread to handle incoming connections
                thread::spawn(move|| {
                    // connection succeeded
                    handle_connection(&mut stream, &clients_clone, &port2clone);
                    // Remove the client from the hashmap when the connection is closed
                    clients_clone.lock().unwrap().remove(&client_id);
                    // Print the updated list of connected clients
                    let client_list = clients_clone.lock().unwrap().keys().cloned().collect::<Vec<String>>();
                    println!("Connected clients: {:?}", client_list);
                });
            }
            Err(e) => { /* connection failed */ }
        }
    } 
    println!("Stopping server listener");
    drop(listener);
}
