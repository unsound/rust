use std::marker::PhantomData;
use std::fmt;

use context::Context;
use context;
use object;
use object::{Object, ToObject};
use structs::{self, Struct};

use gccjit_sys::gcc_jit_types::*;

/// A representation of a type, as it is known to the JIT compiler.
/// Types can be created through the Typeable trait or they can
/// be created dynamically by composing Field types.
#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct Type<'ctx> {
    marker: PhantomData<&'ctx Context<'ctx>>,
    ptr: *mut gccjit_sys::gcc_jit_type
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct VectorType<'ctx> {
    marker: PhantomData<&'ctx Context<'ctx>>,
    ptr: *mut gccjit_sys::gcc_jit_vector_type
}

impl<'ctx> VectorType<'ctx> {
    unsafe fn from_ptr(ptr: *mut gccjit_sys::gcc_jit_vector_type) -> VectorType<'ctx> {
        VectorType {
            marker: PhantomData,
            ptr
        }
    }

    pub fn get_element_type(&self) -> Type<'ctx> {
        unsafe {
            from_ptr(gccjit_sys::gcc_jit_vector_type_get_element_type(self.ptr))
        }
    }

    pub fn get_num_units(&self) -> usize {
        unsafe {
            gccjit_sys::gcc_jit_vector_type_get_num_units(self.ptr) as usize
        }
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct FunctionPtrType<'ctx> {
    marker: PhantomData<&'ctx Context<'ctx>>,
    ptr: *mut gccjit_sys::gcc_jit_function_type
}

impl<'ctx> fmt::Debug for FunctionPtrType<'ctx> {
    fn fmt<'a>(&self, fmt: &mut fmt::Formatter<'a>) -> Result<(), fmt::Error> {
        write!(fmt, "{:?} (", self.get_return_type())?;
        for i in 0..self.get_param_count() {
            write!(fmt, "{:?}, ", self.get_param_type(i))?;
        }
        write!(fmt, ")")
    }
}

impl<'ctx> FunctionPtrType<'ctx> {
    unsafe fn from_ptr(ptr: *mut gccjit_sys::gcc_jit_function_type) -> FunctionPtrType<'ctx> {
        FunctionPtrType {
            marker: PhantomData,
            ptr
        }
    }

    pub fn get_return_type(&self) -> Type<'ctx> {
        unsafe {
            from_ptr(gccjit_sys::gcc_jit_function_type_get_return_type(self.ptr))
        }
    }

    pub fn get_param_count(&self) -> usize {
        unsafe {
            gccjit_sys::gcc_jit_function_type_get_param_count(self.ptr) as usize
        }
    }

    pub fn get_param_type(&self, index: usize) -> Type<'ctx> {
        // TODO: return Option?
        unsafe {
            from_ptr(gccjit_sys::gcc_jit_function_type_get_param_type(self.ptr, index as _))
        }
    }
}

impl<'ctx> ToObject<'ctx> for Type<'ctx> {
    fn to_object(&self) -> Object<'ctx> {
        unsafe {
            let ptr = gccjit_sys::gcc_jit_type_as_object(self.ptr);
            object::from_ptr(ptr)
        }
    }
}

impl<'ctx> fmt::Debug for Type<'ctx> {
    fn fmt<'a>(&self, fmt: &mut fmt::Formatter<'a>) -> Result<(), fmt::Error> {
        let obj = self.to_object();
        obj.fmt(fmt)
    }
}

impl<'ctx> Type<'ctx> {
    /// Given a type T, creates a type to *T, a pointer to T.
    pub fn make_pointer(self) -> Type<'ctx> {
        unsafe {
            from_ptr(gccjit_sys::gcc_jit_type_get_pointer(self.ptr))
        }
    }

    #[cfg(feature="master")]
    pub fn set_packed(&self) {
        unsafe {
            gccjit_sys::gcc_jit_type_set_packed(self.ptr);
        }
    }

    /// Given a type T, creates a type of const T.
    pub fn make_const(self) -> Type<'ctx> {
        unsafe {
            from_ptr(gccjit_sys::gcc_jit_type_get_const(self.ptr))
        }
    }

    /// Given a type T, creates a new type of volatile T, which
    /// has the semantics of C's volatile.
    pub fn make_volatile(self) -> Type<'ctx> {
        unsafe {
            from_ptr(gccjit_sys::gcc_jit_type_get_volatile(self.ptr))
        }
    }

    /// Given a type T, creates a new type of restrict T, which
    /// has the semantics of C's restrict.
    #[cfg(feature="master")]
    pub fn make_restrict(self) -> Type<'ctx> {
        unsafe {
            from_ptr(gccjit_sys::gcc_jit_type_get_restrict(self.ptr))
        }
    }

    pub fn get_aligned(self, alignment_in_bytes: u64) -> Type<'ctx> {
        unsafe {
            from_ptr(gccjit_sys::gcc_jit_type_get_aligned(self.ptr, alignment_in_bytes as _))
        }
    }

    pub fn dyncast_array(self) -> Option<Type<'ctx>> {
        unsafe {
            let array_type = gccjit_sys::gcc_jit_type_dyncast_array(self.ptr);
            if array_type.is_null() {
                return None;
            }
            Some(from_ptr(array_type))
        }
    }

    pub fn is_bool(self) -> bool {
        unsafe {
            gccjit_sys::gcc_jit_type_is_bool(self.ptr) != 0
        }
    }

    pub fn is_integral(self) -> bool {
        unsafe {
            gccjit_sys::gcc_jit_type_is_integral(self.ptr) != 0
        }
    }

    #[cfg(feature = "master")]
    pub fn is_floating_point(self) -> bool {
        unsafe {
            gccjit_sys::gcc_jit_type_is_floating_point(self.ptr) != 0
        }
    }

    pub fn dyncast_vector(self) -> Option<VectorType<'ctx>> {
        unsafe {
            let vector_type = gccjit_sys::gcc_jit_type_dyncast_vector(self.ptr);
            if vector_type.is_null() {
                return None;
            }
            Some(VectorType::from_ptr(vector_type))
        }
    }

    pub fn is_struct(self) -> Option<Struct<'ctx>> {
        unsafe {
            let struct_type = gccjit_sys::gcc_jit_type_is_struct(self.ptr);
            if struct_type.is_null() {
                return None;
            }
            Some(structs::from_ptr(struct_type))
        }
    }

    pub fn dyncast_function_ptr_type(self) -> Option<FunctionPtrType<'ctx>> {
        unsafe {
            let function_ptr_type = gccjit_sys::gcc_jit_type_dyncast_function_ptr_type(self.ptr);
            if function_ptr_type.is_null() {
                return None;
            }
            Some(FunctionPtrType::from_ptr(function_ptr_type))
        }
    }

    pub fn get_size(&self) -> u32 {
        unsafe {
            let size = gccjit_sys::gcc_jit_type_get_size(self.ptr);
            assert_ne!(size, -1, "called get_size of unsupported type: {:?}", self);
            size as u32
        }
    }

    pub fn unqualified(&self) -> Type<'ctx> {
        unsafe {
            from_ptr(gccjit_sys::gcc_jit_type_unqualified(self.ptr))
        }
    }

    pub fn get_pointee(&self) -> Option<Type<'ctx>> {
        unsafe {
            let value = gccjit_sys::gcc_jit_type_is_pointer(self.ptr);
            if value.is_null() {
                return None;
            }
            Some(from_ptr(value))
        }
    }

    #[cfg(feature="master")]
    pub fn is_const(&self) -> Option<Type<'ctx>> {
        unsafe {
            let value = gccjit_sys::gcc_jit_type_is_const(self.ptr);
            if value.is_null() {
                return None;
            }
            Some(from_ptr(value))
        }
    }

    pub fn is_compatible_with(&self, typ: Type<'ctx>) -> bool {
        unsafe {
            gccjit_sys::gcc_jit_compatible_types(self.ptr, typ.ptr)
        }
    }
}

/// Typeable is a trait for types that have a corresponding type within
/// gccjit. This library implements this type for a variety of primitive types,
/// but it's also possible to implement this trait for more complex types
/// that will use the API on Context to construct analagous struct/union types.
pub trait Typeable {
    fn get_type<'a, 'ctx>(ctx: &'a Context<'ctx>) -> Type<'a>;
}

macro_rules! typeable_def {
    ($ty:ty, $expr:expr) => {
        impl Typeable for $ty {
            fn get_type<'a, 'ctx>(ctx: &'a Context<'ctx>) -> Type<'a> {
                unsafe {
                    let ctx_ptr = context::get_ptr(ctx);
                    let ptr = gccjit_sys::gcc_jit_context_get_type(ctx_ptr, $expr);
                    #[cfg(debug_assertions)]
                    if let Ok(Some(error)) = ctx.get_last_error() {
                        panic!("{}", error);
                    }
                    from_ptr(ptr)
                }
            }
        }
    }
}

typeable_def!((), GCC_JIT_TYPE_VOID);
typeable_def!(bool, GCC_JIT_TYPE_BOOL);
typeable_def!(char, GCC_JIT_TYPE_CHAR);
typeable_def!(f32, GCC_JIT_TYPE_FLOAT);
typeable_def!(f64, GCC_JIT_TYPE_DOUBLE);
typeable_def!(usize, GCC_JIT_TYPE_SIZE_T);

macro_rules! typeable_int_def {
    ($ty:ty, $num_bytes:expr, $signed:expr) => {
        impl Typeable for $ty {
            fn get_type<'a, 'ctx>(ctx: &'a Context<'ctx>) -> Type<'a> {
                unsafe {
                    let ctx_ptr = context::get_ptr(ctx);
                    let ptr = gccjit_sys::gcc_jit_context_get_int_type(ctx_ptr, $num_bytes, $signed as i32);
                    from_ptr(ptr)
                }
            }
        }
    }
}

typeable_int_def!(i8, 1, true);
typeable_int_def!(u8, 1, false);
typeable_int_def!(i16, 2, true);
typeable_int_def!(u16, 2, false);
typeable_int_def!(i32, 4, true);
typeable_int_def!(u32, 4, false);
typeable_int_def!(i64, 8, true);
typeable_int_def!(u64, 8, false);
//typeable_int_def!(i128, 16, true); // FIXME: unsupported by libgccjit for now.
//typeable_int_def!(u128, 16, false); // FIXME: unsupported by libgccjit for now.

/// Specific implementations of Typeable for *mut T and *const T that
/// represent void* and const void*, respectively. These impls should
/// only be used to expose opaque pointers to gccjit, not to create
/// pointers that are not opaque to gcc. For that, the make_pointer
/// function should be used.
impl<T> Typeable for *mut T {
    fn get_type<'a, 'ctx>(ctx: &'a Context<'ctx>) -> Type<'a> {
        unsafe {
            let ctx_ptr = context::get_ptr(ctx);
            let ptr = gccjit_sys::gcc_jit_context_get_type(ctx_ptr, GCC_JIT_TYPE_VOID_PTR);
            #[cfg(debug_assertions)]
            if let Ok(Some(error)) = ctx.get_last_error() {
                panic!("{}", error);
            }
            from_ptr(ptr)
        }
    }
}

impl<T> Typeable for *const T {
    fn get_type<'a, 'ctx>(ctx: &'a Context<'ctx>) -> Type<'a> {
        unsafe {
            let ctx_ptr = context::get_ptr(ctx);
            let ptr = gccjit_sys::gcc_jit_context_get_type(ctx_ptr, GCC_JIT_TYPE_VOID_PTR);
            #[cfg(debug_assertions)]
            if let Ok(Some(error)) = ctx.get_last_error() {
                panic!("{}", error);
            }
            from_ptr(ptr).make_const()
        }
    }
}

pub unsafe fn from_ptr<'ctx>(ptr: *mut gccjit_sys::gcc_jit_type) -> Type<'ctx> {
    Type {
        marker: PhantomData,
        ptr
    }
}

pub unsafe fn get_ptr<'ctx>(ty: &Type<'ctx>) -> *mut gccjit_sys::gcc_jit_type {
    ty.ptr
}
