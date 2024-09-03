// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A serializer to use for converting data to the ORC format before storing it in Amazon S3. For more information, see <a href="https://orc.apache.org/docs/">Apache ORC</a>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct OrcSerDe {
    /// <p>The number of bytes in each stripe. The default is 64 MiB and the minimum is 8 MiB.</p>
    pub stripe_size_bytes: ::std::option::Option<i32>,
    /// <p>The Hadoop Distributed File System (HDFS) block size. This is useful if you intend to copy the data from Amazon S3 to HDFS before querying. The default is 256 MiB and the minimum is 64 MiB. Firehose uses this value for padding calculations.</p>
    pub block_size_bytes: ::std::option::Option<i32>,
    /// <p>The number of rows between index entries. The default is 10,000 and the minimum is 1,000.</p>
    pub row_index_stride: ::std::option::Option<i32>,
    /// <p>Set this to <code>true</code> to indicate that you want stripes to be padded to the HDFS block boundaries. This is useful if you intend to copy the data from Amazon S3 to HDFS before querying. The default is <code>false</code>.</p>
    pub enable_padding: ::std::option::Option<bool>,
    /// <p>A number between 0 and 1 that defines the tolerance for block padding as a decimal fraction of stripe size. The default value is 0.05, which means 5 percent of stripe size.</p>
    /// <p>For the default values of 64 MiB ORC stripes and 256 MiB HDFS blocks, the default block padding tolerance of 5 percent reserves a maximum of 3.2 MiB for padding within the 256 MiB block. In such a case, if the available size within the block is more than 3.2 MiB, a new, smaller stripe is inserted to fit within that space. This ensures that no stripe crosses block boundaries and causes remote reads within a node-local task.</p>
    /// <p>Firehose ignores this parameter when <code>OrcSerDe$EnablePadding</code> is <code>false</code>.</p>
    pub padding_tolerance: ::std::option::Option<f64>,
    /// <p>The compression code to use over data blocks. The default is <code>SNAPPY</code>.</p>
    pub compression: ::std::option::Option<crate::types::OrcCompression>,
    /// <p>The column names for which you want Firehose to create bloom filters. The default is <code>null</code>.</p>
    pub bloom_filter_columns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The Bloom filter false positive probability (FPP). The lower the FPP, the bigger the Bloom filter. The default value is 0.05, the minimum is 0, and the maximum is 1.</p>
    pub bloom_filter_false_positive_probability: ::std::option::Option<f64>,
    /// <p>Represents the fraction of the total number of non-null rows. To turn off dictionary encoding, set this fraction to a number that is less than the number of distinct keys in a dictionary. To always use dictionary encoding, set this threshold to 1.</p>
    pub dictionary_key_threshold: ::std::option::Option<f64>,
    /// <p>The version of the file to write. The possible values are <code>V0_11</code> and <code>V0_12</code>. The default is <code>V0_12</code>.</p>
    pub format_version: ::std::option::Option<crate::types::OrcFormatVersion>,
}
impl OrcSerDe {
    /// <p>The number of bytes in each stripe. The default is 64 MiB and the minimum is 8 MiB.</p>
    pub fn stripe_size_bytes(&self) -> ::std::option::Option<i32> {
        self.stripe_size_bytes
    }
    /// <p>The Hadoop Distributed File System (HDFS) block size. This is useful if you intend to copy the data from Amazon S3 to HDFS before querying. The default is 256 MiB and the minimum is 64 MiB. Firehose uses this value for padding calculations.</p>
    pub fn block_size_bytes(&self) -> ::std::option::Option<i32> {
        self.block_size_bytes
    }
    /// <p>The number of rows between index entries. The default is 10,000 and the minimum is 1,000.</p>
    pub fn row_index_stride(&self) -> ::std::option::Option<i32> {
        self.row_index_stride
    }
    /// <p>Set this to <code>true</code> to indicate that you want stripes to be padded to the HDFS block boundaries. This is useful if you intend to copy the data from Amazon S3 to HDFS before querying. The default is <code>false</code>.</p>
    pub fn enable_padding(&self) -> ::std::option::Option<bool> {
        self.enable_padding
    }
    /// <p>A number between 0 and 1 that defines the tolerance for block padding as a decimal fraction of stripe size. The default value is 0.05, which means 5 percent of stripe size.</p>
    /// <p>For the default values of 64 MiB ORC stripes and 256 MiB HDFS blocks, the default block padding tolerance of 5 percent reserves a maximum of 3.2 MiB for padding within the 256 MiB block. In such a case, if the available size within the block is more than 3.2 MiB, a new, smaller stripe is inserted to fit within that space. This ensures that no stripe crosses block boundaries and causes remote reads within a node-local task.</p>
    /// <p>Firehose ignores this parameter when <code>OrcSerDe$EnablePadding</code> is <code>false</code>.</p>
    pub fn padding_tolerance(&self) -> ::std::option::Option<f64> {
        self.padding_tolerance
    }
    /// <p>The compression code to use over data blocks. The default is <code>SNAPPY</code>.</p>
    pub fn compression(&self) -> ::std::option::Option<&crate::types::OrcCompression> {
        self.compression.as_ref()
    }
    /// <p>The column names for which you want Firehose to create bloom filters. The default is <code>null</code>.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.bloom_filter_columns.is_none()`.
    pub fn bloom_filter_columns(&self) -> &[::std::string::String] {
        self.bloom_filter_columns.as_deref().unwrap_or_default()
    }
    /// <p>The Bloom filter false positive probability (FPP). The lower the FPP, the bigger the Bloom filter. The default value is 0.05, the minimum is 0, and the maximum is 1.</p>
    pub fn bloom_filter_false_positive_probability(&self) -> ::std::option::Option<f64> {
        self.bloom_filter_false_positive_probability
    }
    /// <p>Represents the fraction of the total number of non-null rows. To turn off dictionary encoding, set this fraction to a number that is less than the number of distinct keys in a dictionary. To always use dictionary encoding, set this threshold to 1.</p>
    pub fn dictionary_key_threshold(&self) -> ::std::option::Option<f64> {
        self.dictionary_key_threshold
    }
    /// <p>The version of the file to write. The possible values are <code>V0_11</code> and <code>V0_12</code>. The default is <code>V0_12</code>.</p>
    pub fn format_version(&self) -> ::std::option::Option<&crate::types::OrcFormatVersion> {
        self.format_version.as_ref()
    }
}
impl OrcSerDe {
    /// Creates a new builder-style object to manufacture [`OrcSerDe`](crate::types::OrcSerDe).
    pub fn builder() -> crate::types::builders::OrcSerDeBuilder {
        crate::types::builders::OrcSerDeBuilder::default()
    }
}

/// A builder for [`OrcSerDe`](crate::types::OrcSerDe).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct OrcSerDeBuilder {
    pub(crate) stripe_size_bytes: ::std::option::Option<i32>,
    pub(crate) block_size_bytes: ::std::option::Option<i32>,
    pub(crate) row_index_stride: ::std::option::Option<i32>,
    pub(crate) enable_padding: ::std::option::Option<bool>,
    pub(crate) padding_tolerance: ::std::option::Option<f64>,
    pub(crate) compression: ::std::option::Option<crate::types::OrcCompression>,
    pub(crate) bloom_filter_columns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) bloom_filter_false_positive_probability: ::std::option::Option<f64>,
    pub(crate) dictionary_key_threshold: ::std::option::Option<f64>,
    pub(crate) format_version: ::std::option::Option<crate::types::OrcFormatVersion>,
}
impl OrcSerDeBuilder {
    /// <p>The number of bytes in each stripe. The default is 64 MiB and the minimum is 8 MiB.</p>
    pub fn stripe_size_bytes(mut self, input: i32) -> Self {
        self.stripe_size_bytes = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of bytes in each stripe. The default is 64 MiB and the minimum is 8 MiB.</p>
    pub fn set_stripe_size_bytes(mut self, input: ::std::option::Option<i32>) -> Self {
        self.stripe_size_bytes = input;
        self
    }
    /// <p>The number of bytes in each stripe. The default is 64 MiB and the minimum is 8 MiB.</p>
    pub fn get_stripe_size_bytes(&self) -> &::std::option::Option<i32> {
        &self.stripe_size_bytes
    }
    /// <p>The Hadoop Distributed File System (HDFS) block size. This is useful if you intend to copy the data from Amazon S3 to HDFS before querying. The default is 256 MiB and the minimum is 64 MiB. Firehose uses this value for padding calculations.</p>
    pub fn block_size_bytes(mut self, input: i32) -> Self {
        self.block_size_bytes = ::std::option::Option::Some(input);
        self
    }
    /// <p>The Hadoop Distributed File System (HDFS) block size. This is useful if you intend to copy the data from Amazon S3 to HDFS before querying. The default is 256 MiB and the minimum is 64 MiB. Firehose uses this value for padding calculations.</p>
    pub fn set_block_size_bytes(mut self, input: ::std::option::Option<i32>) -> Self {
        self.block_size_bytes = input;
        self
    }
    /// <p>The Hadoop Distributed File System (HDFS) block size. This is useful if you intend to copy the data from Amazon S3 to HDFS before querying. The default is 256 MiB and the minimum is 64 MiB. Firehose uses this value for padding calculations.</p>
    pub fn get_block_size_bytes(&self) -> &::std::option::Option<i32> {
        &self.block_size_bytes
    }
    /// <p>The number of rows between index entries. The default is 10,000 and the minimum is 1,000.</p>
    pub fn row_index_stride(mut self, input: i32) -> Self {
        self.row_index_stride = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of rows between index entries. The default is 10,000 and the minimum is 1,000.</p>
    pub fn set_row_index_stride(mut self, input: ::std::option::Option<i32>) -> Self {
        self.row_index_stride = input;
        self
    }
    /// <p>The number of rows between index entries. The default is 10,000 and the minimum is 1,000.</p>
    pub fn get_row_index_stride(&self) -> &::std::option::Option<i32> {
        &self.row_index_stride
    }
    /// <p>Set this to <code>true</code> to indicate that you want stripes to be padded to the HDFS block boundaries. This is useful if you intend to copy the data from Amazon S3 to HDFS before querying. The default is <code>false</code>.</p>
    pub fn enable_padding(mut self, input: bool) -> Self {
        self.enable_padding = ::std::option::Option::Some(input);
        self
    }
    /// <p>Set this to <code>true</code> to indicate that you want stripes to be padded to the HDFS block boundaries. This is useful if you intend to copy the data from Amazon S3 to HDFS before querying. The default is <code>false</code>.</p>
    pub fn set_enable_padding(mut self, input: ::std::option::Option<bool>) -> Self {
        self.enable_padding = input;
        self
    }
    /// <p>Set this to <code>true</code> to indicate that you want stripes to be padded to the HDFS block boundaries. This is useful if you intend to copy the data from Amazon S3 to HDFS before querying. The default is <code>false</code>.</p>
    pub fn get_enable_padding(&self) -> &::std::option::Option<bool> {
        &self.enable_padding
    }
    /// <p>A number between 0 and 1 that defines the tolerance for block padding as a decimal fraction of stripe size. The default value is 0.05, which means 5 percent of stripe size.</p>
    /// <p>For the default values of 64 MiB ORC stripes and 256 MiB HDFS blocks, the default block padding tolerance of 5 percent reserves a maximum of 3.2 MiB for padding within the 256 MiB block. In such a case, if the available size within the block is more than 3.2 MiB, a new, smaller stripe is inserted to fit within that space. This ensures that no stripe crosses block boundaries and causes remote reads within a node-local task.</p>
    /// <p>Firehose ignores this parameter when <code>OrcSerDe$EnablePadding</code> is <code>false</code>.</p>
    pub fn padding_tolerance(mut self, input: f64) -> Self {
        self.padding_tolerance = ::std::option::Option::Some(input);
        self
    }
    /// <p>A number between 0 and 1 that defines the tolerance for block padding as a decimal fraction of stripe size. The default value is 0.05, which means 5 percent of stripe size.</p>
    /// <p>For the default values of 64 MiB ORC stripes and 256 MiB HDFS blocks, the default block padding tolerance of 5 percent reserves a maximum of 3.2 MiB for padding within the 256 MiB block. In such a case, if the available size within the block is more than 3.2 MiB, a new, smaller stripe is inserted to fit within that space. This ensures that no stripe crosses block boundaries and causes remote reads within a node-local task.</p>
    /// <p>Firehose ignores this parameter when <code>OrcSerDe$EnablePadding</code> is <code>false</code>.</p>
    pub fn set_padding_tolerance(mut self, input: ::std::option::Option<f64>) -> Self {
        self.padding_tolerance = input;
        self
    }
    /// <p>A number between 0 and 1 that defines the tolerance for block padding as a decimal fraction of stripe size. The default value is 0.05, which means 5 percent of stripe size.</p>
    /// <p>For the default values of 64 MiB ORC stripes and 256 MiB HDFS blocks, the default block padding tolerance of 5 percent reserves a maximum of 3.2 MiB for padding within the 256 MiB block. In such a case, if the available size within the block is more than 3.2 MiB, a new, smaller stripe is inserted to fit within that space. This ensures that no stripe crosses block boundaries and causes remote reads within a node-local task.</p>
    /// <p>Firehose ignores this parameter when <code>OrcSerDe$EnablePadding</code> is <code>false</code>.</p>
    pub fn get_padding_tolerance(&self) -> &::std::option::Option<f64> {
        &self.padding_tolerance
    }
    /// <p>The compression code to use over data blocks. The default is <code>SNAPPY</code>.</p>
    pub fn compression(mut self, input: crate::types::OrcCompression) -> Self {
        self.compression = ::std::option::Option::Some(input);
        self
    }
    /// <p>The compression code to use over data blocks. The default is <code>SNAPPY</code>.</p>
    pub fn set_compression(mut self, input: ::std::option::Option<crate::types::OrcCompression>) -> Self {
        self.compression = input;
        self
    }
    /// <p>The compression code to use over data blocks. The default is <code>SNAPPY</code>.</p>
    pub fn get_compression(&self) -> &::std::option::Option<crate::types::OrcCompression> {
        &self.compression
    }
    /// Appends an item to `bloom_filter_columns`.
    ///
    /// To override the contents of this collection use [`set_bloom_filter_columns`](Self::set_bloom_filter_columns).
    ///
    /// <p>The column names for which you want Firehose to create bloom filters. The default is <code>null</code>.</p>
    pub fn bloom_filter_columns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.bloom_filter_columns.unwrap_or_default();
        v.push(input.into());
        self.bloom_filter_columns = ::std::option::Option::Some(v);
        self
    }
    /// <p>The column names for which you want Firehose to create bloom filters. The default is <code>null</code>.</p>
    pub fn set_bloom_filter_columns(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.bloom_filter_columns = input;
        self
    }
    /// <p>The column names for which you want Firehose to create bloom filters. The default is <code>null</code>.</p>
    pub fn get_bloom_filter_columns(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.bloom_filter_columns
    }
    /// <p>The Bloom filter false positive probability (FPP). The lower the FPP, the bigger the Bloom filter. The default value is 0.05, the minimum is 0, and the maximum is 1.</p>
    pub fn bloom_filter_false_positive_probability(mut self, input: f64) -> Self {
        self.bloom_filter_false_positive_probability = ::std::option::Option::Some(input);
        self
    }
    /// <p>The Bloom filter false positive probability (FPP). The lower the FPP, the bigger the Bloom filter. The default value is 0.05, the minimum is 0, and the maximum is 1.</p>
    pub fn set_bloom_filter_false_positive_probability(mut self, input: ::std::option::Option<f64>) -> Self {
        self.bloom_filter_false_positive_probability = input;
        self
    }
    /// <p>The Bloom filter false positive probability (FPP). The lower the FPP, the bigger the Bloom filter. The default value is 0.05, the minimum is 0, and the maximum is 1.</p>
    pub fn get_bloom_filter_false_positive_probability(&self) -> &::std::option::Option<f64> {
        &self.bloom_filter_false_positive_probability
    }
    /// <p>Represents the fraction of the total number of non-null rows. To turn off dictionary encoding, set this fraction to a number that is less than the number of distinct keys in a dictionary. To always use dictionary encoding, set this threshold to 1.</p>
    pub fn dictionary_key_threshold(mut self, input: f64) -> Self {
        self.dictionary_key_threshold = ::std::option::Option::Some(input);
        self
    }
    /// <p>Represents the fraction of the total number of non-null rows. To turn off dictionary encoding, set this fraction to a number that is less than the number of distinct keys in a dictionary. To always use dictionary encoding, set this threshold to 1.</p>
    pub fn set_dictionary_key_threshold(mut self, input: ::std::option::Option<f64>) -> Self {
        self.dictionary_key_threshold = input;
        self
    }
    /// <p>Represents the fraction of the total number of non-null rows. To turn off dictionary encoding, set this fraction to a number that is less than the number of distinct keys in a dictionary. To always use dictionary encoding, set this threshold to 1.</p>
    pub fn get_dictionary_key_threshold(&self) -> &::std::option::Option<f64> {
        &self.dictionary_key_threshold
    }
    /// <p>The version of the file to write. The possible values are <code>V0_11</code> and <code>V0_12</code>. The default is <code>V0_12</code>.</p>
    pub fn format_version(mut self, input: crate::types::OrcFormatVersion) -> Self {
        self.format_version = ::std::option::Option::Some(input);
        self
    }
    /// <p>The version of the file to write. The possible values are <code>V0_11</code> and <code>V0_12</code>. The default is <code>V0_12</code>.</p>
    pub fn set_format_version(mut self, input: ::std::option::Option<crate::types::OrcFormatVersion>) -> Self {
        self.format_version = input;
        self
    }
    /// <p>The version of the file to write. The possible values are <code>V0_11</code> and <code>V0_12</code>. The default is <code>V0_12</code>.</p>
    pub fn get_format_version(&self) -> &::std::option::Option<crate::types::OrcFormatVersion> {
        &self.format_version
    }
    /// Consumes the builder and constructs a [`OrcSerDe`](crate::types::OrcSerDe).
    pub fn build(self) -> crate::types::OrcSerDe {
        crate::types::OrcSerDe {
            stripe_size_bytes: self.stripe_size_bytes,
            block_size_bytes: self.block_size_bytes,
            row_index_stride: self.row_index_stride,
            enable_padding: self.enable_padding,
            padding_tolerance: self.padding_tolerance,
            compression: self.compression,
            bloom_filter_columns: self.bloom_filter_columns,
            bloom_filter_false_positive_probability: self.bloom_filter_false_positive_probability,
            dictionary_key_threshold: self.dictionary_key_threshold,
            format_version: self.format_version,
        }
    }
}
