#[no_mangle]
pub unsafe extern "C" fn bar_fn() -> () {
    println!("bar_fn");
}

use std::vec::Vec;

pub fn fns() -> Vec<*const u8> {
    vec![
        &bar_fn as *const _ as *const u8,
    ]
}
