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
    swt_rs_macros::{swt_field, swt_fn},
};

define_swt_object!(Rectangle);

impl<'a> Rectangle<'a>
{
    // Constructor

    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Result<Self, Error>
    {
        let jenv = JVM.get_env()?;
        let jcls = jenv.find_class("org/eclipse/swt/graphics/Rectangle")?;
        let jobj = jenv.new_object(
            jcls,
            "(IIII)V",
            &[
                jni::objects::JValue::Int(x),
                jni::objects::JValue::Int(y),
                jni::objects::JValue::Int(width),
                jni::objects::JValue::Int(height),
            ],
        )?;

        Ok(Self { jobj })
    }

    // Fields

    swt_field!(x, "I");
    // swt_field!(y, "I");
    // swt_field!(width, "I");
    // swt_field!(height, "I");

    // Methods

    swt_fn!(add, "(Lorg/eclipse/swt/graphics/Rectangle;)V", rect);
    swt_fn!(contains, "(II)Z", x, y);
    swt_fn!(hash_code, "()I");
    swt_fn!(is_empty, "()Z");
    swt_fn!(to_string, "()Ljava/lang/String;");

    swt_fn!(
        union,
        "(Lorg/eclipse/swt/graphics/Rectangle;)Lorg/eclipse/swt/graphics/Rectangle;",
        rectangle
    );

    // pub fn union(&self, rect: &Rectangle) -> Result<Rectangle, Error>
    // {
    //     let jenv = JVM.get_env()?;
    //     let args = [rect.jobj.to_jvalue()];
    //     let result = jenv.call_method(self.jobj, "union", "(Lorg/eclipse/swt/graphics/Rectangle;)Lorg/eclipse/swt/graphics/Rectangle;", &args)?;
    //
    //     match result
    //     {
    //         JValue::Object(val) => Ok(val.into()),
    //         _ => Err(Error::WrongJValueType("JValue::Object", result.type_name())),
    //     }
    // }

    // boolean	contains(Point pt)
    // boolean	equals(Object object)
    // void	intersect(Rectangle rect)
    // Rectangle	intersection(Rectangle rect)
    // boolean	intersects(int x, int y, int width, int height)
    // boolean	intersects(Rectangle rect)
}
