// pub fn add_two(a: i32) -> i32 {
//     a + 2
// }

// pub fn greeting(name: &str) -> String {
//     format!("안녕하세요 {}!", name)
//     // String::from("안녕하세요!")
// }

// fn prints_and_returns_10(a: i32) -> i32 {
//     println!("입력값: {}", a);
//     10
// }

pub fn add_two(a: i32) -> i32 {
    a + 2
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // #[test]
    // fn exploration() {
    //     let result = 2 + 2;
    //     assert_eq!(result, 4);
    // }

    // #[test]
    // fn another() {
    //     panic!("테스트 실패!");
    // }

    // #[test]
    // fn it_adds_two() {
    //     assert_eq!(4, add_two(2));
    // }

    // #[test]
    // fn greeting_contains_name() {
    //     let result = greeting("캐롤");
    //     assert!(
    //         result.contains("캐롤"),
    //         "Greeting 함수의 결과에 이름이 포함되어 있지 않음. 결과값: '{}'", result
    //     );
    // }

    // #[test]
    // fn this_test_will_pass() {
    //     let value = prints_and_returns_10(4);
    //     assert_eq!(10, value);
    // }

    // #[test]
    // fn this_test_will_fail() {
    //     let value = prints_and_returns_10(8);
    //     assert_eq!(5, value);
    // }

    // #[test]
    // fn add_two_and_two() {
    //     assert_eq!(4, add_two(2));
    // }

    // #[test]
    // fn add_three_and_two() {
    //     assert_eq!(5, add_two(3));
    // }

    // #[test]
    // fn one_hundred() {
    //     assert_eq!(102, add_two(100));
    // }

    // #[test]
    // fn it_works() {
    //     assert_eq!(2 + 2, 4);
    // }

    // #[test]
    // #[ignore]
    // fn expensive_test() {
    //     //실행에 한시간 걸리는 테스트
    // }

    // #[test]
    // fn internal() {
    //     assert_eq!(4, internal_adder(2,2));
    // }
    
    // use adder;
    
    // #[test]
    // fn it_adds_two() {
    //     assert_eq!(4, adder::add_two(2,2));
    // }
}
