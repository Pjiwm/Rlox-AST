static mut HAD_ERROR: bool = false;
pub fn error(line: u32, message: &str) {
    report(line, "", message);
}

fn report(line: u32, location: &str, message: &str) {
    let location = if location.is_empty() { "0" } else { location };
    if location.is_empty() {}
    println!("Error at {}-{}: {}", line, location, message);
    println!("{}", message);
    set_error(true);
}

pub fn set_error(error: bool) {
    unsafe {
        HAD_ERROR = error;
    }
}

pub fn get_error() -> bool {
    unsafe { HAD_ERROR }
}
