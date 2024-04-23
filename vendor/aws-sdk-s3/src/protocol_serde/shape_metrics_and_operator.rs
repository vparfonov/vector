// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_metrics_and_operator(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::MetricsAndOperator, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::MetricsAndOperator::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Prefix") /* Prefix com.amazonaws.s3#MetricsAndOperator$Prefix */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_prefix(var_1);
            }
            ,
            s if s.matches("Tag") /* Tags com.amazonaws.s3#MetricsAndOperator$Tags */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::vec::Vec::<crate::types::Tag>, ::aws_smithy_xml::decode::XmlDecodeError>::Ok({
                            let mut list_3 = builder.tags.take().unwrap_or_default();
                            list_3.push(
                                crate::protocol_serde::shape_tag::de_tag(&mut tag)
                                ?
                            );
                            list_3
                        })
                        ?
                    )
                ;
                builder = builder.set_tags(var_2);
            }
            ,
            s if s.matches("AccessPointArn") /* AccessPointArn com.amazonaws.s3#MetricsAndOperator$AccessPointArn */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_access_point_arn(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}

pub fn ser_metrics_and_operator(
    input: &crate::types::MetricsAndOperator,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_5) = &input.prefix {
        let mut inner_writer = scope.start_el("Prefix").finish();
        inner_writer.data(var_5.as_str());
    }
    if let Some(var_6) = &input.tags {
        for list_item_7 in var_6 {
            {
                let inner_writer = scope.start_el("Tag");
                crate::protocol_serde::shape_tag::ser_tag(list_item_7, inner_writer)?
            }
        }
    }
    if let Some(var_8) = &input.access_point_arn {
        let mut inner_writer = scope.start_el("AccessPointArn").finish();
        inner_writer.data(var_8.as_str());
    }
    scope.finish();
    Ok(())
}
