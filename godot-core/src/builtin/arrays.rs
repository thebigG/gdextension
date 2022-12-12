/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot_ffi as sys;

use crate::builtin::{FromVariant, Variant};
use std::marker::PhantomData;
use godot_ffi::{GDNATIVE_VARIANT_TYPE_PACKED_VECTOR2_ARRAY, GDNATIVE_VARIANT_TYPE_VECTOR2, GDNativeTypePtr, GDNativeVariantType, TagType};
use sys::{ffi_methods, interface_fn, types::*, GodotFfi};
use crate::obj::Base;

impl_builtin_stub!(Array, OpaqueArray);
impl_builtin_stub!(ByteArray, OpaquePackedByteArray);
impl_builtin_stub!(ColorArray, OpaquePackedColorArray);
impl_builtin_stub!(Float32Array, OpaquePackedFloat32Array);
impl_builtin_stub!(Float64Array, OpaquePackedFloat64Array);
impl_builtin_stub!(Int32Array, OpaquePackedInt32Array);
impl_builtin_stub!(Int64Array, OpaquePackedInt64Array);
impl_builtin_stub!(StringArray, OpaquePackedStringArray);
impl_builtin_stub!(Vector2Array, OpaquePackedVector2Array);
impl_builtin_stub!(Vector3Array, OpaquePackedVector3Array);

impl_builtin_froms!(Array;
    ByteArray => array_from_packed_byte_array,
    ColorArray => array_from_packed_color_array,
    Float32Array => array_from_packed_float32_array,
    Float64Array => array_from_packed_float64_array,
    Int32Array => array_from_packed_int32_array,
    Int64Array => array_from_packed_int64_array,
    StringArray => array_from_packed_string_array,
    Vector2Array => array_from_packed_vector2_array,
    Vector3Array => array_from_packed_vector3_array,
);

impl_builtin_froms!(ByteArray; Array => packed_byte_array_from_array);
impl_builtin_froms!(ColorArray; Array => packed_color_array_from_array);
impl_builtin_froms!(Float32Array; Array => packed_float32_array_from_array);
impl_builtin_froms!(Float64Array; Array => packed_float64_array_from_array);
impl_builtin_froms!(Int32Array; Array => packed_int32_array_from_array);
impl_builtin_froms!(Int64Array; Array => packed_int64_array_from_array);
impl_builtin_froms!(StringArray; Array => packed_string_array_from_array);
impl_builtin_froms!(Vector2Array; Array => packed_vector2_array_from_array);
impl_builtin_froms!(Vector3Array; Array => packed_vector3_array_from_array);

impl Array {
    pub fn get(&self, index: i64) -> Option<Variant> {
        unsafe {
            let ptr = (interface_fn!(array_operator_index))(self.sys(), index) as *mut Variant;
            if ptr.is_null() {
                return None;
            }
            Some((*ptr).clone())
        }
    }



    //     pub fn new(&self) -> i32
    //     {
    //     // unsafe {
    //     //     let ptr = (interface_fn!(variant_get_ptr_constructor))(Self) as *mut Variant;
    //     //     if ptr.is_null() {
    //     //         return None;
    //     //     }
    //     //     Some((*ptr).clone())
    //     // }
    //          100
    // }

}

#[repr(C)]
pub struct TypedArray<T> {
    opaque: OpaqueArray,
    _phantom: PhantomData<T>,
}
impl<T> TypedArray<T> {
    fn from_opaque(opaque: OpaqueArray) -> Self {
        Self {
            opaque,
            _phantom: PhantomData,
        }
    }

    // pub fn new(&self) -> Vector2Array
    //     {
    //         let array = Vector2Array::from_opaque(self.opaque);
    //         array
    //     }
}

impl<T> Clone for TypedArray<T> {
    fn clone(&self) -> Self {
        unsafe {
            Self::from_sys_init(|opaque_ptr| {
                let ctor = sys::method_table().array_construct_copy;
                ctor(opaque_ptr, &self.sys() as *const sys::GDNativeTypePtr);
            })
        }
    }
}
impl<T> GodotFfi for TypedArray<T> {
    ffi_methods! { type sys::GDNativeTypePtr = *mut Opaque; .. }
}
impl<T> Drop for TypedArray<T> {
    fn drop(&mut self) {
        unsafe {
            let destructor = sys::method_table().array_destroy;
            destructor(self.sys_mut());
        }
    }
}

impl<T: FromVariant> TypedArray<T> {
    pub fn get(&self, index: i64) -> Option<T> {
        unsafe {
            let ptr = (interface_fn!(array_operator_index))(self.sys(), index);
            let v = Variant::from_var_sys(ptr);
            T::try_from_variant(&v).ok()
        }
    }

    pub fn new() -> Self {
        unsafe {
            let args: ::std::os::raw::c_uint = 0;
            let ptr = (interface_fn!(variant_get_ptr_constructor))(GDNATIVE_VARIANT_TYPE_PACKED_VECTOR2_ARRAY, 0).unwrap_unchecked()(GDNATIVE_VARIANT_TYPE_VECTOR2 as  *mut TagType, args as *const *mut TagType );
            // let ctor = interface_fn!(ptr);
                // (self.sys(), 0);
            // ptr.
            // let v = Variant::from_var_sys(ctor);
            // T::try_from_variant(&v).ok()
            ptr as TypedArray<T>
        }
    }
}
