// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_variant(
                         object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
                         input: &crate::types::Variant,
                    ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
     {
        object.key("id").string(input.id.as_str());
    }
     {
        object.key("variant_type").string(input.variant_type.as_str());
    }
    if let Some(var_1) = &input.context_id {
        object.key("context_id").string(var_1.as_str());
    }
    if let Some(var_2) = &input.override_id {
        object.key("override_id").string(var_2.as_str());
    }
     {
        object.key("overrides").document(&input.overrides);
    }
    Ok(())
}

pub(crate) fn de_variant<'a, I>(tokens: &mut ::std::iter::Peekable<I>) -> ::std::result::Result<Option<crate::types::Variant>, ::aws_smithy_json::deserialize::error::DeserializeError>
                        where I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::VariantBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "id" => {
                                builder = builder.set_id(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "variant_type" => {
                                builder = builder.set_variant_type(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::types::VariantType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "context_id" => {
                                builder = builder.set_context_id(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "override_id" => {
                                builder = builder.set_override_id(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "overrides" => {
                                builder = builder.set_overrides(
                                    Some(::aws_smithy_json::deserialize::token::expect_document(tokens)?)
                                );
                            }
                            _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?
                        }
                    }
                    other => return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!("expected object key or end object, found: {:?}", other)))
                }
            }
            Ok(Some(crate::serde_util::variant_correct_errors(builder).build().map_err(|err|::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?))
        }
        _ => {
            Err(::aws_smithy_json::deserialize::error::DeserializeError::custom("expected start object or null"))
        }
    }
}

