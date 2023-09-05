use std::fmt::Formatter;

/// Representation of a signature. These types match to [Java JNI signature types](https://docs.oracle.com/javase/7/docs/technotes/guides/jni/spec/types.html)
#[derive(PartialEq)]
pub enum JavaType
{
    Bool,
    Byte,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Void,
    String,
    SwtObject(String),
    Array(Box<JavaType>),
}

impl std::fmt::Debug for JavaType
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        let type_as_string = |t: &JavaType| -> String {
            match t
            {
                JavaType::Bool => format!("JavaType::Bool"),
                JavaType::Byte => format!("JavaType::Byte"),
                JavaType::Char => format!("JavaType::Char"),
                JavaType::Short => format!("JavaType::Short"),
                JavaType::Int => format!("JavaType::Int"),
                JavaType::Long => format!("JavaType::Long"),
                JavaType::Float => format!("JavaType::Float"),
                JavaType::Double => format!("JavaType::Double"),
                JavaType::Void => format!("JavaType::Void"),
                JavaType::String => format!("JavaType::String"),
                JavaType::SwtObject(obj) => format!("JavaType::SwtObject({})", obj),
                _ => panic!("Unhandled type"),
            }
        };

        let str = match self
        {
            JavaType::Array(java_type) => format!("JavaType::Array({})", type_as_string(java_type)),
            _ => type_as_string(self),
        };

        write!(f, "{}", str)
    }
}

/// Clone cannot be derived automatically due to the presence of a `String`.
impl Clone for JavaType
{
    fn clone(&self) -> Self
    {
        match self
        {
            JavaType::Bool => JavaType::Bool,
            JavaType::Byte => JavaType::Byte,
            JavaType::Char => JavaType::Char,
            JavaType::Short => JavaType::Short,
            JavaType::Int => JavaType::Int,
            JavaType::Long => JavaType::Long,
            JavaType::Float => JavaType::Float,
            JavaType::Double => JavaType::Double,
            JavaType::Void => JavaType::Void,
            JavaType::String => JavaType::String,
            JavaType::SwtObject(str) => JavaType::SwtObject(str.clone()),
            JavaType::Array(array_type) => JavaType::Array(array_type.clone()),
        }
    }

    fn clone_from(&mut self, source: &Self)
    {
        *self = source.clone();
    }
}

impl JavaType
{
    /// Converts this `PrimitiveType` to the string of the Rust equivalent.
    pub fn rust_type_str(&self) -> &str
    {
        match self
        {
            JavaType::Bool => "bool",
            JavaType::Byte => "i8",
            JavaType::Char => "u16",
            JavaType::Short => "i16",
            JavaType::Int => "i32",
            JavaType::Long => "i64",
            JavaType::Float => "f32",
            JavaType::Double => "f64",
            JavaType::Void => "()",
            JavaType::String => "String",
            JavaType::SwtObject(str) => str.as_str(),
            JavaType::Array(_array_type) => "Vec<FontData>",
        }
    }
}
