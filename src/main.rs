fn main() {
    // println!("Hello, world!");


    // let greeting =String::from("Hello");
    // println!("{}",greeting); 

    // let is_even= true;

    // if is_even {
    //     println!("is even");
    // } else if !is_even {
    //     println!("is not even");
    // }

    // for i32 in 0..10 {
    //     println!("i32 {}",i32);
    // }

    let sentence = "hello world";
let first_word = get_first_char(sentence.to_string());
print!("{}",first_word);


// stack v/s heap
// stack faster store number fixed array
// heap bit slower store string, object, array



}


fn get_first_char(sentence: String) -> char {
    
    let mut ans: String = String::from("");
    for char in sentence.chars() {
        ans = char.to_string();
        break;
    }
    return ans.chars().nth(0).unwrap();
}


fn stack_fn(){
    let a : i32 =10;
    let b :i32 =20;
    let c : i32 =a+b;
    println!("c {}",c);
}


fn heap_fn(){
    let s1: String = String::from("hello");
    let s2: String = String::from("world");
    let s3: String = s1 + &s2;
    println!("s3 {}",s3);
}

fn update_string(){
    let mut s1: String = String::from("hello");
    s1.push_str(" world");
    println!("s1 {}",s1);
}