// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.

// 语法错误：未定义函数参数类型

fn main() {
    call_me(3);
}

fn call_me(num:u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
