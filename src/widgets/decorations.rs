use {
    crate::{
        conversion::{jobject_to_array, ToJValue},
        graphics::{font_data::FontData, image::Image},
        java_object::{define_swt_object, JavaObject},
        jvm::JVM,
        widgets::{button::Button, canvas::CanvasTrait, composite::CompositeTrait, control::ControlTrait},
    },
    convert_case::{Case, Casing},
    jni::{
        errors::Error,
        objects::{JObject, JValue},
    },
    swt_rs_macros::swt_fn,
};

define_swt_object!(Decorations);

pub trait DecorationsTrait<'a>: JavaObject<'a>
{
    swt_fn!(dispose, "()V");
    swt_fn!(get_default_button, "()Lorg/eclipse/swt/widgets/Button;");
    swt_fn!(get_image, "()Lorg/eclipse/swt/graphics/Image;");
    swt_fn!(get_images, "()[Lorg/eclipse/swt/graphics/Image;");
    swt_fn!(get_maximized, "()Z");
    // swt_fn!(get_menu_bar, "()Lorg/eclipse/swt/widgets/Menu;");
    swt_fn!(get_minimized, "()Z");
    swt_fn!(get_text, "()Ljava/lang/String;");
    swt_fn!(is_reparentable, "()Z");
    swt_fn!(
        set_default_button,
        "(Lorg/eclipse/swt/widgets/Button;)V",
        button
    );
    swt_fn!(set_image, "(Lorg/eclipse/swt/widgets/Image;)V", image);
    swt_fn!(set_images, "([Lorg/eclipse/swt/widgets/Image;)V", images);
    swt_fn!(set_maximized, "(Z)V", maximized);
    // swt_fn!(set_menu_bar, "(Lorg/eclipse/swt/widgets/Menu;)V", menu);
    swt_fn!(set_minimized, "(Z)V", minimized);
    swt_fn!(set_orientation, "(I)V", orientation);
    swt_fn!(set_text, "(Ljava/lang/String;)V", string);
    swt_fn!(set_visible, "(Z)V", visible);
}

impl<'a> Decorations<'a>
{
    // Constructor

    pub fn new<T>(parent: &T, style: i32) -> Result<Self, Error>
    where
        T: JavaObject<'a>,
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/widgets/Decorations")?;
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

// impl<'a> WidgetTrait<'a> for Decorations<'a> {}
impl<'a> ControlTrait<'a> for Decorations<'a> {}
// impl<'a> ScrollableTrait<'a> for Decorations<'a> {}
// impl<'a> ScrollableTrait<'a> for Decorations<'a> {}
impl<'a> CompositeTrait<'a> for Decorations<'a> {}
impl<'a> CanvasTrait<'a> for Decorations<'a> {}
impl<'a> DecorationsTrait<'a> for Decorations<'a> {}
