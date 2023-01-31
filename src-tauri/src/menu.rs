use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

pub fn make_menu()->Menu{
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    let menu = Menu::new()
    .add_native_item(MenuItem::Copy)
    .add_item(CustomMenuItem::new("hide", "Hide"))
    .add_submenu(submenu);
    return menu;

}

pub fn make_window(){
    let local_window = tauri::WindowBuilder::new(
        &app,
        "local",
        tauri::WindowUrl::App("index.html".into())
      ).build()?;

}