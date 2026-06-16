use serde::{Deserialize, Serialize};

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
