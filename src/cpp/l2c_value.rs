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

#[derive(Copy, Clone, Default, Debug)]
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

impl L2CValue {
    pub fn new_void() -> Self {
        Self::default()
    }

    pub fn new_bool(val: bool) -> Self {
        Self {
            val_type: L2CValueType::Bool,
            unk1: 0,
            inner: L2CValueInner { raw: val as u64 },
            unk2: 0
        }
    }

    pub fn new_int(val: u64) -> Self {
        Self {
            val_type: L2CValueType::Int,
            unk1: 0,
            inner: L2CValueInner { raw: val as u64 },
            unk2: 0
        }
    }

    pub fn new_num(val: f32) -> Self {
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
            panic!("L2CValue not a bool");
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
            panic!("L2CValue not an int");
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
            panic!("L2CValue not an float");
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
            panic!("L2CValue is not a pointer");
        }
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

impl super::root::lib::L2CAgent {
    #[inline]
    pub unsafe fn pop_lua_stack(&mut self, index: libc::c_int) -> L2CValue {
        let mut l2c_val = L2CValue::new();
        asm!("mov x8, $0"
        :                               // outputs
        :  "r"(&mut l2c_val as *mut _)  // inputs
        :  "x8"                         // clobbers
        :                               // no options
        );
        lib::L2CAgent_pop_lua_stack(self, index);
        l2c_val
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

impl<T: Into<i32> + Clone> PartialEq<T> for LuaConst {
    fn eq(&self, other: &T) -> bool {
        return **self == other.clone().into();
    }
}

impl<T: Into<i32> + Clone> PartialOrd<T> for LuaConst {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        Some((**self).cmp(&(other.clone().into())))
    }
}

impl PartialEq<LuaConst> for i32 {
    fn eq(&self, other: &LuaConst) -> bool {
        return *self == **other;
    }
}

impl PartialOrd<LuaConst> for i32 {
    fn partial_cmp(&self, other: &LuaConst) -> Option<Ordering> {
        Some((*self).cmp(&**other))
    }
}

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
