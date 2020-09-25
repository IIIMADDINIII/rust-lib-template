extern crate rust_lib_template;

#[test]
fn test_add() {
  assert_eq!(rust_lib_template::add(2, 2), 4);
}