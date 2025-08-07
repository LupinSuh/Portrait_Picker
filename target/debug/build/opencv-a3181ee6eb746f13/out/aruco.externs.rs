pub fn cv_aruco_EstimateParameters_EstimateParameters(ocvrs_return: *mut Result<*mut c_void>);
pub fn cv_aruco_EstimateParameters_implicitClone_const(instance: *const c_void) -> *mut c_void;
pub fn cv_aruco_EstimateParameters_propPattern_const(instance: *const c_void, ocvrs_return: *mut crate::aruco::PatternPositionType);
pub fn cv_aruco_EstimateParameters_propPattern_const_PatternPositionType(instance: *mut c_void, val: crate::aruco::PatternPositionType);
pub fn cv_aruco_EstimateParameters_propUseExtrinsicGuess_const(instance: *const c_void) -> bool;
pub fn cv_aruco_EstimateParameters_propUseExtrinsicGuess_const_bool(instance: *mut c_void, val: bool);
pub fn cv_aruco_EstimateParameters_propSolvePnPMethod_const(instance: *const c_void) -> i32;
pub fn cv_aruco_EstimateParameters_propSolvePnPMethod_const_int(instance: *mut c_void, val: i32);
pub fn cv_aruco_EstimateParameters_delete(instance: *mut c_void);
