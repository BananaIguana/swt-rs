use {
    crate::{
        conversion::ToJValue,
        java_object::{define_swt_object, JavaObject},
        jvm::JVM,
        widgets::{control::ControlTrait, display::Display},
    },
    convert_case::{Case, Casing},
    jni::{
        errors::Error,
        objects::{JObject, JValue},
    },
    swt_rs_macros::swt_fn,
};

define_swt_object!(Shell);

impl<'a> Shell<'a>
{
    pub fn new(display: &Display) -> Result<Shell<'a>, Error>
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/widgets/Shell")?;
        let jobj = jenv.new_object(
            jcls,
            "(Lorg/eclipse/swt/widgets/Display;)V",
            &[display.get_object().to_jvalue()],
        )?;

        Ok(Shell { jobj })
    }
}

pub trait ShellTrait<'a>: JavaObject<'a>
{
    swt_fn!(open, "()V");
    swt_fn!(is_disposed, "()Z");

    // org.eclipse.swt.widgets.Shell

    swt_fn!(close, "()V");
    swt_fn!(force_active, "()V");
    swt_fn!(get_alpha, "()I");
    swt_fn!(get_enabled, "()Z");
    swt_fn!(get_full_screen, "()Z");
    swt_fn!(get_ime_input_mode, "()I");
    swt_fn!(get_maximized, "()Z");
    swt_fn!(get_modified, "()Z");
    swt_fn!(is_enabled, "()Z");
    swt_fn!(is_visible, "()Z");
    swt_fn!(set_active, "()V");

    swt_fn!(set_alpha, "(I)V", alpha);
    swt_fn!(set_enabled, "(Z)V", enabled);
    swt_fn!(set_full_screen, "(Z)V", full_screen);
    swt_fn!(set_ime_input_mode, "(I)V", mode); // NOTE THIS CAN BE CONVERTED TO AN ENUM
                                               //  build_swt_fn!(void, set_maximum_size, "(Point)");   // Update with Jobject
    swt_fn!(set_maximum_size, "(II)V", width, height);
    swt_fn!(request_layout, "()V");
    swt_fn!(set_minimum_size, "(II)V", width, height);
    swt_fn!(set_modified, "(Z)V", modified);
    //  build_swt_fn!(void, set_region, "Region");          // Update with Jobject
    swt_fn!(set_visible, "(Z)V", visible);

    /*
    void addShellListener(ShellListener listener)
    Point getMaximumSize()
    Point getMinimumSize()
    Region getRegion()
    Shell getShell()
    Shell[]	getShells()
    ToolBar	getToolBar()
    static Shell	internal_new​(Display display, long handle)
    boolean	print(GC gc)
    void removeShellListener(ShellListener listener)
    static Shell win32_new(Display display, long handle)
    */
}

// java.lang.Object
//     org.eclipse.swt.widgets.Widget
//         org.eclipse.swt.widgets.Control
//             org.eclipse.swt.widgets.Scrollable
//                 org.eclipse.swt.widgets.Composite
//                     org.eclipse.swt.widgets.Canvas
//                         org.eclipse.swt.widgets.Decorations
//                             org.eclipse.swt.widgets.Shell
impl<'a> ShellTrait<'a> for Shell<'a> {}
impl<'a> ControlTrait<'a> for Shell<'a> {}
