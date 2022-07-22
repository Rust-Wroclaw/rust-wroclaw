use std::mem::MaybeUninit;

pub struct RawTx([u32; 32]);

pub fn initialize() -> RawTx {
    RawTx(unsafe { MaybeUninit::uninit().assume_init() })
}
