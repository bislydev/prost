use core::{ops::BitOr, fmt, convert::TryFrom};
use prost_types::field_descriptor_proto::Type;

macro_rules! implement_conversions {
    ($filter:ident, $selector:ident { $($var:ident,)* }) => {
        impl $filter {
            pub(crate) fn is_set(&self, en: $selector) -> bool {
                self.0 & (en as u32) != 0
            }
        }

        impl From<$selector> for $filter {
            fn from(en: $selector) -> Self {
                $filter(en as u32)
            }
        }

        impl<T: Into<$filter>> BitOr<T> for $selector {
            type Output = $filter;
            fn bitor(self, rhs: T) -> Self::Output {
                $filter::from(self) | <T as Into<$filter>>::into(rhs)
            }
        }

        impl<T: Into<$filter>> BitOr<T> for $filter {
            type Output = Self;
            fn bitor(self, rhs: T) -> Self::Output {
                $filter(self.0 | rhs.into().0)
            }
        }

        impl fmt::Debug for $filter {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, concat!(stringify!($filter), "( "))?;
                let mut written = false;
                for idx in 0..32 {
                    let mask = 1<<idx;
                    if mask & self.0 != 0 {
                        if written { write!(f, " | ")?; }
                        written = true;
                        match $selector::try_from(mask) {
                            Ok(ref x) => fmt::Debug::fmt(x, f)?,
                            Err(e) => write!(f, "Unknonwn({})", e)?,
                        }
                    }
                }
                write!(f, " )")
            }
        }

        // this would make automatic https://github.com/rust-lang/rust/pull/81642
        impl TryFrom<u32> for $selector {
            type Error = u32;

            fn try_from(value: u32) -> Result<Self, Self::Error> {
                match value {
                    $( n if n == Self::$var as u32   => Ok(Self::$var), )*
                    oth => Err(oth)
                }
            }
        }
    }
}

/// A collection of `TypeSelector`s. The filter matches if ANY of the
/// inner `TypeSelector`s match. Can be created by `BitOr`ing together
/// `TypeSelector`s or calling `Into::into()` on a `TypeSelector`.
#[derive(Default, Clone, Copy)]
pub struct TypeFilter(u32);

/// Selects a output object (rust struct or enum) during code
/// generation based on either the output rust type or the
/// protobuf type (message, enum oneof) that it represents.
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum TypeSelector {
    ProtobufMessage   = 1<<0,
    ProtobufEnum      = 1<<1,
    ProtobufOneof     = 1<<2,
    RustStruct        = 1<<3,
    RustEnum          = 1<<4,
    RustEnumCLike     = 1<<5,
    RustEnumWithData  = 1<<6,
    Everything        = u32::MAX,
}

implement_conversions!(
    TypeFilter,
    TypeSelector {
        ProtobufMessage,
        ProtobufEnum,
        ProtobufOneof,
        RustStruct,
        RustEnum,
        RustEnumCLike,
        RustEnumWithData,
    }
);

macro_rules! impl_from_type {
    ($selector:ident { $($var:ident,)* }) => {
        impl From<Type> for $selector {
            fn from(t: Type) -> $selector {
                match t {
                    $( Type::$var => $selector::$var ),*

                }
            }
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct FieldFilter(u32);

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum FieldSelector {
    // Protobuf types
    Double     = 1<<1,
    Float      = 1<<2,
    Int64      = 1<<3,
    Uint64     = 1<<4,
    Int32      = 1<<5,
    Fixed64    = 1<<6,
    Fixed32    = 1<<7,
    Bool       = 1<<8,
    String     = 1<<9,
    Group      = 1<<10,
    Message    = 1<<11,
    Bytes      = 1<<12,
    Uint32     = 1<<13,
    Enum       = 1<<14,
    Sfixed32   = 1<<15,
    Sfixed64   = 1<<16,
    Sint32     = 1<<17,
    Sint64     = 1<<18,

    /// Enum variant with no data
    NoDataEnumVariant = 1<<19,

    /// oneof field
    OneofField = 1<<20,

    /// map field
    MapField = 1<<21,
    Everything = u32::MAX,
}

implement_conversions!(
    FieldFilter,
    FieldSelector {
        Double, Float, Int64, Uint64, Int32, Fixed64,
        Fixed32, Bool, String, Group, Message, Bytes,
        Uint32, Enum, Sfixed32, Sfixed64, Sint32, Sint64,

        NoDataEnumVariant, OneofField, MapField,
    }
);

impl_from_type!(
    FieldSelector {
        Double, Float, Int64, Uint64, Int32, Fixed64,
        Fixed32, Bool, String, Group, Message, Bytes,
        Uint32, Enum, Sfixed32, Sfixed64, Sint32, Sint64,
    }
);
