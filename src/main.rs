use std::{process::{Command, exit},
    time::{Instant, self, Duration,
        SystemTime, UNIX_EPOCH},
        thread, collections::HashMap, hash::Hash, env, net::{TcpListener, TcpStream}, io::{BufRead, Write}};

use abserde::Location;
use byte_unit::Byte;
use chrono::Local;
mod abserdeapi;
// use byte_unit::ByteUnit;
use fltk::{app::{App, self}, window::{Window, OverlayWindow, self, SingleWindow, DoubleWindow}, prelude::*, enums::{Color, self}, text::{TextDisplay, TextBuffer}, frame, menu};
// use network_interface::{NetworkInterface, NetworkInterfaceConfig};
use sysinfo::{SystemExt, NetworkExt, System};

use abserde::*;
// use serde::{Serialize, Deserialize};

// #[derive(Serialize, Deserialize)]
// struct MyConfig {
// 	// window_width: i32,
// 	// window_height: i32,
// 	// window_x: i32,
// 	// window_y: i32,
// 	user_data: HashMap<String, u128>,
// }
use abserdeapi::*;
fn main() {
    // let mut reload=false;
    // println!("Hello, world!");
    // let my_abserde = Abserde {
    //     app: "tnsoverlay".to_string(),
    //     location: Location::Auto,
    //     format: Format::Toml,
    // };

    // match MyConfig::load_config(&my_abserde){
    //     Ok(map) => {
    //       // my_config = MyConfig::load_config(&my_abserde).expect("");
    //       // println!("{:#?}", dirs::config_dir());
    //       // println!("{:#?}",my_abserde.config_path());


    //       // Ok(my_abserde.delete()?)
    //     }
    //     Err(e) => {
    //         reload=true;
    //       // warn!("Error while loading preferences: {:?}", e);
    //       println!("is empty.Error.");

    //     }
    //   }
    //   if(reload){
    //     println!("reload");

    //     let mut dm:HashMap<String,u128>=HashMap::new();
    //         let mut ptx:u64=0;
    //         let mut prx:u64=0;
    //         let date = Local::now();
    //         let current_date = date.format("%Y-%m-%d").to_string();

    //         dm.insert(current_date, (updateupdowndatausg(&mut ptx, &mut prx)));
    //         for (key, value) in &dm {
    //             println!("{}: {}", key, value);
    //         }
    // //         let mut my_config =  MyConfig {
    // //             // window_width: 320,
    // //             // window_height: 30,
    // //             // window_x: 0,
    // //             // window_y: 0,
    // //             user_data: dm,
    // //         };
    // //         my_config.save_config(&my_abserde);
    // //   }

    // //   match MyConfig::load_config(&my_abserde){
    // //     Ok(map) => {
    // //         for (key, value) in &map.user_data {
    // //             println!("{}: {}", key, value);
    // //         }
    // //       // my_config = MyConfig::load_config(&my_abserde).expect("");
    // //       // println!("{:#?}", dirs::config_dir());
    // //       // println!("{:#?}",my_abserde.config_path());


    // //       // Ok(my_abserde.delete()?)
    // //     }
    // //     Err(e) => {
    // //       // warn!("Error while loading preferences: {:?}", e);
    // //       println!("is empty.Error.2");

    // //     }
    //   }

    // my_config.user_data;
    let mut iname=String::new();
    let args: Vec<String> = env::args().collect();

    match args.get(1){
        Some(g)=>{
            iname=g.to_owned();
        },
        None=>{
            iname="all".to_string();
        }
    }
    match args.get(2){
        Some(g)=>{
            if(g=="tu"){
                let date = Local::now();
                let current_date = date.format("%Y-%m-%d").to_string();
                println!("{}",byte_unit::Byte::from_bytes(
//                    match setup(true, getasv().1).1.get(&current_date){
                    match getasv().1.get(&current_date){
                        Some(a)=>{
                            *a
                        },
                        None=>{
                            0 as u128
                        }
                    }
                ).get_appropriate_unit(true));
                exit(0);
            }
        },
        None=>{
        }
    }
    let mut showgui=false;
    match args.get(3){
        Some(g)=>{
            if(g=="gui"){
                showgui=true;
            }
        },
        None=>{
        }
    }

    // let iname="all".to_string();
    // thread::spawn(||{

    // let mut ptx:u64=0;
    // let mut prx:u64=0;
//    let mut val=0;
    // while
    // updateusage(true,&mut val,&mut ptx,&mut prx,ina)
    // });
    // thread::spawn(|| {
//        let mut ptx:u64=0;
//        let mut prx:u64=0;
        let mut dtpr:Vec<u64>=vec![0,0,0];
        let ina=iname.clone();
        thread::spawn(move || loop {
            // tx.send(updateusage(false,&mut val,&mut ptx,&mut prx,iname.clone()));
            println!("fromhere------------>1");
            updateusage(true/*,&mut val,&mut ptx,&mut prx*/,ina.to_owned(),&mut dtpr);
            thread::sleep(Duration::from_secs(60));
        });
        match TcpListener::bind("127.0.0.1:6798") {
            Ok(listener) =>{
                // match local_ip() {
                //     Ok(lip) =>{
                //         println!("{}",lip);
                //     },
                //     Err(e) =>{
                //         println!("Internet issue.\n Error:{}",e)
                //     }
                // }

                for stream in listener.incoming(){
                    // println!("{:?}",stream);
                    let stream = stream.unwrap();
                    // let ts=stream.peer_addr().unwrap().to_string();
                    // println!("{}",format!("conctd to {}",stream.peer_addr().unwrap()));
                    // tp.send(&ts).unwrap();
                    // tp.push_str(&format!("conctd to {}",stream.peer_addr().unwrap()));
                    // thread::spawn(|| {
                        handle_con(stream,iname.clone());
                    // });
                }
            },
            Err(e) =>{
                println!("Internet issue.\n Error:{}",e)
            }
        }
    // });
    if showgui{
        showapp(iname.clone());
    }

}
fn handle_con(mut stream:TcpStream,iname:String){
    // let (status_line, html_code)={
    // {

    //     ("HTTP/1.1 200 OK",
    //     [
    //     r##"
    //         <!DOCTYPE html>
    //         <html>
    //         <head>
    //         <style>
    //         </style>

    //         <title>Bookmarks</title>
    //         </head>
    //         <body>
    //         </body>
    //         <footer>
    //         <script>
    //         if ( window.history.replaceState ) {
    //         window.history.replaceState( null, null, window.location.href );
    //         }
    //         </script>
    //         </footer>
    //         </html>
    //     "##].concat())
    // }
    // };
    // println!("{:?}",stream);
    let buf_reader = std::io::BufReader::new(&mut stream);
    // let brd=buf_reader;
    // for oi in brd.lines(){

    // }
    // println!("{:?}",buf_reader);
    let request_line = match buf_reader.lines().next() {
        None => "".to_string(),
        Some(secondline) => {
            match secondline {
                Ok(reqline)  => reqline,
                Err(e) => "".to_string(),
            }
        },
    };
    // let request_line = buf_reader.lines().next().unwrap().unwrap();
        // println!("req------>{request_line}");
        // println!("---->{}",request_line);

        // let mut headers = [httparse::EMPTY_HEADER; 16];
        // let mut req = httparse::Request::new(&mut headers);
        // let res = req.parse(request_line.as_bytes()).unwrap();
        // if res.is_partial() {
        //     match req.path {
        //         Some(ref path) => {

        //         },
        //         _=>{

        //         }
        //     }
        // }
        // let mut ptx=*ptx;
        // let mut prx=*prx;
    let (status_line, filecontent,contentheader) =

        if request_line == "GET / HTTP/1.1".to_string() {
        //     println!("option 1");
             ("HTTP/1.1 200 OK", marks(&iname),String::from("Content-Type: application/json"))
        //     ("HTTP/1.1 200 OK", String::from(&html_code),format!("Content-Length: {}",html_code.len()))
        }
        else{
            ("HTTP/1.1 200 OK", sincelastread(),String::from("Content-Type: application/json"))
        };
        // else if request_line == "GET /json HTTP/1.1".to_string() {
        //     println!("option 2");
            // ("HTTP/1.1 200 OK", marks(),String::from("Content-Type: application/json"))
        // }
        // else if request_line.starts_with("GET /json/api") {
        //     ("HTTP/1.1 200 OK", marks(),String::from("Content-Type: application/json"))
        // }
        // // if request_line == "GET / HTTP/1.1".to_string()
        // else
        // {
        //     ("HTTP/1.1 200 OK", String::from(&html_code),format!("Content-Length: {}",html_code.len()))
        // };

        let response =
        format!("{status_line}\r\n{contentheader}\r\n\r\n{filecontent}");
    // let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}",
    // status_line,
    // html_code.len(),
    // html_code);
    match stream.write(response.as_bytes()) {
        Ok(file) => {

        },
        Err(error) =>{
            return ;
        },
    };match stream.flush() {
        Ok(file) => {

        },
        Err(error) =>{
            return ;
        },
    };
    // stream.write(response.as_bytes()).unwrap();
    }
    pub fn marks(iname:&String)->String{

                    let mut sys = System::new();
                    // First we update all information of our `System` struct.
                    sys.refresh_networks_list();
                    let mut total_rx: u64 = 0;
                    let mut total_tx: u64 = 0;
                    let networks = sys.networks();
                    // let mut k=0;
                    for (name, network) in networks {

                            let mut nametostat=iname.as_str();
                            if(nametostat=="all"){
                            total_rx += network.total_received();
                            total_tx += network.total_transmitted();
                            // couldfind=true;
                            // k=1;
                            }
                            else if(*name == *iname){
                                total_rx += network.total_received();
                                total_tx += network.total_transmitted();
                                // couldfind=true;
                                break;
                            }

                    }
                    let date = Local::now();
                            let current_date = date.format("%Y-%m-%d").to_string();
                            println!("fromhere------------>3");
//                            let tt=0;
                            let tt=match getasv().1.get(&current_date){
                                Some(a)=>{
                                    *a
                                },
                                None=>{
                                    0 as u128
                                }
                            };
//                    let tt=match setup(true, getasv().1).1.get(&current_date){
//                        Some(a)=>{
//                            *a
//                        },
//                        None=>{
//                            0 as u128
//                        }
//                    };
                // println!("cjson------0");
                return serde_json::to_string_pretty(&vec![total_tx,total_rx,tt as u64]).unwrap();

        }

pub fn sincelastread()->String{
    let date = Local::now();
    let current_date = date.format("%Y-%m-%d").to_string();
//    let tt=0;
    let tt=match getasv().1.get(&current_date){
                Some(a)=>{
                    *a
                },
                None=>{
                    0 as u128
                }
            };
//    let tt=match setup(true, getasv().1).1.get(&current_date){
//        Some(a)=>{
//            *a
//        },
//        None=>{
//            0 as u128
//        }
//    };
    return serde_json::to_string_pretty(&vec![tt as u64]).unwrap();
}


// fn dragwin(win:&mut DoubleWindow){
//     let mut xoff:i32 = 0;
//             let mut yoff:i32 = 0;
//             let mut xsp:i32 = 0;
//             let mut ysp:i32 = 0;
//             win.make_resizable(true);
//             win.handle(Box::new(move |f:&mut DoubleWindow, ev|{
//                 // println!("{}----->{}",ev,fltk::app::event_text());
//              match ev {
//                 // fltk::enums::Event::Resize=>{
//                 //     // let my_abserde = Abserde {
//                 //     //     app: "tnsoverlay".to_string(),
//                 //     //     location: Location::Auto,
//                 //     //     format: Format::Toml,
//                 //     // };
//                 //     // let my_lconfig;
//                 //     let mut reload =false;
//                 //     // match MyConfig::load_config(&my_abserde){
//                 //     //     Ok(map) => {
//                 //     //       // my_config = MyConfig::load_config(&my_abserde).expect("");
//                 //     //       // println!("{:#?}", dirs::config_dir());
//                 //     //       // println!("{:#?}",my_abserde.config_path());
//                 //     //       my_lconfig=map;

//                 //     //       // Ok(my_abserde.delete()?)
//                 //     //     }
//                 //     //     Err(e) => {
//                 //     //         reload=true;
//                 //     //       // warn!("Error while loading preferences: {:?}", e);
//                 //     //       println!("is empty.Error.");

//                 //     //     }
//                 //     //   }
//                 //       if(reload){
//                 //         let mut dm:HashMap<String,u128>=HashMap::new();
//                 //             let mut ptx:u64=0;
//                 //             let mut prx:u64=0;
//                 //             let date = Local::now();
//                 //             let current_date = date.format("%Y-%m-%d").to_string();

//                 //             dm.insert(current_date, (updateupdowndatausg(&mut ptx, &mut prx)));
//                 //             // let mut my_config =  MyConfig {
//                 //             //     // window_width: 320,
//                 //             //     // window_height: 30,
//                 //             //     // window_x: 0,
//                 //             //     // window_y: 0,
//                 //             //     user_data: dm,
//                 //             // };
//                 //             // my_config.save_config(&my_abserde);
//                 //       }
//                 //     //   else{
//                 //     //     let mut my_config = MyConfig {
//                 //     //     // window_height:f.h(),
//                 //     //     // window_width:f.w(),
//                 //     //     // window_x: f.x(),
//                 //     //     // window_y:f.y(),
//                 //     //     user_data:my_lconfig.user_data
//                 //     //   };
//                 //     //   my_config.save_config(&my_abserde).unwrap();
//                 //     //   }


//                 //     true

//                 // },
//                 // DOWNCLICK IN WINDOW CREATES CURSOR OFFSETS
//                 fltk::enums::Event::Push => {
//                     xoff = f.x() - fltk::app::event_x_root();
//                     yoff = f.y() - fltk::app::event_y_root();
//                     // println!("1-------->{}---{}",f.x(),f.y());
//                     // println!("2-------->{}---{}",f.x() - fltk::app::event_x_root(),f.y() - fltk::app::event_y_root());
//                     true
//                 },
//                 fltk::enums::Event::Drag => {
//                     xsp=xoff + fltk::app::event_x_root();
//                     ysp=yoff + fltk::app::event_y_root();
//                     f.set_pos(xsp,ysp);
//                     // f.set_override();
//                     // f.resize(xsp,ysp,320,30);
//                     f.flush();
//                     // println!("{}----{}",xsp as f64/fltk::app::screen_size().0 as f64,ysp as f64/fltk::app::screen_size().1 as f64);

//                     // println!("3-------->{}----{}",fltk::app::event_x_root(),fltk::app::event_y_root());
//                     // println!("4-------->{}----{}",fltk::app::event_x_root(),fltk::app::event_y_root());
//                         // position(xoff + fltk::app::event_x_root(), yoff + fltk::app::event_y_root());
//                         // redraw();
//                         // ret = 1;
//                         true
//                 },
//                 fltk::enums::Event::Released => {
//                     // f.show();
//                         // show();
//                         true
//                 },
//                 // Event::Paste => {

//                 //     true
//                 // }
//                 fltk::enums::Event::Move => {
//                         // println!("A resize happening: x:{}, y:{}, w:{}, h:{}", f.x(), f.y(), f.width(), f.height());
//                         true
//                     }
//                 // fltk::enums::Event::Resize => {
//                 //     println!("A resize happening: x:{}, y:{}, w:{}, h:{}", f.x(), f.y(), f.width(), f.height());
//                 //     true
//                 // }
//                 fltk::enums::Event::KeyDown => {

//                      if fltk::app::event_key() == fltk::enums::Key::from_char('f') {
//                         // win.fullscreen(!win.fullscreen_active());
//                     } else if fltk::app::event_key() == fltk::enums::Key::from_char('q') {
//                         fltk::app::quit();
//                     };

//                     true
//                 }
//                 ,
//                  _ => {
//                      false
//                  }
//              }
// }));
// }
fn showapp(iname:String){
//    let mut val=0;

    // let checknload =true;
    let mut dm:HashMap<String,u128>=HashMap::new();
            let date = Local::now();
            let current_date = date.format("%Y-%m-%d").to_string();

            dm.insert(current_date, 0);
//    let (showgui,hm)=setup( true,getasv().1);
    let (showgui,hm)=getasv();
    for (key, value) in &hm {
        println!("{}: {}", key, value);
    }
    // let (tx, rx) = std::sync::mpsc::channel();
    let (tx, rx) = fltk::app::channel();
    let mut app = App::default();
//    let mut ptx:u64=0;
//    let mut prx:u64=0;
    let mut dtpr:Vec<u64>=vec![0,0,0];

    app::background(0, 0, 0);
    // app::background2(1, 1, 1);
    // tx.send("").unwrap();
	// test();
    // OverlayWindow::
            let mut win = DoubleWindow::default()
            .with_size(320, 30)
            .with_label(if cfg!(debug_assertions) {
                "tns_beta"
            } else {
                "tns"
            });

            // let mut popup = window::MenuWindow::default().with_size(150, 60);
            // let mut content = frame::Frame::default()
            //     .size_of_parent()
            //     .center_of_parent()
            //     .with_label("This is a popup");
            // content.set_frame(enums::FrameType::BorderBox);
            // popup.end();

            // // this function sets the NOBORDER and OVERRIDE flags
            // // NOTE: this is not currently exposed in the rust binding
            // popup.set_override();

            // popup.handle(|p, evt| match evt {
            //     enums::Event::Push => {
            //         p.hide();
            //         // stop the popup window from intercepting all events
            //         app::set_grab::<window::MenuWindow>(None);
            //         true
            //     }
            //     _ => false,
            // });
            // let menu = menu::MenuItem::new(&["1st menu item\t", "2nd menu item\t", "3rd menu item\t"]);
            // win.handle(move |f,_| {
            //     if app::event_mouse_button() == app::MouseButton::Right {
            //         // or app::event_button() == 3
            //         let coords = app::event_coords();
            //         match menu.popup(coords.0, coords.1) {
            //             None => println!("No value was chosen!"),
            //             Some(val) => {
            //                 if f.border(){
            //                     f.set_border(false);
            //                     // f.flush();
            //                 }
            //                 else{
            //                     f.set_border(true);
            //                 }
            //                 // println!("{}", val.label().unwrap())
            //             },
            //         }
            //     }
            //     true
            // });

            // win.handle(move |f, ev|{
            //     match ev {
            //         fltk::enums::Event::KeyDown => {
            //             if fltk::app::event_key() == fltk::enums::Key::from_char('q') {
            //                 fltk::app::quit();
            //             };
            //             true
            //         },fltk::enums::Event::Push => {
            //             // if fltk::app::event_mouse_button() == fltk::app::MouseButton::Right {
            //             //     println!("{}",f.border());
            //             //     if f.border(){
            //             //         f.set_border(false);
            //             //         // f.flush();
            //             //     }
            //             //     else{
            //             //         f.set_border(true);
            //             //     }
            //             //     // win.fullscreen(!win.fullscreen_active());
            //             // }
            //             if app::event_mouse_button() == app::MouseButton::Left {
            //                 // let (ex, ey) = (app::event_x_root(), app::event_y_root());
            //                 // popup.set_pos(ex, ey);
            //                 // popup.show();
            //                 // // allow the popup window to intercept all events
            //                 // app::set_grab(Some(popup.clone()));
            //             } else if app::event_mouse_button() == app::MouseButton::Right {
            //                 // let (ex, ey) = app::event_coords();
            //                 let menu = menu::MenuItem::new(&["Hide/Show Window"]);
            //                 // menu.popup(ex, ey);
            //                 let coords = app::event_coords();
            //                 match menu.popup(coords.0, coords.1) {
            //                     None => println!("No value was chosen!"),
            //                     Some(val) => {
            //                             if f.border(){
            //                                 f.set_border(false);
            //                                 f.redraw();
            //                             }
            //                             else{
            //                                 f.set_border(true);
            //                             }
            //                         // println!("{}", val.label().unwrap())
            //                         }
            //                     }
            //                 }
            //                 true
            //             },
            //             _ => {
            //                 false
            //             }
            //         }
            // });
                //                 // println!("{}----->{}",ev,fltk::app::event_text());
                //              match ev {
                //                 // println!("{}----->{}",ev,fltk::app::event_text());
                //              match ev {;
            // win.set_border(false);
            if cfg!(debug_assertions) {
                // println!("Debugging enabled");
                // print!("{:?}",win.xclass());
            } else {
                // println!("Debugging disabled");
            }



            // let mut activation_frame = frame::Frame::default()
            //     .with_size(150, 50)
            //     .center_of_parent()
            //     .with_label("Left click for popup\nRight click for menu");
            // activation_frame.set_tooltip("this is a tooltip");
            // activation_frame.set_frame(enums::FrameType::BorderBox);
            // win.end();

            // let mut popup = window::MenuWindow::default().with_size(150, 60);
            // let content = frame::Frame::default()
            //     .size_of_parent()
            //     .center_of_parent()
            //     .with_label("This is a popup");
            // popup.end();
            // popup.set_border(false);
            // popup.set_frame(enums::FrameType::ShadowBox);

            // popup.handle(|p, evt| match evt {
            //     // close the popup when it no longer has focus
            //     enums::Event::Unfocus => {
            //         p.hide();
            //         true
            //     }
            //     _ => false,
            // });

            // activation_frame.handle(move |_, evt| match evt {
            //     enums::Event::Push => {
            //         if app::event_mouse_button() == app::MouseButton::Left {
            //             let (ex, ey) = (app::event_x_root(), app::event_y_root());
            //             popup.set_pos(ex, ey);
            //             popup.show();
            //         } else if app::event_mouse_button() == app::MouseButton::Right {
            //             let (ex, ey) = app::event_coords();
            //             let menu = menu::MenuItem::new(&["Item 1", "Item 2"]);
            //             menu.popup(ex, ey);
            //         }

            //         true
            //     }
            //     _ => false,
            // });


            // win.force_position(true);

			// let (s, r) = fltk::app::channel();
            let mut vpack=fltk::group::Pack::default()
            // .with_size(120,120)
            .center_of(&win);
            // dragwin(&mut win);
            // let mut disp = TextDisplay::default().with_size(200,50);
            // let mut buf = TextBuffer::default();
            // buf.append("Initiating app\n");
            // disp.set_buffer(buf);
            // disp.set_label("title");
            // win.resizable(&disp);
            let mut frame = fltk::frame::Frame::default()
                // .with_size(60,60)
                ;
                frame.set_label_color(Color::White);
            frame.set_label("title");
            // let network_interfaces = NetworkInterface::show().unwrap();

            // for (no,itf) in network_interfaces.iter().enumerate() {
            //     println!("{}----{}", no,itf.name);
            //     println!("{:?}----{}", no,itf.addr.unwrap().ip());
            // }

            // std::thread::spawn({
            //     move || loop {

            // // win.redraw_overlay();
            //         std::thread::sleep(std::time::Duration::from_millis(1000));
            //     }
            // });

            // The returned result indicates that the task failed.
            // assert!(join.await.is_err());
            // let elapsed_time = curr_time.duration_since(prev_net_access_time).as_secs_f64();

            // let (rx, tx) = if elapsed_time == 0.0 {
            //     (0, 0)
            // } else {
            //     (
            //         ((total_rx.saturating_sub(*prev_net_rx)) as f64 / elapsed_time) as u64,
            //         ((total_tx.saturating_sub(*prev_net_tx)) as f64 / elapsed_time) as u64,
            //     )
            // };

            // *prev_net_rx = total_rx;
            // *prev_net_tx = total_tx;
            // prev_net_access_time=curr_time;

            // format!(
            //     "cat /sys/class/net/{0}/statistics/rx_bytes",
            //     ""
            // );
            // format!(
            //     "cat /sys/class/net/{0}/statistics/tx_bytes",
            //     ""
            // );

            // let mut lbs=fltk::group::Pack::default().with_size(120,480);
			// Button::default().with_size( 60, 60)
            // .with_label("exit").emit(s.clone(), "exit");
            // Button::default().with_size( 60, 60)
            // .with_label("reload").emit(s.clone(), "reloaddb");
            // lbs.end();
            vpack.end();
            vpack.set_type(fltk::group::PackType::Vertical);
			win.show_with_env_args();

            // win.set_opacity(0.9);
            // win.set_border(false);
            // win.set_override();
            // win.set_color(Color::White);

            win.make_modal(true);
            win.set_override();
            win.force_position(true);
            // println!("{}",win.is_override());

            win.end();

            win.show();

            thread::spawn(move || loop {
                println!("fromhere------------>2");
                tx.send(updateusage(false/*,&mut val,&mut ptx,&mut prx*/,iname.clone(),&mut dtpr));
                // updateusage(true,&mut val,&mut ptx,&mut prx,iname.clone());
                thread::sleep(Duration::from_secs(1));
            });
            while app.wait() {
                match rx.recv(){
                    Some(k)=>{
                        frame.set_label(&k);
                    },
                    None=>{
                        // frame.set_label("NA");
                    }
                }

            // frame.set_label(&format!("{:?}",time::Instant::now().elapsed()));

                win.redraw();

            }
            app.run().unwrap();
}
// const DEFAULT_SIZE: Byte = Byte::from_bytes(1024);

fn updateusage(whethertosave:bool/*,val:&mut u128,ptx:&mut u64,prx:&mut u64*/,iname:String,dtpr:&mut Vec<u64>)->String{
//    for i in dtpr.into_iter(){
//        print!("{}---",i);
//    }
//    print!("\n");
    let date = Local::now();
    let current_date = date.format("%Y-%m-%d").to_string();
    // for (dt,usg) in abserdeapi::setup(true, HashMap::new()).1{
    //     if(dt==current_date){
    //         *val= usg;
    //     }
    // }
    println!("fromhere------------>4");
    dtpr[0] = match getasv().1.get(&current_date){
                        Some(a)=>{
                            *a as u64
                        },
                        None=>{
                            0 as u64
                        }
    };
//     dtpr[0] = match setup(true, getasv().1).1.get(&current_date){
//            Some(usg)=>{
//                usg.to_owned() as u64
//            },
//            _=>{
//                0
//            }
//    };
    // println!("");

    // println!("{}",byte_unit::Byte::from_bytes(*val).get_appropriate_unit(true));

    // let my_abserde = Abserde {
    //     app: "tnsoverlay".to_string(),
    //     location: Location::Auto,
    //     format: Format::Toml,
    // };
            let mut sys = System::new();
            // First we update all information of our `System` struct.
            sys.refresh_networks_list();
            let mut total_rx: u64 = 0;
            let mut couldfind=false;
            let mut total_tx: u64 = 0;

            // let sys: &sysinfo::System;
            // let prev_net_access_time: Instant;
            // let prev_net_tx: &mut u64;
            // let curr_time: Instant= std::time::Instant::now();

            // let prev_net_rx: &mut u64;
            let networks = sys.networks();
            let mut k=0;
            for (name, network) in networks {
                // println!("--------->{}",name);
                // let to_keep = if let Some(filter) = filter {
                //     let mut ret = filter.is_list_ignored;
                //     for r in &filter.list {
                //         if r.is_match(name) {
                //             ret = !filter.is_list_ignored;
                //             break;
                //         }
                //     }
                //     ret
                // } else {
                //     true
                // };

                // if to_keep {
                    // if name == "enp5s0"
                    // if !name.contains("bridge")
                    // if network.
                    // let mut nametostat=iname;
                    if(iname=="all"){
                    total_rx += network.total_received();
                    total_tx += network.total_transmitted();
                    couldfind=true;
                    // k=1;
                    }
                    else if(*name == *iname){
                        total_rx += network.total_received();
                        total_tx += network.total_transmitted();
                        couldfind=true;
                        break;
                    }

                    // if k==1 {
                    //     k=0;
                    //     break;
                    // }

                //     total_rx += io.bytes_recv().get::<heim::units::information::bit>();
                // total_tx += io.bytes_sent().get::<heim::units::information::bit>();
                // }

            }
            // let mut contains=false;
            let mut turx=total_rx.saturating_sub(dtpr[2]);
            let mut tutx=total_tx.saturating_sub(dtpr[1]);
            if dtpr[1]!=0 ||dtpr[2]!=0 {
                dtpr[0]+=turx+tutx;
                let mut dm:HashMap<String,u128>=HashMap::new();
            let date = Local::now();
            let current_date = date.format("%Y-%m-%d").to_string();
            // for (dt,usg) in abserdeapi::setup(true, HashMap::new()).1{

            //     if(dt==current_date){
            //         contains=true;
            //     }
            //     else{
            //         contains=false;
            //     }

            // }
            // if(contains){
            //     let mut local =val.clone();
            //     *abserdeapi::setup(false, HashMap::new()).1.get_mut(&current_date).unwrap()=*val;
            // }
            // else
            if whethertosave{
//                let mh =&mut abserdeapi::setup(true, getasv().1).1;
                let mh =&mut getasv().1;
                mh.insert(current_date, dtpr[0] as u128);
                abserdeapi::setup(false, mh.to_owned());
            }
            // println!("");

    // println!("{}",byte_unit::Byte::from_bytes(*val).get_appropriate_unit(true));
            }

            // turx*=8;
            // tutx*=8;
            dtpr[1]=total_tx;
            dtpr[2]=total_rx;

            let tt=total_rx+total_tx;
            // let tt=updateupdowndatausg(ptx, prx);

            let byte_rx = byte_unit::Byte::from_bytes(turx as u128);
            let byte_tx = byte_unit::Byte::from_bytes(tutx as u128);
            let byte_t = byte_unit::Byte::from_bytes(tt as u128);

            // let mut dm:HashMap<String,u128>=HashMap::new();
            // let date = Local::now();
            // let current_date = date.format("%Y-%m-%d").to_string();


            // let k = date.format("%Y-%m-%d").to_string();
            // let my_oconfig = MyConfig::load_config(&my_abserde).unwrap();
            // let ntd= my_oconfig.user_data.get(&current_date).unwrap();
            // dm.insert(current_date, (val.to_owned()+turx as u128+tutx as u128));

            // let mut my_config = MyConfig {
            //     // window_height:my_oconfig.window_height,
            //     // window_width:my_oconfig.window_width,
            //     // window_x: my_oconfig.window_x,
            //     // window_y:my_oconfig.window_y,
            //     user_data:dm
            //   };
            //   my_config.save_config(&my_abserde).expect("msg");





            let adjusted_rx = byte_rx.get_appropriate_unit(true);

            let adjusted_tx = byte_tx.get_appropriate_unit(true);
            // let adjusted_t = byte_t.get_appropriate_unit(true);
            let adjusted_st = byte_unit::Byte::from_bytes(dtpr[0] as u128).get_appropriate_unit(true);

// adjusted_byte.format();
            // println!("{}↓ {}↑",adjusted_rx,adjusted_tx);
            // std::thread::sleep(std::time::Duration::from_millis(1000));

            // format!("{}↓ {}↑ {}",adjusted_rx,adjusted_tx,adjusted_t )
            // let d=networks.contains_key();
            // let d=networks.into_iter().filter_map(|(key, value)| {if()};
            if couldfind{
                format!("{}↓ {}↑ {}",adjusted_rx,adjusted_tx,adjusted_st )
            }
            else{
                format!("No such network")
            }


    //         let since_the_epoch = SystemTime::now()
    //     .duration_since(UNIX_EPOCH)
    //     .expect("Time went backwards");
    // format!("{:?}", since_the_epoch)
}

fn updateupdowndatausg(ptx:&mut u64,prx:&mut u64)->u128{
    let mut sys = System::new();
            // First we update all information of our `System` struct.
            sys.refresh_networks_list();
            let mut total_rx: u64 = 0;
            let mut total_tx: u64 = 0;

            // let sys: &sysinfo::System;
            // let prev_net_access_time: Instant;
            // let prev_net_tx: &mut u64;
            // let curr_time: Instant= std::time::Instant::now();

            // let prev_net_rx: &mut u64;
            let networks = sys.networks();
            let mut k=0;
            for (name, network) in networks {
                // println!("--------->{}",name);
                // let to_keep = if let Some(filter) = filter {
                //     let mut ret = filter.is_list_ignored;
                //     for r in &filter.list {
                //         if r.is_match(name) {
                //             ret = !filter.is_list_ignored;
                //             break;
                //         }
                //     }
                //     ret
                // } else {
                //     true
                // };

                // if to_keep {
                    // if name == "enp5s0"
                    {
                    total_rx += network.total_received();
                    total_tx += network.total_transmitted();
                    k=1;
                    }
                    // if k==1 {
                    //     k=0;
                    //     break;
                    // }

                //     total_rx += io.bytes_recv().get::<heim::units::information::bit>();
                // total_tx += io.bytes_sent().get::<heim::units::information::bit>();
                // }

            }

            let mut turx=total_rx.saturating_sub(*prx);
            let mut tutx=total_tx.saturating_sub(*ptx);
            // turx*=8;
            // tutx*=8;
            *ptx=total_tx;
            *prx=total_rx;

            let tt=total_rx+total_tx;

            tt as u128
}