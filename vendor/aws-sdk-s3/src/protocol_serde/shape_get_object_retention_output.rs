// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_retention_payload(
    body: &[u8],
) -> std::result::Result<::std::option::Option<crate::types::ObjectLockRetention>, crate::operation::get_object_retention::GetObjectRetentionError> {
    (!body.is_empty())
        .then(|| {
            crate::protocol_serde::shape_get_object_retention_output::de_retention(body)
                .map_err(crate::operation::get_object_retention::GetObjectRetentionError::unhandled)
        })
        .transpose()
}

pub fn de_retention(inp: &[u8]) -> Result<crate::types::ObjectLockRetention, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;
    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    let start_el = decoder.start_el();
    if !(start_el.matches("Retention")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected Retention got {:?}",
            start_el
        )));
    }
    crate::protocol_serde::shape_object_lock_retention::de_object_lock_retention(&mut decoder)
}
