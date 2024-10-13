use rfd::FileDialog;
use std::fs::File;
use std::io::{Write, Read};
use serde_json::{json, Value};
use std::io;

struct Config {
    name: String,
    age: u32,
}

fn config_setup(){
    if let Some(folder_path) = FileDialog::new().pick_folder(){
        let target = folder_path.display();
        let mut config = File::create("usync.json").expect("파일 생성 실패!");
        let save = json! ({
            "target": format!("{}", target),
            "version": "0.0.1",
        });
        let json_string = serde_json::to_string(&save).expect("JSON 변환 실패"); //바이트 저장을 위해 문자열로 변환
        config.write_all(json_string.as_bytes()).expect("쓰기 실패"); //저장하기
    } else {}
}

fn main() {
    let mut config = File::open("usync.json");
    match config {
        Ok(mut config) => {
            println!("config load ok");
            let mut target = String::new();
            config.read_to_string(&mut target).expect("파일 읽기 실패!");
            println!("{}", target);
            
            let json_value: Value = serde_json::from_str(target).expect("JSON 파싱 실패!");
            println!("JSON 값: {:?}", json_value);
        }
        Err(e) => {match e.kind() {
                io::ErrorKind::NotFound => {eprintln!("config not found"); config_setup()}
                io::ErrorKind::PermissionDenied => {}
                _ => {eprintln!("파일 열기 실패: {}", e);} //finally 
            }}
    }
}
