// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_restore_status(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::RestoreStatus, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::RestoreStatus::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("IsRestoreInProgress") /* IsRestoreInProgress com.amazonaws.s3#RestoreStatus$IsRestoreInProgress */ =>  {
                let var_1 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.s3#IsRestoreInProgress`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_is_restore_in_progress(var_1);
            }
            ,
            s if s.matches("RestoreExpiryDate") /* RestoreExpiryDate com.amazonaws.s3#RestoreStatus$RestoreExpiryDate */ =>  {
                let var_2 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.s3#RestoreExpiryDate`)"))
                        ?
                    )
                ;
                builder = builder.set_restore_expiry_date(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
