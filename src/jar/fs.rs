use std::{fs::read_dir, path::Path};

use super::JarLocal;

pub fn find_jars(pth: &Path) -> Result<Vec<JarLocal>, std::io::Error> {
    let dir = read_dir(pth)?;
    Ok(dir
        .filter_map(|entry| {
            if entry.is_ok() {
                Some(entry.unwrap())
            } else {
                None
            }
        })
        .filter(|entry| entry.file_type().is_ok() && entry.file_type().unwrap().is_file())
        .filter_map(|entry| {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".jar") {
                    Some(JarLocal::from(entry.path()))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect())
}

#[cfg(test)]
mod test_jar_fs {
    use std::path::Path;

    use crate::jar::fs::find_jars;

    const JAR_DIR: &str = "tests/mods";

    #[test]
    fn test_find_jars() {
        assert_eq!(find_jars(Path::new(JAR_DIR)).unwrap().len(), 2);
    }
}
