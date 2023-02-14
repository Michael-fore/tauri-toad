#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod screen;
mod drivers;
use std::collections::HashMap;
use drivers::return_types::MyReturn;
use drivers::return_types::MyRow;
use drivers::return_types::MyColumn;
use std::sync::Mutex;
use mysql::{*, prelude::Queryable};
use serde_json::json;
use serde_json;

#[derive(Debug)]
enum Driver {
    Oracle(drivers::oracle::Oracle),
    Mysql(drivers::mysql::MySQL),
    Postgres
}

impl Driver{
    pub fn print(&self){
        match self {
            Driver::Oracle(oracle) => oracle.print(),
            Driver::Mysql(mysql) => mysql.print(),
            _=>println!("{}","unimplemented driver".to_string())
        }
    }

    pub fn execute(&self, sql:&str)->MyReturn{
        match &self {
            Driver::Oracle(oracle) => MyReturn{columns:vec![MyColumn{value:"Unimplemented Driver".to_string(),datatype:"ERROR".to_string()}],rows:vec![]},
            Driver::Mysql(mysql) => mysql.execute(sql),
            _=>MyReturn{columns:vec![MyColumn{value:"Unimplemented Driver".to_string(),datatype:"ERROR".to_string()}],rows:vec![]}
        }
    }

    pub fn connect(&mut self){
        match self {
            Driver::Oracle(oracle) => oracle.connect(),
            Driver::Mysql(mysql) => mysql.connect(),
            _=>println!("{}","unimplemented driver".to_string())
        }
    }
}

#[tauri::command]
fn test_connection(driver:&str, url: &str, username: &str, password: &str, database: &str) -> String {
    //wtf does &* do?
    let conn_url = &*format!("{driver}://{username}:{password}@{url}/{database}");
    println!("Testing - {}",conn_url);
    let val = match Pool::new(conn_url){
        Ok(_)=> "Success".to_string(),
        Err(e)=> e.to_string()
    };
    println!("{}",val);
    return val;
}

#[tauri::command]
fn connect(state: tauri::State<Connections>, driver:&str, url: &str, username: &str, password: &str, database: &str, name: &str) -> String {
    let mut new_driver = match driver {
                "mysql"=> Driver::Mysql(drivers::mysql::MySQL{
                                name:name.to_string(),
                                username:username.to_string(),
                                password:password.to_string(),
                                url:url.to_string(),
                                port:123,
                                database:database.to_string(),
                                pool:None
                            }),
                "oracle"=>Driver::Oracle(drivers::oracle::Oracle{
                                username:username.to_string(),
                                password:password.to_string(),
                                url:url.to_string(),
                                port:123,
                                database:database.to_string()
                            }),
                _=> panic!("{}","INVALID DRIVER TYPE".to_string())
    };
    new_driver.connect();
    let length = state.0.lock().unwrap().len();
    let key = format!("{{name:{name},id:{length},url:{url}, username:{username}, driver:{driver}}}");
    state.0.lock().unwrap().insert(key, new_driver);
    println!("{}-{}-{}-{}-{}-{}", name, driver, url, username, password, database);
    return "Success".to_string();
}

#[tauri::command]
fn get_connections(state: tauri::State<Connections>) -> serde_json::Value {
    let binding = state.0.lock().unwrap();

    let mut keys= vec![]; 
    for key in binding.keys(){
        keys.push(key);
    }
    
    return json!(keys);
}


#[tauri::command]
fn execute(state: tauri::State<Connections>, sql: &str) -> MyReturn{
    let binding = state.0.lock().unwrap();
    let t = binding.get("test").unwrap().to_owned();
    t.print();
    let output = t.execute(sql);
    return output;
}

#[derive(Debug, Default)] //
struct Connections(Mutex<HashMap<String,Driver>>);

fn main() {

    let menu = screen::make_menu();
    let _app = tauri::Builder::default()
        .menu(menu)
        .manage(Connections::default())
        .invoke_handler(tauri::generate_handler![execute,test_connection,connect,get_connections])
        .on_menu_event(move |event| {
      match event.menu_item_id() {
        "quit" => {
          std::process::exit(0);
        }
        "close" => {
          event.window().close().unwrap();
        },
        "add_connection" => {
            println!("{}","RUST- clicked".to_string());
            event.window().emit("connect", "/connections").unwrap();
          },
        _ => {}
      }
    })   
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    // screen::make_window(&app);
    
}
