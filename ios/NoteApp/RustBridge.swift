import Foundation

// MARK: - C 函数声明
@_silgen_name("note_manager_init")
func note_manager_init()

@_silgen_name("create_note")
func c_create_note(_ title: UnsafePointer<CChar>, _ content: UnsafePointer<CChar>) -> UnsafeMutablePointer<CChar>?

@_silgen_name("get_all_notes")
func c_get_all_notes() -> UnsafeMutablePointer<CChar>?

@_silgen_name("get_note")
func c_get_note(_ id: UnsafePointer<CChar>) -> UnsafeMutablePointer<CChar>?

@_silgen_name("update_note")
func c_update_note(
    _ id: UnsafePointer<CChar>,
    _ title: UnsafePointer<CChar>,
    _ content: UnsafePointer<CChar>
) -> UnsafeMutablePointer<CChar>?

@_silgen_name("delete_note")
func c_delete_note(_ id: UnsafePointer<CChar>) -> Bool

@_silgen_name("search_notes")
func c_search_notes(_ keyword: UnsafePointer<CChar>) -> UnsafeMutablePointer<CChar>?

@_silgen_name("free_cstring")
func free_cstring(_ ptr: UnsafeMutablePointer<CChar>)

// MARK: - Note 数据模型
struct Note: Codable, Identifiable {
    let id: String
    let title: String
    let content: String
    let created_at: String
    let updated_at: String
}

// MARK: - Rust 桥接管理器
class RustBridge {
    static let shared = RustBridge()
    
    private init() {
        note_manager_init()
    }
    
    // MARK: - Private Helpers
    private func cStringToString(_ cString: UnsafeMutablePointer<CChar>?) -> String? {
        guard let cString = cString else { return nil }
        let string = String(cString: cString)
        free_cstring(cString)
        return string
    }
    
    private func stringToCString(_ string: String) -> UnsafeMutablePointer<CChar> {
        return UnsafeMutablePointer(mutating: (string as NSString).utf8String!)
    }
    
    // MARK: - Public Methods
    func createNote(title: String, content: String) -> Note? {
        let titleCStr = stringToCString(title)
        let contentCStr = stringToCString(content)
        
        guard let jsonCStr = c_create_note(titleCStr, contentCStr) else { return nil }
        guard let jsonString = cStringToString(jsonCStr) else { return nil }
        
        let decoder = JSONDecoder()
        if let note = try? decoder.decode(Note.self, from: jsonString.data(using: .utf8)!) {
            return note
        }
        return nil
    }
    
    func getAllNotes() -> [Note] {
        guard let jsonCStr = c_get_all_notes() else { return [] }
        guard let jsonString = cStringToString(jsonCStr) else { return [] }
        
        let decoder = JSONDecoder()
        if let notes = try? decoder.decode([Note].self, from: jsonString.data(using: .utf8)!) {
            return notes
        }
        return []
    }
    
    func getNote(id: String) -> Note? {
        let idCStr = stringToCString(id)
        guard let jsonCStr = c_get_note(idCStr) else { return nil }
        guard let jsonString = cStringToString(jsonCStr) else { return nil }
        
        let decoder = JSONDecoder()
        if let note = try? decoder.decode(Note.self, from: jsonString.data(using: .utf8)!) {
            return note
        }
        return nil
    }
    
    func updateNote(id: String, title: String, content: String) -> Note? {
        let idCStr = stringToCString(id)
        let titleCStr = stringToCString(title)
        let contentCStr = stringToCString(content)
        
        guard let jsonCStr = c_update_note(idCStr, titleCStr, contentCStr) else { return nil }
        guard let jsonString = cStringToString(jsonCStr) else { return nil }
        
        let decoder = JSONDecoder()
        if let note = try? decoder.decode(Note.self, from: jsonString.data(using: .utf8)!) {
            return note
        }
        return nil
    }
    
    func deleteNote(id: String) -> Bool {
        let idCStr = stringToCString(id)
        return c_delete_note(idCStr)
    }
    
    func searchNotes(keyword: String) -> [Note] {
        let keywordCStr = stringToCString(keyword)
        guard let jsonCStr = c_search_notes(keywordCStr) else { return [] }
        guard let jsonString = cStringToString(jsonCStr) else { return [] }
        
        let decoder = JSONDecoder()
        if let notes = try? decoder.decode([Note].self, from: jsonString.data(using: .utf8)!) {
            return notes
        }
        return []
    }
}
