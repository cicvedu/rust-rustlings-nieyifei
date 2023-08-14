// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.



#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // 如果 Option 是 None，这里仍然会发生 panic，你可以考虑使用 expect 或者 match 来处理 None 的情况
        panic!("The Option is None!");
    }

    let my_arr = &[
        -1, -2, -3, // 需要在每个元素后面加上逗号
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let my_empty_vec: Vec<i32> = Vec::new(); // 使用 Vec::new() 来创建一个空的 Vec
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    std::mem::swap(&mut value_a, &mut value_b); // 使用 std::mem::swap 函数来交换值
    println!("value a: {}; value b: {}", value_a, value_b);
}

