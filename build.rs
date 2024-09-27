use std::process::Command;

fn main() {
    // Create a build directory
    Command::new("sh")
        .arg("-c")
        // .arg("mkdir -p build && cd build && cmake .. && make && cp libdata.so ../rust_call_cpp")
        .arg("mkdir -p build && cd build && cmake .. && make")
        .status()
        .expect("Failed to execute build commands");

    Command::new("sh")
        .arg("-c")
        .arg("export LD_LIBRARY_PATH=build")
        .status()
        .expect("failed set env");

    println!("===================");
    // Link the library if needed (adjust the name as necessary)
    println!("cargo:rustc-link-lib=data");
    println!("cargo:rustc-link-search=build"); // Adjust to where the output is
}
