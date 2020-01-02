use crate::SubtleCrypto;

#[derive(Clone)]
pub struct Crypto {
    inner: web_sys::Crypto,
}

impl Crypto {
    pub fn fill_random<I>(&self, input: &mut I)
    where
        I: FillRandom,
    {
        input.fill_random(self);
    }

    pub fn subtle(&self) -> SubtleCrypto {
        self.inner.subtle().into()
    }
}

impl From<web_sys::Crypto> for Crypto {
    fn from(inner: web_sys::Crypto) -> Self {
        Crypto { inner }
    }
}

impl AsRef<web_sys::Crypto> for Crypto {
    fn as_ref(&self) -> &web_sys::Crypto {
        &self.inner
    }
}

pub trait FillRandom: fill_random_seal::Sealed {
    fn fill_random(&mut self, crypto: &Crypto);
}

mod fill_random_seal {
    pub trait Sealed {}

    impl Sealed for [i8] {}
    impl Sealed for [i16] {}
    impl Sealed for [i32] {}
    impl Sealed for [u8] {}
    impl Sealed for [u16] {}
    impl Sealed for [u32] {}
}

// TODO: Panic or error on quota exceeded? Panic for now.

impl FillRandom for [i8] {
    fn fill_random(&mut self, crypto: &Crypto) {
        unsafe {
            let view = js_sys::Int8Array::view_mut_raw(self.as_mut_ptr(), self.len());

            crypto
                .inner
                .get_random_values_with_array_buffer_view(&view.into())
                .unwrap();
        }
    }
}

impl FillRandom for [i16] {
    fn fill_random(&mut self, crypto: &Crypto) {
        unsafe {
            let view = js_sys::Int16Array::view_mut_raw(self.as_mut_ptr(), self.len());

            crypto
                .inner
                .get_random_values_with_array_buffer_view(&view.into())
                .unwrap();
        }
    }
}

impl FillRandom for [i32] {
    fn fill_random(&mut self, crypto: &Crypto) {
        unsafe {
            let view = js_sys::Int32Array::view_mut_raw(self.as_mut_ptr(), self.len());

            crypto
                .inner
                .get_random_values_with_array_buffer_view(&view.into())
                .unwrap();
        }
    }
}

impl FillRandom for [u8] {
    fn fill_random(&mut self, crypto: &Crypto) {
        unsafe {
            let view = js_sys::Uint8Array::view_mut_raw(self.as_mut_ptr(), self.len());

            crypto
                .inner
                .get_random_values_with_array_buffer_view(&view.into())
                .unwrap();
        }
    }
}

impl FillRandom for [u16] {
    fn fill_random(&mut self, crypto: &Crypto) {
        unsafe {
            let view = js_sys::Uint16Array::view_mut_raw(self.as_mut_ptr(), self.len());

            crypto
                .inner
                .get_random_values_with_array_buffer_view(&view.into())
                .unwrap();
        }
    }
}

impl FillRandom for [u32] {
    fn fill_random(&mut self, crypto: &Crypto) {
        unsafe {
            let view = js_sys::Uint32Array::view_mut_raw(self.as_mut_ptr(), self.len());

            crypto
                .inner
                .get_random_values_with_array_buffer_view(&view.into())
                .unwrap();
        }
    }
}
