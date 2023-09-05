use {
    crate::{
        conversion::ToJValue,
        java_object::{define_swt_object, JavaObject},
        jvm::JVM,
        widgets::dialog::DialogTrait,
        Shell,
    },
    convert_case::{Case, Casing},
    jni::{
        errors::Error,
        objects::{JObject, JValue},
    },
    swt_rs_macros::swt_fn,
};

define_swt_object!(DirectoryDialog);

pub trait DirectoryDialogTrait<'a>: JavaObject<'a>
{
    swt_fn!(get_filter_path, "()Ljava/lang/String;");
    swt_fn!(get_message, "()Ljava/lang/String;");
    swt_fn!(open, "()Ljava/lang/String;");
    swt_fn!(set_filter_path, "(Ljava/lang/String;)V", string);
    swt_fn!(set_message, "(Ljava/lang/String;)V", string);
}

impl<'a> DirectoryDialog<'a>
{
    // Constructor

    pub fn new<T>(parent: &Shell, style: i32) -> Result<Self, Error>
    where
        T: JavaObject<'a>,
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/widgets/DirectoryDialog")?;
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

impl<'a> DialogTrait<'a> for DirectoryDialog<'a> {}
impl<'a> DirectoryDialogTrait<'a> for DirectoryDialog<'a> {}
