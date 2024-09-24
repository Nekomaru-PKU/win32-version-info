fn main() {
    use win32_version_info::VersionInfo;
    let file_name =
        std::env::args()
            .nth(1)
            .expect("file name not provided");
    let info = VersionInfo::from_file(file_name);
    println!("{:#?}", info);
}
