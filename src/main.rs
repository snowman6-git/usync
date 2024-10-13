use rfd::FileDialog;
use std::fs::File;
use std::io::Write;

fn main() {
    

    if let Some(folder_path) = FileDialog::new().pick_folder() {
        let target = folder_path.display();
        println!("선택한 폴더: {}", target);
        
        // 선택한 폴더 내의 파일 목록을 출력할 수 있음
        if let Ok(entries) = std::fs::read_dir(folder_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    println!("{}", entry.file_name().to_string_lossy());
                }
            }
        }

    } else {
        println!("폴더가 선택되지 않았습니다.");
    }
}
