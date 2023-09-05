use {
    crate::{
        conversion::ToJValue,
        graphics::{color::Color, device::Device, resource::ResourceTrait},
        java_object::{define_swt_object, JavaObject},
        jvm::JVM,
        Rectangle,
    },
    convert_case::{Case, Casing},
    jni::{
        errors::Error,
        objects::{JObject, JValue},
    },
    swt_rs_macros::{swt_field, swt_fn},
};

define_swt_object!(Image);

impl<'a> Image<'a>
{
    pub fn new(_device: &Device, width: i32, height: i32) -> Result<Self, Error>
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/graphics/Image")?;
        let jobj = jenv.new_object(
            jcls,
            "(Lorg/eclipse/swt/graphics/Device;II)V",
            &[jni::objects::JValue::Int(width), jni::objects::JValue::Int(height)],
        )?;

        Ok(Self { jobj })
    }

    swt_field!(handle, "J");
    swt_field!(_type, "I");
}

pub trait ImageTrait<'a>: JavaObject<'a>
{
    // swt_fn!(equals, "(Ljava/lang/Object;)Z", object);
    swt_fn!(get_background, "()Lorg/eclipse/swt/graphics/Color;");
    swt_fn!(get_bounds, "()Lorg/eclipse/swt/graphics/Rectangle;");
    swt_fn!(
        get_bounds_in_pixels,
        "()Lorg/eclipse/swt/graphics/Rectangle;"
    );
    // swt_fn!(get_image_data, "()Lorg/eclipse/swt/graphics/ImageData;");
    // swt_fn!(get_image_data2, "(I)Lorg/eclipse/swt/graphics/ImageData;", zoom);
    // swt_fn!(get_image_data_at_current_zoom, "()Lorg/eclipse/swt/graphics/ImageData;");
    swt_fn!(hash_code, "()I");
    //  swt_fn!(internal_dispose_GC, "(JLorg/eclipse/swt/graphics/GCData;)V", hDC, data);
    //  swt_fn!(internal_new_GC, "(Lorg/eclipse/swt/graphics/GCData;)J", data);
    swt_fn!(is_disposed, "()Z");
    swt_fn!(set_background, "(Lorg/eclipse/swt/graphics/Color;)V", color);
    swt_fn!(to_string, "()Ljava/lang/String;");
    //  static Image win32_new(Device device, int type, long handle)
}

impl<'a> ResourceTrait<'a> for Image<'a> {}
impl<'a> ImageTrait<'a> for Image<'a> {}
