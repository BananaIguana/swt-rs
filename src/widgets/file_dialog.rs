use {
    crate::{
        conversion::{jobject_to_array, ToJValue},
        graphics::font_data::FontData,
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

define_swt_object!(FileDialog);

pub trait FileDialogTrait<'a>: JavaObject<'a>
{
    swt_fn!(get_file_name, "()Ljava/lang/String;");
    swt_fn!(get_file_names, "()[Ljava/lang/String;");
    swt_fn!(get_filter_extensions, "()[Ljava/lang/String;");
    swt_fn!(get_filter_index, "()I");
    swt_fn!(get_filter_names, "()[Ljava/lang/String;");
    swt_fn!(get_filter_path, "()Ljava/lang/String;");
    swt_fn!(get_overwrite, "()Z");
    swt_fn!(open, "()Ljava/lang/String;");
    swt_fn!(set_file_name, "(Ljava/lang/String;)V", string);
    // swt_fn!(set_filter_extensions, "([Ljava/lang/String;)V", extensions);
    swt_fn!(set_filter_index, "(I)V", index);
    // swt_fn!(set_filter_names, "([Ljava/lang/String;)V", names);
    swt_fn!(set_filter_path, "(Ljava/lang/String;)V", string);
    swt_fn!(set_overwrite, "(Z)V", overwrite);
}

impl<'a> FileDialog<'a>
{
    pub fn new(parent: &Shell) -> Result<Self, Error>
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/widgets/FileDialog")?;
        let jobj = jenv.new_object(
            jcls,
            "(Lorg/eclipse/swt/widgets/Shell;)V",
            &[jni::objects::JValue::Object(parent.get_object())],
        )?;

        Ok(Self { jobj })
    }

    pub fn new2(parent: &Shell, style: i32) -> Result<Self, Error>
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/widgets/FileDialog")?;
        let jobj = jenv.new_object(
            jcls,
            "(Lorg/eclipse/swt/widgets/Shell;I)V",
            &[
                jni::objects::JValue::Object(parent.get_object()),
                jni::objects::JValue::Int(style),
            ],
        )?;

        Ok(Self { jobj })
    }
}

impl<'a> DialogTrait<'a> for FileDialog<'a> {}
impl<'a> FileDialogTrait<'a> for FileDialog<'a> {}
