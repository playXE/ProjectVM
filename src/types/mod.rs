pub mod value;

pub use self::value::Value;

use std::sync::Arc;


pub type Ptr<T> = Arc<T>;
pub type Name = Arc<String>;

#[allow(non_snake_case)]
pub fn Ptr<T>(value: T) -> Ptr<T> {
    Arc::new(value)
}

#[derive(Clone,Debug,PartialEq,Hash,Eq)]
pub struct StructSig {
    pub name: Name,
    pub idx: u32,
    pub types: Vec<Ptr<Type>>,
}

#[derive(Clone,Debug,PartialEq,Hash,Eq)]
pub struct FunctionSig {
    pub name: Name,
    pub idx: u32,
    pub is_native: bool,
    pub args: Vec<Ptr<Type>>,
    pub ret_ty: Ptr<Type>,
}

#[derive(Clone,Debug,PartialEq)]
pub struct Struct {
    pub sig: StructSig,
    pub tys: Vec<Type>,
}

use crate::instruction::Block;

#[derive(Clone)]
pub enum FunctionKind {
    Native(&'static Fn(Vec<Ptr<Value>>) -> Value),
    Internal(Vec<Block>),
}

use std::fmt;

impl fmt::Debug for FunctionKind {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"")
    }
}

impl fmt::Display for StructSig {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"struct ({})",crate::utils::vec_as_str(self.types.clone()))
    }
}


impl fmt::Display for FunctionSig {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"({}) -> {}",crate::utils::vec_as_str(self.args.clone()),self.ret_ty)
    }
}


#[derive(Clone,Debug)]
pub struct Function {
    pub kind: FunctionKind,
    pub sig: FunctionSig,
}

impl fmt::Display for Function {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"({}) -> {}",crate::utils::vec_as_str(self.sig.args.clone()),self.sig.ret_ty)
    }
}

#[derive(Clone,Debug,PartialEq,Hash,Eq)]
pub enum Type {
    /// Int<size> 
    Int8,
    Int16,
    Int32,
    Int64,

    /// Floating point with 32 bits
    Float,
    /// Floating point with 64 bits
    Double,

    Void,

    /// ref: reference type
    Ref(Ptr<Type>),
    /// uptr: unsafe pointer 
    UPtr(Ptr<Type>),
    /// @funcsig: function signature
    FuncSig(Ptr<FunctionSig>),
    /// unsafe pointer to function
    UFuncPtr(Ptr<FunctionSig>),
    /// Struct signature
    StructSig(Ptr<StructSig>),
    /// Unsafe pointer to structure
    UStructPtr(Ptr<StructSig>),

    Array(Ptr<Type>,u32),
}

impl fmt::Display for Type {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        use self::Type::*;
        match self {
            Int8 => write!(f,"int8"),
            Int16 => write!(f,"int16"),
            Int32 => write!(f,"int32"),
            Int64 => write!(f,"int64"),
            Float => write!(f,"float"),
            Double => write!(f,"double"),
            UPtr(ptr) => write!(f,"uptr<{}>",ptr),
            Ref(ptr)  => write!(f,"ref<{}>",ptr),
            FuncSig(sig) => write!(f,"@func<{}>",sig),
            StructSig(sig) => write!(f,"@struct<{}>",sig),
            UFuncPtr(ptr)  => write!(f,"@ufuncptr<{}>",ptr),
            UStructPtr(ptr) => write!(f,"@ustructptr<{}>",ptr),
            Array(ty,size) => write!(f, "array<{} {}>",ty,size),
            Void => write!(f, "void"),
        }
    }
}

impl Type {
    pub fn is_float(&self) -> bool {
        match self {
            Type::Float => true,
            _ => false,
        }
    }

    pub fn is_double(&self) -> bool {
        match self {
            Type::Double => true,
            _ => false,
        }
    }

    pub fn is_ptr(&self) -> bool {
        match self {
            Type::UPtr(_) => true,
            _ => false,
        }
    }

    pub fn is_ptr_of(&self,t: Type) -> bool {
        match self {
            Type::UPtr(p) => p == &Ptr(t),
            _ => false,
        }
    }


    pub fn is_int(&self) -> bool {
        match self {
            Type::Int8 | Type::Int16 | Type::Int32 | Type::Int64 => true,
            _ => false,
        }
    }

    pub fn is_ref(&self,t: Type) -> bool {
        match self {
            Type::Ref(p) => p == &Ptr(t),
            _ => false
        }
    }

    pub fn get_ptr(&self) -> Ptr<Type> {
        match self {
            Type::UPtr(ptr) => ptr.clone(),
            _ => unreachable!()
        }
    }

}