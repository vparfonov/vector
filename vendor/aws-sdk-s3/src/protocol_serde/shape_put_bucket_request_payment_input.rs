// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_request_payment_configuration_http_payload(
    payload: &::std::option::Option<crate::types::RequestPaymentConfiguration>,
) -> Result<::std::vec::Vec<u8>, ::aws_smithy_types::error::operation::BuildError> {
    let payload = match payload.as_ref() {
        Some(t) => t,
        None => return Ok(crate::protocol_serde::rest_xml_unset_struct_payload()),
    };
    Ok(crate::protocol_serde::shape_put_bucket_request_payment_input::ser_request_payment_configuration_payload(payload)?)
}

pub fn ser_request_payment_configuration_payload(
    input: &crate::types::RequestPaymentConfiguration,
) -> std::result::Result<std::vec::Vec<u8>, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    {
        let mut writer = ::aws_smithy_xml::encode::XmlWriter::new(&mut out);
        #[allow(unused_mut)]
        let mut root = writer
            .start_el("RequestPaymentConfiguration")
            .write_ns("http://s3.amazonaws.com/doc/2006-03-01/", None);
        crate::protocol_serde::shape_request_payment_configuration::ser_request_payment_configuration(input, root)?
    }
    Ok(out.into_bytes())
}
