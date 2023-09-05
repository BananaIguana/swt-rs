use {
    crate::{
        java_object::{define_swt_object, JavaObject},
        jvm::JVM,
    },
    jni::{errors::Error, objects::JObject},
};

define_swt_object!(DeviceData);

impl<'a> DeviceData<'a>
{
    pub fn new() -> Result<Self, Error>
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/graphics/DeviceData")?;
        let jobj = jenv.new_object(jcls, "()V", &[])?;

        Ok(Self { jobj })
    }
}

pub trait DeviceDataTrait<'a>: JavaObject<'a>
{
    // swt_field!(debug, "()Z");
    // swt_field!(errors, "?");
    // swt_field!(objects, "?");
    // swt_field!(tracking, "()Z");
}

impl<'a> DeviceDataTrait<'a> for DeviceData<'a> {}
