pub fn theMax(){
    let nums = [1,3,42,44,56,103,543];

    let mut max = nums[0];

    for num in nums.iter(){
        if num > &max 
        {
           max = *num;
        }
    } 
    println!("The max number is: {}", max);
}

pub fn theMin(){
    let nums = [1,3,42,44,56,103,543];

    let mut min = nums[0];

    for num in nums.iter(){
        if num < &min 
        {
           min = *num;
        }
    } 
    println!("The min number is: {}", min);
}