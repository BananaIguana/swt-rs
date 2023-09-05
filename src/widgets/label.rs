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

define_swt_object!(Label);

pub trait LabelTrait<'a>: JavaObject<'a>
{
    swt_fn!(get_alignment, "()I");
    // swt_fn!(get_image, "()Lorg/eclipse/swt/widgets/Image;");
    swt_fn!(get_text, "()Ljava/lang/String;");
    swt_fn!(set_alignment, "(I)V", alignment);
    swt_fn!(set_enabled, "(Z)V", image);
    // swt_fn!(set_image, "(Lorg/eclipse/swt/widgets/Image;)V", image);
    swt_fn!(set_text, "(Ljava/lang/String;)V", string);
}

impl<'a> Label<'a>
{
    // Constructor

    pub fn new<T>(parent: &T, style: i32) -> Result<Self, Error>
    where
        T: JavaObject<'a>,
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/widgets/Label")?;
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

impl<'a> LabelTrait<'a> for Label<'a> {}
impl<'a> ControlTrait<'a> for Label<'a> {}
