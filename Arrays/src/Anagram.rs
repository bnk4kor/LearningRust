// Given two strings s and t, return true if t is an anagram of s, and false otherwise.

// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

// Example 1:

// Input: s = "anagram", t = "nagaram"
// Output: true
// Example 2:

// Input: s = "rat", t = "car"
// Output: false

pub fn findAnagram() {
    let s = "anagram";
    let t = "nagaram";

    let mut a: Vec<char> = s.chars().collect();
    let mut b: Vec<char> = t.chars().collect();

    a.sort();
    b.sort();

    let mut j = 0;
    for i in a.iter() {
        if(a[j] != b[j]){
            println!("false");
            return;
        }

        j=j+1;
    }
}
