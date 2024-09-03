// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies details of the Auto-Tune action. See the <a href="https://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/auto-tune.html" target="_blank">Developer Guide</a> for more information.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AutoTuneDetails {
    /// <p>Specifies details of the scheduled Auto-Tune action. See the <a href="https://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/auto-tune.html" target="_blank">Developer Guide</a> for more information.</p>
    pub scheduled_auto_tune_details: ::std::option::Option<crate::types::ScheduledAutoTuneDetails>,
}
impl AutoTuneDetails {
    /// <p>Specifies details of the scheduled Auto-Tune action. See the <a href="https://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/auto-tune.html" target="_blank">Developer Guide</a> for more information.</p>
    pub fn scheduled_auto_tune_details(&self) -> ::std::option::Option<&crate::types::ScheduledAutoTuneDetails> {
        self.scheduled_auto_tune_details.as_ref()
    }
}
impl AutoTuneDetails {
    /// Creates a new builder-style object to manufacture [`AutoTuneDetails`](crate::types::AutoTuneDetails).
    pub fn builder() -> crate::types::builders::AutoTuneDetailsBuilder {
        crate::types::builders::AutoTuneDetailsBuilder::default()
    }
}

/// A builder for [`AutoTuneDetails`](crate::types::AutoTuneDetails).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct AutoTuneDetailsBuilder {
    pub(crate) scheduled_auto_tune_details: ::std::option::Option<crate::types::ScheduledAutoTuneDetails>,
}
impl AutoTuneDetailsBuilder {
    /// <p>Specifies details of the scheduled Auto-Tune action. See the <a href="https://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/auto-tune.html" target="_blank">Developer Guide</a> for more information.</p>
    pub fn scheduled_auto_tune_details(mut self, input: crate::types::ScheduledAutoTuneDetails) -> Self {
        self.scheduled_auto_tune_details = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies details of the scheduled Auto-Tune action. See the <a href="https://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/auto-tune.html" target="_blank">Developer Guide</a> for more information.</p>
    pub fn set_scheduled_auto_tune_details(mut self, input: ::std::option::Option<crate::types::ScheduledAutoTuneDetails>) -> Self {
        self.scheduled_auto_tune_details = input;
        self
    }
    /// <p>Specifies details of the scheduled Auto-Tune action. See the <a href="https://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/auto-tune.html" target="_blank">Developer Guide</a> for more information.</p>
    pub fn get_scheduled_auto_tune_details(&self) -> &::std::option::Option<crate::types::ScheduledAutoTuneDetails> {
        &self.scheduled_auto_tune_details
    }
    /// Consumes the builder and constructs a [`AutoTuneDetails`](crate::types::AutoTuneDetails).
    pub fn build(self) -> crate::types::AutoTuneDetails {
        crate::types::AutoTuneDetails {
            scheduled_auto_tune_details: self.scheduled_auto_tune_details,
        }
    }
}
