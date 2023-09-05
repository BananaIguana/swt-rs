use {
    crate::{
        conversion::{jobject_to_array, ToJValue},
        graphics::{device::Device, font_data::FontData, resource::ResourceTrait},
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

define_swt_object!(Font);

impl<'a> Font<'a>
{
    pub fn from_description(device: &Device, name: &String, height: i32, style: i32) -> Result<Self, Error>
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/graphics/Font")?;
        let jobj = jenv.new_object(
            jcls,
            "(Lorg/eclipse/swt/graphics/Device;Ljava/lang/String;II)V",
            &[
                device.get_object().to_jvalue(),
                name.to_jvalue(),
                height.to_jvalue(),
                style.to_jvalue(),
            ],
        )?;

        Ok(Self { jobj })
    }

    pub fn from_font_data<T>(device: &T, font_data: &FontData) -> Result<Self, Error>
    where
        T: JavaObject<'a>,
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/graphics/Font")?;
        let jobj = jenv.new_object(
            jcls,
            "(Lorg/eclipse/swt/graphics/Device;Lorg/eclipse/swt/graphics/FontData;)V",
            &[device.get_object().to_jvalue(), font_data.get_object().to_jvalue()],
        )?;

        Ok(Self { jobj })
    }

    // pub fn from_font_data_array<T>(device: &T, font_data: &[FontData]) -> Result<Self, Error> where T: JavaObject<'a>
    // {
    //     let jenv = JVM.get_env()?;
    //     let jcls = jenv.find_class("org/eclipse/swt/graphics/Font")?;
    //     let jobj = jenv.new_object(
    //         jcls,
    //         "(Lorg/eclipse/swt/graphics/Device;[Lorg/eclipse/swt/graphics/FontData;)V",
    //         &[
    //             device.get_object().to_jvalue(),
    //             font_data.get_object().to_jvalue(),
    //         ])?;
    //
    //     Ok(Self { jobj })
    // }
}

pub trait FontTrait<'a>: JavaObject<'a>
{
    // swt_fn!(equals, "(Ljava/lang/Object;)Z");
    swt_fn!(get_font_data, "()[Lorg/eclipse/swt/graphics/FontData;");
    swt_fn!(hash_code, "()I");
    swt_fn!(is_disposed, "()Z");
    swt_fn!(to_string, "()Ljava/lang/String;");
}

impl<'a> ResourceTrait<'a> for Font<'a> {}
impl<'a> FontTrait<'a> for Font<'a> {}
