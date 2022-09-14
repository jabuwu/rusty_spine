use std::{
    ffi::OsStr,
    fs::{copy, create_dir_all, read_dir, read_to_string, remove_dir_all, write},
    io,
    process::{Command, Output, Stdio},
};

pub fn cmd<I, S>(program: S, args: I) -> Result<Output, io::Error>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    Command::new(program)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
}

pub fn replace_identifier(src: String, identifier: &str, replacement: &str) -> String {
    let mut slice = &src[0..];
    let mut slice_offset = 0;
    while let Some(index) = slice.find(identifier) {
        let src_index = slice_offset + index;
        let before_is_alphanumeric = src_index != 0
            && src[src_index - 1..]
                .chars()
                .nth(0)
                .map_or(false, |c| c.is_alphanumeric() || c == '_');
        let after_is_alphanumeric = src_index + identifier.len() < src.len()
            && src[(src_index + identifier.len())..]
                .chars()
                .nth(0)
                .map_or(false, |c| c.is_alphanumeric() || c == '_');
        if !before_is_alphanumeric && !after_is_alphanumeric {
            let new_src = String::from(&src[0..src_index])
                + replacement
                + &src[(src_index + identifier.len())..];
            return replace_identifier(new_src, identifier, replacement);
        }
        slice = &slice[(index + 1)..];
        slice_offset = src_index + 1;
    }
    src
}

pub fn spine_c_dir() -> String {
    String::from("/spine-runtimes/spine-c")
}

pub fn spine_c_src_dir() -> String {
    spine_c_dir() + "/spine-c/src/spine"
}

pub fn spine_c_include_dir() -> String {
    spine_c_dir() + "/spine-c/include"
}

pub fn run() {
    c_merge_files("/tmp/spine-merged.c");
    c_fixes_before_preprocessor(
        "/tmp/spine-merged.c",
        "/tmp/spine-fixes-before-preprocessor.c",
    );
    c_run_preprocessor(
        "/tmp/spine-fixes-before-preprocessor.c",
        "/tmp/spine-preprocessor.c",
    );
    c_fixes_after_preprocessor(
        "/tmp/spine-preprocessor.c",
        "/tmp/spine-fixes-after-preprocessor.c",
    );
    copy("/tmp/spine-fixes-after-preprocessor.c", "/out/spine.c").unwrap();

    c2rust(
        "/tmp/spine-fixes-after-preprocessor.c",
        "/tmp/spine-c2rust.rs",
    );

    rust_fixes("/tmp/spine-c2rust.rs", "/tmp/spine-fixes.rs");
    copy("/tmp/spine-fixes.rs", "/out/spine_c.rs").unwrap();
}

pub fn c_merge_files(output: &str) {
    let src_dir = spine_c_src_dir();
    let dir = read_dir(&src_dir).unwrap();
    let mut conglomerate = String::new();
    for file in dir {
        let name = String::from(file.unwrap().file_name().to_str().unwrap());
        if name.ends_with(".c") && name != "SkeletonBinary.c" {
            let full_path = src_dir.clone() + "/" + name.as_str();
            conglomerate += &read_to_string(&full_path).unwrap();
        }
    }
    write(output, conglomerate).unwrap();
}

pub fn c_fixes_before_preprocessor(input: &str, output: &str) {
    let mut src = read_to_string(input).unwrap();
    src = src.replace("isspace", "isspace_");
    write(
        output,
        String::from("typedef double _Float128;\nint isspace_(int x) { return x <= 32; }\n") + &src,
    )
    .unwrap();
}

pub fn c_run_preprocessor(input: &str, output: &str) {
    let src_dir = spine_c_src_dir();
    let include_dir = spine_c_include_dir();
    cmd(
        "gcc",
        [
            "-E",
            input,
            "-I",
            include_dir.as_str(),
            "-I",
            src_dir.as_str(),
            "-o",
            output,
        ],
    )
    .unwrap();
}

pub fn c_fixes_after_preprocessor(input: &str, output: &str) {
    let mut src = read_to_string(input).unwrap();
    src = replace_identifier(src, "memmove", "spine_memmove");
    src = replace_identifier(src, "strlen", "spine_strlen");
    src = replace_identifier(src, "memcpy", "spine_memcpy");
    src = replace_identifier(src, "memset", "spine_memset");
    src = replace_identifier(src, "strcpy", "spine_strcpy");
    src = replace_identifier(src, "strcmp", "spine_strcmp");
    src = replace_identifier(src, "strrchr", "spine_strrchr");
    src = replace_identifier(src, "sqrtf", "spine_sqrtf");
    src = replace_identifier(src, "strcasecmp", "spine_strcasecmp");
    src = replace_identifier(src, "strncmp", "spine_strncmp");
    src = replace_identifier(src, "strncat", "spine_strncat");
    src = replace_identifier(src, "malloc", "spine_malloc");
    src = replace_identifier(src, "realloc", "spine_realloc");
    src = replace_identifier(src, "free", "spine_free");
    src = replace_identifier(src, "strtol", "spine_strtol");
    src = replace_identifier(src, "sprintf", "spine_sprintf");
    src = replace_identifier(src, "printf", "spine_printf");
    src = replace_identifier(src, "sscanf", "spine_sscanf");
    src = replace_identifier(src, "strtoul", "spine_strtoul");
    src = replace_identifier(src, "rand", "spine_rand");
    src = replace_identifier(src, "fopen", "spine_fopen");
    src = replace_identifier(src, "fseek", "spine_fseek");
    src = replace_identifier(src, "ftell", "spine_ftell");
    src = replace_identifier(src, "fread", "spine_fread");
    src = replace_identifier(src, "fclose", "spine_fclose");
    src = replace_identifier(src, "fclose", "spine_fclose");
    write(output, src).unwrap();
}

pub fn c2rust(input: &str, output: &str) {
    let _ = remove_dir_all("/tmp/build");
    create_dir_all("/tmp/build").unwrap();
    let mut input_data = read_to_string(input).unwrap();
    input_data = input_data.replace("\"/tmp/spine-fixes-before-preprocessor.c\"", "\"spine.c\"");
    write("/tmp/build/spine.c", input_data).unwrap();
    write(
        "/tmp/build/compile_commands.json",
        r#"
        [
            {
                "directory": "/tmp/build",
                "command": "/usr/bin/cc -c /tmp/build/spine.c",
                "file": "/tmp/build/spine.c"
            }
        ]
    "#,
    )
    .unwrap();
    cmd("c2rust", ["transpile", "/tmp/build/compile_commands.json"]).unwrap();
    copy("/tmp/build/spine.rs", output).unwrap();
}

pub fn rust_fixes(input: &str, output: &str) {
    let mut src = read_to_string(input).unwrap();
    src = src.replace("libc::", "");
    src = src.replace("#![register_tool(c2rust)]\n", "");
    src = src.replace(
        "#![feature(extern_types, label_break_value, register_tool)]\n",
        "",
    );
    src = src.replace("pub type _IO_wide_data;", "");
    src = src.replace("pub type _IO_codecvt;", "");
    src = src.replace("pub type _IO_marker;", "");
    src = src
        + "\n
type _IO_wide_data = u8;
type _IO_codecvt = u8;
type _IO_marker = u8;
pub use crate::c::environment::types::*;
    ";
    write(output, src).unwrap();
}
