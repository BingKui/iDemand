use std::path::Path;
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use fs_err as fs;

#[derive(Serialize, Deserialize)]
pub struct WinItem {
    pub demand_id: String, // 需求id
    pub theme: String,     // 主题，后续可能支持
    pub x: f64,            // 位置
    pub y: f64,            // 位置
    pub fixed: bool,       // 是否置顶
    pub date: f64,         // 创建时间
}

pub struct WinApp {
    pub conn: Connection,
}

impl WinApp {
    pub fn new(app_handle: &AppHandle) -> Result<WinApp> {
        let res_dir = app_handle.path_resolver().app_data_dir().unwrap();
        let db_path = res_dir.join("idemand.sqlite");
        let is_folder = Path::new(res_dir.as_os_str()).exists();
        if !is_folder {
            fs::create_dir(res_dir.as_os_str().to_str().unwrap().to_string()).unwrap();
        }
        let is_exist = Path::new(&db_path).exists();
        if !is_exist {
            // 创建文件并初始化
            fs::write(db_path.as_os_str().to_str().unwrap().to_string(), "").unwrap();
        }
        let conn = Connection::open(db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS DemandWin (
                demand_id varchar(64) PRIMARY KEY,
                theme text default 'normal',
                x numeric DEFAULT 0,
                y numeric DEFAULT 0,
                fixed numeric DEFAULT 0,
                date numeric DEFAULT 0
            )",
            [],
        )?;
        Ok(WinApp { conn })
    }
    // 新加一个 win
    pub fn add_win(&self, item: WinItem) -> bool {
        let WinItem {
            demand_id,
            theme,
            x,
            y,
            fixed,
            date,
        } = item;
        let fixed_val;
        if fixed {
            fixed_val = 1;
        } else {
            fixed_val = 0;
        }
        match self.conn.execute(
            "INSERT INTO DemandWin (demand_id, theme, x, y, fixed, date) VALUES (?, ?, ?, ?, ?, ?)",
            [demand_id.to_string(), theme.to_string(), x.to_string(), y.to_string(), fixed_val.to_string(), date.to_string()],
        ) {
            Ok(insert) => {
                println!("{} 个 Win 添加成功！", insert);
                true
            }
            Err(err) => {
                println!("添加 Win 失败: {}", err);
                false
            }
        }
    }
    // 删除 win
    pub fn del_win(&self, id: &String) -> bool {
        match self.conn.execute("DELETE FROM DemandWin WHERE demand_id = ?", [id]) {
            Ok(..) => true,
            Err(err) => {
                println!("删除 Win 出错: {}", err);
                false
            }
        }
    }
    // 修改置顶
    pub fn update_fixed(&self, id: String, fixed: bool) -> bool {
        let fixed_value: i32;
        if fixed {
            fixed_value = 1
        } else {
            fixed_value = 0
        };
        match self.conn.execute(
            "UPDATE DemandWin SET fixed = ? WHERE demand_id = ?",
            [fixed_value.to_string(), id],
        ) {
            Ok(..) => true,
            Err(err) => {
                println!("更新 Win fixed 出错: {}", err);
                false
            }
        }
    }
    // 修改位置
    pub fn update_position(&self, id: String, x: f64, y: f64) -> bool {
        match self.conn.execute(
            "UPDATE DemandWin SET x = ?, y = ? WHERE demand_id = ?",
            [x.to_string(), y.to_string(), id],
        ) {
            Ok(..) => true,
            Err(err) => {
                println!("更新 Win 位置出错: {}", err);
                false
            }
        }
    }
    // 获取所有的 win
    pub fn get_win_list(&self) -> Result<Vec<WinItem>> {
        let mut stmt = self.conn.prepare("SELECT * FROM DemandWin").unwrap();
        let items_iter = stmt.query_map([], |row| {
            Ok(WinItem {
                demand_id: row.get(0)?,
                theme: row.get(1)?,
                x: row.get(2)?,
                y: row.get(3)?,
                fixed: row.get(4)?,
                date: row.get(5)?,
            })
        })?;
        let mut items: Vec<WinItem> = Vec::new();
        for item in items_iter {
            items.push(item?);
        }
        Ok(items)
    }
}
