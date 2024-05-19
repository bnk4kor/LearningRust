// Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order.

// Example 1:
// Input: nums = [1,1,1,2,2,3], k = 2
// Output: [1,2]

// Example 2:
// Input: nums = [1], k = 1
// Output: [1]


use std::collections::HashSet;

pub fn freq() {
    let nums = [1, 1, 1, 2, 2, 3];
    

    let mut sets: HashSet<i32> = HashSet::new();

    for n in nums.iter() {
        if !sets.contains(n) {
            sets.insert(*n);
        } else {
            println!("This is a occured {}", n);
            return;
        }
    }
}
