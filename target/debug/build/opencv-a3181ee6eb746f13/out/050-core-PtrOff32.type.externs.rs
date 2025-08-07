pub fn cv_PtrLfloatG_getInnerPtr_const(instance: *const c_void) -> *const f32;
pub fn cv_PtrLfloatG_getInnerPtrMut(instance: *mut c_void) -> *mut f32;
pub fn cv_PtrLfloatG_delete(instance: *mut c_void);
pub fn cv_PtrLfloatG_new_const_float(val: f32) -> *mut c_void;
