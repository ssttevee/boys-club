fn main() {
    let project_dir = std::env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let out_path = std::path::Path::new(&out_dir);

    let mut files: Vec<String> = vec![];
    for file in std::fs::read_dir("resources").unwrap() {
        if let Ok(entry) = file {
            if let Some(path) = entry.path().to_str() {
                files.push(format!("include_bytes!(\"{}/{}\")", project_dir.to_str().unwrap(), String::from(path)).replace("\\", "/"));
            }
        }
    }

    let dest_path = out_path.join("pepes.rs");
    std::fs::write(
        dest_path,
        format!(
            "const PEPES: &[&[u8]] = &[{}];",
            files.join(","),
        ),
    ).unwrap();
    
    println!("cargo:rerun-if-changed=resources/*");
}
