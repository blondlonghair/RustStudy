fn main() {
    // let s1 = String::from("hello");
    // let s2 = s1.clone();

    // println!("{}, world", s1);
    // println!("{}, world", s2);

    // let x = 5;
    // let y = x;

    // println!("x = {}, y = {}", x, y);



    // let s = String::from("hello");

    // takes_ownership(s);

    // let x = 5;

    // makes_copy(5);



    // let s1 = give_ownership();

    // let s2 = String::from("hello");

    // let s3 = takes_and_gives_back(s2);



    // let s1 = String::from("hello");

    // let (s2, len) = calculate_length(s1);

    // println!("'{}'의 길이는 {}입니다.", s2, len);



    // let s1 = String::from("hello");

    // let len = calculate_length(&s1);

    // println!("'{}'의 길이는 {}입니다.", s1, len);



    // let s = String::from("hello");

    // change(&s);



    // let mut s = String::from("hello");

    // change(&mut s);

    // println!("{}", s);



    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{} {}", r1, r2);



    // let reference_to_nothing = dangle();



    // let mut s = String::from("hello world");

    // let word = first_word(&s);;

    // s.clear();



    // let s = String::from("hello world");

    // let hello = &s[0..5];
    // let world = &s[6..11];

    // let s = String::from("hello");

    // let len = s.len();

    // let slice = &s[0..len];
    // let slice = &s[..];



    // let mut s = String::from("hello world");

    // let word = first_word(&s);

    // s.clear();
    // println!("the first word is: {}", word);



    let my_string = String::from("hello world");

    let word = first_word(&my_string[..]);

    println!("{}", word);
    
    let my_string_literal = "hello world";

    let word = first_word(&my_string_literal[..]);

    println!("{}", word);
 
    let word = first_word(my_string_literal);
 
    println!("{}", word);
}

// fn takes_ownership(some_string: String)
// {
//     println!("{}", some_string);
// }

// fn makes_copy(some_integer: i32)
// {
//     println!("{}", some_integer);
// }



// fn give_ownership() -> String
// {
//     let some_string = String::from("hello");

//     some_string
// }

// fn takes_and_gives_back(a_string: String) -> String
// {
//     a_string
// }



// fn calculate_length(s: String) -> (String, usize)
// {
//     let length = s.len();

//     (s, length)
// }



// fn calculate_length(s: &String) -> usize
// {
//     s.len()
// }



// fn change(some_string: &String)
// {
//     some_string.push_str(", world");
// }



// fn change(some_string: &mut String)
// {
//     some_string.push_str(", world");
// }



// fn dangle() -> &String 
// {
//     let s = String::from("hello");

//     s
// }



// fn first_word(s: &String) -> usize
// {
//     let byte = s.as_bytes();

//     for (i, &item) in byte.iter().enumerate()
//     {
//         if item == b' '
//         {
//             return i;
//         }
//     }

//     s.len()
// }

fn first_word(s: &str) -> &str
{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            return &s[0..i];
        }
    }

    &s[..]
}