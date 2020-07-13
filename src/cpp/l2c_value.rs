use super::root::lib;
use core::cell::UnsafeCell;
use core::cmp::Ordering;
use core::fmt;

#[repr(C)]
#[derive(Copy, Clone)]
pub union L2CValueInner {
    pub raw: u64,
    pub raw_float: f32,
    pub raw_pointer: *mut libc::c_void,
    pub raw_table: *mut L2CTable,
    pub raw_innerfunc: *mut L2CInnerFunctionBase,
}

impl fmt::Debug for L2CValueInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            f.debug_tuple("")
            .field(&self.raw)
            .field(&self.raw_float)
            .field(&self.raw_pointer)
            .field(&self.raw_table)
            .field(&self.raw_innerfunc)
            .finish()
        }
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum L2CValueType {
    Void = 0,
    Bool = 1,
    Int = 2,
    Num = 3,
    Pointer = 4,
    Table = 5,
    InnerFunc = 6,
    Hash = 7,
    String = 8
}

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct L2CValue {
    pub val_type: L2CValueType,
    pub unk1: u32,
    pub inner: L2CValueInner,
    pub unk2: u8, // for enforcing X8 AArch64 struct behavior
}

impl fmt::Debug for L2CValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            match self.val_type {
                L2CValueType::Void => f.debug_tuple("")
                                        .field(&self.val_type)
                                        .field(&self.inner.raw)
                                        .finish(),
                L2CValueType::Bool => f.debug_tuple("")
                                        .field(&self.val_type)
                                        .field(&(self.inner.raw != 0))
                                        .finish(),
                L2CValueType::Int => f.debug_tuple("")
                                        .field(&self.val_type)
                                        .field(&self.inner.raw)
                                        .finish(),
                L2CValueType::Num => f.debug_tuple("")
                                        .field(&self.val_type)
                                        .field(&self.inner.raw_float)
                                        .finish(),
                L2CValueType::InnerFunc => f.debug_tuple("")
                                        .field(&self.val_type)
                                        .field(&self.inner.raw_innerfunc)
                                        .finish(),
                L2CValueType::Hash => f.debug_tuple("")
                                        .field(&self.val_type)
                                        .field(&self.inner.raw)
                                        .finish(),
                _ => f.debug_tuple("")
                        .field(&self.val_type)
                        .field(&self.unk1)
                        .field(&self.inner.raw)
                        .field(&self.unk2)
                        .finish()
            }
        }
    }
}

pub trait LuaTableIndex: Sized {
    fn index_with<'a>(&self, l2c_val: &'a L2CValue) -> &'a L2CValue;
    fn index_with_mut<'a>(&self, l2c_val: &'a mut L2CValue) -> &'a mut L2CValue;
}

impl LuaTableIndex for i32 {
    fn index_with<'a>(&self, l2c_val: &'a L2CValue) -> &'a L2CValue {
        unsafe {
            crate::lib::L2CValue__index_int(l2c_val, *self)
        }
    }

    fn index_with_mut<'a>(&self, l2c_val: &'a mut L2CValue) -> &'a mut L2CValue {
        unsafe {
            crate::lib::L2CValue__index_int_mut(l2c_val, *self)
        }
    }
}

impl LuaTableIndex for L2CValue {
    fn index_with<'a>(&self, l2c_val: &'a L2CValue) -> &'a L2CValue {
        unsafe {
            crate::lib::L2CValue__index_L2CValue(l2c_val, self)
        }
    }

    fn index_with_mut<'a>(&self, l2c_val: &'a mut L2CValue) -> &'a mut L2CValue {
        unsafe {
            crate::lib::L2CValue__index_L2CValue_mut(l2c_val, self)
        }
    }
}

impl LuaTableIndex for u64 {
    fn index_with<'a>(&self, l2c_val: &'a L2CValue) -> &'a L2CValue {
        unsafe {
            crate::lib::L2CValue__index_hash40(l2c_val, *self)
        }
    }

    fn index_with_mut<'a>(&self, l2c_val: &'a mut L2CValue) -> &'a mut L2CValue {
        unsafe {
            crate::lib::L2CValue__index_hash40_mut(l2c_val, *self)
        }
    }
}

impl LuaTableIndex for &str {
    fn index_with<'a>(&self, l2c_val: &'a L2CValue) -> &'a L2CValue {
        let hash = crate::hash40(*self);

        hash.index_with(l2c_val)
    }

    fn index_with_mut<'a>(&self, l2c_val: &'a mut L2CValue) -> &'a mut L2CValue {
        let hash = crate::hash40(*self);

        hash.index_with_mut(l2c_val)
    }
}

impl<Idx: LuaTableIndex> core::ops::Index<Idx> for L2CValue {
    type Output = L2CValue;

    fn index(&self, index: Idx) -> &Self::Output {
        index.index_with(self)
    }
}

impl<Idx: LuaTableIndex> core::ops::IndexMut<Idx> for L2CValue {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        index.index_with_mut(self)
    }
}

impl L2CValue {
    pub const fn new_void() -> Self {
        Self {
            val_type: L2CValueType::Void,
            unk1: 0,
            inner: L2CValueInner { raw: 0 as u64 },
            unk2: 0
        }
    }

    pub const fn new_bool(val: bool) -> Self {
        Self {
            val_type: L2CValueType::Bool,
            unk1: 0,
            inner: L2CValueInner { raw: val as u64 },
            unk2: 0
        }
    }

    pub const fn new_int(val: u64) -> Self {
        Self {
            val_type: L2CValueType::Int,
            unk1: 0,
            inner: L2CValueInner { raw: val as u64 },
            unk2: 0
        }
    }

    pub const fn new_num(val: f32) -> Self {
        Self {
            val_type: L2CValueType::Num,
            unk1: 0,
            inner: L2CValueInner { raw_float: val },
            unk2: 0
        }
    }

    pub fn try_get_bool(&self) -> Option<bool> {
        if let L2CValueType::Bool = self.val_type {
            Some(unsafe { self.inner.raw } & 1 != 0)
        } else {
            None
        }
    }

    #[track_caller]
    pub fn get_bool(&self) -> bool {
        if let Some(val) = self.try_get_bool() {
            val
        } else {
            panic!("L2CValue: {:?} not a bool", self);
        }
    }

    pub fn try_get_int(&self) -> Option<u64> {
        if let L2CValueType::Int = self.val_type {
            Some(unsafe { self.inner.raw })
        } else {
            None
        }
    }

    #[track_caller]
    pub fn get_int(&self) -> u64 {
        if let Some(val) = self.try_get_int() {
            val
        } else {
            panic!("L2CValue: {:?} not an int", self);
        }
    }

    pub fn try_get_num(&self) -> Option<f32> {
        if let L2CValueType::Num = self.val_type {
            Some(unsafe { self.inner.raw_float })
        } else {
            None
        }
    }

    #[track_caller]
    pub fn get_num(&self) -> f32 {
        if let Some(val) = self.try_get_num() {
            val
        } else {
            panic!("L2CValue: {:?} not a float", self);
        }
    }

    pub fn try_get_ptr<T>(&self) -> Option<*mut T> {
        if let L2CValueType::Pointer = self.val_type {
            Some(unsafe { self.inner.raw as *mut T })
        } else {
            None
        }
    }

    #[track_caller]
    pub fn get_ptr<T>(&self) -> *mut T {
        if let Some(val) = self.try_get_ptr() {
            val
        } else {
            panic!("L2CValue: {:?} is not a pointer", self);
        }
    }
}

pub fn lua_val<T: Into<L2CValue> + Sized>(val: T) -> L2CValue {
    val.into()
}

macro_rules! impl_into_l2cvalue_int {
    (
        $(
            $ty:ty
        )*
    ) => {
        $(
            impl Into<L2CValue> for $ty {
                fn into(self) -> L2CValue {
                    L2CValue::new_int(self as u64)
                }
            }
        )*
    };
}

macro_rules! impl_into_l2cvalue_float {
    (
        $(
            $ty:ty
        )*
    ) => {
        $(
            impl Into<L2CValue> for $ty {
                fn into(self) -> L2CValue {
                    L2CValue::new_num(self as f32)
                }
            }
        )*
    };
}

impl Into<L2CValue> for LuaConst {
    fn into(self) -> L2CValue {
        L2CValue::new_int(*self as u64)
    }
}

impl Into<L2CValue> for bool {
    fn into(self) -> L2CValue {
        L2CValue::new_bool(self)
    }
}

impl_into_l2cvalue_int!(i8 u8 i16 u16 i32 u32 u64 i64);
impl_into_l2cvalue_float!(f32 f64);

impl Default for L2CValueInner {
    fn default() -> Self {
        Self { raw: 0 }
    }
}

impl Default for L2CValueType {
    fn default() -> Self {
        Self::Void
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct L2CTable_meta {
    pub a: u64,
    pub b: u64,
    pub c: u64,
    pub d: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct L2CTable {
    pub refcnt: u32,
    pub unk: u32,
    pub begin: u64,
    pub end: u64,
    pub also_end: u64,
    pub meta: L2CTable_meta,
    pub unk_ptr: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct L2CInnerFunctionBase {
    pub unk: u64,
    pub refcnt: u32,
}

impl super::root::lib::L2CAgent {
    pub fn new(lua_state: u64) -> Self {
        let mut l2c_agent = Self {
            vtable: 0,
            lua_state_agent: 0,
            unk10: 0,
            unk18: 0,
            unk20: 0,
            unk28: 0,
            unk30: 0,
            unk38: 0,
            lua_state_agentbase: 0
        };
        unsafe {
            super::root::lib::L2CAgent_L2CAgent_constr(&mut l2c_agent, lua_state); 
        }

        l2c_agent
    }
}

impl core::ops::Deref for super::root::lib::L2CAgent {
    type Target = u64;
    
    fn deref(&self) -> &Self::Target {
        &self.lua_state_agent
    }
}

/*pub unsafe fn lua_const<S: AsRef<[u8]>>(string: S) -> libc::c_int {
    let mut val : i32 = -1;
    if lib::lua_bind_get_value(lua_bind_hash::lua_bind_hash(string), &mut val) {
        val
    } else{
        -1
    }
}*/

pub struct LuaConst {
    lua_bind_hash: u64,
    // do not try this at home
    cache: UnsafeCell<Option<i32>>,
}

impl LuaConst {
    pub const fn new(lua_bind_hash: u64) -> Self {
        Self {
            lua_bind_hash,
            cache: UnsafeCell::new(None)
        }
    }

    pub fn as_lua_int(&self) -> L2CValue {
        L2CValue::new_int(**self as u64)
    }
}

//Release

impl From<LuaConst> for i32 {
    fn from(lua_const: LuaConst) -> Self {
        *lua_const
    }
}

impl Clone for LuaConst {
    fn clone(&self) -> Self {
        LuaConst::new(self.lua_bind_hash)
    }
}

macro_rules! lua_const_partialeq_impl {
    (
        $(
            $ty:ty
        )*
    ) => {
        $(
            impl PartialEq<$ty> for LuaConst {
                fn eq(&self, other: &$ty) -> bool {
                    return **self == other.clone() as i32;
                }
            }

            impl PartialOrd<$ty> for LuaConst {
                fn partial_cmp(&self, other: &$ty) -> Option<Ordering> {
                    Some((**self).cmp(&(other.clone() as i32)))
                }
            }

            impl PartialEq<LuaConst> for $ty {
                fn eq(&self, other: &LuaConst) -> bool {
                    return *self as i32 == **other;
                }
            }

            impl PartialOrd<LuaConst> for $ty {
                fn partial_cmp(&self, other: &LuaConst) -> Option<Ordering> {
                    Some((*self as i32).cmp(&**other))
                }
            }
        )*
    };
}

impl PartialEq<L2CValue> for LuaConst {
    #[track_caller]
    fn eq(&self, other: &L2CValue) -> bool {
        return *self == other.get_int() as i32;
    }
}

impl PartialOrd<L2CValue> for LuaConst {
    #[track_caller]
    fn partial_cmp(&self, other: &L2CValue) -> Option<Ordering> {
        Some((**self).cmp(&(other.get_int() as i32)))
    }
}

impl PartialEq<LuaConst> for L2CValue {
    #[track_caller]
    fn eq(&self, other: &LuaConst) -> bool {
        return self.get_int() as i32 == **other;
    }
}

impl PartialOrd<LuaConst> for L2CValue {
    #[track_caller]
    fn partial_cmp(&self, other: &LuaConst) -> Option<Ordering> {
        Some((self.get_int() as i32).cmp(&**other))
    }
}

lua_const_partialeq_impl!(i32 u32 u64);

impl core::ops::Deref for LuaConst {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        let cache = self.cache.get();

        if let Some(ref val) = unsafe { *cache }  {
            unsafe {
                // if there is a bug with this implementation
                // DEFINITELY start here
                core::mem::transmute(val)
            }
        } else {
            let mut val : i32 = -1;
            if unsafe { lib::lua_bind_get_value(self.lua_bind_hash, &mut val) } {
                unsafe {
                    *cache = Some(val);

                    (*cache).as_ref().unwrap()
                }
            } else{
                &-1
            }
        }
    }
}
