use {
    crate::{
        conversion::ToJValue,
        graphics::{font::Font, image::Image, point::Point},
        java_object::{define_swt_object, JavaObject},
        jvm::JVM,
        widgets::canvas::Canvas,
        Rectangle,
    },
    convert_case::{Case, Casing},
    jni::{
        errors::Error,
        objects::{JObject, JValue},
    },
    swt_rs_macros::swt_fn,
};

define_swt_object!(Caret);

pub trait CaretTrait<'a>: JavaObject<'a>
{
    swt_fn!(get_bounds, "()Lorg/eclipse/swt/graphics/Rectangle;");
    swt_fn!(get_font, "()Lorg/eclipse/swt/graphics/Font;");
    swt_fn!(get_image, "()Lorg/eclipse/swt/graphics/Image;");
    swt_fn!(get_location, "()Lorg/eclipse/swt/graphics/Point;");
    swt_fn!(get_parent, "()Lorg/eclipse/swt/widgets/Canvas;");
    swt_fn!(get_size, "()Lorg/eclipse/swt/graphics/Point;");
    swt_fn!(get_visible, "()Z");
    swt_fn!(is_visible, "()Z");
    swt_fn!(set_bounds, "(IIII)V", x, y, width, height);
    swt_fn!(
        set_bounds2,
        "(Lorg/eclipse/swt/graphics/Rectangle;)V",
        rectangle
    );
    swt_fn!(set_font, "(Lorg/eclipse/swt/graphics/Font;)V", font);
    swt_fn!(set_image, "(Lorg/eclipse/swt/graphics/Image;)V", image);
    swt_fn!(set_location, "(II)V", x, y);
    swt_fn!(
        set_location2,
        "(Lorg/eclipse/swt/graphics/Point;)V",
        location
    );
    swt_fn!(set_size, "(II)V", width, height);
    swt_fn!(set_size2, "(Lorg/eclipse/swt/graphics/Point;)V", size);
    swt_fn!(set_visible, "(Z)V", visible);
}

impl<'a> Caret<'a>
{
    // Constructor

    pub fn new<T>(parent: &T, style: i32) -> Result<Self, Error>
    where
        T: JavaObject<'a>,
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/widgets/Caret")?;
        let jobj = jenv.new_object(
            jcls,
            "(Lorg/eclipse/swt/widgets/Composite;I)V",
            &[
                jni::objects::JValue::Object(parent.get_object()),
                jni::objects::JValue::Int(style),
            ],
        )?;

        Ok(Self { jobj })
    }
}

// impl<'a> WidgetTrait<'a> for Canvas<'a> {}
