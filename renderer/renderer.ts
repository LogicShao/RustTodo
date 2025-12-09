/**
 * 渲染进程（前端）
 * 负责：
 * 1. 用户界面交互
 * 2. 通过 IPC 与主进程通信
 * 3. 更新 UI
 */

import { ipcRenderer } from 'electron';

// 定义 Todo 接口
interface Todo {
  id: number;
  title: string;
  completed: boolean;
}

// DOM 元素
const todoInput = document.getElementById('todoInput') as HTMLInputElement;
const addBtn = document.getElementById('addBtn') as HTMLButtonElement;
const todoList = document.getElementById('todoList') as HTMLDivElement;
const totalCount = document.getElementById('totalCount') as HTMLElement;
const activeCount = document.getElementById('activeCount') as HTMLElement;
const completedCount = document.getElementById('completedCount') as HTMLElement;

// 存储当前任务列表
let todos: Todo[] = [];

/**
 * 从主进程加载任务列表
 */
async function loadTodos() {
  const result = await ipcRenderer.invoke('get-todos');
  if (result.success) {
    todos = result.data;
    renderTodos();
  } else {
    alert('加载任务失败: ' + result.error);
  }
}

/**
 * 渲染任务列表
 */
function renderTodos() {
  // 清空列表
  todoList.innerHTML = '';

  // 如果没有任务
  if (todos.length === 0) {
    todoList.innerHTML = '<div class="empty">暂无任务</div>';
    updateStats();
    return;
  }

  // 渲染每个任务
  todos.forEach((todo) => {
    const todoItem = document.createElement('div');
    todoItem.className = `todo-item ${todo.completed ? 'completed' : ''}`;

    todoItem.innerHTML = `
      <input
        type="checkbox"
        ${todo.completed ? 'checked' : ''}
        data-id="${todo.id}"
        class="todo-checkbox"
      >
      <span class="todo-title">${escapeHtml(todo.title)}</span>
      <button class="delete-btn" data-id="${todo.id}">删除</button>
    `;

    todoList.appendChild(todoItem);
  });

  // 绑定事件
  bindEvents();
  updateStats();
}

/**
 * 绑定事件监听器
 */
function bindEvents() {
  // 复选框切换
  const checkboxes = document.querySelectorAll('.todo-checkbox');
  checkboxes.forEach((checkbox) => {
    checkbox.addEventListener('change', async (e) => {
      const target = e.target as HTMLInputElement;
      const id = parseInt(target.dataset.id!);

      if (target.checked) {
        await completeTodo(id);
      }
    });
  });

  // 删除按钮
  const deleteButtons = document.querySelectorAll('.delete-btn');
  deleteButtons.forEach((btn) => {
    btn.addEventListener('click', async (e) => {
      const target = e.target as HTMLButtonElement;
      const id = parseInt(target.dataset.id!);
      await removeTodo(id);
    });
  });
}

/**
 * 添加新任务
 */
async function addTodo() {
  const title = todoInput.value.trim();

  if (!title) {
    alert('请输入任务内容');
    return;
  }

  const result = await ipcRenderer.invoke('add-todo', title);

  if (result.success) {
    todoInput.value = '';
    await loadTodos(); // 重新加载列表
  } else {
    alert('添加失败: ' + result.error);
  }
}

/**
 * 完成任务
 */
async function completeTodo(id: number) {
  const result = await ipcRenderer.invoke('complete-todo', id);

  if (result.success) {
    await loadTodos();
  } else {
    alert('操作失败: ' + result.error);
  }
}

/**
 * 删除任务
 */
async function removeTodo(id: number) {
  if (!confirm('确定要删除这个任务吗？')) {
    return;
  }

  const result = await ipcRenderer.invoke('remove-todo', id);

  if (result.success) {
    await loadTodos();
  } else {
    alert('删除失败: ' + result.error);
  }
}

/**
 * 更新统计信息
 */
function updateStats() {
  const total = todos.length;
  const completed = todos.filter(t => t.completed).length;
  const active = total - completed;

  totalCount.textContent = total.toString();
  activeCount.textContent = active.toString();
  completedCount.textContent = completed.toString();
}

/**
 * HTML 转义（防止 XSS）
 */
function escapeHtml(text: string): string {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
}

// 事件监听
addBtn.addEventListener('click', addTodo);
todoInput.addEventListener('keypress', (e) => {
  if (e.key === 'Enter') {
    addTodo();
  }
});

// 初始加载
loadTodos();
