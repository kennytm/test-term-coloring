use std::ptr::read_volatile;

fn main() {
    let ptr = 1usize as *const u8;
    let value = unsafe { read_volatile(ptr) };
    println!("{}", value);
}
