use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;

// TODO: 这是你需要完成的第一个任务！
// 任务1: 定义 Todo 结构体
// 提示: 一个 Todo 应该包含以下字段：
// - id: u32 类型，任务的唯一标识
// - title: String 类型，任务的标题
// - completed: bool 类型，任务是否完成
//
// 请在下面定义这个结构体，并添加 #[derive(Serialize, Deserialize, Debug, Clone)]

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    // 在这里添加你的字段
    // 例如: pub id: u32,
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

// TODO: 任务2: 为 Todo 实现构造函数
// 请实现一个 new 函数，它接收 id 和 title，返回一个新的 Todo
// completed 字段默认为 false
// id 从 1 开始编号
impl Todo {
    pub fn new(id: u32, title: String) -> Self {
        Todo {
            id,
            title,
            completed: false,
        }
    }
}

// 这个结构体用来管理所有的 Todo
#[derive(Serialize, Deserialize, Debug)]
pub struct TodoList {
    pub todos: Vec<Todo>,
}

impl TodoList {
    // 创建一个新的空 TodoList
    pub fn new() -> Self {
        TodoList { todos: Vec::new() }
    }

    // TODO: 任务3: 实现添加任务的功能
    // 提示:
    // 1. 生成新的 id（可以用 self.todos.len() + 1）
    // 2. 创建新的 Todo
    // 3. 将它添加到 self.todos 中
    pub fn add(&mut self, title: String) {
        self.todos.push(Todo::new((self.todos.len() + 1) as u32, title));
    }

    // 列出所有任务
    pub fn list(&self) {
        if self.todos.is_empty() {
            println!("📭 暂无任务");
            return;
        }

        println!("\n📝 任务列表:");
        println!("-----------------------------------");
        for todo in &self.todos {
            let status = if todo.completed { "completed" } else { "uncompleted" };
            // TODO: 任务4: 完成这行打印语句
            // 提示: 打印格式应该是: "status [id] title"
            // println!("你的代码");
            println!("{} [{}] {}", status, todo.id, todo.title);
        }
        println!("-----------------------------------\n");
    }

    // TODO: 任务5: 实现标记完成的功能
    // 提示:
    // 1. 使用 iter_mut() 遍历 todos
    // 2. 找到 id 匹配的 todo
    // 3. 将它的 completed 设为 true
    pub fn complete(&mut self, id: u32) -> Result<(), String> {
        // 修复版本：使用 iter_mut() 遍历，通过 id 字段匹配（而不是索引）
        for todo in self.todos.iter_mut() {
            if todo.id == id {
                todo.completed = true;
                return Ok(());
            }
        }
        // 如果循环结束都没找到，说明任务不存在
        Err("任务不存在".to_string())
    }

    // TODO: 任务6: 实现删除任务的功能
    // 提示:
    // 1. 使用 iter().position() 找到要删除的任务索引
    // 2. 使用 remove() 删除它
    pub fn remove(&mut self, id: u32) -> Result<(), String> {
        // 修复版本：使用 position() 找到 id 匹配的索引位置
        match self.todos.iter().position(|todo| todo.id == id) {
            Some(index) => {
                self.todos.remove(index);
                Ok(())
            }
            None => Err("任务不存在".to_string()),
        }
    }

    // ==================== 第三阶段：文件持久化 ====================

    // TODO: 任务7: 实现保存到文件的功能
    // 提示:
    // 1. 使用 serde_json::to_string_pretty() 将 self 转换为 JSON 字符串
    // 2. 使用 fs::write() 将字符串写入文件
    // 3. 返回 Result<(), io::Error>
    pub fn save_to_file(&self, filename: &str) -> Result<(), io::Error> {
        let json = serde_json::to_string(self)?;
        fs::write(filename, json)?;
        Ok(())
    }

    // TODO: 任务8: 实现从文件加载的功能
    // 提示:
    // 1. 使用 Path::new(filename).exists() 检查文件是否存在
    // 2. 如果不存在，返回一个新的 TodoList
    // 3. 使用 fs::read_to_string() 读取文件内容
    // 4. 使用 serde_json::from_str() 将 JSON 解析为 TodoList
    pub fn load_from_file(filename: &str) -> Result<Self, io::Error> {
        if Path::new(filename).exists() {
            let data = fs::read_to_string(filename)?;
            let todo_list: TodoList = serde_json::from_str(&data)?;
            Ok(todo_list)
        } else {
            Ok(TodoList::new())
        }
    }
}
