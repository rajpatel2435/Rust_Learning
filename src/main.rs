enum Shape{
    Circle(f64),
    Rectangle(f64,f64),
    Triangle(f64,f64,f64)
}

fn calculate_area(shape: Shape) -> f64{

    //matching

    let ans: f64 = match shape {
        Shape::Circle(radius) => 3.14*radius*radius,
        Shape::Rectangle(length,width) => length*width,
        Shape::Triangle(a,b,c) => {
            let s = (a+b+c)/2.0;
            let area = (s*(s-a)*(s-b)*(s-c)).sqrt();
            area
        }
    };

    return ans  ;
}


fn vector_fn(){
    let mut vec = Vec::new();

    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("vec {}", vec[0]);

    //lest move the ownehsip to anotehr function
// passing the ownhsip
    let ans= even_filter(&mut vec);

    
}

fn even_filter(vec : &Vec<i32>)  {
    let mut  new_vac= Vec::new();
    for i in vec {
        if i % 2 == 0 {
            new_vac.push(i);
        }
    }

}


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
    print!("{}", first_word);

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

fn stack_fn() {
    let a: i32 = 10;
    let b: i32 = 20;
    let c: i32 = a + b;
    println!("c {}", c);
}

fn heap_fn() {
    let s1: String = String::from("hello");
    let s2: String = String::from("world");
    let s3: String = s1 + &s2;
    println!("s3 {}", s3);
}

fn update_string() {
    let mut s1: String = String::from("hello");
    s1.push_str(" world");
    println!("s1 {}", s1);
}

// ownership
//  one owner at time if s2=s1 then s1 is no longer valid
// 2 people cannot own same thing

fn main_ownership() {
    let s1: String = String::from("hello");
    let s2: String = s1;
    // println!("s1 {}",s1); // this should not work 2 people cannot own same thing
    println!("s2 {}", s2);
}

// borowing
// 2 people can own same thing but only one can use it at a time

fn main_borowing() {
    let s1: String = String::from("hello");
    let s2: &String = &s1;
    println!("s1 {}", s1);
    println!("s2 {}", s2);
}
