#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod menu;
use mysql::{*, prelude::Queryable};
use serde::{Deserialize, Serialize};
// use serde::Serialize;

//just prevents the cmd window from opening
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
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

#[derive(Debug, Serialize, Deserialize)]
struct TauriToadRow{
    values:Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
struct TauriToadColumn{
    value:String,
    datatype:String
}

#[derive(Debug, Serialize, Deserialize)]
struct TauriToadReturn {
    columns: Vec<TauriToadColumn>,
    rows: Vec<TauriToadRow>
}

#[tauri::command]
fn execute(sql: &str) ->  TauriToadReturn{
    let vec = match bk_exec(sql){
        Ok(r)=>r,
        Err(e)=> return TauriToadReturn{
                columns:vec![TauriToadColumn{value:e.to_string(),datatype:"ERROR".to_string()}],
                rows:vec![]
            }
    };
    let mut r = Vec::new();

    if vec.is_empty(){
        return TauriToadReturn{columns:vec![],rows:vec![]};
    }

    let mut out_columns = Vec::new();
    for i in vec[0].columns_ref().clone(){
        // let columns = i.; 
        out_columns.push(TauriToadColumn{value:i.name_str().to_string(), datatype:"".to_string()})
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
        r.push(TauriToadRow{values:vals_vec});
    }
    let output = TauriToadReturn{columns:out_columns, rows:r}; 
    return output;
}

fn main() {
    let menu = menu::make();
    tauri::Builder::default()
        .menu(menu)
        .invoke_handler(tauri::generate_handler![greet, execute])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
