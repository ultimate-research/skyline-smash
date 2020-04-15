#[repr(C)]
#[derive(Copy, Clone)]
pub union L2CValueInner {
    pub raw: u64,
    pub raw_float: f32,
    pub raw_pointer: *mut libc::c_void,
    pub raw_table: *mut L2CTable,
    pub raw_innerfunc: *mut L2CInnerFunctionBase,
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
    pub unk: u32,
    pub inner: L2CValueInner,
}

impl L2CValue {
    pub fn new_void() -> Self {
        Self::default()
    }

    pub fn new_bool(val: bool) -> Self {
        Self {
            val_type: L2CValueType::Bool,
            unk: 0,
            inner: L2CValueInner { raw: val as u64 }
        }
    }

    pub fn new_int(val: u64) -> Self {
        Self {
            val_type: L2CValueType::Int,
            unk: 0,
            inner: L2CValueInner { raw: val as u64 }
        }
    }

    pub fn new_num(val: f32) -> Self {
        Self {
            val_type: L2CValueType::Num,
            unk: 0,
            inner: L2CValueInner { raw_float: val }
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
