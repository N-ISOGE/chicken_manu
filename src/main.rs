//! # chicken_menu
//!
//!
//!
//!
//!

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
fn main() {}
