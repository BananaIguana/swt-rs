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
    swt_rs_macros::swt_fn,
};

define_swt_object!(Text);

impl<'a> Text<'a>
{
    // Constructor

    pub fn new<T>(parent: &T, style: i32) -> Result<Self, Error>
    where
        T: JavaObject<'a>,
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/widgets/Text")?;
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

    // swt_fn!(add_modify_listener, "?")
    // swt_fn!(add_segment_listener, "?")
    // swt_fn!(add_selection_listener, "?")
    // swt_fn!(add_verify_listener, "?")
    swt_fn!(append, "(Ljava/lang/String;)V", string);
    swt_fn!(clear_selection, "()V");
    swt_fn!(copy, "()V");
    swt_fn!(cut, "()V");
    swt_fn!(get_caret_line_number, "()I");
    // swt_fn!(get_caret_location, "()?");
    swt_fn!(get_caret_position, "()I");
    swt_fn!(get_char_count, "()I");
    swt_fn!(get_double_click_enabled, "()Z");
    swt_fn!(get_echo_char, "()C");
    swt_fn!(get_editable, "()Z");
    swt_fn!(get_line_count, "()I");
    swt_fn!(get_line_delimiter, "()Ljava/lang/String;");
    swt_fn!(get_line_height, "()I");
    swt_fn!(get_message, "()Ljava/lang/String;");
    swt_fn!(get_orientation, "()I");
    // swt_fn!(get_selection, "()?");
    swt_fn!(get_selection_count, "()I");
    swt_fn!(get_selection_text, "()Ljava/lang/String;");
    swt_fn!(get_tabs, "()I");
    swt_fn!(get_text, "()Ljava/lang/String;");
    // swt_fn!(get_text2, "(II)Ljava/lang/String;");
    // swt_fn!(get_text_chars, "()[C]");
    swt_fn!(get_text_limit, "()I");
    swt_fn!(get_top_index, "()I");
    swt_fn!(get_top_pixel, "()I");
    // swt_fn!(insert, "(Ljava/lang/String;)V", string);
    swt_fn!(paste, "()V");
    // swt_fn!(remove_modify_listener, "?");
    // swt_fn!(remove_segment_listener, "?");
    // swt_fn!(remove_selection_listener, "?");
    // swt_fn!(remove_verify_listener, "?");
    swt_fn!(select_all, "()V");
    swt_fn!(select_double_click_enabled, "(Z)V", double_click);
    swt_fn!(set_echo_char, "(C)V", double_click);
    swt_fn!(set_editable, "(Z)V", editable);
    // swt_fn!(set_font, "(?)V", font);
    // swt_fn!(set_message, "(Ljava/lang/String;)V", message);
    swt_fn!(set_orientation, "(I)V", orientation);
    swt_fn!(set_redraw, "(Z)V", redraw);
    swt_fn!(set_selection, "(I)V", start);
    // swt_fn!(set_selection2, "(II)V", start);
    // swt_fn!(set_selection3, "(?)V", selection);
    swt_fn!(set_tabs, "(I)V", tabs);
    swt_fn!(set_text, "(Ljava/lang/String;)V", string);
    // swt_fn!(set_text_chars, "(Ljava/lang/String;)V");
    swt_fn!(set_text_limit, "(I)V", limit);
    swt_fn!(set_top_index, "(I)V", index);
    swt_fn!(show_selection, "()V");

    // TEMPORARY
    swt_fn!(pack, "()V");
}
