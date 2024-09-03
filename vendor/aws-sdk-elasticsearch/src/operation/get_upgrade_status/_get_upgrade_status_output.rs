// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Container for response returned by <code> <code>GetUpgradeStatus</code> </code> operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetUpgradeStatusOutput {
    /// <p>Represents one of 3 steps that an Upgrade or Upgrade Eligibility Check does through:</p>
    /// <ul>
    /// <li>PreUpgradeCheck</li>
    /// <li>Snapshot</li>
    /// <li>Upgrade</li>
    /// </ul>
    /// <p></p>
    pub upgrade_step: ::std::option::Option<crate::types::UpgradeStep>,
    /// <p>One of 4 statuses that a step can go through returned as part of the <code> <code>GetUpgradeStatusResponse</code> </code> object. The status can take one of the following values:</p>
    /// <ul>
    /// <li>In Progress</li>
    /// <li>Succeeded</li>
    /// <li>Succeeded with Issues</li>
    /// <li>Failed</li>
    /// </ul>
    /// <p></p>
    pub step_status: ::std::option::Option<crate::types::UpgradeStatus>,
    /// <p>A string that describes the update briefly</p>
    pub upgrade_name: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetUpgradeStatusOutput {
    /// <p>Represents one of 3 steps that an Upgrade or Upgrade Eligibility Check does through:</p>
    /// <ul>
    /// <li>PreUpgradeCheck</li>
    /// <li>Snapshot</li>
    /// <li>Upgrade</li>
    /// </ul>
    /// <p></p>
    pub fn upgrade_step(&self) -> ::std::option::Option<&crate::types::UpgradeStep> {
        self.upgrade_step.as_ref()
    }
    /// <p>One of 4 statuses that a step can go through returned as part of the <code> <code>GetUpgradeStatusResponse</code> </code> object. The status can take one of the following values:</p>
    /// <ul>
    /// <li>In Progress</li>
    /// <li>Succeeded</li>
    /// <li>Succeeded with Issues</li>
    /// <li>Failed</li>
    /// </ul>
    /// <p></p>
    pub fn step_status(&self) -> ::std::option::Option<&crate::types::UpgradeStatus> {
        self.step_status.as_ref()
    }
    /// <p>A string that describes the update briefly</p>
    pub fn upgrade_name(&self) -> ::std::option::Option<&str> {
        self.upgrade_name.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for GetUpgradeStatusOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetUpgradeStatusOutput {
    /// Creates a new builder-style object to manufacture [`GetUpgradeStatusOutput`](crate::operation::get_upgrade_status::GetUpgradeStatusOutput).
    pub fn builder() -> crate::operation::get_upgrade_status::builders::GetUpgradeStatusOutputBuilder {
        crate::operation::get_upgrade_status::builders::GetUpgradeStatusOutputBuilder::default()
    }
}

/// A builder for [`GetUpgradeStatusOutput`](crate::operation::get_upgrade_status::GetUpgradeStatusOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetUpgradeStatusOutputBuilder {
    pub(crate) upgrade_step: ::std::option::Option<crate::types::UpgradeStep>,
    pub(crate) step_status: ::std::option::Option<crate::types::UpgradeStatus>,
    pub(crate) upgrade_name: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetUpgradeStatusOutputBuilder {
    /// <p>Represents one of 3 steps that an Upgrade or Upgrade Eligibility Check does through:</p>
    /// <ul>
    /// <li>PreUpgradeCheck</li>
    /// <li>Snapshot</li>
    /// <li>Upgrade</li>
    /// </ul>
    /// <p></p>
    pub fn upgrade_step(mut self, input: crate::types::UpgradeStep) -> Self {
        self.upgrade_step = ::std::option::Option::Some(input);
        self
    }
    /// <p>Represents one of 3 steps that an Upgrade or Upgrade Eligibility Check does through:</p>
    /// <ul>
    /// <li>PreUpgradeCheck</li>
    /// <li>Snapshot</li>
    /// <li>Upgrade</li>
    /// </ul>
    /// <p></p>
    pub fn set_upgrade_step(mut self, input: ::std::option::Option<crate::types::UpgradeStep>) -> Self {
        self.upgrade_step = input;
        self
    }
    /// <p>Represents one of 3 steps that an Upgrade or Upgrade Eligibility Check does through:</p>
    /// <ul>
    /// <li>PreUpgradeCheck</li>
    /// <li>Snapshot</li>
    /// <li>Upgrade</li>
    /// </ul>
    /// <p></p>
    pub fn get_upgrade_step(&self) -> &::std::option::Option<crate::types::UpgradeStep> {
        &self.upgrade_step
    }
    /// <p>One of 4 statuses that a step can go through returned as part of the <code> <code>GetUpgradeStatusResponse</code> </code> object. The status can take one of the following values:</p>
    /// <ul>
    /// <li>In Progress</li>
    /// <li>Succeeded</li>
    /// <li>Succeeded with Issues</li>
    /// <li>Failed</li>
    /// </ul>
    /// <p></p>
    pub fn step_status(mut self, input: crate::types::UpgradeStatus) -> Self {
        self.step_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>One of 4 statuses that a step can go through returned as part of the <code> <code>GetUpgradeStatusResponse</code> </code> object. The status can take one of the following values:</p>
    /// <ul>
    /// <li>In Progress</li>
    /// <li>Succeeded</li>
    /// <li>Succeeded with Issues</li>
    /// <li>Failed</li>
    /// </ul>
    /// <p></p>
    pub fn set_step_status(mut self, input: ::std::option::Option<crate::types::UpgradeStatus>) -> Self {
        self.step_status = input;
        self
    }
    /// <p>One of 4 statuses that a step can go through returned as part of the <code> <code>GetUpgradeStatusResponse</code> </code> object. The status can take one of the following values:</p>
    /// <ul>
    /// <li>In Progress</li>
    /// <li>Succeeded</li>
    /// <li>Succeeded with Issues</li>
    /// <li>Failed</li>
    /// </ul>
    /// <p></p>
    pub fn get_step_status(&self) -> &::std::option::Option<crate::types::UpgradeStatus> {
        &self.step_status
    }
    /// <p>A string that describes the update briefly</p>
    pub fn upgrade_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.upgrade_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A string that describes the update briefly</p>
    pub fn set_upgrade_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.upgrade_name = input;
        self
    }
    /// <p>A string that describes the update briefly</p>
    pub fn get_upgrade_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.upgrade_name
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetUpgradeStatusOutput`](crate::operation::get_upgrade_status::GetUpgradeStatusOutput).
    pub fn build(self) -> crate::operation::get_upgrade_status::GetUpgradeStatusOutput {
        crate::operation::get_upgrade_status::GetUpgradeStatusOutput {
            upgrade_step: self.upgrade_step,
            step_status: self.step_status,
            upgrade_name: self.upgrade_name,
            _request_id: self._request_id,
        }
    }
}
