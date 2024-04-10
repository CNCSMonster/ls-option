fn main() {
    use ls_option::ListOption;
    dbg!(ListOption::default()
        .dir(true)
        .file(true)
        .hidden(false)
        .unhidden(true)
        .recursive(true)
        .level(1)
        .list("unexisted_dir"));
}
