use {
    crate::{
        conversion::ToJValue,
        graphics::{color::Color, device_data::DeviceData, font::Font, rectangle::Rectangle},
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

define_swt_object!(Device);

impl<'a> Device<'a>
{
    // Constructor

    pub fn new() -> Result<Self, Error>
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/graphics/Device")?;
        let jobj = jenv.new_object(jcls, "()", &[])?;

        Ok(Self { jobj })
    }
}

pub trait DeviceTrait<'a>: JavaObject<'a>
{
    swt_fn!(dispose, "()V");
    swt_fn!(get_bounds, "()Lorg/eclipse/swt/graphics/Rectangle;");
    swt_fn!(get_client_area, "()Lorg/eclipse/swt/graphics/Rectangle;");
    swt_fn!(get_depth, "()I");
    swt_fn!(get_device_data, "()Lorg/eclipse/swt/graphics/DeviceData;");
    // Point	getDPI()
    // FontData[]	getFontList(String faceName, boolean scalable)
    swt_fn!(
        get_system_color,
        "(I)Lorg/eclipse/swt/graphics/Color;",
        color_id
    );
    swt_fn!(get_system_font, "()Lorg/eclipse/swt/graphics/Font;");
    swt_fn!(get_warnings, "()Z");
    swt_fn!(is_disposed, "()Z");
    swt_fn!(is_tracking, "()Z");
    swt_fn!(load_font, "(Ljava/lang/String;)Z", path);
    swt_fn!(set_tracking, "(Z)V", tracking);
    swt_fn!(set_warnings, "(Z)V", warnings);
}

impl<'a> DeviceTrait<'a> for Device<'a> {}
