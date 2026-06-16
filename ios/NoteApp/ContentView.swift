import SwiftUI

struct ContentView: View {
    @State private var notes: [Note] = []
    @State private var showCreateSheet = false
    @State private var newNoteTitle = ""
    @State private var newNoteContent = ""
    @State private var searchText = ""
    
    var filteredNotes: [Note] {
        if searchText.isEmpty {
            return notes
        } else {
            return RustBridge.shared.searchNotes(keyword: searchText)
        }
    }
    
    var body: some View {
        NavigationStack {
            VStack {
                // 搜索栏
                SearchBar(text: $searchText)
                
                // 笔记列表
                if filteredNotes.isEmpty {
                    VStack(spacing: 20) {
                        Image(systemName: "note.text")
                            .font(.system(size: 60))
                            .foregroundColor(.gray)
                        Text("暂无笔记")
                            .font(.headline)
                            .foregroundColor(.gray)
                        Text("点击 + 按钮创建新笔记")
                            .font(.subheadline)
                            .foregroundColor(.gray)
                    }
                    .frame(maxWidth: .infinity, maxHeight: .infinity)
                    .background(Color(.systemGray6))
                } else {
                    List {
                        ForEach(filteredNotes) { note in
                            NavigationLink(destination: NoteDetailView(note: note, onUpdate: refreshNotes)) {
                                VStack(alignment: .leading, spacing: 8) {
                                    Text(note.title)
                                        .font(.headline)
                                        .lineLimit(1)
                                    Text(note.content)
                                        .font(.subheadline)
                                        .foregroundColor(.gray)
                                        .lineLimit(2)
                                    Text(formatDate(note.updated_at))
                                        .font(.caption)
                                        .foregroundColor(.gray)
                                }
                                .padding(.vertical, 8)
                            }
                        }
                        .onDelete(perform: deleteNotes)
                    }
                }
            }
            .navigationTitle("我的笔记")
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button(action: { showCreateSheet = true }) {
                        Image(systemName: "plus.circle.fill")
                            .font(.title2)
                    }
                }
                ToolbarItem(placement: .navigationBarLeading) {
                    EditButton()
                }
            }
            .sheet(isPresented: $showCreateSheet) {
                CreateNoteSheet(
                    isPresented: $showCreateSheet,
                    title: $newNoteTitle,
                    content: $newNoteContent,
                    onCreate: createNote
                )
            }
            .onAppear(perform: refreshNotes)
        }
    }
    
    private func createNote() {
        guard !newNoteTitle.isEmpty, !newNoteContent.isEmpty else { return }
        
        if let _ = RustBridge.shared.createNote(title: newNoteTitle, content: newNoteContent) {
            newNoteTitle = ""
            newNoteContent = ""
            showCreateSheet = false
            refreshNotes()
        }
    }
    
    private func deleteNotes(offsets: IndexSet) {
        for index in offsets {
            let note = filteredNotes[index]
            RustBridge.shared.deleteNote(id: note.id)
        }
        refreshNotes()
    }
    
    private func refreshNotes() {
        notes = RustBridge.shared.getAllNotes()
    }
    
    private func formatDate(_ dateString: String) -> String {
        let formatter = ISO8601DateFormatter()
        if let date = formatter.date(from: dateString) {
            let displayFormatter = DateFormatter()
            displayFormatter.dateStyle = .medium
            displayFormatter.timeStyle = .short
            displayFormatter.locale = Locale(identifier: "zh_CN")
            return displayFormatter.string(from: date)
        }
        return dateString
    }
}

// MARK: - SearchBar Component
struct SearchBar: View {
    @Binding var text: String
    
    var body: some View {
        HStack {
            Image(systemName: "magnifyingglass")
                .foregroundColor(.gray)
            
            TextField("搜索笔记", text: $text)
                .textFieldStyle(.roundedBorder)
            
            if !text.isEmpty {
                Button(action: { text = "" }) {
                    Image(systemName: "xmark.circle.fill")
                        .foregroundColor(.gray)
                }
            }
        }
        .padding()
    }
}

// MARK: - CreateNoteSheet Component
struct CreateNoteSheet: View {
    @Binding var isPresented: Bool
    @Binding var title: String
    @Binding var content: String
    let onCreate: () -> Void
    
    var body: some View {
        NavigationStack {
            Form {
                Section(header: Text("标题")) {
                    TextField("输入笔记标题", text: $title)
                }
                
                Section(header: Text("内容")) {
                    TextEditor(text: $content)
                        .frame(minHeight: 200)
                }
            }
            .navigationTitle("新建笔记")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("取消") {
                        isPresented = false
                    }
                }
                
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button("保存") {
                        onCreate()
                    }
                    .disabled(title.isEmpty || content.isEmpty)
                }
            }
        }
    }
}

// MARK: - NoteDetailView Component
struct NoteDetailView: View {
    let note: Note
    let onUpdate: () -> Void
    
    @State private var editTitle = ""
    @State private var editContent = ""
    @State private var isEditing = false
    @Environment(\.dismiss) var dismiss
    
    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            if isEditing {
                TextField("标题", text: $editTitle)
                    .font(.title2)
                    .fontWeight(.bold)
                
                TextEditor(text: $editContent)
                    .frame(minHeight: 300)
                    .border(Color.gray.opacity(0.2))
            } else {
                Text(note.title)
                    .font(.title2)
                    .fontWeight(.bold)
                
                ScrollView {
                    Text(note.content)
                        .frame(maxWidth: .infinity, alignment: .leading)
                }
                
                VStack(alignment: .leading, spacing: 8) {
                    Text("创建时间: \(formatDate(note.created_at))")
                        .font(.caption)
                        .foregroundColor(.gray)
                    Text("更新时间: \(formatDate(note.updated_at))")
                        .font(.caption)
                        .foregroundColor(.gray)
                }
            }
            
            Spacer()
        }
        .padding()
        .navigationTitle(isEditing ? "编辑笔记" : "查看笔记")
        .navigationBarTitleDisplayMode(.inline)
        .toolbar {
            ToolbarItem(placement: .navigationBarTrailing) {
                if isEditing {
                    Button("保存") {
                        RustBridge.shared.updateNote(id: note.id, title: editTitle, content: editContent)
                        isEditing = false
                        onUpdate()
                        dismiss()
                    }
                } else {
                    Button("编辑") {
                        editTitle = note.title
                        editContent = note.content
                        isEditing = true
                    }
                }
            }
        }
        .onAppear {
            editTitle = note.title
            editContent = note.content
        }
    }
    
    private func formatDate(_ dateString: String) -> String {
        let formatter = ISO8601DateFormatter()
        if let date = formatter.date(from: dateString) {
            let displayFormatter = DateFormatter()
            displayFormatter.dateStyle = .medium
            displayFormatter.timeStyle = .short
            displayFormatter.locale = Locale(identifier: "zh_CN")
            return displayFormatter.string(from: date)
        }
        return dateString
    }
}

#Preview {
    ContentView()
}
