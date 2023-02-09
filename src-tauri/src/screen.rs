use tauri::{CustomMenuItem, Menu,  Submenu, Window};

pub fn make_menu()->Menu{
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let add_connection = CustomMenuItem::new("add_connection".to_string(), "Add connection");
    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    let connect_menu = Submenu::new("Connections", Menu::new().add_item(add_connection));
    let menu = Menu::new()
    // .add_native_item(MenuItem::Copy)
    .add_submenu(submenu)
    .add_submenu(connect_menu)
    .add_item(CustomMenuItem::new("hide", "Hide"));
   
    return menu;

}

pub fn make_window(app:&Window)->std::result::Result<(),tauri::Error>{
    let local_window = match tauri::WindowBuilder::new(
        app,
        "local",
        tauri::WindowUrl::App("index.html".into())
      ).build(){
        Ok (x)=>x,
        Err(_)=>panic!("Error creating window")
      };
      local_window.on_menu_event(move | event | {
        println!("{}",event.menu_item_id().to_string());
      });

      Ok(())
}