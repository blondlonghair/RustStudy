use std::io;
use std::io::Read;
use std::fs::File;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    // panic!("crash and burn");
    
    // let v = vec![1,2,3];

    // v[99];



    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(ref error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("파일을 생성하지 못했습니다: {:?}", e),
    //         },
    //         other_error => panic!("파일을 열지 못했습니다: {:?}", other_error),
    //     },
    // };

    // let f = File::open("hello.txt").unwrap();


    
    let f = File::open("hello.txt")?;
    
    Ok(())
}

// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");

//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();

//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut f = File::open("hello.txt");
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut s = String::new();

//     File::open("hello.txt")?.read_to_string(&mut s)?;

//     Ok(s)
// }

// fn read_username_from_file() -> Result<String, io::Error> {
//     fs::read_to_string("hello.txt")
// }
