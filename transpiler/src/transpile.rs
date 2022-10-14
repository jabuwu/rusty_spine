use std::{
    ffi::OsStr,
    fs::{copy, create_dir_all, read_dir, read_to_string, remove_dir_all, write},
    io,
    path::Path,
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

pub fn replace_identifier(
    src: String,
    identifier: &str,
    replacement: &str,
    offset: usize,
) -> String {
    let mut slice = &src[offset..];
    let mut slice_offset = offset;
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
            return replace_identifier(new_src, identifier, replacement, src_index + 1);
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
    checks();

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

pub fn checks() {
    if !Path::new(&spine_c_dir()).is_dir() {
        println!("");
        println!(
            "Spine source not found. Please checkout spine-runtimes in the transpiler directory."
        );
        println!("");
        println!("git clone https://github.com/EsotericSoftware/spine-runtimes.git");
        println!("");
        panic!();
    }
}

pub fn c_merge_files(output: &str) {
    let src_dir = spine_c_src_dir();
    let dir = read_dir(&src_dir).unwrap();
    let mut conglomerate = String::new();
    for file in dir {
        let name = String::from(file.unwrap().file_name().to_str().unwrap());
        if name.ends_with(".c") {
            let full_path = src_dir.clone() + "/" + name.as_str();
            conglomerate += &fix_source(&name, read_to_string(&full_path).unwrap());
        }
    }
    write(output, conglomerate).unwrap();
}

pub fn fix_source(name: &str, mut src: String) -> String {
    if name == "SkeletonJson.c" {
        src = replace_identifier(src, "_spLinkedMesh", "_spLinkedMeshJson", 0);
        src = replace_identifier(src, "setBezier", "setBezierJson", 0);
        src = replace_identifier(src, "readTimeline", "readTimelineJson", 0);
        src = replace_identifier(src, "readTimeline2", "readTimeline2Json", 0);
        src = replace_identifier(src, "readSequence", "readSequenceJson", 0);
        src = replace_identifier(src, "readCurve", "readCurveJson", 0);
        src = replace_identifier(src, "_readVertices", "_readVerticesJson", 0);
        src = replace_identifier(src, "string_starts_with", "string_starts_with_json", 0);
        src
    } else if name == "SkeletonBinary.c" {
        src = replace_identifier(src, "_spLinkedMesh", "_spLinkedMeshBinary", 0);
        src = replace_identifier(src, "setBezier", "setBezierBinary", 0);
        src = replace_identifier(src, "readTimeline", "readTimelineBinary", 0);
        src = replace_identifier(src, "readTimeline2", "readTimeline2Binary", 0);
        src = replace_identifier(src, "readSequence", "readSequenceBinary", 0);
        src = replace_identifier(src, "readCurve", "readCurveBinary", 0);
        src = replace_identifier(src, "_readVertices", "_readVerticesBinary", 0);
        src = replace_identifier(src, "string_starts_with", "string_starts_with_binary", 0);
        src
    } else if name == "AnimationState.c" {
        src = replace_identifier(src, "binarySearch1", "binarySearch1_state", 0);
        src
    } else {
        src
    }
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
    src = replace_identifier(src, "memmove", "spine_memmove", 0);
    src = replace_identifier(src, "strlen", "spine_strlen", 0);
    src = replace_identifier(src, "memcpy", "spine_memcpy", 0);
    src = replace_identifier(src, "memset", "spine_memset", 0);
    src = replace_identifier(src, "strcpy", "spine_strcpy", 0);
    src = replace_identifier(src, "strcmp", "spine_strcmp", 0);
    src = replace_identifier(src, "strrchr", "spine_strrchr", 0);
    src = replace_identifier(src, "sqrtf", "spine_sqrtf", 0);
    src = replace_identifier(src, "strcasecmp", "spine_strcasecmp", 0);
    src = replace_identifier(src, "strncmp", "spine_strncmp", 0);
    src = replace_identifier(src, "strncat", "spine_strncat", 0);
    src = replace_identifier(src, "malloc", "spine_malloc", 0);
    src = replace_identifier(src, "realloc", "spine_realloc", 0);
    src = replace_identifier(src, "free", "spine_free", 0);
    src = replace_identifier(src, "strtol", "spine_strtol", 0);
    src = replace_identifier(src, "sprintf", "spine_sprintf", 0);
    src = replace_identifier(src, "printf", "spine_printf", 0);
    src = replace_identifier(src, "sscanf", "spine_sscanf", 0);
    src = replace_identifier(src, "strtoul", "spine_strtoul", 0);
    src = replace_identifier(src, "rand", "spine_rand", 0);
    src = replace_identifier(src, "fopen", "spine_fopen", 0);
    src = replace_identifier(src, "fseek", "spine_fseek", 0);
    src = replace_identifier(src, "ftell", "spine_ftell", 0);
    src = replace_identifier(src, "fread", "spine_fread", 0);
    src = replace_identifier(src, "fclose", "spine_fclose", 0);
    src = replace_identifier(src, "fclose", "spine_fclose", 0);
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
    src = src.replace(
        "fn spine_printf(__format: *const c_char, _: ...) -> c_int;",
        "",
    );
    src = src.replace(
        "fn spine_sprintf(\n        __s: *mut c_char,\n        __format: *const c_char,\n        _: ...\n    ) -> c_int;",
        "",
    );
    src = src.replace(
        "fn spine_sscanf(\n        __s: *const c_char,\n        __format: *const c_char,\n        _: ...\n    ) -> c_int;",
        "",
    );
    src = replace_identifier(src, "spine_printf", "spine_printf!", 0);
    src = replace_identifier(src, "spine_sprintf", "spine_sprintf!", 0);
    src = replace_identifier(src, "spine_sscanf", "spine_sscanf!", 0);
    write(output, src).unwrap();
}
