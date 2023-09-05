use {
    crate::{conversion::ToJValue, jvm::JVM},
    jni::{errors::Error, objects::JValue},
};

pub fn jvalue_to_array<'a, T>(value: JValue) -> Result<Vec<T>, Error>
where
    T: ToJValue<'a>,
{
    let jenv = JVM.get_env()?;
    let result = value.l().unwrap();
    let result = result.into_raw();
    let length = jenv.get_array_length(result).unwrap();

    let mut array = Vec::new();

    for i in 0..length
    {
        let element = jenv.get_object_array_element(result, i).unwrap();

        array.push(T::from_jobject(element));
    }

    Ok(array)
}
