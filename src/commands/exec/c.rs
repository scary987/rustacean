use std::path::PathBuf;

use crate::commands::exec::language::Language;

#[derive(Debug)]
pub struct C;

impl Language for C {
    fn get_image_name(&self) -> String {
        "rustacean-c".into()
    }

    fn get_lang_name(&self) -> String {
        "C".into()
    }

    fn get_source_file_ext(&self) -> String {
        ".c".into()
    }

    fn pre_process_code(&self, code: &str, _src_path: &PathBuf) -> Option<String> {
        use regex::Regex;

        let re = Regex::new(r"(?s)(int\s+main\s*\(.*\))").unwrap();
        if !re.is_match(&code) {
            let result = format!("#include <stdio.h>\r\n#include <stdlib.h>\r\nint main(int argc, char** argv) {{\r\n{}\r\n}}", code);
            return Some(result);
        }

        None
    }

    fn get_compiler_command(&self, src_path: &PathBuf, exe_path: &PathBuf) -> Option<String> {
        Some(format!(
            "gcc {} -o {}",
            src_path.to_str().unwrap(),
            exe_path.to_str().unwrap()
        ))
    }

    fn check_compiler_or_interpreter(&self) -> String {
        String::from("gcc --version")
    }
}
