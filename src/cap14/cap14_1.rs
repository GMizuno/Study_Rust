// Rodando no PLayground
// Exemplo 1
#[test]
fn test_is_two() {
    assert_eq!(2, 2);
    assert_eq!(2, 3);
    assert!(2 == 3);
}

// Exemplo 2
fn return_two() -> i8 {
    2
}
#[test]
fn it_returns_two() {
    assert_eq!(return_two(), 2);
}
fn return_six() -> i8 {
    4 + return_two()
}
#[test]
fn it_returns_six() {
    assert_eq!(return_six(), 6)
}
