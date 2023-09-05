use jni::objects::JObject;

pub trait JavaObject<'a>
{
    fn get_object(&self) -> JObject<'_>;
}

/// Generates a trait implementation for primitive types.
macro_rules! define_swt_object {
    ($name:tt) => {
        pub struct $name<'a>
        {
            jobj : JObject<'a>,
        }

        impl<'a> JavaObject<'a> for $name<'a>
        {
            fn get_object(&self) -> JObject<'a>
            {
                self.jobj
            }
        }

        impl<'a> From<JObject<'a>> for $name<'a>
        {
            fn from(jobj: JObject<'a>) -> Self
            {
                $name { jobj }
            }
        }
    };
}

pub(crate) use define_swt_object;
