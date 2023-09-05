use {
    crate::{
        conversion::{jobject_to_array, ToJValue},
        graphics::{font::Font, font_data::FontData},
        java_object::{define_swt_object, JavaObject},
        jvm::JVM,
        widgets::{composite::CompositeTrait, control::ControlTrait, expand_item::ExpandItem},
    },
    convert_case::{Case, Casing},
    jni::{
        errors::Error,
        objects::{JObject, JValue},
    },
    swt_rs_macros::swt_fn,
};

define_swt_object!(ExpandBar);

pub trait ExpandBarTrait<'a>: JavaObject<'a>
{
    // swt_fn!(add_expand_listener, "(Lorg/eclipse/swt/events/ExpandListener;)V", listener);
    swt_fn!(get_item, "(I)Lorg/eclipse/swt/widgets/ExpandItem;", index);
    swt_fn!(get_item_count, "()I");
    swt_fn!(get_items, "()[Lorg/eclipse/swt/widgets/ExpandItem;");
    swt_fn!(get_spacing, "()I");
    swt_fn!(index_of, "(Lorg/eclipse/swt/widgets/ExpandItem;)I", item);
    // swt_fn!(remove_expand_listener, "(Lorg/eclipse/swt/events/ExpandListener;)V", listener);
    swt_fn!(set_font, "(Lorg/eclipse/swt/graphics/Font;)V", font);
    swt_fn!(set_spacing, "(I)V", spacing);
}

impl<'a> ExpandBar<'a>
{
    pub fn new<T>(parent: &T, style: i32) -> Result<Self, Error>
    where
        T: JavaObject<'a>,
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/widgets/ExpandBar")?;
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

// impl<'a> WidgetTrait<'a> for ExpandBar<'a> {}
impl<'a> ControlTrait<'a> for ExpandBar<'a> {}
// impl<'a> ScrollableTrait<'a> for ExpandBar<'a> {}
impl<'a> CompositeTrait<'a> for ExpandBar<'a> {}
impl<'a> ExpandBarTrait<'a> for ExpandBar<'a> {}
