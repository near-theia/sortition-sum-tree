use std::collections::HashMap;

type TypeAddress = u128;
type TypeKey = u128;

struct SortitionSumTree {
    K: usize,
    stack: Vec<usize>,
    nodes: Vec<u128>,
    ids_to_node_indexes: HashMap<TypeAddress, usize>,
    node_indexes_to_ids: HashMap<usize, TypeAddress>,
}

impl SortitionSumTree {
    pub fn new(k: usize) -> SortitionSumTree {
        SortitionSumTree {
            K: k,
            stack: Vec::new(),
            nodes: Vec::new(),
            ids_to_node_indexes: HashMap::new(),
            node_indexes_to_ids: HashMap::new(),
        }
    }
}

struct SortitionSumTrees {
    sortition_sum_trees: HashMap<TypeKey, SortitionSumTree>,
}

impl SortitionSumTrees {
    /**
     *  @dev Create a sortition sum tree with a key.
     *  @param _key The key of the new tree.
     *  @param _k The max number of children for each node in the new tree.
     */
    pub fn create_tree(&mut self, key: TypeKey, k: usize) {
        let mut tree: SortitionSumTree = SortitionSumTree::new(k);
        tree.nodes.push(0);
        self.sortition_sum_trees.insert(key, tree);
    }

    /**
     *  @dev Update the parents of a node until root.
     *  @param _key The key of the tree to update.
     *  @param _tree_index The index of the node to start from.
     *  @param _plus_or_minus Wether to add (true) or substract (false).
     *  @param _value The value to add or substract.
     */
    pub fn update_parents(
        &mut self,
        key: TypeKey,
        tree_index: usize,
        plus_or_minus: bool,
        value: u128,
    ) {
        if let Some(tree) = self.sortition_sum_trees.get_mut(&key) {
            let mut parent_index = tree_index;
            while parent_index != 0 {
                parent_index = (parent_index - 1) / tree.K;
                tree.nodes[parent_index] = if plus_or_minus {
                    tree.nodes[parent_index] + value
                } else {
                    tree.nodes[parent_index] - value
                };
            }
        }
    }

    /**
     *  @dev Set a value of an address in a tree.
     *  @param _key The key of the tree.
     *  @param _value The new value.
     *  @param _id The ID of the value.
     *  `O(log_k(n))` where
     *  `k` is the maximum number of childs per node in the tree,
     *   and `n` is the maximum number of nodes ever appended.
     */
    pub fn set(&mut self, key: TypeKey, value: u128, id: TypeAddress) {
        if let Some(tree) = self.sortition_sum_trees.get_mut(&key) {
            if let Some(_tree_index) = tree.ids_to_node_indexes.get_mut(&id) {
                //node exist
                let tree_index = _tree_index.clone();
                if value == 0 {
                    //new value==0
                    //remove
                    let value = tree.nodes[tree_index];
                    tree.nodes[tree_index.clone()] = 0;
                    tree.stack.push(tree_index);
                    tree.node_indexes_to_ids.remove(&tree_index);
                    tree.ids_to_node_indexes.remove(&id);
                    self.update_parents(key, tree_index, false, value);
                } else if value != tree.nodes[tree_index] {
                    // New value,and!=0
                    // Set.
                    let plus_or_minus = tree.nodes[tree_index] <= value;
                    let plus_or_minus_value: u128 = if plus_or_minus {
                        value - tree.nodes[tree_index.clone()]
                    } else {
                        tree.nodes[tree_index.clone()] - value
                    };
                    tree.nodes[tree_index] = value;
                    self.update_parents(key, tree_index, plus_or_minus, plus_or_minus_value);
                }
            } else {
                if value != 0 {
                    //node not exist
                    let mut tree_index: usize = 0;
                    if tree.stack.len() == 0 {
                        //no vacant node
                        tree_index = tree.nodes.len();
                        tree.nodes.push(value);
                        if (tree_index != 1) && ((tree_index - 1) % tree.K == 0) {
                            //is the first child node.
                            //move the parent  down
                            let parent_index = tree_index / tree.K;
                            let parent_id: TypeAddress = tree.node_indexes_to_ids[&parent_index];
                            let new_index = tree_index + 1;
                            tree.nodes.push(tree.nodes[parent_index]);
                            tree.node_indexes_to_ids.remove(&parent_index);
                            tree.ids_to_node_indexes.insert(parent_id, new_index);
                            tree.node_indexes_to_ids.insert(new_index, parent_id);
                        }
                    } else {
                        //vacant node
                        tree_index = tree.stack[tree.stack.len() - 1];
                        tree.stack.pop();
                        tree.nodes[tree_index] = value;
                    }
                    tree.ids_to_node_indexes.insert(id, tree_index);
                    tree.node_indexes_to_ids.insert(tree_index, id);
                    //update_parents( _key, tree_index, true, _value);
                    self.update_parents(key, tree_index, true, value);
                }
            }
        }
    }

    /** @dev Gets a specified ID's associated value.
        *  @param _key The key of the tree.
        *  @param _id The ID of the value.
        *  @return value The associated value.
     */
    pub fn stake_of(&self, key: TypeKey, id: TypeAddress) -> u128 {
        if let Some(tree) = self.sortition_sum_trees.get(&key) {
            if let Some(tree_index) = tree.ids_to_node_indexes.get(&id) {
                return tree.nodes[*tree_index];
            }
        }
        return 0;
    }

    /**
     *  @dev Draw an ID from a tree using a number. Note that this function reverts if the sum of all values in the tree is 0.
     *  @param _key The key of the tree.
     *  @param _drawn_number The drawn number.
     *  @return ID The drawn ID.
     *  `O(k * log_k(n))` where
     *  `k` is the maximum number of childs per node in the tree,
     *   and `n` is the maximum number of nodes ever appended.
     */
    pub fn draw(&self, key: TypeKey, drawn_number: u128) -> TypeAddress {
        if let Some(tree) = self.sortition_sum_trees.get(&key) {
            let mut tree_index: usize = 0;
            let mut current_drawn_number = drawn_number % tree.nodes[0];
            while (tree.K * tree_index) + 1 < tree.nodes.len() {
                for i in 1..=tree.K {
                    let node_index = (tree.K * tree_index) + i;
                    let node_value = tree.nodes[node_index];
                    if current_drawn_number >= node_value {
                        current_drawn_number = current_drawn_number - node_value;
                    } else {
                        tree_index = node_index;
                        break;
                    }
                }
            }
            return tree.node_indexes_to_ids[&tree_index];
        }

        return 0;
    }

    /**
     *  @dev Query the leaves of a tree. Note that if `startIndex == 0`, the tree is empty and the root node will be returned.
     *  @param key The key of the tree to get the leaves from.
     *  @param cursor The pagination cursor.
     *  @param count The number of items to return.
     *  @return startIndex The index at which leaves start.
     *  @return values The values of the returned leaves.
     *  @return hasMore Whether there are more for pagination.
     *  `O(n)` where
     *  `n` is the maximum number of nodes ever appended.
     */
    pub fn query_leaves(
        &self,
        key: TypeKey,
        cursor: usize,
        count: usize,
    ) -> (usize, Vec<u128>, bool) {
        let mut start_index: usize = 0;
        let mut values: Vec<u128> = Vec::new();
        let mut has_more: bool = false;
        if let Some(tree) = self.sortition_sum_trees.get(&key) {
            for i in 1..=tree.nodes.len() {
                if (tree.K) + 1 >= tree.nodes.len() {
                    start_index = i;
                    break;
                }
            }
            let loop_start_index = start_index + cursor;
            for j in loop_start_index..tree.nodes.len() {
                if values.len() < count {
                    values.push(tree.nodes[j]);
                } else {
                    has_more = true;
                    break;
                }
            }
        }
        return (start_index, values, has_more);
    }
}
