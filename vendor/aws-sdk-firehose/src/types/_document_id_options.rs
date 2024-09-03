// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Indicates the method for setting up document ID. The supported methods are Firehose generated document ID and OpenSearch Service generated document ID.</p>
/// <p></p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DocumentIdOptions {
    /// <p>When the <code>FIREHOSE_DEFAULT</code> option is chosen, Firehose generates a unique document ID for each record based on a unique internal identifier. The generated document ID is stable across multiple delivery attempts, which helps prevent the same record from being indexed multiple times with different document IDs.</p>
    /// <p>When the <code>NO_DOCUMENT_ID</code> option is chosen, Firehose does not include any document IDs in the requests it sends to the Amazon OpenSearch Service. This causes the Amazon OpenSearch Service domain to generate document IDs. In case of multiple delivery attempts, this may cause the same record to be indexed more than once with different document IDs. This option enables write-heavy operations, such as the ingestion of logs and observability data, to consume less resources in the Amazon OpenSearch Service domain, resulting in improved performance.</p>
    pub default_document_id_format: crate::types::DefaultDocumentIdFormat,
}
impl DocumentIdOptions {
    /// <p>When the <code>FIREHOSE_DEFAULT</code> option is chosen, Firehose generates a unique document ID for each record based on a unique internal identifier. The generated document ID is stable across multiple delivery attempts, which helps prevent the same record from being indexed multiple times with different document IDs.</p>
    /// <p>When the <code>NO_DOCUMENT_ID</code> option is chosen, Firehose does not include any document IDs in the requests it sends to the Amazon OpenSearch Service. This causes the Amazon OpenSearch Service domain to generate document IDs. In case of multiple delivery attempts, this may cause the same record to be indexed more than once with different document IDs. This option enables write-heavy operations, such as the ingestion of logs and observability data, to consume less resources in the Amazon OpenSearch Service domain, resulting in improved performance.</p>
    pub fn default_document_id_format(&self) -> &crate::types::DefaultDocumentIdFormat {
        &self.default_document_id_format
    }
}
impl DocumentIdOptions {
    /// Creates a new builder-style object to manufacture [`DocumentIdOptions`](crate::types::DocumentIdOptions).
    pub fn builder() -> crate::types::builders::DocumentIdOptionsBuilder {
        crate::types::builders::DocumentIdOptionsBuilder::default()
    }
}

/// A builder for [`DocumentIdOptions`](crate::types::DocumentIdOptions).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DocumentIdOptionsBuilder {
    pub(crate) default_document_id_format: ::std::option::Option<crate::types::DefaultDocumentIdFormat>,
}
impl DocumentIdOptionsBuilder {
    /// <p>When the <code>FIREHOSE_DEFAULT</code> option is chosen, Firehose generates a unique document ID for each record based on a unique internal identifier. The generated document ID is stable across multiple delivery attempts, which helps prevent the same record from being indexed multiple times with different document IDs.</p>
    /// <p>When the <code>NO_DOCUMENT_ID</code> option is chosen, Firehose does not include any document IDs in the requests it sends to the Amazon OpenSearch Service. This causes the Amazon OpenSearch Service domain to generate document IDs. In case of multiple delivery attempts, this may cause the same record to be indexed more than once with different document IDs. This option enables write-heavy operations, such as the ingestion of logs and observability data, to consume less resources in the Amazon OpenSearch Service domain, resulting in improved performance.</p>
    /// This field is required.
    pub fn default_document_id_format(mut self, input: crate::types::DefaultDocumentIdFormat) -> Self {
        self.default_document_id_format = ::std::option::Option::Some(input);
        self
    }
    /// <p>When the <code>FIREHOSE_DEFAULT</code> option is chosen, Firehose generates a unique document ID for each record based on a unique internal identifier. The generated document ID is stable across multiple delivery attempts, which helps prevent the same record from being indexed multiple times with different document IDs.</p>
    /// <p>When the <code>NO_DOCUMENT_ID</code> option is chosen, Firehose does not include any document IDs in the requests it sends to the Amazon OpenSearch Service. This causes the Amazon OpenSearch Service domain to generate document IDs. In case of multiple delivery attempts, this may cause the same record to be indexed more than once with different document IDs. This option enables write-heavy operations, such as the ingestion of logs and observability data, to consume less resources in the Amazon OpenSearch Service domain, resulting in improved performance.</p>
    pub fn set_default_document_id_format(mut self, input: ::std::option::Option<crate::types::DefaultDocumentIdFormat>) -> Self {
        self.default_document_id_format = input;
        self
    }
    /// <p>When the <code>FIREHOSE_DEFAULT</code> option is chosen, Firehose generates a unique document ID for each record based on a unique internal identifier. The generated document ID is stable across multiple delivery attempts, which helps prevent the same record from being indexed multiple times with different document IDs.</p>
    /// <p>When the <code>NO_DOCUMENT_ID</code> option is chosen, Firehose does not include any document IDs in the requests it sends to the Amazon OpenSearch Service. This causes the Amazon OpenSearch Service domain to generate document IDs. In case of multiple delivery attempts, this may cause the same record to be indexed more than once with different document IDs. This option enables write-heavy operations, such as the ingestion of logs and observability data, to consume less resources in the Amazon OpenSearch Service domain, resulting in improved performance.</p>
    pub fn get_default_document_id_format(&self) -> &::std::option::Option<crate::types::DefaultDocumentIdFormat> {
        &self.default_document_id_format
    }
    /// Consumes the builder and constructs a [`DocumentIdOptions`](crate::types::DocumentIdOptions).
    /// This method will fail if any of the following fields are not set:
    /// - [`default_document_id_format`](crate::types::builders::DocumentIdOptionsBuilder::default_document_id_format)
    pub fn build(self) -> ::std::result::Result<crate::types::DocumentIdOptions, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::DocumentIdOptions {
            default_document_id_format: self.default_document_id_format.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "default_document_id_format",
                    "default_document_id_format was not specified but it is required when building DocumentIdOptions",
                )
            })?,
        })
    }
}
