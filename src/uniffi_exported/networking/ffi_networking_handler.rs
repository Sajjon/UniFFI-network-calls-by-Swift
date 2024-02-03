use crate::prelude::*;

#[uniffi::export(with_foreign)]
pub trait FFINetworkingHandler: FFIOperationHandler<FFINetworkingResultListener> {
    /// Rust will tell the handler to execute `operation` by calling this
    /// function, which a concrete type FFI side (Swift side) has implemented.
    /// Once the operation has finished with a result (Success/Failure) it
    /// passes back the result using the `listener_rust_side` callback.
    fn execute_network_request(
        &self,
        request: NetworkRequest,
        listener_rust_side: Arc<FFINetworkingResultListener>,
    ) -> Result<(), SwiftSideError>;
}

impl<U: FFINetworkingHandler> FFIOperationHandler<FFINetworkingResultListener> for U {
    fn supported_operations(&self) -> Vec<FFIOperationKind> {
        vec![FFIOperationKind::Networking]
    }

    fn execute_operation(
        &self,
        operation: FFIOperation,
        listener_rust_side: FFINetworkingResultListener,
    ) -> Result<(), SwiftSideError> {
        let request = operation.into_networking().expect("Network request");
        self.execute_network_request(request, listener_rust_side.into())
    }
}
