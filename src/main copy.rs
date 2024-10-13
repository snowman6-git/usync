use std::fs::File;
use std::io::{self, Read};
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    name: String,
    age: u32,
}

fn read_json() -> io::Result<Config> {
    let file = File::open("config.json")?;
    let config: Config = serde_json::from_reader(file)?;
    Ok(config)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match read_json() {
        Ok(config) => {
            println!("Config load ok: {:?}", config);
            // JSON 값을 문자열로 변환
            let target = serde_json::to_string(&config).expect("JSON 변환 실패!");
            println!("{}", target);

            // JSON 문자열을 Value로 파싱
            let json_value: Value = serde_json::from_str(&target).expect("JSON 파싱 실패!");
            println!("JSON 값: {:?}", json_value);
        }
        Err(e) => {
            eprintln!("JSON 읽기 실패: {}", e);
            match e {
                // 추가적인 오류 처리
                _ => eprintln!("알 수 없는 오류 발생"),
            }
        }
    }
    Ok(())
}
