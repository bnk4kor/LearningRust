

pub fn quicksort(){
    let mut arr = [4, 3, 2, 1, 5, 6, 7, 8, 9];
    let end = (arr.len()-1) as i32;
    qsort(&mut arr,0,end);

    for n in arr.iter() {
        print!("{} ",n);
    }
}

fn qsort(arr: &mut [i32; 9], start: i32 , end : i32){
    
    if end<=start{
        return;
    }

    let pi = partition(arr, start, end);

    qsort(arr,start, pi-1);
    qsort(arr,pi+1, end);

}

fn partition( arr: &mut [i32; 9], start: i32 , end : i32)-> i32 {

    let pivot: i32 = arr[end as usize];
    let mut i: i32= start-1;


    for j in start as usize..end as usize{
        if  arr[j]<pivot {
            i += 1;
            arr.swap(i as usize,j );
        }
    }

    i += 1;
    arr.swap(i as usize, end as usize);

   return  i as i32;
}