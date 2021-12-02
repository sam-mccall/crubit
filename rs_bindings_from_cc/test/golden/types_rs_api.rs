#![rustfmt::skip]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(const_maybe_uninit_as_ptr, const_ptr_offset_from, custom_inner_attributes)]

use memoffset_unstable_const::offset_of;
use static_assertions::const_assert_eq;

// <unknown location>
// Error while generating bindings for item 'std::__u':
// Items contained in namespaces are not supported yet

// google3/external/clang/toolchain/lib/clang/google3-trunk/include/__stddef_max_align_t.h;l=19
// Error while generating bindings for item '(anonymous struct)::':
// UNIMPLEMENTED: Unsupported type 'max_align_t *' [type.googleapis.com/devtools.rust.cc_interop.rs_binding_from_cc.type='max_align_t *']

// google3/external/clang/toolchain/lib/clang/google3-trunk/include/__stddef_max_align_t.h;l=19
// Error while generating bindings for item '(anonymous struct)::':
// The Record for method '(anonymous struct)::' could not be found

// google3/external/clang/toolchain/lib/clang/google3-trunk/include/__stddef_max_align_t.h;l=19
// Error while generating bindings for item '(anonymous struct)::':
// UNIMPLEMENTED: Unsupported type 'max_align_t *' [type.googleapis.com/devtools.rust.cc_interop.rs_binding_from_cc.type='max_align_t *']

// google3/external/clang/toolchain/lib/clang/google3-trunk/include/__stddef_max_align_t.h;l=19
// Error while generating bindings for item '(anonymous struct)::':
// Parameter type 'const max_align_t &' is not supported

// google3/external/clang/toolchain/lib/clang/google3-trunk/include/__stddef_max_align_t.h;l=19
// Error while generating bindings for item '(anonymous struct)::':
// The Record for method '(anonymous struct)::' could not be found

// google3/external/clang/toolchain/lib/clang/google3-trunk/include/__stddef_max_align_t.h;l=19
// Error while generating bindings for item '(anonymous struct)::operator=':
// UNIMPLEMENTED: Unsupported type 'max_align_t *' [type.googleapis.com/devtools.rust.cc_interop.rs_binding_from_cc.type='max_align_t *']

// google3/external/clang/toolchain/lib/clang/google3-trunk/include/__stddef_max_align_t.h;l=19
// Error while generating bindings for item '(anonymous struct)::operator=':
// Parameter type 'const max_align_t &' is not supported

// <unknown location>
// Error while generating bindings for item '(anonymous struct)::operator=':
// Return type 'max_align_t &' is not supported

// google3/external/clang/toolchain/lib/clang/google3-trunk/include/__stddef_max_align_t.h;l=19
// Error while generating bindings for item '(anonymous struct)::operator=':
// The Record for method '(anonymous struct)::operator=' could not be found

// google3/external/clang/toolchain/lib/clang/google3-trunk/include/__stddef_max_align_t.h;l=19
// Error while generating bindings for item '(anonymous struct)::':
// UNIMPLEMENTED: Unsupported type 'max_align_t *' [type.googleapis.com/devtools.rust.cc_interop.rs_binding_from_cc.type='max_align_t *']

// google3/external/clang/toolchain/lib/clang/google3-trunk/include/__stddef_max_align_t.h;l=19
// Error while generating bindings for item '(anonymous struct)::':
// Parameter type 'max_align_t &&' is not supported

// google3/external/clang/toolchain/lib/clang/google3-trunk/include/__stddef_max_align_t.h;l=19
// Error while generating bindings for item '(anonymous struct)::':
// The Record for method '(anonymous struct)::' could not be found

// google3/external/clang/toolchain/lib/clang/google3-trunk/include/__stddef_max_align_t.h;l=19
// Error while generating bindings for item '(anonymous struct)::operator=':
// UNIMPLEMENTED: Unsupported type 'max_align_t *' [type.googleapis.com/devtools.rust.cc_interop.rs_binding_from_cc.type='max_align_t *']

// google3/external/clang/toolchain/lib/clang/google3-trunk/include/__stddef_max_align_t.h;l=19
// Error while generating bindings for item '(anonymous struct)::operator=':
// Parameter type 'max_align_t &&' is not supported

// <unknown location>
// Error while generating bindings for item '(anonymous struct)::operator=':
// Return type 'max_align_t &' is not supported

// google3/external/clang/toolchain/lib/clang/google3-trunk/include/__stddef_max_align_t.h;l=19
// Error while generating bindings for item '(anonymous struct)::operator=':
// The Record for method '(anonymous struct)::operator=' could not be found

// google3/external/clang/toolchain/lib/clang/google3-trunk/include/__stddef_max_align_t.h;l=19
// Error while generating bindings for item '(anonymous struct)::~':
// UNIMPLEMENTED: Unsupported type 'max_align_t *' [type.googleapis.com/devtools.rust.cc_interop.rs_binding_from_cc.type='max_align_t *']

// google3/external/clang/toolchain/lib/clang/google3-trunk/include/__stddef_max_align_t.h;l=19
// Error while generating bindings for item '(anonymous struct)::~':
// The Record for method '(anonymous struct)::~' could not be found

// <unknown location>
// Error while generating bindings for item '(anonymous struct)::':
// UNIMPLEMENTED: Unsupported type '__fsid_t *' [type.googleapis.com/devtools.rust.cc_interop.rs_binding_from_cc.type='__fsid_t *']

// <unknown location>
// Error while generating bindings for item '(anonymous struct)::':
// The Record for method '(anonymous struct)::' could not be found

// <unknown location>
// Error while generating bindings for item '(anonymous struct)::':
// UNIMPLEMENTED: Unsupported type '__fsid_t *' [type.googleapis.com/devtools.rust.cc_interop.rs_binding_from_cc.type='__fsid_t *']

// <unknown location>
// Error while generating bindings for item '(anonymous struct)::':
// Parameter type 'const __fsid_t &' is not supported

// <unknown location>
// Error while generating bindings for item '(anonymous struct)::':
// The Record for method '(anonymous struct)::' could not be found

// <unknown location>
// Error while generating bindings for item '(anonymous struct)::operator=':
// UNIMPLEMENTED: Unsupported type '__fsid_t *' [type.googleapis.com/devtools.rust.cc_interop.rs_binding_from_cc.type='__fsid_t *']

// <unknown location>
// Error while generating bindings for item '(anonymous struct)::operator=':
// Parameter type 'const __fsid_t &' is not supported

// <unknown location>
// Error while generating bindings for item '(anonymous struct)::operator=':
// Return type '__fsid_t &' is not supported

// <unknown location>
// Error while generating bindings for item '(anonymous struct)::operator=':
// The Record for method '(anonymous struct)::operator=' could not be found

// <unknown location>
// Error while generating bindings for item '(anonymous struct)::':
// UNIMPLEMENTED: Unsupported type '__fsid_t *' [type.googleapis.com/devtools.rust.cc_interop.rs_binding_from_cc.type='__fsid_t *']

// <unknown location>
// Error while generating bindings for item '(anonymous struct)::':
// Parameter type '__fsid_t &&' is not supported

// <unknown location>
// Error while generating bindings for item '(anonymous struct)::':
// The Record for method '(anonymous struct)::' could not be found

// <unknown location>
// Error while generating bindings for item '(anonymous struct)::operator=':
// UNIMPLEMENTED: Unsupported type '__fsid_t *' [type.googleapis.com/devtools.rust.cc_interop.rs_binding_from_cc.type='__fsid_t *']

// <unknown location>
// Error while generating bindings for item '(anonymous struct)::operator=':
// Parameter type '__fsid_t &&' is not supported

// <unknown location>
// Error while generating bindings for item '(anonymous struct)::operator=':
// Return type '__fsid_t &' is not supported

// <unknown location>
// Error while generating bindings for item '(anonymous struct)::operator=':
// The Record for method '(anonymous struct)::operator=' could not be found

// <unknown location>
// Error while generating bindings for item '(anonymous struct)::~':
// UNIMPLEMENTED: Unsupported type '__fsid_t *' [type.googleapis.com/devtools.rust.cc_interop.rs_binding_from_cc.type='__fsid_t *']

// <unknown location>
// Error while generating bindings for item '(anonymous struct)::~':
// The Record for method '(anonymous struct)::~' could not be found

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SomeStruct {
    /// Prevent empty C++ struct being zero-size in Rust.
    placeholder: core::mem::MaybeUninit<u8>,
}

// rs_bindings_from_cc/test/golden/types.h;l=7
// Error while generating bindings for item 'SomeStruct::SomeStruct':
// Nested classes are not supported yet

// rs_bindings_from_cc/test/golden/types.h;l=7
// Error while generating bindings for item 'SomeStruct::SomeStruct':
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/types.h;l=7
// Error while generating bindings for item 'SomeStruct::operator=':
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/types.h;l=7
// Error while generating bindings for item 'SomeStruct::SomeStruct':
// Parameter type 'struct SomeStruct &&' is not supported

// rs_bindings_from_cc/test/golden/types.h;l=7
// Error while generating bindings for item 'SomeStruct::operator=':
// Parameter type 'struct SomeStruct &&' is not supported

#[derive(Clone, Copy)]
#[repr(C)]
pub struct FieldTypeTestStruct {
    pub bool_field: bool,
    pub char_field: u8,
    pub unsigned_char_field: u8,
    pub signed_char_field: i8,
    pub char16_t_field: u16,
    pub char32_t_field: u32,
    pub wchar_t_field: i32,
    pub short_field: i16,
    pub int_field: i32,
    pub long_field: i64,
    pub long_long_field: i64,
    pub unsigned_short_field: u16,
    pub unsigned_int_field: u32,
    pub unsigned_long_field: u64,
    pub unsigned_long_long_field: u64,
    pub signed_short_field: i16,
    pub signed_int_field: i32,
    pub signed_long_field: i64,
    pub signed_long_long_field: i64,
    pub int8_t_field: i8,
    pub int16_t_field: i16,
    pub int32_t_field: i32,
    pub int64_t_field: i64,
    pub uint8_t_field: u8,
    pub uint16_t_field: u16,
    pub uint32_t_field: u32,
    pub uint64_t_field: u64,
    pub ptrdiff_t_field: isize,
    pub size_t_field: usize,
    pub intptr_t_field: isize,
    pub uintptr_t_field: usize,
    pub float_field: f32,
    pub double_field: f64,
    pub ptr_field: *mut i32,
    pub struct_field: SomeStruct,
    pub struct_ptr_field: *mut SomeStruct,
    pub const_struct_ptr_field: *const SomeStruct,
    pub struct_ref_field: *mut SomeStruct,
    pub const_struct_ref_field: *const SomeStruct,
}

// rs_bindings_from_cc/test/golden/types.h;l=10
// Error while generating bindings for item 'FieldTypeTestStruct::FieldTypeTestStruct':
// Nested classes are not supported yet

// rs_bindings_from_cc/test/golden/types.h;l=10
// Error while generating bindings for item 'FieldTypeTestStruct::FieldTypeTestStruct':
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/types.h;l=10
// Error while generating bindings for item 'FieldTypeTestStruct::operator=':
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/types.h;l=10
// Error while generating bindings for item 'FieldTypeTestStruct::FieldTypeTestStruct':
// Parameter type 'struct FieldTypeTestStruct &&' is not supported

// rs_bindings_from_cc/test/golden/types.h;l=10
// Error while generating bindings for item 'FieldTypeTestStruct::operator=':
// Parameter type 'struct FieldTypeTestStruct &&' is not supported

#[inline(always)]
pub fn VoidReturningFunction() -> () {
    unsafe { crate::detail::__rust_thunk__VoidReturningFunction() }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TYPES_H_

mod detail {
    use super::*;
    extern "C" {
        pub(crate) fn __rust_constructor_thunk__SomeStruct(__this: *mut SomeStruct) -> ();
        pub(crate) fn __rust_constructor_thunk__FieldTypeTestStruct(
            __this: *mut FieldTypeTestStruct,
        ) -> ();
        pub(crate) fn __rust_thunk__VoidReturningFunction() -> ();
    }
}

const_assert_eq!(std::mem::size_of::<SomeStruct>(), 1usize);
const_assert_eq!(std::mem::align_of::<SomeStruct>(), 1usize);

const_assert_eq!(std::mem::size_of::<FieldTypeTestStruct>(), 216usize);
const_assert_eq!(std::mem::align_of::<FieldTypeTestStruct>(), 8usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, bool_field) * 8, 0usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, char_field) * 8, 8usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, unsigned_char_field) * 8, 16usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, signed_char_field) * 8, 24usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, char16_t_field) * 8, 32usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, char32_t_field) * 8, 64usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, wchar_t_field) * 8, 96usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, short_field) * 8, 128usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, int_field) * 8, 160usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, long_field) * 8, 192usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, long_long_field) * 8, 256usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, unsigned_short_field) * 8, 320usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, unsigned_int_field) * 8, 352usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, unsigned_long_field) * 8, 384usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, unsigned_long_long_field) * 8, 448usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, signed_short_field) * 8, 512usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, signed_int_field) * 8, 544usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, signed_long_field) * 8, 576usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, signed_long_long_field) * 8, 640usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, int8_t_field) * 8, 704usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, int16_t_field) * 8, 720usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, int32_t_field) * 8, 736usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, int64_t_field) * 8, 768usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, uint8_t_field) * 8, 832usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, uint16_t_field) * 8, 848usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, uint32_t_field) * 8, 864usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, uint64_t_field) * 8, 896usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, ptrdiff_t_field) * 8, 960usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, size_t_field) * 8, 1024usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, intptr_t_field) * 8, 1088usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, uintptr_t_field) * 8, 1152usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, float_field) * 8, 1216usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, double_field) * 8, 1280usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, ptr_field) * 8, 1344usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, struct_field) * 8, 1408usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, struct_ptr_field) * 8, 1472usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, const_struct_ptr_field) * 8, 1536usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, struct_ref_field) * 8, 1600usize);
const_assert_eq!(offset_of!(FieldTypeTestStruct, const_struct_ref_field) * 8, 1664usize);
