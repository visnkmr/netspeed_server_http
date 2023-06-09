#![windows_subsystem = "windows"]
#![allow(warnings)]
use std::{process::{exit},
    time::{Duration},
        thread::{self, sleep},env::{self, var}, sync::mpsc::channel};
// use abserde::Location;
// use byte_unit::Byte;
use chrono::Local;
// mod abserdeapi;
// use fltk::{app::{App, self}, window::{Window, OverlayWindow, self, SingleWindow, DoubleWindow}, prelude::*, enums::{Color, self}, text::{TextDisplay, TextBuffer}, frame, menu};
use sysinfo::{SystemExt, NetworkExt, System};
// use abserde::*;
// use abserdeapi::*;
use prefstore::*;
use tiny_http::{Server, Request, Method, Header, Response, StatusCode};
const APPNAME:&str="ns_daemon";
fn main() ->Result<(), ()> {
    
    //to store interface name
    let mut iname=String::new();
    // let date = Local::now();
    // let current_date = date.format("%Y-%m-%d").to_string();
    // setappname(var("CARGO_PKG_NAME").unwrap_or_else(|_| env!("CARGO_PKG_NAME").to_string()));
    // println!("{}",byte_unit::Byte::from_bytes(getpreference(APPNAME,&current_date,0 as u128).parse::<u128>().unwrap()).get_appropriate_unit(true));
    //set interface name from commandline to set it to track only specific network interface
    let args: Vec<String> = env::args().collect();
    match args.get(1){
        Some(g)=>{
            iname=g.to_owned();
             if(iname=="tu"){
                let date = Local::now();
                let current_date = date.format("%Y-%m-%d").to_string();
                println!("{}",byte_unit::Byte::from_bytes(
                    getpreference(APPNAME,&current_date,0 as u128).parse::<u128>().unwrap()
                ).get_appropriate_unit(true));
                exit(0);
            }
        },
        None=>{
            iname="all".to_string();
        }
    }
        
    let mut dtpr:Vec<u64>=vec![0,0,0]; 
    let date = Local::now();
    let mut current_date = date.format("%Y-%m-%d").to_string();
    let mut last_saved_date:String =String::new();
    // println!("fromhere------------>4");
    // dtpr[0] = getpreference(APPNAME,&current_date,0 as u64).parse::<u64>().unwrap();//stores total upload and download bytes count of current session and total data usage since the start of the ns_daemon in a day
    dtpr[0] = 0 as u64;//stores total upload and download bytes count of current session and total data usage since the start of the ns_daemon in a day
    let ina=iname.clone();
    thread::spawn(move || loop {
        // println!("fromhere------------>1");
        updateusage(true/*,&mut val,&mut ptx,&mut prx*/,ina.to_owned(),&mut dtpr,&mut last_saved_date);
        thread::sleep(Duration::from_secs(60));
    });
    let mut sys = System::new();
    let mut tt:u128=getpreference(APPNAME,&current_date,0 as u128).parse::<u128>().unwrap();
    let mut perminute=0;
    let address="127.0.0.1:6798".to_string();
    
    let server=Server::http(&address).map_err(|err|{
        eprintln!("{err}")
    })?;
    println!("listening @ {}",address);
    for request in server.incoming_requests(){
        handle_con(request,iname.clone(),&mut sys,&mut tt,&mut perminute)?;
    }
    Ok(())
    //customize port and address here.
    // match TcpListener::bind("127.0.0.1:6798") {
    //     Ok(listener) =>{
    //         for stream in listener.incoming(){
    //             let stream = stream.unwrap();
    //                 handle_con(stream,iname.clone(),&mut sys,&mut tt,&mut perminute);
    //         }
    //     },
    //     Err(e) =>{
    //         println!("Internet issue.\n Error:{}",e)
    //     }
    // }
    
}
// fn watcher(paths: Vec<String>, request: Request) {
//     thread::spawn(move || {
//         match event_stream(request) {
//             _ => {}
//         }
//     });
// }
fn event_stream(request: Request) -> Result<(), ()> {
    let mut res = Response::new(StatusCode::from(200), Vec::new(), std::io::Empty::default(), None, None);
    let header = tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/event-stream"[..]).unwrap();
    res.add_header(header);

    let http_version = request.http_version().clone();
    let mut writer = request.into_writer();
    

    let mut buffer: Vec<u8> = Vec::new();
    res.raw_print(&mut buffer, http_version, &[], true, None);

    writer.write_all(&buffer);
    writer.flush();

    // let (tx, rx) = channel();
    // let mut watcher = raw_watcher(tx)?;

    // for path in paths {
    //     watcher.watch(path, RecursiveMode::NonRecursive)?;
    // }
let mut i=0;
thread::spawn(move || {
            loop {
        // match rx.recv() {
            // Ok(e) => {
                // match e.op {
                    // Ok(Op::WRITE) => {
                        let message = "event: reload\ndata:\n\n".to_owned()+i.to_string().as_str();
                        i+=1;
                        write!(writer, "{:x}\r\n", message.len());
                        write!(writer, "{}\r\n", message);
                        writer.flush();
                        sleep(Duration::from_secs(1));
    //                 }
    //                 _ => {}
    //             }
    //         },
    //         Err(e) => return Err(e.into()),
    //     }
    }
        });
    
    Ok(())
}
// fn handle_con(mut stream:TcpStream,iname:String,sys:&mut System,tt:&mut u128,perminute:&mut i32){
//     let buf_reader = std::io::BufReader::new(&mut stream);
//     let request_line = match buf_reader.lines().next() {
//         None => "".to_string(),
//         Some(secondline) => {
//             match secondline {
//                 Ok(reqline)  => reqline,
//                 Err(e) => "".to_string(),
//             }
//         },
//     };
//     let (status_line, filecontent,contentheader) =
//         if request_line == "GET / HTTP/1.1".to_string() {
//              ("HTTP/1.1 200 OK", marks(&iname,sys,tt,perminute),String::from("Content-Type: application/json"))
//         }
//         else{
//             ("HTTP/1.1 200 OK", sincelastread(),String::from("Content-Type: application/json"))
//         };
//         let response =
//         format!("{status_line}\r\n{contentheader}\r\n\r\n{filecontent}");
//     match stream.write(response.as_bytes()) {
//         Ok(file) => {
//         },
//         Err(error) =>{
//             return ;
//         },
//     };match stream.flush() {
//         Ok(file) => {
//         },
//         Err(error) =>{
//             return ;
//         },
//     };
//     }
    fn handle_con(mut request:Request,iname:String,sys:&mut System,tt:&mut u128,perminute:&mut i32)->Result<(),()>{
        eprintln!("request method:{} url:{}",request.method(),request.url());
        eprintln!("{:?}",request);
        match(request.method(),request.url()){
            (Method::Get,"/")=>{
                let content_header=Header::from_bytes("Content-Type","application/json; charset=utf-8").unwrap();
                request.respond(Response::from_string(marks(&iname,sys,tt,perminute)).with_header(content_header)).map_err(|err|{
                    eprintln!("could not serve request error {}",err);
                })?;
            },
            (Method::Get,"/stream")=>{
                event_stream(request);
            },
            _=>{
                let content_header=Header::from_bytes("Content-Type","application/json; charset=utf-8").unwrap();
            request.respond(Response::from_string(sincelastread()).with_header(content_header)).map_err(|err|{
                eprintln!("could not serve request error {}",err);
            })?;
            }
        }
    Ok(())
    }
    //returns total upload and download bytes count of current session and total data usage since the start of the ns_daemon in a day
    pub fn marks(iname:&String,sys:&mut System,tt:&mut u128,perminute:&mut i32)->String{
                    sys.refresh_networks_list();
                    
                    let mut total_rx: u64 = 0;
                    let mut total_tx: u64 = 0;
                    let networks = sys.networks();
                    for (name, network) in networks {
                            let mut nametostat=iname.as_str();
                            if(nametostat=="all"){
                            total_rx += network.total_received();
                            total_tx += network.total_transmitted();
                            }
                            else if(*name == *iname){
                                total_rx += network.total_received();
                                total_tx += network.total_transmitted();
                                break;
                            }
                    }
                    let date = Local::now();
                            let current_date = date.format("%Y-%m-%d").to_string();
                            // println!("fromhere------------>3");
                            if(*perminute>60){
                                *tt=getpreference(APPNAME,&current_date,0 as u128).parse::<u128>().unwrap();
                                *perminute=0;
                            }
                            *perminute+=1;
                return serde_json::to_string_pretty(&vec![total_tx,total_rx,*tt as u64]).unwrap();
        }
//returns todays total while ns_daemon running
pub fn sincelastread()->String{
    let date = Local::now();
    let current_date = date.format("%Y-%m-%d").to_string();
    let tt=getpreference(APPNAME,&current_date,0 as u128).parse::<u128>().unwrap();
    return serde_json::to_string_pretty(&vec![tt as u64]).unwrap();
}
// saves bytes used every minute to file while ns_daemon running
fn updateusage(whethertosave:bool/*,val:&mut u128,ptx:&mut u64,prx:&mut u64*/,iname:String,dtpr:&mut Vec<u64>,lastsaveddate:&mut String){//->String{
            
            let date = Local::now();
            let current_date = date.format("%Y-%m-%d").to_string();
            if *lastsaveddate!=current_date{
                dtpr[0]=getpreference(APPNAME,&current_date,0 as u64).parse::<u64>().unwrap()
            }
            let mut sys = System::new();
            sys.refresh_networks_list();
            let mut total_rx: u64 = 0;
            let mut couldfind=false;
            let mut total_tx: u64 = 0;
            let networks = sys.networks();
            let mut k=0;
            for (name, network) in networks {
                    if(iname=="all"){
                    total_rx += network.total_received();
                    total_tx += network.total_transmitted();
                    couldfind=true;
                    }
                    else if(*name == *iname){
                        total_rx += network.total_received();
                        total_tx += network.total_transmitted();
                        couldfind=true;
                        break;
                    }
            }
            let mut turx=total_rx.saturating_sub(dtpr[2]);
            let mut tutx=total_tx.saturating_sub(dtpr[1]);
            if dtpr[1]!=0 ||dtpr[2]!=0 {
                dtpr[0]+=turx+tutx;
                // let mut dm:HashMap<String,u128>=HashMap::new();
            
            {
                if whethertosave{
                savepreference(APPNAME,&current_date, dtpr[0] as u128)
            }
            }
            
            }
            dtpr[1]=total_tx;
            dtpr[2]=total_rx;
            *lastsaveddate = current_date;
            // let tt=total_rx+total_tx;
            // let byte_rx = byte_unit::Byte::from_bytes(turx as u128);
            // let byte_tx = byte_unit::Byte::from_bytes(tutx as u128);
            // let byte_t = byte_unit::Byte::from_bytes(tt as u128);
            // let adjusted_rx = byte_rx.get_appropriate_unit(true);
            // let adjusted_tx = byte_tx.get_appropriate_unit(true);
            // let adjusted_st = byte_unit::Byte::from_bytes(dtpr[0] as u128).get_appropriate_unit(true);
            // if couldfind{
            //     format!("{}↓ {}↑ {}",adjusted_rx,adjusted_tx,adjusted_st )
            // }
            // else{
            //     format!("No such network")
            // }
}
// returns total data used since session started / present logged in session usage
// fn updateupdowndatausg(ptx:&mut u64,prx:&mut u64)->u128{
//     let mut sys = System::new();
//             sys.refresh_networks_list();
//             let mut total_rx: u64 = 0;
//             let mut total_tx: u64 = 0;
//             let networks = sys.networks();
//             let mut k=0;
//             for (name, network) in networks {
//                     {
//                     total_rx += network.total_received();
//                     total_tx += network.total_transmitted();
//                     k=1;
//                     }
//             }
//             let mut turx=total_rx.saturating_sub(*prx);
//             let mut tutx=total_tx.saturating_sub(*ptx);
//             *ptx=total_tx;
//             *prx=total_rx;
//             let tt=total_rx+total_tx;
//             tt as u128
// }
