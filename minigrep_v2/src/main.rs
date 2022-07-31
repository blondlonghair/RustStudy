use std::env;
use std::process;

use minigrep_v2::Config;

fn main() {
    let config = Config::new(env.args()).unwrap_or_else(|err| {
        eprintln!("인수를 구문분석하는 동안 오류가 발생했습니다: {}", err);
        process::exit(1);
    });

    let query = &args[1];
    let filename = &args[2];

    println!("검색어: {}", config.query);
    println!("대상 파일: {}", config.filename);

    if let Err(e) = minigrep_v2::run(config) {
        eprintln!("애플리케이션 에러: {}", e);

        process::exit(1);
    }
}
