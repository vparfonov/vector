// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete(
    input: &crate::types::Delete,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    {
        for list_item_1 in &input.objects {
            {
                let inner_writer = scope.start_el("Object");
                crate::protocol_serde::shape_object_identifier::ser_object_identifier(list_item_1, inner_writer)?
            }
        }
    }
    if let Some(var_2) = &input.quiet {
        let mut inner_writer = scope.start_el("Quiet").finish();
        inner_writer.data(::aws_smithy_types::primitive::Encoder::from(*var_2).encode());
    }
    scope.finish();
    Ok(())
}
