// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutDashboardInput {
    /// <p>The name of the dashboard. If a dashboard with this name already exists, this call modifies that dashboard, replacing its current contents. Otherwise, a new dashboard is created. The maximum length is 255, and valid characters are A-Z, a-z, 0-9, "-", and "_". This parameter is required.</p>
    pub dashboard_name: ::std::option::Option<::std::string::String>,
    /// <p>The detailed information about the dashboard in JSON format, including the widgets to include and their location on the dashboard. This parameter is required.</p>
    /// <p>For more information about the syntax, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/CloudWatch-Dashboard-Body-Structure.html">Dashboard Body Structure and Syntax</a>.</p>
    pub dashboard_body: ::std::option::Option<::std::string::String>,
}
impl PutDashboardInput {
    /// <p>The name of the dashboard. If a dashboard with this name already exists, this call modifies that dashboard, replacing its current contents. Otherwise, a new dashboard is created. The maximum length is 255, and valid characters are A-Z, a-z, 0-9, "-", and "_". This parameter is required.</p>
    pub fn dashboard_name(&self) -> ::std::option::Option<&str> {
        self.dashboard_name.as_deref()
    }
    /// <p>The detailed information about the dashboard in JSON format, including the widgets to include and their location on the dashboard. This parameter is required.</p>
    /// <p>For more information about the syntax, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/CloudWatch-Dashboard-Body-Structure.html">Dashboard Body Structure and Syntax</a>.</p>
    pub fn dashboard_body(&self) -> ::std::option::Option<&str> {
        self.dashboard_body.as_deref()
    }
}
impl PutDashboardInput {
    /// Creates a new builder-style object to manufacture [`PutDashboardInput`](crate::operation::put_dashboard::PutDashboardInput).
    pub fn builder() -> crate::operation::put_dashboard::builders::PutDashboardInputBuilder {
        crate::operation::put_dashboard::builders::PutDashboardInputBuilder::default()
    }
}

/// A builder for [`PutDashboardInput`](crate::operation::put_dashboard::PutDashboardInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct PutDashboardInputBuilder {
    pub(crate) dashboard_name: ::std::option::Option<::std::string::String>,
    pub(crate) dashboard_body: ::std::option::Option<::std::string::String>,
}
impl PutDashboardInputBuilder {
    /// <p>The name of the dashboard. If a dashboard with this name already exists, this call modifies that dashboard, replacing its current contents. Otherwise, a new dashboard is created. The maximum length is 255, and valid characters are A-Z, a-z, 0-9, "-", and "_". This parameter is required.</p>
    /// This field is required.
    pub fn dashboard_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.dashboard_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the dashboard. If a dashboard with this name already exists, this call modifies that dashboard, replacing its current contents. Otherwise, a new dashboard is created. The maximum length is 255, and valid characters are A-Z, a-z, 0-9, "-", and "_". This parameter is required.</p>
    pub fn set_dashboard_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.dashboard_name = input;
        self
    }
    /// <p>The name of the dashboard. If a dashboard with this name already exists, this call modifies that dashboard, replacing its current contents. Otherwise, a new dashboard is created. The maximum length is 255, and valid characters are A-Z, a-z, 0-9, "-", and "_". This parameter is required.</p>
    pub fn get_dashboard_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.dashboard_name
    }
    /// <p>The detailed information about the dashboard in JSON format, including the widgets to include and their location on the dashboard. This parameter is required.</p>
    /// <p>For more information about the syntax, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/CloudWatch-Dashboard-Body-Structure.html">Dashboard Body Structure and Syntax</a>.</p>
    /// This field is required.
    pub fn dashboard_body(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.dashboard_body = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The detailed information about the dashboard in JSON format, including the widgets to include and their location on the dashboard. This parameter is required.</p>
    /// <p>For more information about the syntax, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/CloudWatch-Dashboard-Body-Structure.html">Dashboard Body Structure and Syntax</a>.</p>
    pub fn set_dashboard_body(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.dashboard_body = input;
        self
    }
    /// <p>The detailed information about the dashboard in JSON format, including the widgets to include and their location on the dashboard. This parameter is required.</p>
    /// <p>For more information about the syntax, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/CloudWatch-Dashboard-Body-Structure.html">Dashboard Body Structure and Syntax</a>.</p>
    pub fn get_dashboard_body(&self) -> &::std::option::Option<::std::string::String> {
        &self.dashboard_body
    }
    /// Consumes the builder and constructs a [`PutDashboardInput`](crate::operation::put_dashboard::PutDashboardInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::put_dashboard::PutDashboardInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::put_dashboard::PutDashboardInput {
            dashboard_name: self.dashboard_name,
            dashboard_body: self.dashboard_body,
        })
    }
}
