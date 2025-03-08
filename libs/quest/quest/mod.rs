use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::{command, State};

// 任务状态枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestStatus {
    Pending,
    Running,
    Completed,
    Canceled,
    Failed,
}

// 任务结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quest {
    pub id: String,
    pub name: String,
    pub status: QuestStatus,
    pub progress: u8,
    pub created_at: i64,
}

// 任务管理器
#[derive(Debug)]
pub struct QuestManager {
    quests: Arc<Mutex<Vec<Quest>>>,
}

impl QuestManager {
    pub fn new() -> Self {
        Self {
            quests: Arc::new(Mutex::new(Vec::new())),
        }
    }

    // 添加新任务
    pub fn add_quest(&self, quest: Quest) {
        let mut quests = self.quests.lock().unwrap();
        quests.push(quest);
    }

    // 获取所有任务
    pub fn get_quests(&self) -> Vec<Quest> {
        let quests = self.quests.lock().unwrap();
        quests.clone()
    }

    // 取消任务
    pub fn cancel_quest(&self, id: &str) -> Option<Quest> {
        let mut quests = self.quests.lock().unwrap();
        if let Some(pos) = quests.iter().position(|q| q.id == id) {
            let mut quest = quests.remove(pos);
            quest.status = QuestStatus::Canceled;
            return Some(quest);
        }
        None
    }
}

// Tauri 命令
#[command]
pub async fn create_quest(name: String, manager: State<'_, QuestManager>) -> Result<Quest, String> {
    let quest = Quest {
        id: uuid::Uuid::new_v4().to_string(),
        name,
        status: QuestStatus::Pending,
        progress: 0,
        created_at: chrono::Local::now().timestamp(),
    };

    manager.add_quest(quest.clone());
    Ok(quest)
}

///// 创建任务 前端如何使用
// const quest = await invoke('create_quest', { name: '下载文件' });
// // 获取列表
// const quests = await invoke('list_quests');
// // 取消任务
// await invoke('cancel_quest', { id: quest.id });
///

#[command]
pub async fn list_quests(manager: State<'_, QuestManager>) -> Result<Vec<Quest>, String> {
    Ok(manager.get_quests())
}

#[command]
pub async fn cancel_quest(id: String, manager: State<'_, QuestManager>) -> Result<Option<Quest>, String> {
    Ok(manager.cancel_quest(&id))
}

// 初始化模块
pub fn init_quest_module<R: tauri::Runtime>(manager: &Arc<Mutex<QuestManager>>, app: &tauri::App<R>) {
    app.manage(manager.lock().unwrap().clone());
}
