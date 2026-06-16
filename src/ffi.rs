use serde::{Deserialize, Serialize};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// FFI 请求类型枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", content = "params")]
pub enum FfiRequest {
    /// 初始化管理器
    Init,
    /// 创建笔记
    CreateNote { title: String, content: String },
    /// 获取所有笔记
    GetAllNotes,
    /// 获取单个笔记
    GetNote { id: String },
    /// 更新笔记
    UpdateNote { id: String, title: String, content: String },
    /// 删除笔记
    DeleteNote { id: String },
    /// 搜索笔记
    SearchNotes { keyword: String },
}

/// FFI 响应类型枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FfiResponse {
    #[serde(rename = "success")]
    Success(serde_json::Value),
    #[serde(rename = "error")]
    Error(String),
}

use crate::business::NoteManager;

/// 全局笔记管理器实例
static mut NOTE_MANAGER: Option<NoteManager> = None;

/// 通用 FFI 路由处理器
/// 
/// # Arguments
/// * `request_json` - JSON 格式的请求字符串
/// 
/// # Returns
/// JSON 格式的响应字符串，必须通过 `free_cstring` 释放
/// 
/// # Safety
/// - `request_json` 必须是有效的 UTF-8 C 字符串
/// - 返回的字符串指针必须通过 `free_cstring` 释放
/// 
/// # Example Request JSON
/// ```json
/// {"action":"Init"}
/// {"action":"CreateNote","params":{"title":"My Note","content":"Content..."}}
/// {"action":"GetAllNotes"}
/// {"action":"SearchNotes","params":{"keyword":"keyword"}}
/// ```
#[no_mangle]
pub extern "C" fn ffi_dispatch(request_json: *const c_char) -> *const c_char {
    unsafe {
        let request_str = match CStr::from_ptr(request_json).to_str() {
            Ok(s) => s,
            Err(_) => {
                return create_error_response("Invalid UTF-8 in request");
            }
        };

        let request: FfiRequest = match serde_json::from_str(request_str) {
            Ok(req) => req,
            Err(e) => {
                return create_error_response(&format!("JSON parse error: {}", e));
            }
        };

        let response = handle_request(request);
        let response_json = serde_json::to_string(&response).unwrap_or_default();
        let c_string = CString::new(response_json).unwrap();
        return c_string.into_raw();
    }
}

/// 处理具体的 FFI 请求
fn handle_request(request: FfiRequest) -> FfiResponse {
    unsafe {
        match request {
            FfiRequest::Init => {
                NOTE_MANAGER = Some(NoteManager::new());
                FfiResponse::Success(serde_json::json!({"status": "initialized"}))
            }

            FfiRequest::CreateNote { title, content } => {
                if let Some(manager) = &mut NOTE_MANAGER {
                    let note = manager.create_note(title, content);
                    FfiResponse::Success(serde_json::to_value(note).unwrap())
                } else {
                    FfiResponse::Error("NoteManager not initialized".to_string())
                }
            }

            FfiRequest::GetAllNotes => {
                if let Some(manager) = &NOTE_MANAGER {
                    let notes = manager.get_all_notes();
                    FfiResponse::Success(serde_json::to_value(notes).unwrap())
                } else {
                    FfiResponse::Error("NoteManager not initialized".to_string())
                }
            }

            FfiRequest::GetNote { id } => {
                if let Some(manager) = &NOTE_MANAGER {
                    if let Some(note) = manager.get_note(&id) {
                        FfiResponse::Success(serde_json::to_value(note).unwrap())
                    } else {
                        FfiResponse::Error(format!("Note not found: {}", id))
                    }
                } else {
                    FfiResponse::Error("NoteManager not initialized".to_string())
                }
            }

            FfiRequest::UpdateNote { id, title, content } => {
                if let Some(manager) = &mut NOTE_MANAGER {
                    if let Some(note) = manager.update_note(id, title, content) {
                        FfiResponse::Success(serde_json::to_value(note).unwrap())
                    } else {
                        FfiResponse::Error("Note update failed".to_string())
                    }
                } else {
                    FfiResponse::Error("NoteManager not initialized".to_string())
                }
            }

            FfiRequest::DeleteNote { id } => {
                if let Some(manager) = &mut NOTE_MANAGER {
                    if manager.delete_note(&id) {
                        FfiResponse::Success(serde_json::json!({"deleted": id}))
                    } else {
                        FfiResponse::Error(format!("Failed to delete note: {}", id))
                    }
                } else {
                    FfiResponse::Error("NoteManager not initialized".to_string())
                }
            }

            FfiRequest::SearchNotes { keyword } => {
                if let Some(manager) = &NOTE_MANAGER {
                    let results = manager.search_notes(&keyword);
                    FfiResponse::Success(serde_json::to_value(results).unwrap())
                } else {
                    FfiResponse::Error("NoteManager not initialized".to_string())
                }
            }
        }
    }
}

/// 创建错误响应
fn create_error_response(error: &str) -> *const c_char {
    let response = FfiResponse::Error(error.to_string());
    let json = serde_json::to_string(&response).unwrap_or_default();
    let c_string = CString::new(json).unwrap();
    c_string.into_raw()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ffi_request_serialization() {
        let request = FfiRequest::CreateNote {
            title: "Test".to_string(),
            content: "Content".to_string(),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("CreateNote"));
    }

    #[test]
    fn test_ffi_response_serialization() {
        let response = FfiResponse::Success(serde_json::json!({"key": "value"}));
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("success"));
    }

    #[test]
    fn test_ffi_error_response() {
        let response = FfiResponse::Error("Test error".to_string());
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("error"));
    }
}
