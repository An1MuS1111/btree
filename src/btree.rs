use std::fmt::Debug;
/*
** every btree has minimum degree where degree >= 2
** every node except the root must contain at least degree - 1 keys
** every node can contain at most 2*degree - 1 keys
** the root can have as few as 1 key (unless it's the only node)
** the root has to have atleast 1 key (unless it's the only node)
** all keys within the node are stored in (ascending) order Ex. 1, 2, 3, 4,...
** for any key "k" in the node, all keys in the left subtree are less than "k",
** and all keys in the right subtree is greater than "k"
*/

#[derive(Debug, Clone)]
pub struct BtreeNode<T: Ord + Clone + Debug> {
    keys: Vec<T>,
    children: Vec<Box<BtreeNode<T>>>,
    is_leaf: bool,
    degree: usize,
}

impl<T: Ord + Clone + Debug> BtreeNode<T> {
    fn new(degree: usize, is_leaf: bool) -> Self {
        assert!(degree >= 2, "degree must be getter than 2");
        BtreeNode {
            keys: Vec::new(),
            children: Vec::new(),
            is_leaf,
            degree,
        }
    }

    // check if the BtreeNode is full (contains 2t - 1)
    fn is_full(&self) -> bool {
        self.keys.len() == 2 * self.degree - 1
    }

    // lower bound would be the index where key would be inserted to maintain the sorted array
    // or where the key should be located
    fn lower_bound(&self, key: &T) -> usize {
        match self.keys.binary_search(key) {
            Ok(i) | Err(i) => i,
        }
    }

    fn search(&self, key: &T) -> bool {
        let i = self.lower_bound(key);
        if i < self.keys.len() && &self.keys[i] == key {
            return true;
        }
        if self.is_leaf {
            false
        } else {
            self.children[i].search(key)
        }
    }

    // search for a key in this sub tree (will implement later)

    // insert a key into non full node
    fn insert_non_full(&mut self, key: T) {
        // getting the last key's index in a non full node
        // let mut i = self.keys.len() as i32 - 1;

        // if it's leaf node then we insert the key and then sort the keys of the node
        if self.is_leaf {
            /*
             ** let's say the degree is 3
             ** so the leaf node may contain at most 5 keys
             ** so for non full has to be less than 5; (keys.len() > 5)
             ** let's say it has 4 keys right now; (size = 4)
             ** so the last key's index should be 4 - 1 = 3; index "i" = 3
             ** imagine the node contains [1, 2, 5, 7] keys with respectable index of "0" , "1", "2" , "3"
             ** suppose the key we are about to insert is 4
             ** keys.push appends the key with the value of 4 in the back with the index of "4"
             ** now let's start to adjust the key into ascending order
             ** if the index "i" is getter that 0 and keys[i] is getter than the key = 4 (value)
             ** we shift the value by 1 index keys[i] >> keys[i + 1]
             ** so the array before shifting [1, 2, 5, 7, 4]
             ** array after the shifting [1, 2, 5, 7, 7]
             ** then we decrement the index "i" by one so index i = 2
             ** we target the next value [remember: we are moving from back to front]
             ** next value 5 is getter than key = 4
             ** so the array before [1, 2, 5, 7, 7]
             ** array after [1, 2, 5, 5, 7]
             ** we again decrement the index "i" by one so now the index is i = 1
             ** next value 2 is less than key = 4
             ** so the loops ends
             ** and we set the keys[i + 1] = key which is 4
             ** final look at the array after [1, 2, 4, 5, 7]
             ** self.keys.push(key.clone());
             *******************************************************************
             ** while i >= 0 && self.keys[i as usize] > key {
             **     self.keys[(i + 1) as usize] = self.keys[i as usize].clone();
             **     i -= 1;
             ** }
             ** insert the new key
             ** self.keys[(i + 1) as usize] = key;
             *******************************************************************
             */

            // get the position where the key could be inserted in sorted array
            let pos = self.keys.binary_search(&key).unwrap_or_else(|e| e);
            // insert the new key
            self.keys.insert(pos, key);
        } else {
            /*
             ** internal node: choose child and ensure it's not full before descending
             ** find child where new key should be inserted
             ** while i >= 0 && self.keys[i as usize] > key {
             **     i -= 1
             ** }
             ** move to correct child index
             ** i += 1;
             */
            let mut i = self.lower_bound(&key);

            // if the child is full, we need to split it first
            if self.children[i as usize].is_full() {
                self.split_child(i as usize);

                // after split decide which side to insert to
                if key > self.keys[i as usize] {
                    i += 1;
                }
            }

            // recursively insert into the appropriate children
            self.children[i as usize].insert_non_full(key);
        }
    }

    /*
     ** split a child at index i (child has 2t - 1 keys)
     ** after the split:
     ** left child keeps the first t - 1 keys
     ** median key index "t - 1" moves up into this node
     ** right child gets the last t - 1 keys
     */
    fn split_child(&mut self, i: usize) {
        let degree = self.degree;

        // safety: caller guarantees child i exists and is full
        let full_child = &mut self.children[i];
        debug_assert_eq!(full_child.keys.len(), 2 * degree - 1);

        // prepare the new right sibling
        // both siblings share the same degree & leaf flag
        // this new node holds the second half of keys
        let mut new_child = BtreeNode::new(degree, full_child.is_leaf);

        // move the second half of the keys to new node
        // keys at position [t, 2t - 1] move to new node
        for j in 0..degree - 1 {
            new_child.keys.push(full_child.keys[j + degree].clone());
        }

        // if not leaf, move the second half of the children too
        // underlying method removes the children one by one, shifting the vector repeatedly (O(n^2))
        // if !full_child.is_leaf {
        //     for _ in 0..degree {
        //         new_child.children.push(full_child.children.remove(degree));
        //     }
        // }
        // better to drain like
        if !full_child.is_leaf {
            new_child
                .children
                .extend(full_child.children.drain(degree..));
        }

        // remove the moved keys from the original child
        full_child.keys.truncate(degree);

        // let middle key at position (t - 1) moves up to parent
        let middle_key = full_child.keys.remove(degree - 1);

        // insert the new child in to parent's array
        self.children.insert(i + 1, Box::new(new_child));

        // insert middle key into parent's keys array
        self.keys.insert(i, middle_key);
    }

    // Helper method to print the tree structure
    fn print_tree(&self, level: usize) {
        println!(
            "{}Keys: {:?} (leaf: {})",
            " ".repeat(level),
            self.keys,
            self.is_leaf
        );
        for child in &self.children {
            child.print_tree(level + 1);
        }
    }
    /*
     ** deletation in btrees is significantly more complex than insertion.
     ** let's break down all the edge cases and decision making process first.
     ** when deleting a key we need to consider:
     ** 1. where is the key located? (internal node or leaf node)
     ** 2. does the node have enough keys? (more than minimum)
     ** 3. do siblings have spare keys? (for borrowing)
     ** 4. can we merge with sibling node? (when borrowing isn't possible)
     */
    fn delete(&mut self, key: &T) -> bool {
        // get the index (i) where the key should be or should be inserted
        let i = self.lower_bound(key);

        // - this line might be unnecessary
        if i < self.keys.len() && &self.keys[i] == key {
            // key found in this node
            if self.is_leaf {
                // key is in leaf node
                self.delete_from_leaf(i)
            } else {
                self.delete_from_internal(i)
            }
        } else {
            // key not in this node, must be in child (if exists)

            if self.is_leaf {
                return false;
            }
            let is_last_child = i == self.keys.len();

            // Ensure child has enough keys before recursing
            if self.children[i].keys.len() < self.degree {
                self.fix_child_underflow(i);
            }

            // After fixing, the child might have moved
            let child_idx = if is_last_child && i > self.keys.len() {
                i - 1
            } else {
                i
            };

            self.children[child_idx].delete(key)
        }
    }

    fn delete_from_leaf(&mut self, i: usize) -> bool {
        self.keys.remove(i);
        true
    }

    // delete from the internal node
    fn delete_from_internal(&mut self, i: usize) -> bool {
        let key = &self.keys[i].clone();

        // check if left child node has >= degree keys
        // find predecessor (largest key in left subtree)
        if self.children[i].keys.len() >= self.degree {
            let predecessor = self.get_predecessor(i);
            self.keys[i] = predecessor.clone();
            self.children[i].delete(&predecessor)

            // check if right child node has >= degree keys
            // find successor (smallest key in the right subtree)
        } else if self.children[i + 1].keys.len() >= self.degree {
            let successor = self.get_successor(i);
            self.keys[i] = successor.clone();
            self.children[i + 1].delete(&successor)
            // both children has exactly t - 1 keys
            // merge key with both children
        } else {
            // need to fix this
            self.merge_children(i);
            self.children[i].delete(key)
        }
    }
    // get predecessor of key at index idx (largest key in left subtree)
    fn get_predecessor(&self, idx: usize) -> T {
        let mut current = &self.children[idx];
        while !current.is_leaf {
            current = &current.children[current.children.len() - 1];
        }
        current.keys[current.keys.len() - 1].clone()
    }

    // get successor of key at index idx (smallest key in right subtree)
    fn get_successor(&self, idx: usize) -> T {
        let mut current = &self.children[idx + 1];
        while !current.is_leaf {
            current = &current.children[0];
        }
        current.keys[0].clone()
    }

    // merge key at idx with its left and right children
    fn merge_children(&mut self, idx: usize) {
        let key = self.keys.remove(idx);
        let right_child = self.children.remove(idx + 1);
        let left_child = &mut self.children[idx];

        // add the key to left child
        left_child.keys.push(key);

        // add all keys from right child
        left_child.keys.extend(right_child.keys);

        // add all children from right child (if not leaf)
        if !left_child.is_leaf {
            left_child.children.extend(right_child.children);
        }
    }

    // fix underflow in child at index idx
    fn fix_child_underflow(&mut self, idx: usize) {
        // try to borrow from left sibling
        if idx > 0 && self.children[idx - 1].keys.len() >= self.degree {
            self.borrow_from_left_sibling(idx);
        }
        // try to borrow from right sibling
        else if idx < self.children.len() - 1 && self.children[idx + 1].keys.len() >= self.degree
        {
            self.borrow_from_right_sibling(idx);
        }
        // merge with sibling
        else {
            if idx > 0 {
                // merge with left sibling
                self.merge_children(idx - 1);
            } else {
                // merge with right sibling
                self.merge_children(idx);
            }
        }
    }

    // borrow a key from left sibling
    fn borrow_from_left_sibling(&mut self, idx: usize) {
        // used split_at_mut to get mutable references to both children
        let (left, right) = self.children.split_at_mut(idx);
        let sibling = &mut left[idx - 1]; // left sibling
        let child = &mut right[0]; // the child that needs a key

        // move parent key down to child
        child.keys.insert(0, self.keys[idx - 1].clone());

        // move sibling's last key up to parent
        self.keys[idx - 1] = sibling.keys.pop().unwrap();

        // if not leaf, move sibling's last child to child's first
        if !child.is_leaf {
            child.children.insert(0, sibling.children.pop().unwrap());
        }
    }

    // borrow a key from right sibling
    fn borrow_from_right_sibling(&mut self, idx: usize) {
        // use split_at_mut to get mutable references to both children
        let (left, right) = self.children.split_at_mut(idx + 1);
        let child = &mut left[idx]; // the child that needs a key
        let sibling = &mut right[0]; // right sibling

        // move parent key down to child
        child.keys.push(self.keys[idx].clone());

        // move sibling's first key up to parent
        self.keys[idx] = sibling.keys.remove(0);

        // if not leaf, move sibling's first child to child's last
        if !child.is_leaf {
            child.children.push(sibling.children.remove(0));
        }
    }
}

#[derive(Debug)]
pub struct Btree<T: Ord + Debug + Clone> {
    root: Option<Box<BtreeNode<T>>>,
    degree: usize,
}

impl<T: Ord + Clone + Debug> Btree<T> {
    pub fn new(degree: usize) -> Self {
        assert!(degree >= 2, "degree must be atleast 2!");
        Btree { root: None, degree }
    }

    // search for a key in the tree
    pub fn search(&self, key: &T) -> bool {
        match &self.root {
            None => false,
            Some(root) => root.search(key),
        }
    }

    pub fn insert(&mut self, key: T) {
        match self.root.as_mut() {
            None => {
                // we create a 1 key leaf root
                let mut root = BtreeNode::new(self.degree, true);
                root.keys.push(key);
                self.root = Some(Box::new(root));
            }

            Some(root) if root.is_full() => {
                // if the root is full we allocate a new root
                // make old root its child, split, and then insert
                // this is the only case where the tree height increases
                let mut new_root = BtreeNode::new(self.degree, false);
                new_root.children.push(self.root.take().unwrap());
                // pplit the old root
                new_root.split_child(0);

                // after split the appropriate child is guaranteed not full
                new_root.insert_non_full(key);
                self.root = Some(Box::new(new_root));
            }

            // insert into possibly new root
            Some(root) => {
                root.insert_non_full(key);
            }
        }
    }

    // check if the tree is empty
    // used in the tests
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    // print the entire tree structure
    pub fn print_tree(&self) {
        match &self.root {
            None => println!("Empty tree"),
            Some(root) => {
                println!("B-tree (degree {}):", self.degree);
                root.print_tree(0);
            }
        }
    }

    // Delete a key from the tree
    pub fn delete(&mut self, key: &T) -> bool {
        match &mut self.root {
            None => false, // tree is empty
            Some(root) => {
                let found = root.delete(key);

                // special case: if root becomes empty after deletion
                if root.keys.is_empty() {
                    if root.is_leaf {
                        // tree becomes empty
                        self.root = None;
                    } else {
                        // root had only one key, and it was deleted
                        // the first child becomes the new root (tree height decreases)
                        self.root = Some(root.children.remove(0));
                    }
                }

                found
            }
        }
    }
}
