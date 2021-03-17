use super::root::lib;
use core::cell::UnsafeCell;
use core::cmp::Ordering;
use core::fmt;

extern "C" {
    #[link_name = "\u{1}_ZN3lib8L2CValueC1Ev"]
    fn L2CValue_L2CValue(arg: *mut L2CValue);
    #[link_name = "\u{1}_ZN3lib8L2CValueC1ERKS0_"]
    fn L2CValue_L2CValue2(arg: *mut L2CValue, src: *const L2CValue);
    #[link_name = "\u{1}_ZN3lib8L2CValueC1Eb"]
    fn L2CValue_L2CValue3(arg: *mut L2CValue, val: bool);
    #[link_name = "\u{1}_ZN3lib8L2CValueC1Ei"]
    fn L2CValue_L2CValue4(arg: *mut L2CValue, val: i32);
    #[link_name = "\u{1}_ZN3lib8L2CValueC1Ej"]
    fn L2CValue_L2CValue5(arg: *mut L2CValue, val: u32);
    #[link_name = "\u{1}_ZN3lib8L2CValueC1El"]
    fn L2CValue_L2CValue6(arg: *mut L2CValue, val: i64);
    #[link_name = "\u{1}_ZN3lib8L2CValueC1Em"]
    fn L2CValue_L2CValue7(arg: *mut L2CValue, val: u64);
    #[link_name = "\u{1}_ZN3lib8L2CValueC1Ef"]
    fn L2CValue_L2CValue8(arg: *mut L2CValue, val: f32);
    #[link_name = "\u{1}_ZN3lib8L2CValueC1EPv"]
    fn L2CValue_L2CValue9(arg: *mut L2CValue, val: *mut libc::c_void);
    #[link_name = "\u{1}_ZN3lib8L2CValueC1EPNS_8L2CTableE"]
    fn L2CValue_L2CValue10(arg: *mut L2CValue, val: *mut L2CTable);
    #[link_name = "\u{1}_ZN3lib8L2CValueC1EPNS_20L2CInnerFunctionBaseE"]
    fn L2CValue_L2CValue11(arg: *mut L2CValue, val: *mut L2CInnerFunctionBase);
    #[link_name = "\u{1}_ZN3lib8L2CValueC1EN3phx6Hash40E"]
    fn L2CValue_L2CValue12(arg: *mut L2CValue, val: crate::phx::Hash40);
    #[link_name = "\u{1}_ZN3lib8L2CValueC1EPKc"]
    fn L2CValue_L2CValue13(arg: *mut L2CValue, val: *const libc::c_char);
    #[link_name = "\u{1}_ZNK3lib8L2CValue7as_boolEv"]
    fn L2CValue_as_bool(arg: *const L2CValue) -> bool;
    #[link_name = "\u{1}_ZNK3lib8L2CValue7as_hashEv"]
    fn L2CValue_as_hash(arg: *const L2CValue) -> crate::phx::Hash40;
    #[link_name = "\u{1}_ZNK3lib8L2CValue17as_inner_functionEv"]
    fn L2CValue_as_inner_function(arg: *const L2CValue) -> *mut L2CInnerFunctionBase;
    #[link_name = "\u{1}_ZNK3lib8L2CValue10as_integerEv"]
    fn L2CValue_as_integer(arg: *const L2CValue) -> u64;
    #[link_name = "\u{1}_ZNK3lib8L2CValue9as_numberEv"]
    fn L2CValue_as_number(arg: *const L2CValue) -> f32;
    #[link_name = "\u{1}_ZNK3lib8L2CValue10as_pointerEv"]
    fn L2CValue_as_pointer(arg: *const L2CValue) -> *mut libc::c_void;
    #[link_name = "\u{1}_ZNK3lib8L2CValue9as_stringEv"]
    fn L2CValue_as_string(arg: *const L2CValue) -> *const libc::c_char;
    #[link_name = "\u{1}_ZNK3lib8L2CValue8as_tableEv"]
    fn L2CValue_as_table(arg: *const L2CValue) -> *mut L2CTable;
    #[link_name = "\u{1}_ZNK3lib8L2CValueixEi"]
    fn L2CValue_idx<'a>(arg: *const L2CValue, idx: i32) -> &'a L2CValue;
    #[link_name = "\u{1}_ZNK3lib8L2CValueixEi"]
    fn L2CValue_idx_mut<'a>(arg: *const L2CValue, idx: i32) -> &'a mut L2CValue;
    #[link_name = "\u{1}_ZNK3lib8L2CValueixEN3phx6Hash40E"]
    fn L2CValue_idx_hash<'a>(arg: *const L2CValue, idx: crate::phx::Hash40) -> &'a L2CValue;
    #[link_name = "\u{1}_ZNK3lib8L2CValueixEN3phx6Hash40E"]
    fn L2CValue_idx_hash_mut<'a>(arg: *const L2CValue, idx: crate::phx::Hash40) -> &'a mut L2CValue;
    #[link_name = "\u{1}_ZNK3lib8L2CValueixERKS0_"]
    fn L2CValue_idx_l2c<'a>(arg: *const L2CValue, idx: *const L2CValue) -> &'a L2CValue;
    #[link_name = "\u{1}_ZNK3lib8L2CValueixERKS0_"]
    fn L2CValue_idx_l2c_mut<'a>(arg: *const L2CValue, idx: *const L2CValue) -> &'a mut L2CValue;
    #[link_name = "\u{1}_ZN3lib8L2CValueD1Ev"]
    fn L2CValue_dtor(arg: *mut L2CValue);
    #[link_name = "\u{1}_ZN3lib8L2CTableC1Ei"]
    fn L2CTable_L2CTable(arg: *mut L2CTable, count: i32);
    #[link_name = "\u{1}_Znwm"]
    fn cpp_new(size: u64) -> *mut libc::c_void;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union L2CValueInner {
    pub raw: u64,
    pub raw_float: f32,
    pub raw_pointer: *mut libc::c_void,
    pub raw_table: *mut L2CTable,
    pub raw_innerfunc: *mut L2CInnerFunctionBase
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
#[derive(Copy, Clone, Debug, PartialEq)]
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

#[derive(Default)]
#[repr(C)]
pub struct L2CValue {
    pub val_type: L2CValueType,
    pub unk1: u32,
    pub inner: L2CValueInner,
    // pub unk2: u8, // for enforcing X8 AArch64 struct behavior
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
    pub fn Void() -> Self {
        unsafe {
            let mut ret = Self::default();
            L2CValue_L2CValue(&mut ret as *mut Self);
            ret
        }
    }
    pub fn Bool(val: bool) -> Self {
        unsafe {
            let mut ret = Self::default();
            L2CValue_L2CValue3(&mut ret as *mut Self, val);
            ret
        }
    }
    pub fn I32(val: i32) -> Self {
        unsafe {
            let mut ret = Self::default();
            L2CValue_L2CValue4(&mut ret as *mut Self, val);
            ret
        }
    }
    pub fn U32(val: u32) -> Self {
        unsafe {
            let mut ret = Self::default();
            L2CValue_L2CValue5(&mut ret as *mut Self, val);
            ret
        }
    }
    pub fn I64(val: i64) -> Self {
        unsafe {
            let mut ret = Self::default();
            L2CValue_L2CValue6(&mut ret as *mut Self, val);
            ret
        }
    }
    pub fn U64(val: u64) -> Self {
        unsafe {
            let mut ret = Self::default();
            L2CValue_L2CValue7(&mut ret as *mut Self, val);
            ret
        }
    }
    pub fn F32(val: f32) -> Self {
        unsafe {
            let mut ret = Self::default();
            L2CValue_L2CValue8(&mut ret as *mut Self, val);
            ret
        }
    }
    pub fn Ptr<T>(val: *mut T) -> Self {
        unsafe {
            let mut ret = Self::default();
            L2CValue_L2CValue9(&mut ret as *mut Self, val as *mut libc::c_void);
            ret
        }
    }
    pub fn Table(val: *mut L2CTable) -> Self {
        unsafe {
            let mut ret = Self::default();
            L2CValue_L2CValue10(&mut ret as *mut Self, val);
            ret
        }
    }
    pub fn InnerFunc(val: *mut L2CInnerFunctionBase) -> Self {
        unsafe {
            let mut ret = Self::default();
            L2CValue_L2CValue11(&mut ret as *mut Self, val);
            ret
        }
    }
    pub fn Hash40(val: crate::phx::Hash40) -> Self {
        unsafe {
            let mut ret = Self::default();
            L2CValue_L2CValue12(&mut ret as *mut Self, val);
            ret
        }
    }
    pub fn Hash40s<S: AsRef<str>>(val: S) -> Self {
        unsafe {
            let mut ret = Self::default();
            L2CValue_L2CValue12(&mut ret as *mut Self, crate::phx::Hash40::new(val.as_ref()));
            ret
        }
    }
    pub fn String<S: AsRef<str>>(val: S) -> Self {
        unsafe {
            let mut ret = Self::default();
            let str_ref = val.as_ref();
            let c_str = [str_ref.as_bytes(), "\u{0}".as_bytes()].concat().as_ptr() as *const libc::c_char;
            L2CValue_L2CValue13(&mut ret as *mut Self, c_str);
            ret
        }
    }

    pub fn get_bool(&self) -> bool {
        self.into()
    }

    pub fn get_i32(&self) -> i32 {
        self.into()
    }

    pub fn get_u32(&self) -> u32 {
        self.into()
    }

    pub fn get_i64(&self) -> i64 {
        self.into()
    }

    pub fn get_u64(&self) -> u64 {
        self.into()
    }

    pub fn get_f32(&self) -> f32 {
        self.into()
    }

    pub fn get_f64(&self) -> f64 {
        self.into()
    }

    pub fn get_ptr<T>(&self) -> *const T {
        unsafe {
            L2CValue_as_pointer(self as *const L2CValue) as *const T
        }
    }

    pub fn get_ptr_mut<T>(&self) -> *mut T {
        unsafe {
            L2CValue_as_pointer(self as *const L2CValue) as *mut T
        }
    }

    pub fn get_table(&self) -> *mut L2CTable {
        self.into()
    }

    pub fn get_inner_func(&self) -> *mut L2CInnerFunctionBase {
        self.into()
    }

    pub fn get_hash(&self) -> crate::phx::Hash40 {
        self.into()
    }

    pub fn get_string(&self) -> String {
        self.into()
    }
}

impl Drop for L2CValue {
    fn drop(&mut self) {
        unsafe {
            L2CValue_dtor(self as *mut Self);
        }
    }
}

impl Into<bool> for L2CValue {
    fn into(self) -> bool {
        unsafe {
            L2CValue_as_bool(&self as *const Self)
        }
    }
}
impl Into<bool> for &L2CValue {
    fn into(self) -> bool {
        unsafe {
            L2CValue_as_bool(self as *const L2CValue)
        }
    }
}

impl Into<crate::phx::Hash40> for L2CValue {
    fn into(self) -> crate::phx::Hash40 {
        unsafe {
            L2CValue_as_hash(&self as *const Self)
        }
    }
}
impl Into<crate::phx::Hash40> for &L2CValue {
    fn into(self) -> crate::phx::Hash40 {
        unsafe {
            L2CValue_as_hash(self as *const L2CValue)
        }
    }
}

impl Into<*mut L2CInnerFunctionBase> for L2CValue {
    fn into(self) -> *mut L2CInnerFunctionBase {
        unsafe {
            L2CValue_as_inner_function(&self as *const Self)
        }
    }
}
impl Into<*mut L2CInnerFunctionBase> for &L2CValue {
    fn into(self) -> *mut L2CInnerFunctionBase {
        unsafe {
            L2CValue_as_inner_function(self as *const L2CValue)
        }
    }
}

impl Into<*mut L2CTable> for L2CValue {
    fn into(self) -> *mut L2CTable {
        unsafe {
            L2CValue_as_table(&self as *const Self)
        }
    }
}
impl Into<*mut L2CTable> for &L2CValue {
    fn into(self) -> *mut L2CTable {
        unsafe {
            L2CValue_as_table(self as *const L2CValue)
        }
    }
}

impl Into<String> for L2CValue {
    fn into(self) -> String {
        unsafe {
            skyline::from_c_str(L2CValue_as_string(&self as *const Self))
        }
    }
}
impl Into<String> for &L2CValue {
    fn into(self) -> String {
        unsafe {
            skyline::from_c_str(L2CValue_as_string(self as *const L2CValue))
        }
    }
}

impl Into<L2CValue> for LuaConst {
    fn into(self) -> L2CValue {
        L2CValue::I32(*self)
    }
}

impl Into<L2CValue> for () {
    fn into(self) -> L2CValue {
        L2CValue::Void()
    }
}

impl Into<L2CValue> for bool {
    fn into(self) -> L2CValue {
        L2CValue::Bool(self)
    }
}

impl Into<L2CValue> for i32 {
    fn into(self) -> L2CValue {
        L2CValue::I32(self)
    }
}

impl Into<L2CValue> for u32 {
    fn into(self) -> L2CValue {
        L2CValue::U32(self)
    }
}

impl Into<L2CValue> for i64 {
    fn into(self) -> L2CValue {
        L2CValue::I64(self)
    }
}

impl Into<L2CValue> for u64 {
    fn into(self) -> L2CValue {
        L2CValue::U64(self)
    }
}

impl Into<L2CValue> for f32 {
    fn into(self) -> L2CValue {
        L2CValue::F32(self)
    }
}

impl Into<L2CValue> for f64 {
    fn into(self) -> L2CValue {
        L2CValue::F32(self as f32)
    }
}

default impl<T> Into<L2CValue> for *mut T {
    fn into(self) -> L2CValue {
        L2CValue::Ptr(self)
    }
}

impl Into<L2CValue> for *mut L2CTable {
    fn into(self) -> L2CValue {
        L2CValue::Table(self)
    }
}

impl Into<L2CValue> for *mut L2CInnerFunctionBase {
    fn into(self) -> L2CValue {
        L2CValue::InnerFunc(self)
    }
}

impl Into<L2CValue> for crate::phx::Hash40 {
    fn into(self) -> L2CValue {
        L2CValue::Hash40(self)
    }
}

impl Into<L2CValue> for &str {
    fn into(self) -> L2CValue {
        L2CValue::String(self)
    }
}

impl Into<L2CValue> for String {
    fn into(self) -> L2CValue {
        L2CValue::String(self)
    }
}

impl Clone for L2CValue {
    fn clone(&self) -> Self {
        unsafe {
            let mut ret = Self::default();
            L2CValue_L2CValue2(&mut ret as *mut Self, self as *const Self);
            ret
        }
    }
}

pub fn lua_val<T: Into<L2CValue> + Sized>(val: T) -> L2CValue {
    val.into()
}

macro_rules! impl_into_int_l2cvalue {
    (
        $(
            $ty: ty
        )*
    ) => {
        $(
            impl Into<$ty> for L2CValue {
                fn into(self) -> $ty {
                    unsafe {
                        L2CValue_as_integer(&self as *const Self) as $ty
                    }
                }
            }

            impl Into<$ty> for &L2CValue {
                fn into(self) -> $ty {
                    unsafe { 
                        L2CValue_as_integer(self as *const L2CValue) as $ty
                    }
                }
            }
        )*
    };
}

macro_rules! impl_into_float_l2cvalue {
    (
        $(
            $ty: ty
        )*
    ) => {
        $(
            impl Into<$ty> for L2CValue {
                fn into(self) -> $ty {
                    unsafe {
                        L2CValue_as_number(&self as *const Self) as $ty
                    }
                }
            }
            impl Into<$ty> for &L2CValue {
                fn into(self) -> $ty {
                    unsafe {
                        L2CValue_as_number(self as *const L2CValue) as $ty
                    }
                }
            }
        )*
    };
}

macro_rules! impl_partialeq_int_l2cvalue {
    (
        $(
            $ty:ty
        )*
    ) => {
        $(
            impl PartialEq<$ty> for L2CValue {
                fn eq(&self, other: &$ty) -> bool {
                    self.val_type == L2CValueType::Int && (self.get_u64() as $ty) == *other
                }
            }

            impl PartialEq<L2CValue> for $ty {
                fn eq(&self, other: &L2CValue) -> bool {
                    other.val_type == L2CValueType::Int && (other.get_u64() as $ty) == *self
                }
            }
        )*
    }
}

macro_rules! impl_partialeq_float_l2cvalue {
    (
        $(
            $ty:ty
        )*
    ) => {
        $(
            impl PartialEq<$ty> for L2CValue {
                fn eq(&self, other: &$ty) -> bool {
                    self.val_type == L2CValueType::Num && (self.get_f32() as $ty) == *other
                }
            }

            impl PartialEq<L2CValue> for $ty {
                fn eq(&self, other: &L2CValue) -> bool {
                    other.val_type == L2CValueType::Num && (other.get_f32() as $ty) == *self
                }
            }
        )*
    }
}

impl_partialeq_int_l2cvalue!(i8 u8 i16 u16 i32 u32 u64 i64);
impl_partialeq_float_l2cvalue!(f32 f64);

impl_into_int_l2cvalue!(i8 u8 i16 u16 i32 u32 u64 i64);
impl_into_float_l2cvalue!(f32 f64);

impl PartialEq<LuaConst> for L2CValue {
    fn eq(&self, other: &LuaConst) -> bool {
        self.val_type == L2CValueType::Int && self.get_i32() == **other
    }
}

impl PartialEq<L2CValue> for LuaConst {
    fn eq(&self, other: &L2CValue) -> bool {
        other.val_type == L2CValueType::Int && other.get_i32() == **self
    }
}

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

impl L2CTable {
    pub fn new(size: i32) -> *mut Self {
        unsafe {
            let ret = cpp_new(0x48) as *mut L2CTable;
            L2CTable_L2CTable(ret, size);
            ret
        }
    }
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
            battle_object: 0 as *mut super::root::app::BattleObject,
            module_accessor: 0 as *mut super::root::app::BattleObjectModuleAccessor
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
        L2CValue::I32(**self)
    }
}

//Release

impl From<LuaConst> for i32 {
    fn from(lua_const: LuaConst) -> Self {
        *lua_const
    }
}

impl From<LuaConst> for u32 {
    fn from(lua_const: LuaConst) -> Self {
        *lua_const as u32
    }
}

impl Clone for LuaConst {
    fn clone(&self) -> Self {
        LuaConst::new(self.lua_bind_hash)
    }
}

impl std::hash::Hash for LuaConst {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.lua_bind_hash.hash(state);
    }
}

impl Eq for LuaConst {}

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

impl PartialEq for LuaConst {
    fn eq(&self, other: &LuaConst) -> bool {
        *self == *other
    }
}

// impl PartialOrd<LuaConst> for L2CValue {
//     #[track_caller]
//     fn partial_cmp(&self, other: &LuaConst) -> Option<Ordering> {
//         Some((self.get_i32()).cmp(&**other))
//     }
// }

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
