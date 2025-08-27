mod btree;
use crate::btree::Btree;

// ***Example usage and testing***
fn main() {
    println!("=== B-Tree Implementation with Deletion Demo ===\n");

    // create a B-tree with minimum degree 3
    let mut btree = Btree::new(3);

    // build a substantial tree for deletion testing
    println!("1. Building initial tree:");
    let initial_keys = vec![1, 3, 7, 10, 16, 18, 23, 26, 30, 33, 35, 38, 41, 45];
    for key in initial_keys {
        btree.insert(key);
    }
    btree.print_tree();
    println!("\n{}", "=".repeat(50));

    // test Case 1: Delete from leaf (simple case)
    println!("\n2. CASE 1: Delete from leaf - Deleting 3 (leaf has > t-1 keys):");
    println!("Before deletion:");
    btree.print_tree();

    let deleted = btree.delete(&3);
    println!("Deleted 3: {}", deleted);
    println!("After deletion:");
    btree.print_tree();
    println!("\n{}", "=".repeat(50));

    // test Case 2a: Delete from internal node (predecessor case)
    println!("\n3. CASE 2a: Delete from internal - Deleting 16 (using predecessor):");
    println!("Before deletion:");
    btree.print_tree();

    let deleted = btree.delete(&16);
    println!("Deleted 16: {}", deleted);
    println!("After deletion (16 replaced with predecessor):");
    btree.print_tree();
    println!("\n{}", "=".repeat(50));

    // add more keys to create scenarios for borrowing and merging
    println!("\n4. Adding more keys to test complex deletion scenarios:");
    let more_keys = vec![2, 4, 5, 6, 8, 9, 11, 12, 13, 14, 15, 17, 19, 20, 21, 22];
    for key in more_keys {
        btree.insert(key);
    }
    println!("Tree after adding more keys:");
    btree.print_tree();
    println!("\n{}", "=".repeat(50));

    // test borrowing from sibling
    println!("\n5. BORROWING TEST: Deleting keys to trigger borrowing:");

    // delete a few keys to create underflow scenarios
    let keys_to_delete = vec![2, 4, 5];
    for key in keys_to_delete {
        println!("\nDeleting {}:", key);
        println!("Before:");
        btree.print_tree();

        let deleted = btree.delete(&key);
        println!("Deleted {}: {}", key, deleted);
        println!("After:");
        btree.print_tree();
        println!("{}", "-".repeat(30));
    }

    println!("\n{}", "=".repeat(50));

    // test merging scenario
    println!("\n6. MERGING TEST: Deleting keys to trigger merging:");

    // delete more keys to force merging
    let merge_keys = vec![6, 8, 9, 11];
    for key in merge_keys {
        println!("\nDeleting {}:", key);
        println!("Before:");
        btree.print_tree();

        let deleted = btree.delete(&key);
        println!("Deleted {}: {}", key, deleted);
        println!("After:");
        btree.print_tree();
        println!("{}", "-".repeat(30));
    }

    // test deleting root key
    println!("\n{}", "=".repeat(50));
    println!("\n7. ROOT DELETION TEST:");
    println!("Current tree:");
    btree.print_tree();

    // try to delete what might be a root key
    println!("\nDeleting 18 (might cause root changes):");
    let deleted = btree.delete(&18);
    println!("Deleted 18: {}", deleted);
    println!("After deletion:");
    btree.print_tree();

    // test edge case: delete non-existent key
    println!("\n{}", "=".repeat(50));
    println!("\n8. EDGE CASE: Delete non-existent key:");
    let deleted = btree.delete(&100);
    println!("Trying to delete 100 (doesn't exist): {}", deleted);

    // final verification
    println!("\n{}", "=".repeat(50));
    println!("\n9. FINAL VERIFICATION - Search for remaining keys:");
    let test_keys = vec![
        1, 7, 10, 12, 13, 14, 15, 17, 19, 20, 21, 22, 23, 26, 30, 33, 35, 38, 41, 45,
    ];
    for key in test_keys {
        let found = btree.search(&key);
        println!("Search {}: {}", key, if found { "✓" } else { "✗" });
    }

    println!("\nFinal tree structure:");
    btree.print_tree();
}
