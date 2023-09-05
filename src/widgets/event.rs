use {
    crate::{
        conversion::{jobject_to_array, ToJValue},
        graphics::font_data::FontData,
        java_object::{define_swt_object, JavaObject},
        jvm::JVM,
        Display,
        Rectangle,
    },
    convert_case::{Case, Casing},
    jni::{
        errors::Error,
        objects::{JObject, JValue},
    },
    swt_rs_macros::{swt_field, swt_fn},
};

define_swt_object!(Event);

pub trait EventTrait<'a>: JavaObject<'a>
{
    swt_fn!(get_bounds, "()Lorg/eclipse/swt/graphics/Rectangle;");
    swt_fn!(set_bounds, "(Lorg/eclipse/swt/graphics/Rectangle;)V", rect);
    swt_fn!(to_string, "()Ljava/lang/String;");

    swt_field!(button, "I");
    swt_field!(character, "C");
    swt_field!(count, "I");
    // swt_field!(data, "I");
    swt_field!(detail, "I");
    swt_field!(display, "Lorg/eclipse/swt/widgets/Display;");
    swt_field!(doit, "Z");
    swt_field!(end, "I");
    // swt_field!(gc, "Lorg/eclipse/swt/graphics/GC;");
    swt_field!(height, "I");
    swt_field!(index, "I");
    // swt_field!(item, "Lorg/eclipse/swt/widgets/Widget;");
    swt_field!(key_code, "I");
    swt_field!(key_location, "I");
    swt_field!(magnification, "D");
    swt_field!(rotation, "D");
    swt_field!(segments, "[I");
    swt_field!(segments_chars, "[C");
    swt_field!(start, "I");
    swt_field!(state_mask, "I");
    swt_field!(text, "Ljava/lang/String;");
    swt_field!(time, "I");
    swt_field!(touches, "[Lorg/eclipse/swt/widgets/Touch;");
    // swt_field!(type, "I");
    // swt_field!(widget, "Lorg/eclipse/swt/widgets/Widget;");
    swt_field!(width, "I");
    swt_field!(x, "I");
    swt_field!(x_direction, "I");
    swt_field!(y, "I");
    swt_field!(y_direction, "I");
}

impl<'a> Event<'a>
{
    pub fn new() -> Result<Self, Error>
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/widgets/Event")?;
        let jobj = jenv.new_object(jcls, "()V", &[])?;

        Ok(Self { jobj })
    }
}
