use std::{process, collections::HashMap, fs::File, io::{Read, Write}};
use abserde::*;
use chrono::Local;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Default, Debug)]

struct MyConfig {
    showgui: bool,
//    lastuploadtoday: u128,
//    lastdownloadtoday: u128,
//    total: u128,
    // #[serde(with = "indexmap::serde_seq")]
	// users: Vec<String>,
    // #[serde(with = "indexmap::serde_seq")]
	// usage: u128,
    usage: HashMap<String,u128>,
	// iname: String,
	// window_x: usize,
	// window_y: usize,
	// theme: String,
    // #[serde(with = "indexmap::serde_seq")]
	// bookmarklist: IndexMap<String, String>,
}
pub fn getasv()->(bool, HashMap<String, u128>){
    let my_abserde = Abserde {
        app: "TNS".to_string(),
        location: Location::Auto,
        format: Format::Json,
    };
    if MyConfig::load_config(&my_abserde).is_ok(){
        let map =MyConfig::load_config(&my_abserde).unwrap();
//            (map.showgui,map.usage,map.lastuploadtoday,map.lastdownloadtoday,map.total)
            (map.showgui,map.usage)
        }
        else {

            // warn!("Error while loading preferences: {:?}", e);
            println!("Error.Config File Not found.");
            (false,HashMap::new())
            // process::exit(1);
        }
//    (bv,hmv,tt)
}
pub fn readnstore(readorwrite:bool,wtw:u64){
    let file_name = "du.txt";
    match File::open(file_name) {
        Ok(mut file) => {
            if readorwrite{
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            }
            else{
//            writeln!(file, "Hello my name is Missy!").unwrap(); // writing using the macro 'writeln!'
            file.write_all(&wtw.to_ne_bytes()).unwrap();

            }
        },
        Err(_) => println!("Unable to create the file: '{}'", file_name),
    }
}
pub fn setup(mut laodifneccesary: bool,shm:HashMap<String,u128>) -> (bool,HashMap<String,u128>) {
//    let k=true;
//    if k{
//        return (false,HashMap::new());
//    }
    // fn setup(mut needload: bool) -> (IndexMap<String,String>,bool,Vec<Bookmark>,Vec<String>) {
    // fn setup(mut needload: bool) -> (IndexMap<String,String>,bool,Vec<String>) {

        // let mut pref = IndexMap::<String,String>::new();
        // let mut blist=Vec::new() ;
        // let mut users:Vec<String>=vec!["".to_string()];
        // users.push("".to_string());
        // let mut needload = false;
        let mut issue = false;
        let my_abserde = Abserde {
            app: "TNS".to_string(),
            location: Location::Auto,
            format: Format::Json,
        };
        // if MyConfig::load_config(&my_abserde).expect("").user_data.is_empty(){
        //
        // }
        // my_abserde.delete().expect("");

        match MyConfig::load_config(&my_abserde){
                  Ok(map) => {

                  }
                  Err(e) => {
                    // needload=true;
                    issue=true;
                    // warn!("Error while loading preferences: {:?}", e);
                    println!("Error.Config File Not found.Reinit using loaddata");
                    // process::exit(1);
                  }
                }

                    let my_config = MyConfig {
                        showgui: true,
//                        total:tt,
                        // users: users,
                        usage:shm,
                        // iname:i_name
                    // window_height: 45,
                    // window_x: 45,
                    // window_y: 45,
                    // theme: "".to_string(),
                    // bookmarklist: pref,
                    };
                    // println!("reloading");

                    // println!("{:?}",dirs::config_dir().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "no system config directory detected")));
                    if laodifneccesary{
                        if issue{
                            println!("reinit");
                            my_config.save_config(&my_abserde);
                        }
                    }
                    else{
                        println!("------------saving-----------{:?}",my_config.usage);
                        my_config.save_config(&my_abserde);
                    }


                // if(issue){
                //     HashMap::<String,String>::new()
                // }
                // else
                {
                     (//MyConfig::load_config(&my_abserde).expect("").bookmarklist,
                    MyConfig::load_config(&my_abserde).expect("").showgui,
                    MyConfig::load_config(&my_abserde).expect("").usage,
//                     MyConfig::load_config(&my_abserde).expect("").total,

                    // MyConfig::load_config(&my_abserde).expect("").iname,
                    // MyConfig::load_config(&my_abserde).expect("").users
                )
                }


    }