// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.



#[test]
fn main() {
    let mut x = 100;
    let z = &mut x;
    *z += 1000; // z is a mutable reference to x
    let y = &mut x; // y is a mutable reference to x
    *y += 100; // y is a mutable reference to x
    
    assert_eq!(x, 1200);
}


