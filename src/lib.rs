extern crate foo;
extern crate bar;

#[no_mangle]
pub unsafe extern "C" fn megazord_fn() -> () {
    println!("megazord_fn");
}

use std::vec::Vec;

#[no_mangle]
pub unsafe extern "C" fn _fns() -> Vec<*const u8> {
// #[inline(never)]
// pub unsafe fn fns() -> Vec<*const u8> {
    let mut fns = vec![
        &megazord_fn as *const _ as *const u8,
        // &bar::bar_fn as *const _ as *const u8,
    ];
    fns.append(&mut foo::fns().clone());
    fns.append(&mut bar::fns().clone());

    // for &i in fns.iter() {
    //     ::std::ptr::read_volatile::<u8>(i);
    // }
    fns
}
