#!/usr/bin/env python3
"""删除一批 crate 并清理 Cargo.toml / use 引用。

用法:
    python3 remove_batch.py crate1 crate2 ...

操作:
1. 删除 crates/<crate> 目录（如果存在）。
2. 从根 Cargo.toml workspace.members 中移除对应条目。
3. 从根 Cargo.toml [workspace.dependencies.<crate>] 中移除对应块。
4. 从所有 crates/*/Cargo.toml 的 [dependencies]/[dev-dependencies]/[build-dependencies]
   中移除以该 crate 开头的依赖行。
5. 在所有保留 crate 的 .rs 文件中，注释掉形如 `use crate_name::...;`、
   `use crate_name::{...};`、`pub use crate_name::...;`、`extern crate crate_name;`
   的导入语句。

注意: 这不会删除 Rust 代码中对已删除 crate 符号的深层使用，那些会作为
`broken-ref` 在 Plan 7 中处理。
"""

import os
import re
import sys
import shutil
from pathlib import Path

ROOT = Path(os.environ.get('ROOT', os.path.expanduser('~/Documents/zerminal')))
CRATES_DIR = ROOT / 'crates'
WORKSPACE_TOML = ROOT / 'Cargo.toml'

def read_text(path):
    return path.read_text(encoding='utf-8')

def write_text(path, content):
    path.write_text(content, encoding='utf-8')

def remove_crate_dirs(crates):
    for c in crates:
        d = CRATES_DIR / c
        if d.exists():
            shutil.rmtree(d)
            print(f'  删除目录 {d.relative_to(ROOT)}')

def edit_workspace_toml(crates):
    content = read_text(WORKSPACE_TOML)
    original = content
    for c in crates:
        # members 列表条目，例如 "crates/anthropic",
        content = re.sub(rf'\s*"crates/{re.escape(c)}",', '', content)
        # [workspace.dependencies.<crate>] 块（含后续一行 path = ...）
        content = re.sub(
            rf'\n\[workspace\.dependencies\.{re.escape(c)}\]\n[^\n]*\n',
            '\n',
            content
        )
    if content != original:
        write_text(WORKSPACE_TOML, content)
        print(f'  更新 {WORKSPACE_TOML.relative_to(ROOT)}')

def edit_crate_tomls(crates):
    # 需要保留的 crate 集合（所有在 crates/ 下且不在本次删除列表中的目录）
    deleted_set = set(crates)
    for d in sorted(CRATES_DIR.iterdir()):
        if not d.is_dir() or d.name in deleted_set:
            continue
        cargo = d / 'Cargo.toml'
        if not cargo.exists():
            continue
        content = read_text(cargo)
        original = content
        for c in crates:
            # 依赖键形式:
            #   crate = { ... }
            #   crate.workspace = true
            # 也允许前面有空格。
            content = re.sub(rf'^[ \t]*{re.escape(c)}(?:\s*=|\.workspace\s*=).*\n', '', content, flags=re.MULTILINE)
        if content != original:
            write_text(cargo, content)
            print(f'  更新 {cargo.relative_to(ROOT)}')

def comment_imports(crates):
    # 只在保留 crate 中操作
    deleted_set = set(crates)
    patterns = []
    for c in crates:
        # 匹配:
        #   use crate::...;
        #   pub use crate::...;
        #   pub(crate) use crate::...;
        #   extern crate crate;
        # 允许前面有空白，简单处理单行导入。
        patterns.append(rf'^[ \t]*((?:pub\s+)?(?:\(crate\)\s+)?use\s+{re.escape(c)}(::|\s+as\s+))[^;]*;')
        patterns.append(rf'^[ \t]*extern\s+crate\s+{re.escape(c)}\s*;')
    combined = re.compile('|'.join(f'({p})' for p in patterns), re.MULTILINE)
    # 仅匹配首个 use/extern 模式的分组索引需要动态确定；这里简单逐个 crate 处理。
    for d in sorted(CRATES_DIR.iterdir()):
        if not d.is_dir() or d.name in deleted_set:
            continue
        for rs in d.rglob('*.rs'):
            content = read_text(rs)
            original = content
            for c in crates:
                # use 语句（单行）
                content = re.sub(
                    rf'^[ \t]*((?:pub\s+)?(?:\(crate\)\s+)?use\s+{re.escape(c)}(?:::[^;]*|(?:\s+as\s+\w+))\s*;)',
                    r'// \1  // removed-crate: ' + c,
                    content,
                    flags=re.MULTILINE
                )
                # extern crate
                content = re.sub(
                    rf'^[ \t]*(extern\s+crate\s+{re.escape(c)}\s*;)',
                    r'// \1  // removed-crate: ' + c,
                    content,
                    flags=re.MULTILINE
                )
            if content != original:
                write_text(rs, content)
                print(f'  注释 {rs.relative_to(ROOT)}')

def main():
    crates = sys.argv[1:]
    if not crates:
        print('请提供要删除的 crate 名称')
        sys.exit(1)
    print(f'处理批次: {crates}')
    remove_crate_dirs(crates)
    edit_workspace_toml(crates)
    edit_crate_tomls(crates)
    comment_imports(crates)
    print('完成。')

if __name__ == '__main__':
    main()
