// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetResolvedConfigInput  {
    #[allow(missing_docs)] // documentation missing in model
    pub workspace_id: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub org_id: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub prefix: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub version: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub show_reasoning: ::std::option::Option<bool>,
    #[allow(missing_docs)] // documentation missing in model
    pub merge_strategy: ::std::option::Option<crate::types::MergeStrategy>,
    #[allow(missing_docs)] // documentation missing in model
    pub context_id: ::std::option::Option<::std::string::String>,
    /// Map representing the context. Keys correspond to the names of the dimensions.
    pub context: ::std::option::Option<::std::collections::HashMap::<::std::string::String, ::aws_smithy_types::Document>>,
}
impl  GetResolvedConfigInput  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn workspace_id(&self) -> ::std::option::Option<&str> {
        self.workspace_id.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn org_id(&self) -> ::std::option::Option<&str> {
        self.org_id.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn prefix(&self) -> ::std::option::Option<&str> {
        self.prefix.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn version(&self) -> ::std::option::Option<&str> {
        self.version.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn show_reasoning(&self) -> ::std::option::Option<bool> {
        self.show_reasoning
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn merge_strategy(&self) -> ::std::option::Option<&crate::types::MergeStrategy> {
        self.merge_strategy.as_ref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn context_id(&self) -> ::std::option::Option<&str> {
        self.context_id.as_deref()
    }
    /// Map representing the context. Keys correspond to the names of the dimensions.
    pub fn context(&self) -> ::std::option::Option<&::std::collections::HashMap::<::std::string::String, ::aws_smithy_types::Document>> {
        self.context.as_ref()
    }
}
impl GetResolvedConfigInput {
    /// Creates a new builder-style object to manufacture [`GetResolvedConfigInput`](crate::operation::get_resolved_config::GetResolvedConfigInput).
    pub fn builder() -> crate::operation::get_resolved_config::builders::GetResolvedConfigInputBuilder {
        crate::operation::get_resolved_config::builders::GetResolvedConfigInputBuilder::default()
    }
}

/// A builder for [`GetResolvedConfigInput`](crate::operation::get_resolved_config::GetResolvedConfigInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetResolvedConfigInputBuilder {
    pub(crate) workspace_id: ::std::option::Option<::std::string::String>,
    pub(crate) org_id: ::std::option::Option<::std::string::String>,
    pub(crate) prefix: ::std::option::Option<::std::string::String>,
    pub(crate) version: ::std::option::Option<::std::string::String>,
    pub(crate) show_reasoning: ::std::option::Option<bool>,
    pub(crate) merge_strategy: ::std::option::Option<crate::types::MergeStrategy>,
    pub(crate) context_id: ::std::option::Option<::std::string::String>,
    pub(crate) context: ::std::option::Option<::std::collections::HashMap::<::std::string::String, ::aws_smithy_types::Document>>,
}
impl GetResolvedConfigInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn workspace_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.workspace_id = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_workspace_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.workspace_id = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_workspace_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.workspace_id
    }
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn org_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.org_id = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_org_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.org_id = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_org_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.org_id
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.prefix = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.prefix = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_prefix(&self) -> &::std::option::Option<::std::string::String> {
        &self.prefix
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.version = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.version = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_version(&self) -> &::std::option::Option<::std::string::String> {
        &self.version
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn show_reasoning(mut self, input: bool) -> Self {
        self.show_reasoning = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_show_reasoning(mut self, input: ::std::option::Option<bool>) -> Self {
        self.show_reasoning = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_show_reasoning(&self) -> &::std::option::Option<bool> {
        &self.show_reasoning
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn merge_strategy(mut self, input: crate::types::MergeStrategy) -> Self {
        self.merge_strategy = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_merge_strategy(mut self, input: ::std::option::Option<crate::types::MergeStrategy>) -> Self {
        self.merge_strategy = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_merge_strategy(&self) -> &::std::option::Option<crate::types::MergeStrategy> {
        &self.merge_strategy
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn context_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.context_id = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_context_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.context_id = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_context_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.context_id
    }
    /// Adds a key-value pair to `context`.
    ///
    /// To override the contents of this collection use [`set_context`](Self::set_context).
    ///
    /// Map representing the context. Keys correspond to the names of the dimensions.
    pub fn context(mut self, k: impl ::std::convert::Into<::std::string::String>, v: ::aws_smithy_types::Document) -> Self {
        let mut hash_map = self.context.unwrap_or_default();
                        hash_map.insert(k.into(), v);
                        self.context = ::std::option::Option::Some(hash_map);
                        self
    }
    /// Map representing the context. Keys correspond to the names of the dimensions.
    pub fn set_context(mut self, input: ::std::option::Option<::std::collections::HashMap::<::std::string::String, ::aws_smithy_types::Document>>) -> Self {
        self.context = input; self
    }
    /// Map representing the context. Keys correspond to the names of the dimensions.
    pub fn get_context(&self) -> &::std::option::Option<::std::collections::HashMap::<::std::string::String, ::aws_smithy_types::Document>> {
        &self.context
    }
    /// Consumes the builder and constructs a [`GetResolvedConfigInput`](crate::operation::get_resolved_config::GetResolvedConfigInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::get_resolved_config::GetResolvedConfigInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(
            crate::operation::get_resolved_config::GetResolvedConfigInput {
                workspace_id: self.workspace_id
                ,
                org_id: self.org_id
                ,
                prefix: self.prefix
                ,
                version: self.version
                ,
                show_reasoning: self.show_reasoning
                ,
                merge_strategy: self.merge_strategy
                ,
                context_id: self.context_id
                ,
                context: self.context
                ,
            }
        )
    }
}

