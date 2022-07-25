use std::collections::HashMap;

fn main() {
    // Vector
    // let v = vec![1,2,3,4,5];

    // let third: &i32 = &v[100];
    // println!("세 번째 원소: {}", third);

    // match v.get(100) {
    //     Some(third) => println!("세 번째 원소: {}", third),
    //     None => println!("세 번째 원소가 없습니다."),
    // }



    //first에 참조가 되어있는데 push를 하면 메모리공간을 새로운 곳으로 옮길 수 있기 때문에 에러
    // let mut v = vec![1,2,3,4,5];

    // let first = &v[0];

    // v.push(6);



    // let mut v = vec![1,2,3,4,5];
    // for i in &mut v {
    //     *i += 50;
    //     println!("{}", i);
    // }



    // enum SpreadsheetCell {
    //     Int(i32),
    //     Float(f64),
    //     Text(String),
    // }

    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Text(String::from("블루")),
    //     SpreadsheetCell::Float(10.12),
    // ];



    //String
    // let mut s = String::from("foo");
    // s.push_str("bar");
    // println!("{}", s);



    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2;



    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");

    // //문자열의 최종형태를 가늠하기 쉽지 않다.
    // let s = s1 + "-" + s2 + "-" + s3;
    // //복잡한 문자열에서는 format! 매크로가 더 적합하다
    // let s = format!("{}-{}-{}", s1, s2, s3);



    // let hello = "안녕하세요";
    
    // let s = &hello[0..3];

    // for c in "안녕하세요".chars() {
    //     println!("{}", c);
    // }



    //HashMap
    // let mut scores = HashMap::new();

    // scores.insert(String::from("블루"), 10);
    // scores.insert(String::from("옐로"), 50);

    // let team_name = String::from("블루");
    // let score = scores.get(&team_name);

    // for (key, value) in &scores {
    //     println!("{}: {}", key, value);
    // }



    // let teams = vec![String::from("블루"), String::from("옐로")];
    // let initial_scores = vec![10, 50];

    // let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();



    // let field_name = String::from("Favorite color");
    // let field_value = String::from("블루");

    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);
    // //field_name과 field_value 변수는 이 지점부터 유효하지 않다.



    // let mut scores = HashMap::new();

    // scores.insert(String::from("블루"), 10);

    // scores.entry(String::from("옐로")).or_insert(50);
    // scores.entry(String::from("블루")).or_insert(50);

    // println!("{:?}", scores);



    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
