fn main() {
    let num:i32 = 5;
    m_num_to_zero(num);
    println!("The value of the number is: {}",num);
}

fn m_num_to_zero(mut para_num:i32) {
    para_num = para_num*0;
    println!("param_num value is: {}",para_num);
}


