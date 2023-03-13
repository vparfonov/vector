use metrics::counter;
use vector_core::internal_event::InternalEvent;

#[derive(Debug)]
pub struct DetectExceptionsStaleEventFlushed;

impl InternalEvent for DetectExceptionsStaleEventFlushed {
    fn emit(self) {
        counter!("detect_exceptions_stale_flushed_total", 1);
    }
}
