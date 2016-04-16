/* automatically generated by rust-bindgen */


#![feature(const_fn)]
#![allow(non_snake_case)]


pub type whatever_int_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Struct_A {
    pub b: whatever_int_t,
}
impl ::std::clone::Clone for Struct_A {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Struct_A() {
    assert_eq!(::std::mem::size_of::<Struct_A>() , 4usize);
    assert_eq!(::std::mem::align_of::<Struct_A>() , 4usize);
}
#[repr(C)]
#[derive(Debug)]
pub struct Struct_C<T> {
    pub _base: Struct_A,
    pub m_c: T,
    pub m_c_ptr: *mut T,
    pub m_c_arr: [T; 10usize],
}
#[repr(C)]
#[derive(Debug)]
pub struct Struct_D<T> {
    pub m_c: Struct_C<T>,
    pub _phantom0: ::std::marker::PhantomData<T>,
}
extern "C" {
    #[link_name = "_Z9top_levelv"]
    pub fn top_level();
    #[link_name = "_ZN8whatever11in_whateverEv"]
    pub fn in_whatever();
    #[link_name = "_ZN12_GLOBAL__N_13fooEv"]
    pub fn foo();
    #[link_name = "_ZN1w3hehEv"]
    pub fn heh() -> whatever_int_t;
    #[link_name = "_ZN1w3fooEv"]
    pub fn foo1() -> Struct_C<::std::os::raw::c_int>;
    #[link_name = "_ZN1w4barrEv"]
    pub fn barr() -> Struct_C<f32>;
}
