use {
    crate::{
        conversion::{jobject_to_array, ToJValue},
        graphics::{
            color::Color,
            cursor::Cursor,
            device::DeviceTrait,
            font::Font,
            font_data::FontData,
            image::Image,
            point::Point,
        },
        java_object::{define_swt_object, JavaObject},
        jvm::JVM,
        widgets::{control::Control, event::Event},
        Rectangle,
        Shell,
    },
    convert_case::{Case, Casing},
    jni::{
        errors::Error,
        objects::{JObject, JValue},
    },
    swt_rs_macros::swt_fn,
};

define_swt_object!(Display);

pub trait DisplayTrait<'a>: JavaObject<'a>
{
    // swt_fn!(add_filter, "(ILorg/eclipse/swt/widgets/Listener;)V", event_type, listener);
    // swt_fn!(add_listener, "(ILorg/eclipse/swt/widgets/Listener;)V", event_type, listener);
    // swt_fn!(async_exec...
    swt_fn!(beep, "()V");
    swt_fn!(close, "()V");
    // swt_fn!(dispose_exec...
    // static? find_display...
    // swt_fn!(find_widget, "(J)Lorg/eclipse/swt/widgets/Widget;", handle);
    // swt_fn!(find_widget2, "(JJ)Lorg/eclipse/swt/widgets/Widget;", handle, id);
    // swt_fn!(find_widget3, "(Lorg/eclipse/swt/widgets/Widget;J)Lorg/eclipse/swt/widgets/Widget;", widget, id);
    swt_fn!(get_active_shell, "()Lorg/eclipse/swt/widgets/Shell;");
    // static? get_app_name...
    // static? get_app_version...
    swt_fn!(get_bounds, "()Lorg/eclipse/swt/graphics/Rectangle;");
    swt_fn!(get_client_area, "()Lorg/eclipse/swt/graphics/Rectangle;");
    // static? get_current...
    swt_fn!(get_cursor_control, "()Lorg/eclipse/swt/widgets/Control;");
    swt_fn!(get_cursor_location, "()Lorg/eclipse/swt/graphics/Point;");
    swt_fn!(get_cursor_sizes, "()[Lorg/eclipse/swt/graphics/Point;");
    // swt_fn!(get_data...
    // swt_fn!(get_data(String)...
    // static? get_default...
    swt_fn!(get_dismissal_alignment, "()V");
    swt_fn!(get_double_click_time, "()I");
    // swt_fn!(get_error_handler...
    swt_fn!(get_focus_control, "()Lorg/eclipse/swt/widgets/Control;");
    swt_fn!(get_high_contrast, "()Z");
    swt_fn!(get_icon_depth, "()I");
    swt_fn!(get_icon_sizes, "()[Lorg/eclipse/swt/graphics/Point;");
    // swt_fn!(get_menu_bar, "()Lorg/eclipse/swt/widgets/Menu;");
    swt_fn!(get_monitors, "()[Lorg/eclipse/swt/widgets/Monitor;");
    // swt_fn!(get_primary_monitor, "()Lorg/eclipse/swt/widgets/Monitor;");
    // swt_fn!(get_runtime_exception_handler...
    swt_fn!(get_shells, "()[Lorg/eclipse/swt/widgets/Shell;");
    // swt_fn!(get_synchronizer, "()Lorg/eclipse/swt/widgets/Synchronizer;");
    // swt_fn!(get_sync_thread...)
    swt_fn!(get_system_color, "(I)Lorg/eclipse/swt/graphics/Color;", id);
    swt_fn!(
        get_system_cursor,
        "(I)Lorg/eclipse/swt/graphics/Cursor;",
        id
    );
    swt_fn!(get_system_font, "()Lorg/eclipse/swt/graphics/Font;");
    swt_fn!(get_system_image, "(I)Lorg/eclipse/swt/graphics/Image;", id);
    // swt_fn!(get_system_menu, "()Lorg/eclipse/swt/widgets/Menu;");
    // swt_fn!(get_system_task_bar, "()Lorg/eclipse/swt/widgets/TaskBar;");
    // swt_fn!(get_system_tray, "()Lorg/eclipse/swt/widgets/Tray;");
    // swt_fn!(get_thread...)
    swt_fn!(get_touch_enabled, "()Z");
    // swt_fn!(internal_dispose_GC, "(JLorg/eclipse/swt/graphics/GCData;)V", hDC, data);
    // swt_fn!(internal_new_GC, "(Lorg/eclipse/swt/graphics/GCData;)J", data);
    // static? is_system_dark_theme...
    swt_fn!(
        map,
        "(Lorg/eclipse/swt/widgets/Control;Lorg/eclipse/swt/widgets/Control;II)Lorg/eclipse/swt/graphics/Point;",
        from,
        to,
        x,
        y
    );
    swt_fn!(
        map1,
        "(Lorg/eclipse/swt/widgets/Control;Lorg/eclipse/swt/widgets/Control;IIII)Lorg/eclipse/swt/graphics/Rectangle;",
        from,
        to,
        x,
        y,
        width,
        height
    );
    swt_fn!(map2, "(Lorg/eclipse/swt/widgets/Control;Lorg/eclipse/swt/widgets/Control;Lorg/eclipse/swt/graphics/Point;)Lorg/eclipse/swt/graphics/Point;", from, to, point);
    swt_fn!(map3, "(Lorg/eclipse/swt/widgets/Control;Lorg/eclipse/swt/widgets/Control;Lorg/eclipse/swt/graphics/Rectangle;)Lorg/eclipse/swt/graphics/Rectangle;", from, to, rectangle);
    swt_fn!(post, "(Lorg/eclipse/swt/widgets/Event;)Z", event);
    swt_fn!(read_and_dispatch, "()Z");
    // swt_fn!(remove_filter, "(ILorg/eclipse/swt/widgets/Listener;)V", eventType, listener);
    // swt_fn!(remove_listener, "(ILorg/eclipse/swt/widgets/Listener;)V", eventType, listener);
    swt_fn!(send_post_external_event_dispatch_event, "()V");
    swt_fn!(send_pre_external_event_dispatch_event, "()V");
    // static? set_app_name...
    // static? set_app_version...
    swt_fn!(set_cursor_location, "(II)V", x, y);
    swt_fn!(
        set_cursor_location2,
        "(Lorg/eclipse/swt/graphics/Point;)V",
        point
    );
    // swt_fn!(set_data...
    // swt_fn!(set_data...
    // swt_fn!(set_error_handle...
    // swt_fn!(set_runtime_exception_handler...
    // swt_fn!(set_synchronizer, "(Lorg/eclipse/swt/widgets/Synchronizer;)V", synchronizer);
    swt_fn!(sleep, "()Z");
    // swt_fn!(sync_call..
    // swt_fn!(sync_exec..
    // swt_fn!(timer_exec..
    swt_fn!(update, "()V");
    swt_fn!(wake, "()V");
}

impl<'a> Display<'a>
{
    pub fn new() -> Result<Display<'a>, Error>
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/widgets/Display")?;
        let jobj = jenv.new_object(jcls, "()V", &[])?;

        Ok(Self { jobj })
    }
}

impl<'a> DeviceTrait<'a> for Display<'a> {}
impl<'a> DisplayTrait<'a> for Display<'a> {}
