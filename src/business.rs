use serde::{Deserialize, Serialize};

/// 笔记数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

/// 笔记管理器 - 核心业务逻辑
pub struct NoteManager {
    notes: Vec<Note>,
}

impl NoteManager {
    /// 创建新的笔记管理器
    pub fn new() -> Self {
        NoteManager {
            notes: Vec::new(),
        }
    }

    /// 创建笔记
    pub fn create_note(&mut self, title: String, content: String) -> Note {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Local::now().to_rfc3339();
        
        let note = Note {
            id: id.clone(),
            title,
            content,
            created_at: now.clone(),
            updated_at: now,
        };
        
        self.notes.push(note.clone());
        note
    }

    /// 获取所有笔记
    pub fn get_all_notes(&self) -> Vec<Note> {
        self.notes.clone()
    }

    /// 根据 ID 获取笔记
    pub fn get_note(&self, id: &str) -> Option<Note> {
        self.notes.iter().find(|n| n.id == id).cloned()
    }

    /// 更新笔记
    pub fn update_note(&mut self, id: String, title: String, content: String) -> Option<Note> {
        if let Some(note) = self.notes.iter_mut().find(|n| n.id == id) {
            note.title = title;
            note.content = content;
            note.updated_at = chrono::Local::now().to_rfc3339();
            return Some(note.clone());
        }
        None
    }

    /// 删除笔记
    pub fn delete_note(&mut self, id: &str) -> bool {
        if let Some(pos) = self.notes.iter().position(|n| n.id == id) {
            self.notes.remove(pos);
            return true;
        }
        false
    }

    /// 搜索笔记
    pub fn search_notes(&self, keyword: &str) -> Vec<Note> {
        self.notes
            .iter()
            .filter(|n| {
                n.title.contains(keyword) || n.content.contains(keyword)
            })
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_note() {
        let mut manager = NoteManager::new();
        let note = manager.create_note("Test".to_string(), "Content".to_string());
        assert_eq!(note.title, "Test");
        assert_eq!(note.content, "Content");
    }

    #[test]
    fn test_search_notes() {
        let mut manager = NoteManager::new();
        manager.create_note("Hello World".to_string(), "Test content".to_string());
        manager.create_note("Goodbye".to_string(), "Test content".to_string());
        
        let results = manager.search_notes("Hello");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_delete_note() {
        let mut manager = NoteManager::new();
        let note = manager.create_note("Test".to_string(), "Content".to_string());
        assert!(manager.delete_note(&note.id));
        assert_eq!(manager.get_all_notes().len(), 0);
    }
}
