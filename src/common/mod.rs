#[cfg(any(feature = "sources-datadog_agent", feature = "sinks-datadog_metrics"))]
pub(crate) mod datadog;

<<<<<<< HEAD
#[cfg(any(
    feature = "sources-aws_sqs",
    feature = "sinks-aws_sqs",
    feature = "sources-aws_s3"
))]
pub(crate) mod sqs;
