// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
// You may assume that each input would have exactly one solution, and you may not use the same element twice.
// You can return the answer in any order.


// Example 1:

// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
// Example 2:

// Input: nums = [3,2,4], target = 6
// Output: [1,2]
// Example 3:

// Input: nums = [3,3], target = 6
// Output: [0,1]

use core::num;
use std::collections::{HashMap, HashSet};

pub fn sum1(){
    let mut nums = [3,2,5,6,7,8,1];
    let target = 11;

    let mut st: HashSet<&i32> = HashSet::new();

    for i in nums.iter(){
        let diff = target - i;
        if st.contains(&diff){
            println!("{} {}", i, diff);
        }

        st.insert(&i);
    } 
}

