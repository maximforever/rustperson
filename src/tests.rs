use super::*;

#[test]
fn test_valid_input() {
    let g = Game::new();

    assert!(g.is_valid(&'r'));
    assert!(g.is_valid(&'T'));
}

#[test]
fn test_invalid_input() {
    let g = Game::new();
    assert_eq!(g.is_valid(&'😋'), false);
    assert_eq!(g.is_valid(&'3'), false);
    assert_eq!(g.is_valid(&'#'), false);
    assert_eq!(g.is_valid(&'ن'), false);
    assert_eq!(g.is_valid(&'¢'), false);
}
