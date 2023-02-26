use hello::say_hello;

#[test]
fn test_hello() {
    assert_eq!(say_hello("Alice"), "Hello Alice!");
    assert_eq!(say_hello("Bob"), "Hello Bob!");
}
