use crate::prelude::*;

#[derive(Object)]
pub struct FFINetworkingResultListener {
    result_listener: FFIOperationResultListener,
}
impl ResultListener for FFINetworkingResultListener {}
impl From<FFIOperationResultListener> for FFINetworkingResultListener {
    fn from(value: FFIOperationResultListener) -> Self {
        Self::with_result_listener(value)
    }
}
impl FFINetworkingResultListener {
    pub fn with_result_listener(result_listener: FFIOperationResultListener) -> Self {
        Self { result_listener }
    }
}

#[export]
impl FFINetworkingResultListener {
    /// This is called from FFI Side (Swift side), inside the implementation of
    /// an `execute_network_request:request:listener_rust_side` method on a [`FFINetworkingHandler`],
    /// when the operation has finished, with the [`FFINetworkResult`].
    fn notify_result(&self, result: FFINetworkResult) {
        self.result_listener.notify_result(result.into())
    }
}
