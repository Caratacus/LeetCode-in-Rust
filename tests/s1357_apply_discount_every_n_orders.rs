// Tests for Problem 1357: Apply Discount Every n Orders
// Java reference: src/test/java/g1301_1400/s1357_apply_discount_every_n_orders/SolutionTest.java

use leetcode_in_rust::s1357::apply_discount_every_n_orders::Cashier;

#[test]
fn test_cashier_test() {
    let mut cashier = Cashier::new(3, 50, vec![1, 2, 3, 4, 5, 6, 7], vec![100, 200, 300, 400, 300, 200, 100]);
    assert_eq!(cashier.get_bill(vec![1, 2], vec![1, 2]), 500.0);
    assert_eq!(cashier.get_bill(vec![3, 7], vec![10, 10]), 4000.0);
    assert_eq!(cashier.get_bill(vec![1, 2, 3, 4, 5, 6, 7], vec![1, 1, 1, 1, 1, 1, 1]), 800.0);
    assert_eq!(cashier.get_bill(vec![4], vec![10]), 4000.0);
    assert_eq!(cashier.get_bill(vec![7, 3], vec![10, 10]), 4000.0);
    assert_eq!(cashier.get_bill(vec![7, 5, 3, 1, 6, 4, 2], vec![10, 10, 10, 9, 9, 9, 7]), 7350.0);
    assert_eq!(cashier.get_bill(vec![2, 3, 5], vec![5, 3, 2]), 2500.0);
}
