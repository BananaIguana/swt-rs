use {
    crate::{
        conversion::ToJValue,
        java_object::{define_swt_object, JavaObject},
        jvm::JVM,
    },
    convert_case::{Case, Casing},
    jni::{
        errors::Error,
        objects::{JObject, JValue},
    },
    std::fmt::{Debug, Formatter},
    swt_rs_macros::swt_fn,
};

define_swt_object!(FontData);

impl<'a> FontData<'a>
{
    pub fn new() -> Result<Self, Error>
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/graphics/FontData")?;
        let jobj = jenv.new_object(jcls, "()V", &[])?;

        Ok(Self { jobj })
    }
}

pub trait FontDataTrait<'a>: JavaObject<'a>
{
    // swt_fn!(equals, "(Ljava/lang/Object;)Z");
    swt_fn!(get_height, "()I");
    swt_fn!(get_locale, "()Ljava/lang/String;");
    swt_fn!(get_name, "()Ljava/lang/String;");
    swt_fn!(get_style, "()I");
    swt_fn!(hash_code, "()I");
    swt_fn!(set_height, "(I)V", height);
    swt_fn!(set_locale, "(Ljava/lang/String;)V", locale);
    swt_fn!(set_name, "(Ljava/lang/String;)V", name);
    swt_fn!(set_style, "(I)V", style);
    swt_fn!(to_string, "()Ljava/lang/String;");
}

impl<'a> FontDataTrait<'a> for FontData<'a> {}

impl<'a> Debug for FontData<'a>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "FontData")
    }
}
