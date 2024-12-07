use rfd::FileDialog;
use std::fs::{read, File};
use std::io::{Write, Read};
use serde_json::{json, Value};
use std::io;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config { //json과 일치해야함
    target: String,
    version: f32
}

fn config_setup(){
    if let Some(folder_path) = FileDialog::new().pick_folder(){
        let target = folder_path.display();
        let config = File::create("usync.json").expect("파일 생성 실패!");
        let save = json! ({
            "target" : format!("{}", target),
            "version" : 0.1,
        });
        serde_json::to_writer(config, &save).expect("JSON 쓰기 실패!");
    } else {}
}

fn main() {
    let mut target = String::new();
    loop{
        let config_load = match File::open("usync.json") {
            Ok(file) => {
                let mut file = file; // file 변수를 사용할 수 있도록
                file.read_to_string(&mut target).expect("파일 읽기 실패!"); // 파일 읽기
                break;},
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => {
                    eprintln!("파일이 존재하지 않습니다. 새로운 파일을 생성합니다.");
                    config_setup();
                },
                io::ErrorKind::PermissionDenied => {
                    eprintln!("파일에 접근할 수 있는 권한이 없습니다.");
                    return; // 또는 다른 로직으로 처리
                },
                _ => {
                    eprintln!("파일 열기 실패: {}", e);
                    return; // 또는 다른 로직으로 처리
                }
            },
        };
    }
    let loot: Config = serde_json::from_str(&target).unwrap(); //&를 넣어 String이었던 target을 &str로 변환할수있다. 매우중요
    println!("파일이 성공적으로 열렸습니다. {}", target);
    if let Ok(entries) = std::fs::read_dir(loot.target) {
        for entry in entries {
            if let Ok(entry) = entry {
                println!("{}", entry.file_name().to_string_lossy());
            }
        }
    }
}