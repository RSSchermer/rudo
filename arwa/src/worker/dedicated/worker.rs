use crate::fetch::RequestCredentials;
use crate::message::{
    message_event_target_seal, message_sender_seal, MessageEventTarget, MessageSender,
};
use crate::security::SecurityError;
use crate::url::AbsoluteOrRelativeUrl;
use crate::worker::{worker_seal, CreateWorkerError, Worker, WorkerOptions, WorkerType};
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};

#[derive(Clone)]
pub struct DedicatedWorker {
    inner: web_sys::Worker,
}

impl DedicatedWorker {
    pub fn create<T>(url: T, options: WorkerOptions) -> Self
    where
        T: AbsoluteOrRelativeUrl,
    {
        create_dedicated_worker_internal(url, options).unwrap_throw()
    }

    pub fn try_create<T>(url: T, options: WorkerOptions) -> Result<Self, CreateWorkerError>
    where
        T: AbsoluteOrRelativeUrl,
    {
        create_dedicated_worker_internal(url, options)
            .map_err(|err| CreateWorkerError::new(err.unchecked_into()))
    }

    pub fn terminate(&self) {
        self.inner.terminate()
    }
}

impl worker_seal::Seal for DedicatedWorker {
    fn as_web_sys_event_target(&self) -> &web_sys::EventTarget {
        self.inner.as_ref()
    }
}

impl Worker for DedicatedWorker {}

impl message_sender_seal::Seal for DedicatedWorker {}

impl MessageSender for DedicatedWorker {}

impl message_event_target_seal::Seal for DedicatedWorker {
    fn as_web_sys_event_target(&self) -> &web_sys::EventTarget {
        self.inner.as_ref()
    }
}

impl MessageEventTarget for DedicatedWorker {}

impl From<web_sys::Worker> for DedicatedWorker {
    fn from(inner: web_sys::Worker) -> Self {
        DedicatedWorker { inner }
    }
}

impl AsRef<web_sys::Worker> for DedicatedWorker {
    fn as_ref(&self) -> &web_sys::Worker {
        &self.inner
    }
}

impl_event_target_traits!(DedicatedWorker);
impl_try_from_event_target_traits!(DedicatedWorker, web_sys::DedicatedWorker);

fn create_dedicated_worker_internal<T>(
    url: T,
    options: WorkerOptions,
) -> Result<DedicatedWorker, JsValue>
where
    T: AbsoluteOrRelativeUrl,
{
    let result = web_sys::Worker::new_with_worker_options(
        url.as_str(),
        &options.into_web_sys_worker_options(),
    );

    result.map(|w| w.into())
}
