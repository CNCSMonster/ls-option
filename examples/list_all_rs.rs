fn main() {
    use ls_option::*;
    let fs = ListOption::default()
        .file(true)
        .dir(false)
        .unhidden(true)
        .hidden(false)
        .recursive(true)
        .sufs(vec![".rs"])
        .list(".");
    dbg!(fs);
}
