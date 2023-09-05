use {
    crate::{
        conversion::ToJValue,
        java_object::{define_swt_object, JavaObject},
        jvm::JVM,
        widgets::control::ControlTrait,
    },
    convert_case::{Case, Casing},
    jni::{
        errors::Error,
        objects::{JObject, JValue},
    },
    swt_rs_macros::swt_fn,
};

define_swt_object!(Button);

pub trait ButtonTrait<'a>: JavaObject<'a>
{
    //  swt_fn!(add_selection_listener, "(Lorg/eclipse/swt/events/SelectionListener;)V", listener);
    swt_fn!(get_alignment, "()I");
    swt_fn!(get_grayed, "()Z");
    //   swt_fn!(get_image, "()Lorg/eclipse/swt/graphics/Image;");
    swt_fn!(get_selection, "()Z");
    swt_fn!(get_text, "()Ljava/lang/String;");
    //  swt_fn!(remove_selection_listener, "(Lorg/eclipse/swt/events/SelectionListener;)V", listener);
    swt_fn!(set_alignment, "(I)V", alignment);
    //  swt_fn!(set_background, "(Lorg/eclipse/swt/graphics/Color;)V", background);
    swt_fn!(set_focus, "()Z");
    swt_fn!(set_grayed, "(Z)V", grayed);
    //  swt_fn!(set_image, "(Lorg/eclipse/swt/graphics/Image;)V", image);
    swt_fn!(set_selection, "(Z)V", selected);
    swt_fn!(set_text, "(Ljava/lang/String;)V", string);
}

impl<'a> Button<'a>
{
    // Constructor

    pub fn new<T>(parent: &T, style: i32) -> Result<Self, Error>
    where
        T: JavaObject<'a>,
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/widgets/Button")?;
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

//impl<'a> WidgetTrait<'a> for Button<'a> {}
impl<'a> ControlTrait<'a> for Button<'a> {}
impl<'a> ButtonTrait<'a> for Button<'a> {}
