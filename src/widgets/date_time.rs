use {
    crate::{
        conversion::ToJValue,
        java_object::{define_swt_object, JavaObject},
        jvm::JVM,
        widgets::{composite::CompositeTrait, control::ControlTrait},
    },
    convert_case::{Case, Casing},
    jni::{
        errors::Error,
        objects::{JObject, JValue},
    },
    swt_rs_macros::swt_fn,
};

define_swt_object!(DateTime);

pub trait DateTimeTrait<'a>: JavaObject<'a>
{
    // swt_fn!(add_selection_listener, "(Lorg/eclipse/swt/events/SelectionListener;)V", listener);
    swt_fn!(get_day, "()I");
    swt_fn!(get_hours, "()I");
    swt_fn!(get_minutes, "()I");
    swt_fn!(get_month, "()I");
    swt_fn!(get_seconds, "()I");
    swt_fn!(get_year, "()I");
    // swt_fn!(remove_selection_listener, "(Lorg/eclipse/swt/events/SelectionListener;)V", listener);
    swt_fn!(set_date, "(III)V", year, month, day);
    swt_fn!(set_day, "(I)V", day);
    swt_fn!(set_hours, "(I)V", hours);
    swt_fn!(set_minutes, "(I)V", minutes);
    swt_fn!(set_month, "(I)V", month);
    swt_fn!(set_orientation, "(I)V", orientation);
    swt_fn!(set_seconds, "(I)V", seconds);
    swt_fn!(set_time, "(III)V", hours, minutes, seconds);
    swt_fn!(set_year, "(I)V", year);
}

impl<'a> DateTime<'a>
{
    // Constructor

    pub fn new<T>(parent: &T, style: i32) -> Result<Self, Error>
    where
        T: JavaObject<'a>,
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/widgets/DateTime")?;
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

// impl<'a> WidgetTrait<'a> for DateTime<'a> {}
impl<'a> ControlTrait<'a> for DateTime<'a> {}
// impl<'a> ScrollableTrait<'a> for DateTime<'a> {}
impl<'a> CompositeTrait<'a> for DateTime<'a> {}
impl<'a> DateTimeTrait<'a> for DateTime<'a> {}
