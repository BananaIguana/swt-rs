use {
    crate::{
        java_object::{define_swt_object, JavaObject},
        jvm::JVM,
    },
    convert_case::{Case, Casing},
    jni::{
        errors::Error,
        objects::{JObject, JValue},
    },
    swt_rs_macros::swt_fn,
};

define_swt_object!(Point);

impl<'a> Point<'a>
{
    // Constructor

    pub fn new(x: i32, y: i32) -> Result<Self, Error>
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/graphics/Point")?;
        let jobj = jenv.new_object(
            jcls,
            "(II)V",
            &[jni::objects::JValue::Int(x), jni::objects::JValue::Int(y)],
        )?;

        Ok(Self { jobj })
    }

    // Fields

    // swt_field!(x, "I");
    // swt_field!(y, "I");

    // Methods

    // boolean equals(Object object)
    swt_fn!(hash_code, "()I");
    swt_fn!(to_string, "()Ljava/lang/String;");
}
