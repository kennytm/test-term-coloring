/**

```
# use std::os::raw::{c_char, c_int};
# extern "C" { fn printf(_: *const c_char, ...) -> c_int; }
unsafe {
    use std::ffi::CString;
    let fmt = CString::new("test\n").unwrap();
    printf(fmt.as_ptr());
    let fmt = CString::new("number = %d\n").unwrap();
    printf(fmt.as_ptr(), 3);
    let fmt = CString::new("%d, %d\n").unwrap();
    printf(fmt.as_ptr(), 10, 5);
}
println!("woot");
```

*/

pub fn wut() {}