#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused)]
/* automatically generated by rust-bindgen 0.69.4 */

pub const MDNPAD: u32 = 96;
pub const MDIOVERSION: u32 = 0;
pub const MD_CLUSTER: u32 = 1;
pub const MD_RESERVE: u32 = 2;
pub const MD_AUTOUNIT: u32 = 4;
pub const MD_READONLY: u32 = 8;
pub const MD_COMPRESS: u32 = 16;
pub const MD_FORCE: u32 = 32;
pub const MD_ASYNC: u32 = 64;
pub const MD_VERIFY: u32 = 128;
pub const MD_CACHE: u32 = 256;
pub const MD_MUSTDEALLOC: u32 = 512;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type __off_t = __int64_t;
pub type u_int64_t = __uint64_t;
pub type off_t = __off_t;
pub const md_types_MD_MALLOC: md_types = 0;
pub const md_types_MD_PRELOAD: md_types = 1;
pub const md_types_MD_VNODE: md_types = 2;
pub const md_types_MD_SWAP: md_types = 3;
pub const md_types_MD_NULL: md_types = 4;
pub type md_types = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct md_ioctl {
    pub md_version:    ::std::os::raw::c_uint,
    pub md_unit:       ::std::os::raw::c_uint,
    pub md_type:       md_types,
    pub md_file:       *mut ::std::os::raw::c_char,
    pub md_mediasize:  off_t,
    pub md_sectorsize: ::std::os::raw::c_uint,
    pub md_options:    ::std::os::raw::c_uint,
    pub md_base:       u_int64_t,
    pub md_fwheads:    ::std::os::raw::c_int,
    pub md_fwsectors:  ::std::os::raw::c_int,
    pub md_label:      *mut ::std::os::raw::c_char,
    pub md_pad:        [::std::os::raw::c_int; 96usize],
}
#[test]
fn bindgen_test_layout_md_ioctl() {
    const UNINIT: ::std::mem::MaybeUninit<md_ioctl> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<md_ioctl>(),
        448usize,
        concat!("Size of: ", stringify!(md_ioctl))
    );
    assert_eq!(
        ::std::mem::align_of::<md_ioctl>(),
        8usize,
        concat!("Alignment of ", stringify!(md_ioctl))
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).md_version) as usize - ptr as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(md_ioctl),
            "::",
            stringify!(md_version)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).md_unit) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(md_ioctl),
            "::",
            stringify!(md_unit)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).md_type) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(md_ioctl),
            "::",
            stringify!(md_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).md_file) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(md_ioctl),
            "::",
            stringify!(md_file)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).md_mediasize) as usize - ptr as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(md_ioctl),
            "::",
            stringify!(md_mediasize)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).md_sectorsize) as usize - ptr as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(md_ioctl),
            "::",
            stringify!(md_sectorsize)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).md_options) as usize - ptr as usize
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(md_ioctl),
            "::",
            stringify!(md_options)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).md_base) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(md_ioctl),
            "::",
            stringify!(md_base)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).md_fwheads) as usize - ptr as usize
        },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(md_ioctl),
            "::",
            stringify!(md_fwheads)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).md_fwsectors) as usize - ptr as usize
        },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(md_ioctl),
            "::",
            stringify!(md_fwsectors)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).md_label) as usize - ptr as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(md_ioctl),
            "::",
            stringify!(md_label)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).md_pad) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(md_ioctl),
            "::",
            stringify!(md_pad)
        )
    );
}
