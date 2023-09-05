use {
    crate::{
        conversion::ToJValue,
        graphics::image::Image,
        java_object::{define_swt_object, JavaObject},
        jvm::JVM,
        widgets::{control::Control, expand_bar::ExpandBar},
    },
    convert_case::{Case, Casing},
    jni::{
        errors::Error,
        objects::{JObject, JValue},
    },
    swt_rs_macros::swt_fn,
};

define_swt_object!(ExpandItem);

pub trait ExpandItemTrait<'a>: JavaObject<'a>
{
    swt_fn!(get_control, "()Lorg/eclipse/swt/widgets/Control;");
    swt_fn!(get_expanded, "()Z");
    swt_fn!(get_header_height, "()I");
    swt_fn!(get_height, "()I");
    swt_fn!(get_parent, "()Lorg/eclipse/swt/widgets/ExpandBar;");

    swt_fn!(set_control, "(Lorg/eclipse/swt/widgets/Control;)V", control);
    swt_fn!(set_expanded, "(Z)V", expanded);

    swt_fn!(set_height, "(I)Z", height);
    swt_fn!(set_image, "(Lorg/eclipse/swt/graphics/Image;)Z", image);
    swt_fn!(set_text, "(Ljava/lang/String;)V", string);
}

impl<'a> ExpandItem<'a>
{
    pub fn new(parent: &ExpandBar, style: i32) -> Result<Self, Error>
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/widgets/ExpandItem")?;
        let jobj = jenv.new_object(
            jcls,
            "(Lorg/eclipse/swt/widgets/ExpandBar;I)V",
            &[
                jni::objects::JValue::Object(parent.get_object()),
                jni::objects::JValue::Int(style),
            ],
        )?;

        Ok(Self { jobj })
    }

    pub fn new2(parent: &ExpandBar, style: i32, index: i32) -> Result<Self, Error>
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/widgets/ExpandItem")?;
        let jobj = jenv.new_object(
            jcls,
            "(Lorg/eclipse/swt/widgets/ExpandBar;II)V",
            &[
                jni::objects::JValue::Object(parent.get_object()),
                jni::objects::JValue::Int(style),
                jni::objects::JValue::Int(index),
            ],
        )?;

        Ok(Self { jobj })
    }
}

// impl<'a> WidgetTrait<'a> for ExpandItem<'a> {}
// impl<'a> ItemTrait<'a> for ExpandItem<'a> {}
impl<'a> ExpandItemTrait<'a> for ExpandItem<'a> {}
