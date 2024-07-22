use std::mem::transmute;

pub struct Interface {
    interface_pointer: *const usize,
}

unsafe impl Sync for Interface {}
unsafe impl Send for Interface {}

impl Interface {
    fn get_method(&self, index: isize) -> *const usize {
        let vtable = unsafe { *(self.interface_pointer) as *const *const usize };
        let vfunc = unsafe { *vtable.offset(index) };
        vfunc
    }

    pub fn new(interface_pointer: *const usize) -> Self {
        Self { interface_pointer }
    }

    pub fn is_in_game(&self) -> bool {
        let vfunc = unsafe {
            transmute::<_, unsafe extern "fastcall" fn(thisptr: *const usize) -> bool>(
                self.get_method(35),
            )
        };
        unsafe { vfunc(self.interface_pointer) }
    }
}
