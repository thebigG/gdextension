/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot_ffi as sys;

use crate::builtin::meta::VariantMetadata;
use crate::builtin::{FromVariant, StringName, ToVariant, Variant, Vector2};
use crate::obj::{Base, Gd};
use godot_ffi::{GDNativeTypePtr, GDNativeVariantType, TagType, VariantType, GDEXTENSION_VARIANT_TYPE_ARRAY, GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY, GDEXTENSION_VARIANT_TYPE_VECTOR2, GDNativeObjectPtr};
use std::marker::PhantomData;
use sys::{ffi_methods, interface_fn, types::*, GodotFfi};

use crate::engine::Object as Obj;


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
impl_builtin_froms!(Int64Array; Array =>  packed_int64_array_from_array);
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

    pub fn new() -> Array {
        // For now, this unnit business seems to be required. But I'd like to study it more
        //and really understand what it does.
        let mut uninit = std::mem::MaybeUninit::<Array>::uninit();
        unsafe {
            let self_ptr = (*uninit.as_mut_ptr()).sys_mut();
            let ctor = sys::method_table().array_construct_default;
            ctor(self_ptr, std::ptr::null_mut());

            uninit.assume_init()
        }
    }
    // pub fn append(&mut self, v: Vector2) -> () {
    //     unsafe {
    //         // let obj_class_name = StringName::from("Variant");
    //         // let obj_method_name = StringName::from("append");
    //         // Obj::call("", "");

    //         let class_name = StringName::from("PackedVector2Array");
    //         let method_name = StringName::from("append");
    //         let method_bind = {
    //             unsafe {
    //                 ::godot_ffi::get_interface()
    //                     .classdb_get_method_bind
    //                     .unwrap_unchecked()
    //             }
    //         }(
    //             class_name.string_sys(),
    //             method_name.string_sys(),
    //             4188891560i64,
    //         );
    //         let call_fn = {
    //             unsafe {
    //                 ::godot_ffi::get_interface()
    //                     .object_method_bind_ptrcall
    //                     .unwrap_unchecked()
    //             }
    //         };
    //         let args = [
    //             <Vector2 as sys::GodotFfi>::sys(&v),
    //         ];
    //         let args_ptr = args.as_ptr();
    //         <Vector2Array as sys::GodotFfi>::from_sys_init(|return_ptr| {
    //             call_fn(method_bind, self.sys() as GDNativeObjectPtr, args_ptr, return_ptr);
    //         });

    //     }
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
}

impl<T> Clone for TypedArray<T> {
    fn clone(&self) -> Self {
        unsafe {
            Self::from_sys_init(|self_ptr| {
                let ctor = ::godot_ffi::builtin_fn!(array_construct_copy);
                let args = [self.sys_const()];
                ctor(self_ptr, args.as_ptr());
            })
        }
    }
}

// TODO enable this:
// impl_builtin_traits! {
//     for TypedArray<T> {
//         Clone => array_construct_copy;
//         Drop => array_destroy;
//     }
// }

impl<T> GodotFfi for TypedArray<T> {
    ffi_methods! { type sys::GDExtensionTypePtr = *mut Opaque; .. }
}

impl<T> Drop for TypedArray<T> {
    fn drop(&mut self) {
        unsafe {
            let destructor = sys::builtin_fn!(array_destroy @1);
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
}
