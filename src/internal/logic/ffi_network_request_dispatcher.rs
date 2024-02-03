use crate::prelude::*;

#[derive(Object)]
pub struct FFINetworkRequestDispatcher {
    pub dispatcher: FFIOperationDispatcher<FFINetworkingResultListener>,
}

impl FFINetworkRequestDispatcher {
    pub fn with_dispatcher(
        dispatcher: FFIOperationDispatcher<FFINetworkingResultListener>,
    ) -> Self {
        Self { dispatcher }
    }
    pub fn new(network_antenna: Arc<dyn FFINetworkingHandler>) -> Self {
        Self::with_dispatcher(FFIOperationDispatcher::new(network_antenna))
    }
}
