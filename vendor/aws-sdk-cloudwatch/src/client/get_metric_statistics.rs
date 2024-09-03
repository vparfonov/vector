// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetMetricStatistics`](crate::operation::get_metric_statistics::builders::GetMetricStatisticsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`namespace(impl Into<String>)`](crate::operation::get_metric_statistics::builders::GetMetricStatisticsFluentBuilder::namespace) / [`set_namespace(Option<String>)`](crate::operation::get_metric_statistics::builders::GetMetricStatisticsFluentBuilder::set_namespace):<br>required: **true**<br><p>The namespace of the metric, with or without spaces.</p><br>
    ///   - [`metric_name(impl Into<String>)`](crate::operation::get_metric_statistics::builders::GetMetricStatisticsFluentBuilder::metric_name) / [`set_metric_name(Option<String>)`](crate::operation::get_metric_statistics::builders::GetMetricStatisticsFluentBuilder::set_metric_name):<br>required: **true**<br><p>The name of the metric, with or without spaces.</p><br>
    ///   - [`dimensions(Dimension)`](crate::operation::get_metric_statistics::builders::GetMetricStatisticsFluentBuilder::dimensions) / [`set_dimensions(Option<Vec::<Dimension>>)`](crate::operation::get_metric_statistics::builders::GetMetricStatisticsFluentBuilder::set_dimensions):<br>required: **false**<br><p>The dimensions. If the metric contains multiple dimensions, you must include a value for each dimension. CloudWatch treats each unique combination of dimensions as a separate metric. If a specific combination of dimensions was not published, you can't retrieve statistics for it. You must specify the same dimensions that were used when the metrics were created. For an example, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/cloudwatch_concepts.html#dimension-combinations">Dimension Combinations</a> in the <i>Amazon CloudWatch User Guide</i>. For more information about specifying dimensions, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/publishingMetrics.html">Publishing Metrics</a> in the <i>Amazon CloudWatch User Guide</i>.</p><br>
    ///   - [`start_time(DateTime)`](crate::operation::get_metric_statistics::builders::GetMetricStatisticsFluentBuilder::start_time) / [`set_start_time(Option<DateTime>)`](crate::operation::get_metric_statistics::builders::GetMetricStatisticsFluentBuilder::set_start_time):<br>required: **true**<br><p>The time stamp that determines the first data point to return. Start times are evaluated relative to the time that CloudWatch receives the request.</p> <p>The value specified is inclusive; results include data points with the specified time stamp. In a raw HTTP query, the time stamp must be in ISO 8601 UTC format (for example, 2016-10-03T23:00:00Z).</p> <p>CloudWatch rounds the specified time stamp as follows:</p> <ul>  <li>   <p>Start time less than 15 days ago - Round down to the nearest whole minute. For example, 12:32:34 is rounded down to 12:32:00.</p></li>  <li>   <p>Start time between 15 and 63 days ago - Round down to the nearest 5-minute clock interval. For example, 12:32:34 is rounded down to 12:30:00.</p></li>  <li>   <p>Start time greater than 63 days ago - Round down to the nearest 1-hour clock interval. For example, 12:32:34 is rounded down to 12:00:00.</p></li> </ul> <p>If you set <code>Period</code> to 5, 10, or 30, the start time of your request is rounded down to the nearest time that corresponds to even 5-, 10-, or 30-second divisions of a minute. For example, if you make a query at (HH:mm:ss) 01:05:23 for the previous 10-second period, the start time of your request is rounded down and you receive data from 01:05:10 to 01:05:20. If you make a query at 15:07:17 for the previous 5 minutes of data, using a period of 5 seconds, you receive data timestamped between 15:02:15 and 15:07:15.</p><br>
    ///   - [`end_time(DateTime)`](crate::operation::get_metric_statistics::builders::GetMetricStatisticsFluentBuilder::end_time) / [`set_end_time(Option<DateTime>)`](crate::operation::get_metric_statistics::builders::GetMetricStatisticsFluentBuilder::set_end_time):<br>required: **true**<br><p>The time stamp that determines the last data point to return.</p> <p>The value specified is exclusive; results include data points up to the specified time stamp. In a raw HTTP query, the time stamp must be in ISO 8601 UTC format (for example, 2016-10-10T23:00:00Z).</p><br>
    ///   - [`period(i32)`](crate::operation::get_metric_statistics::builders::GetMetricStatisticsFluentBuilder::period) / [`set_period(Option<i32>)`](crate::operation::get_metric_statistics::builders::GetMetricStatisticsFluentBuilder::set_period):<br>required: **true**<br><p>The granularity, in seconds, of the returned data points. For metrics with regular resolution, a period can be as short as one minute (60 seconds) and must be a multiple of 60. For high-resolution metrics that are collected at intervals of less than one minute, the period can be 1, 5, 10, 30, 60, or any multiple of 60. High-resolution metrics are those metrics stored by a <code>PutMetricData</code> call that includes a <code>StorageResolution</code> of 1 second.</p> <p>If the <code>StartTime</code> parameter specifies a time stamp that is greater than 3 hours ago, you must specify the period as follows or no data points in that time range is returned:</p> <ul>  <li>   <p>Start time between 3 hours and 15 days ago - Use a multiple of 60 seconds (1 minute).</p></li>  <li>   <p>Start time between 15 and 63 days ago - Use a multiple of 300 seconds (5 minutes).</p></li>  <li>   <p>Start time greater than 63 days ago - Use a multiple of 3600 seconds (1 hour).</p></li> </ul><br>
    ///   - [`statistics(Statistic)`](crate::operation::get_metric_statistics::builders::GetMetricStatisticsFluentBuilder::statistics) / [`set_statistics(Option<Vec::<Statistic>>)`](crate::operation::get_metric_statistics::builders::GetMetricStatisticsFluentBuilder::set_statistics):<br>required: **false**<br><p>The metric statistics, other than percentile. For percentile statistics, use <code>ExtendedStatistics</code>. When calling <code>GetMetricStatistics</code>, you must specify either <code>Statistics</code> or <code>ExtendedStatistics</code>, but not both.</p><br>
    ///   - [`extended_statistics(impl Into<String>)`](crate::operation::get_metric_statistics::builders::GetMetricStatisticsFluentBuilder::extended_statistics) / [`set_extended_statistics(Option<Vec::<String>>)`](crate::operation::get_metric_statistics::builders::GetMetricStatisticsFluentBuilder::set_extended_statistics):<br>required: **false**<br><p>The percentile statistics. Specify values between p0.0 and p100. When calling <code>GetMetricStatistics</code>, you must specify either <code>Statistics</code> or <code>ExtendedStatistics</code>, but not both. Percentile statistics are not available for metrics when any of the metric values are negative numbers.</p><br>
    ///   - [`unit(StandardUnit)`](crate::operation::get_metric_statistics::builders::GetMetricStatisticsFluentBuilder::unit) / [`set_unit(Option<StandardUnit>)`](crate::operation::get_metric_statistics::builders::GetMetricStatisticsFluentBuilder::set_unit):<br>required: **false**<br><p>The unit for a given metric. If you omit <code>Unit</code>, all data that was collected with any unit is returned, along with the corresponding units that were specified when the data was reported to CloudWatch. If you specify a unit, the operation returns only data that was collected with that unit specified. If you specify a unit that does not match the data collected, the results of the operation are null. CloudWatch does not perform unit conversions.</p><br>
    /// - On success, responds with [`GetMetricStatisticsOutput`](crate::operation::get_metric_statistics::GetMetricStatisticsOutput) with field(s):
    ///   - [`label(Option<String>)`](crate::operation::get_metric_statistics::GetMetricStatisticsOutput::label): <p>A label for the specified metric.</p>
    ///   - [`datapoints(Option<Vec::<Datapoint>>)`](crate::operation::get_metric_statistics::GetMetricStatisticsOutput::datapoints): <p>The data points for the specified metric.</p>
    /// - On failure, responds with [`SdkError<GetMetricStatisticsError>`](crate::operation::get_metric_statistics::GetMetricStatisticsError)
    pub fn get_metric_statistics(&self) -> crate::operation::get_metric_statistics::builders::GetMetricStatisticsFluentBuilder {
        crate::operation::get_metric_statistics::builders::GetMetricStatisticsFluentBuilder::new(self.handle.clone())
    }
}
