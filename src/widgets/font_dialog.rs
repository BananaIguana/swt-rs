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

define_swt_object!(FontDialog);

pub trait FontDialogTrait<'a>: JavaObject<'a>
{
    swt_fn!(get_effects_visible, "()Z");
    swt_fn!(get_font_data, "()Lorg/eclipse/swt/graphics/FontData;");
    swt_fn!(get_font_list, "()[Lorg/eclipse/swt/graphics/FontData;");
    // TODO: Need string conversion override.
    // swt_fn!(getRGB, "()Lorg/eclipse/swt/graphics/RGB;");
    swt_fn!(open, "()Lorg/eclipse/swt/graphics/FontData;");
    swt_fn!(set_effects_visible, "(Z)V", visible);
    swt_fn!(
        set_font_data,
        "(Lorg/eclipse/swt/graphics/FontData;)V",
        font_data
    );
    swt_fn!(
        set_font_list,
        "([Lorg/eclipse/swt/graphics/FontData;)V",
        font_data
    );
    // TODO: Need string conversion override.
    // swt_fn!(set_RGB, "(Lorg/eclipse/swt/graphics/RGB;)V");
}

impl<'a> FontDialog<'a>
{
    pub fn new(parent: &Shell) -> Result<Self, Error>
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/widgets/FontDialog")?;
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
        let jcls = jenv.find_class("org/eclipse/swt/widgets/FontDialog")?;
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

impl<'a> DialogTrait<'a> for FontDialog<'a> {}
// impl<'a> FontDialogTraitTrait<'a> for FontDialog<'a> {}
