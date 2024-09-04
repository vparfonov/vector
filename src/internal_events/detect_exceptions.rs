use metrics::counter;
use vector_lib::internal_event::InternalEvent;

#[derive(Debug)]
pub struct DetectExceptionsStaleEventFlushed;

impl InternalEvent for DetectExceptionsStaleEventFlushed {
    fn emit(self) {
        counter!("detect_exceptions_stale_flushed_total").increment(1);
    }
}
