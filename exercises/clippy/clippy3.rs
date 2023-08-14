fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // 使用 expect 来处理 None 的情况，避免 panic
        panic!("The Option is None!");
    }

    let my_arr = &[
        -1, -2, -3, // 在每个元素后面加上逗号
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
