use {
    crate::{
        conversion::{jobject_to_array, ToJValue},
        graphics::{font::Font, font_data::FontData, point::Point},
        java_object::{define_swt_object, JavaObject},
        jvm::JVM,
        widgets::control::ControlTrait,
    },
    convert_case::{Case, Casing},
    jni::{
        errors::Error,
        objects::{JObject, JValue},
    },
    swt_rs_macros::{swt_field, swt_fn},
};

define_swt_object!(Combo);

pub trait ComboTrait<'a>: JavaObject<'a>
{
    swt_field!(LIMIT, "I");

    swt_fn!(add, "(Ljava/lang/String;)V", string);
    swt_fn!(add2, "(Ljava/lang/String;I)V", string, index);
    //    swt_fn!(add_modifiy_listener, "(Lorg/eclipse/swt/events/ModifyListener;)V", listener);
    //    swt_fn!(add_segment_listener, "(Lorg/eclipse/swt/events/SegmentListener;)V", listener);
    //    swt_fn!(add_selection_listener, "(Lorg/eclipse/swt/events/SelectionListener;)V", listener);
    //    swt_fn!(add_verify_listener, "(Lorg/eclipse/swt/events/VerifyListener;)V", listener);
    swt_fn!(check_subclass, "()V");
    swt_fn!(clear_selection, "()V");
    swt_fn!(copy, "()V");
    swt_fn!(cut, "()V");
    swt_fn!(deselect, "(I)V", index);
    swt_fn!(deselect_all, "()V");
    swt_fn!(get_caret_location, "()Lorg/eclipse/swt/graphics/Point;");
    swt_fn!(get_caret_position, "()I");
    swt_fn!(get_item, "(I)Ljava/lang/String;", index);
    swt_fn!(get_item_count, "()I");
    swt_fn!(get_item_height, "()I");
    swt_fn!(get_items, "()[Ljava/lang/String;");
    swt_fn!(get_list_visible, "()Z");
    swt_fn!(get_orientation, "()I");
    swt_fn!(get_selection, "()Lorg/eclipse/swt/graphics/Point;");
    swt_fn!(get_selection_index, "()I");
    swt_fn!(get_text, "()Ljava/lang/String;");
    swt_fn!(get_text_height, "()I");
    swt_fn!(get_text_limit, "()I");
    swt_fn!(get_visible_item_count, "()I");
    swt_fn!(index_of, "(Ljava/lang/String;)I", string);
    swt_fn!(index_of2, "(Ljava/lang/String;I)I", string, start);
    swt_fn!(paste, "()V");
    swt_fn!(remove, "(I)V", index);
    swt_fn!(remove2, "(II)V", start, end);
    swt_fn!(remove3, "(Ljava/lang/String;)V", string);
    swt_fn!(remove_add, "()V");
    // swt_fn!(remove_modify_listener, "(Lorg/eclipse/swt/events/ModifyListener;)V", listener);
    // swt_fn!(remove_segment_listener, "(Lorg/eclipse/swt/events/SegmentListener;)V", listener);
    // swt_fn!(remove_selection_listener, "(Lorg/eclipse/swt/events/SelectionListener;)V", listener);
    // swt_fn!(remove_verify_listener, "(Lorg/eclipse/swt/events/VerifyListener;)V", listener);
    swt_fn!(select, "(I)V", index);
    swt_fn!(set_font, "(Lorg/eclipse/swt/graphics/Font;)V", font);
    swt_fn!(set_item, "(II)V", index, string);
    // swt_fn!(set_items, "(?)V", items);
    swt_fn!(set_list_visible, "(Z)V", visible);
    swt_fn!(set_orientation, "(I)V", orientation);
    swt_fn!(
        set_selection,
        "(Lorg/eclipse/swt/graphics/Point;)V",
        selection
    );
    swt_fn!(set_text, "(Ljava/lang/String;)V", string);
    swt_fn!(set_text_limit, "(I)V", limit);
    swt_fn!(set_visible_item_count, "(I)V", count);
}

impl<'a> Combo<'a>
{
    // Constructor

    pub fn new<T>(parent: &T, style: i32) -> Result<Self, Error>
    where
        T: JavaObject<'a>,
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/widgets/Combo")?;
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

// impl<'a> WidgetTrait<'a> for Combo<'a> {}
impl<'a> ControlTrait<'a> for Combo<'a> {}
// impl<'a> ScrollableTrait<'a> for Combo<'a> {}
// impl<'a> CompositeTrait<'a> for Combo<'a> {}
