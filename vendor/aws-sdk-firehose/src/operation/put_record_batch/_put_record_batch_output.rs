// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutRecordBatchOutput {
    /// <p>The number of records that might have failed processing. This number might be greater than 0 even if the <code>PutRecordBatch</code> call succeeds. Check <code>FailedPutCount</code> to determine whether there are records that you need to resend.</p>
    pub failed_put_count: i32,
    /// <p>Indicates whether server-side encryption (SSE) was enabled during this operation.</p>
    pub encrypted: ::std::option::Option<bool>,
    /// <p>The results array. For each record, the index of the response element is the same as the index used in the request array.</p>
    pub request_responses: ::std::vec::Vec<crate::types::PutRecordBatchResponseEntry>,
    _request_id: Option<String>,
}
impl PutRecordBatchOutput {
    /// <p>The number of records that might have failed processing. This number might be greater than 0 even if the <code>PutRecordBatch</code> call succeeds. Check <code>FailedPutCount</code> to determine whether there are records that you need to resend.</p>
    pub fn failed_put_count(&self) -> i32 {
        self.failed_put_count
    }
    /// <p>Indicates whether server-side encryption (SSE) was enabled during this operation.</p>
    pub fn encrypted(&self) -> ::std::option::Option<bool> {
        self.encrypted
    }
    /// <p>The results array. For each record, the index of the response element is the same as the index used in the request array.</p>
    pub fn request_responses(&self) -> &[crate::types::PutRecordBatchResponseEntry] {
        use std::ops::Deref;
        self.request_responses.deref()
    }
}
impl ::aws_types::request_id::RequestId for PutRecordBatchOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl PutRecordBatchOutput {
    /// Creates a new builder-style object to manufacture [`PutRecordBatchOutput`](crate::operation::put_record_batch::PutRecordBatchOutput).
    pub fn builder() -> crate::operation::put_record_batch::builders::PutRecordBatchOutputBuilder {
        crate::operation::put_record_batch::builders::PutRecordBatchOutputBuilder::default()
    }
}

/// A builder for [`PutRecordBatchOutput`](crate::operation::put_record_batch::PutRecordBatchOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct PutRecordBatchOutputBuilder {
    pub(crate) failed_put_count: ::std::option::Option<i32>,
    pub(crate) encrypted: ::std::option::Option<bool>,
    pub(crate) request_responses: ::std::option::Option<::std::vec::Vec<crate::types::PutRecordBatchResponseEntry>>,
    _request_id: Option<String>,
}
impl PutRecordBatchOutputBuilder {
    /// <p>The number of records that might have failed processing. This number might be greater than 0 even if the <code>PutRecordBatch</code> call succeeds. Check <code>FailedPutCount</code> to determine whether there are records that you need to resend.</p>
    /// This field is required.
    pub fn failed_put_count(mut self, input: i32) -> Self {
        self.failed_put_count = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of records that might have failed processing. This number might be greater than 0 even if the <code>PutRecordBatch</code> call succeeds. Check <code>FailedPutCount</code> to determine whether there are records that you need to resend.</p>
    pub fn set_failed_put_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.failed_put_count = input;
        self
    }
    /// <p>The number of records that might have failed processing. This number might be greater than 0 even if the <code>PutRecordBatch</code> call succeeds. Check <code>FailedPutCount</code> to determine whether there are records that you need to resend.</p>
    pub fn get_failed_put_count(&self) -> &::std::option::Option<i32> {
        &self.failed_put_count
    }
    /// <p>Indicates whether server-side encryption (SSE) was enabled during this operation.</p>
    pub fn encrypted(mut self, input: bool) -> Self {
        self.encrypted = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether server-side encryption (SSE) was enabled during this operation.</p>
    pub fn set_encrypted(mut self, input: ::std::option::Option<bool>) -> Self {
        self.encrypted = input;
        self
    }
    /// <p>Indicates whether server-side encryption (SSE) was enabled during this operation.</p>
    pub fn get_encrypted(&self) -> &::std::option::Option<bool> {
        &self.encrypted
    }
    /// Appends an item to `request_responses`.
    ///
    /// To override the contents of this collection use [`set_request_responses`](Self::set_request_responses).
    ///
    /// <p>The results array. For each record, the index of the response element is the same as the index used in the request array.</p>
    pub fn request_responses(mut self, input: crate::types::PutRecordBatchResponseEntry) -> Self {
        let mut v = self.request_responses.unwrap_or_default();
        v.push(input);
        self.request_responses = ::std::option::Option::Some(v);
        self
    }
    /// <p>The results array. For each record, the index of the response element is the same as the index used in the request array.</p>
    pub fn set_request_responses(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::PutRecordBatchResponseEntry>>) -> Self {
        self.request_responses = input;
        self
    }
    /// <p>The results array. For each record, the index of the response element is the same as the index used in the request array.</p>
    pub fn get_request_responses(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::PutRecordBatchResponseEntry>> {
        &self.request_responses
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`PutRecordBatchOutput`](crate::operation::put_record_batch::PutRecordBatchOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`failed_put_count`](crate::operation::put_record_batch::builders::PutRecordBatchOutputBuilder::failed_put_count)
    /// - [`request_responses`](crate::operation::put_record_batch::builders::PutRecordBatchOutputBuilder::request_responses)
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::put_record_batch::PutRecordBatchOutput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::put_record_batch::PutRecordBatchOutput {
            failed_put_count: self.failed_put_count.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "failed_put_count",
                    "failed_put_count was not specified but it is required when building PutRecordBatchOutput",
                )
            })?,
            encrypted: self.encrypted,
            request_responses: self.request_responses.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "request_responses",
                    "request_responses was not specified but it is required when building PutRecordBatchOutput",
                )
            })?,
            _request_id: self._request_id,
        })
    }
}
