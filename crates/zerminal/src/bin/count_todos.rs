// 洞计数工具
// 来源: spec §8.1 — 需要统计剩余迁移洞数量
// 用法: 编译并运行此二进制文件，打印每个类别的洞数和总数

use zerminal_macros::zerminal_todo;
use zerminal_macros_types::ZerminalTodo;

#[zerminal_todo("stub", "count_todos 占位，确保 Pass 1 期间 count_todos > 0")]
pub struct __CountTodosMarker;

fn main() {
    let _ = __CountTodosMarker;
    let todos: Vec<_> = inventory::iter::<ZerminalTodo>().collect();

    if todos.is_empty() {
        println!("zerminal: 没有剩余迁移洞。");
        return;
    }

    // 按类别分组统计
    let mut by_category: std::collections::BTreeMap<&str, Vec<&ZerminalTodo>> =
        std::collections::BTreeMap::new();
    for todo in &todos {
        by_category
            .entry(todo.category)
            .or_default()
            .push(todo);
    }

    for (category, items) in &by_category {
        eprintln!("  {}: {} 个洞", category, items.len());
    }
    eprintln!("总计: {} 个剩余迁移洞", todos.len());
}
