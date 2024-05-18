mod TwoSum;
mod Anagram;


fn main() {
    println!("Hello, world!");
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr2: [f32; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
    let mut a = 0;
    arr[0] = 32;
    let mut arr4: [f32; 5] = [0.0; 5];
    for i in arr.iter() {
        let x = ((arr[a] as f32) + arr2[a] + 1.6) as f32;
        println!("{}", x);

        arr4[a] = x;
        a += 1;
    }

    let arr3 = &arr4[0..2];
    for i in arr3.iter() {
        println!("{}", i);
        
    }

    TwoSum::sum1();
    Anagram::findAnagram();
}
