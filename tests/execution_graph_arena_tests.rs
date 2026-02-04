use graphoid::execution_graph::arena::{ArenaSet, NodeRef, ArenaId, NodeIndex};

#[test]
fn test_new_arena_returns_unique_ids() {
    let mut arenas: ArenaSet<String> = ArenaSet::new();
    let a1 = arenas.new_arena();
    let a2 = arenas.new_arena();
    assert_ne!(a1, a2);
}

#[test]
fn test_alloc_and_get() {
    let mut arenas: ArenaSet<String> = ArenaSet::new();
    let arena_id = arenas.new_arena();
    let idx = arenas.alloc(arena_id, "hello".to_string());
    let node_ref = NodeRef::new(arena_id, idx);
    assert_eq!(arenas.get(node_ref), Some(&"hello".to_string()));
}

#[test]
fn test_alloc_multiple_items() {
    let mut arenas: ArenaSet<i32> = ArenaSet::new();
    let arena_id = arenas.new_arena();
    let i0 = arenas.alloc(arena_id, 10);
    let i1 = arenas.alloc(arena_id, 20);
    let i2 = arenas.alloc(arena_id, 30);

    assert_eq!(arenas.get(NodeRef::new(arena_id, i0)), Some(&10));
    assert_eq!(arenas.get(NodeRef::new(arena_id, i1)), Some(&20));
    assert_eq!(arenas.get(NodeRef::new(arena_id, i2)), Some(&30));
}

#[test]
fn test_get_mut() {
    let mut arenas: ArenaSet<String> = ArenaSet::new();
    let arena_id = arenas.new_arena();
    let idx = arenas.alloc(arena_id, "before".to_string());
    let node_ref = NodeRef::new(arena_id, idx);

    if let Some(val) = arenas.get_mut(node_ref) {
        *val = "after".to_string();
    }
    assert_eq!(arenas.get(node_ref), Some(&"after".to_string()));
}

#[test]
fn test_multiple_arenas_independent() {
    let mut arenas: ArenaSet<i32> = ArenaSet::new();
    let a1 = arenas.new_arena();
    let a2 = arenas.new_arena();

    let idx_a1 = arenas.alloc(a1, 100);
    let idx_a2 = arenas.alloc(a2, 200);

    assert_eq!(arenas.get(NodeRef::new(a1, idx_a1)), Some(&100));
    assert_eq!(arenas.get(NodeRef::new(a2, idx_a2)), Some(&200));
}

#[test]
fn test_drop_arena() {
    let mut arenas: ArenaSet<i32> = ArenaSet::new();
    let a1 = arenas.new_arena();
    let a2 = arenas.new_arena();

    let idx_a1 = arenas.alloc(a1, 100);
    let idx_a2 = arenas.alloc(a2, 200);

    // Drop a1
    arenas.drop_arena(a1);

    // a1 lookups return None
    assert_eq!(arenas.get(NodeRef::new(a1, idx_a1)), None);

    // a2 is unaffected
    assert_eq!(arenas.get(NodeRef::new(a2, idx_a2)), Some(&200));
}

#[test]
fn test_drop_arena_allows_new_allocations_in_other_arenas() {
    let mut arenas: ArenaSet<i32> = ArenaSet::new();
    let a1 = arenas.new_arena();
    let a2 = arenas.new_arena();

    arenas.alloc(a1, 100);
    arenas.drop_arena(a1);

    // Can still allocate in a2
    let idx = arenas.alloc(a2, 300);
    assert_eq!(arenas.get(NodeRef::new(a2, idx)), Some(&300));
}

#[test]
fn test_node_ref_equality() {
    let r1 = NodeRef::new(ArenaId(1), NodeIndex(0));
    let r2 = NodeRef::new(ArenaId(1), NodeIndex(0));
    let r3 = NodeRef::new(ArenaId(1), NodeIndex(1));
    let r4 = NodeRef::new(ArenaId(2), NodeIndex(0));

    assert_eq!(r1, r2);
    assert_ne!(r1, r3);
    assert_ne!(r1, r4);
}

#[test]
fn test_node_ref_hash() {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    let r1 = NodeRef::new(ArenaId(1), NodeIndex(0));
    let r2 = NodeRef::new(ArenaId(1), NodeIndex(1));

    map.insert(r1, "first");
    map.insert(r2, "second");

    assert_eq!(map.get(&r1), Some(&"first"));
    assert_eq!(map.get(&r2), Some(&"second"));
}

#[test]
fn test_node_ref_copy() {
    let r1 = NodeRef::new(ArenaId(1), NodeIndex(0));
    let r2 = r1; // Copy
    assert_eq!(r1, r2); // r1 still valid (Copy, not Move)
}

#[test]
fn test_arena_count() {
    let mut arenas: ArenaSet<i32> = ArenaSet::new();
    assert_eq!(arenas.arena_count(), 0);

    let a1 = arenas.new_arena();
    assert_eq!(arenas.arena_count(), 1);

    let _a2 = arenas.new_arena();
    assert_eq!(arenas.arena_count(), 2);

    arenas.drop_arena(a1);
    assert_eq!(arenas.arena_count(), 1);
}

#[test]
fn test_node_count_in_arena() {
    let mut arenas: ArenaSet<i32> = ArenaSet::new();
    let a1 = arenas.new_arena();

    assert_eq!(arenas.node_count(a1), Some(0));
    arenas.alloc(a1, 10);
    assert_eq!(arenas.node_count(a1), Some(1));
    arenas.alloc(a1, 20);
    assert_eq!(arenas.node_count(a1), Some(2));

    // Dropped arena returns None
    arenas.drop_arena(a1);
    assert_eq!(arenas.node_count(a1), None);
}

#[test]
fn test_get_invalid_ref_returns_none() {
    let arenas: ArenaSet<i32> = ArenaSet::new();
    // No arenas exist
    let bad_ref = NodeRef::new(ArenaId(99), NodeIndex(0));
    assert_eq!(arenas.get(bad_ref), None);
}

#[test]
fn test_get_out_of_bounds_index_returns_none() {
    let mut arenas: ArenaSet<i32> = ArenaSet::new();
    let a1 = arenas.new_arena();
    arenas.alloc(a1, 42);
    // Index 1 doesn't exist (only 0)
    let bad_ref = NodeRef::new(a1, NodeIndex(99));
    assert_eq!(arenas.get(bad_ref), None);
}
