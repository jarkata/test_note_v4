use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use crate::business::NoteManager;

/// 全局笔记管理器实例
static mut NOTE_MANAGER: Option<NoteManager> = None;

/// 初始化笔记管理器
/// 
/// # Safety
/// 必须在应用启动时调用一次
#[no_mangle]
pub extern "C" fn note_manager_init() {
    unsafe {
        NOTE_MANAGER = Some(NoteManager::new());
    }
}

/// 创建笔记 (FFI)
/// 
/// # Safety
/// - `title` 必须是有效的 UTF-8 C 字符串
/// - `content` 必须是有效的 UTF-8 C 字符串
/// - 返回的字符串指针必须通过 `free_cstring` 释放
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
/// 
/// # Returns
/// JSON 数组格式的笔记列表。返回的字符串指针必须通过 `free_cstring` 释放
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

/// 根据 ID 获取单个笔记 (FFI)
/// 
/// # Safety
/// - `id` 必须是有效的 UTF-8 C 字符串
/// - 返回的字符串指针必须通过 `free_cstring` 释放
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
/// 
/// # Safety
/// - `id`, `title`, `content` 必须是有效的 UTF-8 C 字符串
/// - 返回的字符串指针必须通过 `free_cstring` 释放
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
/// 
/// # Safety
/// - `id` 必须是有效的 UTF-8 C 字符串
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
/// 
/// # Safety
/// - `keyword` 必须是有效的 UTF-8 C 字符串
/// - 返回的字符串指针必须通过 `free_cstring` 释放
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
/// 
/// # Safety
/// - `ptr` 必须是通过 FFI 函数返回的有效指针
/// - 不能多次释放同一个指针
#[no_mangle]
pub extern "C" fn free_cstring(ptr: *mut c_char) {
    unsafe {
        if !ptr.is_null() {
            let _ = CString::from_raw(ptr);
        }
    }
}
