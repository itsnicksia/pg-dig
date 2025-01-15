#[track_caller]
pub unsafe fn print_hex_bytes(ptr: *const u8, num_bytes: usize) {
    let location = std::panic::Location::caller();
    print!("[{}:{}] ", location.file(), location.line());

    unsafe {
        for i in 0..num_bytes {
            let byte = ptr.add(i).read();
            print!("{:02X} ", byte);
        }
    }
    println!();
}

#[track_caller]
pub unsafe fn print_hex_bytes_for_test(ptr: *const u8, num_bytes: usize) {
    let location = std::panic::Location::caller();
    print!("[{}:{}] ", location.file(), location.line());
    print!("pub const TEST_BUFFER: [u8; 1024] = [");

    unsafe {
        for i in 0..num_bytes {
            let byte = ptr.add(i).read();
            print!("0x{:01X}, ", byte);
        }
    }

    println!();
}

#[macro_export]
macro_rules! stop {
    ($reason:literal) => {
        const LINE_WIDTH: usize = 80;

        println!();
        println!("{}", "=".repeat(LINE_WIDTH));
        println!(
            "[{}:{}] stopped: press enter to exit. reason: {}",
            file!(),
            line!(),
            $reason
        );
        println!("{}", "=".repeat(LINE_WIDTH));

        let mut input = String::new();

        std::io::stdin().read_line(&mut input).unwrap_or_else(|_| 0);

        std::process::exit(1);
    };
}
