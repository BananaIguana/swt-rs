use {
    crate::java_type::JavaType,
    quote::{format_ident, quote},
};

/// A breakdown of a [Java JNI signature](https://docs.oracle.com/javase/7/docs/technotes/guides/jni/spec/types.html).
/// Contains each parameter and the return type for a full Java JNI signature string.
/// e.g.
/// * `()V` has no parameters and a returns void.
/// * `(II)Z` takes two integers and returns a boolean value.
/// * `(J)Ljava/lang/String;` takes a single long parameter and returns a String object.
pub struct JavaSignature
{
    params: Vec<JavaType>,
    result: JavaType,
}

impl JavaSignature
{
    /// Iterate over a string parsing the contents up to a semicolon.
    fn parse_object<T>(iter: &mut T) -> String
    where
        T: Iterator<Item = char>,
    {
        let mut object_string = String::new();
        let mut ok = true;

        while ok
        {
            if let Some(char) = iter.next()
            {
                if char == ';'
                {
                    ok = false;
                }
                else
                {
                    object_string.push(char);
                }
            }
            else
            {
                ok = false;
            }
        }

        object_string
    }

    fn parse_item<T>(iter: &mut T) -> Option<JavaType>
    where
        T: Iterator<Item = char>,
    {
        if let Some(item) = iter.next()
        {
            match item
            {
                // Basic types
                'Z' => return Some(JavaType::Bool),
                'B' => return Some(JavaType::Byte),
                'C' => return Some(JavaType::Char),
                'S' => return Some(JavaType::Short),
                'I' => return Some(JavaType::Int),
                'J' => return Some(JavaType::Long),
                'F' => return Some(JavaType::Float),
                'D' => return Some(JavaType::Double),
                'V' => return Some(JavaType::Void),
                '[' =>
                {
                    let array_type = Self::parse_item(iter).expect("Failed to parse array type.");

                    return Some(JavaType::Array(Box::new(array_type)));
                }
                'L' =>
                {
                    let object_string = Self::parse_object(iter);

                    let obj_type_str = object_string
                        .split(|c| c == '/' || c == ';')
                        .last()
                        .unwrap_or_else(|| panic!("Failed to extract object type from object path."));

                    // Loose check as I don't believe there is an SWT object type of 'String' under a different package path.
                    if obj_type_str.contains("String")
                    {
                        return Some(JavaType::String);
                    }
                    else
                    {
                        return Some(JavaType::SwtObject(obj_type_str.to_string()));
                    }
                }
                _ =>
                {}
            }
        }

        None
    }

    /// | Java | PrimitiveType | Rust             |
    /// |:-----|:--------------|:-----------------|
    /// | Z    | Bool          | bool             |
    /// | B    | Byte          | i8               |
    /// | C    | Char          | u16              |
    /// | S    | Short         | i16              |
    /// | I    | Int           | i32              |
    /// | J    | Long          | i64              |
    /// | F    | Float         | f32              |
    /// | D    | Double        | f64              |
    /// | L    | Java Object   | trait JavaObject |
    /// | [    | Array         | Vec<_>           |
    pub fn parse(signature: &str) -> JavaSignature
    {
        let mut params = Vec::new();
        let mut result = Vec::new();
        let mut store = &mut result;

        let mut chars = signature.chars().peekable();

        let mut char = chars.peek();

        while char.is_some()
        {
            if let Some(item) = char
            {
                match item
                {
                    // Switch storage to 'parameter' mode when an open bracket is parsed.
                    '(' =>
                    {
                        store = &mut params;
                        chars.next();
                    }
                    // Switch storage to 'return' mode when a closed bracket is parsed.
                    ')' =>
                    {
                        store = &mut result;
                        chars.next();
                    }
                    _ =>
                    {
                        if let Some(object) = Self::parse_item(&mut chars)
                        {
                            store.push(object);
                        }
                    }
                }
            }

            char = chars.peek();
        }

        JavaSignature {
            params,
            result: result
                .first()
                .unwrap_or_else(|| panic!("Failed to extract return value."))
                .clone(),
        }
    }

    /// Function parameters
    pub fn params(&self) -> &Vec<JavaType>
    {
        &self.params
    }

    /// Function return signature type
    pub fn return_type(&self) -> &JavaType
    {
        &self.result
    }

    /// Convert the function return signature type to a Rust type
    pub fn return_type_token_stream(&self) -> proc_macro2::TokenStream
    {
        if matches!(self.return_type(), JavaType::Void)
        {
            quote! { () }
        }
        else
        {
            let rust_type_str = self.return_type().rust_type_str();

            if rust_type_str.starts_with("Vec")
            {
                return rust_type_str.parse().unwrap();
            }

            let identifier = format_ident!("{}", rust_type_str);

            quote! { #identifier }
        }
    }
}

#[cfg(test)]
pub mod tests
{
    use crate::{java_signature::JavaSignature, java_type::JavaType};

    pub fn parse_test(signature: &str, expected_param: &[JavaType], expected_return: JavaType)
    {
        let signature = JavaSignature::parse(signature);

        // Check expected param length
        assert_eq!(signature.params.len(), expected_param.len());

        for (idx, param) in expected_param.iter().enumerate()
        {
            assert_eq!(param, &signature.params[idx]);
        }

        // Check return
        assert_eq!(signature.result, expected_return);
    }

    #[test]
    fn parse_test1()
    {
        parse_test("(I)V", &[JavaType::Int], JavaType::Void);
    }

    #[test]
    fn parse_test2()
    {
        parse_test("()I", &[], JavaType::Int);
    }

    #[test]
    fn parse_test3()
    {
        let signature = "(ZBCSIJFD)V";
        let expected_param = &[
            JavaType::Bool,
            JavaType::Byte,
            JavaType::Char,
            JavaType::Short,
            JavaType::Int,
            JavaType::Long,
            JavaType::Float,
            JavaType::Double,
        ];
        let return_param = JavaType::Void;

        parse_test(signature, expected_param, return_param);
    }

    #[test]
    fn parse_test4()
    {
        let signature = "(Lorg/eclipse/swt/graphics/Point;)Lorg/eclipse/swt/graphics/Rectangle;";
        let expected_param = &[JavaType::SwtObject("Point".to_string())];
        let return_param = JavaType::SwtObject("Rectangle".to_string());

        parse_test(signature, expected_param, return_param);
    }

    fn parse_item_test<T>(sig: &mut T, expected_type: JavaType)
    where
        T: Iterator<Item = char>,
    {
        assert_eq!(JavaSignature::parse_item(sig).unwrap(), expected_type);
    }

    #[test]
    fn parse_item_test1()
    {
        parse_item_test(&mut "Z".chars(), JavaType::Bool);
        parse_item_test(&mut "B".chars(), JavaType::Byte);
        parse_item_test(&mut "C".chars(), JavaType::Char);
        parse_item_test(&mut "S".chars(), JavaType::Short);
        parse_item_test(&mut "I".chars(), JavaType::Int);
        parse_item_test(&mut "J".chars(), JavaType::Long);
        parse_item_test(&mut "F".chars(), JavaType::Float);
        parse_item_test(&mut "D".chars(), JavaType::Double);
    }

    #[test]
    fn parse_item_test2()
    {
        let mut chars = "ZBCSIJFD".chars();

        parse_item_test(&mut chars, JavaType::Bool);
        parse_item_test(&mut chars, JavaType::Byte);
        parse_item_test(&mut chars, JavaType::Char);
        parse_item_test(&mut chars, JavaType::Short);
        parse_item_test(&mut chars, JavaType::Int);
        parse_item_test(&mut chars, JavaType::Long);
        parse_item_test(&mut chars, JavaType::Float);
        parse_item_test(&mut chars, JavaType::Double);
        assert!(JavaSignature::parse_item(&mut chars).is_none());
    }

    #[test]
    fn parse_item_test3()
    {
        let mut chars = "java/lang/String;".chars();

        let item = JavaSignature::parse_object(&mut chars);

        assert_eq!("java/lang/String", item);
        assert!(JavaSignature::parse_item(&mut chars).is_none());
    }

    #[test]
    fn parse_item_test4()
    {
        let mut chars = "Ljava/lang/String;".chars();

        parse_item_test(&mut chars, JavaType::String);
        assert!(JavaSignature::parse_item(&mut chars).is_none());
    }

    #[test]
    fn parse_item_test5()
    {
        let mut chars = "Lorg/eclipse/swt/graphics/Rectangle;".chars();

        parse_item_test(&mut chars, JavaType::SwtObject("Rectangle".to_string()));

        // End
        assert!(JavaSignature::parse_item(&mut chars).is_none());
    }

    #[test]
    fn parse_item_test6()
    {
        let mut chars = "[I".chars();

        parse_item_test(&mut chars, JavaType::Array(Box::new(JavaType::Int)));

        // End
        assert!(JavaSignature::parse_item(&mut chars).is_none());
    }

    #[test]
    fn parse_item_test7()
    {
        let mut chars = "[Lorg/eclipse/swt/graphics/Point;IJ".chars();

        parse_item_test(
            &mut chars,
            JavaType::Array(Box::new(JavaType::SwtObject("Point".to_string()))),
        );
        parse_item_test(&mut chars, JavaType::Int);
        parse_item_test(&mut chars, JavaType::Long);

        // End
        assert!(JavaSignature::parse_item(&mut chars).is_none());
    }
}
