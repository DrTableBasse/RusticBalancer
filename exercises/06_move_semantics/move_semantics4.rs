// move_semantics4.rs
//
// Refactor this code so that instead of passing `vec0` into the `fill_vec`
// function, the Vector gets created in the function itself and passed back to
// the main function.
//
// Execute `rustlings hint move_semantics4` or use the `hint` watch subcommand
// for a hint.



#[test]
fn main() {
    let vec1 = fill_vec();

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec() -> Vec<i32> {
    let mut vec = vec![22, 44, 66]; // Create and fill the Vec inside the function

    vec.push(88);

    vec
}
