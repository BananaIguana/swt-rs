use {
    crate::{java_object::JavaObject, jvm::JVM},
    jni::objects::{JObject, JValue},
};

/// Trait simplify conversion to JValue.
pub trait ToJValue<'a>
{
    fn to_jvalue(&self) -> JValue<'a>;

    fn from_jobject(jobject: JObject<'a>) -> Self;
}

/// Generates a trait implementation for primitive types.
macro_rules! jvalue_primitive_type_helper {
    ($jt:expr, $rt:ty) => {

        impl<'a> ToJValue<'a> for $rt {

            fn to_jvalue(&self) -> JValue<'a> { $jt((*self).try_into().unwrap()) }

            fn from_jobject(_jobject: JObject<'a>) -> Self
            {
               todo!();
            }
        }
    };
}

// Java signature character '`Z`'
// `JavaType` '`Bool`'
// Rust type '`bool`'
jvalue_primitive_type_helper!(JValue::Bool, bool);

// Java signature character '`B`'
// `JavaType` '`Byte`'
// Rust type '`i8`'
jvalue_primitive_type_helper!(JValue::Byte, i8);

// Java signature character '`C`'
// `JavaType` '`Char`'
// Rust type '`u16`'
jvalue_primitive_type_helper!(JValue::Char, u16);

// Java signature character '`S`'
// `JavaType` '`Short`'
// Rust type '`i16`'
jvalue_primitive_type_helper!(JValue::Short, i16);

// Java signature character '`I`'
// `JavaType` '`Int`'
// Rust type '`i32`'
jvalue_primitive_type_helper!(JValue::Int, i32);

// Java signature character '`J`'
// `JavaType` '`Long`'
// Rust type '`i64`'
jvalue_primitive_type_helper!(JValue::Long, i64);

// Java signature character '`F`'
// `JavaType` '`Float`'
// Rust type '`f32`'
jvalue_primitive_type_helper!(JValue::Float, f32);

// Java signature character '`D`'
// `JavaType` '`Double`'
// Rust type '`f64`'
jvalue_primitive_type_helper!(JValue::Double, f64);

// Java signature character '`L`'
// `JavaType` '`Object`'
impl<'a> ToJValue<'a> for JObject<'a>
{
    fn to_jvalue(&self) -> JValue<'a>
    {
        JValue::Object(*self)
    }

    fn from_jobject(_jobject: JObject<'a>) -> Self
    {
        todo!()
    }
}

// Java signature character '`V`'
// `JavaType` '`Void`'
// Rust type '`()`'
impl<'a> ToJValue<'a> for ()
{
    fn to_jvalue(&self) -> JValue<'a>
    {
        JValue::Void
    }

    fn from_jobject(_jobject: JObject<'a>) -> Self
    {
        todo!()
    }
}

impl<'a> ToJValue<'a> for String
{
    fn to_jvalue(&self) -> JValue<'a>
    {
        let jenv = JVM.get_env().unwrap();
        let f = jenv.new_string(self).unwrap();
        let jobj = JObject::from(f);
        JValue::Object(jobj)
    }

    fn from_jobject(_jobject: JObject<'a>) -> Self
    {
        todo!()
    }
}

pub fn jobject_to_array<'a, T: JavaObject<'a> + std::convert::From<jni::objects::JObject<'a>>>(jvalue: JValue) -> Vec<T>
{
    let jenv = JVM.get_env().unwrap();

    match jvalue
    {
        JValue::Object(obj) =>
        {
            let obj = obj.into_raw();

            let length = jenv.get_array_length(obj).unwrap();

            let mut store = Vec::<T>::new();

            for i in 0..length
            {
                let element = jenv.get_object_array_element(obj, i).unwrap();

                let robj: T = element.into();

                store.push(robj);
            }

            store
        }
        _ => Vec::new(),
    }
}
