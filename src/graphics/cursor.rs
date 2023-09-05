use {
    crate::{
        graphics::resource::ResourceTrait,
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

define_swt_object!(Cursor);

impl<'a> Cursor<'a>
{
    pub fn new() -> Result<Self, Error>
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/graphics/Cursor")?;
        let jobj = jenv.new_object(jcls, "()", &[])?;

        Ok(Self { jobj })
    }
}

pub trait CursorTrait<'a>: JavaObject<'a>
{
    // swt_fn!(equals, "(Ljava/lang/Object;)Z");
    swt_fn!(hash_code, "()I");
    swt_fn!(is_disposed, "()Z");
    swt_fn!(to_string, "()Ljava/lang/String;");
}

impl<'a> ResourceTrait<'a> for Cursor<'a> {}
impl<'a> CursorTrait<'a> for Cursor<'a> {}
