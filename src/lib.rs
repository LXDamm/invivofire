use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::ptr::null_mut;
use nfc1_sys::{nfc_context, nfc_device, nfc_version, nfc_init, nfc_open, nfc_device_get_name, nfc_close, nfc_exit};

pub mod chip;
pub mod desfire;

pub struct Reader {
    context: *mut nfc_context,
    device: *mut nfc_device
}

impl Reader {
    pub fn new() -> Reader {
        Reader {
            context: null_mut(),
            device: null_mut()
        }
    }

    pub fn init(&mut self) {
        self.context = unsafe {
            let mut ctx = MaybeUninit::uninit();
            nfc_init(ctx.as_mut_ptr());
            ctx.assume_init()
        };
        self.device = unsafe {
            let pnd = nfc_open(self.context, std::ptr::null_mut());
            if pnd.as_ref().is_none() {
                nfc_exit(self.context);
                panic!("Error opening NFC reader");
            }
            pnd
        };
    }
}