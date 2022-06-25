pub mod sortitionSumTree;

include!("sortitionSumTree.rs");
#[test]
fn BuildTreeTest() {
    let mut trees: SortitionSumTrees = SortitionSumTrees {
        sortition_sum_trees: HashMap::new(),
    };
    trees.create_tree(1, 2);
    trees.set(1, 25, 1);
    trees.set(1, 25, 2);
    trees.set(1, 25, 3);
    trees.set(1, 25, 4);
    let tree = &trees.sortition_sum_trees.get(&1);
    assert_eq!(tree.unwrap().nodes[0], 100);
    assert_eq!(tree.unwrap().nodes[1], 50);
    assert_eq!(tree.unwrap().nodes[2], 50);
    assert_eq!(tree.unwrap().nodes[3], 25);
    assert_eq!(tree.unwrap().nodes[4], 25);
    assert_eq!(tree.unwrap().nodes[5], 25);
    assert_eq!(tree.unwrap().nodes[6], 25);
}
#[test]
fn removeAndAddNodeTest() {
    let mut trees: SortitionSumTrees = SortitionSumTrees {
        sortition_sum_trees: HashMap::new(),
    };
    trees.create_tree(1, 2);
    trees.set(1, 25, 1);
    trees.set(1, 25, 2);
    trees.set(1, 25, 3);
    trees.set(1, 25, 4);
    let index = &trees.sortition_sum_trees.get(&1).unwrap().ids_to_node_indexes[&3].clone();
    trees.set(1, 0, 3);
    let tree = &trees.sortition_sum_trees.get(&1);
    assert_eq!(trees.stake_of(1, 3), 0);
    assert_eq!(&trees.sortition_sum_trees.get(&1).unwrap().stack[0], index);
    trees.set(1, 25, 5);
    let stack0 = &trees.sortition_sum_trees.get(&1).unwrap().stack.get(0);
    match stack0 {
        Some(s) => assert!(false, "Error, stack is not empty!"),
        None => assert!(true),
    }
}
#[test]
fn drawTest() {
    let mut trees: SortitionSumTrees = SortitionSumTrees {
        sortition_sum_trees: HashMap::new(),
    };
    trees.create_tree(1, 2);
    trees.set(1, 25, 1);
    trees.set(1, 25, 2);
    trees.set(1, 25, 3);
    trees.set(1, 25, 4);
    let addr = trees.draw(1, 20);
    assert_eq!(3, addr);
    let addr = trees.draw(1, 40);
    assert_eq!(1, addr);
    let addr = trees.draw(1, 60);
    assert_eq!(4, addr);
    let addr = trees.draw(1, 80);
    assert_eq!(2, addr);
}
fn main() {}
