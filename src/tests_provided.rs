//
// Problem 3
//

#[test]
fn test_sieve_basic() {
    assert_eq!(vec![2,3,5,7,11], sieve(12));
}

//
// Problem 4
//

#[test]
fn test_hanoi_1_disks() {
    let result = hanoi(1, Peg::A, Peg::B, Peg::C);
    assert_eq!(vec![(Peg::A, Peg::C)], result);
    assert_eq!(1, result.len());
}
