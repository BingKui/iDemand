use fs_err as fs;

use serde::{Deserialize, Serialize};
use std::path::Path;
use tauri::AppHandle;

#[derive(Serialize, Deserialize)]
pub struct SettingInfo {
    pub code_app: String, // 代码软件
    pub is_notice: bool, // 是否提醒
    pub notice_time: String, // 提醒时间
    pub auto_start: bool, // 是否自启动
    // pub auto_restart_wifi: bool,
    // pub auto_save: bool,
    pub auto_update: bool, // 是否自动更新
    pub dock_show: bool, // 是否dock显示
}

pub struct SettingApp {
    pub path: String,
}

impl SettingApp {
    pub fn new(app_handle: &AppHandle) -> SettingApp {
        let res_dir = app_handle.path_resolver().app_dir().unwrap();
        let db_path = res_dir.join("setting.json");
        let is_folder = Path::new(res_dir.as_os_str()).exists();
        if !is_folder {
            fs::create_dir(res_dir.as_os_str().to_str().unwrap().to_string()).unwrap();
        }
        let is_exist = Path::new(&db_path).exists();
        if !is_exist {
            let info: SettingInfo = SettingInfo {
                code_app: "vscode".to_string(),
                dock_show: false,
                is_notice: false,
                notice_time: "10:00".to_string(),
                auto_start: false,
                auto_update: false,
            };
            let serialized = serde_json::to_string(&info).unwrap();
            // 创建文件并初始化
            fs::write(db_path.as_os_str().to_str().unwrap().to_string(), serialized).unwrap();
        }
        let path = db_path.as_os_str().to_str().unwrap().to_string();
        SettingApp {
            path,
        }
    }
    pub fn get_sys(&self) -> SettingInfo {
        let contents = fs::read_to_string(self.path.to_string()).unwrap();
        let setting: SettingInfo = serde_json::from_str(&contents).unwrap();
        setting
    }
    pub fn set_sys(&self, setting: SettingInfo) -> bool {
        let content = serde_json::to_string(&setting).unwrap();
        println!("设置信息为 -> {:?}", content);
        fs::write(self.path.to_string(), content).unwrap();
        true
    }
}
