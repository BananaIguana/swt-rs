use std::path::Path;

fn main() -> Result<(), &'static str>
{
    let java_home = std::env::var("JDK_HOME").map_err(|_| "Environment variable 'JDK_HOME' not set.")?;

    let library_path = Path::new(&java_home).join("lib").join("server");
    let library_path = library_path.to_str().unwrap();

    println!(
        "cargo rustc -- -C link-args=\"-Wl,-rpath,{}\"",
        library_path
    );
    println!("cargo:rustc-link-search=native={}", library_path);
    println!("cargo:rustc-link-lib=dylib=jvm");

    Ok(())
}
