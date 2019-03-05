use super::*;

#[test]
fn test_is_valid_input() {
    let g = Game::new();

    assert!(g.is_valid(&'r'));

    assert_eq!(g.is_valid(&'#'), false);
}