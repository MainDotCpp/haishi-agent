fn main() {
    let file_dir = "admin/test.rs";
    let prefix = "admin";
    println!("{}", file_dir.starts_with(prefix));
    let file_dir = file_dir.strip_prefix(prefix).unwrap();
    print!("{}", file_dir);
}