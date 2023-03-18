fn main() {
    let in_container = if let Some(arg1) = std::env::args().nth(1) {
        arg1 == "--container"
    } else {
        false
    };
    let interactive = if let Some(arg1) = std::env::args().nth(1) {
        arg1 == "--interactive"
    } else {
        false
    };
    if in_container {
        transpiler::transpile::run();
    } else if interactive {
        transpiler::dockerize::run_interactive();
    } else {
        transpiler::dockerize::run();
    }
}
