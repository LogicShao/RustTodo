/**
 * Electron 主进程
 * 负责：
 * 1. 创建和管理窗口
 * 2. 处理 IPC 通信
 * 3. 调用 Rust 后端
 */

import { app, BrowserWindow, ipcMain } from 'electron';
import * as path from 'path';
import { spawn } from 'child_process';

// 定义 Todo 类型（与 Rust 端保持一致）
interface Todo {
  id: number;
  title: string;
  completed: boolean;
}

interface TodoList {
  todos: Todo[];
}

let mainWindow: BrowserWindow | null = null;

/**
 * 创建主窗口
 */
function createWindow() {
  mainWindow = new BrowserWindow({
    width: 800,
    height: 600,
    webPreferences: {
      // 启用 Node.js 集成（开发模式，生产环境建议使用 preload）
      nodeIntegration: true,
      contextIsolation: false,
    },
  });

  // 加载 HTML 文件
  mainWindow.loadFile(path.join(__dirname, '../renderer/index.html'));

  // 打开开发者工具（可选）
  // mainWindow.webContents.openDevTools();

  mainWindow.on('closed', () => {
    mainWindow = null;
  });
}

/**
 * 调用 Rust CLI 执行命令
 * @param args 命令参数
 * @returns Promise<string> 返回输出结果
 */
function callRustCLI(args: string[]): Promise<string> {
  return new Promise((resolve, reject) => {
    // Rust 可执行文件路径
    const rustExePath = path.join(
      __dirname,
      '..',
      'target',
      'release',
      process.platform === 'win32' ? 'hello_rust.exe' : 'hello_rust'
    );

    const rustProcess = spawn(rustExePath, args);
    let output = '';
    let errorOutput = '';

    rustProcess.stdout.on('data', (data) => {
      output += data.toString();
    });

    rustProcess.stderr.on('data', (data) => {
      errorOutput += data.toString();
    });

    rustProcess.on('close', (code) => {
      if (code === 0) {
        resolve(output);
      } else {
        reject(new Error(`Rust process exited with code ${code}: ${errorOutput}`));
      }
    });
  });
}

/**
 * 设置 IPC 处理器
 */
function setupIPC() {
  // 获取所有任务
  ipcMain.handle('get-todos', async () => {
    try {
      // 这里需要修改 Rust 端，添加 JSON 输出模式
      const output = await callRustCLI(['list', '--json']);
      const todoList: TodoList = JSON.parse(output);
      return { success: true, data: todoList.todos };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  // 添加任务
  ipcMain.handle('add-todo', async (_, title: string) => {
    try {
      await callRustCLI(['add', title]);
      return { success: true };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  // 完成任务
  ipcMain.handle('complete-todo', async (_, id: number) => {
    try {
      await callRustCLI(['complete', id.toString()]);
      return { success: true };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  // 删除任务
  ipcMain.handle('remove-todo', async (_, id: number) => {
    try {
      await callRustCLI(['remove', id.toString()]);
      return { success: true };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });
}

// 应用准备就绪
app.on('ready', () => {
  createWindow();
  setupIPC();
});

// 所有窗口关闭
app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

// 激活应用
app.on('activate', () => {
  if (mainWindow === null) {
    createWindow();
  }
});
