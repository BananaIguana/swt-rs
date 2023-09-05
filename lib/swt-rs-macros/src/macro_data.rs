use {
    crate::{java_signature::JavaSignature, java_type::JavaType},
    proc_macro2::{self, Ident},
    syn::{
        parse::{Parse, ParseStream},
        LitStr,
        Result,
        Token,
    },
};

/// Function generation macros decouple input data into this struct.
pub struct MacroData
{
    function_name: String,              // The function name only.
    java_signature: String,             // The Java function signature (i.e. `()Ljava/lang/String;`)
    signature_info: JavaSignature,      // A breakdown of the java signature.
    variables: Vec<(String, JavaType)>, // Full input variable details.
}

impl MacroData
{
    fn generate_parameters(&self) -> String
    {
        let mut string = String::new();

        if !self.variables.is_empty()
        {
            for (var_name, var_sig) in self.variables.iter()
            {
                string += ",";
                string += var_name;
                string += ":";

                let co = |sig: &JavaType| -> String {
                    let mut string = String::new();

                    match sig
                    {
                        JavaType::Char | JavaType::Int | JavaType::Long | JavaType::Bool => string += var_sig.rust_type_str(),
                        JavaType::SwtObject(str) =>
                        {
                            string += "&";
                            string += str;
                        }
                        JavaType::String =>
                        {
                            string += "&";
                            string += "String";
                        }
                        _ => panic!("SigType '{:?}' missing in gen params", var_sig),
                    }

                    string
                };

                if let JavaType::Array(inner_type) = var_sig
                {
                    string += &co(inner_type);
                }
                else
                {
                    string += &co(var_sig);
                }
            }
        }

        string
    }

    /// Build argument variable.
    /// ```note
    /// let my_value: i32 = 1;
    /// let args = [
    ///     my_value.to_jvalue(),
    ///     my_object.get_object().to_jvalue(),
    /// ];
    /// ```
    fn build_args(&self) -> String
    {
        let mut string = "let args = [".to_string();

        self.variables.iter().for_each(|(arg, sig)| {
            match sig
            {
                JavaType::Array(_) => string += format!("{}.get_object().to_jvalue(), ", arg).as_str(), //todo!("Missing handler for {:?} type param", sig),
                JavaType::SwtObject(_) => string += format!("{}.get_object().to_jvalue(), ", arg).as_str(),
                _ => string += format!("{}.to_jvalue(), ", arg).as_str(),
            }
        });

        string += "];\n";
        string
    }

    fn build_fn(&self, custom: &str) -> String
    {
        let function_name = &self.function_name();
        let extra_params = self.generate_parameters();
        let rtt = self.signature_info.return_type_token_stream();

        let conversion = conversion_snippet(self.signature_info.return_type());

        let str = format!(
            "fn {function_name}(&self {extra_params}) -> Result<{rtt}, Error>\n\
            {{\n\
                let java_field_name = \"{function_name}\".to_case(Case::Camel);\n\
                let jenv = JVM.get_env()?;\n\
                {custom}
                match result\n\
                {{\n\
                    {conversion}\n\
                }}\n\
            }}",
            function_name = function_name,
            extra_params = extra_params,
            rtt = rtt,
            custom = custom,
            conversion = conversion
        );

        str
    }

    pub fn as_member_fn(&self) -> String
    {
        let str = format!(
            "
                {}\n\
                let result = jenv.call_method(self.get_object(), java_field_name, \"{}\", &args)?;\n\
            ",
            self.build_args(),
            self.java_signature
        );

        self.build_fn(str.as_str())
    }

    pub fn as_field_fn(&self) -> String
    {
        let str = format!(
            "let result = jenv.get_field(self.get_object(), java_field_name, \"{}\")?;\n",
            self.java_signature
        );

        self.build_fn(str.as_str())
    }

    pub fn function_name(&self) -> &str
    {
        self.function_name.as_str()
    }
}

// Implementation of `Parse` trait as defined in the `syn` crate.
impl Parse for MacroData
{
    fn parse(stream: ParseStream) -> Result<Self>
    {
        if stream.is_empty()
        {
            panic!("Write full function signature.");
        }

        let fn_name: Ident = stream
            .parse()
            .unwrap_or_else(|_| panic!("First Param Problem"));
        let _: Token![,] = stream
            .parse()
            .unwrap_or_else(|_| panic!("Comma detect problem between function name and signature."));
        let java_sig: LitStr = stream
            .parse()
            .unwrap_or_else(|_| panic!("Second Param Problem"));

        let signature_info = JavaSignature::parse(&java_sig.value());

        let mut variables = Vec::<(String, JavaType)>::new();

        let mut i = 0;

        while stream.peek(Token![,])
        {
            let _: Token![,] = stream
                .parse()
                .unwrap_or_else(|_| panic!("Comma detect problem while parsing parameters."));
            let variable: Ident = stream
                .parse()
                .unwrap_or_else(|_| panic!("Variable Parse Problem"));

            let param = signature_info
                .params()
                .get(i)
                .unwrap_or_else(|| panic!("Unbalanced var/sig"));

            variables.push((variable.to_string(), param.clone()));
            i += 1;
        }

        // Fired? Did you add the params to the definition?
        assert_eq!(variables.len(), signature_info.params().len());

        Ok(MacroData {
            function_name: fn_name.to_string(),
            java_signature: java_sig.value(),
            variables,
            signature_info,
        })
    }
}

#[cfg(test)]
mod tests
{
    use {
        crate::{java_signature, java_type::JavaType, MacroData},
        quote::quote,
    };

    // Copied from: https://stackoverflow.com/a/63904992
    macro_rules! function {
        () => {{
            fn f() {}
            fn type_name_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let name = type_name_of(f);

            // Find and cut the rest of the path
            match &name[..name.len() - 3].rfind(':') {
                Some(pos) => &name[pos + 1..name.len() - 3],
                None => &name[..name.len() - 3],
            }
        }};
    }

    /// Test helper to convert function information firstly into it's expected macro format and
    /// secondly into a `MacroData` structure. This mimics a portion of the proc macro steps.
    fn gen_macro_data(function_name: &str, signature: &str, params: &[&str]) -> MacroData
    {
        let mut quote_string = format!("{}, \"{}\"", function_name, signature);

        for param in params.iter()
        {
            quote_string += format!(", {}", param).as_str();
        }

        let tokenized_function_details: proc_macro2::TokenStream = quote_string.parse().unwrap();

        let quote = quote! {#tokenized_function_details}.into();

        syn::parse2(quote).unwrap()
    }

    /// Check the expanded `MacroData` matches expectation.
    fn test(
        macro_data: &MacroData,
        expected_function_name: &str,
        expected_signature: &str,
        expected_param: &[JavaType],
        expected_return: JavaType,
    )
    {
        assert_eq!(macro_data.function_name, expected_function_name);
        assert_eq!(macro_data.java_signature(), expected_signature);

        java_signature::tests::parse_test(
            macro_data.java_signature().as_str(),
            expected_param,
            expected_return,
        );
    }

    #[test]
    fn test_1()
    {
        let function_name = function!();
        let signature = "()I";
        let params = &[];

        let macro_data = gen_macro_data(function_name, signature, params);

        test(&macro_data, function_name, signature, &[], JavaType::Int);
    }

    #[test]
    fn test_2()
    {
        let function_name = function!();
        let signature = "()Lorg/eclipse/swt/graphics/Point;";
        let params = &[];

        let macro_data = gen_macro_data(function_name, signature, params);

        test(
            &macro_data,
            function_name,
            signature,
            &[],
            JavaType::SwtObject("Point".to_string()),
        );
    }

    #[test]
    fn test_3()
    {
        let function_name = function!();
        let signature = "(I)Lorg/eclipse/swt/graphics/Point;";
        let params = &["value"];

        let macro_data = gen_macro_data(function_name, signature, params);

        test(
            &macro_data,
            function_name,
            signature,
            &[JavaType::Int],
            JavaType::SwtObject("Point".to_string()),
        );
    }

    #[test]
    fn test_4()
    {
        let function_name = function!();
        let signature = "(IIII)Lorg/eclipse/swt/graphics/Rectangle;";
        let params = &["x, y, width, height"];

        let macro_data = gen_macro_data(function_name, signature, params);

        test(
            &macro_data,
            function_name,
            signature,
            &[JavaType::Int, JavaType::Int, JavaType::Int, JavaType::Int],
            JavaType::SwtObject("Rectangle".to_string()),
        );
    }

    #[test]
    fn test_5()
    {
        let function_name = function!();
        let signature = "([F)[D];";
        let params = &["value"];

        let macro_data = gen_macro_data(function_name, signature, params);

        test(
            &macro_data,
            function_name,
            signature,
            &[JavaType::Array(Box::new(JavaType::Float))],
            JavaType::Array(Box::new(JavaType::Double)),
        );
    }
}

fn conversion_snippet(st: &JavaType) -> String
{
    match st
    {
        JavaType::Bool => "JValue::Bool(val) => Ok(val != 0),\n\
                                            _ => Err(Error::WrongJValueType(\"JValue::Bool\", result.type_name()))"
            .to_string(),
        JavaType::Void => "JValue::Void => Ok(()),\n\
                                            _ => Err(Error::WrongJValueType(\"JValue::Void\", result.type_name()))"
            .to_string(),
        JavaType::String => "JValue::Object(val) => Ok(JVM.get_env()?.get_string(val.into())?.into()),\n\
                                            _ => Err(Error::WrongJValueType(\"JValue::Object\", result.type_name()))"
            .to_string(),
        JavaType::Int => "JValue::Int(val) => Ok(val),\n\
                                            _ => Err(Error::WrongJValueType(\"JValue::Int\", result.type_name()))"
            .to_string(),
        JavaType::Long => "JValue::Long(val) => Ok(val),\n\
                                            _ => Err(Error::WrongJValueType(\"JValue::Long\", result.type_name()))"
            .to_string(),
        JavaType::Char => "JValue::Char(val) => Ok(val),\n\
                                            _ => Err(Error::WrongJValueType(\"JValue::Char\", result.type_name()))"
            .to_string(),
        JavaType::Double => "JValue::Double(val) => Ok(val),\n\
                                            _ => Err(Error::WrongJValueType(\"JValue::Double\", result.type_name()))"
            .to_string(),
        JavaType::SwtObject(_) => "JValue::Object(val) => Ok(val.into()),
                                            _ => Err(Error::WrongJValueType(\"JValue::Object\", result.type_name()))"
            .to_string(),
        JavaType::Array(_) => "JValue::Object(val) => Ok(jobject_to_array(val.to_jvalue())),
                                            _ => Err(Error::WrongJValueType(\"JValue::Object\", result.type_name()))"
            .to_string(),
        _ => panic!("{}", format!("Conversion Type for {:?} not handled.", st)),
    }
}
