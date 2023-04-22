pub fn envvar(var: String) -> String {
    let path = match std::env::var(&var) {
        Ok(v) => v,
        Err(e) => panic!("${} is not defined! ({})", var, e),
    };
    path
}
pub fn input() -> String {
    let mut i = String::new();
    std::io::stdin().read_line(&mut i).expect("Failed to read line!");
    i
}
pub fn args() -> Option<Vec<String>> {
    let arg: Vec<String> = std::env::args().collect();
    if arg.len() < 2 {
        Some(arg)
    } else {
        None
    }
}
