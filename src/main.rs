mod todo;

use todo::TodoList;
use std::io::{self, Write};

const TODO_FILE: &str = "todos.json";

fn main() {
    println!("🚀 欢迎使用 Todo CLI 任务管理器！");
    println!("输入 'help' 查看帮助\n");

    // TODO: 任务9: 在这里加载已保存的任务
    // 提示: 使用 TodoList::load_from_file(TODO_FILE)
    // 如果加载失败，打印警告信息并创建新的 TodoList
    let mut todo_list = match TodoList::load_from_file(TODO_FILE) {
        Ok(list) => {
            println!("已加载 {} 个任务", list.todos.len());
            list
        }
        Err(e) => {
            println!("加载失败: {}，创建新列表", e);
            TodoList::new()
        }
    };

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0];

        match command {
            "add" => {
                if parts.len() < 2 {
                    println!("请提供任务标题");
                    continue;
                }
                let title = parts[1..].join(" ");
                todo_list.add(title.clone());
                println!("✅ 已添加任务: {}", title);

                // TODO: 任务10: 在添加任务后自动保存
                // 提示: 调用 todo_list.save_to_file(TODO_FILE)
                // 如果保存失败，打印警告信息
                // 你的实现：
                if let Err(e) = todo_list.save_to_file(TODO_FILE) {
                    println!("保存失败: {}", e);
                }
            }
            "list" => {
                todo_list.list();
            }
            "complete" => {
                if parts.len() < 2 {
                    println!("请提供任务 ID");
                    continue;
                }
                match parts[1].parse::<u32>() {
                    Ok(id) => match todo_list.complete(id) {
                        Ok(_) => {
                            println!("任务 {} 已完成", id);
                            // TODO: 任务11: 在完成任务后自动保存（同任务10）
                        }
                        Err(e) => println!("❌ {}", e),
                    },
                    Err(_) => println!("无效的 ID"),
                }
            }
            "remove" => {
                if parts.len() < 2 {
                    println!("❌ 请提供任务 ID");
                    continue;
                }
                match parts[1].parse::<u32>() {
                    Ok(id) => match todo_list.remove(id) {
                        Ok(_) => {
                            println!("已删除任务 {}", id);
                            // TODO: 任务12: 在删除任务后自动保存（同任务10）
                        }
                        Err(e) => println!("出现错误：{}", e),
                    },
                    Err(_) => println!("无效的 ID"),
                }
            }
            "help" => {
                println!("\n 可用命令:");
                println!("  add <任务标题>     - 添加新任务");
                println!("  list              - 列出所有任务");
                println!("  complete <ID>     - 标记任务为完成");
                println!("  remove <ID>       - 删除任务");
                println!("  help              - 显示帮助");
                println!("  quit              - 退出程序\n");
            }
            "quit" | "exit" => {
                println!("再见!");
                break;
            }
            _ => {
                println!("未知命令: {}，输入 'help' 查看帮助", command);
            }
        }
    }
}
