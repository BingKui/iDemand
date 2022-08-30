#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[cfg(target_os = "macos")]
#[macro_use]

mod demand;
// mod log;
mod setting;
mod win;

use demand::{DemandApp, DemandItem};
use setting::{SettingApp, SettingInfo};
use tauri::CustomMenuItem;
use tauri::Manager;
use tauri::RunEvent;
use tauri::{AppHandle, Window, WindowBuilder, WindowEvent};
use tauri::{SystemTray, SystemTrayEvent};
use tauri::{SystemTrayMenu, SystemTrayMenuItem};
use win::{WinApp, WinItem};

fn main() {
  // 解决路径问题，No such file or directory
  fix_path_env::fix().unwrap();
  // 内容
  let context = tauri::generate_context!();
  // 系统托盘
  let main = CustomMenuItem::new("open".to_string(), "打开");
  let exit = CustomMenuItem::new("exit".to_string(), "退出");
  let tray_menu = SystemTrayMenu::new()
    .add_item(main)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(exit); // insert the menu items here
  let system_tray = SystemTray::new().with_menu(tray_menu);
  let mut app = tauri::Builder::default()
    .menu(tauri::Menu::os_default("iCheck"))
    .system_tray(system_tray)
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
        "open" => {
          let main_win = app.get_window("main").unwrap();
          main_win.set_focus().unwrap();
          main_win.show().unwrap();
        }
        "exit" => {
          std::process::exit(0);
        }
        _ => {}
      },
      _ => {}
    })
    .setup(|app| {
      let app_handle = app.handle();
      // 打开所有的桌面窗口
      let win_app = WinApp::new(&app_handle).unwrap();
      let win_list = win_app.get_win_list().unwrap();
      for win in win_list {
        create_win(app.handle(), &win);
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      // setting 相关
      get_setting,
      // 需求
      add_demand,
      delete_demand,
      update_demand,
      update_status,
      get_demand,
      get_demands,
      get_demands_by_status,
      open_demand,
      get_wins,
      update_position,
      update_fixed,
      delete_win,
    ])
    .build(context)
    .expect("error while running tauri application");
  let setting_app = SettingApp::new(&app.handle());
  // 添加事件监听
  app.listen_global("change-setting", move |event| {
    let str = event.payload().unwrap();
    let setting: SettingInfo = serde_json::from_str(str).unwrap();
    setting_app.set_sys(setting);
  });
  // 设置为无dock
  #[cfg(target_os = "macos")]
  app.set_activation_policy(tauri::ActivationPolicy::Accessory);
  // 设置关闭保留系统托盘
  app.run(|app_handle, e| match e {
    // 禁用原生关闭事件，改为隐藏
    RunEvent::WindowEvent {
      label,
      event: WindowEvent::CloseRequested { api, .. },
      ..
    } => {
      if label == "main" {
        let app_handle = app_handle.clone();
        let window = app_handle.get_window(&label).unwrap();
        api.prevent_close();
        window.hide().unwrap();
      }
    }
    RunEvent::ExitRequested { api, .. } => {
      api.prevent_exit();
    }
    _ => {}
  });
}

// 系统设置相关-获取设置
#[tauri::command]
fn get_setting(app_handle: AppHandle) -> SettingInfo {
  let sys_app = SettingApp::new(&app_handle);
  let setting = sys_app.get_sys();
  setting
}

// 添加需求
#[tauri::command]
fn add_demand(app_handle: AppHandle, item: DemandItem) -> bool {
  let demand_app = DemandApp::new(&app_handle).unwrap();
  let result = demand_app.add_demand(item);
  result
}
// 删除需求
#[tauri::command]
fn delete_demand(app_handle: AppHandle, id: String) -> bool {
  let demand_app = DemandApp::new(&app_handle).unwrap();
  let result = demand_app.del_demand(id);
  result
}

// 修改需求
#[tauri::command]
fn update_demand(app_handle: AppHandle, item: DemandItem) -> bool {
  let demand_app = DemandApp::new(&app_handle).unwrap();
  let result = demand_app.update_demand(item);
  result
}
// 修改需求状态
#[tauri::command]
fn update_status(app_handle: AppHandle, id: String, status: f64) -> bool {
  let demand_app = DemandApp::new(&app_handle).unwrap();
  let result = demand_app.update_status(id, status);
  result
}

// 获取需求
#[tauri::command]
fn get_demand(app_handle: AppHandle, id: String) -> DemandItem {
  let demand_app = DemandApp::new(&app_handle).unwrap();
  let result = demand_app.get_item(id).unwrap();
  result
}

// 获取需求列表
#[tauri::command]
fn get_demands(app_handle: AppHandle) -> Vec<DemandItem> {
  let demand_app = DemandApp::new(&app_handle).unwrap();
  let result = demand_app.get_demands().unwrap();
  result
}

// 获取特定需求列表
#[tauri::command]
fn get_demands_by_status(app_handle: AppHandle, status: f64) -> Vec<DemandItem> {
  let demand_app = DemandApp::new(&app_handle).unwrap();
  let result = demand_app.get_demands_by_status(status).unwrap();
  result
}

// 新建一个窗口
fn create_win(app_handle: AppHandle, item: &WinItem) -> Window {
  let mut url = "index.html/#/demand/".to_string();
  url.push_str(&item.demand_id);
  let target_url: String = url.parse().unwrap();
  let target = tauri::WindowUrl::App(target_url.into());
  let win = WindowBuilder::new(&app_handle, &item.demand_id, target)
    .visible(false)
    .resizable(false)
    .transparent(true)
    .build()
    .expect("创建 Demand 窗口失败！");
  win.set_decorations(false).unwrap();
  win.set_always_on_top(item.fixed).unwrap();
  win.set_size(tauri::LogicalSize::new(400, 200)).unwrap();
  win.set_position(tauri::PhysicalPosition::new(item.x, item.y)).unwrap();
  win
}

#[tauri::command]
fn open_demand(app_handle: tauri::AppHandle, id: String) -> bool {
  let win_app = WinApp::new(&app_handle).unwrap();
  let item = WinItem {
    demand_id: id.to_string(),
    theme: "default".to_string(),
    x: 0.0,
    y: 0.0,
    fixed: false,
    date: 0.0,
  };
  create_win(app_handle, &item);
  let result = win_app.add_win(item);
  result
}

// 获取所有小窗
#[tauri::command]
fn get_wins(app_handle: tauri::AppHandle) -> Vec<WinItem> {
  let win_app = WinApp::new(&app_handle).unwrap();
  let result = win_app.get_win_list().unwrap();
  result
}
// 更新窗口位置
#[tauri::command]
fn update_position(app_handle: tauri::AppHandle, id: String, x: f64, y: f64) -> bool {
  let win_app = WinApp::new(&app_handle).unwrap();
  let result = win_app.update_position(id, x, y);
  result
}
// 更新置顶
#[tauri::command]
fn update_fixed(app_handle: tauri::AppHandle, id: String, fixed: bool) -> bool {
  let win_app = WinApp::new(&app_handle).unwrap();
  let result = win_app.update_fixed(id, fixed);
  result
}
// 删除窗口
#[tauri::command]
fn delete_win(app_handle: tauri::AppHandle, id: String) -> bool {
  let win_app = WinApp::new(&app_handle).unwrap();
  let result = win_app.del_win(&id);
  result
}
