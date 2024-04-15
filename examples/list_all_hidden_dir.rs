fn main() {
    use ls_option::ListOption;
    dbg!(ListOption::default()
        .dir(true)
        .file(false)
        .hidden(true)
        .unhidden(false)
        .hidden(true)
        .recursive(false)
        .level(1)
        .list("."));
}
