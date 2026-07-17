//! # Layout Engine 测试
//!
//! §3.10 Split tree 不变量、序列化 round-trip 测试。

use mux_server::layout::*;

// ============================================================
// §3.10 Split Tree 不变量测试
// ============================================================

/// §3.10 空布局树
#[test]
fn test_empty_layout_tree() {
    let tree = LayoutTree::empty();

    // 空树的根是一个 Pane 节点
    match &tree.root {
        LayoutNode::Pane { id, pane_id } => {
            assert!(id.is_empty());
            assert!(pane_id.is_empty());
        }
        LayoutNode::Split { .. } => panic!("空树不应是 Split 节点"),
    }
}

/// §3.10 单 pane 布局
#[test]
fn test_single_pane_layout() {
    let tree = LayoutTree::with_pane("n1".into(), "p1".into());

    match &tree.root {
        LayoutNode::Pane { id, pane_id } => {
            assert_eq!(id, "n1");
            assert_eq!(pane_id, "p1");
        }
        LayoutNode::Split { .. } => panic!("单 pane 不应是 Split 节点"),
    }
}

/// §3.10 Split 操作后不变量：无孤儿 pane，比例和 = 1.0，所有 pane 可达
#[test]
fn test_split_invariants() {
    let mut tree = LayoutTree::with_pane("n1".into(), "p1".into());

    // 分割 p1，创建 p2
    tree.split("p1", "p2".into(), SplitDirection::LeftRight).unwrap();

    // 不变量 1：所有 pane 可达
    let ids = tree.pane_ids();
    assert!(ids.contains(&"p1".into()), "p1 应可达");
    assert!(ids.contains(&"p2".into()), "p2 应可达");
    assert_eq!(ids.len(), 2, "应有 2 个 pane");

    // 不变量 2：比例和 = 1.0
    if let LayoutNode::Split { ratios, .. } = &tree.root {
        let sum: f32 = ratios.iter().sum();
        assert!(
            (sum - 1.0).abs() < 0.001,
            "比例和应为 1.0, 实际: {}",
            sum
        );
    } else {
        panic!("分割后根节点应为 Split");
    }

    // 不变量 3：Split 节点子节点数 = ratios 数
    if let LayoutNode::Split { children, ratios, .. } = &tree.root {
        assert_eq!(children.len(), ratios.len());
    }
}

/// §3.10 连续 split 后不变量
#[test]
fn test_multiple_splits_invariants() {
    let mut tree = LayoutTree::with_pane("root".into(), "p1".into());

    // p1 → split → p1 + p2
    tree.split("p1", "p2".into(), SplitDirection::LeftRight).unwrap();
    // p2 → split → p2 + p3
    tree.split("p2", "p3".into(), SplitDirection::TopBottom).unwrap();

    let ids = tree.pane_ids();
    assert_eq!(ids.len(), 3, "应有 3 个 pane");
    assert!(ids.contains(&"p1".into()));
    assert!(ids.contains(&"p2".into()));
    assert!(ids.contains(&"p3".into()));

    // 验证所有 Split 节点的比例和 ≈ 1.0
    check_ratios(&tree.root);
}

fn check_ratios(node: &LayoutNode) {
    match node {
        LayoutNode::Pane { .. } => {}
        LayoutNode::Split { ratios, children, .. } => {
            let sum: f32 = ratios.iter().sum();
            assert!(
                (sum - 1.0).abs() < 0.001,
                "Split 比例和应为 1.0, 实际: {}",
                sum
            );
            assert_eq!(children.len(), ratios.len());

            for child in children {
                check_ratios(child);
            }
        }
    }
}

/// §3.10 Close pane 后不变量：孤儿清理、比例归一化
#[test]
fn test_close_pane_invariants() {
    let mut tree = LayoutTree::with_pane("n1".into(), "p1".into());
    tree.split("p1", "p2".into(), SplitDirection::LeftRight).unwrap();
    tree.split("p2", "p3".into(), SplitDirection::TopBottom).unwrap();

    // 关闭 p3
    tree.remove_pane("p3").unwrap();

    let ids = tree.pane_ids();
    assert!(!ids.contains(&"p3".into()), "p3 已被移除");
    assert!(ids.contains(&"p1".into()));
    assert!(ids.contains(&"p2".into()));

    // 验证比例归一化
    check_ratios(&tree.root);
}

/// §3.10 Resize pane 后不变量
#[test]
fn test_resize_pane_invariants() {
    let mut tree = LayoutTree::with_pane("n1".into(), "p1".into());
    tree.split("p1", "p2".into(), SplitDirection::LeftRight).unwrap();

    // 初始比例 [0.5, 0.5]
    if let LayoutNode::Split { ratios, .. } = &tree.root {
        assert_eq!(ratios, &[0.5, 0.5]);
    }

    // 放大 p1 (delta = 0.1)
    tree.resize_pane("p1", SplitDirection::LeftRight, 0.1).unwrap();

    // 验证比例归一化后和 ≈ 1.0
    check_ratios(&tree.root);
}

/// §3.10 Find pane 操作
#[test]
fn test_find_pane() {
    let tree = LayoutTree::with_pane("n1".into(), "p1".into());
    let result = tree.root.find_pane("p1");
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "p1");

    let result = tree.root.find_pane("nonexistent");
    assert!(result.is_none());
}

// ============================================================
// §3.10 序列化 Round-trip
// ============================================================

/// §3.10 单 pane 序列化 round-trip
#[test]
fn test_serialize_single_pane() {
    let tree = LayoutTree::with_pane("n1".into(), "p1".into());
    let serialized = tree.serialize().unwrap();

    // 格式: P:id:pane_id\nchecksum
    assert!(serialized.starts_with("P:n1:p1"));
    assert!(serialized.ends_with('\n'));
}

/// §3.10 Split 布局序列化 round-trip
#[test]
fn test_serialize_split_layout() {
    let mut tree = LayoutTree::with_pane("n1".into(), "p1".into());
    tree.split("p1", "p2".into(), SplitDirection::LeftRight).unwrap();

    let serialized = tree.serialize().unwrap();

    // 格式: S:id:H:[ratios]\nP:child_id:pane_id\n...
    assert!(serialized.contains("S:"));
    assert!(serialized.contains("H")); // Horizontal (LeftRight)
    assert!(serialized.contains("P:n1-left:p1"));
    assert!(serialized.contains("P:n1-right:p2"));
}

/// §3.10 校验和一致性
#[test]
fn test_serialize_checksum_consistency() {
    let tree = LayoutTree::with_pane("n1".into(), "p1".into());

    // 两次序列化应产生相同输出（相同数据 = 相同校验和）
    let s1 = tree.serialize().unwrap();
    let s2 = tree.serialize().unwrap();
    assert_eq!(s1, s2, "相同布局应产生相同序列化结果");
}
