use {
    crate::{
        graphics::device::Device,
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

define_swt_object!(Resource);

impl<'a> Resource<'a>
{
    pub fn new() -> Result<Self, Error>
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/graphics/Resource")?;
        let jobj = jenv.new_object(jcls, "()V", &[])?;

        Ok(Self { jobj })
    }
}

pub trait ResourceTrait<'a>: JavaObject<'a>
{
    swt_fn!(dispose, "()V");
    swt_fn!(get_device, "()Lorg/eclipse/swt/graphics/Device;");
    // [abstract] -- swt_fn!(is_disposed, "()Z");
}

impl<'a> ResourceTrait<'a> for Resource<'a> {}
