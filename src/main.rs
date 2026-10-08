#![doc = include_str!("../docs/index.md")]

pub mod docs;

use std::io::{Read, Write};
use std::path::Path;
use std::{fmt, fs};
// struct BeMusicScript;
// struct Mets;

/// 결과 파일(`*.readed`)을 쓰는 기본 폴더.
const DEFAULT_OUTPUT_DIR: &str = "./test_resource/result";

/// 외부 `ipfs` 실행 파일을 바꿀 때 쓰는 환경 변수. 없으면 `PATH`의 `ipfs`를 쓴다.
const IPFS_BIN_ENV: &str = "CHICKEN_MANU_IPFS";

#[derive(Clone, Debug)]
pub enum BMSReadError {
    MissingFile(String),
    FailToReadFile(String),
    IncorrectEncoding(String, String, String),
    /// 외부 도구(ipfs, libmagic) 호출에 실패함. (파일 경로, 이유)
    ToolFailure(String, String),
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
                write!(
                    f,
                    "WARNING - incorrect encoding: \n\t file {path},\n\t expect {encoding}, \n\t encoding used {using_encoding}"
                )
            }
            BMSReadError::ToolFailure(path, reason) => {
                write!(
                    f,
                    "WARNING - external tool failed: \n\t file {path},\n\t {reason}"
                )
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
/// 결과는 기본 폴더(`./test_resource/result`)에 `<파일 이름>.readed`로 쓴다.
pub fn read_bms_file(bms_path: &str, encoding_name: &str) -> Result<(), BMSReadError> {
    read_bms_file_to(bms_path, encoding_name, Path::new(DEFAULT_OUTPUT_DIR))
}

/// [`read_bms_file`]과 같지만 결과를 쓸 폴더를 정한다. 폴더가 없으면 만든다.
pub fn read_bms_file_to(
    bms_path: &str,
    encoding_name: &str,
    output_dir: &Path,
) -> Result<(), BMSReadError> {
    use encoding_rs::Encoding;
    use log::debug;

    // 입력한 인코딩 이름으로 인코딩 정의
    let encoding: &Encoding =
        Encoding::for_label(encoding_name.as_bytes()).unwrap_or(encoding_rs::UTF_8);

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
    // `read_to_end`는 기존 내용 뒤에 붙이므로 길이 0인 버퍼에 용량만 예약한다.
    let mut buffer: Vec<u8> = Vec::with_capacity(file_size);
    if let Err(error) = bms_file.read_to_end(&mut buffer) {
        debug!("fail to read, {}", error);
        return Err(BMSReadError::FailToReadFile(String::from(bms_path)));
    }

    let (cow, encoding_used, had_errors) = encoding.decode(&buffer);

    // 디코딩한 결과에 따라 error, metadata 반환
    if had_errors {
        debug!("error , {}", encoding_used.name());
        return Err(BMSReadError::IncorrectEncoding(
            String::from(bms_path),
            String::from(encoding_name),
            String::from(encoding_used.name()),
        ));
    }

    // 플랫폼의 경로 규칙으로 파일 이름(확장자 제외)을 구한다.
    let Some(file_stem) = Path::new(bms_path).file_stem().and_then(|s| s.to_str()) else {
        return Err(BMSReadError::FailToReadFile(String::from(bms_path)));
    };
    if let Err(error) = fs::create_dir_all(output_dir) {
        debug!("fail creating output dir, {}", error);
        return Err(BMSReadError::FailToReadFile(
            output_dir.to_string_lossy().into_owned(),
        ));
    }
    let output_file_path = output_dir.join(format!("{file_stem}.readed"));
    let output_file_name = output_file_path.to_string_lossy().into_owned();
    let mut out_put = match fs::File::create_new(&output_file_path) {
        Ok(file) => file,
        Err(_) => return Err(BMSReadError::FailToReadFile(output_file_name)),
    };
    for (num, line) in cow.lines().enumerate() {
        if let Err(error) = writeln!(out_put, "num {} -> {}", num, line) {
            debug!("can not write, {}", error);
            return Err(BMSReadError::FailToReadFile(output_file_name));
        }
    }
    Ok(())
}

/// CSV 필드를 큰따옴표로 감싸고 내부 큰따옴표는 두 번 쓴다(RFC 4180).
/// 파일명에 쉼표가 있어도 열 수가 바뀌지 않게 한다.
fn csv_field(value: &str) -> String {
    format!("\"{}\"", value.replace('"', "\"\""))
}

/// `ipfs add --offline --only-hash`로 파일의 CID를 구한다.
///
/// 셸을 거치지 않고 `ipfs`를 직접 실행하며 경로는 별도 인자로 넘긴다.
fn ipfs_cid(file_path: &Path) -> Result<String, BMSReadError> {
    let path_string = file_path.to_string_lossy().into_owned();
    let ipfs = std::env::var(IPFS_BIN_ENV).unwrap_or_else(|_| String::from("ipfs"));
    let output = std::process::Command::new(&ipfs)
        .args(["add", "--offline", "--only-hash", "--"])
        .arg(file_path)
        .output()
        .map_err(|error| {
            BMSReadError::ToolFailure(path_string.clone(), format!("{ipfs} 실행 실패: {error}"))
        })?;
    if !output.status.success() {
        return Err(BMSReadError::ToolFailure(
            path_string,
            format!(
                "{ipfs} 종료 상태 {}: {}",
                output.status,
                String::from_utf8_lossy(&output.stderr).trim()
            ),
        ));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsing_pattern = regex::Regex::new(r"added (\w+) ").expect("constant pattern");
    match parsing_pattern.captures(&stdout) {
        Some(captures) => Ok(captures[1].to_string()),
        None => Err(BMSReadError::ToolFailure(
            path_string,
            String::from("ipfs 출력에서 CID를 찾지 못함"),
        )),
    }
}

/// libmagic으로 (MIME 타입, 문자 인코딩)을 구한다.
///
/// DB는 시스템 기본값을 쓰고, 바꾸려면 libmagic의 `MAGIC` 환경 변수를 설정한다.
fn mime_and_charset(file_path: &Path) -> Result<(String, String), BMSReadError> {
    let path_string = file_path.to_string_lossy().into_owned();
    let fail = |step: &str, error: &dyn fmt::Display| {
        BMSReadError::ToolFailure(
            path_string.clone(),
            format!("libmagic {step} 실패: {error}"),
        )
    };
    let flags = magic::cookie::Flags::MIME_TYPE | magic::cookie::Flags::MIME_ENCODING;
    let cookie = magic::Cookie::open(flags).map_err(|error| fail("열기", &error))?;
    let cookie = cookie
        .load(&magic::cookie::DatabasePaths::default())
        .map_err(|error| fail("DB 불러오기", &error))?;
    let result = cookie
        .file(file_path)
        .map_err(|error| fail("판별", &error))?;
    Ok(match result.split_once("; charset=") {
        Some((mime, encoding)) => (mime.to_string(), encoding.to_string()),
        None => (result, String::new()),
    })
}

/// # 파일 행 출력
///
/// `파일명,크기,MIME,인코딩,SHA3-256,해시,CID` 한 줄을 만든다.
/// 파일명은 CSV 규칙대로 인용한다.
pub fn print_file_row(file_path: &str) -> Result<String, BMSReadError> {
    use log::debug;
    use sha3::Digest;
    let file_path = Path::new(file_path);
    let path_string = file_path.to_string_lossy().into_owned();
    // 경로에 있는 파일이 유효한지 확인
    let target_file = match fs::File::open(file_path) {
        Err(error_info) => {
            debug!("std::io error {}", error_info);
            return Err(BMSReadError::MissingFile(path_string));
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
            return Err(BMSReadError::FailToReadFile(path_string));
        }
        Ok(metadata) => metadata,
    };
    // 하위 디렉터리는 열 수는 있어도 파일로 읽을 수 없다.
    if !file_metadata.is_file() {
        return Err(BMSReadError::FailToReadFile(path_string));
    }
    let bytes = match fs::read(file_path) {
        Ok(bytes) => bytes,
        Err(error) => {
            debug!("fail to read, {}", error);
            return Err(BMSReadError::FailToReadFile(path_string));
        }
    };
    let mut hasher = sha3::Sha3_256::new();
    hasher.update(&bytes);

    let cid = ipfs_cid(file_path)?;
    let (mime_type, encoding) = mime_and_charset(file_path)?;
    let Some(filename) = file_path.file_name() else {
        return Err(BMSReadError::FailToReadFile(path_string));
    };
    Ok(format!(
        "{filename},{filesize},{mimetype},{encoding_type},SHA3-256,{hash_value},{cid}",
        filename = csv_field(&filename.to_string_lossy()),
        mimetype = mime_type,
        encoding_type = encoding,
        filesize = file_metadata.len(),
        hash_value = hasher
            .finalize()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>(),
        cid = cid
    ))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);

    let Some(input) = args.get(1) else {
        eprintln!("usage: chicken_manu <directory>");
        return;
    };
    let current_path = Path::new(input.as_str());
    let Some(dir_name) = current_path.file_name().and_then(|n| n.to_str()) else {
        eprintln!("invalid directory: {input}");
        return;
    };
    let output_dir = Path::new(DEFAULT_OUTPUT_DIR);
    if let Err(msg) = fs::create_dir_all(output_dir) {
        println!("{:?}", msg);
        return;
    }
    let output_file_path = output_dir.join(format!("{dir_name}.readed"));
    let mut out_put = match fs::File::create(&output_file_path) {
        Ok(file) => file,
        Err(msg) => {
            println!("{:?}", msg);
            return;
        }
    };
    let entries = match current_path.read_dir() {
        Ok(entries) => entries,
        Err(msg) => {
            println!("{:?}", msg);
            return;
        }
    };
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(msg) => {
                println!("{:?}", msg);
                continue;
            }
        };
        let file_row = match print_file_row(&entry.path().to_string_lossy()) {
            Ok(row) => row,
            Err(err) => {
                println!("{:?}", err);
                continue;
            }
        };

        if let Err(msg) = writeln!(out_put, "{file_row}") {
            println!("{:?}", msg);
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use test_log::test;

    /// 테스트마다 따로 쓰는 임시 폴더. 끝나면 지운다.
    struct TempDir(PathBuf);

    impl TempDir {
        fn new(name: &str) -> Self {
            let path =
                std::env::temp_dir().join(format!("chicken_manu_{}_{}", std::process::id(), name));
            let _ = fs::remove_dir_all(&path);
            fs::create_dir_all(&path).unwrap();
            TempDir(path)
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    /// `song.bms`를 만들어 읽고, (결과, 만들어진 결과 파일의 내용)을 돌려준다.
    /// 결과 폴더는 일부러 만들어 두지 않는다.
    fn read_case(name: &str, bytes: &[u8], encoding: &str) -> (Result<(), BMSReadError>, String) {
        let dir = TempDir::new(name);
        let bms_path = dir.0.join("song.bms");
        fs::write(&bms_path, bytes).unwrap();
        let output_dir = dir.0.join("result");
        let result = read_bms_file_to(bms_path.to_str().unwrap(), encoding, &output_dir);
        let output = fs::read_to_string(output_dir.join("song.readed")).unwrap_or_default();
        (result, output)
    }

    #[test]
    fn ascii_case() {
        let (result, output) = read_case("ascii", b"#TITLE test\n#ARTIST a\n", "ASCII");
        assert!(result.is_ok(), "{:?}", result);
        // 첫 줄이 NUL 등으로 손상되지 않았는지도 본다.
        assert_eq!(output, "num 0 -> #TITLE test\nnum 1 -> #ARTIST a\n");
    }

    #[test]
    fn non_ascii_case() {
        let (bytes, _, _) = encoding_rs::SHIFT_JIS.encode("#TITLE 日本語\n");
        let (result, output) = read_case("sjis", &bytes, "SHIFT-JIS");
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!(output, "num 0 -> #TITLE 日本語\n");
    }

    #[test]
    fn wrong_encoding_case() {
        let (bytes, _, _) = encoding_rs::SHIFT_JIS.encode("#TITLE 日本語\n");
        let (result, output) = read_case("wrong", &bytes, "UTF-8");
        assert!(matches!(result, Err(BMSReadError::IncorrectEncoding(..))));
        assert!(output.is_empty());
    }

    #[test]
    fn missing_file_case() {
        let dir = TempDir::new("missing");
        let missing = dir.0.join("none.bms");
        let result = read_bms_file_to(missing.to_str().unwrap(), "UTF-8", &dir.0.join("result"));
        assert!(matches!(result, Err(BMSReadError::MissingFile(_))));
    }

    #[test]
    fn directory_is_not_a_file_row() {
        let dir = TempDir::new("dir_row");
        let result = print_file_row(dir.0.to_str().unwrap());
        assert!(matches!(result, Err(BMSReadError::FailToReadFile(_))));
    }

    #[test]
    fn csv_field_quotes_commas_and_quotes() {
        assert_eq!(csv_field("kick.wav"), "\"kick.wav\"");
        assert_eq!(csv_field("song,edit.wav"), "\"song,edit.wav\"");
        assert_eq!(csv_field("a\"b.wav"), "\"a\"\"b.wav\"");
    }
}
