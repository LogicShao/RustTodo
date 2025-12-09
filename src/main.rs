mod todo;

use std::env;
use todo::TodoList;
use std::io::{self, Write};

const TODO_FILE: &str = "todos.json";

fn main() {
    let args: Vec<String> = env::args().collect();

    // 如果没有参数，运行交互式模式
    if args.len() < 2 {
        run_interactive_mode();
        return;
    }

    // 命令行模式（用于 Electron 调用）
    let command = &args[1];
    let mut todo_list = TodoList::load_from_file(TODO_FILE).unwrap_or_else(|_| TodoList::new());

    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                eprintln!("错误: 请提供任务标题");
                std::process::exit(1);
            }
            let title = args[2..].join(" ");
            todo_list.add(title);
            if let Err(e) = todo_list.save_to_file(TODO_FILE) {
                eprintln!("保存失败: {}", e);
                std::process::exit(1);
            }
            println!("任务已添加");
        }
        "list" => {
            // 检查是否需要 JSON 格式输出
            let json_mode = args.contains(&"--json".to_string());
            if json_mode {
                // 输出 JSON 格式（供 Electron 使用）
                match serde_json::to_string_pretty(&todo_list) {
                    Ok(json) => println!("{}", json),
                    Err(e) => {
                        eprintln!("JSON 序列化失败: {}", e);
                        std::process::exit(1);
                    }
                }
            } else {
                // 普通格式输出
                todo_list.list();
            }
        }
        "complete" => {
            if args.len() < 3 {
                eprintln!("错误: 请提供任务 ID");
                std::process::exit(1);
            }
            match args[2].parse::<u32>() {
                Ok(id) => {
                    match todo_list.complete(id) {
                        Ok(_) => {
                            if let Err(e) = todo_list.save_to_file(TODO_FILE) {
                                eprintln!("保存失败: {}", e);
                                std::process::exit(1);
                            }
                            println!("任务已完成");
                        }
                        Err(e) => {
                            eprintln!("错误: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(_) => {
                    eprintln!("错误: 无效的 ID");
                    std::process::exit(1);
                }
            }
        }
        "remove" => {
            if args.len() < 3 {
                eprintln!("错误: 请提供任务 ID");
                std::process::exit(1);
            }
            match args[2].parse::<u32>() {
                Ok(id) => {
                    match todo_list.remove(id) {
                        Ok(_) => {
                            if let Err(e) = todo_list.save_to_file(TODO_FILE) {
                                eprintln!("保存失败: {}", e);
                                std::process::exit(1);
                            }
                            println!("任务已删除");
                        }
                        Err(e) => {
                            eprintln!("错误: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(_) => {
                    eprintln!("错误: 无效的 ID");
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("未知命令: {}", command);
            eprintln!("可用命令: add, list, complete, remove");
            std::process::exit(1);
        }
    }
}

// 交互式模式（原来的 REPL）
fn run_interactive_mode() {
    println!("欢迎使用 Todo CLI 任务管理器！");
    println!("输入 'help' 查看帮助\n");

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
                            if let Err(e) = todo_list.save_to_file(TODO_FILE) {
                                println!("保存失败: {}", e);
                            }
                        }
                        Err(e) => println!("出现错误：{}", e),
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
                            if let Err(e) = todo_list.save_to_file(TODO_FILE) {
                                println!("保存失败: {}", e);
                            }
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
