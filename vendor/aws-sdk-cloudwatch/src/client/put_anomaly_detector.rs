// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutAnomalyDetector`](crate::operation::put_anomaly_detector::builders::PutAnomalyDetectorFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`namespace(impl Into<String>)`](crate::operation::put_anomaly_detector::builders::PutAnomalyDetectorFluentBuilder::namespace) / [`set_namespace(Option<String>)`](crate::operation::put_anomaly_detector::builders::PutAnomalyDetectorFluentBuilder::set_namespace):<br>required: **false**<br><p>The namespace of the metric to create the anomaly detection model for.</p><br>
    ///   - [`metric_name(impl Into<String>)`](crate::operation::put_anomaly_detector::builders::PutAnomalyDetectorFluentBuilder::metric_name) / [`set_metric_name(Option<String>)`](crate::operation::put_anomaly_detector::builders::PutAnomalyDetectorFluentBuilder::set_metric_name):<br>required: **false**<br><p>The name of the metric to create the anomaly detection model for.</p><br>
    ///   - [`dimensions(Dimension)`](crate::operation::put_anomaly_detector::builders::PutAnomalyDetectorFluentBuilder::dimensions) / [`set_dimensions(Option<Vec::<Dimension>>)`](crate::operation::put_anomaly_detector::builders::PutAnomalyDetectorFluentBuilder::set_dimensions):<br>required: **false**<br><p>The metric dimensions to create the anomaly detection model for.</p><br>
    ///   - [`stat(impl Into<String>)`](crate::operation::put_anomaly_detector::builders::PutAnomalyDetectorFluentBuilder::stat) / [`set_stat(Option<String>)`](crate::operation::put_anomaly_detector::builders::PutAnomalyDetectorFluentBuilder::set_stat):<br>required: **false**<br><p>The statistic to use for the metric and the anomaly detection model.</p><br>
    ///   - [`configuration(AnomalyDetectorConfiguration)`](crate::operation::put_anomaly_detector::builders::PutAnomalyDetectorFluentBuilder::configuration) / [`set_configuration(Option<AnomalyDetectorConfiguration>)`](crate::operation::put_anomaly_detector::builders::PutAnomalyDetectorFluentBuilder::set_configuration):<br>required: **false**<br><p>The configuration specifies details about how the anomaly detection model is to be trained, including time ranges to exclude when training and updating the model. You can specify as many as 10 time ranges.</p> <p>The configuration can also include the time zone to use for the metric.</p><br>
    ///   - [`metric_characteristics(MetricCharacteristics)`](crate::operation::put_anomaly_detector::builders::PutAnomalyDetectorFluentBuilder::metric_characteristics) / [`set_metric_characteristics(Option<MetricCharacteristics>)`](crate::operation::put_anomaly_detector::builders::PutAnomalyDetectorFluentBuilder::set_metric_characteristics):<br>required: **false**<br><p>Use this object to include parameters to provide information about your metric to CloudWatch to help it build more accurate anomaly detection models. Currently, it includes the <code>PeriodicSpikes</code> parameter.</p><br>
    ///   - [`single_metric_anomaly_detector(SingleMetricAnomalyDetector)`](crate::operation::put_anomaly_detector::builders::PutAnomalyDetectorFluentBuilder::single_metric_anomaly_detector) / [`set_single_metric_anomaly_detector(Option<SingleMetricAnomalyDetector>)`](crate::operation::put_anomaly_detector::builders::PutAnomalyDetectorFluentBuilder::set_single_metric_anomaly_detector):<br>required: **false**<br><p>A single metric anomaly detector to be created.</p> <p>When using <code>SingleMetricAnomalyDetector</code>, you cannot include the following parameters in the same operation:</p> <ul>  <li>   <p><code>Dimensions</code></p></li>  <li>   <p><code>MetricName</code></p></li>  <li>   <p><code>Namespace</code></p></li>  <li>   <p><code>Stat</code></p></li>  <li>   <p>the <code>MetricMathAnomalyDetector</code> parameters of <code>PutAnomalyDetectorInput</code></p></li> </ul> <p>Instead, specify the single metric anomaly detector attributes as part of the property <code>SingleMetricAnomalyDetector</code>.</p><br>
    ///   - [`metric_math_anomaly_detector(MetricMathAnomalyDetector)`](crate::operation::put_anomaly_detector::builders::PutAnomalyDetectorFluentBuilder::metric_math_anomaly_detector) / [`set_metric_math_anomaly_detector(Option<MetricMathAnomalyDetector>)`](crate::operation::put_anomaly_detector::builders::PutAnomalyDetectorFluentBuilder::set_metric_math_anomaly_detector):<br>required: **false**<br><p>The metric math anomaly detector to be created.</p> <p>When using <code>MetricMathAnomalyDetector</code>, you cannot include the following parameters in the same operation:</p> <ul>  <li>   <p><code>Dimensions</code></p></li>  <li>   <p><code>MetricName</code></p></li>  <li>   <p><code>Namespace</code></p></li>  <li>   <p><code>Stat</code></p></li>  <li>   <p>the <code>SingleMetricAnomalyDetector</code> parameters of <code>PutAnomalyDetectorInput</code></p></li> </ul> <p>Instead, specify the metric math anomaly detector attributes as part of the property <code>MetricMathAnomalyDetector</code>.</p><br>
    /// - On success, responds with [`PutAnomalyDetectorOutput`](crate::operation::put_anomaly_detector::PutAnomalyDetectorOutput)
    /// - On failure, responds with [`SdkError<PutAnomalyDetectorError>`](crate::operation::put_anomaly_detector::PutAnomalyDetectorError)
    pub fn put_anomaly_detector(&self) -> crate::operation::put_anomaly_detector::builders::PutAnomalyDetectorFluentBuilder {
        crate::operation::put_anomaly_detector::builders::PutAnomalyDetectorFluentBuilder::new(self.handle.clone())
    }
}
