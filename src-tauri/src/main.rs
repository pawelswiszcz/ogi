#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[derive(Debug)]
struct Device {
    pub id: String,
    pub name: String,
    pub resolutions: Vec<Resolution>,
    pub full_info: String,
}

#[derive(Debug)]
struct Resolution {
    pub value: String,
    pub index: i32,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_device_info(device_name: &str) -> String {
    let output = Command::new("ratbagctl")
        .arg(device_name)
        .arg("info")
        // Tell the OS to record the command's output
        .stdout(Stdio::piped())
        // execute the command, wait for it to complete, then capture the output
        .output()
        // Blow up if the OS was unable to start the program
        .unwrap();

    // extract the raw bytes that we captured and interpret them as a string
    let stdout = String::from_utf8(output.stdout).unwrap();
    return stdout;
}
#[tauri::command]
fn get_all_devices_to_view() -> String {
    //device = "MSG FROM RUST";

    let all_devices_dist = get_all_devices();

    let mut result = String::new();

    for i in &all_devices_dist {
        result = result + i.full_info.as_str();
    }

    return result;
}

use std::process::{Command, Stdio};

fn set_dpi(device_name: &str, dpi: &str) {
    let output = Command::new("ratbagctl")
        .arg(device_name)
        .arg("dpi")
        .arg("set")
        .arg(dpi)
        // Tell the OS to record the command's output
        .stdout(Stdio::piped())
        // execute the command, wait for it to complete, then capture the output
        .output()
        // Blow up if the OS was unable to start the program
        .unwrap();

    // extract the raw bytes that we captured and interpret them as a string
    let stdout = String::from_utf8(output.stdout).unwrap();

    println!("{}", stdout);
}

fn set_active_resolution_from_set(device_name: &str, set: &str) {
    let output = Command::new("ratbagctl")
        .arg(device_name)
        .arg("resolution")
        .arg("active")
        .arg("set")
        .arg(set)
        // Tell the OS to record the command's output
        .stdout(Stdio::piped())
        // execute the command, wait for it to complete, then capture the output
        .output()
        // Blow up if the OS was unable to start the program
        .unwrap();

    // extract the raw bytes that we captured and interpret them as a string
    let stdout = String::from_utf8(output.stdout).unwrap();

    println!("{}", stdout);
}

fn get_device_name(name: &str) -> String {
    let output = Command::new("ratbagctl")
        .arg(name)
        .arg("name")
        // Tell the OS to record the command's output
        .stdout(Stdio::piped())
        // execute the command, wait for it to complete, then capture the output
        .output()
        // Blow up if the OS was unable to start the program
        .unwrap();

    // extract the raw bytes that we captured and interpret them as a string
    let stdout = String::from_utf8(output.stdout).unwrap();

    return stdout;
}

fn get_resolution(device_name: &str, id: &str) -> String {
    let output = Command::new("ratbagctl")
        .arg(device_name)
        .arg("resolution")
        .arg(id)
        .arg("get")
        // Tell the OS to record the command's output
        .stdout(Stdio::piped())
        // execute the command, wait for it to complete, then capture the output
        .output()
        // Blow up if the OS was unable to start the program
        .unwrap();

    // extract the raw bytes that we captured and interpret them as a string
    let stdout = String::from_utf8(output.stdout).unwrap();

    return stdout;
}

fn get_all_devices() -> Vec<Device> {
    let output = Command::new("ratbagctl")
        .arg("list")
        // Tell the OS to record the command's output
        .stdout(Stdio::piped())
        // execute the command, wait for it to complete, then capture the output
        .output()
        // Blow up if the OS was unable to start the program
        .unwrap();

    // extract the raw bytes that we captured and interpret them as a string
    let stdout = String::from_utf8(output.stdout).unwrap();

    let mut devices: Vec<Device> = Vec::new();

    let devices_lines = stdout.lines();

    for device_line in devices_lines {
        let device_row = device_line.trim();
        let dot_index = device_row.find(":");
        let id = device_row.get(0..dot_index.unwrap()).unwrap().to_string();
        let name = device_row
            .get(dot_index.unwrap() + 3..)
            .unwrap()
            .to_string();

        let mut resolutions: Vec<Resolution> = Vec::new();
        for i in 0..5 {
            let dev_res_value = get_resolution(&name.to_string(), &i.to_string());
            resolutions.push(Resolution {
                value: dev_res_value,
                index: i,
            });
        }

        let full_info = get_device_info(&name.to_string());

        let device = Device {
            id,
            name,
            resolutions,
            full_info,
        };

        devices.push(device);
    }

    return devices;
}

use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

fn main() {
    let all_devices_dist = get_all_devices();

    let mut tray_menu = SystemTrayMenu::new();

    for i in &all_devices_dist {
        let device_try_item = CustomMenuItem::new("DEVICE", i.name.to_string());
        tray_menu = tray_menu.add_item(device_try_item);
        //let device_try_item = CustomMenuItem::new("DPI".to_string(), "Dpi");
        //tray_menu = tray_menu.add_item(device_try_item);

        for f in &i.resolutions {
            let res_item = f.value.as_str().trim();
            let dot_index = res_item.find(":");

            let res_item_right = res_item
                .get(dot_index.unwrap() + 2..)
                .unwrap()
                .to_string()
                .replace(" (active)", "");
            let res_item_right_id = f.index.to_string() + "_" + i.id.as_str();

            //let res_id = f.index;

            let device_try_item = CustomMenuItem::new(res_item_right_id, &res_item_right);
            tray_menu = tray_menu.add_item(device_try_item);
        }

        tray_menu = tray_menu.add_native_item(SystemTrayMenuItem::Separator);
    }

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let show = CustomMenuItem::new("show".to_string(), "Show");

    tray_menu = tray_menu.add_item(hide);
    tray_menu = tray_menu.add_item(show);
    tray_menu = tray_menu.add_native_item(SystemTrayMenuItem::Separator);
    tray_menu = tray_menu.add_item(quit);

    //tray_menu = tray_menu.add_submenu(quit);

    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|_app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                //let item_handle = _app.tray_handle().get_item(&id);

                //println!("{:?}", item_handle);
                match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "hide" => {
                        let window = _app.get_window("main").unwrap();
                        window.hide().unwrap();
                        //item_handle.set_title("show").unwrap();
                    }
                    "show" => {
                        let window = _app.get_window("main").unwrap();
                        window.show().unwrap();
                        //item_handle.set_title("hide").unwrap();
                    }
                    "DEVICE" => {
                        let window = _app.get_window("main").unwrap();
                        window.show().unwrap();
                    }
                    _ => {
                        let item = id
                            .as_str()
                            .replace("dpi", "")
                            .replace(" (active)", "")
                            .replace(" (default)", "");

                        //println!("{:?}", item);
                        let device_pointer = item.find("_");
                        let resolution_id =
                            item.get(0..device_pointer.unwrap()).unwrap().to_string();
                        let device = item.get(device_pointer.unwrap() + 1..).unwrap().to_string();

                        //let id_as_string = id.as_str();
                        //println!("{:?}", id_as_string);

                        set_active_resolution_from_set(&device, resolution_id.as_str());
                        //println!("{:?}", device);
                        //set_dpi(&device, &dpi);
                    }
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![get_all_devices_to_view])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
