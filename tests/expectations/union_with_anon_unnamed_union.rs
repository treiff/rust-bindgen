/* automatically generated by rust-bindgen */


#![feature(const_fn)]
#![allow(non_snake_case)]


#[derive(Copy, Debug)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl <T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self { __BindgenUnionField(::std::marker::PhantomData) }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T { ::std::mem::transmute(self) }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T { ::std::mem::transmute(self) }
}
impl <T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self { Self::new() }
}
impl <T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self { Self::new() }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Union_foo {
    pub a: __BindgenUnionField<::std::os::raw::c_uint>,
    pub foo_union_with_anon_unnamed_union_h_unnamed_1: __BindgenUnionField<Union_foo_union_with_anon_unnamed_union_h_unnamed_1>,
    pub _bindgen_data_: u32,
}
impl Union_foo {
    pub unsafe fn a(&mut self) -> *mut ::std::os::raw::c_uint {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn foo_union_with_anon_unnamed_union_h_unnamed_1(&mut self)
     -> *mut Union_foo_union_with_anon_unnamed_union_h_unnamed_1 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_foo {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Union_foo() {
    assert_eq!(::std::mem::size_of::<Union_foo>() , 4usize);
    assert_eq!(::std::mem::align_of::<Union_foo>() , 4usize);
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Union_foo_union_with_anon_unnamed_union_h_unnamed_1 {
    pub b: __BindgenUnionField<::std::os::raw::c_ushort>,
    pub c: __BindgenUnionField<::std::os::raw::c_uchar>,
    pub _bindgen_data_: u16,
}
impl Union_foo_union_with_anon_unnamed_union_h_unnamed_1 {
    pub unsafe fn b(&mut self) -> *mut ::std::os::raw::c_ushort {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn c(&mut self) -> *mut ::std::os::raw::c_uchar {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for
 Union_foo_union_with_anon_unnamed_union_h_unnamed_1 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Union_foo_union_with_anon_unnamed_union_h_unnamed_1() {
    assert_eq!(::std::mem::size_of::<Union_foo_union_with_anon_unnamed_union_h_unnamed_1>()
               , 2usize);
    assert_eq!(::std::mem::align_of::<Union_foo_union_with_anon_unnamed_union_h_unnamed_1>()
               , 2usize);
}
