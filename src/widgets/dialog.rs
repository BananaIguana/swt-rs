use {
    crate::{
        conversion::ToJValue,
        java_object::{define_swt_object, JavaObject},
        jvm::JVM,
        widgets::button::Button,
        Shell,
    },
    convert_case::{Case, Casing},
    jni::{
        errors::Error,
        objects::{JObject, JValue},
    },
    swt_rs_macros::swt_fn,
};

define_swt_object!(Dialog);

pub trait DialogTrait<'a>: JavaObject<'a>
{
    swt_fn!(get_parent, "()Lorg/eclipse/swt/widgets/Shell;");
    swt_fn!(get_style, "()I");
    swt_fn!(get_text, "()Ljava/lang/String;");
    swt_fn!(set_text, "(Ljava/lang/String;)V", string);
}

impl<'a> Dialog<'a>
{
    // Constructor

    pub fn new<T>(parent: &Shell, style: i32) -> Result<Self, Error>
    where
        T: JavaObject<'a>,
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/widgets/Dialog")?;
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

impl<'a> DialogTrait<'a> for Button<'a> {}
