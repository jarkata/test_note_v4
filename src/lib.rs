use serde::{Deserialize, Serialize};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

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

// ============ FFI 接口 - 用于 iOS 调用 ============

static mut NOTE_MANAGER: Option<NoteManager> = None;

/// 初始化笔记管理器
#[no_mangle]
pub extern "C" fn note_manager_init() {
    unsafe {
        NOTE_MANAGER = Some(NoteManager::new());
    }
}

/// 创建笔记 (FFI)
#[no_mangle]
pub extern "C" fn create_note(
    title: *const c_char,
    content: *const c_char,
) -> *const c_char {
    unsafe {
        if let Some(manager) = &mut NOTE_MANAGER {
            let title_str = CStr::from_ptr(title).to_string_lossy().to_string();
            let content_str = CStr::from_ptr(content).to_string_lossy().to_string();
            
            let note = manager.create_note(title_str, content_str);
            let json = serde_json::to_string(&note).unwrap_or_default();
            let c_string = CString::new(json).unwrap();
            return c_string.into_raw();
        }
    }
    std::ptr::null()
}

/// 获取所有笔记 (FFI)
#[no_mangle]
pub extern "C" fn get_all_notes() -> *const c_char {
    unsafe {
        if let Some(manager) = &NOTE_MANAGER {
            let notes = manager.get_all_notes();
            let json = serde_json::to_string(&notes).unwrap_or_default();
            let c_string = CString::new(json).unwrap();
            return c_string.into_raw();
        }
    }
    std::ptr::null()
}

/// 获取单个笔记 (FFI)
#[no_mangle]
pub extern "C" fn get_note(id: *const c_char) -> *const c_char {
    unsafe {
        if let Some(manager) = &NOTE_MANAGER {
            let id_str = CStr::from_ptr(id).to_string_lossy();
            if let Some(note) = manager.get_note(&id_str) {
                let json = serde_json::to_string(&note).unwrap_or_default();
                let c_string = CString::new(json).unwrap();
                return c_string.into_raw();
            }
        }
    }
    std::ptr::null()
}

/// 更新笔记 (FFI)
#[no_mangle]
pub extern "C" fn update_note(
    id: *const c_char,
    title: *const c_char,
    content: *const c_char,
) -> *const c_char {
    unsafe {
        if let Some(manager) = &mut NOTE_MANAGER {
            let id_str = CStr::from_ptr(id).to_string_lossy().to_string();
            let title_str = CStr::from_ptr(title).to_string_lossy().to_string();
            let content_str = CStr::from_ptr(content).to_string_lossy().to_string();
            
            if let Some(note) = manager.update_note(id_str, title_str, content_str) {
                let json = serde_json::to_string(&note).unwrap_or_default();
                let c_string = CString::new(json).unwrap();
                return c_string.into_raw();
            }
        }
    }
    std::ptr::null()
}

/// 删除笔记 (FFI)
#[no_mangle]
pub extern "C" fn delete_note(id: *const c_char) -> bool {
    unsafe {
        if let Some(manager) = &mut NOTE_MANAGER {
            let id_str = CStr::from_ptr(id).to_string_lossy();
            return manager.delete_note(&id_str);
        }
    }
    false
}

/// 搜索笔记 (FFI)
#[no_mangle]
pub extern "C" fn search_notes(keyword: *const c_char) -> *const c_char {
    unsafe {
        if let Some(manager) = &NOTE_MANAGER {
            let keyword_str = CStr::from_ptr(keyword).to_string_lossy();
            let results = manager.search_notes(&keyword_str);
            let json = serde_json::to_string(&results).unwrap_or_default();
            let c_string = CString::new(json).unwrap();
            return c_string.into_raw();
        }
    }
    std::ptr::null()
}

/// 释放 C 字符串内存
#[no_mangle]
pub extern "C" fn free_cstring(ptr: *mut c_char) {
    unsafe {
        if !ptr.is_null() {
            let _ = CString::from_raw(ptr);
        }
    }
}
