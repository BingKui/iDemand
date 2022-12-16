// 清单管理
use std::path::Path;
use chrono::Local;
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use fs_err as fs;

#[derive(Serialize, Deserialize)]
pub struct ExtraLinkItem {
    pub text: String,
    pub link: String,
}

#[derive(Serialize, Deserialize)]
pub struct DemandItem {
    pub id: String,
    pub name: String,                    // 需求名字
    pub desc: String,                    // 需求描述信息
    pub status: f64,                     // 状态：1-待开发 2-开发中 3-已完成 4-已发布 5-已废弃
    pub publish_date: f64,               // 发布时间
    pub demand_link: String,             // 需求链接
    pub ui_link: String,                 // 设计稿链接
    pub api_link: String,                // 接口文档链接
    pub code_path: String,               // 代码地址
    pub publish_link: String,            // 发布地址
    pub extra_links: Vec<ExtraLinkItem>, // 额外链接
    pub create_date: f64,                // 创建时间
    pub update_date: f64,                // 更新时间
}

pub struct DemandApp {
    pub conn: Connection,
}

impl DemandApp {
    pub fn new(app_handle: &AppHandle) -> Result<DemandApp> {
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
            "CREATE TABLE IF NOT EXISTS DemandList (
                id varchar(64) PRIMARY KEY,
                name text NOT NULL,
                desc text DEFAULT '',
                status numeric DEFAULT 1,
                publish_date numeric DEFAULT 0,
                demand_link text NOT NULL,
                ui_link text DEFAULT '',
                api_link text DEFAULT '',
                code_path text DEFAULT '',
                publish_link text DEFAULT '',
                extra_links text DEFAULT '[]',
                create_date numeric DEFAULT 0,
                update_date numeric DEFAULT 0
            )",
            [],
        )?;
        Ok(DemandApp { conn })
    }
    pub fn add_demand(&self, item: DemandItem) -> bool {
        let DemandItem {
            id,
            name,
            desc,
            status,
            publish_date,
            demand_link,
            ui_link,
            api_link,
            code_path,
            publish_link,
            extra_links,
            ..
        } = item;
        let now = Local::now().timestamp();
        let extra_str = serde_json::to_string(&extra_links).unwrap();
        match self.conn.execute(
            "INSERT INTO DemandList (id, name, desc, status, publish_date, demand_link, ui_link, api_link, code_path, publish_link, extra_links, create_date) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            [id, name.to_string(), desc.to_string(), status.to_string(), publish_date.to_string(), demand_link.to_string(),
                ui_link.to_string(), api_link.to_string(), code_path.to_string(), publish_link.to_string(), extra_str, now.to_string()]
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
    pub fn del_demand(&self, id: String) -> bool {
        match self.conn.execute("DELETE FROM DemandList WHERE id = ?", [id]) {
            Ok(..) => true,
            Err(err) => {
                println!("some error: {}", err);
                false
            }
        }
    }
    pub fn get_item(&self, id: String) -> Result<DemandItem> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM DemandList WHERE id = ?")
            .unwrap();
        let item = stmt.query_row([id], |row| {
            let links_str: String = row.get(10).unwrap();
            let links = serde_json::from_str(&links_str).unwrap();
            Ok(DemandItem {
                id: row.get(0)?,
                name: row.get(1)?,
                desc: row.get(2)?,
                status: row.get(3)?,
                publish_date: row.get(4)?,
                demand_link: row.get(5)?,
                ui_link: row.get(6)?,
                api_link: row.get(7)?,
                code_path: row.get(8)?,
                publish_link: row.get(9)?,
                extra_links: links,
                create_date: row.get(11)?,
                update_date: row.get(12)?,
            })
        })?;
        Ok(item)
    }
    pub fn get_demands(&self) -> Result<Vec<DemandItem>> {
        let mut stmt = self.conn.prepare("SELECT * FROM DemandList").unwrap();
        let items_iter = stmt.query_map([], |row| {
            let links_str: String = row.get(10).unwrap();
            let links = serde_json::from_str(&links_str).unwrap();
            Ok(DemandItem {
                id: row.get(0)?,
                name: row.get(1)?,
                desc: row.get(2)?,
                status: row.get(3)?,
                publish_date: row.get(4)?,
                demand_link: row.get(5)?,
                ui_link: row.get(6)?,
                api_link: row.get(7)?,
                code_path: row.get(8)?,
                publish_link: row.get(9)?,
                extra_links: links,
                create_date: row.get(11)?,
                update_date: row.get(12)?,
            })
        })?;
        let mut items: Vec<DemandItem> = Vec::new();
        for item in items_iter {
            items.push(item?);
        }
        Ok(items)
    }
    pub fn get_demands_by_status(&self, status: f64) -> Result<Vec<DemandItem>> {
        let mut stmt = self.conn.prepare("SELECT * FROM DemandList WHERE status = ? ORDER BY create_date DESC").unwrap();
        let items_iter = stmt.query_map([status.to_string()], |row| {
            let links_str: String = row.get(10).unwrap();
            let links = serde_json::from_str(&links_str).unwrap();
            Ok(DemandItem {
                id: row.get(0)?,
                name: row.get(1)?,
                desc: row.get(2)?,
                status: row.get(3)?,
                publish_date: row.get(4)?,
                demand_link: row.get(5)?,
                ui_link: row.get(6)?,
                api_link: row.get(7)?,
                code_path: row.get(8)?,
                publish_link: row.get(9)?,
                extra_links: links,
                create_date: row.get(11)?,
                update_date: row.get(12)?,
            })
        })?;
        let mut items: Vec<DemandItem> = Vec::new();
        for item in items_iter {
            items.push(item?);
        }
        Ok(items)
    }
    pub fn update_demand(&self, item: DemandItem) -> bool {
        let now = Local::now().timestamp();
        let DemandItem {
            id,
            name,
            desc,
            publish_date,
            demand_link,
            ui_link,
            api_link,
            code_path,
            publish_link,
            extra_links,
            ..
        } = item;
        let links = serde_json::to_string(&extra_links).unwrap();
        match self.conn.execute(
            "UPDATE DemandList SET name = ?, desc = ?, publish_date = ?, demand_link = ?, ui_link = ?, api_link = ?, code_path = ?, publish_link = ?, extra_links = ?, update_date = ? WHERE id = ?",
            [name.to_string(), desc.to_string(), publish_date.to_string(), demand_link.to_string(), ui_link.to_string(), api_link.to_string(), code_path.to_string(), publish_link.to_string(), links, now.to_string(), id],
        ) {
            Ok(..) => true,
            Err(err) => {
                println!("some error: {}", err);
                false
            }
        }
    }
    pub fn update_status(&self, id: String, status: f64) -> bool {
        let now = Local::now().timestamp();
        match self.conn.execute(
            "UPDATE DemandList SET status = ?, update_date = ? WHERE id = ?",
            [status.to_string(), now.to_string(), id],
        ) {
            Ok(..) => true,
            Err(err) => {
                println!("some error: {}", err);
                false
            }
        }
    }
}
