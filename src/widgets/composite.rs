use {
    crate::{
        conversion::{jobject_to_array, ToJValue},
        graphics::font_data::FontData,
        java_object::{define_swt_object, JavaObject},
        jvm::JVM,
        widgets::control::{Control, ControlTrait},
    },
    convert_case::{Case, Casing},
    jni::{
        errors::Error,
        objects::{JObject, JValue},
    },
    swt_rs_macros::swt_fn,
};

define_swt_object!(Composite);

pub trait CompositeTrait<'a>: JavaObject<'a>
{
    swt_fn!(changed, "(Lorg/eclipse/swt/widgets/Control;)V", changed);
    // swt_fn!(draw_background, "(Lorg/eclipse/swt/graphics/GC;IIIIII)V", gc, x, y, width, height, offsetX, offsetY);
    swt_fn!(get_background_mode, "()I");
    swt_fn!(get_children, "()[Lorg/eclipse/swt/widgets/Control;");
    // swt_fn!(get_layout, "()Lorg/eclipse/swt/widgets/Layout;");
    swt_fn!(get_layout_deferred, "()Z");
    swt_fn!(get_tab_list, "()[Lorg/eclipse/swt/widgets/Control;");
    swt_fn!(is_layout_deferred, "()Z");
    swt_fn!(layout, "()V");
    swt_fn!(layout1, "(Z)V", changed);
    swt_fn!(layout2, "(ZZ)V", changed, all);
    swt_fn!(layout3, "([Lorg/eclipse/swt/widgets/Control;)V", changed);
    swt_fn!(
        layout4,
        "([Lorg/eclipse/swt/widgets/Control;I)V",
        changed,
        flags
    );
    swt_fn!(mode, "(I)V", mode);
    swt_fn!(set_focus, "()Z");
    // swt_fn!(set_layout, "(Lorg/eclipse/swt/widgets/Layout;)V", layout);
    swt_fn!(set_layout_deferred, "(Z)V", defer);
    swt_fn!(
        set_tab_list,
        "([Lorg/eclipse/swt/widgets/Control;)V",
        tabList
    );
    swt_fn!(to_string, "()Ljava/lang/String;");
}

impl<'a> Composite<'a>
{
    // Constructor

    pub fn new<T>(parent: &T, style: i32) -> Result<Self, Error>
    where
        T: JavaObject<'a>,
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/widgets/Composite")?;
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

// impl<'a> WidgetTrait<'a> for Composite<'a> {}
impl<'a> ControlTrait<'a> for Composite<'a> {}
// impl<'a> ScrollableTrait<'a> for Composite<'a> {}
