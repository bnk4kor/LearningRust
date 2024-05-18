use std::char;

fn main() {
    let s1 = "the fox jumps over the lazy dog";

    println!("s1: {}", s1);

    for word in s1.split_whitespace() {
        println!("{}", word);
    }

    let mut s2 = s1.to_string();
    s2.push('!');
    s2.push_str(" zomething");
    let s3 = s2;
    println!("s2: {}", s3);

    for ch in s3.chars() {
        println!("{}", ch);
    }

    let mut col: Vec<_> = s3.split_whitespace().collect();
    col.sort();
    col.dedup();
    let col2 = col.clone();
    for ss in col {
        println!("{}", ss);
    }

    let newStr = String::new();
    let mut newStr1 = newStr + "Hello";
    newStr1.push_str(" World");
    newStr1 = newStr1 + "!";
    println!("newStr: {}", newStr1);

    for con in col2 {
        newStr1 = newStr1 + con;
        newStr1.push(',');
    }
    println!("newStr: {}", newStr1);
}
