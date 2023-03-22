use crate::tree::binary_tree::*;

// Evaluate a reverse polish notation and return a boolean
fn eval_formula(formula: &str) -> bool {
    let reversed: String = formula.chars().rev().collect();

    // Create root node
    let mut tree = BinaryTree::new(reversed.chars().nth(0).unwrap());

    // Populate binary tree
    for c in reversed.chars() {
        if c == reversed.chars().nth(0).unwrap() {
            continue ;
        }
        tree.insert(c);
    }

    tree.print();
    return true;
}

// Test section
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gray_code0() {
        assert!(eval_formula("10&") == true);
    }
}