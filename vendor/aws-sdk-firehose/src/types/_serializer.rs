// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The serializer that you want Firehose to use to convert data to the target format before writing it to Amazon S3. Firehose supports two types of serializers: the <a href="https://hive.apache.org/javadocs/r1.2.2/api/org/apache/hadoop/hive/ql/io/orc/OrcSerde.html">ORC SerDe</a> and the <a href="https://hive.apache.org/javadocs/r1.2.2/api/org/apache/hadoop/hive/ql/io/parquet/serde/ParquetHiveSerDe.html">Parquet SerDe</a>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Serializer {
    /// <p>A serializer to use for converting data to the Parquet format before storing it in Amazon S3. For more information, see <a href="https://parquet.apache.org/documentation/latest/">Apache Parquet</a>.</p>
    pub parquet_ser_de: ::std::option::Option<crate::types::ParquetSerDe>,
    /// <p>A serializer to use for converting data to the ORC format before storing it in Amazon S3. For more information, see <a href="https://orc.apache.org/docs/">Apache ORC</a>.</p>
    pub orc_ser_de: ::std::option::Option<crate::types::OrcSerDe>,
}
impl Serializer {
    /// <p>A serializer to use for converting data to the Parquet format before storing it in Amazon S3. For more information, see <a href="https://parquet.apache.org/documentation/latest/">Apache Parquet</a>.</p>
    pub fn parquet_ser_de(&self) -> ::std::option::Option<&crate::types::ParquetSerDe> {
        self.parquet_ser_de.as_ref()
    }
    /// <p>A serializer to use for converting data to the ORC format before storing it in Amazon S3. For more information, see <a href="https://orc.apache.org/docs/">Apache ORC</a>.</p>
    pub fn orc_ser_de(&self) -> ::std::option::Option<&crate::types::OrcSerDe> {
        self.orc_ser_de.as_ref()
    }
}
impl Serializer {
    /// Creates a new builder-style object to manufacture [`Serializer`](crate::types::Serializer).
    pub fn builder() -> crate::types::builders::SerializerBuilder {
        crate::types::builders::SerializerBuilder::default()
    }
}

/// A builder for [`Serializer`](crate::types::Serializer).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct SerializerBuilder {
    pub(crate) parquet_ser_de: ::std::option::Option<crate::types::ParquetSerDe>,
    pub(crate) orc_ser_de: ::std::option::Option<crate::types::OrcSerDe>,
}
impl SerializerBuilder {
    /// <p>A serializer to use for converting data to the Parquet format before storing it in Amazon S3. For more information, see <a href="https://parquet.apache.org/documentation/latest/">Apache Parquet</a>.</p>
    pub fn parquet_ser_de(mut self, input: crate::types::ParquetSerDe) -> Self {
        self.parquet_ser_de = ::std::option::Option::Some(input);
        self
    }
    /// <p>A serializer to use for converting data to the Parquet format before storing it in Amazon S3. For more information, see <a href="https://parquet.apache.org/documentation/latest/">Apache Parquet</a>.</p>
    pub fn set_parquet_ser_de(mut self, input: ::std::option::Option<crate::types::ParquetSerDe>) -> Self {
        self.parquet_ser_de = input;
        self
    }
    /// <p>A serializer to use for converting data to the Parquet format before storing it in Amazon S3. For more information, see <a href="https://parquet.apache.org/documentation/latest/">Apache Parquet</a>.</p>
    pub fn get_parquet_ser_de(&self) -> &::std::option::Option<crate::types::ParquetSerDe> {
        &self.parquet_ser_de
    }
    /// <p>A serializer to use for converting data to the ORC format before storing it in Amazon S3. For more information, see <a href="https://orc.apache.org/docs/">Apache ORC</a>.</p>
    pub fn orc_ser_de(mut self, input: crate::types::OrcSerDe) -> Self {
        self.orc_ser_de = ::std::option::Option::Some(input);
        self
    }
    /// <p>A serializer to use for converting data to the ORC format before storing it in Amazon S3. For more information, see <a href="https://orc.apache.org/docs/">Apache ORC</a>.</p>
    pub fn set_orc_ser_de(mut self, input: ::std::option::Option<crate::types::OrcSerDe>) -> Self {
        self.orc_ser_de = input;
        self
    }
    /// <p>A serializer to use for converting data to the ORC format before storing it in Amazon S3. For more information, see <a href="https://orc.apache.org/docs/">Apache ORC</a>.</p>
    pub fn get_orc_ser_de(&self) -> &::std::option::Option<crate::types::OrcSerDe> {
        &self.orc_ser_de
    }
    /// Consumes the builder and constructs a [`Serializer`](crate::types::Serializer).
    pub fn build(self) -> crate::types::Serializer {
        crate::types::Serializer {
            parquet_ser_de: self.parquet_ser_de,
            orc_ser_de: self.orc_ser_de,
        }
    }
}
