#![feature(async_closure)]

use futures_signals::cancelable_future;
use futures_signals::signal::{Broadcaster, Mutable, ReadOnlyMutable, Signal, SignalExt};
use js_sys::Function;
use std::pin::Pin;
use wasm_bindgen::prelude::{wasm_bindgen, Closure, JsValue};
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
pub struct Writable {
    value: Mutable<JsValue>,
}

impl From<Mutable<JsValue>> for Writable {
    fn from(v: Mutable<JsValue>) -> Writable {
        Writable { value: v }
    }
}

#[wasm_bindgen]
impl Writable {
    pub fn set(&mut self, v: JsValue) {
        let mut lock = self.value.lock_mut();
        *lock = v;
    }

    pub fn get(&self) -> JsValue {
        self.value.get_cloned()
    }

    pub fn subscribe(&self, callback: Function) -> JsValue {
        self.value.signal_cloned().subscribe(callback)
    }

    pub fn read_only(&self) -> Readable {
        Readable {
            value: self.value.read_only(),
        }
    }
}

#[wasm_bindgen]
pub struct Readable {
    value: ReadOnlyMutable<JsValue>,
}

impl From<ReadOnlyMutable<JsValue>> for Readable {
    fn from(v: ReadOnlyMutable<JsValue>) -> Readable {
        Readable { value: v }
    }
}

impl From<Mutable<JsValue>> for Readable {
    fn from(v: Mutable<JsValue>) -> Readable {
        v.read_only().into()
    }
}

#[wasm_bindgen]
impl Readable {
    pub fn get(&self) -> JsValue {
        self.value.get_cloned()
    }

    pub fn subscribe(&self, callback: Function) -> JsValue {
        self.value.signal_cloned().subscribe(callback)
    }
}

#[wasm_bindgen]
pub struct Derived {
    value: Broadcaster<Pin<Box<dyn Signal<Item = JsValue>>>>,
}

impl<S: Signal<Item = JsValue> + 'static> From<S> for Derived {
    fn from(s: S) -> Derived {
        Derived {
            value: Broadcaster::new(Box::pin(s)),
        }
    }
}

#[wasm_bindgen]
impl Derived {
    pub fn subscribe(&self, callback: Function) -> JsValue {
        self.value.signal_cloned().subscribe(callback)
    }
}

trait SignalAsSvelteStore: SignalExt<Item = JsValue> + Sized {
    fn subscribe(self, callback: Function) -> JsValue
    where
        Self: 'static,
    {
        let this = JsValue::null();
        let fut = self.for_each(move |value| {
            callback.call1(&this, &value).unwrap();
            async {}
        });

        let (handle, cancelable_fut) = cancelable_future(fut, || {});
        spawn_local(cancelable_fut);

        let mut handle = Some(handle);
        let unsubscribe: Closure<dyn FnMut()> = Closure::new(move || {
            if let Some(handle) = handle.take() {
                std::mem::drop(handle);
            }
        });
        unsubscribe.into_js_value()
    }
}

impl<T> SignalAsSvelteStore for T where T: SignalExt<Item = JsValue> {}
