// Tests for Problem 1600: Throne Inheritance
// Java reference: src/test/java/g1501_1600/s1600_throne_inheritance/ThroneInheritanceTest.java

use leetcode_in_rust::s1600::throne_inheritance::ThroneInheritance;

#[test]
fn test_throne_inheritance() {
    // order: king
    let mut t = ThroneInheritance::new("king".to_string());
    // order: king > andy
    t.birth("king".to_string(), "andy".to_string());
    // order: king > andy > bob
    t.birth("king".to_string(), "bob".to_string());
    // order: king > andy > bob > catherine
    t.birth("king".to_string(), "catherine".to_string());
    // order: king > andy > matthew > bob > catherine
    t.birth("andy".to_string(), "matthew".to_string());
    // order: king > andy > matthew > bob > alex > catherine
    t.birth("bob".to_string(), "alex".to_string());
    // order: king > andy > matthew > bob > alex > asha > catherine
    t.birth("bob".to_string(), "asha".to_string());
    assert_eq!(
        t.get_inheritance_order(),
        vec![
            "king".to_string(),
            "andy".to_string(),
            "matthew".to_string(),
            "bob".to_string(),
            "alex".to_string(),
            "asha".to_string(),
            "catherine".to_string()
        ]
    );
    // order: king > andy > matthew > bob > alex > asha > catherine
    t.death("bob".to_string());
    assert_eq!(
        t.get_inheritance_order(),
        vec![
            "king".to_string(),
            "andy".to_string(),
            "matthew".to_string(),
            "alex".to_string(),
            "asha".to_string(),
            "catherine".to_string()
        ]
    );
}
