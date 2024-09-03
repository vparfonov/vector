// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_single_metric_anomaly_detector(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::SingleMetricAnomalyDetector,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("AccountId");
    if let Some(var_2) = &input.account_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Namespace");
    if let Some(var_4) = &input.namespace {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("MetricName");
    if let Some(var_6) = &input.metric_name {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("Dimensions");
    if let Some(var_8) = &input.dimensions {
        let mut list_10 = scope_7.start_list(false, None);
        for item_9 in var_8 {
            #[allow(unused_mut)]
            let mut entry_11 = list_10.entry();
            crate::protocol_serde::shape_dimension::ser_dimension(entry_11, item_9)?;
        }
        list_10.finish();
    }
    #[allow(unused_mut)]
    let mut scope_12 = writer.prefix("Stat");
    if let Some(var_13) = &input.stat {
        scope_12.string(var_13);
    }
    Ok(())
}

#[allow(clippy::needless_question_mark)]
pub fn de_single_metric_anomaly_detector(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::SingleMetricAnomalyDetector, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::SingleMetricAnomalyDetector::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("AccountId") /* AccountId com.amazonaws.cloudwatch#SingleMetricAnomalyDetector$AccountId */ =>  {
                let var_14 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_account_id(var_14);
            }
            ,
            s if s.matches("Namespace") /* Namespace com.amazonaws.cloudwatch#SingleMetricAnomalyDetector$Namespace */ =>  {
                let var_15 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_namespace(var_15);
            }
            ,
            s if s.matches("MetricName") /* MetricName com.amazonaws.cloudwatch#SingleMetricAnomalyDetector$MetricName */ =>  {
                let var_16 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_metric_name(var_16);
            }
            ,
            s if s.matches("Dimensions") /* Dimensions com.amazonaws.cloudwatch#SingleMetricAnomalyDetector$Dimensions */ =>  {
                let var_17 =
                    Some(
                        crate::protocol_serde::shape_dimensions::de_dimensions(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_dimensions(var_17);
            }
            ,
            s if s.matches("Stat") /* Stat com.amazonaws.cloudwatch#SingleMetricAnomalyDetector$Stat */ =>  {
                let var_18 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_stat(var_18);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
