use std::collections::HashMap;
use abserde::*;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Default, Debug)]
struct MyConfig {
    showgui: bool,
    usage: HashMap<String,u128>,
}
//read today total used bytes from config file
pub fn getasv()->(bool, HashMap<String, u128>){
    let my_abserde = Abserde {
        app: "TNS".to_string(),
        location: Location::Auto,
        format: Format::Json,
    };
    if MyConfig::load_config(&my_abserde).is_ok(){
        let map =MyConfig::load_config(&my_abserde).unwrap();
            (map.showgui,map.usage)
        }
        else {
            println!("Config File Not found.Should be auto created shortly.");
            (false,HashMap::new())
        }
}

// pub fn readnstore(readorwrite:bool,wtw:u64){
//     let file_name = "du.txt";
//     match File::open(file_name) {
//         Ok(mut file) => {
//             if readorwrite{
//             let mut contents = String::new();
//             file.read_to_string(&mut contents).unwrap();
//             }
//             else{
//             file.write_all(&wtw.to_ne_bytes()).unwrap();
//             }
//         },
//         Err(_) => println!("Unable to create the file: '{}'", file_name),
//     }
// }

//write today total used bytes to config file
pub fn setup(mut laodifneccesary: bool,shm:HashMap<String,u128>) -> (bool,HashMap<String,u128>) {
        let mut issue = false;
        let my_abserde = Abserde {
            app: "TNS".to_string(),
            location: Location::Auto,
            format: Format::Json,
        };
        match MyConfig::load_config(&my_abserde){
                  Ok(map) => {
                  }
                  Err(e) => {
                    issue=true;
                    println!("Config File Not found.Creating new config file.");
                  }
                }
                    let my_config = MyConfig {
                        showgui: true,
                        usage:shm,
                    };
                    if laodifneccesary{
                        if issue{
                            println!("reinit");
                            my_config.save_config(&my_abserde);
                        }
                    }
                    else{
                        // println!("------------saving-----------{:?}",my_config.usage);
                        my_config.save_config(&my_abserde);
                    }
                {
                    (
                        MyConfig::load_config(&my_abserde).expect("").showgui,
                        MyConfig::load_config(&my_abserde).expect("").usage,
                    )
                }
    }
