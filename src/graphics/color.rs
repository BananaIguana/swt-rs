use {
    crate::{
        graphics::{device::Device, resource::ResourceTrait},
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

define_swt_object!(Color);

impl<'a> Color<'a>
{
    // Constructor(s)

    pub fn new(red: i32, green: i32, blue: i32) -> Result<Self, Error>
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/graphics/Color")?;
        let jobj = jenv.new_object(
            jcls,
            "(III)V",
            &[
                jni::objects::JValue::Int(red),
                jni::objects::JValue::Int(green),
                jni::objects::JValue::Int(blue),
            ],
        )?;

        Ok(Self { jobj })
    }
}

pub trait ColorTrait<'a>: JavaObject<'a>
{
    // Method(s)

    swt_fn!(dispose, "()V");
    // swt_fn!(equals, "(Ljava/lang/Object;)Z");
    swt_fn!(get_alpha, "()I");
    swt_fn!(get_blue, "()I");
    swt_fn!(get_device, "()Lorg/eclipse/swt/graphics/Device;");
    swt_fn!(get_green, "()I");
    swt_fn!(get_red, "()I");
    // RGB	getRGB()
    // RGBA	getRGBA()
    swt_fn!(hash_code, "()I");
    swt_fn!(is_disposed, "()Z");
    swt_fn!(to_string, "()Ljava/lang/String;");
}

impl<'a> ResourceTrait<'a> for Color<'a> {}
impl<'a> ColorTrait<'a> for Color<'a> {}
