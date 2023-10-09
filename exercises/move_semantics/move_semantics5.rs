// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

//注意一个引用的作用域从声明的地方开始一直持续到最后一次使用为止。
fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;  
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
