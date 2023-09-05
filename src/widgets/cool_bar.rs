use {
    crate::{
        conversion::{jobject_to_array, ToJValue},
        graphics::font_data::FontData,
        java_object::{define_swt_object, JavaObject},
        jvm::JVM,
        widgets::{composite::CompositeTrait, control::ControlTrait, cool_item::CoolItem},
    },
    convert_case::{Case, Casing},
    jni::{
        errors::Error,
        objects::{JObject, JValue},
    },
    swt_rs_macros::swt_fn,
};

define_swt_object!(CoolBar);

pub trait CoolBarTrait<'a>: JavaObject<'a>
{
    swt_fn!(get_item, "(I)Lorg/eclipse/swt/widgets/CoolItem;", index);
    swt_fn!(get_item_count, "()I");
    swt_fn!(get_item_order, "()[I");
    swt_fn!(get_items, "()[Lorg/eclipse/swt/widgets/CoolItem;");
    swt_fn!(get_item_sizes, "()[Lorg/eclipse/swt/graphics/Point;");
    swt_fn!(get_locked, "()Z");
    swt_fn!(get_wrap_indices, "()[I");
    swt_fn!(index_of, "(Lorg/eclipse/swt/widgets/CoolItem;)I", item);
    // swt_fn!(set_item_layout, "([I[I[Lorg/eclipse/swt/graphics/Point;)V", item_order, wrap_indices, sizes);
    swt_fn!(set_locked, "(Z)V", locked);
    // swt_fn!(set_wrap_indices, "([I)V", indices);
}

impl<'a> CoolBar<'a>
{
    // Constructor

    pub fn new<T>(parent: &T, style: i32) -> Result<Self, Error>
    where
        T: JavaObject<'a>,
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/widgets/CoolBar")?;
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

// impl<'a> WidgetTrait<'a> for CoolBar<'a> {}
impl<'a> ControlTrait<'a> for CoolBar<'a> {}
// impl<'a> ScrollableTrait<'a> for CoolBar<'a> {}
impl<'a> CompositeTrait<'a> for CoolBar<'a> {}
impl<'a> CoolBarTrait<'a> for CoolBar<'a> {}
