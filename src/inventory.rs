use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// 디렉터리의 파일 정보를 새 결과 파일에 쓴다. 기존 결과는 덮어쓰지 않는다.
/// 행 생성이나 쓰기에 실패하면 이번 호출이 만든 불완전한 결과를 지운다.
pub fn write_inventory(
    input: &Path,
    output: &Path,
    mut file_row: impl FnMut(&Path) -> Result<String, String>,
) -> io::Result<()> {
    // 입력을 먼저 확인하고 목록을 확정해 결과 파일 자체가 입력에 섞이지 않게 한다.
    let mut files = Vec::new();
    for entry in input.read_dir()? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            files.push(entry.path());
        }
    }
    files.sort();
    if let Some(parent) = output
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent)?;
    }
    let mut result = fs::File::create_new(output)?;
    let written = (|| {
        for path in files {
            let row = file_row(&path).map_err(io::Error::other)?;
            writeln!(result, "{row}")?;
        }
        result.flush()
    })();
    drop(result);
    if let Err(error) = written {
        fs::remove_file(output).map_err(|cleanup| {
            io::Error::other(format!("{error}; 불완전한 결과 삭제 실패: {cleanup}"))
        })?;
        return Err(error);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    struct Workspace(PathBuf);

    impl Workspace {
        fn new() -> Self {
            static NEXT: AtomicU64 = AtomicU64::new(0);
            let root = std::env::temp_dir().join(format!(
                "chicken_manu_inventory_{}_{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&root).unwrap();
            Self(root)
        }
    }

    impl Drop for Workspace {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).unwrap();
        }
    }

    #[test]
    fn existing_result_is_preserved() {
        let workspace = Workspace::new();
        let output = workspace.0.join("result.readed");
        fs::write(&output, "previous result").unwrap();
        let error = write_inventory(&workspace.0, &output, |_| panic!("must not run")).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::AlreadyExists);
        assert_eq!(fs::read_to_string(output).unwrap(), "previous result");
    }

    #[test]
    fn invalid_input_does_not_create_output() {
        let workspace = Workspace::new();
        let output = workspace.0.join("result.readed");
        let result = write_inventory(&workspace.0.join("missing"), &output, |_| {
            panic!("must not run")
        });
        assert!(result.is_err());
        assert!(!output.exists());
    }

    #[test]
    fn failed_row_removes_partial_result() {
        let workspace = Workspace::new();
        fs::write(workspace.0.join("a"), "first").unwrap();
        fs::write(workspace.0.join("b"), "second").unwrap();
        let output = workspace.0.join("result.readed");
        let error = write_inventory(&workspace.0, &output, |path| {
            if path.file_name().unwrap() == "a" {
                Ok("first row".into())
            } else {
                Err("external tool failed".into())
            }
        })
        .unwrap_err();
        assert!(error.to_string().contains("external tool failed"));
        assert!(!output.exists());
    }

    #[test]
    fn writes_files_in_order_without_directories_or_own_output() {
        let workspace = Workspace::new();
        fs::create_dir(workspace.0.join("directory")).unwrap();
        fs::write(workspace.0.join("b"), "second").unwrap();
        fs::write(workspace.0.join("a"), "first").unwrap();
        let output = workspace.0.join("result.readed");
        write_inventory(&workspace.0, &output, |path| {
            Ok(path.file_name().unwrap().to_str().unwrap().into())
        })
        .unwrap();
        assert_eq!(fs::read_to_string(output).unwrap(), "a\nb\n");
    }
}
