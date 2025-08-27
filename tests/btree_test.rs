use btree::btree::Btree;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_tree() {
        let btree: Btree<i32> = Btree::new(2);
        assert!(btree.is_empty());
        assert!(!btree.search(&5));
    }

    #[test]
    fn test_single_insertion() {
        let mut btree = Btree::new(2);
        btree.insert(10);
        assert!(!btree.is_empty());
        assert!(btree.search(&10));
        assert!(!btree.search(&5));
    }

    #[test]
    fn test_multiple_insertions() {
        let mut btree = Btree::new(3);
        let keys = vec![1, 3, 7, 10, 16, 18, 23, 26, 30];

        for key in keys.clone() {
            btree.insert(key);
        }

        // all inserted keys should be found
        for key in keys {
            assert!(btree.search(&key));
        }

        // non-inserted keys should not be found
        assert!(!btree.search(&2));
        assert!(!btree.search(&25));
        assert!(!btree.search(&50));
    }

    #[test]
    fn test_deletion_from_leaf() {
        let mut btree = Btree::new(3);
        let keys = vec![1, 3, 7, 10, 16, 18, 23];

        for key in keys {
            btree.insert(key);
        }

        // delete from leaf
        assert!(btree.delete(&1));
        assert!(!btree.search(&1));
        assert!(btree.search(&3)); // other keys should still exist
    }

    #[test]
    fn test_deletion_from_internal() {
        let mut btree = Btree::new(3);
        let keys = vec![1, 3, 7, 10, 16, 18, 23, 26, 30];

        for key in keys {
            btree.insert(key);
        }

        // delete from internal node
        assert!(btree.delete(&16));
        assert!(!btree.search(&16));

        //other keys should still exist
        assert!(btree.search(&1));
        assert!(btree.search(&30));
    }

    #[test]
    fn test_deletion_nonexistent() {
        let mut btree = Btree::new(3);
        let keys = vec![1, 3, 7, 10];

        for key in keys {
            btree.insert(key);
        }

        // try to delete non-existent key
        assert!(!btree.delete(&5));
        assert!(!btree.delete(&100));

        // original keys should still exist
        assert!(btree.search(&1));
        assert!(btree.search(&10));
    }

    #[test]
    fn test_delete_all_keys() {
        let mut btree = Btree::new(2);
        let keys = vec![1, 2, 3, 4, 5];

        // insert all keys
        for key in keys.clone() {
            btree.insert(key);
        }

        // delete all keys
        for key in keys {
            assert!(btree.delete(&key));
        }

        // tree should be empty
        assert!(btree.is_empty());
    }
}
