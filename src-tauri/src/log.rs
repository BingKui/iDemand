// 日志信息，用于统计报表
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

#[derive(Serialize, Deserialize)]
pub struct LogItem {
    pub id: String,
    pub log_type: String,
    pub item_type: String,
    pub date: f64,
}

pub struct LogApp {
    pub conn: Connection,
}

impl LogApp {
    pub fn new(app_handle: &AppHandle) -> Result<LogApp> {
        let res_dir = app_handle.path_resolver().app_data_dir().unwrap();
        let db_path = res_dir.join("idemand.sqlite");
        let is_exist = Path::new(&db_path).exists();
        if !is_exist {
            // 创建文件并初始化
            fs::write(db_path.as_os_str().to_str().unwrap().to_string(), "").unwrap();
        }
        let conn = Connection::open(db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS DemandLog (
                id varchar(64) PRIMARY KEY,
                log_type text DEFAULT '',
                item_type text DEFAULT '',
                date numeric DEFAULT 0
            )",
            [],
        )?;
        Ok(LogApp { conn })
    }
    // 添加日志
    pub fn add_log(&self, item: LogItem) -> bool {
        let LogItem {
            id,
            log_type,
            item_type,
            date,
        } = item;
        match self.conn.execute(
            "INSERT INTO DemandLog (id, log_type, item_type, date) VALUES (?, ?, ?, ?)",
            [id, log_type, item_type, date.to_string()]
        ) {
            Ok(insert) => {
                println!("{} row inserted", insert);
                true
            }
            Err(err) => {
                println!("some error: {}", err);
                false
            }
        }
    }
    pub fn get_all_logs(&self) -> Result<Vec<LogItem>> {
        let mut stmt = self.conn.prepare("SELECT * FROM DemandLog").unwrap();
        let items_iter = stmt.query_map([], |row| {
            Ok(LogItem {
                id: row.get(0)?,
                log_type: row.get(1)?,
                item_type: row.get(2)?,
                date: row.get(3)?,
            })
        })?;
        let mut items: Vec<LogItem> = Vec::new();
        for item in items_iter {
            items.push(item?);
        }
        Ok(items)
    }
    pub fn get_logs_by_type(&self, item_type: String, start_time: f64, end_time: f64) -> Result<Vec<LogItem>> {
        let mut stmt = self.conn.prepare("SELECT * FROM DemandLog WHERE item_type LIKE ? AND date BETWEEN ? AND ?").unwrap();
        let items_iter = stmt.query_map([item_type.to_string(), start_time.to_string(), end_time.to_string()], |row| {
            Ok(LogItem {
                id: row.get(0)?,
                log_type: row.get(1)?,
                item_type: row.get(2)?,
                date: row.get(3)?,
            })
        })?;
        let mut items: Vec<LogItem> = Vec::new();
        for item in items_iter {
            items.push(item?);
        }
        Ok(items)
    }
}
