use {
    crate::{
        java_object::{define_swt_object, JavaObject},
        jvm::JVM,
        widgets::{dialog::DialogTrait, label::Label},
    },
    jni::{errors::Error, objects::JObject},
};

define_swt_object!(ColorDialog);

pub trait ColorDialogTrait<'a>: JavaObject<'a>
{
    // swt_fn!(get_RGB, "()Lorg/eclipse/swt/graphics/RGB;");
    // swt_fn!(get_RGBs, "()[Lorg/eclipse/swt/graphics/RGB;");
    // swt_fn!(open, "()Lorg/eclipse/swt/graphics/RGB;");
    // swt_fn!(set_RGB, "(Lorg/eclipse/swt/graphics/RGB;)V", rgb);
    // swt_fn!(set_RGBs, "([Lorg/eclipse/swt/graphics/RGB;)V", rgbs);
}

impl<'a> ColorDialog<'a>
{
    // Constructor

    pub fn new_with_style<T>(parent: &T, style: i32) -> Result<Self, Error>
    where
        T: JavaObject<'a>,
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/widgets/ColorDialog")?;
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

    pub fn new<T>(parent: &T) -> Result<Self, Error>
    where
        T: JavaObject<'a>,
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/widgets/ColorDialog")?;
        let jobj = jenv.new_object(
            jcls,
            "(Lorg/eclipse/swt/widgets/Composite;)V",
            &[jni::objects::JValue::Object(parent.get_object())],
        )?;

        Ok(Self { jobj })
    }
}

impl<'a> DialogTrait<'a> for Label<'a> {}
