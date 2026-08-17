use vector_lib::{
    NamedInternalEvent, counter,
    internal_event::{CounterName, InternalEvent},
};

#[derive(Debug, NamedInternalEvent)]
pub struct DetectExceptionsStaleEventFlushed;

impl InternalEvent for DetectExceptionsStaleEventFlushed {
    fn emit(self) {
        counter!(CounterName::DetectExceptionsStaleFlushedTotal).increment(1);
    }
}
