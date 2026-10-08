//! # chicken_menu
//!
//!
//!
//!
//!

use std::io::Write;
use std::path::Path;
use std::{fmt, fs, io::Read};
// struct BeMusicScript;
// struct Mets;

#[derive(Clone, Debug)]
pub enum BMSReadError {
    MissingFile(String),
    FailToReadFile(String),
    IncorrectEncoding(String, String, String),
}

impl fmt::Display for BMSReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BMSReadError::MissingFile(path) => {
                write!(f, "WARNING - File not read (file does not exist): {}", path)
            }
            BMSReadError::FailToReadFile(path) => {
                write!(f, "WARNING - File can not be read: {}", path)
            }
            BMSReadError::IncorrectEncoding(path, encoding, using_encoding) => {
                write!(f, "WARNING - incorrect encoding: \n\t file {path},\n\t expect {encoding}, \n\t encoding used {using_encoding}")
            }
        }
    }
}

/// # bms metadata parser
///
/// 패턴에 대한 메타데이터 읽는 함수
/// data, action, calc 중 calc
///
/// bms파일, 인코딩 입력한 값 -> data
///
/// |                          bms file                          |     |    mets file    |
/// | :--------------------------------------------------------: | :-: | :-------------: |
/// |                        hash value?                         |     | mets::ID, OBJID |
/// | files(BMP, WAV, STAGEFILE, BANNER, BACKBMP, CHARFILE, ...) |     |     fileSec     |
///
///
///

pub fn read_bms_file(bms_path: &str, encoding_name: &str) -> Result<(), BMSReadError> {
    use encoding_rs::Encoding;
    use log::debug;
    use std::io::Write;

    // 입력한 인코딩 이름으로 인코딩 정의
    let encoding: &Encoding =
        Encoding::for_label(encoding_name.as_bytes()).unwrap_or_else(|| encoding_rs::UTF_8);

    // 경로에 있는 파일이 유효한지 확인
    let mut bms_file = match fs::File::open(bms_path) {
        Err(error_info) => {
            debug!("std::io error {}", error_info);
            return Err(BMSReadError::MissingFile(String::from(bms_path)));
        }
        Ok(read_file) => {
            debug!("read {:#?}", read_file.metadata());
            read_file
        }
    };

    debug!("{:?}", bms_file);

    let file_size = match bms_file.metadata() {
        Err(error) => {
            debug!("fail getting file size, {}", error);
            return Err(BMSReadError::FailToReadFile(String::from(bms_path)));
        }
        Ok(metadata) => metadata.len() as usize,
    };

    // 파일 읽고 인코딩에 따라 디코딩
    let mut buffer: Vec<u8> = vec![0; file_size];

    bms_file.read_to_end(&mut buffer).expect("fail to read");

    let (cow, encoding_used, had_errors) = encoding.decode(&buffer);

    // 디코딩한 결과에 따라 error, metadata 반환
    if had_errors {
        debug!("error , { }", encoding_used.name());
        Err(BMSReadError::IncorrectEncoding(
            String::from(bms_path),
            String::from(encoding_name),
            String::from(encoding_used.name()),
        ))
    } else {
        use log::debug;
        use regex::Regex;
        let name_regex = Regex::new(r"([^/]*)\.[^/]*$").expect("wow");
        debug!("{:?}", name_regex.captures(bms_path));
        let Some(file_name) = name_regex.captures(bms_path) else {
            return Err(BMSReadError::FailToReadFile(String::from("regex error")));
        };
        let output_file_path = format!("./test_resource/result/{}.readed", &file_name[1]);
        let mut out_put = match fs::File::create_new(&output_file_path) {
            Ok(file) => file,
            Err(_) => return Err(BMSReadError::FailToReadFile(output_file_path.clone())),
        };
        for (num, line) in cow.lines().enumerate() {
            write!(out_put, "num {} -> {}\n", num, line).expect("can not write");
        }
        Ok(())
    }
}

/// # 파일 행 출력

pub fn print_file_row(file_path: &str) -> Result<String, BMSReadError> {
    use log::debug;
    use magic;
    use sha3::Digest;
    let file_path = Path::new(file_path);
    // 경로에 있는 파일이 유효한지 확인
    let target_file = match fs::File::open(file_path) {
        Err(error_info) => {
            debug!("std::io error {}", error_info);
            return Err(BMSReadError::MissingFile(String::from(
                file_path.to_str().unwrap(),
            )));
        }
        Ok(read_file) => {
            debug!("read {:#?}", read_file.metadata());
            read_file
        }
    };
    debug!("{:?}", target_file);

    let file_metadata = match target_file.metadata() {
        Err(error) => {
            debug!("fail getting file size, {}", error);
            return Err(BMSReadError::FailToReadFile(String::from(
                file_path.to_str().unwrap(),
            )));
        }
        Ok(metadata) => metadata,
    };
    let mut hasher = sha3::Sha3_256::new();
    hasher.update(&fs::read(file_path).expect("fail to read"));
    let ipfs_path = "";
    let result_cid = /*if cfg!(target_os = "windows")*/ {
        std::process::Command::new("pwsh")
            .arg("-Command")
            .arg(format!(
                "{ipfs_path} add --offline --only-hash \"{}\"",
                file_path.to_str().unwrap()
            ))
            .output()
            .expect("fail to run pwsh")
    } ;
    let parsing_pattern = regex::Regex::new(r"added (\w*) .*").unwrap();
    let cid = String::from_utf8(result_cid.stdout).unwrap();

    let parsed_cid = parsing_pattern.captures(cid.as_str()).unwrap()[1].to_string();
    let flags = magic::cookie::Flags::MIME_TYPE | magic::cookie::Flags::MIME_ENCODING;
    let cookie = magic::Cookie::open(flags).unwrap();

    let magic_db = ["D:/Tool/magic"].try_into().unwrap();
    let cookie = cookie.load(&magic_db).unwrap();
    let mime_type = cookie.file(file_path).unwrap();
    let (mime_type, encoding) = if let Some((mime, encoding)) = mime_type.split_once("; charset=") {
        (mime, encoding)
    } else {
        (mime_type.as_str(), "")
    };
    Ok(format!(
        "{filename},{filesize},{mimetype},{encoding_type},SHA3-256,{hash_value},{cid}",
        filename = file_path.file_name().unwrap().to_str().unwrap(),
        mimetype = mime_type,
        encoding_type = encoding,
        filesize = file_metadata.len(),
        hash_value = hasher
            .finalize()
            .to_vec()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>(),
        cid = parsed_cid
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;
    #[test]
    fn test_intention_case() -> Result<(), BMSReadError> {
        use log::debug;
        let bms_path = "test_resource/";
        let encoding_input = "ASCII";
        let result = read_bms_file(bms_path, encoding_input);
        match result {
            Ok(_) => Ok(()),
            Err(some_err) => {
                debug!("error description : {}", some_err);
                panic!("{}", some_err)
            }
        }
    }

    #[test]
    fn test_non_ascii_case() -> Result<(), BMSReadError> {
        use log::debug;
        let bms_path = "test_resource/";
        let encoding_input = "SHIFT-JIS";
        let result = read_bms_file(bms_path, encoding_input);
        match result {
            Ok(_) => Ok(()),
            Err(some_err) => {
                debug!("error description : {}", some_err);
                panic!("{}", some_err)
            }
        }
    }
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);

    let current_path = Path::new(args[1].as_str());
    let out_f_str = format!(
        "./test_resource/result/{}.readed",
        current_path.file_name().unwrap().to_str().unwrap()
    );
    let output_file_path = Path::new(out_f_str.as_str());
    let mut out_put = match fs::File::create(&output_file_path) {
        Ok(file) => file,
        Err(msg) => {
            println!("{:?}", msg);
            return;
        }
    };
    for entry in current_path.read_dir().unwrap() {
        let file_row = match print_file_row(entry.unwrap().path().to_str().unwrap()) {
            Ok(row) => row,
            Err(err) => {
                println!("{:?}", err);
                continue;
            }
        };

        out_put.write(format!("{}\n", file_row).as_bytes()).unwrap();
    }
}
