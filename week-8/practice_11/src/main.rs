fn main() {
    let nums = [1,2,3,4,5];
    println!("original array = {:?}",nums);

    let slice1 = &nums[1..3];
    println!("2nd and 3rd elements sliced = {:?}",slice1);

    let slice2 = &nums[..3];
    println!("index 0 to index 3 sliced = {:?}",slice2);


    let slice3 = &nums[2..];
    println!("index 2 to index 5 sliced = {:?}",slice3);


    let slice4 = &nums[..];
    println!("index 0 to index 5 sliced = {:?}",slice4);
}
