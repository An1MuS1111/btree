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

#[derive(Debug)]
pub struct Btree<T: Ord + Debug + Clone> {
    root: Option<Box<BtreeNode<T>>>,
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

    // lower bound would be the index where key would be inserted to maitain the sorted array
    // todo: will edit the implementation in insert_non_full later
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
}

fn main() {}
