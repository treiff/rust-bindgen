/* automatically generated by rust-bindgen */


#![feature(const_fn)]
#![allow(non_snake_case)]


#[repr(C)]
pub struct opaque {
    pub _bindgen_opaque_blob: u32,
}
#[test]
fn bindgen_test_layout_opaque() {
    assert_eq!(::std::mem::size_of::<opaque>() , 4usize);
    assert_eq!(::std::mem::align_of::<opaque>() , 4usize);
}
#[repr(C)]
pub struct Struct_container {
    pub contained: u32,
}
#[test]
fn bindgen_test_layout_Struct_container() {
    assert_eq!(::std::mem::size_of::<Struct_container>() , 4usize);
    assert_eq!(::std::mem::align_of::<Struct_container>() , 4usize);
}
