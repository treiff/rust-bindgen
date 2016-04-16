/* automatically generated by rust-bindgen */


#![feature(const_fn)]
#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy)]
pub struct Struct_bitfield {
    pub _bitfield_1: ::std::os::raw::c_ushort,
    pub e: ::std::os::raw::c_int,
    pub _bitfield_2: ::std::os::raw::c_uint,
    pub _bitfield_3: ::std::os::raw::c_uint,
}
impl Struct_bitfield {
    pub fn set_a(&mut self, val: bool) {
        self._bitfield_1 &=
            !(((1 << (1u32 as ::std::os::raw::c_ushort)) - 1) << 0usize);
        self._bitfield_1 |= (val as ::std::os::raw::c_ushort) << 0usize;
    }
    pub fn set_b(&mut self, val: bool) {
        self._bitfield_1 &=
            !(((1 << (1u32 as ::std::os::raw::c_ushort)) - 1) << 1usize);
        self._bitfield_1 |= (val as ::std::os::raw::c_ushort) << 1usize;
    }
    pub fn set_c(&mut self, val: bool) {
        self._bitfield_1 &=
            !(((1 << (1u32 as ::std::os::raw::c_ushort)) - 1) << 2usize);
        self._bitfield_1 |= (val as ::std::os::raw::c_ushort) << 2usize;
    }
    pub fn set_at_offset_3(&mut self, val: bool) {
        self._bitfield_1 &=
            !(((1 << (1u32 as ::std::os::raw::c_ushort)) - 1) << 3usize);
        self._bitfield_1 |= (val as ::std::os::raw::c_ushort) << 3usize;
    }
    pub fn set_at_offset_4(&mut self, val: u8) {
        self._bitfield_1 &=
            !(((1 << (2u32 as ::std::os::raw::c_ushort)) - 1) << 4usize);
        self._bitfield_1 |= (val as ::std::os::raw::c_ushort) << 4usize;
    }
    pub fn set_d(&mut self, val: u8) {
        self._bitfield_1 &=
            !(((1 << (2u32 as ::std::os::raw::c_ushort)) - 1) << 6usize);
        self._bitfield_1 |= (val as ::std::os::raw::c_ushort) << 6usize;
    }
    pub const fn new_bitfield_1(a: bool, b: bool, c: bool,
                                unnamed_bitfield1: bool,
                                unnamed_bitfield2: u8, d: u8)
     -> ::std::os::raw::c_ushort {
        0 | ((a as ::std::os::raw::c_ushort) << 0u32) |
            ((b as ::std::os::raw::c_ushort) << 1u32) |
            ((c as ::std::os::raw::c_ushort) << 2u32) |
            ((unnamed_bitfield1 as ::std::os::raw::c_ushort) << 3u32) |
            ((unnamed_bitfield2 as ::std::os::raw::c_ushort) << 4u32) |
            ((d as ::std::os::raw::c_ushort) << 6u32)
    }
    pub fn set_f(&mut self, val: u8) {
        self._bitfield_2 &=
            !(((1 << (2u32 as ::std::os::raw::c_uint)) - 1) << 0usize);
        self._bitfield_2 |= (val as ::std::os::raw::c_uint) << 0usize;
    }
    pub const fn new_bitfield_2(f: u8) -> ::std::os::raw::c_uint {
        0 | ((f as ::std::os::raw::c_uint) << 0u32)
    }
    pub fn set_g(&mut self, val: u32) {
        self._bitfield_3 &=
            !(((1 << (0u32 as ::std::os::raw::c_uint)) - 1) << 0usize);
        self._bitfield_3 |= (val as ::std::os::raw::c_uint) << 0usize;
    }
    pub const fn new_bitfield_3(g: u32) -> ::std::os::raw::c_uint {
        0 | ((g as ::std::os::raw::c_uint) << 0u32)
    }
}
impl ::std::clone::Clone for Struct_bitfield {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Struct_bitfield() {
    assert_eq!(::std::mem::size_of::<Struct_bitfield>() , 16usize);
    assert_eq!(::std::mem::align_of::<Struct_bitfield>() , 4usize);
}
