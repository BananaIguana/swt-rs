use {
    crate::{
        conversion::ToJValue,
        graphics::{color::Color, cursor::Cursor, font::Font, image::Image, point::Point},
        java_object::{define_swt_object, JavaObject},
        jvm::JVM,
        widgets::{composite::Composite, event::Event},
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

define_swt_object!(Control);

pub trait ControlTrait<'a>: JavaObject<'a>
{
    // swt_fn!(add_control_listener, "(Lorg/eclipse/swt/events/ControlListener;)V", listener);
    // swt_fn!(add_drag_detect_listener, "(Lorg/eclipse/swt/events/DragDetectListener;)V", listener);
    // swt_fn!(add_focus_listener, "(Lorg/eclipse/swt/events/FocusListener;)V", listener);
    // swt_fn!(add_gesture_listener, "(Lorg/eclipse/swt/events/GestureListener;)V", listener);
    // swt_fn!(add_help_listener, "(Lorg/eclipse/swt/events/HelpListener;)V", listener);
    // swt_fn!(add_key_listener, "(Lorg/eclipse/swt/events/KeyListener;)V", listener);
    // swt_fn!(add_menu_detect_listener, "(Lorg/eclipse/swt/events/MenuDetectListener;)V", listener);
    // swt_fn!(add_mouse_listener, "(Lorg/eclipse/swt/events/MouseListener;)V", listener);
    // swt_fn!(add_mouse_move_listener, "(Lorg/eclipse/swt/events/MouseMoveListener;)V", listener);
    // swt_fn!(add_mouse_track_listener, "(Lorg/eclipse/swt/events/MouseTrackListener;)V", listener);
    // swt_fn!(add_mouse_wheel_listener, "(Lorg/eclipse/swt/events/MouseWheelListener;)V", listener);
    // swt_fn!(add_paint_listener, "(Lorg/eclipse/swt/events/PaintListener;)V", listener);
    // swt_fn!(add_touch_listener, "(Lorg/eclipse/swt/events/TouchListener;)V", listener);
    // swt_fn!(add_traverse_listener, "(Lorg/eclipse/swt/events/TraverseListener;)V", listener);
    swt_fn!(
        compute_size,
        "(II)Lorg/eclipse/swt/graphics/Point;",
        width_hint,
        height_hint
    );
    swt_fn!(
        compute_size2,
        "(IIZ)Lorg/eclipse/swt/graphics/Point;",
        width_hint,
        height_hint,
        changed
    );
    // swt_fn!(drag_detect, "(Lorg/eclipse/swt/events/MouseEvent;)Z", mouse_event);
    swt_fn!(drag_detect2, "(Lorg/eclipse/swt/events/Event;)Z", event);
    swt_fn!(force_focus, "()Z");
    // swt_fn!(get_accessible, "()Lorg/eclipse/swt/accessibility/Accessible;");
    swt_fn!(get_background, "()Lorg/eclipse/swt/graphics/Color;");
    swt_fn!(get_background_image, "()Lorg/eclipse/swt/graphics/Image;");
    swt_fn!(get_border_width, "()I");
    swt_fn!(get_bounds, "()Lorg/eclipse/swt/graphics/Rectangle;");
    swt_fn!(get_cursor, "()Lorg/eclipse/swt/graphics/Cursor;");
    swt_fn!(get_drag_detect, "()Z");
    swt_fn!(get_enabled, "()Z");
    swt_fn!(get_font, "()Lorg/eclipse/swt/graphics/Font;");
    swt_fn!(get_foreground, "()Lorg/eclipse/swt/graphics/Color;");
    // swt_fn!(get_layout_data, "()Ljava/lang/Object;");
    swt_fn!(get_location, "()Lorg/eclipse/swt/graphics/Point;");
    // swt_fn!(get_menu, "()Lorg/eclipse/swt/widgets/Menu;");
    // swt_fn!(get_monitor, "()Lorg/eclipse/swt/widgets/Monitor;");
    swt_fn!(get_orientation, "()I");
    swt_fn!(get_parent, "()Lorg/eclipse/swt/widgets/Composite;");
    // swt_fn!(get_region, "()Lorg/eclipse/swt/graphics/Region;");
    swt_fn!(get_shell, "()Lorg/eclipse/swt/widgets/Shell;");
    swt_fn!(get_size, "()Lorg/eclipse/swt/graphics/Point;");
    swt_fn!(get_text_direction, "()I");
    swt_fn!(get_tool_tip_text, "()Ljava/lang/String;");
    swt_fn!(get_touch_enabled, "()Z");
    swt_fn!(get_visible, "()Z");
    // swt_fn!(internal_dispose_GC, "(JLorg/eclipse/swt/graphics/GCData;)V", hDC, data);
    // swt_fn!(internal_new_GC, "(Lorg/eclipse/swt/graphics/GCData;)J", data);
    swt_fn!(is_enabled, "()Z");
    swt_fn!(is_focus_control, "()Z");
    swt_fn!(is_reparentable, "()Z");
    swt_fn!(is_visible, "()Z");
    swt_fn!(move_above, "(Lorg/eclipse/swt/widgets/Control;)V", control);
    swt_fn!(move_below, "(Lorg/eclipse/swt/widgets/Control;)V", control);
    swt_fn!(pack, "()V");
    swt_fn!(pack2, "(Z)V", changed);
    // swt_fn!(print, "(Lorg/eclipse/swt/graphics/GC;)Z", gc);
    swt_fn!(redraw, "()V");
    swt_fn!(redraw2, "(IIIIZ)V", x, y, width, height, all);
    // swt_fn!(remove_control_listener, "(Lorg/eclipse/swt/events/ControlListener;)V", listener);
    // swt_fn!(remove_drag_detect_listener, "(Lorg/eclipse/swt/events/DragDetectListener;)V", listener);
    // swt_fn!(remove_focus_listener, "(Lorg/eclipse/swt/events/FocusListener;)V", listener);
    // swt_fn!(remove_gesture_listener, "(Lorg/eclipse/swt/events/GestureListener;)V", listener);
    // swt_fn!(remove_help_listener, "(Lorg/eclipse/swt/events/HelpListener;)V", listener);
    // swt_fn!(remove_key_listener, "(Lorg/eclipse/swt/events/KeyListener;)V", listener);
    // swt_fn!(remove_menu_detect_listener, "(Lorg/eclipse/swt/events/MenuDetectListener;)V", listener);
    // swt_fn!(remove_mouse_listener, "(Lorg/eclipse/swt/events/MouseListener;)V", listener);
    // swt_fn!(remove_mouse_move_listener, "(Lorg/eclipse/swt/events/MouseMoveListener;)V", listener);
    // swt_fn!(remove_mouse_track_listener, "(Lorg/eclipse/swt/events/MouseTrackListener;)V", listener);
    // swt_fn!(remove_mouse_wheel_listener, "(Lorg/eclipse/swt/events/MouseWheelListener;)V", listener);
    // swt_fn!(remove_paint_listener, "(Lorg/eclipse/swt/events/PaintListener;)V", listener);
    // swt_fn!(remove_touch_listener, "(Lorg/eclipse/swt/events/TouchListener;)V", listener);
    // swt_fn!(remove_traverse_listener, "(Lorg/eclipse/swt/events/TraverseListener;)V", listener);
    swt_fn!(request_layout, "()V");
    swt_fn!(set_background, "(Lorg/eclipse/swt/graphics/Color;)V", color);
    swt_fn!(
        set_background_image,
        "(Lorg/eclipse/swt/graphics/Image;)V",
        image
    );
    swt_fn!(set_bounds, "(IIII)V", x, y, width, height);
    swt_fn!(set_bounds2, "(Lorg/eclipse/swt/graphics/Rectangle;)V", rect);
    swt_fn!(set_capture, "(Z)V", capture);
    swt_fn!(set_cursor, "(Lorg/eclipse/swt/graphics/Cursor;)V", cursor);
    swt_fn!(set_drag_detect, "(Z)V", drag_detect);
    swt_fn!(set_enabled, "(Z)V", enabled);
    swt_fn!(set_focus, "()Z");
    swt_fn!(set_font, "(Lorg/eclipse/swt/graphics/Font;)V", font);
    swt_fn!(set_foreground, "(Lorg/eclipse/swt/graphics/Color;)V", color);
    // swt_fn!(set_layout_data, "(Ljava/lang/Object;)V", layout_data);
    swt_fn!(set_location, "(II)V", x, y);
    swt_fn!(
        set_location2,
        "(Lorg/eclipse/swt/graphics/Point;)V",
        location
    );
    // swt_fn!(set_menu, "(Lorg/eclipse/swt/widgets/Menu;)V", menu);
    swt_fn!(set_orientation, "(I)V", orientation);
    swt_fn!(
        set_parent,
        "(Lorg/eclipse/swt/widgets/Composite;)Z",
        composite
    );
    swt_fn!(set_redraw, "(Z)V", redraw);
    // swt_fn!(set_region, "(Lorg/eclipse/swt/graphics/Region;)V", region);
    swt_fn!(set_size, "(II)V", x, y);
    swt_fn!(set_size2, "(Lorg/eclipse/swt/graphics/Point;)V", size);
    swt_fn!(set_text_direction, "(I)V", text_direction);
    swt_fn!(set_tool_tip_text, "(Ljava/lang/String;)V", string);
    swt_fn!(set_touch_enabled, "(Z)V", enabled);
    swt_fn!(set_visible, "(Z)V", visible);
    swt_fn!(to_control, "(II)Lorg/eclipse/swt/graphics/Point;", x, y);
    swt_fn!(
        to_control2,
        "(Lorg/eclipse/swt/graphics/Point;)Lorg/eclipse/swt/graphics/Point;",
        point
    );
    swt_fn!(to_display, "(II)Lorg/eclipse/swt/graphics/Point;", x, y);
    swt_fn!(
        to_display2,
        "(Lorg/eclipse/swt/graphics/Point;)Lorg/eclipse/swt/graphics/Point;",
        point
    );
    swt_fn!(traverse, "(I)Z", traversal);
    // swt_fn!(traverse2, "(ILorg/eclipse/swt/events/KeyEvent;)Z", traversal, key_event);
    swt_fn!(
        traverse3,
        "(ILorg/eclipse/swt/widgets/Event;)Z",
        traversal,
        event
    );
    swt_fn!(update, "()V");
}

impl<'a> Control<'a>
{
    pub fn new<T>(parent: &T, style: i32) -> Result<Self, Error>
    where
        T: JavaObject<'a>,
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/widgets/Control")?;
        let jobj = jenv.new_object(
            jcls,
            "(Lorg/eclipse/swt/widgets/Control;I)V",
            &[
                jni::objects::JValue::Object(parent.get_object()),
                jni::objects::JValue::Int(style),
            ],
        )?;

        Ok(Self { jobj })
    }
}

// impl<'a> WidgetTrait<'a> for Control<'a> {}
impl<'a> ControlTrait<'a> for Control<'a> {}
