use {
    crate::{
        conversion::ToJValue,
        graphics::point::Point,
        java_object::{define_swt_object, JavaObject},
        jvm::JVM,
        widgets::{control::Control, cool_bar::CoolBar},
        Rectangle,
    },
    convert_case::{Case, Casing},
    jni::{
        errors::Error,
        objects::{JObject, JValue},
    },
    swt_rs_macros::swt_fn,
};

define_swt_object!(CoolItem);

pub trait CoolItemTrait<'a>: JavaObject<'a>
{
    // swt_fn!(add_selection_listener, "(Lorg/eclipse/swt/events/SelectionListener;)V", listener);
    swt_fn!(compute_size, "(II)V", wHint, hHint);
    swt_fn!(get_bounds, "()Lorg/eclipse/swt/graphics/Rectangle;");
    swt_fn!(get_control, "()Lorg/eclipse/swt/widgets/Control;");
    swt_fn!(get_minimum_size, "()Lorg/eclipse/swt/graphics/Point;");
    swt_fn!(get_parent, "()Lorg/eclipse/swt/widgets/CoolBar;");
    swt_fn!(get_preferred_size, "()Lorg/eclipse/swt/graphics/Point;");
    swt_fn!(get_size, "()Lorg/eclipse/swt/graphics/Point;");
    // swt_fn!(remove_selection_listener, "(Lorg/eclipse/swt/events/SelectionListener;)V", listener);
    swt_fn!(set_control, "(Lorg/eclipse/swt/widgets/Control;)V", control);
    swt_fn!(set_minimum_size, "(II)V", width, height);
    swt_fn!(
        set_minimum_size2,
        "(Lorg/eclipse/swt/graphics/Point;)V",
        point
    );
    swt_fn!(set_preferred_size, "(II)V", width, height);
    swt_fn!(
        set_preferred_size2,
        "(Lorg/eclipse/swt/graphics/Point;)V",
        point
    );
    swt_fn!(set_size, "(II)V", width, height);
    swt_fn!(set_size2, "(Lorg/eclipse/swt/graphics/Point;)V", point);
}

impl<'a> CoolItem<'a>
{
    // Constructor

    pub fn new<T>(parent: &T, style: i32) -> Result<Self, Error>
    where
        T: JavaObject<'a>,
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/widgets/CoolItem")?;
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

// impl<'a> WidgetTrait<'a> for CoolItem<'a> {}
// impl<'a> ItemTrait<'a> for CoolItem<'a> {}
impl<'a> CoolItemTrait<'a> for CoolBar<'a> {}
