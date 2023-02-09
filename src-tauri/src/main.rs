#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod screen;
mod drivers;
use return_types::{MyRow, MyColumn, MyReturn};
mod return_types;
use std::collections::HashMap;
use std::sync::Mutex;
use mysql::{*, prelude::Queryable};

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

    pub fn execute(&self, sql:&str){
        match self {
            Driver::Oracle(oracle) => oracle.print(),
            Driver::Mysql(mysql) => mysql.print(),
            _=>println!("{}","unimplemented driver".to_string())
        }
    }

    pub fn connect(&self, sql:&str){
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
fn connect(state: tauri::State<Connections>, driver:&str, url: &str, username: &str, password: &str, database: &str) -> String {
    let new_driver = match driver {
                "mysql"=> Driver::Mysql(drivers::mysql::MySQL{
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
    state.0.lock().unwrap().insert("test", new_driver);
    println!("{}-{}-{}-{}-{}", driver, url, username, password, database);
    return "Success".to_string();
}

fn bk_exec(sql: &str)->std::result::Result<Vec<mysql::Row>,mysql::Error>{
    let url = "mysql://test:password@10.200.11.141:3306";

    let pool = match Pool::new(url){
        Ok(x)=>x,
        Err(e)=> return Err(e)
    };

    let mut conn = match pool.to_owned().get_conn(){
        Ok(x)=>x,
        Err(e)=> return Err(e)
    };

    let vec = match conn.query(sql){
        Ok(x)=>x.to_owned().into_iter().collect(),
        Err(e)=> return Err(e)
    };
    // let x = vec;
    return Ok(vec);
}


#[tauri::command]
fn execute(state: tauri::State<Connections>, sql: &str) ->  MyReturn{
    let binding = state.0.lock().unwrap();
    let t = binding.get("test").unwrap().to_owned();
        t.print();
    
    let vec = match bk_exec(sql){
        Ok(r)=>r,
        Err(e)=> return MyReturn{
                columns:vec![MyColumn{value:e.to_string(),datatype:"ERROR".to_string()}],
                rows:vec![]
            }
    };
    let mut r = Vec::new();

    if vec.is_empty(){
        return MyReturn{columns:vec![],rows:vec![]};
    }

    let mut out_columns = Vec::new();
    for i in vec[0].columns_ref().clone(){
        // let columns = i.; 
        out_columns.push(MyColumn{value:i.name_str().to_string(), datatype:"".to_string()})
    }

    for i in vec {

        //as of now every row loops through and filles the columns
        // as a side project i am gonna leave it, but will proly 
        //chagne to {key:value} output style

        let vals = i.unwrap();
       
        let mut vals_vec = Vec::new();
        for val in vals{   
            vals_vec.push(val.as_sql(true).to_owned())
            // r.push(val.as_sql(true).to_owned());
        }
        r.push(MyRow{values:vals_vec});
    }
    let output = MyReturn{columns:out_columns, rows:r}; 
    return output;
}

#[derive(Debug, Default)] //
struct Connections<'a>(Mutex<HashMap<&'a str,Driver>>);

fn main() {

    let menu = screen::make_menu();
    let _app = tauri::Builder::default()
        .menu(menu)
        .manage(Connections::default())
        .invoke_handler(tauri::generate_handler![execute,test_connection,connect])
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
