use rand::Rng;
use std::vec::Vec;

//need to make more functions
//need to make a function that outputs arrays
//learn typecasting
//how to make for loop work with dynamic length arrays
fn main() {
    let mut rand = rand::thread_rng();
    
    const LEN:usize = 10;

    let mut arr:[i32;LEN] = [0;LEN];
    for i in 0..LEN 
    { 
        arr[i] = rand.gen_range(0..20);
    }

    println!("a:{:?}", arr); 

    let mut min:i32 = arr[0];
    let mut max:i32 = arr[0];

    for i in 0..LEN
    {
        if min > arr[i]
        {  
            min = arr[i];
        }
        if max < arr[i]
        {  
            max = arr[i];
        }
    }

    println!("min: {}", min);
    println!("max: {}", max);

    let mut dy_array: Vec<i32> = Vec::new(); // Create an empty Vec of i32 elements

    let range:i32 = max-min;
    let mut i:i32 = 0;
    while i <= range
    { 
        dy_array.push(i+ min);
        i+=1;
    }

    println!("dy_array:{:?}", dy_array); 

    let mut count_array: Vec<i32> = Vec::new();
    i= 0;
    while i < dy_array.len() as i32
    { 
        count_array.push(0);
        i+=1;
    }

    i = 0;
    while i < arr.len() as i32
    {
        let tmp:i32 = arr[i as usize] - min;
        count_array[tmp as usize] +=1;
        i+=1;
    }

    println!("count_array:{:?}", count_array);
    
    i = 0;
    let mut j=0;
    while j < count_array.len() as i32 
    {
        
        let mut curr = count_array[j as usize];
        loop 
        { 
            if curr > 0
            {
                arr[i as usize] = j + min;//(j).try_into().unwrap();
                curr-=1;
                i+=1;
            }
            else {
                break;
            }
        }
        j+=1;
    }

    println!("final array: {:?}", arr);
    
}

