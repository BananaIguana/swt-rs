use {
    crate::{
        java_object::{define_swt_object, JavaObject},
        jvm::JVM,
        widgets::control::ControlTrait,
    },
    jni::{errors::Error, objects::JObject},
};

define_swt_object!(Canvas);

pub trait CanvasTrait<'a>: JavaObject<'a>
{
    // swt_fn!(draw_background, "(Lorg/eclipse/swt/graphics/GC;IIII)V", gc, x, y, width, height);
    // swt_fn!(get_caret, "()Lorg/eclipse/swt/widgets/Caret;");
    // swt_fn!(get_IME, getIME, "()Lorg/eclipse/swt/widgets/IME;");
    // swt_fn!(scroll, "(IIIIIIZ)V", destX, destY, x, y, width, height, all);
    // swt_fn!(set_caret, "(Lorg/eclipse/swt/widgets/Caret;)V", caret);
    // swt_fn!(set_font, "(Lorg/eclipse/swt/graphics/Font;)V", font);
    // swt_fn!(set_IME, setIME, "(Lorg/eclipse/swt/widgets/IME;)V", ime);
}

impl<'a> Canvas<'a>
{
    // Constructor

    pub fn new<T>(parent: &T, style: i32) -> Result<Self, Error>
    where
        T: JavaObject<'a>,
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/widgets/Canvas")?;
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

//impl<'a> WidgetTrait<'a> for Canvas<'a> {}
impl<'a> ControlTrait<'a> for Canvas<'a> {}
//impl<'a> ScrollableTrait<'a> for Canvas<'a> {}
//impl<'a> CompositeTrait<'a> for Canvas<'a> {}
