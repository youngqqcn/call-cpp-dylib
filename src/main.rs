#[link(name = "data", kind = "dylib")] // Adjust this based on your library
extern "C" {
    fn createData(n: std::ffi::c_int) -> *mut std::ffi::c_void;
    fn destroyData(data: *mut std::ffi::c_void);
    fn doSomething(data: *mut std::ffi::c_void);
}

fn main() {
    unsafe {
        // Create a Data instance
        let data_instance: *mut std::ffi::c_void = createData(8889);

        doSomething(data_instance);

        // Clean up
        destroyData(data_instance);
    }

    println!("Data instance created and destroyed successfully.");
}
