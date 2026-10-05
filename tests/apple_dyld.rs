#![cfg(any(target_os = "macos", target_os = "ios"))]
#![allow(deprecated)]

// These APIs are public on iOS since 2.0 (mach-o/dyld.h), not macOS-only.
#[test]
fn dyld_image_api_signatures() {
    let _: unsafe extern "C" fn() -> u32 = libc::_dyld_image_count;
    let _: unsafe extern "C" fn(u32) -> *const libc::mach_header = libc::_dyld_get_image_header;
    let _: unsafe extern "C" fn(u32) -> libc::intptr_t = libc::_dyld_get_image_vmaddr_slide;
    let _: unsafe extern "C" fn(u32) -> *const libc::c_char = libc::_dyld_get_image_name;
    assert_eq!(core::mem::size_of::<libc::mach_header>(), 28);
    assert_eq!(core::mem::size_of::<libc::mach_header_64>(), 32);
}
