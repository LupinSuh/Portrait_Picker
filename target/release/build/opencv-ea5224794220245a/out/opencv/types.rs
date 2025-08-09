
mod core_types {
	use crate::{mod_prelude::*, core, types, sys};

	ptr_extern! { core::Algorithm,
		cv_PtrLcv_AlgorithmG_new_null_const, cv_PtrLcv_AlgorithmG_delete, cv_PtrLcv_AlgorithmG_getInnerPtr_const, cv_PtrLcv_AlgorithmG_getInnerPtrMut
	}

	ptr_extern_ctor! { core::Algorithm, cv_PtrLcv_AlgorithmG_new_const_Algorithm }
	impl core::Ptr<core::Algorithm> {
		#[inline] pub fn as_raw_PtrOfAlgorithm(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAlgorithm(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<core::Algorithm> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<core::Algorithm> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl std::fmt::Debug for core::Ptr<core::Algorithm> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfAlgorithm")
				.finish()
		}
	}

	ptr_extern! { core::ConjGradSolver,
		cv_PtrLcv_ConjGradSolverG_new_null_const, cv_PtrLcv_ConjGradSolverG_delete, cv_PtrLcv_ConjGradSolverG_getInnerPtr_const, cv_PtrLcv_ConjGradSolverG_getInnerPtrMut
	}

	impl core::Ptr<core::ConjGradSolver> {
		#[inline] pub fn as_raw_PtrOfConjGradSolver(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfConjGradSolver(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl core::ConjGradSolverTraitConst for core::Ptr<core::ConjGradSolver> {
		#[inline] fn as_raw_ConjGradSolver(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::ConjGradSolverTrait for core::Ptr<core::ConjGradSolver> {
		#[inline] fn as_raw_mut_ConjGradSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<core::ConjGradSolver> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<core::ConjGradSolver> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<core::ConjGradSolver>, core::Ptr<core::Algorithm>, cv_PtrLcv_ConjGradSolverG_to_PtrOfAlgorithm }

	impl core::MinProblemSolverTraitConst for core::Ptr<core::ConjGradSolver> {
		#[inline] fn as_raw_MinProblemSolver(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::MinProblemSolverTrait for core::Ptr<core::ConjGradSolver> {
		#[inline] fn as_raw_mut_MinProblemSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<core::ConjGradSolver>, core::Ptr<core::MinProblemSolver>, cv_PtrLcv_ConjGradSolverG_to_PtrOfMinProblemSolver }

	impl std::fmt::Debug for core::Ptr<core::ConjGradSolver> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfConjGradSolver")
				.finish()
		}
	}

	ptr_extern! { core::DownhillSolver,
		cv_PtrLcv_DownhillSolverG_new_null_const, cv_PtrLcv_DownhillSolverG_delete, cv_PtrLcv_DownhillSolverG_getInnerPtr_const, cv_PtrLcv_DownhillSolverG_getInnerPtrMut
	}

	impl core::Ptr<core::DownhillSolver> {
		#[inline] pub fn as_raw_PtrOfDownhillSolver(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDownhillSolver(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl core::DownhillSolverTraitConst for core::Ptr<core::DownhillSolver> {
		#[inline] fn as_raw_DownhillSolver(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::DownhillSolverTrait for core::Ptr<core::DownhillSolver> {
		#[inline] fn as_raw_mut_DownhillSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<core::DownhillSolver> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<core::DownhillSolver> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<core::DownhillSolver>, core::Ptr<core::Algorithm>, cv_PtrLcv_DownhillSolverG_to_PtrOfAlgorithm }

	impl core::MinProblemSolverTraitConst for core::Ptr<core::DownhillSolver> {
		#[inline] fn as_raw_MinProblemSolver(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::MinProblemSolverTrait for core::Ptr<core::DownhillSolver> {
		#[inline] fn as_raw_mut_MinProblemSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<core::DownhillSolver>, core::Ptr<core::MinProblemSolver>, cv_PtrLcv_DownhillSolverG_to_PtrOfMinProblemSolver }

	impl std::fmt::Debug for core::Ptr<core::DownhillSolver> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfDownhillSolver")
				.finish()
		}
	}

	ptr_extern! { core::FileStorage,
		cv_PtrLcv_FileStorageG_new_null_const, cv_PtrLcv_FileStorageG_delete, cv_PtrLcv_FileStorageG_getInnerPtr_const, cv_PtrLcv_FileStorageG_getInnerPtrMut
	}

	ptr_extern_ctor! { core::FileStorage, cv_PtrLcv_FileStorageG_new_const_FileStorage }
	impl core::Ptr<core::FileStorage> {
		#[inline] pub fn as_raw_PtrOfFileStorage(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFileStorage(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl core::FileStorageTraitConst for core::Ptr<core::FileStorage> {
		#[inline] fn as_raw_FileStorage(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::FileStorageTrait for core::Ptr<core::FileStorage> {
		#[inline] fn as_raw_mut_FileStorage(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl std::fmt::Debug for core::Ptr<core::FileStorage> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfFileStorage")
				.field("state", &core::FileStorageTraitConst::state(self))
				.field("elname", &core::FileStorageTraitConst::elname(self))
				.finish()
		}
	}

	ptr_extern! { core::Formatted,
		cv_PtrLcv_FormattedG_new_null_const, cv_PtrLcv_FormattedG_delete, cv_PtrLcv_FormattedG_getInnerPtr_const, cv_PtrLcv_FormattedG_getInnerPtrMut
	}

	impl core::Ptr<core::Formatted> {
		#[inline] pub fn as_raw_PtrOfFormatted(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFormatted(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl core::FormattedTraitConst for core::Ptr<core::Formatted> {
		#[inline] fn as_raw_Formatted(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::FormattedTrait for core::Ptr<core::Formatted> {
		#[inline] fn as_raw_mut_Formatted(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl std::fmt::Debug for core::Ptr<core::Formatted> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfFormatted")
				.finish()
		}
	}

	ptr_extern! { core::Formatter,
		cv_PtrLcv_FormatterG_new_null_const, cv_PtrLcv_FormatterG_delete, cv_PtrLcv_FormatterG_getInnerPtr_const, cv_PtrLcv_FormatterG_getInnerPtrMut
	}

	impl core::Ptr<core::Formatter> {
		#[inline] pub fn as_raw_PtrOfFormatter(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFormatter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl core::FormatterTraitConst for core::Ptr<core::Formatter> {
		#[inline] fn as_raw_Formatter(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::FormatterTrait for core::Ptr<core::Formatter> {
		#[inline] fn as_raw_mut_Formatter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl std::fmt::Debug for core::Ptr<core::Formatter> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfFormatter")
				.finish()
		}
	}

	ptr_extern! { core::KeyPoint,
		cv_PtrLcv_KeyPointG_new_null_const, cv_PtrLcv_KeyPointG_delete, cv_PtrLcv_KeyPointG_getInnerPtr_const, cv_PtrLcv_KeyPointG_getInnerPtrMut
	}

	ptr_extern_ctor! { core::KeyPoint, cv_PtrLcv_KeyPointG_new_const_KeyPoint }
	impl core::Ptr<core::KeyPoint> {
		#[inline] pub fn as_raw_PtrOfKeyPoint(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfKeyPoint(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl core::KeyPointTraitConst for core::Ptr<core::KeyPoint> {
		#[inline] fn as_raw_KeyPoint(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::KeyPointTrait for core::Ptr<core::KeyPoint> {
		#[inline] fn as_raw_mut_KeyPoint(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl std::fmt::Debug for core::Ptr<core::KeyPoint> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfKeyPoint")
				.finish()
		}
	}

	ptr_extern! { core::MinProblemSolver,
		cv_PtrLcv_MinProblemSolverG_new_null_const, cv_PtrLcv_MinProblemSolverG_delete, cv_PtrLcv_MinProblemSolverG_getInnerPtr_const, cv_PtrLcv_MinProblemSolverG_getInnerPtrMut
	}

	impl core::Ptr<core::MinProblemSolver> {
		#[inline] pub fn as_raw_PtrOfMinProblemSolver(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMinProblemSolver(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl core::MinProblemSolverTraitConst for core::Ptr<core::MinProblemSolver> {
		#[inline] fn as_raw_MinProblemSolver(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::MinProblemSolverTrait for core::Ptr<core::MinProblemSolver> {
		#[inline] fn as_raw_mut_MinProblemSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<core::MinProblemSolver> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<core::MinProblemSolver> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<core::MinProblemSolver>, core::Ptr<core::Algorithm>, cv_PtrLcv_MinProblemSolverG_to_PtrOfAlgorithm }

	impl std::fmt::Debug for core::Ptr<core::MinProblemSolver> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfMinProblemSolver")
				.finish()
		}
	}

	ptr_extern! { core::MinProblemSolver_Function,
		cv_PtrLcv_MinProblemSolver_FunctionG_new_null_const, cv_PtrLcv_MinProblemSolver_FunctionG_delete, cv_PtrLcv_MinProblemSolver_FunctionG_getInnerPtr_const, cv_PtrLcv_MinProblemSolver_FunctionG_getInnerPtrMut
	}

	impl core::Ptr<core::MinProblemSolver_Function> {
		#[inline] pub fn as_raw_PtrOfMinProblemSolver_Function(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMinProblemSolver_Function(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl core::MinProblemSolver_FunctionTraitConst for core::Ptr<core::MinProblemSolver_Function> {
		#[inline] fn as_raw_MinProblemSolver_Function(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::MinProblemSolver_FunctionTrait for core::Ptr<core::MinProblemSolver_Function> {
		#[inline] fn as_raw_mut_MinProblemSolver_Function(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl std::fmt::Debug for core::Ptr<core::MinProblemSolver_Function> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfMinProblemSolver_Function")
				.finish()
		}
	}

	ptr_extern! { f32,
		cv_PtrLfloatG_new_null_const, cv_PtrLfloatG_delete, cv_PtrLfloatG_getInnerPtr_const, cv_PtrLfloatG_getInnerPtrMut
	}

	ptr_extern_ctor! { f32, cv_PtrLfloatG_new_const_float }
	impl core::Ptr<f32> {
		#[inline] pub fn as_raw_PtrOff32(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOff32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl core::Tuple<(core::Rect, i32)> {
		pub fn as_raw_TupleOfRect_i32(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_TupleOfRect_i32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	tuple_extern! { (core::Rect, i32),
		std_pairLcv_Rect__intG_new_const_Rect_int, std_pairLcv_Rect__intG_delete,
		0 = arg: core::Rect, get_0 via std_pairLcv_Rect__intG_get_0_const,
		1 = arg_1: i32, get_1 via std_pairLcv_Rect__intG_get_1_const
	}

	impl core::Tuple<(i32, f32)> {
		pub fn as_raw_TupleOfi32_f32(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_TupleOfi32_f32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	tuple_extern! { (i32, f32),
		std_pairLint__floatG_new_const_int_float, std_pairLint__floatG_delete,
		0 = arg: i32, get_0 via std_pairLint__floatG_get_0_const,
		1 = arg_1: f32, get_1 via std_pairLint__floatG_get_1_const
	}

	impl core::Vector<core::DMatch> {
		pub fn as_raw_VectorOfDMatch(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfDMatch(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::DMatch,
		std_vectorLcv_DMatchG_new_const, std_vectorLcv_DMatchG_delete,
		std_vectorLcv_DMatchG_len_const, std_vectorLcv_DMatchG_isEmpty_const,
		std_vectorLcv_DMatchG_capacity_const, std_vectorLcv_DMatchG_shrinkToFit,
		std_vectorLcv_DMatchG_reserve_size_t, std_vectorLcv_DMatchG_remove_size_t,
		std_vectorLcv_DMatchG_swap_size_t_size_t, std_vectorLcv_DMatchG_clear,
		std_vectorLcv_DMatchG_get_const_size_t, std_vectorLcv_DMatchG_set_size_t_const_DMatch,
		std_vectorLcv_DMatchG_push_const_DMatch, std_vectorLcv_DMatchG_insert_size_t_const_DMatch,
	}

	vector_copy_non_bool! { core::DMatch,
		std_vectorLcv_DMatchG_data_const, std_vectorLcv_DMatchG_dataMut, cv_fromSlice_const_const_DMatchX_size_t,
		std_vectorLcv_DMatchG_clone_const,
	}


	impl core::Vector<core::KeyPoint> {
		pub fn as_raw_VectorOfKeyPoint(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfKeyPoint(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::KeyPoint,
		std_vectorLcv_KeyPointG_new_const, std_vectorLcv_KeyPointG_delete,
		std_vectorLcv_KeyPointG_len_const, std_vectorLcv_KeyPointG_isEmpty_const,
		std_vectorLcv_KeyPointG_capacity_const, std_vectorLcv_KeyPointG_shrinkToFit,
		std_vectorLcv_KeyPointG_reserve_size_t, std_vectorLcv_KeyPointG_remove_size_t,
		std_vectorLcv_KeyPointG_swap_size_t_size_t, std_vectorLcv_KeyPointG_clear,
		std_vectorLcv_KeyPointG_get_const_size_t, std_vectorLcv_KeyPointG_set_size_t_const_KeyPoint,
		std_vectorLcv_KeyPointG_push_const_KeyPoint, std_vectorLcv_KeyPointG_insert_size_t_const_KeyPoint,
	}

	vector_non_copy_or_bool! { clone core::KeyPoint }

	vector_boxed_ref! { core::KeyPoint }

	vector_extern! { BoxedRef<'t, core::KeyPoint>,
		std_vectorLcv_KeyPointG_new_const, std_vectorLcv_KeyPointG_delete,
		std_vectorLcv_KeyPointG_len_const, std_vectorLcv_KeyPointG_isEmpty_const,
		std_vectorLcv_KeyPointG_capacity_const, std_vectorLcv_KeyPointG_shrinkToFit,
		std_vectorLcv_KeyPointG_reserve_size_t, std_vectorLcv_KeyPointG_remove_size_t,
		std_vectorLcv_KeyPointG_swap_size_t_size_t, std_vectorLcv_KeyPointG_clear,
		std_vectorLcv_KeyPointG_get_const_size_t, std_vectorLcv_KeyPointG_set_size_t_const_KeyPoint,
		std_vectorLcv_KeyPointG_push_const_KeyPoint, std_vectorLcv_KeyPointG_insert_size_t_const_KeyPoint,
	}


	impl core::Vector<core::Mat> {
		pub fn as_raw_VectorOfMat(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfMat(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Mat,
		std_vectorLcv_MatG_new_const, std_vectorLcv_MatG_delete,
		std_vectorLcv_MatG_len_const, std_vectorLcv_MatG_isEmpty_const,
		std_vectorLcv_MatG_capacity_const, std_vectorLcv_MatG_shrinkToFit,
		std_vectorLcv_MatG_reserve_size_t, std_vectorLcv_MatG_remove_size_t,
		std_vectorLcv_MatG_swap_size_t_size_t, std_vectorLcv_MatG_clear,
		std_vectorLcv_MatG_get_const_size_t, std_vectorLcv_MatG_set_size_t_const_Mat,
		std_vectorLcv_MatG_push_const_Mat, std_vectorLcv_MatG_insert_size_t_const_Mat,
	}

	vector_non_copy_or_bool! { clone core::Mat }

	vector_boxed_ref! { core::Mat }

	vector_extern! { BoxedRef<'t, core::Mat>,
		std_vectorLcv_MatG_new_const, std_vectorLcv_MatG_delete,
		std_vectorLcv_MatG_len_const, std_vectorLcv_MatG_isEmpty_const,
		std_vectorLcv_MatG_capacity_const, std_vectorLcv_MatG_shrinkToFit,
		std_vectorLcv_MatG_reserve_size_t, std_vectorLcv_MatG_remove_size_t,
		std_vectorLcv_MatG_swap_size_t_size_t, std_vectorLcv_MatG_clear,
		std_vectorLcv_MatG_get_const_size_t, std_vectorLcv_MatG_set_size_t_const_Mat,
		std_vectorLcv_MatG_push_const_Mat, std_vectorLcv_MatG_insert_size_t_const_Mat,
	}


	impl core::Vector<core::PlatformInfo> {
		pub fn as_raw_VectorOfPlatformInfo(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPlatformInfo(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::PlatformInfo,
		std_vectorLcv_ocl_PlatformInfoG_new_const, std_vectorLcv_ocl_PlatformInfoG_delete,
		std_vectorLcv_ocl_PlatformInfoG_len_const, std_vectorLcv_ocl_PlatformInfoG_isEmpty_const,
		std_vectorLcv_ocl_PlatformInfoG_capacity_const, std_vectorLcv_ocl_PlatformInfoG_shrinkToFit,
		std_vectorLcv_ocl_PlatformInfoG_reserve_size_t, std_vectorLcv_ocl_PlatformInfoG_remove_size_t,
		std_vectorLcv_ocl_PlatformInfoG_swap_size_t_size_t, std_vectorLcv_ocl_PlatformInfoG_clear,
		std_vectorLcv_ocl_PlatformInfoG_get_const_size_t, std_vectorLcv_ocl_PlatformInfoG_set_size_t_const_PlatformInfo,
		std_vectorLcv_ocl_PlatformInfoG_push_const_PlatformInfo, std_vectorLcv_ocl_PlatformInfoG_insert_size_t_const_PlatformInfo,
	}

	vector_non_copy_or_bool! { core::PlatformInfo }

	vector_boxed_ref! { core::PlatformInfo }

	vector_extern! { BoxedRef<'t, core::PlatformInfo>,
		std_vectorLcv_ocl_PlatformInfoG_new_const, std_vectorLcv_ocl_PlatformInfoG_delete,
		std_vectorLcv_ocl_PlatformInfoG_len_const, std_vectorLcv_ocl_PlatformInfoG_isEmpty_const,
		std_vectorLcv_ocl_PlatformInfoG_capacity_const, std_vectorLcv_ocl_PlatformInfoG_shrinkToFit,
		std_vectorLcv_ocl_PlatformInfoG_reserve_size_t, std_vectorLcv_ocl_PlatformInfoG_remove_size_t,
		std_vectorLcv_ocl_PlatformInfoG_swap_size_t_size_t, std_vectorLcv_ocl_PlatformInfoG_clear,
		std_vectorLcv_ocl_PlatformInfoG_get_const_size_t, std_vectorLcv_ocl_PlatformInfoG_set_size_t_const_PlatformInfo,
		std_vectorLcv_ocl_PlatformInfoG_push_const_PlatformInfo, std_vectorLcv_ocl_PlatformInfoG_insert_size_t_const_PlatformInfo,
	}


	impl core::Vector<core::Point> {
		pub fn as_raw_VectorOfPoint(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPoint(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Point,
		std_vectorLcv_PointG_new_const, std_vectorLcv_PointG_delete,
		std_vectorLcv_PointG_len_const, std_vectorLcv_PointG_isEmpty_const,
		std_vectorLcv_PointG_capacity_const, std_vectorLcv_PointG_shrinkToFit,
		std_vectorLcv_PointG_reserve_size_t, std_vectorLcv_PointG_remove_size_t,
		std_vectorLcv_PointG_swap_size_t_size_t, std_vectorLcv_PointG_clear,
		std_vectorLcv_PointG_get_const_size_t, std_vectorLcv_PointG_set_size_t_const_Point,
		std_vectorLcv_PointG_push_const_Point, std_vectorLcv_PointG_insert_size_t_const_Point,
	}

	vector_copy_non_bool! { core::Point,
		std_vectorLcv_PointG_data_const, std_vectorLcv_PointG_dataMut, cv_fromSlice_const_const_PointX_size_t,
		std_vectorLcv_PointG_clone_const,
	}

	impl ToInputArray for core::Vector<core::Point> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_PointG_inputArray_const(self.as_raw_VectorOfPoint(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<core::Point> }

	impl ToOutputArray for core::Vector<core::Point> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_PointG_outputArray(self.as_raw_mut_VectorOfPoint(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<core::Point> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_PointG_inputOutputArray(self.as_raw_mut_VectorOfPoint(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<core::Point> }

	impl core::Vector<core::Point2d> {
		pub fn as_raw_VectorOfPoint2d(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPoint2d(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Point2d,
		std_vectorLcv_Point2dG_new_const, std_vectorLcv_Point2dG_delete,
		std_vectorLcv_Point2dG_len_const, std_vectorLcv_Point2dG_isEmpty_const,
		std_vectorLcv_Point2dG_capacity_const, std_vectorLcv_Point2dG_shrinkToFit,
		std_vectorLcv_Point2dG_reserve_size_t, std_vectorLcv_Point2dG_remove_size_t,
		std_vectorLcv_Point2dG_swap_size_t_size_t, std_vectorLcv_Point2dG_clear,
		std_vectorLcv_Point2dG_get_const_size_t, std_vectorLcv_Point2dG_set_size_t_const_Point2d,
		std_vectorLcv_Point2dG_push_const_Point2d, std_vectorLcv_Point2dG_insert_size_t_const_Point2d,
	}

	vector_copy_non_bool! { core::Point2d,
		std_vectorLcv_Point2dG_data_const, std_vectorLcv_Point2dG_dataMut, cv_fromSlice_const_const_Point2dX_size_t,
		std_vectorLcv_Point2dG_clone_const,
	}

	impl ToInputArray for core::Vector<core::Point2d> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Point2dG_inputArray_const(self.as_raw_VectorOfPoint2d(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<core::Point2d> }

	impl ToOutputArray for core::Vector<core::Point2d> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Point2dG_outputArray(self.as_raw_mut_VectorOfPoint2d(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<core::Point2d> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Point2dG_inputOutputArray(self.as_raw_mut_VectorOfPoint2d(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<core::Point2d> }

	impl core::Vector<core::Point2f> {
		pub fn as_raw_VectorOfPoint2f(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPoint2f(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Point2f,
		std_vectorLcv_Point2fG_new_const, std_vectorLcv_Point2fG_delete,
		std_vectorLcv_Point2fG_len_const, std_vectorLcv_Point2fG_isEmpty_const,
		std_vectorLcv_Point2fG_capacity_const, std_vectorLcv_Point2fG_shrinkToFit,
		std_vectorLcv_Point2fG_reserve_size_t, std_vectorLcv_Point2fG_remove_size_t,
		std_vectorLcv_Point2fG_swap_size_t_size_t, std_vectorLcv_Point2fG_clear,
		std_vectorLcv_Point2fG_get_const_size_t, std_vectorLcv_Point2fG_set_size_t_const_Point2f,
		std_vectorLcv_Point2fG_push_const_Point2f, std_vectorLcv_Point2fG_insert_size_t_const_Point2f,
	}

	vector_copy_non_bool! { core::Point2f,
		std_vectorLcv_Point2fG_data_const, std_vectorLcv_Point2fG_dataMut, cv_fromSlice_const_const_Point2fX_size_t,
		std_vectorLcv_Point2fG_clone_const,
	}

	impl ToInputArray for core::Vector<core::Point2f> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Point2fG_inputArray_const(self.as_raw_VectorOfPoint2f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<core::Point2f> }

	impl ToOutputArray for core::Vector<core::Point2f> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Point2fG_outputArray(self.as_raw_mut_VectorOfPoint2f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<core::Point2f> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Point2fG_inputOutputArray(self.as_raw_mut_VectorOfPoint2f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<core::Point2f> }

	impl core::Vector<core::Range> {
		pub fn as_raw_VectorOfRange(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfRange(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Range,
		std_vectorLcv_RangeG_new_const, std_vectorLcv_RangeG_delete,
		std_vectorLcv_RangeG_len_const, std_vectorLcv_RangeG_isEmpty_const,
		std_vectorLcv_RangeG_capacity_const, std_vectorLcv_RangeG_shrinkToFit,
		std_vectorLcv_RangeG_reserve_size_t, std_vectorLcv_RangeG_remove_size_t,
		std_vectorLcv_RangeG_swap_size_t_size_t, std_vectorLcv_RangeG_clear,
		std_vectorLcv_RangeG_get_const_size_t, std_vectorLcv_RangeG_set_size_t_const_Range,
		std_vectorLcv_RangeG_push_const_Range, std_vectorLcv_RangeG_insert_size_t_const_Range,
	}

	vector_non_copy_or_bool! { core::Range }

	vector_boxed_ref! { core::Range }

	vector_extern! { BoxedRef<'t, core::Range>,
		std_vectorLcv_RangeG_new_const, std_vectorLcv_RangeG_delete,
		std_vectorLcv_RangeG_len_const, std_vectorLcv_RangeG_isEmpty_const,
		std_vectorLcv_RangeG_capacity_const, std_vectorLcv_RangeG_shrinkToFit,
		std_vectorLcv_RangeG_reserve_size_t, std_vectorLcv_RangeG_remove_size_t,
		std_vectorLcv_RangeG_swap_size_t_size_t, std_vectorLcv_RangeG_clear,
		std_vectorLcv_RangeG_get_const_size_t, std_vectorLcv_RangeG_set_size_t_const_Range,
		std_vectorLcv_RangeG_push_const_Range, std_vectorLcv_RangeG_insert_size_t_const_Range,
	}


	impl core::Vector<core::Rect> {
		pub fn as_raw_VectorOfRect(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfRect(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Rect,
		std_vectorLcv_RectG_new_const, std_vectorLcv_RectG_delete,
		std_vectorLcv_RectG_len_const, std_vectorLcv_RectG_isEmpty_const,
		std_vectorLcv_RectG_capacity_const, std_vectorLcv_RectG_shrinkToFit,
		std_vectorLcv_RectG_reserve_size_t, std_vectorLcv_RectG_remove_size_t,
		std_vectorLcv_RectG_swap_size_t_size_t, std_vectorLcv_RectG_clear,
		std_vectorLcv_RectG_get_const_size_t, std_vectorLcv_RectG_set_size_t_const_Rect,
		std_vectorLcv_RectG_push_const_Rect, std_vectorLcv_RectG_insert_size_t_const_Rect,
	}

	vector_copy_non_bool! { core::Rect,
		std_vectorLcv_RectG_data_const, std_vectorLcv_RectG_dataMut, cv_fromSlice_const_const_RectX_size_t,
		std_vectorLcv_RectG_clone_const,
	}

	impl ToInputArray for core::Vector<core::Rect> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_RectG_inputArray_const(self.as_raw_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<core::Rect> }

	impl ToOutputArray for core::Vector<core::Rect> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_RectG_outputArray(self.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<core::Rect> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_RectG_inputOutputArray(self.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<core::Rect> }

	impl core::Vector<core::Rect2d> {
		pub fn as_raw_VectorOfRect2d(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfRect2d(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Rect2d,
		std_vectorLcv_Rect2dG_new_const, std_vectorLcv_Rect2dG_delete,
		std_vectorLcv_Rect2dG_len_const, std_vectorLcv_Rect2dG_isEmpty_const,
		std_vectorLcv_Rect2dG_capacity_const, std_vectorLcv_Rect2dG_shrinkToFit,
		std_vectorLcv_Rect2dG_reserve_size_t, std_vectorLcv_Rect2dG_remove_size_t,
		std_vectorLcv_Rect2dG_swap_size_t_size_t, std_vectorLcv_Rect2dG_clear,
		std_vectorLcv_Rect2dG_get_const_size_t, std_vectorLcv_Rect2dG_set_size_t_const_Rect2d,
		std_vectorLcv_Rect2dG_push_const_Rect2d, std_vectorLcv_Rect2dG_insert_size_t_const_Rect2d,
	}

	vector_copy_non_bool! { core::Rect2d,
		std_vectorLcv_Rect2dG_data_const, std_vectorLcv_Rect2dG_dataMut, cv_fromSlice_const_const_Rect2dX_size_t,
		std_vectorLcv_Rect2dG_clone_const,
	}

	impl ToInputArray for core::Vector<core::Rect2d> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Rect2dG_inputArray_const(self.as_raw_VectorOfRect2d(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<core::Rect2d> }

	impl ToOutputArray for core::Vector<core::Rect2d> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Rect2dG_outputArray(self.as_raw_mut_VectorOfRect2d(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<core::Rect2d> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Rect2dG_inputOutputArray(self.as_raw_mut_VectorOfRect2d(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<core::Rect2d> }

	impl core::Vector<core::RotatedRect> {
		pub fn as_raw_VectorOfRotatedRect(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfRotatedRect(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::RotatedRect,
		std_vectorLcv_RotatedRectG_new_const, std_vectorLcv_RotatedRectG_delete,
		std_vectorLcv_RotatedRectG_len_const, std_vectorLcv_RotatedRectG_isEmpty_const,
		std_vectorLcv_RotatedRectG_capacity_const, std_vectorLcv_RotatedRectG_shrinkToFit,
		std_vectorLcv_RotatedRectG_reserve_size_t, std_vectorLcv_RotatedRectG_remove_size_t,
		std_vectorLcv_RotatedRectG_swap_size_t_size_t, std_vectorLcv_RotatedRectG_clear,
		std_vectorLcv_RotatedRectG_get_const_size_t, std_vectorLcv_RotatedRectG_set_size_t_const_RotatedRect,
		std_vectorLcv_RotatedRectG_push_const_RotatedRect, std_vectorLcv_RotatedRectG_insert_size_t_const_RotatedRect,
	}

	vector_copy_non_bool! { core::RotatedRect,
		std_vectorLcv_RotatedRectG_data_const, std_vectorLcv_RotatedRectG_dataMut, cv_fromSlice_const_const_RotatedRectX_size_t,
		std_vectorLcv_RotatedRectG_clone_const,
	}


	impl core::Vector<String> {
		pub fn as_raw_VectorOfString(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfString(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { String,
		std_vectorLcv_StringG_new_const, std_vectorLcv_StringG_delete,
		std_vectorLcv_StringG_len_const, std_vectorLcv_StringG_isEmpty_const,
		std_vectorLcv_StringG_capacity_const, std_vectorLcv_StringG_shrinkToFit,
		std_vectorLcv_StringG_reserve_size_t, std_vectorLcv_StringG_remove_size_t,
		std_vectorLcv_StringG_swap_size_t_size_t, std_vectorLcv_StringG_clear,
		std_vectorLcv_StringG_get_const_size_t, std_vectorLcv_StringG_set_size_t_const_String,
		std_vectorLcv_StringG_push_const_String, std_vectorLcv_StringG_insert_size_t_const_String,
	}

	vector_non_copy_or_bool! { String }


	impl core::Vector<core::UMat> {
		pub fn as_raw_VectorOfUMat(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfUMat(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::UMat,
		std_vectorLcv_UMatG_new_const, std_vectorLcv_UMatG_delete,
		std_vectorLcv_UMatG_len_const, std_vectorLcv_UMatG_isEmpty_const,
		std_vectorLcv_UMatG_capacity_const, std_vectorLcv_UMatG_shrinkToFit,
		std_vectorLcv_UMatG_reserve_size_t, std_vectorLcv_UMatG_remove_size_t,
		std_vectorLcv_UMatG_swap_size_t_size_t, std_vectorLcv_UMatG_clear,
		std_vectorLcv_UMatG_get_const_size_t, std_vectorLcv_UMatG_set_size_t_const_UMat,
		std_vectorLcv_UMatG_push_const_UMat, std_vectorLcv_UMatG_insert_size_t_const_UMat,
	}

	vector_non_copy_or_bool! { clone core::UMat }

	vector_boxed_ref! { core::UMat }

	vector_extern! { BoxedRef<'t, core::UMat>,
		std_vectorLcv_UMatG_new_const, std_vectorLcv_UMatG_delete,
		std_vectorLcv_UMatG_len_const, std_vectorLcv_UMatG_isEmpty_const,
		std_vectorLcv_UMatG_capacity_const, std_vectorLcv_UMatG_shrinkToFit,
		std_vectorLcv_UMatG_reserve_size_t, std_vectorLcv_UMatG_remove_size_t,
		std_vectorLcv_UMatG_swap_size_t_size_t, std_vectorLcv_UMatG_clear,
		std_vectorLcv_UMatG_get_const_size_t, std_vectorLcv_UMatG_set_size_t_const_UMat,
		std_vectorLcv_UMatG_push_const_UMat, std_vectorLcv_UMatG_insert_size_t_const_UMat,
	}


	impl core::Vector<core::Vec2d> {
		pub fn as_raw_VectorOfVec2d(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec2d(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Vec2d,
		std_vectorLcv_Vec2dG_new_const, std_vectorLcv_Vec2dG_delete,
		std_vectorLcv_Vec2dG_len_const, std_vectorLcv_Vec2dG_isEmpty_const,
		std_vectorLcv_Vec2dG_capacity_const, std_vectorLcv_Vec2dG_shrinkToFit,
		std_vectorLcv_Vec2dG_reserve_size_t, std_vectorLcv_Vec2dG_remove_size_t,
		std_vectorLcv_Vec2dG_swap_size_t_size_t, std_vectorLcv_Vec2dG_clear,
		std_vectorLcv_Vec2dG_get_const_size_t, std_vectorLcv_Vec2dG_set_size_t_const_Vec2d,
		std_vectorLcv_Vec2dG_push_const_Vec2d, std_vectorLcv_Vec2dG_insert_size_t_const_Vec2d,
	}

	vector_copy_non_bool! { core::Vec2d,
		std_vectorLcv_Vec2dG_data_const, std_vectorLcv_Vec2dG_dataMut, cv_fromSlice_const_const_Vec2dX_size_t,
		std_vectorLcv_Vec2dG_clone_const,
	}

	impl ToInputArray for core::Vector<core::Vec2d> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec2dG_inputArray_const(self.as_raw_VectorOfVec2d(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<core::Vec2d> }

	impl ToOutputArray for core::Vector<core::Vec2d> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec2dG_outputArray(self.as_raw_mut_VectorOfVec2d(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<core::Vec2d> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec2dG_inputOutputArray(self.as_raw_mut_VectorOfVec2d(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<core::Vec2d> }

	impl core::Vector<core::Vec2f> {
		pub fn as_raw_VectorOfVec2f(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec2f(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Vec2f,
		std_vectorLcv_Vec2fG_new_const, std_vectorLcv_Vec2fG_delete,
		std_vectorLcv_Vec2fG_len_const, std_vectorLcv_Vec2fG_isEmpty_const,
		std_vectorLcv_Vec2fG_capacity_const, std_vectorLcv_Vec2fG_shrinkToFit,
		std_vectorLcv_Vec2fG_reserve_size_t, std_vectorLcv_Vec2fG_remove_size_t,
		std_vectorLcv_Vec2fG_swap_size_t_size_t, std_vectorLcv_Vec2fG_clear,
		std_vectorLcv_Vec2fG_get_const_size_t, std_vectorLcv_Vec2fG_set_size_t_const_Vec2f,
		std_vectorLcv_Vec2fG_push_const_Vec2f, std_vectorLcv_Vec2fG_insert_size_t_const_Vec2f,
	}

	vector_copy_non_bool! { core::Vec2f,
		std_vectorLcv_Vec2fG_data_const, std_vectorLcv_Vec2fG_dataMut, cv_fromSlice_const_const_Vec2fX_size_t,
		std_vectorLcv_Vec2fG_clone_const,
	}

	impl ToInputArray for core::Vector<core::Vec2f> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec2fG_inputArray_const(self.as_raw_VectorOfVec2f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<core::Vec2f> }

	impl ToOutputArray for core::Vector<core::Vec2f> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec2fG_outputArray(self.as_raw_mut_VectorOfVec2f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<core::Vec2f> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec2fG_inputOutputArray(self.as_raw_mut_VectorOfVec2f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<core::Vec2f> }

	impl core::Vector<core::Vec3d> {
		pub fn as_raw_VectorOfVec3d(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec3d(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Vec3d,
		std_vectorLcv_Vec3dG_new_const, std_vectorLcv_Vec3dG_delete,
		std_vectorLcv_Vec3dG_len_const, std_vectorLcv_Vec3dG_isEmpty_const,
		std_vectorLcv_Vec3dG_capacity_const, std_vectorLcv_Vec3dG_shrinkToFit,
		std_vectorLcv_Vec3dG_reserve_size_t, std_vectorLcv_Vec3dG_remove_size_t,
		std_vectorLcv_Vec3dG_swap_size_t_size_t, std_vectorLcv_Vec3dG_clear,
		std_vectorLcv_Vec3dG_get_const_size_t, std_vectorLcv_Vec3dG_set_size_t_const_Vec3d,
		std_vectorLcv_Vec3dG_push_const_Vec3d, std_vectorLcv_Vec3dG_insert_size_t_const_Vec3d,
	}

	vector_copy_non_bool! { core::Vec3d,
		std_vectorLcv_Vec3dG_data_const, std_vectorLcv_Vec3dG_dataMut, cv_fromSlice_const_const_Vec3dX_size_t,
		std_vectorLcv_Vec3dG_clone_const,
	}

	impl ToInputArray for core::Vector<core::Vec3d> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec3dG_inputArray_const(self.as_raw_VectorOfVec3d(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<core::Vec3d> }

	impl ToOutputArray for core::Vector<core::Vec3d> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec3dG_outputArray(self.as_raw_mut_VectorOfVec3d(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<core::Vec3d> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec3dG_inputOutputArray(self.as_raw_mut_VectorOfVec3d(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<core::Vec3d> }

	impl core::Vector<core::Vec3f> {
		pub fn as_raw_VectorOfVec3f(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec3f(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Vec3f,
		std_vectorLcv_Vec3fG_new_const, std_vectorLcv_Vec3fG_delete,
		std_vectorLcv_Vec3fG_len_const, std_vectorLcv_Vec3fG_isEmpty_const,
		std_vectorLcv_Vec3fG_capacity_const, std_vectorLcv_Vec3fG_shrinkToFit,
		std_vectorLcv_Vec3fG_reserve_size_t, std_vectorLcv_Vec3fG_remove_size_t,
		std_vectorLcv_Vec3fG_swap_size_t_size_t, std_vectorLcv_Vec3fG_clear,
		std_vectorLcv_Vec3fG_get_const_size_t, std_vectorLcv_Vec3fG_set_size_t_const_Vec3f,
		std_vectorLcv_Vec3fG_push_const_Vec3f, std_vectorLcv_Vec3fG_insert_size_t_const_Vec3f,
	}

	vector_copy_non_bool! { core::Vec3f,
		std_vectorLcv_Vec3fG_data_const, std_vectorLcv_Vec3fG_dataMut, cv_fromSlice_const_const_Vec3fX_size_t,
		std_vectorLcv_Vec3fG_clone_const,
	}

	impl ToInputArray for core::Vector<core::Vec3f> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec3fG_inputArray_const(self.as_raw_VectorOfVec3f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<core::Vec3f> }

	impl ToOutputArray for core::Vector<core::Vec3f> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec3fG_outputArray(self.as_raw_mut_VectorOfVec3f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<core::Vec3f> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec3fG_inputOutputArray(self.as_raw_mut_VectorOfVec3f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<core::Vec3f> }

	impl core::Vector<core::Vec4f> {
		pub fn as_raw_VectorOfVec4f(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec4f(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Vec4f,
		std_vectorLcv_Vec4fG_new_const, std_vectorLcv_Vec4fG_delete,
		std_vectorLcv_Vec4fG_len_const, std_vectorLcv_Vec4fG_isEmpty_const,
		std_vectorLcv_Vec4fG_capacity_const, std_vectorLcv_Vec4fG_shrinkToFit,
		std_vectorLcv_Vec4fG_reserve_size_t, std_vectorLcv_Vec4fG_remove_size_t,
		std_vectorLcv_Vec4fG_swap_size_t_size_t, std_vectorLcv_Vec4fG_clear,
		std_vectorLcv_Vec4fG_get_const_size_t, std_vectorLcv_Vec4fG_set_size_t_const_Vec4f,
		std_vectorLcv_Vec4fG_push_const_Vec4f, std_vectorLcv_Vec4fG_insert_size_t_const_Vec4f,
	}

	vector_copy_non_bool! { core::Vec4f,
		std_vectorLcv_Vec4fG_data_const, std_vectorLcv_Vec4fG_dataMut, cv_fromSlice_const_const_Vec4fX_size_t,
		std_vectorLcv_Vec4fG_clone_const,
	}

	impl ToInputArray for core::Vector<core::Vec4f> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec4fG_inputArray_const(self.as_raw_VectorOfVec4f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<core::Vec4f> }

	impl ToOutputArray for core::Vector<core::Vec4f> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec4fG_outputArray(self.as_raw_mut_VectorOfVec4f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<core::Vec4f> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec4fG_inputOutputArray(self.as_raw_mut_VectorOfVec4f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<core::Vec4f> }

	impl core::Vector<core::Vec4i> {
		pub fn as_raw_VectorOfVec4i(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec4i(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Vec4i,
		std_vectorLcv_Vec4iG_new_const, std_vectorLcv_Vec4iG_delete,
		std_vectorLcv_Vec4iG_len_const, std_vectorLcv_Vec4iG_isEmpty_const,
		std_vectorLcv_Vec4iG_capacity_const, std_vectorLcv_Vec4iG_shrinkToFit,
		std_vectorLcv_Vec4iG_reserve_size_t, std_vectorLcv_Vec4iG_remove_size_t,
		std_vectorLcv_Vec4iG_swap_size_t_size_t, std_vectorLcv_Vec4iG_clear,
		std_vectorLcv_Vec4iG_get_const_size_t, std_vectorLcv_Vec4iG_set_size_t_const_Vec4i,
		std_vectorLcv_Vec4iG_push_const_Vec4i, std_vectorLcv_Vec4iG_insert_size_t_const_Vec4i,
	}

	vector_copy_non_bool! { core::Vec4i,
		std_vectorLcv_Vec4iG_data_const, std_vectorLcv_Vec4iG_dataMut, cv_fromSlice_const_const_Vec4iX_size_t,
		std_vectorLcv_Vec4iG_clone_const,
	}

	impl ToInputArray for core::Vector<core::Vec4i> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec4iG_inputArray_const(self.as_raw_VectorOfVec4i(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<core::Vec4i> }

	impl ToOutputArray for core::Vector<core::Vec4i> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec4iG_outputArray(self.as_raw_mut_VectorOfVec4i(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<core::Vec4i> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec4iG_inputOutputArray(self.as_raw_mut_VectorOfVec4i(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<core::Vec4i> }

	impl core::Vector<core::Vec6f> {
		pub fn as_raw_VectorOfVec6f(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec6f(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Vec6f,
		std_vectorLcv_Vec6fG_new_const, std_vectorLcv_Vec6fG_delete,
		std_vectorLcv_Vec6fG_len_const, std_vectorLcv_Vec6fG_isEmpty_const,
		std_vectorLcv_Vec6fG_capacity_const, std_vectorLcv_Vec6fG_shrinkToFit,
		std_vectorLcv_Vec6fG_reserve_size_t, std_vectorLcv_Vec6fG_remove_size_t,
		std_vectorLcv_Vec6fG_swap_size_t_size_t, std_vectorLcv_Vec6fG_clear,
		std_vectorLcv_Vec6fG_get_const_size_t, std_vectorLcv_Vec6fG_set_size_t_const_Vec6f,
		std_vectorLcv_Vec6fG_push_const_Vec6f, std_vectorLcv_Vec6fG_insert_size_t_const_Vec6f,
	}

	vector_copy_non_bool! { core::Vec6f,
		std_vectorLcv_Vec6fG_data_const, std_vectorLcv_Vec6fG_dataMut, cv_fromSlice_const_const_Vec6fX_size_t,
		std_vectorLcv_Vec6fG_clone_const,
	}

	impl ToInputArray for core::Vector<core::Vec6f> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec6fG_inputArray_const(self.as_raw_VectorOfVec6f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<core::Vec6f> }

	impl ToOutputArray for core::Vector<core::Vec6f> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec6fG_outputArray(self.as_raw_mut_VectorOfVec6f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<core::Vec6f> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec6fG_inputOutputArray(self.as_raw_mut_VectorOfVec6f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<core::Vec6f> }

	impl core::Vector<core::Vector<core::Mat>> {
		pub fn as_raw_VectorOfVectorOfMat(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfMat(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Vector<core::Mat>,
		std_vectorLstd_vectorLcv_MatGG_new_const, std_vectorLstd_vectorLcv_MatGG_delete,
		std_vectorLstd_vectorLcv_MatGG_len_const, std_vectorLstd_vectorLcv_MatGG_isEmpty_const,
		std_vectorLstd_vectorLcv_MatGG_capacity_const, std_vectorLstd_vectorLcv_MatGG_shrinkToFit,
		std_vectorLstd_vectorLcv_MatGG_reserve_size_t, std_vectorLstd_vectorLcv_MatGG_remove_size_t,
		std_vectorLstd_vectorLcv_MatGG_swap_size_t_size_t, std_vectorLstd_vectorLcv_MatGG_clear,
		std_vectorLstd_vectorLcv_MatGG_get_const_size_t, std_vectorLstd_vectorLcv_MatGG_set_size_t_const_vectorLMatG,
		std_vectorLstd_vectorLcv_MatGG_push_const_vectorLMatG, std_vectorLstd_vectorLcv_MatGG_insert_size_t_const_vectorLMatG,
	}

	vector_non_copy_or_bool! { clone core::Vector<core::Mat> }


	impl core::Vector<core::Vector<core::Point>> {
		pub fn as_raw_VectorOfVectorOfPoint(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfPoint(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Vector<core::Point>,
		std_vectorLstd_vectorLcv_PointGG_new_const, std_vectorLstd_vectorLcv_PointGG_delete,
		std_vectorLstd_vectorLcv_PointGG_len_const, std_vectorLstd_vectorLcv_PointGG_isEmpty_const,
		std_vectorLstd_vectorLcv_PointGG_capacity_const, std_vectorLstd_vectorLcv_PointGG_shrinkToFit,
		std_vectorLstd_vectorLcv_PointGG_reserve_size_t, std_vectorLstd_vectorLcv_PointGG_remove_size_t,
		std_vectorLstd_vectorLcv_PointGG_swap_size_t_size_t, std_vectorLstd_vectorLcv_PointGG_clear,
		std_vectorLstd_vectorLcv_PointGG_get_const_size_t, std_vectorLstd_vectorLcv_PointGG_set_size_t_const_vectorLPointG,
		std_vectorLstd_vectorLcv_PointGG_push_const_vectorLPointG, std_vectorLstd_vectorLcv_PointGG_insert_size_t_const_vectorLPointG,
	}

	vector_non_copy_or_bool! { clone core::Vector<core::Point> }

	impl ToInputArray for core::Vector<core::Vector<core::Point>> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLstd_vectorLcv_PointGG_inputArray_const(self.as_raw_VectorOfVectorOfPoint(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<core::Vector<core::Point>> }

	impl ToOutputArray for core::Vector<core::Vector<core::Point>> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLstd_vectorLcv_PointGG_outputArray(self.as_raw_mut_VectorOfVectorOfPoint(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<core::Vector<core::Point>> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLstd_vectorLcv_PointGG_inputOutputArray(self.as_raw_mut_VectorOfVectorOfPoint(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<core::Vector<core::Point>> }

	impl core::Vector<core::Vector<core::Point2f>> {
		pub fn as_raw_VectorOfVectorOfPoint2f(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfPoint2f(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Vector<core::Point2f>,
		std_vectorLstd_vectorLcv_Point2fGG_new_const, std_vectorLstd_vectorLcv_Point2fGG_delete,
		std_vectorLstd_vectorLcv_Point2fGG_len_const, std_vectorLstd_vectorLcv_Point2fGG_isEmpty_const,
		std_vectorLstd_vectorLcv_Point2fGG_capacity_const, std_vectorLstd_vectorLcv_Point2fGG_shrinkToFit,
		std_vectorLstd_vectorLcv_Point2fGG_reserve_size_t, std_vectorLstd_vectorLcv_Point2fGG_remove_size_t,
		std_vectorLstd_vectorLcv_Point2fGG_swap_size_t_size_t, std_vectorLstd_vectorLcv_Point2fGG_clear,
		std_vectorLstd_vectorLcv_Point2fGG_get_const_size_t, std_vectorLstd_vectorLcv_Point2fGG_set_size_t_const_vectorLPoint2fG,
		std_vectorLstd_vectorLcv_Point2fGG_push_const_vectorLPoint2fG, std_vectorLstd_vectorLcv_Point2fGG_insert_size_t_const_vectorLPoint2fG,
	}

	vector_non_copy_or_bool! { clone core::Vector<core::Point2f> }

	impl ToInputArray for core::Vector<core::Vector<core::Point2f>> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLstd_vectorLcv_Point2fGG_inputArray_const(self.as_raw_VectorOfVectorOfPoint2f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<core::Vector<core::Point2f>> }

	impl ToOutputArray for core::Vector<core::Vector<core::Point2f>> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLstd_vectorLcv_Point2fGG_outputArray(self.as_raw_mut_VectorOfVectorOfPoint2f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<core::Vector<core::Point2f>> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLstd_vectorLcv_Point2fGG_inputOutputArray(self.as_raw_mut_VectorOfVectorOfPoint2f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<core::Vector<core::Point2f>> }

	impl core::Vector<core::Vector<core::Range>> {
		pub fn as_raw_VectorOfVectorOfRange(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfRange(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Vector<core::Range>,
		std_vectorLstd_vectorLcv_RangeGG_new_const, std_vectorLstd_vectorLcv_RangeGG_delete,
		std_vectorLstd_vectorLcv_RangeGG_len_const, std_vectorLstd_vectorLcv_RangeGG_isEmpty_const,
		std_vectorLstd_vectorLcv_RangeGG_capacity_const, std_vectorLstd_vectorLcv_RangeGG_shrinkToFit,
		std_vectorLstd_vectorLcv_RangeGG_reserve_size_t, std_vectorLstd_vectorLcv_RangeGG_remove_size_t,
		std_vectorLstd_vectorLcv_RangeGG_swap_size_t_size_t, std_vectorLstd_vectorLcv_RangeGG_clear,
		std_vectorLstd_vectorLcv_RangeGG_get_const_size_t, std_vectorLstd_vectorLcv_RangeGG_set_size_t_const_vectorLRangeG,
		std_vectorLstd_vectorLcv_RangeGG_push_const_vectorLRangeG, std_vectorLstd_vectorLcv_RangeGG_insert_size_t_const_vectorLRangeG,
	}

	vector_non_copy_or_bool! { core::Vector<core::Range> }


	impl core::Vector<core::Vector<f32>> {
		pub fn as_raw_VectorOfVectorOff32(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOff32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Vector<f32>,
		std_vectorLstd_vectorLfloatGG_new_const, std_vectorLstd_vectorLfloatGG_delete,
		std_vectorLstd_vectorLfloatGG_len_const, std_vectorLstd_vectorLfloatGG_isEmpty_const,
		std_vectorLstd_vectorLfloatGG_capacity_const, std_vectorLstd_vectorLfloatGG_shrinkToFit,
		std_vectorLstd_vectorLfloatGG_reserve_size_t, std_vectorLstd_vectorLfloatGG_remove_size_t,
		std_vectorLstd_vectorLfloatGG_swap_size_t_size_t, std_vectorLstd_vectorLfloatGG_clear,
		std_vectorLstd_vectorLfloatGG_get_const_size_t, std_vectorLstd_vectorLfloatGG_set_size_t_const_vectorLfloatG,
		std_vectorLstd_vectorLfloatGG_push_const_vectorLfloatG, std_vectorLstd_vectorLfloatGG_insert_size_t_const_vectorLfloatG,
	}

	vector_non_copy_or_bool! { clone core::Vector<f32> }

	impl ToInputArray for core::Vector<core::Vector<f32>> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLstd_vectorLfloatGG_inputArray_const(self.as_raw_VectorOfVectorOff32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<core::Vector<f32>> }

	impl ToOutputArray for core::Vector<core::Vector<f32>> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLstd_vectorLfloatGG_outputArray(self.as_raw_mut_VectorOfVectorOff32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<core::Vector<f32>> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLstd_vectorLfloatGG_inputOutputArray(self.as_raw_mut_VectorOfVectorOff32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<core::Vector<f32>> }

	impl core::Vector<core::Vector<i32>> {
		pub fn as_raw_VectorOfVectorOfi32(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfi32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Vector<i32>,
		std_vectorLstd_vectorLintGG_new_const, std_vectorLstd_vectorLintGG_delete,
		std_vectorLstd_vectorLintGG_len_const, std_vectorLstd_vectorLintGG_isEmpty_const,
		std_vectorLstd_vectorLintGG_capacity_const, std_vectorLstd_vectorLintGG_shrinkToFit,
		std_vectorLstd_vectorLintGG_reserve_size_t, std_vectorLstd_vectorLintGG_remove_size_t,
		std_vectorLstd_vectorLintGG_swap_size_t_size_t, std_vectorLstd_vectorLintGG_clear,
		std_vectorLstd_vectorLintGG_get_const_size_t, std_vectorLstd_vectorLintGG_set_size_t_const_vectorLintG,
		std_vectorLstd_vectorLintGG_push_const_vectorLintG, std_vectorLstd_vectorLintGG_insert_size_t_const_vectorLintG,
	}

	vector_non_copy_or_bool! { clone core::Vector<i32> }

	impl ToInputArray for core::Vector<core::Vector<i32>> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLstd_vectorLintGG_inputArray_const(self.as_raw_VectorOfVectorOfi32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<core::Vector<i32>> }

	impl ToOutputArray for core::Vector<core::Vector<i32>> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLstd_vectorLintGG_outputArray(self.as_raw_mut_VectorOfVectorOfi32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<core::Vector<i32>> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLstd_vectorLintGG_inputOutputArray(self.as_raw_mut_VectorOfVectorOfi32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<core::Vector<i32>> }

	impl core::Vector<bool> {
		pub fn as_raw_VectorOfbool(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfbool(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { bool,
		std_vectorLboolG_new_const, std_vectorLboolG_delete,
		std_vectorLboolG_len_const, std_vectorLboolG_isEmpty_const,
		std_vectorLboolG_capacity_const, std_vectorLboolG_shrinkToFit,
		std_vectorLboolG_reserve_size_t, std_vectorLboolG_remove_size_t,
		std_vectorLboolG_swap_size_t_size_t, std_vectorLboolG_clear,
		std_vectorLboolG_get_const_size_t, std_vectorLboolG_set_size_t_const_bool,
		std_vectorLboolG_push_const_bool, std_vectorLboolG_insert_size_t_const_bool,
	}

	vector_non_copy_or_bool! { clone bool }

	impl ToInputArray for core::Vector<bool> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLboolG_inputArray_const(self.as_raw_VectorOfbool(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<bool> }

	impl core::Vector<c_char> {
		pub fn as_raw_VectorOfc_char(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfc_char(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl core::Vector<f32> {
		pub fn as_raw_VectorOff32(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOff32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { f32,
		std_vectorLfloatG_new_const, std_vectorLfloatG_delete,
		std_vectorLfloatG_len_const, std_vectorLfloatG_isEmpty_const,
		std_vectorLfloatG_capacity_const, std_vectorLfloatG_shrinkToFit,
		std_vectorLfloatG_reserve_size_t, std_vectorLfloatG_remove_size_t,
		std_vectorLfloatG_swap_size_t_size_t, std_vectorLfloatG_clear,
		std_vectorLfloatG_get_const_size_t, std_vectorLfloatG_set_size_t_const_float,
		std_vectorLfloatG_push_const_float, std_vectorLfloatG_insert_size_t_const_float,
	}

	vector_copy_non_bool! { f32,
		std_vectorLfloatG_data_const, std_vectorLfloatG_dataMut, cv_fromSlice_const_const_floatX_size_t,
		std_vectorLfloatG_clone_const,
	}

	impl ToInputArray for core::Vector<f32> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLfloatG_inputArray_const(self.as_raw_VectorOff32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<f32> }

	impl ToOutputArray for core::Vector<f32> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLfloatG_outputArray(self.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<f32> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLfloatG_inputOutputArray(self.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<f32> }

	impl core::Vector<f64> {
		pub fn as_raw_VectorOff64(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOff64(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { f64,
		std_vectorLdoubleG_new_const, std_vectorLdoubleG_delete,
		std_vectorLdoubleG_len_const, std_vectorLdoubleG_isEmpty_const,
		std_vectorLdoubleG_capacity_const, std_vectorLdoubleG_shrinkToFit,
		std_vectorLdoubleG_reserve_size_t, std_vectorLdoubleG_remove_size_t,
		std_vectorLdoubleG_swap_size_t_size_t, std_vectorLdoubleG_clear,
		std_vectorLdoubleG_get_const_size_t, std_vectorLdoubleG_set_size_t_const_double,
		std_vectorLdoubleG_push_const_double, std_vectorLdoubleG_insert_size_t_const_double,
	}

	vector_copy_non_bool! { f64,
		std_vectorLdoubleG_data_const, std_vectorLdoubleG_dataMut, cv_fromSlice_const_const_doubleX_size_t,
		std_vectorLdoubleG_clone_const,
	}

	impl ToInputArray for core::Vector<f64> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLdoubleG_inputArray_const(self.as_raw_VectorOff64(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<f64> }

	impl ToOutputArray for core::Vector<f64> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLdoubleG_outputArray(self.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<f64> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLdoubleG_inputOutputArray(self.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<f64> }

	impl core::Vector<i32> {
		pub fn as_raw_VectorOfi32(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfi32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { i32,
		std_vectorLintG_new_const, std_vectorLintG_delete,
		std_vectorLintG_len_const, std_vectorLintG_isEmpty_const,
		std_vectorLintG_capacity_const, std_vectorLintG_shrinkToFit,
		std_vectorLintG_reserve_size_t, std_vectorLintG_remove_size_t,
		std_vectorLintG_swap_size_t_size_t, std_vectorLintG_clear,
		std_vectorLintG_get_const_size_t, std_vectorLintG_set_size_t_const_int,
		std_vectorLintG_push_const_int, std_vectorLintG_insert_size_t_const_int,
	}

	vector_copy_non_bool! { i32,
		std_vectorLintG_data_const, std_vectorLintG_dataMut, cv_fromSlice_const_const_intX_size_t,
		std_vectorLintG_clone_const,
	}

	impl ToInputArray for core::Vector<i32> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLintG_inputArray_const(self.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<i32> }

	impl ToOutputArray for core::Vector<i32> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLintG_outputArray(self.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<i32> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLintG_inputOutputArray(self.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<i32> }

	impl core::Vector<i8> {
		pub fn as_raw_VectorOfi8(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfi8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { i8,
		std_vectorLsigned_charG_new_const, std_vectorLsigned_charG_delete,
		std_vectorLsigned_charG_len_const, std_vectorLsigned_charG_isEmpty_const,
		std_vectorLsigned_charG_capacity_const, std_vectorLsigned_charG_shrinkToFit,
		std_vectorLsigned_charG_reserve_size_t, std_vectorLsigned_charG_remove_size_t,
		std_vectorLsigned_charG_swap_size_t_size_t, std_vectorLsigned_charG_clear,
		std_vectorLsigned_charG_get_const_size_t, std_vectorLsigned_charG_set_size_t_const_signed_char,
		std_vectorLsigned_charG_push_const_signed_char, std_vectorLsigned_charG_insert_size_t_const_signed_char,
	}

	vector_copy_non_bool! { i8,
		std_vectorLsigned_charG_data_const, std_vectorLsigned_charG_dataMut, cv_fromSlice_const_const_signed_charX_size_t,
		std_vectorLsigned_charG_clone_const,
	}


	impl core::Vector<size_t> {
		pub fn as_raw_VectorOfsize_t(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfsize_t(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { size_t,
		std_vectorLsize_tG_new_const, std_vectorLsize_tG_delete,
		std_vectorLsize_tG_len_const, std_vectorLsize_tG_isEmpty_const,
		std_vectorLsize_tG_capacity_const, std_vectorLsize_tG_shrinkToFit,
		std_vectorLsize_tG_reserve_size_t, std_vectorLsize_tG_remove_size_t,
		std_vectorLsize_tG_swap_size_t_size_t, std_vectorLsize_tG_clear,
		std_vectorLsize_tG_get_const_size_t, std_vectorLsize_tG_set_size_t_const_size_t,
		std_vectorLsize_tG_push_const_size_t, std_vectorLsize_tG_insert_size_t_const_size_t,
	}

	vector_copy_non_bool! { size_t,
		std_vectorLsize_tG_data_const, std_vectorLsize_tG_dataMut, cv_fromSlice_const_const_size_tX_size_t,
		std_vectorLsize_tG_clone_const,
	}


	impl core::Vector<u8> {
		pub fn as_raw_VectorOfu8(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfu8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { u8,
		std_vectorLunsigned_charG_new_const, std_vectorLunsigned_charG_delete,
		std_vectorLunsigned_charG_len_const, std_vectorLunsigned_charG_isEmpty_const,
		std_vectorLunsigned_charG_capacity_const, std_vectorLunsigned_charG_shrinkToFit,
		std_vectorLunsigned_charG_reserve_size_t, std_vectorLunsigned_charG_remove_size_t,
		std_vectorLunsigned_charG_swap_size_t_size_t, std_vectorLunsigned_charG_clear,
		std_vectorLunsigned_charG_get_const_size_t, std_vectorLunsigned_charG_set_size_t_const_unsigned_char,
		std_vectorLunsigned_charG_push_const_unsigned_char, std_vectorLunsigned_charG_insert_size_t_const_unsigned_char,
	}

	vector_copy_non_bool! { u8,
		std_vectorLunsigned_charG_data_const, std_vectorLunsigned_charG_dataMut, cv_fromSlice_const_const_unsigned_charX_size_t,
		std_vectorLunsigned_charG_clone_const,
	}

	impl ToInputArray for core::Vector<u8> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLunsigned_charG_inputArray_const(self.as_raw_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<u8> }

	impl ToOutputArray for core::Vector<u8> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLunsigned_charG_outputArray(self.as_raw_mut_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<u8> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLunsigned_charG_inputOutputArray(self.as_raw_mut_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<u8> }

	impl core::MatOpTraitConst for types::AbstractRefMut<'static, core::MatOp> {
		#[inline] fn as_raw_MatOp(&self) -> extern_send!(Self) { self.as_raw() }
	}

	impl core::MatOpTrait for types::AbstractRefMut<'static, core::MatOp> {
		#[inline] fn as_raw_mut_MatOp(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

}
pub use core_types::*;

mod dnn_types {
	use crate::{mod_prelude::*, core, types, sys};

	ptr_extern! { crate::dnn::AbsLayer,
		cv_PtrLcv_dnn_AbsLayerG_new_null_const, cv_PtrLcv_dnn_AbsLayerG_delete, cv_PtrLcv_dnn_AbsLayerG_getInnerPtr_const, cv_PtrLcv_dnn_AbsLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::AbsLayer, cv_PtrLcv_dnn_AbsLayerG_new_const_AbsLayer }
	impl core::Ptr<crate::dnn::AbsLayer> {
		#[inline] pub fn as_raw_PtrOfAbsLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAbsLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::AbsLayerTraitConst for core::Ptr<crate::dnn::AbsLayer> {
		#[inline] fn as_raw_AbsLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::AbsLayerTrait for core::Ptr<crate::dnn::AbsLayer> {
		#[inline] fn as_raw_mut_AbsLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::AbsLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::AbsLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::AbsLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_AbsLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::AbsLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::AbsLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::AbsLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_AbsLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::AbsLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::AbsLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::AbsLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_AbsLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::AbsLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfAbsLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::AccumLayer,
		cv_PtrLcv_dnn_AccumLayerG_new_null_const, cv_PtrLcv_dnn_AccumLayerG_delete, cv_PtrLcv_dnn_AccumLayerG_getInnerPtr_const, cv_PtrLcv_dnn_AccumLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::AccumLayer, cv_PtrLcv_dnn_AccumLayerG_new_const_AccumLayer }
	impl core::Ptr<crate::dnn::AccumLayer> {
		#[inline] pub fn as_raw_PtrOfAccumLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAccumLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::AccumLayerTraitConst for core::Ptr<crate::dnn::AccumLayer> {
		#[inline] fn as_raw_AccumLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::AccumLayerTrait for core::Ptr<crate::dnn::AccumLayer> {
		#[inline] fn as_raw_mut_AccumLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::AccumLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::AccumLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::AccumLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_AccumLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::AccumLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::AccumLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::AccumLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_AccumLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::AccumLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfAccumLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::ActivationLayer,
		cv_PtrLcv_dnn_ActivationLayerG_new_null_const, cv_PtrLcv_dnn_ActivationLayerG_delete, cv_PtrLcv_dnn_ActivationLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ActivationLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ActivationLayer, cv_PtrLcv_dnn_ActivationLayerG_new_const_ActivationLayer }
	impl core::Ptr<crate::dnn::ActivationLayer> {
		#[inline] pub fn as_raw_PtrOfActivationLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfActivationLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ActivationLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ActivationLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ActivationLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ActivationLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ActivationLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ActivationLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ActivationLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ActivationLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ActivationLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ActivationLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ActivationLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfActivationLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::ActivationLayerInt8,
		cv_PtrLcv_dnn_ActivationLayerInt8G_new_null_const, cv_PtrLcv_dnn_ActivationLayerInt8G_delete, cv_PtrLcv_dnn_ActivationLayerInt8G_getInnerPtr_const, cv_PtrLcv_dnn_ActivationLayerInt8G_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ActivationLayerInt8, cv_PtrLcv_dnn_ActivationLayerInt8G_new_const_ActivationLayerInt8 }
	impl core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] pub fn as_raw_PtrOfActivationLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfActivationLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ActivationLayerInt8TraitConst for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] fn as_raw_ActivationLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerInt8Trait for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] fn as_raw_mut_ActivationLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ActivationLayerInt8>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ActivationLayerInt8G_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ActivationLayerInt8>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_ActivationLayerInt8G_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ActivationLayerInt8>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ActivationLayerInt8G_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfActivationLayerInt8")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::BNLLLayer,
		cv_PtrLcv_dnn_BNLLLayerG_new_null_const, cv_PtrLcv_dnn_BNLLLayerG_delete, cv_PtrLcv_dnn_BNLLLayerG_getInnerPtr_const, cv_PtrLcv_dnn_BNLLLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::BNLLLayer, cv_PtrLcv_dnn_BNLLLayerG_new_const_BNLLLayer }
	impl core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] pub fn as_raw_PtrOfBNLLLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBNLLLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::BNLLLayerTraitConst for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] fn as_raw_BNLLLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::BNLLLayerTrait for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] fn as_raw_mut_BNLLLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::BNLLLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_BNLLLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::BNLLLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_BNLLLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::BNLLLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_BNLLLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfBNLLLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::BackendNode,
		cv_PtrLcv_dnn_BackendNodeG_new_null_const, cv_PtrLcv_dnn_BackendNodeG_delete, cv_PtrLcv_dnn_BackendNodeG_getInnerPtr_const, cv_PtrLcv_dnn_BackendNodeG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::BackendNode, cv_PtrLcv_dnn_BackendNodeG_new_const_BackendNode }
	impl core::Ptr<crate::dnn::BackendNode> {
		#[inline] pub fn as_raw_PtrOfBackendNode(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackendNode(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::BackendNodeTraitConst for core::Ptr<crate::dnn::BackendNode> {
		#[inline] fn as_raw_BackendNode(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::BackendNodeTrait for core::Ptr<crate::dnn::BackendNode> {
		#[inline] fn as_raw_mut_BackendNode(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl std::fmt::Debug for core::Ptr<crate::dnn::BackendNode> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfBackendNode")
				.field("backend_id", &crate::dnn::BackendNodeTraitConst::backend_id(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::BackendWrapper,
		cv_PtrLcv_dnn_BackendWrapperG_new_null_const, cv_PtrLcv_dnn_BackendWrapperG_delete, cv_PtrLcv_dnn_BackendWrapperG_getInnerPtr_const, cv_PtrLcv_dnn_BackendWrapperG_getInnerPtrMut
	}

	impl core::Ptr<crate::dnn::BackendWrapper> {
		#[inline] pub fn as_raw_PtrOfBackendWrapper(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackendWrapper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::BackendWrapperTraitConst for core::Ptr<crate::dnn::BackendWrapper> {
		#[inline] fn as_raw_BackendWrapper(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::BackendWrapperTrait for core::Ptr<crate::dnn::BackendWrapper> {
		#[inline] fn as_raw_mut_BackendWrapper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl std::fmt::Debug for core::Ptr<crate::dnn::BackendWrapper> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfBackendWrapper")
				.field("backend_id", &crate::dnn::BackendWrapperTraitConst::backend_id(self))
				.field("target_id", &crate::dnn::BackendWrapperTraitConst::target_id(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::BaseConvolutionLayer,
		cv_PtrLcv_dnn_BaseConvolutionLayerG_new_null_const, cv_PtrLcv_dnn_BaseConvolutionLayerG_delete, cv_PtrLcv_dnn_BaseConvolutionLayerG_getInnerPtr_const, cv_PtrLcv_dnn_BaseConvolutionLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::BaseConvolutionLayer, cv_PtrLcv_dnn_BaseConvolutionLayerG_new_const_BaseConvolutionLayer }
	impl core::Ptr<crate::dnn::BaseConvolutionLayer> {
		#[inline] pub fn as_raw_PtrOfBaseConvolutionLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBaseConvolutionLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::BaseConvolutionLayerTraitConst for core::Ptr<crate::dnn::BaseConvolutionLayer> {
		#[inline] fn as_raw_BaseConvolutionLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::BaseConvolutionLayerTrait for core::Ptr<crate::dnn::BaseConvolutionLayer> {
		#[inline] fn as_raw_mut_BaseConvolutionLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::BaseConvolutionLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::BaseConvolutionLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::BaseConvolutionLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_BaseConvolutionLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::BaseConvolutionLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::BaseConvolutionLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::BaseConvolutionLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_BaseConvolutionLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::BaseConvolutionLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfBaseConvolutionLayer")
				.field("kernel", &crate::dnn::BaseConvolutionLayerTraitConst::kernel(self))
				.field("stride", &crate::dnn::BaseConvolutionLayerTraitConst::stride(self))
				.field("pad", &crate::dnn::BaseConvolutionLayerTraitConst::pad(self))
				.field("dilation", &crate::dnn::BaseConvolutionLayerTraitConst::dilation(self))
				.field("adjust_pad", &crate::dnn::BaseConvolutionLayerTraitConst::adjust_pad(self))
				.field("adjust_pads", &crate::dnn::BaseConvolutionLayerTraitConst::adjust_pads(self))
				.field("kernel_size", &crate::dnn::BaseConvolutionLayerTraitConst::kernel_size(self))
				.field("strides", &crate::dnn::BaseConvolutionLayerTraitConst::strides(self))
				.field("dilations", &crate::dnn::BaseConvolutionLayerTraitConst::dilations(self))
				.field("pads_begin", &crate::dnn::BaseConvolutionLayerTraitConst::pads_begin(self))
				.field("pads_end", &crate::dnn::BaseConvolutionLayerTraitConst::pads_end(self))
				.field("pad_mode", &crate::dnn::BaseConvolutionLayerTraitConst::pad_mode(self))
				.field("num_output", &crate::dnn::BaseConvolutionLayerTraitConst::num_output(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::BatchNormLayer,
		cv_PtrLcv_dnn_BatchNormLayerG_new_null_const, cv_PtrLcv_dnn_BatchNormLayerG_delete, cv_PtrLcv_dnn_BatchNormLayerG_getInnerPtr_const, cv_PtrLcv_dnn_BatchNormLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::BatchNormLayer, cv_PtrLcv_dnn_BatchNormLayerG_new_const_BatchNormLayer }
	impl core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] pub fn as_raw_PtrOfBatchNormLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBatchNormLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::BatchNormLayerTraitConst for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] fn as_raw_BatchNormLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::BatchNormLayerTrait for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] fn as_raw_mut_BatchNormLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::BatchNormLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_BatchNormLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::BatchNormLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_BatchNormLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::BatchNormLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_BatchNormLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfBatchNormLayer")
				.field("has_weights", &crate::dnn::BatchNormLayerTraitConst::has_weights(self))
				.field("has_bias", &crate::dnn::BatchNormLayerTraitConst::has_bias(self))
				.field("epsilon", &crate::dnn::BatchNormLayerTraitConst::epsilon(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::BatchNormLayerInt8,
		cv_PtrLcv_dnn_BatchNormLayerInt8G_new_null_const, cv_PtrLcv_dnn_BatchNormLayerInt8G_delete, cv_PtrLcv_dnn_BatchNormLayerInt8G_getInnerPtr_const, cv_PtrLcv_dnn_BatchNormLayerInt8G_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::BatchNormLayerInt8, cv_PtrLcv_dnn_BatchNormLayerInt8G_new_const_BatchNormLayerInt8 }
	impl core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] pub fn as_raw_PtrOfBatchNormLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBatchNormLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::BatchNormLayerInt8TraitConst for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_BatchNormLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::BatchNormLayerInt8Trait for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_mut_BatchNormLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::BatchNormLayerInt8>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_BatchNormLayerInt8G_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::BatchNormLayerInt8>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_BatchNormLayerInt8G_to_PtrOfActivationLayer }

	impl crate::dnn::BatchNormLayerTraitConst for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_BatchNormLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::BatchNormLayerTrait for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_mut_BatchNormLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::BatchNormLayerInt8>, core::Ptr<crate::dnn::BatchNormLayer>, cv_PtrLcv_dnn_BatchNormLayerInt8G_to_PtrOfBatchNormLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::BatchNormLayerInt8>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_BatchNormLayerInt8G_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfBatchNormLayerInt8")
				.field("input_sc", &crate::dnn::BatchNormLayerInt8TraitConst::input_sc(self))
				.field("output_sc", &crate::dnn::BatchNormLayerInt8TraitConst::output_sc(self))
				.field("input_zp", &crate::dnn::BatchNormLayerInt8TraitConst::input_zp(self))
				.field("output_zp", &crate::dnn::BatchNormLayerInt8TraitConst::output_zp(self))
				.field("has_weights", &crate::dnn::BatchNormLayerTraitConst::has_weights(self))
				.field("has_bias", &crate::dnn::BatchNormLayerTraitConst::has_bias(self))
				.field("epsilon", &crate::dnn::BatchNormLayerTraitConst::epsilon(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::BlankLayer,
		cv_PtrLcv_dnn_BlankLayerG_new_null_const, cv_PtrLcv_dnn_BlankLayerG_delete, cv_PtrLcv_dnn_BlankLayerG_getInnerPtr_const, cv_PtrLcv_dnn_BlankLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::BlankLayer, cv_PtrLcv_dnn_BlankLayerG_new_const_BlankLayer }
	impl core::Ptr<crate::dnn::BlankLayer> {
		#[inline] pub fn as_raw_PtrOfBlankLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBlankLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::BlankLayerTraitConst for core::Ptr<crate::dnn::BlankLayer> {
		#[inline] fn as_raw_BlankLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::BlankLayerTrait for core::Ptr<crate::dnn::BlankLayer> {
		#[inline] fn as_raw_mut_BlankLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::BlankLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::BlankLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::BlankLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_BlankLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::BlankLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::BlankLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::BlankLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_BlankLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::BlankLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfBlankLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::ChannelsPReLULayer,
		cv_PtrLcv_dnn_ChannelsPReLULayerG_new_null_const, cv_PtrLcv_dnn_ChannelsPReLULayerG_delete, cv_PtrLcv_dnn_ChannelsPReLULayerG_getInnerPtr_const, cv_PtrLcv_dnn_ChannelsPReLULayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ChannelsPReLULayer, cv_PtrLcv_dnn_ChannelsPReLULayerG_new_const_ChannelsPReLULayer }
	impl core::Ptr<crate::dnn::ChannelsPReLULayer> {
		#[inline] pub fn as_raw_PtrOfChannelsPReLULayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfChannelsPReLULayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ChannelsPReLULayerTraitConst for core::Ptr<crate::dnn::ChannelsPReLULayer> {
		#[inline] fn as_raw_ChannelsPReLULayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ChannelsPReLULayerTrait for core::Ptr<crate::dnn::ChannelsPReLULayer> {
		#[inline] fn as_raw_mut_ChannelsPReLULayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ChannelsPReLULayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ChannelsPReLULayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ChannelsPReLULayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ChannelsPReLULayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ChannelsPReLULayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ChannelsPReLULayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ChannelsPReLULayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_ChannelsPReLULayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ChannelsPReLULayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ChannelsPReLULayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ChannelsPReLULayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ChannelsPReLULayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ChannelsPReLULayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfChannelsPReLULayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::ConcatLayer,
		cv_PtrLcv_dnn_ConcatLayerG_new_null_const, cv_PtrLcv_dnn_ConcatLayerG_delete, cv_PtrLcv_dnn_ConcatLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ConcatLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ConcatLayer, cv_PtrLcv_dnn_ConcatLayerG_new_const_ConcatLayer }
	impl core::Ptr<crate::dnn::ConcatLayer> {
		#[inline] pub fn as_raw_PtrOfConcatLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfConcatLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ConcatLayerTraitConst for core::Ptr<crate::dnn::ConcatLayer> {
		#[inline] fn as_raw_ConcatLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ConcatLayerTrait for core::Ptr<crate::dnn::ConcatLayer> {
		#[inline] fn as_raw_mut_ConcatLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ConcatLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ConcatLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ConcatLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ConcatLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ConcatLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ConcatLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ConcatLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ConcatLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ConcatLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfConcatLayer")
				.field("axis", &crate::dnn::ConcatLayerTraitConst::axis(self))
				.field("padding", &crate::dnn::ConcatLayerTraitConst::padding(self))
				.field("padding_value", &crate::dnn::ConcatLayerTraitConst::padding_value(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::ConstLayer,
		cv_PtrLcv_dnn_ConstLayerG_new_null_const, cv_PtrLcv_dnn_ConstLayerG_delete, cv_PtrLcv_dnn_ConstLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ConstLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ConstLayer, cv_PtrLcv_dnn_ConstLayerG_new_const_ConstLayer }
	impl core::Ptr<crate::dnn::ConstLayer> {
		#[inline] pub fn as_raw_PtrOfConstLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfConstLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ConstLayerTraitConst for core::Ptr<crate::dnn::ConstLayer> {
		#[inline] fn as_raw_ConstLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ConstLayerTrait for core::Ptr<crate::dnn::ConstLayer> {
		#[inline] fn as_raw_mut_ConstLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ConstLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ConstLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ConstLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ConstLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ConstLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ConstLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ConstLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ConstLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ConstLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfConstLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::ConvolutionLayer,
		cv_PtrLcv_dnn_ConvolutionLayerG_new_null_const, cv_PtrLcv_dnn_ConvolutionLayerG_delete, cv_PtrLcv_dnn_ConvolutionLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ConvolutionLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ConvolutionLayer, cv_PtrLcv_dnn_ConvolutionLayerG_new_const_ConvolutionLayer }
	impl core::Ptr<crate::dnn::ConvolutionLayer> {
		#[inline] pub fn as_raw_PtrOfConvolutionLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfConvolutionLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ConvolutionLayerTraitConst for core::Ptr<crate::dnn::ConvolutionLayer> {
		#[inline] fn as_raw_ConvolutionLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ConvolutionLayerTrait for core::Ptr<crate::dnn::ConvolutionLayer> {
		#[inline] fn as_raw_mut_ConvolutionLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ConvolutionLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ConvolutionLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ConvolutionLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ConvolutionLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::BaseConvolutionLayerTraitConst for core::Ptr<crate::dnn::ConvolutionLayer> {
		#[inline] fn as_raw_BaseConvolutionLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::BaseConvolutionLayerTrait for core::Ptr<crate::dnn::ConvolutionLayer> {
		#[inline] fn as_raw_mut_BaseConvolutionLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ConvolutionLayer>, core::Ptr<crate::dnn::BaseConvolutionLayer>, cv_PtrLcv_dnn_ConvolutionLayerG_to_PtrOfBaseConvolutionLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ConvolutionLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ConvolutionLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ConvolutionLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ConvolutionLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ConvolutionLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfConvolutionLayer")
				.field("kernel", &crate::dnn::BaseConvolutionLayerTraitConst::kernel(self))
				.field("stride", &crate::dnn::BaseConvolutionLayerTraitConst::stride(self))
				.field("pad", &crate::dnn::BaseConvolutionLayerTraitConst::pad(self))
				.field("dilation", &crate::dnn::BaseConvolutionLayerTraitConst::dilation(self))
				.field("adjust_pad", &crate::dnn::BaseConvolutionLayerTraitConst::adjust_pad(self))
				.field("adjust_pads", &crate::dnn::BaseConvolutionLayerTraitConst::adjust_pads(self))
				.field("kernel_size", &crate::dnn::BaseConvolutionLayerTraitConst::kernel_size(self))
				.field("strides", &crate::dnn::BaseConvolutionLayerTraitConst::strides(self))
				.field("dilations", &crate::dnn::BaseConvolutionLayerTraitConst::dilations(self))
				.field("pads_begin", &crate::dnn::BaseConvolutionLayerTraitConst::pads_begin(self))
				.field("pads_end", &crate::dnn::BaseConvolutionLayerTraitConst::pads_end(self))
				.field("pad_mode", &crate::dnn::BaseConvolutionLayerTraitConst::pad_mode(self))
				.field("num_output", &crate::dnn::BaseConvolutionLayerTraitConst::num_output(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::ConvolutionLayerInt8,
		cv_PtrLcv_dnn_ConvolutionLayerInt8G_new_null_const, cv_PtrLcv_dnn_ConvolutionLayerInt8G_delete, cv_PtrLcv_dnn_ConvolutionLayerInt8G_getInnerPtr_const, cv_PtrLcv_dnn_ConvolutionLayerInt8G_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ConvolutionLayerInt8, cv_PtrLcv_dnn_ConvolutionLayerInt8G_new_const_ConvolutionLayerInt8 }
	impl core::Ptr<crate::dnn::ConvolutionLayerInt8> {
		#[inline] pub fn as_raw_PtrOfConvolutionLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfConvolutionLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ConvolutionLayerInt8TraitConst for core::Ptr<crate::dnn::ConvolutionLayerInt8> {
		#[inline] fn as_raw_ConvolutionLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ConvolutionLayerInt8Trait for core::Ptr<crate::dnn::ConvolutionLayerInt8> {
		#[inline] fn as_raw_mut_ConvolutionLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ConvolutionLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ConvolutionLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ConvolutionLayerInt8>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ConvolutionLayerInt8G_to_PtrOfAlgorithm }

	impl crate::dnn::BaseConvolutionLayerTraitConst for core::Ptr<crate::dnn::ConvolutionLayerInt8> {
		#[inline] fn as_raw_BaseConvolutionLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::BaseConvolutionLayerTrait for core::Ptr<crate::dnn::ConvolutionLayerInt8> {
		#[inline] fn as_raw_mut_BaseConvolutionLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ConvolutionLayerInt8>, core::Ptr<crate::dnn::BaseConvolutionLayer>, cv_PtrLcv_dnn_ConvolutionLayerInt8G_to_PtrOfBaseConvolutionLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ConvolutionLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ConvolutionLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ConvolutionLayerInt8>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ConvolutionLayerInt8G_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ConvolutionLayerInt8> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfConvolutionLayerInt8")
				.field("input_zp", &crate::dnn::ConvolutionLayerInt8TraitConst::input_zp(self))
				.field("output_zp", &crate::dnn::ConvolutionLayerInt8TraitConst::output_zp(self))
				.field("output_sc", &crate::dnn::ConvolutionLayerInt8TraitConst::output_sc(self))
				.field("kernel", &crate::dnn::BaseConvolutionLayerTraitConst::kernel(self))
				.field("stride", &crate::dnn::BaseConvolutionLayerTraitConst::stride(self))
				.field("pad", &crate::dnn::BaseConvolutionLayerTraitConst::pad(self))
				.field("dilation", &crate::dnn::BaseConvolutionLayerTraitConst::dilation(self))
				.field("adjust_pad", &crate::dnn::BaseConvolutionLayerTraitConst::adjust_pad(self))
				.field("adjust_pads", &crate::dnn::BaseConvolutionLayerTraitConst::adjust_pads(self))
				.field("kernel_size", &crate::dnn::BaseConvolutionLayerTraitConst::kernel_size(self))
				.field("strides", &crate::dnn::BaseConvolutionLayerTraitConst::strides(self))
				.field("dilations", &crate::dnn::BaseConvolutionLayerTraitConst::dilations(self))
				.field("pads_begin", &crate::dnn::BaseConvolutionLayerTraitConst::pads_begin(self))
				.field("pads_end", &crate::dnn::BaseConvolutionLayerTraitConst::pads_end(self))
				.field("pad_mode", &crate::dnn::BaseConvolutionLayerTraitConst::pad_mode(self))
				.field("num_output", &crate::dnn::BaseConvolutionLayerTraitConst::num_output(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::CorrelationLayer,
		cv_PtrLcv_dnn_CorrelationLayerG_new_null_const, cv_PtrLcv_dnn_CorrelationLayerG_delete, cv_PtrLcv_dnn_CorrelationLayerG_getInnerPtr_const, cv_PtrLcv_dnn_CorrelationLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::CorrelationLayer, cv_PtrLcv_dnn_CorrelationLayerG_new_const_CorrelationLayer }
	impl core::Ptr<crate::dnn::CorrelationLayer> {
		#[inline] pub fn as_raw_PtrOfCorrelationLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCorrelationLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::CorrelationLayerTraitConst for core::Ptr<crate::dnn::CorrelationLayer> {
		#[inline] fn as_raw_CorrelationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::CorrelationLayerTrait for core::Ptr<crate::dnn::CorrelationLayer> {
		#[inline] fn as_raw_mut_CorrelationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::CorrelationLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::CorrelationLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::CorrelationLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_CorrelationLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::CorrelationLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::CorrelationLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::CorrelationLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_CorrelationLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::CorrelationLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfCorrelationLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::CropAndResizeLayer,
		cv_PtrLcv_dnn_CropAndResizeLayerG_new_null_const, cv_PtrLcv_dnn_CropAndResizeLayerG_delete, cv_PtrLcv_dnn_CropAndResizeLayerG_getInnerPtr_const, cv_PtrLcv_dnn_CropAndResizeLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::CropAndResizeLayer, cv_PtrLcv_dnn_CropAndResizeLayerG_new_const_CropAndResizeLayer }
	impl core::Ptr<crate::dnn::CropAndResizeLayer> {
		#[inline] pub fn as_raw_PtrOfCropAndResizeLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCropAndResizeLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::CropAndResizeLayerTraitConst for core::Ptr<crate::dnn::CropAndResizeLayer> {
		#[inline] fn as_raw_CropAndResizeLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::CropAndResizeLayerTrait for core::Ptr<crate::dnn::CropAndResizeLayer> {
		#[inline] fn as_raw_mut_CropAndResizeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::CropAndResizeLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::CropAndResizeLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::CropAndResizeLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_CropAndResizeLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::CropAndResizeLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::CropAndResizeLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::CropAndResizeLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_CropAndResizeLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::CropAndResizeLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfCropAndResizeLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::CropLayer,
		cv_PtrLcv_dnn_CropLayerG_new_null_const, cv_PtrLcv_dnn_CropLayerG_delete, cv_PtrLcv_dnn_CropLayerG_getInnerPtr_const, cv_PtrLcv_dnn_CropLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::CropLayer, cv_PtrLcv_dnn_CropLayerG_new_const_CropLayer }
	impl core::Ptr<crate::dnn::CropLayer> {
		#[inline] pub fn as_raw_PtrOfCropLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCropLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::CropLayerTraitConst for core::Ptr<crate::dnn::CropLayer> {
		#[inline] fn as_raw_CropLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::CropLayerTrait for core::Ptr<crate::dnn::CropLayer> {
		#[inline] fn as_raw_mut_CropLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::CropLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::CropLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::CropLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_CropLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::CropLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::CropLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::CropLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_CropLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::CropLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfCropLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::CumSumLayer,
		cv_PtrLcv_dnn_CumSumLayerG_new_null_const, cv_PtrLcv_dnn_CumSumLayerG_delete, cv_PtrLcv_dnn_CumSumLayerG_getInnerPtr_const, cv_PtrLcv_dnn_CumSumLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::CumSumLayer, cv_PtrLcv_dnn_CumSumLayerG_new_const_CumSumLayer }
	impl core::Ptr<crate::dnn::CumSumLayer> {
		#[inline] pub fn as_raw_PtrOfCumSumLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCumSumLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::CumSumLayerTraitConst for core::Ptr<crate::dnn::CumSumLayer> {
		#[inline] fn as_raw_CumSumLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::CumSumLayerTrait for core::Ptr<crate::dnn::CumSumLayer> {
		#[inline] fn as_raw_mut_CumSumLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::CumSumLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::CumSumLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::CumSumLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_CumSumLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::CumSumLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::CumSumLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::CumSumLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_CumSumLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::CumSumLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfCumSumLayer")
				.field("exclusive", &crate::dnn::CumSumLayerTraitConst::exclusive(self))
				.field("reverse", &crate::dnn::CumSumLayerTraitConst::reverse(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::DataAugmentationLayer,
		cv_PtrLcv_dnn_DataAugmentationLayerG_new_null_const, cv_PtrLcv_dnn_DataAugmentationLayerG_delete, cv_PtrLcv_dnn_DataAugmentationLayerG_getInnerPtr_const, cv_PtrLcv_dnn_DataAugmentationLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::DataAugmentationLayer, cv_PtrLcv_dnn_DataAugmentationLayerG_new_const_DataAugmentationLayer }
	impl core::Ptr<crate::dnn::DataAugmentationLayer> {
		#[inline] pub fn as_raw_PtrOfDataAugmentationLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDataAugmentationLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::DataAugmentationLayerTraitConst for core::Ptr<crate::dnn::DataAugmentationLayer> {
		#[inline] fn as_raw_DataAugmentationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::DataAugmentationLayerTrait for core::Ptr<crate::dnn::DataAugmentationLayer> {
		#[inline] fn as_raw_mut_DataAugmentationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::DataAugmentationLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::DataAugmentationLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::DataAugmentationLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_DataAugmentationLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::DataAugmentationLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::DataAugmentationLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::DataAugmentationLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_DataAugmentationLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::DataAugmentationLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfDataAugmentationLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::DeconvolutionLayer,
		cv_PtrLcv_dnn_DeconvolutionLayerG_new_null_const, cv_PtrLcv_dnn_DeconvolutionLayerG_delete, cv_PtrLcv_dnn_DeconvolutionLayerG_getInnerPtr_const, cv_PtrLcv_dnn_DeconvolutionLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::DeconvolutionLayer, cv_PtrLcv_dnn_DeconvolutionLayerG_new_const_DeconvolutionLayer }
	impl core::Ptr<crate::dnn::DeconvolutionLayer> {
		#[inline] pub fn as_raw_PtrOfDeconvolutionLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDeconvolutionLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::DeconvolutionLayerTraitConst for core::Ptr<crate::dnn::DeconvolutionLayer> {
		#[inline] fn as_raw_DeconvolutionLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::DeconvolutionLayerTrait for core::Ptr<crate::dnn::DeconvolutionLayer> {
		#[inline] fn as_raw_mut_DeconvolutionLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::DeconvolutionLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::DeconvolutionLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::DeconvolutionLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_DeconvolutionLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::BaseConvolutionLayerTraitConst for core::Ptr<crate::dnn::DeconvolutionLayer> {
		#[inline] fn as_raw_BaseConvolutionLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::BaseConvolutionLayerTrait for core::Ptr<crate::dnn::DeconvolutionLayer> {
		#[inline] fn as_raw_mut_BaseConvolutionLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::DeconvolutionLayer>, core::Ptr<crate::dnn::BaseConvolutionLayer>, cv_PtrLcv_dnn_DeconvolutionLayerG_to_PtrOfBaseConvolutionLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::DeconvolutionLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::DeconvolutionLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::DeconvolutionLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_DeconvolutionLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::DeconvolutionLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfDeconvolutionLayer")
				.field("kernel", &crate::dnn::BaseConvolutionLayerTraitConst::kernel(self))
				.field("stride", &crate::dnn::BaseConvolutionLayerTraitConst::stride(self))
				.field("pad", &crate::dnn::BaseConvolutionLayerTraitConst::pad(self))
				.field("dilation", &crate::dnn::BaseConvolutionLayerTraitConst::dilation(self))
				.field("adjust_pad", &crate::dnn::BaseConvolutionLayerTraitConst::adjust_pad(self))
				.field("adjust_pads", &crate::dnn::BaseConvolutionLayerTraitConst::adjust_pads(self))
				.field("kernel_size", &crate::dnn::BaseConvolutionLayerTraitConst::kernel_size(self))
				.field("strides", &crate::dnn::BaseConvolutionLayerTraitConst::strides(self))
				.field("dilations", &crate::dnn::BaseConvolutionLayerTraitConst::dilations(self))
				.field("pads_begin", &crate::dnn::BaseConvolutionLayerTraitConst::pads_begin(self))
				.field("pads_end", &crate::dnn::BaseConvolutionLayerTraitConst::pads_end(self))
				.field("pad_mode", &crate::dnn::BaseConvolutionLayerTraitConst::pad_mode(self))
				.field("num_output", &crate::dnn::BaseConvolutionLayerTraitConst::num_output(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::DequantizeLayer,
		cv_PtrLcv_dnn_DequantizeLayerG_new_null_const, cv_PtrLcv_dnn_DequantizeLayerG_delete, cv_PtrLcv_dnn_DequantizeLayerG_getInnerPtr_const, cv_PtrLcv_dnn_DequantizeLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::DequantizeLayer, cv_PtrLcv_dnn_DequantizeLayerG_new_const_DequantizeLayer }
	impl core::Ptr<crate::dnn::DequantizeLayer> {
		#[inline] pub fn as_raw_PtrOfDequantizeLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDequantizeLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::DequantizeLayerTraitConst for core::Ptr<crate::dnn::DequantizeLayer> {
		#[inline] fn as_raw_DequantizeLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::DequantizeLayerTrait for core::Ptr<crate::dnn::DequantizeLayer> {
		#[inline] fn as_raw_mut_DequantizeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::DequantizeLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::DequantizeLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::DequantizeLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_DequantizeLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::DequantizeLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::DequantizeLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::DequantizeLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_DequantizeLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::DequantizeLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfDequantizeLayer")
				.field("scale", &crate::dnn::DequantizeLayerTraitConst::scale(self))
				.field("zeropoint", &crate::dnn::DequantizeLayerTraitConst::zeropoint(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::DetectionOutputLayer,
		cv_PtrLcv_dnn_DetectionOutputLayerG_new_null_const, cv_PtrLcv_dnn_DetectionOutputLayerG_delete, cv_PtrLcv_dnn_DetectionOutputLayerG_getInnerPtr_const, cv_PtrLcv_dnn_DetectionOutputLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::DetectionOutputLayer, cv_PtrLcv_dnn_DetectionOutputLayerG_new_const_DetectionOutputLayer }
	impl core::Ptr<crate::dnn::DetectionOutputLayer> {
		#[inline] pub fn as_raw_PtrOfDetectionOutputLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetectionOutputLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::DetectionOutputLayerTraitConst for core::Ptr<crate::dnn::DetectionOutputLayer> {
		#[inline] fn as_raw_DetectionOutputLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::DetectionOutputLayerTrait for core::Ptr<crate::dnn::DetectionOutputLayer> {
		#[inline] fn as_raw_mut_DetectionOutputLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::DetectionOutputLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::DetectionOutputLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::DetectionOutputLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_DetectionOutputLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::DetectionOutputLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::DetectionOutputLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::DetectionOutputLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_DetectionOutputLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::DetectionOutputLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfDetectionOutputLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::ELULayer,
		cv_PtrLcv_dnn_ELULayerG_new_null_const, cv_PtrLcv_dnn_ELULayerG_delete, cv_PtrLcv_dnn_ELULayerG_getInnerPtr_const, cv_PtrLcv_dnn_ELULayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ELULayer, cv_PtrLcv_dnn_ELULayerG_new_const_ELULayer }
	impl core::Ptr<crate::dnn::ELULayer> {
		#[inline] pub fn as_raw_PtrOfELULayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfELULayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ELULayerTraitConst for core::Ptr<crate::dnn::ELULayer> {
		#[inline] fn as_raw_ELULayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ELULayerTrait for core::Ptr<crate::dnn::ELULayer> {
		#[inline] fn as_raw_mut_ELULayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ELULayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ELULayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ELULayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ELULayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ELULayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ELULayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ELULayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_ELULayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ELULayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ELULayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ELULayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ELULayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ELULayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfELULayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::EltwiseLayer,
		cv_PtrLcv_dnn_EltwiseLayerG_new_null_const, cv_PtrLcv_dnn_EltwiseLayerG_delete, cv_PtrLcv_dnn_EltwiseLayerG_getInnerPtr_const, cv_PtrLcv_dnn_EltwiseLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::EltwiseLayer, cv_PtrLcv_dnn_EltwiseLayerG_new_const_EltwiseLayer }
	impl core::Ptr<crate::dnn::EltwiseLayer> {
		#[inline] pub fn as_raw_PtrOfEltwiseLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEltwiseLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::EltwiseLayerTraitConst for core::Ptr<crate::dnn::EltwiseLayer> {
		#[inline] fn as_raw_EltwiseLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::EltwiseLayerTrait for core::Ptr<crate::dnn::EltwiseLayer> {
		#[inline] fn as_raw_mut_EltwiseLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::EltwiseLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::EltwiseLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::EltwiseLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_EltwiseLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::EltwiseLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::EltwiseLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::EltwiseLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_EltwiseLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::EltwiseLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfEltwiseLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::EltwiseLayerInt8,
		cv_PtrLcv_dnn_EltwiseLayerInt8G_new_null_const, cv_PtrLcv_dnn_EltwiseLayerInt8G_delete, cv_PtrLcv_dnn_EltwiseLayerInt8G_getInnerPtr_const, cv_PtrLcv_dnn_EltwiseLayerInt8G_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::EltwiseLayerInt8, cv_PtrLcv_dnn_EltwiseLayerInt8G_new_const_EltwiseLayerInt8 }
	impl core::Ptr<crate::dnn::EltwiseLayerInt8> {
		#[inline] pub fn as_raw_PtrOfEltwiseLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEltwiseLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::EltwiseLayerInt8TraitConst for core::Ptr<crate::dnn::EltwiseLayerInt8> {
		#[inline] fn as_raw_EltwiseLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::EltwiseLayerInt8Trait for core::Ptr<crate::dnn::EltwiseLayerInt8> {
		#[inline] fn as_raw_mut_EltwiseLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::EltwiseLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::EltwiseLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::EltwiseLayerInt8>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_EltwiseLayerInt8G_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::EltwiseLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::EltwiseLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::EltwiseLayerInt8>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_EltwiseLayerInt8G_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::EltwiseLayerInt8> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfEltwiseLayerInt8")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::ExpLayer,
		cv_PtrLcv_dnn_ExpLayerG_new_null_const, cv_PtrLcv_dnn_ExpLayerG_delete, cv_PtrLcv_dnn_ExpLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ExpLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ExpLayer, cv_PtrLcv_dnn_ExpLayerG_new_const_ExpLayer }
	impl core::Ptr<crate::dnn::ExpLayer> {
		#[inline] pub fn as_raw_PtrOfExpLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfExpLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ExpLayerTraitConst for core::Ptr<crate::dnn::ExpLayer> {
		#[inline] fn as_raw_ExpLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ExpLayerTrait for core::Ptr<crate::dnn::ExpLayer> {
		#[inline] fn as_raw_mut_ExpLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ExpLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ExpLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ExpLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ExpLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ExpLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ExpLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ExpLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_ExpLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ExpLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ExpLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ExpLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ExpLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ExpLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfExpLayer")
				.field("base", &crate::dnn::ExpLayerTraitConst::base(self))
				.field("scale", &crate::dnn::ExpLayerTraitConst::scale(self))
				.field("shift", &crate::dnn::ExpLayerTraitConst::shift(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::FlattenLayer,
		cv_PtrLcv_dnn_FlattenLayerG_new_null_const, cv_PtrLcv_dnn_FlattenLayerG_delete, cv_PtrLcv_dnn_FlattenLayerG_getInnerPtr_const, cv_PtrLcv_dnn_FlattenLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::FlattenLayer, cv_PtrLcv_dnn_FlattenLayerG_new_const_FlattenLayer }
	impl core::Ptr<crate::dnn::FlattenLayer> {
		#[inline] pub fn as_raw_PtrOfFlattenLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFlattenLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::FlattenLayerTraitConst for core::Ptr<crate::dnn::FlattenLayer> {
		#[inline] fn as_raw_FlattenLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::FlattenLayerTrait for core::Ptr<crate::dnn::FlattenLayer> {
		#[inline] fn as_raw_mut_FlattenLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::FlattenLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::FlattenLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::FlattenLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_FlattenLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::FlattenLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::FlattenLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::FlattenLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_FlattenLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::FlattenLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfFlattenLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::FlowWarpLayer,
		cv_PtrLcv_dnn_FlowWarpLayerG_new_null_const, cv_PtrLcv_dnn_FlowWarpLayerG_delete, cv_PtrLcv_dnn_FlowWarpLayerG_getInnerPtr_const, cv_PtrLcv_dnn_FlowWarpLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::FlowWarpLayer, cv_PtrLcv_dnn_FlowWarpLayerG_new_const_FlowWarpLayer }
	impl core::Ptr<crate::dnn::FlowWarpLayer> {
		#[inline] pub fn as_raw_PtrOfFlowWarpLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFlowWarpLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::FlowWarpLayerTraitConst for core::Ptr<crate::dnn::FlowWarpLayer> {
		#[inline] fn as_raw_FlowWarpLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::FlowWarpLayerTrait for core::Ptr<crate::dnn::FlowWarpLayer> {
		#[inline] fn as_raw_mut_FlowWarpLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::FlowWarpLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::FlowWarpLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::FlowWarpLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_FlowWarpLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::FlowWarpLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::FlowWarpLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::FlowWarpLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_FlowWarpLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::FlowWarpLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfFlowWarpLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::GRULayer,
		cv_PtrLcv_dnn_GRULayerG_new_null_const, cv_PtrLcv_dnn_GRULayerG_delete, cv_PtrLcv_dnn_GRULayerG_getInnerPtr_const, cv_PtrLcv_dnn_GRULayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::GRULayer, cv_PtrLcv_dnn_GRULayerG_new_const_GRULayer }
	impl core::Ptr<crate::dnn::GRULayer> {
		#[inline] pub fn as_raw_PtrOfGRULayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGRULayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::GRULayerTraitConst for core::Ptr<crate::dnn::GRULayer> {
		#[inline] fn as_raw_GRULayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::GRULayerTrait for core::Ptr<crate::dnn::GRULayer> {
		#[inline] fn as_raw_mut_GRULayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::GRULayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::GRULayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::GRULayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_GRULayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::GRULayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::GRULayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::GRULayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_GRULayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::GRULayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfGRULayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::InnerProductLayer,
		cv_PtrLcv_dnn_InnerProductLayerG_new_null_const, cv_PtrLcv_dnn_InnerProductLayerG_delete, cv_PtrLcv_dnn_InnerProductLayerG_getInnerPtr_const, cv_PtrLcv_dnn_InnerProductLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::InnerProductLayer, cv_PtrLcv_dnn_InnerProductLayerG_new_const_InnerProductLayer }
	impl core::Ptr<crate::dnn::InnerProductLayer> {
		#[inline] pub fn as_raw_PtrOfInnerProductLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfInnerProductLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::InnerProductLayerTraitConst for core::Ptr<crate::dnn::InnerProductLayer> {
		#[inline] fn as_raw_InnerProductLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::InnerProductLayerTrait for core::Ptr<crate::dnn::InnerProductLayer> {
		#[inline] fn as_raw_mut_InnerProductLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::InnerProductLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::InnerProductLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::InnerProductLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_InnerProductLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::InnerProductLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::InnerProductLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::InnerProductLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_InnerProductLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::InnerProductLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfInnerProductLayer")
				.field("axis", &crate::dnn::InnerProductLayerTraitConst::axis(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::InnerProductLayerInt8,
		cv_PtrLcv_dnn_InnerProductLayerInt8G_new_null_const, cv_PtrLcv_dnn_InnerProductLayerInt8G_delete, cv_PtrLcv_dnn_InnerProductLayerInt8G_getInnerPtr_const, cv_PtrLcv_dnn_InnerProductLayerInt8G_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::InnerProductLayerInt8, cv_PtrLcv_dnn_InnerProductLayerInt8G_new_const_InnerProductLayerInt8 }
	impl core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] pub fn as_raw_PtrOfInnerProductLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfInnerProductLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::InnerProductLayerInt8TraitConst for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] fn as_raw_InnerProductLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::InnerProductLayerInt8Trait for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] fn as_raw_mut_InnerProductLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::InnerProductLayerInt8>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_InnerProductLayerInt8G_to_PtrOfAlgorithm }

	impl crate::dnn::InnerProductLayerTraitConst for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] fn as_raw_InnerProductLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::InnerProductLayerTrait for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] fn as_raw_mut_InnerProductLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::InnerProductLayerInt8>, core::Ptr<crate::dnn::InnerProductLayer>, cv_PtrLcv_dnn_InnerProductLayerInt8G_to_PtrOfInnerProductLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::InnerProductLayerInt8>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_InnerProductLayerInt8G_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfInnerProductLayerInt8")
				.field("output_zp", &crate::dnn::InnerProductLayerInt8TraitConst::output_zp(self))
				.field("axis", &crate::dnn::InnerProductLayerTraitConst::axis(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::InterpLayer,
		cv_PtrLcv_dnn_InterpLayerG_new_null_const, cv_PtrLcv_dnn_InterpLayerG_delete, cv_PtrLcv_dnn_InterpLayerG_getInnerPtr_const, cv_PtrLcv_dnn_InterpLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::InterpLayer, cv_PtrLcv_dnn_InterpLayerG_new_const_InterpLayer }
	impl core::Ptr<crate::dnn::InterpLayer> {
		#[inline] pub fn as_raw_PtrOfInterpLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfInterpLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::InterpLayerTraitConst for core::Ptr<crate::dnn::InterpLayer> {
		#[inline] fn as_raw_InterpLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::InterpLayerTrait for core::Ptr<crate::dnn::InterpLayer> {
		#[inline] fn as_raw_mut_InterpLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::InterpLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::InterpLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::InterpLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_InterpLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::InterpLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::InterpLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::InterpLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_InterpLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::InterpLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfInterpLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::LRNLayer,
		cv_PtrLcv_dnn_LRNLayerG_new_null_const, cv_PtrLcv_dnn_LRNLayerG_delete, cv_PtrLcv_dnn_LRNLayerG_getInnerPtr_const, cv_PtrLcv_dnn_LRNLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::LRNLayer, cv_PtrLcv_dnn_LRNLayerG_new_const_LRNLayer }
	impl core::Ptr<crate::dnn::LRNLayer> {
		#[inline] pub fn as_raw_PtrOfLRNLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLRNLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::LRNLayerTraitConst for core::Ptr<crate::dnn::LRNLayer> {
		#[inline] fn as_raw_LRNLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LRNLayerTrait for core::Ptr<crate::dnn::LRNLayer> {
		#[inline] fn as_raw_mut_LRNLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::LRNLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::LRNLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::LRNLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_LRNLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::LRNLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::LRNLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::LRNLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_LRNLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::LRNLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfLRNLayer")
				.field("typ", &crate::dnn::LRNLayerTraitConst::typ(self))
				.field("size", &crate::dnn::LRNLayerTraitConst::size(self))
				.field("alpha", &crate::dnn::LRNLayerTraitConst::alpha(self))
				.field("beta", &crate::dnn::LRNLayerTraitConst::beta(self))
				.field("bias", &crate::dnn::LRNLayerTraitConst::bias(self))
				.field("norm_by_size", &crate::dnn::LRNLayerTraitConst::norm_by_size(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::LSTMLayer,
		cv_PtrLcv_dnn_LSTMLayerG_new_null_const, cv_PtrLcv_dnn_LSTMLayerG_delete, cv_PtrLcv_dnn_LSTMLayerG_getInnerPtr_const, cv_PtrLcv_dnn_LSTMLayerG_getInnerPtrMut
	}

	impl core::Ptr<crate::dnn::LSTMLayer> {
		#[inline] pub fn as_raw_PtrOfLSTMLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLSTMLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::LSTMLayerTraitConst for core::Ptr<crate::dnn::LSTMLayer> {
		#[inline] fn as_raw_LSTMLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LSTMLayerTrait for core::Ptr<crate::dnn::LSTMLayer> {
		#[inline] fn as_raw_mut_LSTMLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::LSTMLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::LSTMLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::LSTMLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_LSTMLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::LSTMLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::LSTMLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::LSTMLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_LSTMLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::LSTMLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfLSTMLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::Layer,
		cv_PtrLcv_dnn_LayerG_new_null_const, cv_PtrLcv_dnn_LayerG_delete, cv_PtrLcv_dnn_LayerG_getInnerPtr_const, cv_PtrLcv_dnn_LayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::Layer, cv_PtrLcv_dnn_LayerG_new_const_Layer }
	impl core::Ptr<crate::dnn::Layer> {
		#[inline] pub fn as_raw_PtrOfLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::Layer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::Layer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::Layer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::Layer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::Layer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_LayerG_to_PtrOfAlgorithm }

	impl std::fmt::Debug for core::Ptr<crate::dnn::Layer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::MVNLayer,
		cv_PtrLcv_dnn_MVNLayerG_new_null_const, cv_PtrLcv_dnn_MVNLayerG_delete, cv_PtrLcv_dnn_MVNLayerG_getInnerPtr_const, cv_PtrLcv_dnn_MVNLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::MVNLayer, cv_PtrLcv_dnn_MVNLayerG_new_const_MVNLayer }
	impl core::Ptr<crate::dnn::MVNLayer> {
		#[inline] pub fn as_raw_PtrOfMVNLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMVNLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::MVNLayerTraitConst for core::Ptr<crate::dnn::MVNLayer> {
		#[inline] fn as_raw_MVNLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::MVNLayerTrait for core::Ptr<crate::dnn::MVNLayer> {
		#[inline] fn as_raw_mut_MVNLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::MVNLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::MVNLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::MVNLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_MVNLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::MVNLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::MVNLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::MVNLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_MVNLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::MVNLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfMVNLayer")
				.field("eps", &crate::dnn::MVNLayerTraitConst::eps(self))
				.field("norm_variance", &crate::dnn::MVNLayerTraitConst::norm_variance(self))
				.field("across_channels", &crate::dnn::MVNLayerTraitConst::across_channels(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::MaxUnpoolLayer,
		cv_PtrLcv_dnn_MaxUnpoolLayerG_new_null_const, cv_PtrLcv_dnn_MaxUnpoolLayerG_delete, cv_PtrLcv_dnn_MaxUnpoolLayerG_getInnerPtr_const, cv_PtrLcv_dnn_MaxUnpoolLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::MaxUnpoolLayer, cv_PtrLcv_dnn_MaxUnpoolLayerG_new_const_MaxUnpoolLayer }
	impl core::Ptr<crate::dnn::MaxUnpoolLayer> {
		#[inline] pub fn as_raw_PtrOfMaxUnpoolLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMaxUnpoolLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::MaxUnpoolLayerTraitConst for core::Ptr<crate::dnn::MaxUnpoolLayer> {
		#[inline] fn as_raw_MaxUnpoolLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::MaxUnpoolLayerTrait for core::Ptr<crate::dnn::MaxUnpoolLayer> {
		#[inline] fn as_raw_mut_MaxUnpoolLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::MaxUnpoolLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::MaxUnpoolLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::MaxUnpoolLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_MaxUnpoolLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::MaxUnpoolLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::MaxUnpoolLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::MaxUnpoolLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_MaxUnpoolLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::MaxUnpoolLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfMaxUnpoolLayer")
				.field("pool_kernel", &crate::dnn::MaxUnpoolLayerTraitConst::pool_kernel(self))
				.field("pool_pad", &crate::dnn::MaxUnpoolLayerTraitConst::pool_pad(self))
				.field("pool_stride", &crate::dnn::MaxUnpoolLayerTraitConst::pool_stride(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::MishLayer,
		cv_PtrLcv_dnn_MishLayerG_new_null_const, cv_PtrLcv_dnn_MishLayerG_delete, cv_PtrLcv_dnn_MishLayerG_getInnerPtr_const, cv_PtrLcv_dnn_MishLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::MishLayer, cv_PtrLcv_dnn_MishLayerG_new_const_MishLayer }
	impl core::Ptr<crate::dnn::MishLayer> {
		#[inline] pub fn as_raw_PtrOfMishLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMishLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::MishLayerTraitConst for core::Ptr<crate::dnn::MishLayer> {
		#[inline] fn as_raw_MishLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::MishLayerTrait for core::Ptr<crate::dnn::MishLayer> {
		#[inline] fn as_raw_mut_MishLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::MishLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::MishLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::MishLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_MishLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::MishLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::MishLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::MishLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_MishLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::MishLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::MishLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::MishLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_MishLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::MishLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfMishLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::NormalizeBBoxLayer,
		cv_PtrLcv_dnn_NormalizeBBoxLayerG_new_null_const, cv_PtrLcv_dnn_NormalizeBBoxLayerG_delete, cv_PtrLcv_dnn_NormalizeBBoxLayerG_getInnerPtr_const, cv_PtrLcv_dnn_NormalizeBBoxLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::NormalizeBBoxLayer, cv_PtrLcv_dnn_NormalizeBBoxLayerG_new_const_NormalizeBBoxLayer }
	impl core::Ptr<crate::dnn::NormalizeBBoxLayer> {
		#[inline] pub fn as_raw_PtrOfNormalizeBBoxLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfNormalizeBBoxLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::NormalizeBBoxLayerTraitConst for core::Ptr<crate::dnn::NormalizeBBoxLayer> {
		#[inline] fn as_raw_NormalizeBBoxLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::NormalizeBBoxLayerTrait for core::Ptr<crate::dnn::NormalizeBBoxLayer> {
		#[inline] fn as_raw_mut_NormalizeBBoxLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::NormalizeBBoxLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::NormalizeBBoxLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::NormalizeBBoxLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_NormalizeBBoxLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::NormalizeBBoxLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::NormalizeBBoxLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::NormalizeBBoxLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_NormalizeBBoxLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::NormalizeBBoxLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfNormalizeBBoxLayer")
				.field("pnorm", &crate::dnn::NormalizeBBoxLayerTraitConst::pnorm(self))
				.field("epsilon", &crate::dnn::NormalizeBBoxLayerTraitConst::epsilon(self))
				.field("across_spatial", &crate::dnn::NormalizeBBoxLayerTraitConst::across_spatial(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::PaddingLayer,
		cv_PtrLcv_dnn_PaddingLayerG_new_null_const, cv_PtrLcv_dnn_PaddingLayerG_delete, cv_PtrLcv_dnn_PaddingLayerG_getInnerPtr_const, cv_PtrLcv_dnn_PaddingLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::PaddingLayer, cv_PtrLcv_dnn_PaddingLayerG_new_const_PaddingLayer }
	impl core::Ptr<crate::dnn::PaddingLayer> {
		#[inline] pub fn as_raw_PtrOfPaddingLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPaddingLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::PaddingLayerTraitConst for core::Ptr<crate::dnn::PaddingLayer> {
		#[inline] fn as_raw_PaddingLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::PaddingLayerTrait for core::Ptr<crate::dnn::PaddingLayer> {
		#[inline] fn as_raw_mut_PaddingLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::PaddingLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::PaddingLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::PaddingLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_PaddingLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::PaddingLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::PaddingLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::PaddingLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_PaddingLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::PaddingLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfPaddingLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::PermuteLayer,
		cv_PtrLcv_dnn_PermuteLayerG_new_null_const, cv_PtrLcv_dnn_PermuteLayerG_delete, cv_PtrLcv_dnn_PermuteLayerG_getInnerPtr_const, cv_PtrLcv_dnn_PermuteLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::PermuteLayer, cv_PtrLcv_dnn_PermuteLayerG_new_const_PermuteLayer }
	impl core::Ptr<crate::dnn::PermuteLayer> {
		#[inline] pub fn as_raw_PtrOfPermuteLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPermuteLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::PermuteLayerTraitConst for core::Ptr<crate::dnn::PermuteLayer> {
		#[inline] fn as_raw_PermuteLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::PermuteLayerTrait for core::Ptr<crate::dnn::PermuteLayer> {
		#[inline] fn as_raw_mut_PermuteLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::PermuteLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::PermuteLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::PermuteLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_PermuteLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::PermuteLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::PermuteLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::PermuteLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_PermuteLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::PermuteLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfPermuteLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::PoolingLayer,
		cv_PtrLcv_dnn_PoolingLayerG_new_null_const, cv_PtrLcv_dnn_PoolingLayerG_delete, cv_PtrLcv_dnn_PoolingLayerG_getInnerPtr_const, cv_PtrLcv_dnn_PoolingLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::PoolingLayer, cv_PtrLcv_dnn_PoolingLayerG_new_const_PoolingLayer }
	impl core::Ptr<crate::dnn::PoolingLayer> {
		#[inline] pub fn as_raw_PtrOfPoolingLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPoolingLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::PoolingLayerTraitConst for core::Ptr<crate::dnn::PoolingLayer> {
		#[inline] fn as_raw_PoolingLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::PoolingLayerTrait for core::Ptr<crate::dnn::PoolingLayer> {
		#[inline] fn as_raw_mut_PoolingLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::PoolingLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::PoolingLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::PoolingLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_PoolingLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::PoolingLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::PoolingLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::PoolingLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_PoolingLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::PoolingLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfPoolingLayer")
				.field("typ", &crate::dnn::PoolingLayerTraitConst::typ(self))
				.field("kernel_size", &crate::dnn::PoolingLayerTraitConst::kernel_size(self))
				.field("strides", &crate::dnn::PoolingLayerTraitConst::strides(self))
				.field("pads_begin", &crate::dnn::PoolingLayerTraitConst::pads_begin(self))
				.field("pads_end", &crate::dnn::PoolingLayerTraitConst::pads_end(self))
				.field("global_pooling", &crate::dnn::PoolingLayerTraitConst::global_pooling(self))
				.field("is_global_pooling", &crate::dnn::PoolingLayerTraitConst::is_global_pooling(self))
				.field("compute_max_idx", &crate::dnn::PoolingLayerTraitConst::compute_max_idx(self))
				.field("pad_mode", &crate::dnn::PoolingLayerTraitConst::pad_mode(self))
				.field("ceil_mode", &crate::dnn::PoolingLayerTraitConst::ceil_mode(self))
				.field("ave_pool_padded_area", &crate::dnn::PoolingLayerTraitConst::ave_pool_padded_area(self))
				.field("pooled_size", &crate::dnn::PoolingLayerTraitConst::pooled_size(self))
				.field("spatial_scale", &crate::dnn::PoolingLayerTraitConst::spatial_scale(self))
				.field("ps_roi_out_channels", &crate::dnn::PoolingLayerTraitConst::ps_roi_out_channels(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::PoolingLayerInt8,
		cv_PtrLcv_dnn_PoolingLayerInt8G_new_null_const, cv_PtrLcv_dnn_PoolingLayerInt8G_delete, cv_PtrLcv_dnn_PoolingLayerInt8G_getInnerPtr_const, cv_PtrLcv_dnn_PoolingLayerInt8G_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::PoolingLayerInt8, cv_PtrLcv_dnn_PoolingLayerInt8G_new_const_PoolingLayerInt8 }
	impl core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] pub fn as_raw_PtrOfPoolingLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPoolingLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::PoolingLayerInt8TraitConst for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] fn as_raw_PoolingLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::PoolingLayerInt8Trait for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] fn as_raw_mut_PoolingLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::PoolingLayerInt8>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_PoolingLayerInt8G_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::PoolingLayerInt8>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_PoolingLayerInt8G_to_PtrOfLayer }

	impl crate::dnn::PoolingLayerTraitConst for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] fn as_raw_PoolingLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::PoolingLayerTrait for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] fn as_raw_mut_PoolingLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::PoolingLayerInt8>, core::Ptr<crate::dnn::PoolingLayer>, cv_PtrLcv_dnn_PoolingLayerInt8G_to_PtrOfPoolingLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfPoolingLayerInt8")
				.field("input_zp", &crate::dnn::PoolingLayerInt8TraitConst::input_zp(self))
				.field("output_zp", &crate::dnn::PoolingLayerInt8TraitConst::output_zp(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.field("typ", &crate::dnn::PoolingLayerTraitConst::typ(self))
				.field("kernel_size", &crate::dnn::PoolingLayerTraitConst::kernel_size(self))
				.field("strides", &crate::dnn::PoolingLayerTraitConst::strides(self))
				.field("pads_begin", &crate::dnn::PoolingLayerTraitConst::pads_begin(self))
				.field("pads_end", &crate::dnn::PoolingLayerTraitConst::pads_end(self))
				.field("global_pooling", &crate::dnn::PoolingLayerTraitConst::global_pooling(self))
				.field("is_global_pooling", &crate::dnn::PoolingLayerTraitConst::is_global_pooling(self))
				.field("compute_max_idx", &crate::dnn::PoolingLayerTraitConst::compute_max_idx(self))
				.field("pad_mode", &crate::dnn::PoolingLayerTraitConst::pad_mode(self))
				.field("ceil_mode", &crate::dnn::PoolingLayerTraitConst::ceil_mode(self))
				.field("ave_pool_padded_area", &crate::dnn::PoolingLayerTraitConst::ave_pool_padded_area(self))
				.field("pooled_size", &crate::dnn::PoolingLayerTraitConst::pooled_size(self))
				.field("spatial_scale", &crate::dnn::PoolingLayerTraitConst::spatial_scale(self))
				.field("ps_roi_out_channels", &crate::dnn::PoolingLayerTraitConst::ps_roi_out_channels(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::PowerLayer,
		cv_PtrLcv_dnn_PowerLayerG_new_null_const, cv_PtrLcv_dnn_PowerLayerG_delete, cv_PtrLcv_dnn_PowerLayerG_getInnerPtr_const, cv_PtrLcv_dnn_PowerLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::PowerLayer, cv_PtrLcv_dnn_PowerLayerG_new_const_PowerLayer }
	impl core::Ptr<crate::dnn::PowerLayer> {
		#[inline] pub fn as_raw_PtrOfPowerLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPowerLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::PowerLayerTraitConst for core::Ptr<crate::dnn::PowerLayer> {
		#[inline] fn as_raw_PowerLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::PowerLayerTrait for core::Ptr<crate::dnn::PowerLayer> {
		#[inline] fn as_raw_mut_PowerLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::PowerLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::PowerLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::PowerLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_PowerLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::PowerLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::PowerLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::PowerLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_PowerLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::PowerLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::PowerLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::PowerLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_PowerLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::PowerLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfPowerLayer")
				.field("power", &crate::dnn::PowerLayerTraitConst::power(self))
				.field("scale", &crate::dnn::PowerLayerTraitConst::scale(self))
				.field("shift", &crate::dnn::PowerLayerTraitConst::shift(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::PriorBoxLayer,
		cv_PtrLcv_dnn_PriorBoxLayerG_new_null_const, cv_PtrLcv_dnn_PriorBoxLayerG_delete, cv_PtrLcv_dnn_PriorBoxLayerG_getInnerPtr_const, cv_PtrLcv_dnn_PriorBoxLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::PriorBoxLayer, cv_PtrLcv_dnn_PriorBoxLayerG_new_const_PriorBoxLayer }
	impl core::Ptr<crate::dnn::PriorBoxLayer> {
		#[inline] pub fn as_raw_PtrOfPriorBoxLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPriorBoxLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::PriorBoxLayerTraitConst for core::Ptr<crate::dnn::PriorBoxLayer> {
		#[inline] fn as_raw_PriorBoxLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::PriorBoxLayerTrait for core::Ptr<crate::dnn::PriorBoxLayer> {
		#[inline] fn as_raw_mut_PriorBoxLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::PriorBoxLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::PriorBoxLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::PriorBoxLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_PriorBoxLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::PriorBoxLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::PriorBoxLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::PriorBoxLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_PriorBoxLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::PriorBoxLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfPriorBoxLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::ProposalLayer,
		cv_PtrLcv_dnn_ProposalLayerG_new_null_const, cv_PtrLcv_dnn_ProposalLayerG_delete, cv_PtrLcv_dnn_ProposalLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ProposalLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ProposalLayer, cv_PtrLcv_dnn_ProposalLayerG_new_const_ProposalLayer }
	impl core::Ptr<crate::dnn::ProposalLayer> {
		#[inline] pub fn as_raw_PtrOfProposalLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfProposalLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ProposalLayerTraitConst for core::Ptr<crate::dnn::ProposalLayer> {
		#[inline] fn as_raw_ProposalLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ProposalLayerTrait for core::Ptr<crate::dnn::ProposalLayer> {
		#[inline] fn as_raw_mut_ProposalLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ProposalLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ProposalLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ProposalLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ProposalLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ProposalLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ProposalLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ProposalLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ProposalLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ProposalLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfProposalLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::QuantizeLayer,
		cv_PtrLcv_dnn_QuantizeLayerG_new_null_const, cv_PtrLcv_dnn_QuantizeLayerG_delete, cv_PtrLcv_dnn_QuantizeLayerG_getInnerPtr_const, cv_PtrLcv_dnn_QuantizeLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::QuantizeLayer, cv_PtrLcv_dnn_QuantizeLayerG_new_const_QuantizeLayer }
	impl core::Ptr<crate::dnn::QuantizeLayer> {
		#[inline] pub fn as_raw_PtrOfQuantizeLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfQuantizeLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::QuantizeLayerTraitConst for core::Ptr<crate::dnn::QuantizeLayer> {
		#[inline] fn as_raw_QuantizeLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::QuantizeLayerTrait for core::Ptr<crate::dnn::QuantizeLayer> {
		#[inline] fn as_raw_mut_QuantizeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::QuantizeLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::QuantizeLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::QuantizeLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_QuantizeLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::QuantizeLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::QuantizeLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::QuantizeLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_QuantizeLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::QuantizeLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfQuantizeLayer")
				.field("scale", &crate::dnn::QuantizeLayerTraitConst::scale(self))
				.field("zeropoint", &crate::dnn::QuantizeLayerTraitConst::zeropoint(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::RNNLayer,
		cv_PtrLcv_dnn_RNNLayerG_new_null_const, cv_PtrLcv_dnn_RNNLayerG_delete, cv_PtrLcv_dnn_RNNLayerG_getInnerPtr_const, cv_PtrLcv_dnn_RNNLayerG_getInnerPtrMut
	}

	impl core::Ptr<crate::dnn::RNNLayer> {
		#[inline] pub fn as_raw_PtrOfRNNLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRNNLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::RNNLayerTraitConst for core::Ptr<crate::dnn::RNNLayer> {
		#[inline] fn as_raw_RNNLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::RNNLayerTrait for core::Ptr<crate::dnn::RNNLayer> {
		#[inline] fn as_raw_mut_RNNLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::RNNLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::RNNLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::RNNLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_RNNLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::RNNLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::RNNLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::RNNLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_RNNLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::RNNLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfRNNLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::ReLU6Layer,
		cv_PtrLcv_dnn_ReLU6LayerG_new_null_const, cv_PtrLcv_dnn_ReLU6LayerG_delete, cv_PtrLcv_dnn_ReLU6LayerG_getInnerPtr_const, cv_PtrLcv_dnn_ReLU6LayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ReLU6Layer, cv_PtrLcv_dnn_ReLU6LayerG_new_const_ReLU6Layer }
	impl core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] pub fn as_raw_PtrOfReLU6Layer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReLU6Layer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ReLU6LayerTraitConst for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] fn as_raw_ReLU6Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ReLU6LayerTrait for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] fn as_raw_mut_ReLU6Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ReLU6Layer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ReLU6LayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ReLU6Layer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_ReLU6LayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ReLU6Layer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ReLU6LayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfReLU6Layer")
				.field("min_value", &crate::dnn::ReLU6LayerTraitConst::min_value(self))
				.field("max_value", &crate::dnn::ReLU6LayerTraitConst::max_value(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::ReLULayer,
		cv_PtrLcv_dnn_ReLULayerG_new_null_const, cv_PtrLcv_dnn_ReLULayerG_delete, cv_PtrLcv_dnn_ReLULayerG_getInnerPtr_const, cv_PtrLcv_dnn_ReLULayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ReLULayer, cv_PtrLcv_dnn_ReLULayerG_new_const_ReLULayer }
	impl core::Ptr<crate::dnn::ReLULayer> {
		#[inline] pub fn as_raw_PtrOfReLULayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReLULayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ReLULayerTraitConst for core::Ptr<crate::dnn::ReLULayer> {
		#[inline] fn as_raw_ReLULayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ReLULayerTrait for core::Ptr<crate::dnn::ReLULayer> {
		#[inline] fn as_raw_mut_ReLULayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ReLULayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ReLULayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ReLULayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ReLULayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ReLULayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ReLULayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ReLULayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_ReLULayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ReLULayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ReLULayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ReLULayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ReLULayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ReLULayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfReLULayer")
				.field("negative_slope", &crate::dnn::ReLULayerTraitConst::negative_slope(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::RegionLayer,
		cv_PtrLcv_dnn_RegionLayerG_new_null_const, cv_PtrLcv_dnn_RegionLayerG_delete, cv_PtrLcv_dnn_RegionLayerG_getInnerPtr_const, cv_PtrLcv_dnn_RegionLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::RegionLayer, cv_PtrLcv_dnn_RegionLayerG_new_const_RegionLayer }
	impl core::Ptr<crate::dnn::RegionLayer> {
		#[inline] pub fn as_raw_PtrOfRegionLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRegionLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::RegionLayerTraitConst for core::Ptr<crate::dnn::RegionLayer> {
		#[inline] fn as_raw_RegionLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::RegionLayerTrait for core::Ptr<crate::dnn::RegionLayer> {
		#[inline] fn as_raw_mut_RegionLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::RegionLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::RegionLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::RegionLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_RegionLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::RegionLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::RegionLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::RegionLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_RegionLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::RegionLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfRegionLayer")
				.field("nms_threshold", &crate::dnn::RegionLayerTraitConst::nms_threshold(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::ReorgLayer,
		cv_PtrLcv_dnn_ReorgLayerG_new_null_const, cv_PtrLcv_dnn_ReorgLayerG_delete, cv_PtrLcv_dnn_ReorgLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ReorgLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ReorgLayer, cv_PtrLcv_dnn_ReorgLayerG_new_const_ReorgLayer }
	impl core::Ptr<crate::dnn::ReorgLayer> {
		#[inline] pub fn as_raw_PtrOfReorgLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReorgLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ReorgLayerTraitConst for core::Ptr<crate::dnn::ReorgLayer> {
		#[inline] fn as_raw_ReorgLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ReorgLayerTrait for core::Ptr<crate::dnn::ReorgLayer> {
		#[inline] fn as_raw_mut_ReorgLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ReorgLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ReorgLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ReorgLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ReorgLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ReorgLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ReorgLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ReorgLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ReorgLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ReorgLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfReorgLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::RequantizeLayer,
		cv_PtrLcv_dnn_RequantizeLayerG_new_null_const, cv_PtrLcv_dnn_RequantizeLayerG_delete, cv_PtrLcv_dnn_RequantizeLayerG_getInnerPtr_const, cv_PtrLcv_dnn_RequantizeLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::RequantizeLayer, cv_PtrLcv_dnn_RequantizeLayerG_new_const_RequantizeLayer }
	impl core::Ptr<crate::dnn::RequantizeLayer> {
		#[inline] pub fn as_raw_PtrOfRequantizeLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRequantizeLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::RequantizeLayerTraitConst for core::Ptr<crate::dnn::RequantizeLayer> {
		#[inline] fn as_raw_RequantizeLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::RequantizeLayerTrait for core::Ptr<crate::dnn::RequantizeLayer> {
		#[inline] fn as_raw_mut_RequantizeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::RequantizeLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::RequantizeLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::RequantizeLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_RequantizeLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::RequantizeLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::RequantizeLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::RequantizeLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_RequantizeLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::RequantizeLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfRequantizeLayer")
				.field("scale", &crate::dnn::RequantizeLayerTraitConst::scale(self))
				.field("shift", &crate::dnn::RequantizeLayerTraitConst::shift(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::ReshapeLayer,
		cv_PtrLcv_dnn_ReshapeLayerG_new_null_const, cv_PtrLcv_dnn_ReshapeLayerG_delete, cv_PtrLcv_dnn_ReshapeLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ReshapeLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ReshapeLayer, cv_PtrLcv_dnn_ReshapeLayerG_new_const_ReshapeLayer }
	impl core::Ptr<crate::dnn::ReshapeLayer> {
		#[inline] pub fn as_raw_PtrOfReshapeLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReshapeLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ReshapeLayerTraitConst for core::Ptr<crate::dnn::ReshapeLayer> {
		#[inline] fn as_raw_ReshapeLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ReshapeLayerTrait for core::Ptr<crate::dnn::ReshapeLayer> {
		#[inline] fn as_raw_mut_ReshapeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ReshapeLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ReshapeLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ReshapeLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ReshapeLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ReshapeLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ReshapeLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ReshapeLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ReshapeLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ReshapeLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfReshapeLayer")
				.field("new_shape_desc", &crate::dnn::ReshapeLayerTraitConst::new_shape_desc(self))
				.field("new_shape_range", &crate::dnn::ReshapeLayerTraitConst::new_shape_range(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::ResizeLayer,
		cv_PtrLcv_dnn_ResizeLayerG_new_null_const, cv_PtrLcv_dnn_ResizeLayerG_delete, cv_PtrLcv_dnn_ResizeLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ResizeLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ResizeLayer, cv_PtrLcv_dnn_ResizeLayerG_new_const_ResizeLayer }
	impl core::Ptr<crate::dnn::ResizeLayer> {
		#[inline] pub fn as_raw_PtrOfResizeLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfResizeLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ResizeLayerTraitConst for core::Ptr<crate::dnn::ResizeLayer> {
		#[inline] fn as_raw_ResizeLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ResizeLayerTrait for core::Ptr<crate::dnn::ResizeLayer> {
		#[inline] fn as_raw_mut_ResizeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ResizeLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ResizeLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ResizeLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ResizeLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ResizeLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ResizeLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ResizeLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ResizeLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ResizeLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfResizeLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::ScaleLayer,
		cv_PtrLcv_dnn_ScaleLayerG_new_null_const, cv_PtrLcv_dnn_ScaleLayerG_delete, cv_PtrLcv_dnn_ScaleLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ScaleLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ScaleLayer, cv_PtrLcv_dnn_ScaleLayerG_new_const_ScaleLayer }
	impl core::Ptr<crate::dnn::ScaleLayer> {
		#[inline] pub fn as_raw_PtrOfScaleLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfScaleLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ScaleLayerTraitConst for core::Ptr<crate::dnn::ScaleLayer> {
		#[inline] fn as_raw_ScaleLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ScaleLayerTrait for core::Ptr<crate::dnn::ScaleLayer> {
		#[inline] fn as_raw_mut_ScaleLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ScaleLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ScaleLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ScaleLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ScaleLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ScaleLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ScaleLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ScaleLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ScaleLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ScaleLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfScaleLayer")
				.field("has_bias", &crate::dnn::ScaleLayerTraitConst::has_bias(self))
				.field("axis", &crate::dnn::ScaleLayerTraitConst::axis(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::ScaleLayerInt8,
		cv_PtrLcv_dnn_ScaleLayerInt8G_new_null_const, cv_PtrLcv_dnn_ScaleLayerInt8G_delete, cv_PtrLcv_dnn_ScaleLayerInt8G_getInnerPtr_const, cv_PtrLcv_dnn_ScaleLayerInt8G_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ScaleLayerInt8, cv_PtrLcv_dnn_ScaleLayerInt8G_new_const_ScaleLayerInt8 }
	impl core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] pub fn as_raw_PtrOfScaleLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfScaleLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ScaleLayerInt8TraitConst for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] fn as_raw_ScaleLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ScaleLayerInt8Trait for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] fn as_raw_mut_ScaleLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ScaleLayerInt8>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ScaleLayerInt8G_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ScaleLayerInt8>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ScaleLayerInt8G_to_PtrOfLayer }

	impl crate::dnn::ScaleLayerTraitConst for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] fn as_raw_ScaleLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ScaleLayerTrait for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] fn as_raw_mut_ScaleLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ScaleLayerInt8>, core::Ptr<crate::dnn::ScaleLayer>, cv_PtrLcv_dnn_ScaleLayerInt8G_to_PtrOfScaleLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfScaleLayerInt8")
				.field("output_sc", &crate::dnn::ScaleLayerInt8TraitConst::output_sc(self))
				.field("output_zp", &crate::dnn::ScaleLayerInt8TraitConst::output_zp(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.field("has_bias", &crate::dnn::ScaleLayerTraitConst::has_bias(self))
				.field("axis", &crate::dnn::ScaleLayerTraitConst::axis(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::ShiftLayer,
		cv_PtrLcv_dnn_ShiftLayerG_new_null_const, cv_PtrLcv_dnn_ShiftLayerG_delete, cv_PtrLcv_dnn_ShiftLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ShiftLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ShiftLayer, cv_PtrLcv_dnn_ShiftLayerG_new_const_ShiftLayer }
	impl core::Ptr<crate::dnn::ShiftLayer> {
		#[inline] pub fn as_raw_PtrOfShiftLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfShiftLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ShiftLayerTraitConst for core::Ptr<crate::dnn::ShiftLayer> {
		#[inline] fn as_raw_ShiftLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ShiftLayerTrait for core::Ptr<crate::dnn::ShiftLayer> {
		#[inline] fn as_raw_mut_ShiftLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ShiftLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ShiftLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ShiftLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ShiftLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ShiftLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ShiftLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ShiftLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ShiftLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ShiftLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfShiftLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::ShiftLayerInt8,
		cv_PtrLcv_dnn_ShiftLayerInt8G_new_null_const, cv_PtrLcv_dnn_ShiftLayerInt8G_delete, cv_PtrLcv_dnn_ShiftLayerInt8G_getInnerPtr_const, cv_PtrLcv_dnn_ShiftLayerInt8G_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ShiftLayerInt8, cv_PtrLcv_dnn_ShiftLayerInt8G_new_const_ShiftLayerInt8 }
	impl core::Ptr<crate::dnn::ShiftLayerInt8> {
		#[inline] pub fn as_raw_PtrOfShiftLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfShiftLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ShiftLayerInt8TraitConst for core::Ptr<crate::dnn::ShiftLayerInt8> {
		#[inline] fn as_raw_ShiftLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ShiftLayerInt8Trait for core::Ptr<crate::dnn::ShiftLayerInt8> {
		#[inline] fn as_raw_mut_ShiftLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ShiftLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ShiftLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ShiftLayerInt8>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ShiftLayerInt8G_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ShiftLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ShiftLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ShiftLayerInt8>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ShiftLayerInt8G_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ShiftLayerInt8> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfShiftLayerInt8")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::ShuffleChannelLayer,
		cv_PtrLcv_dnn_ShuffleChannelLayerG_new_null_const, cv_PtrLcv_dnn_ShuffleChannelLayerG_delete, cv_PtrLcv_dnn_ShuffleChannelLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ShuffleChannelLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ShuffleChannelLayer, cv_PtrLcv_dnn_ShuffleChannelLayerG_new_const_ShuffleChannelLayer }
	impl core::Ptr<crate::dnn::ShuffleChannelLayer> {
		#[inline] pub fn as_raw_PtrOfShuffleChannelLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfShuffleChannelLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ShuffleChannelLayerTraitConst for core::Ptr<crate::dnn::ShuffleChannelLayer> {
		#[inline] fn as_raw_ShuffleChannelLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ShuffleChannelLayerTrait for core::Ptr<crate::dnn::ShuffleChannelLayer> {
		#[inline] fn as_raw_mut_ShuffleChannelLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ShuffleChannelLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ShuffleChannelLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ShuffleChannelLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ShuffleChannelLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ShuffleChannelLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ShuffleChannelLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ShuffleChannelLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ShuffleChannelLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ShuffleChannelLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfShuffleChannelLayer")
				.field("group", &crate::dnn::ShuffleChannelLayerTraitConst::group(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::SigmoidLayer,
		cv_PtrLcv_dnn_SigmoidLayerG_new_null_const, cv_PtrLcv_dnn_SigmoidLayerG_delete, cv_PtrLcv_dnn_SigmoidLayerG_getInnerPtr_const, cv_PtrLcv_dnn_SigmoidLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::SigmoidLayer, cv_PtrLcv_dnn_SigmoidLayerG_new_const_SigmoidLayer }
	impl core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] pub fn as_raw_PtrOfSigmoidLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSigmoidLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::SigmoidLayerTraitConst for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] fn as_raw_SigmoidLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::SigmoidLayerTrait for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] fn as_raw_mut_SigmoidLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SigmoidLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_SigmoidLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SigmoidLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_SigmoidLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SigmoidLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_SigmoidLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfSigmoidLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::SliceLayer,
		cv_PtrLcv_dnn_SliceLayerG_new_null_const, cv_PtrLcv_dnn_SliceLayerG_delete, cv_PtrLcv_dnn_SliceLayerG_getInnerPtr_const, cv_PtrLcv_dnn_SliceLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::SliceLayer, cv_PtrLcv_dnn_SliceLayerG_new_const_SliceLayer }
	impl core::Ptr<crate::dnn::SliceLayer> {
		#[inline] pub fn as_raw_PtrOfSliceLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSliceLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::SliceLayerTraitConst for core::Ptr<crate::dnn::SliceLayer> {
		#[inline] fn as_raw_SliceLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::SliceLayerTrait for core::Ptr<crate::dnn::SliceLayer> {
		#[inline] fn as_raw_mut_SliceLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SliceLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SliceLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SliceLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_SliceLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SliceLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SliceLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SliceLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_SliceLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::SliceLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfSliceLayer")
				.field("slice_ranges", &crate::dnn::SliceLayerTraitConst::slice_ranges(self))
				.field("slice_steps", &crate::dnn::SliceLayerTraitConst::slice_steps(self))
				.field("axis", &crate::dnn::SliceLayerTraitConst::axis(self))
				.field("num_split", &crate::dnn::SliceLayerTraitConst::num_split(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::SoftmaxLayer,
		cv_PtrLcv_dnn_SoftmaxLayerG_new_null_const, cv_PtrLcv_dnn_SoftmaxLayerG_delete, cv_PtrLcv_dnn_SoftmaxLayerG_getInnerPtr_const, cv_PtrLcv_dnn_SoftmaxLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::SoftmaxLayer, cv_PtrLcv_dnn_SoftmaxLayerG_new_const_SoftmaxLayer }
	impl core::Ptr<crate::dnn::SoftmaxLayer> {
		#[inline] pub fn as_raw_PtrOfSoftmaxLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSoftmaxLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::SoftmaxLayerTraitConst for core::Ptr<crate::dnn::SoftmaxLayer> {
		#[inline] fn as_raw_SoftmaxLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::SoftmaxLayerTrait for core::Ptr<crate::dnn::SoftmaxLayer> {
		#[inline] fn as_raw_mut_SoftmaxLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SoftmaxLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SoftmaxLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SoftmaxLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_SoftmaxLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SoftmaxLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SoftmaxLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SoftmaxLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_SoftmaxLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::SoftmaxLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfSoftmaxLayer")
				.field("log_soft_max", &crate::dnn::SoftmaxLayerTraitConst::log_soft_max(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::SoftmaxLayerInt8,
		cv_PtrLcv_dnn_SoftmaxLayerInt8G_new_null_const, cv_PtrLcv_dnn_SoftmaxLayerInt8G_delete, cv_PtrLcv_dnn_SoftmaxLayerInt8G_getInnerPtr_const, cv_PtrLcv_dnn_SoftmaxLayerInt8G_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::SoftmaxLayerInt8, cv_PtrLcv_dnn_SoftmaxLayerInt8G_new_const_SoftmaxLayerInt8 }
	impl core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] pub fn as_raw_PtrOfSoftmaxLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSoftmaxLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::SoftmaxLayerInt8TraitConst for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] fn as_raw_SoftmaxLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::SoftmaxLayerInt8Trait for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] fn as_raw_mut_SoftmaxLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SoftmaxLayerInt8>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_SoftmaxLayerInt8G_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SoftmaxLayerInt8>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_SoftmaxLayerInt8G_to_PtrOfLayer }

	impl crate::dnn::SoftmaxLayerTraitConst for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] fn as_raw_SoftmaxLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::SoftmaxLayerTrait for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] fn as_raw_mut_SoftmaxLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SoftmaxLayerInt8>, core::Ptr<crate::dnn::SoftmaxLayer>, cv_PtrLcv_dnn_SoftmaxLayerInt8G_to_PtrOfSoftmaxLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfSoftmaxLayerInt8")
				.field("output_sc", &crate::dnn::SoftmaxLayerInt8TraitConst::output_sc(self))
				.field("output_zp", &crate::dnn::SoftmaxLayerInt8TraitConst::output_zp(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.field("log_soft_max", &crate::dnn::SoftmaxLayerTraitConst::log_soft_max(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::SplitLayer,
		cv_PtrLcv_dnn_SplitLayerG_new_null_const, cv_PtrLcv_dnn_SplitLayerG_delete, cv_PtrLcv_dnn_SplitLayerG_getInnerPtr_const, cv_PtrLcv_dnn_SplitLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::SplitLayer, cv_PtrLcv_dnn_SplitLayerG_new_const_SplitLayer }
	impl core::Ptr<crate::dnn::SplitLayer> {
		#[inline] pub fn as_raw_PtrOfSplitLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSplitLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::SplitLayerTraitConst for core::Ptr<crate::dnn::SplitLayer> {
		#[inline] fn as_raw_SplitLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::SplitLayerTrait for core::Ptr<crate::dnn::SplitLayer> {
		#[inline] fn as_raw_mut_SplitLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SplitLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SplitLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SplitLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_SplitLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SplitLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SplitLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SplitLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_SplitLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::SplitLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfSplitLayer")
				.field("outputs_count", &crate::dnn::SplitLayerTraitConst::outputs_count(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::SwishLayer,
		cv_PtrLcv_dnn_SwishLayerG_new_null_const, cv_PtrLcv_dnn_SwishLayerG_delete, cv_PtrLcv_dnn_SwishLayerG_getInnerPtr_const, cv_PtrLcv_dnn_SwishLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::SwishLayer, cv_PtrLcv_dnn_SwishLayerG_new_const_SwishLayer }
	impl core::Ptr<crate::dnn::SwishLayer> {
		#[inline] pub fn as_raw_PtrOfSwishLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSwishLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::SwishLayerTraitConst for core::Ptr<crate::dnn::SwishLayer> {
		#[inline] fn as_raw_SwishLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::SwishLayerTrait for core::Ptr<crate::dnn::SwishLayer> {
		#[inline] fn as_raw_mut_SwishLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SwishLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SwishLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SwishLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_SwishLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::SwishLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::SwishLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SwishLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_SwishLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SwishLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SwishLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SwishLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_SwishLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::SwishLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfSwishLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	ptr_extern! { crate::dnn::TanHLayer,
		cv_PtrLcv_dnn_TanHLayerG_new_null_const, cv_PtrLcv_dnn_TanHLayerG_delete, cv_PtrLcv_dnn_TanHLayerG_getInnerPtr_const, cv_PtrLcv_dnn_TanHLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::TanHLayer, cv_PtrLcv_dnn_TanHLayerG_new_const_TanHLayer }
	impl core::Ptr<crate::dnn::TanHLayer> {
		#[inline] pub fn as_raw_PtrOfTanHLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTanHLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::TanHLayerTraitConst for core::Ptr<crate::dnn::TanHLayer> {
		#[inline] fn as_raw_TanHLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::TanHLayerTrait for core::Ptr<crate::dnn::TanHLayer> {
		#[inline] fn as_raw_mut_TanHLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::TanHLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::TanHLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::TanHLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_TanHLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::TanHLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::TanHLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::TanHLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_TanHLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::TanHLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::TanHLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::TanHLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_TanHLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::TanHLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfTanHLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	impl core::Tuple<(crate::dnn::Backend, crate::dnn::Target)> {
		pub fn as_raw_TupleOfBackend_Target(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_TupleOfBackend_Target(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	tuple_extern! { (crate::dnn::Backend, crate::dnn::Target),
		std_pairLcv_dnn_Backend__cv_dnn_TargetG_new_const_Backend_Target, std_pairLcv_dnn_Backend__cv_dnn_TargetG_delete,
		0 = arg: crate::dnn::Backend, get_0 via std_pairLcv_dnn_Backend__cv_dnn_TargetG_get_0_const,
		1 = arg_1: crate::dnn::Target, get_1 via std_pairLcv_dnn_Backend__cv_dnn_TargetG_get_1_const
	}

	impl core::Vector<crate::dnn::MatShape> {
		pub fn as_raw_VectorOfMatShape(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfMatShape(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl core::Vector<core::Ptr<crate::dnn::BackendNode>> {
		pub fn as_raw_VectorOfPtrOfBackendNode(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPtrOfBackendNode(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Ptr<crate::dnn::BackendNode>,
		std_vectorLcv_PtrLcv_dnn_BackendNodeGG_new_const, std_vectorLcv_PtrLcv_dnn_BackendNodeGG_delete,
		std_vectorLcv_PtrLcv_dnn_BackendNodeGG_len_const, std_vectorLcv_PtrLcv_dnn_BackendNodeGG_isEmpty_const,
		std_vectorLcv_PtrLcv_dnn_BackendNodeGG_capacity_const, std_vectorLcv_PtrLcv_dnn_BackendNodeGG_shrinkToFit,
		std_vectorLcv_PtrLcv_dnn_BackendNodeGG_reserve_size_t, std_vectorLcv_PtrLcv_dnn_BackendNodeGG_remove_size_t,
		std_vectorLcv_PtrLcv_dnn_BackendNodeGG_swap_size_t_size_t, std_vectorLcv_PtrLcv_dnn_BackendNodeGG_clear,
		std_vectorLcv_PtrLcv_dnn_BackendNodeGG_get_const_size_t, std_vectorLcv_PtrLcv_dnn_BackendNodeGG_set_size_t_const_PtrLBackendNodeG,
		std_vectorLcv_PtrLcv_dnn_BackendNodeGG_push_const_PtrLBackendNodeG, std_vectorLcv_PtrLcv_dnn_BackendNodeGG_insert_size_t_const_PtrLBackendNodeG,
	}

	vector_non_copy_or_bool! { core::Ptr<crate::dnn::BackendNode> }


	impl core::Vector<core::Ptr<crate::dnn::BackendWrapper>> {
		pub fn as_raw_VectorOfPtrOfBackendWrapper(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPtrOfBackendWrapper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Ptr<crate::dnn::BackendWrapper>,
		std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_new_const, std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_delete,
		std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_len_const, std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_isEmpty_const,
		std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_capacity_const, std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_shrinkToFit,
		std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_reserve_size_t, std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_remove_size_t,
		std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_swap_size_t_size_t, std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_clear,
		std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_get_const_size_t, std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_set_size_t_const_PtrLBackendWrapperG,
		std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_push_const_PtrLBackendWrapperG, std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_insert_size_t_const_PtrLBackendWrapperG,
	}

	vector_non_copy_or_bool! { core::Ptr<crate::dnn::BackendWrapper> }


	impl core::Vector<core::Ptr<crate::dnn::Layer>> {
		pub fn as_raw_VectorOfPtrOfLayer(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPtrOfLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Ptr<crate::dnn::Layer>,
		std_vectorLcv_PtrLcv_dnn_LayerGG_new_const, std_vectorLcv_PtrLcv_dnn_LayerGG_delete,
		std_vectorLcv_PtrLcv_dnn_LayerGG_len_const, std_vectorLcv_PtrLcv_dnn_LayerGG_isEmpty_const,
		std_vectorLcv_PtrLcv_dnn_LayerGG_capacity_const, std_vectorLcv_PtrLcv_dnn_LayerGG_shrinkToFit,
		std_vectorLcv_PtrLcv_dnn_LayerGG_reserve_size_t, std_vectorLcv_PtrLcv_dnn_LayerGG_remove_size_t,
		std_vectorLcv_PtrLcv_dnn_LayerGG_swap_size_t_size_t, std_vectorLcv_PtrLcv_dnn_LayerGG_clear,
		std_vectorLcv_PtrLcv_dnn_LayerGG_get_const_size_t, std_vectorLcv_PtrLcv_dnn_LayerGG_set_size_t_const_PtrLLayerG,
		std_vectorLcv_PtrLcv_dnn_LayerGG_push_const_PtrLLayerG, std_vectorLcv_PtrLcv_dnn_LayerGG_insert_size_t_const_PtrLLayerG,
	}

	vector_non_copy_or_bool! { core::Ptr<crate::dnn::Layer> }


	impl core::Vector<crate::dnn::Target> {
		pub fn as_raw_VectorOfTarget(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfTarget(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { crate::dnn::Target,
		std_vectorLcv_dnn_TargetG_new_const, std_vectorLcv_dnn_TargetG_delete,
		std_vectorLcv_dnn_TargetG_len_const, std_vectorLcv_dnn_TargetG_isEmpty_const,
		std_vectorLcv_dnn_TargetG_capacity_const, std_vectorLcv_dnn_TargetG_shrinkToFit,
		std_vectorLcv_dnn_TargetG_reserve_size_t, std_vectorLcv_dnn_TargetG_remove_size_t,
		std_vectorLcv_dnn_TargetG_swap_size_t_size_t, std_vectorLcv_dnn_TargetG_clear,
		std_vectorLcv_dnn_TargetG_get_const_size_t, std_vectorLcv_dnn_TargetG_set_size_t_const_Target,
		std_vectorLcv_dnn_TargetG_push_const_Target, std_vectorLcv_dnn_TargetG_insert_size_t_const_Target,
	}

	vector_copy_non_bool! { crate::dnn::Target,
		std_vectorLcv_dnn_TargetG_data_const, std_vectorLcv_dnn_TargetG_dataMut, cv_fromSlice_const_const_TargetX_size_t,
		std_vectorLcv_dnn_TargetG_clone_const,
	}


	impl core::Vector<core::Tuple<(crate::dnn::Backend, crate::dnn::Target)>> {
		pub fn as_raw_VectorOfTupleOfBackend_Target(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfTupleOfBackend_Target(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Tuple<(crate::dnn::Backend, crate::dnn::Target)>,
		std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_new_const, std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_delete,
		std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_len_const, std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_isEmpty_const,
		std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_capacity_const, std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_shrinkToFit,
		std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_reserve_size_t, std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_remove_size_t,
		std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_swap_size_t_size_t, std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_clear,
		std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_get_const_size_t, std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_set_size_t_const_pairLcv_dnn_Backend__cv_dnn_TargetG,
		std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_push_const_pairLcv_dnn_Backend__cv_dnn_TargetG, std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_insert_size_t_const_pairLcv_dnn_Backend__cv_dnn_TargetG,
	}

	vector_non_copy_or_bool! { core::Tuple<(crate::dnn::Backend, crate::dnn::Target)> }


	impl core::Vector<core::Vector<crate::dnn::MatShape>> {
		pub fn as_raw_VectorOfVectorOfMatShape(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfMatShape(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Vector<crate::dnn::MatShape>,
		std_vectorLstd_vectorLcv_dnn_MatShapeGG_new_const, std_vectorLstd_vectorLcv_dnn_MatShapeGG_delete,
		std_vectorLstd_vectorLcv_dnn_MatShapeGG_len_const, std_vectorLstd_vectorLcv_dnn_MatShapeGG_isEmpty_const,
		std_vectorLstd_vectorLcv_dnn_MatShapeGG_capacity_const, std_vectorLstd_vectorLcv_dnn_MatShapeGG_shrinkToFit,
		std_vectorLstd_vectorLcv_dnn_MatShapeGG_reserve_size_t, std_vectorLstd_vectorLcv_dnn_MatShapeGG_remove_size_t,
		std_vectorLstd_vectorLcv_dnn_MatShapeGG_swap_size_t_size_t, std_vectorLstd_vectorLcv_dnn_MatShapeGG_clear,
		std_vectorLstd_vectorLcv_dnn_MatShapeGG_get_const_size_t, std_vectorLstd_vectorLcv_dnn_MatShapeGG_set_size_t_const_vectorLMatShapeG,
		std_vectorLstd_vectorLcv_dnn_MatShapeGG_push_const_vectorLMatShapeG, std_vectorLstd_vectorLcv_dnn_MatShapeGG_insert_size_t_const_vectorLMatShapeG,
	}

	vector_non_copy_or_bool! { core::Vector<crate::dnn::MatShape> }


}
pub use dnn_types::*;

mod imgproc_types {
	use crate::{mod_prelude::*, core, types, sys};

	ptr_extern! { crate::imgproc::CLAHE,
		cv_PtrLcv_CLAHEG_new_null_const, cv_PtrLcv_CLAHEG_delete, cv_PtrLcv_CLAHEG_getInnerPtr_const, cv_PtrLcv_CLAHEG_getInnerPtrMut
	}

	impl core::Ptr<crate::imgproc::CLAHE> {
		#[inline] pub fn as_raw_PtrOfCLAHE(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCLAHE(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::imgproc::CLAHETraitConst for core::Ptr<crate::imgproc::CLAHE> {
		#[inline] fn as_raw_CLAHE(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::imgproc::CLAHETrait for core::Ptr<crate::imgproc::CLAHE> {
		#[inline] fn as_raw_mut_CLAHE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::imgproc::CLAHE> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::imgproc::CLAHE> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::imgproc::CLAHE>, core::Ptr<core::Algorithm>, cv_PtrLcv_CLAHEG_to_PtrOfAlgorithm }

	impl std::fmt::Debug for core::Ptr<crate::imgproc::CLAHE> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfCLAHE")
				.finish()
		}
	}

	ptr_extern! { crate::imgproc::GeneralizedHough,
		cv_PtrLcv_GeneralizedHoughG_new_null_const, cv_PtrLcv_GeneralizedHoughG_delete, cv_PtrLcv_GeneralizedHoughG_getInnerPtr_const, cv_PtrLcv_GeneralizedHoughG_getInnerPtrMut
	}

	impl core::Ptr<crate::imgproc::GeneralizedHough> {
		#[inline] pub fn as_raw_PtrOfGeneralizedHough(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGeneralizedHough(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::imgproc::GeneralizedHoughTraitConst for core::Ptr<crate::imgproc::GeneralizedHough> {
		#[inline] fn as_raw_GeneralizedHough(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::imgproc::GeneralizedHoughTrait for core::Ptr<crate::imgproc::GeneralizedHough> {
		#[inline] fn as_raw_mut_GeneralizedHough(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::imgproc::GeneralizedHough> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::imgproc::GeneralizedHough> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::imgproc::GeneralizedHough>, core::Ptr<core::Algorithm>, cv_PtrLcv_GeneralizedHoughG_to_PtrOfAlgorithm }

	impl std::fmt::Debug for core::Ptr<crate::imgproc::GeneralizedHough> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfGeneralizedHough")
				.finish()
		}
	}

	ptr_extern! { crate::imgproc::GeneralizedHoughBallard,
		cv_PtrLcv_GeneralizedHoughBallardG_new_null_const, cv_PtrLcv_GeneralizedHoughBallardG_delete, cv_PtrLcv_GeneralizedHoughBallardG_getInnerPtr_const, cv_PtrLcv_GeneralizedHoughBallardG_getInnerPtrMut
	}

	impl core::Ptr<crate::imgproc::GeneralizedHoughBallard> {
		#[inline] pub fn as_raw_PtrOfGeneralizedHoughBallard(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGeneralizedHoughBallard(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::imgproc::GeneralizedHoughBallardTraitConst for core::Ptr<crate::imgproc::GeneralizedHoughBallard> {
		#[inline] fn as_raw_GeneralizedHoughBallard(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::imgproc::GeneralizedHoughBallardTrait for core::Ptr<crate::imgproc::GeneralizedHoughBallard> {
		#[inline] fn as_raw_mut_GeneralizedHoughBallard(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::imgproc::GeneralizedHoughBallard> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::imgproc::GeneralizedHoughBallard> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::imgproc::GeneralizedHoughBallard>, core::Ptr<core::Algorithm>, cv_PtrLcv_GeneralizedHoughBallardG_to_PtrOfAlgorithm }

	impl crate::imgproc::GeneralizedHoughTraitConst for core::Ptr<crate::imgproc::GeneralizedHoughBallard> {
		#[inline] fn as_raw_GeneralizedHough(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::imgproc::GeneralizedHoughTrait for core::Ptr<crate::imgproc::GeneralizedHoughBallard> {
		#[inline] fn as_raw_mut_GeneralizedHough(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::imgproc::GeneralizedHoughBallard>, core::Ptr<crate::imgproc::GeneralizedHough>, cv_PtrLcv_GeneralizedHoughBallardG_to_PtrOfGeneralizedHough }

	impl std::fmt::Debug for core::Ptr<crate::imgproc::GeneralizedHoughBallard> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfGeneralizedHoughBallard")
				.finish()
		}
	}

	ptr_extern! { crate::imgproc::GeneralizedHoughGuil,
		cv_PtrLcv_GeneralizedHoughGuilG_new_null_const, cv_PtrLcv_GeneralizedHoughGuilG_delete, cv_PtrLcv_GeneralizedHoughGuilG_getInnerPtr_const, cv_PtrLcv_GeneralizedHoughGuilG_getInnerPtrMut
	}

	impl core::Ptr<crate::imgproc::GeneralizedHoughGuil> {
		#[inline] pub fn as_raw_PtrOfGeneralizedHoughGuil(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGeneralizedHoughGuil(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::imgproc::GeneralizedHoughGuilTraitConst for core::Ptr<crate::imgproc::GeneralizedHoughGuil> {
		#[inline] fn as_raw_GeneralizedHoughGuil(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::imgproc::GeneralizedHoughGuilTrait for core::Ptr<crate::imgproc::GeneralizedHoughGuil> {
		#[inline] fn as_raw_mut_GeneralizedHoughGuil(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::imgproc::GeneralizedHoughGuil> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::imgproc::GeneralizedHoughGuil> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::imgproc::GeneralizedHoughGuil>, core::Ptr<core::Algorithm>, cv_PtrLcv_GeneralizedHoughGuilG_to_PtrOfAlgorithm }

	impl crate::imgproc::GeneralizedHoughTraitConst for core::Ptr<crate::imgproc::GeneralizedHoughGuil> {
		#[inline] fn as_raw_GeneralizedHough(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::imgproc::GeneralizedHoughTrait for core::Ptr<crate::imgproc::GeneralizedHoughGuil> {
		#[inline] fn as_raw_mut_GeneralizedHough(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::imgproc::GeneralizedHoughGuil>, core::Ptr<crate::imgproc::GeneralizedHough>, cv_PtrLcv_GeneralizedHoughGuilG_to_PtrOfGeneralizedHough }

	impl std::fmt::Debug for core::Ptr<crate::imgproc::GeneralizedHoughGuil> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfGeneralizedHoughGuil")
				.finish()
		}
	}

	ptr_extern! { crate::imgproc::LineSegmentDetector,
		cv_PtrLcv_LineSegmentDetectorG_new_null_const, cv_PtrLcv_LineSegmentDetectorG_delete, cv_PtrLcv_LineSegmentDetectorG_getInnerPtr_const, cv_PtrLcv_LineSegmentDetectorG_getInnerPtrMut
	}

	impl core::Ptr<crate::imgproc::LineSegmentDetector> {
		#[inline] pub fn as_raw_PtrOfLineSegmentDetector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLineSegmentDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::imgproc::LineSegmentDetectorTraitConst for core::Ptr<crate::imgproc::LineSegmentDetector> {
		#[inline] fn as_raw_LineSegmentDetector(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::imgproc::LineSegmentDetectorTrait for core::Ptr<crate::imgproc::LineSegmentDetector> {
		#[inline] fn as_raw_mut_LineSegmentDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::imgproc::LineSegmentDetector> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::imgproc::LineSegmentDetector> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::imgproc::LineSegmentDetector>, core::Ptr<core::Algorithm>, cv_PtrLcv_LineSegmentDetectorG_to_PtrOfAlgorithm }

	impl std::fmt::Debug for core::Ptr<crate::imgproc::LineSegmentDetector> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfLineSegmentDetector")
				.finish()
		}
	}

}
pub use imgproc_types::*;

mod objdetect_types {
	use crate::{mod_prelude::*, core, types, sys};

	ptr_extern! { crate::objdetect::BaseCascadeClassifier,
		cv_PtrLcv_BaseCascadeClassifierG_new_null_const, cv_PtrLcv_BaseCascadeClassifierG_delete, cv_PtrLcv_BaseCascadeClassifierG_getInnerPtr_const, cv_PtrLcv_BaseCascadeClassifierG_getInnerPtrMut
	}

	impl core::Ptr<crate::objdetect::BaseCascadeClassifier> {
		#[inline] pub fn as_raw_PtrOfBaseCascadeClassifier(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBaseCascadeClassifier(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::objdetect::BaseCascadeClassifierTraitConst for core::Ptr<crate::objdetect::BaseCascadeClassifier> {
		#[inline] fn as_raw_BaseCascadeClassifier(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::objdetect::BaseCascadeClassifierTrait for core::Ptr<crate::objdetect::BaseCascadeClassifier> {
		#[inline] fn as_raw_mut_BaseCascadeClassifier(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::objdetect::BaseCascadeClassifier> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::objdetect::BaseCascadeClassifier> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::objdetect::BaseCascadeClassifier>, core::Ptr<core::Algorithm>, cv_PtrLcv_BaseCascadeClassifierG_to_PtrOfAlgorithm }

	impl std::fmt::Debug for core::Ptr<crate::objdetect::BaseCascadeClassifier> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfBaseCascadeClassifier")
				.finish()
		}
	}

	ptr_extern! { crate::objdetect::BaseCascadeClassifier_MaskGenerator,
		cv_PtrLcv_BaseCascadeClassifier_MaskGeneratorG_new_null_const, cv_PtrLcv_BaseCascadeClassifier_MaskGeneratorG_delete, cv_PtrLcv_BaseCascadeClassifier_MaskGeneratorG_getInnerPtr_const, cv_PtrLcv_BaseCascadeClassifier_MaskGeneratorG_getInnerPtrMut
	}

	impl core::Ptr<crate::objdetect::BaseCascadeClassifier_MaskGenerator> {
		#[inline] pub fn as_raw_PtrOfBaseCascadeClassifier_MaskGenerator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBaseCascadeClassifier_MaskGenerator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::objdetect::BaseCascadeClassifier_MaskGeneratorTraitConst for core::Ptr<crate::objdetect::BaseCascadeClassifier_MaskGenerator> {
		#[inline] fn as_raw_BaseCascadeClassifier_MaskGenerator(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::objdetect::BaseCascadeClassifier_MaskGeneratorTrait for core::Ptr<crate::objdetect::BaseCascadeClassifier_MaskGenerator> {
		#[inline] fn as_raw_mut_BaseCascadeClassifier_MaskGenerator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl std::fmt::Debug for core::Ptr<crate::objdetect::BaseCascadeClassifier_MaskGenerator> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfBaseCascadeClassifier_MaskGenerator")
				.finish()
		}
	}

	ptr_extern! { crate::objdetect::DetectionBasedTracker_IDetector,
		cv_PtrLcv_DetectionBasedTracker_IDetectorG_new_null_const, cv_PtrLcv_DetectionBasedTracker_IDetectorG_delete, cv_PtrLcv_DetectionBasedTracker_IDetectorG_getInnerPtr_const, cv_PtrLcv_DetectionBasedTracker_IDetectorG_getInnerPtrMut
	}

	impl core::Ptr<crate::objdetect::DetectionBasedTracker_IDetector> {
		#[inline] pub fn as_raw_PtrOfDetectionBasedTracker_IDetector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetectionBasedTracker_IDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::objdetect::DetectionBasedTracker_IDetectorTraitConst for core::Ptr<crate::objdetect::DetectionBasedTracker_IDetector> {
		#[inline] fn as_raw_DetectionBasedTracker_IDetector(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::objdetect::DetectionBasedTracker_IDetectorTrait for core::Ptr<crate::objdetect::DetectionBasedTracker_IDetector> {
		#[inline] fn as_raw_mut_DetectionBasedTracker_IDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl std::fmt::Debug for core::Ptr<crate::objdetect::DetectionBasedTracker_IDetector> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfDetectionBasedTracker_IDetector")
				.finish()
		}
	}

	ptr_extern! { crate::objdetect::FaceDetectorYN,
		cv_PtrLcv_FaceDetectorYNG_new_null_const, cv_PtrLcv_FaceDetectorYNG_delete, cv_PtrLcv_FaceDetectorYNG_getInnerPtr_const, cv_PtrLcv_FaceDetectorYNG_getInnerPtrMut
	}

	impl core::Ptr<crate::objdetect::FaceDetectorYN> {
		#[inline] pub fn as_raw_PtrOfFaceDetectorYN(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFaceDetectorYN(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::objdetect::FaceDetectorYNTraitConst for core::Ptr<crate::objdetect::FaceDetectorYN> {
		#[inline] fn as_raw_FaceDetectorYN(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::objdetect::FaceDetectorYNTrait for core::Ptr<crate::objdetect::FaceDetectorYN> {
		#[inline] fn as_raw_mut_FaceDetectorYN(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl std::fmt::Debug for core::Ptr<crate::objdetect::FaceDetectorYN> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfFaceDetectorYN")
				.finish()
		}
	}

	ptr_extern! { crate::objdetect::FaceRecognizerSF,
		cv_PtrLcv_FaceRecognizerSFG_new_null_const, cv_PtrLcv_FaceRecognizerSFG_delete, cv_PtrLcv_FaceRecognizerSFG_getInnerPtr_const, cv_PtrLcv_FaceRecognizerSFG_getInnerPtrMut
	}

	impl core::Ptr<crate::objdetect::FaceRecognizerSF> {
		#[inline] pub fn as_raw_PtrOfFaceRecognizerSF(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFaceRecognizerSF(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::objdetect::FaceRecognizerSFTraitConst for core::Ptr<crate::objdetect::FaceRecognizerSF> {
		#[inline] fn as_raw_FaceRecognizerSF(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::objdetect::FaceRecognizerSFTrait for core::Ptr<crate::objdetect::FaceRecognizerSF> {
		#[inline] fn as_raw_mut_FaceRecognizerSF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl std::fmt::Debug for core::Ptr<crate::objdetect::FaceRecognizerSF> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfFaceRecognizerSF")
				.finish()
		}
	}

	impl core::Vector<crate::objdetect::DetectionBasedTracker_ExtObject> {
		pub fn as_raw_VectorOfDetectionBasedTracker_ExtObject(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfDetectionBasedTracker_ExtObject(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { crate::objdetect::DetectionBasedTracker_ExtObject,
		std_vectorLcv_DetectionBasedTracker_ExtObjectG_new_const, std_vectorLcv_DetectionBasedTracker_ExtObjectG_delete,
		std_vectorLcv_DetectionBasedTracker_ExtObjectG_len_const, std_vectorLcv_DetectionBasedTracker_ExtObjectG_isEmpty_const,
		std_vectorLcv_DetectionBasedTracker_ExtObjectG_capacity_const, std_vectorLcv_DetectionBasedTracker_ExtObjectG_shrinkToFit,
		std_vectorLcv_DetectionBasedTracker_ExtObjectG_reserve_size_t, std_vectorLcv_DetectionBasedTracker_ExtObjectG_remove_size_t,
		std_vectorLcv_DetectionBasedTracker_ExtObjectG_swap_size_t_size_t, std_vectorLcv_DetectionBasedTracker_ExtObjectG_clear,
		std_vectorLcv_DetectionBasedTracker_ExtObjectG_get_const_size_t, std_vectorLcv_DetectionBasedTracker_ExtObjectG_set_size_t_const_ExtObject,
		std_vectorLcv_DetectionBasedTracker_ExtObjectG_push_const_ExtObject, std_vectorLcv_DetectionBasedTracker_ExtObjectG_insert_size_t_const_ExtObject,
	}

	vector_non_copy_or_bool! { clone crate::objdetect::DetectionBasedTracker_ExtObject }

	vector_boxed_ref! { crate::objdetect::DetectionBasedTracker_ExtObject }

	vector_extern! { BoxedRef<'t, crate::objdetect::DetectionBasedTracker_ExtObject>,
		std_vectorLcv_DetectionBasedTracker_ExtObjectG_new_const, std_vectorLcv_DetectionBasedTracker_ExtObjectG_delete,
		std_vectorLcv_DetectionBasedTracker_ExtObjectG_len_const, std_vectorLcv_DetectionBasedTracker_ExtObjectG_isEmpty_const,
		std_vectorLcv_DetectionBasedTracker_ExtObjectG_capacity_const, std_vectorLcv_DetectionBasedTracker_ExtObjectG_shrinkToFit,
		std_vectorLcv_DetectionBasedTracker_ExtObjectG_reserve_size_t, std_vectorLcv_DetectionBasedTracker_ExtObjectG_remove_size_t,
		std_vectorLcv_DetectionBasedTracker_ExtObjectG_swap_size_t_size_t, std_vectorLcv_DetectionBasedTracker_ExtObjectG_clear,
		std_vectorLcv_DetectionBasedTracker_ExtObjectG_get_const_size_t, std_vectorLcv_DetectionBasedTracker_ExtObjectG_set_size_t_const_ExtObject,
		std_vectorLcv_DetectionBasedTracker_ExtObjectG_push_const_ExtObject, std_vectorLcv_DetectionBasedTracker_ExtObjectG_insert_size_t_const_ExtObject,
	}


	impl core::Vector<crate::objdetect::DetectionBasedTracker_Object> {
		pub fn as_raw_VectorOfDetectionBasedTracker_Object(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfDetectionBasedTracker_Object(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { crate::objdetect::DetectionBasedTracker_Object,
		std_vectorLcv_DetectionBasedTracker_ObjectG_new_const, std_vectorLcv_DetectionBasedTracker_ObjectG_delete,
		std_vectorLcv_DetectionBasedTracker_ObjectG_len_const, std_vectorLcv_DetectionBasedTracker_ObjectG_isEmpty_const,
		std_vectorLcv_DetectionBasedTracker_ObjectG_capacity_const, std_vectorLcv_DetectionBasedTracker_ObjectG_shrinkToFit,
		std_vectorLcv_DetectionBasedTracker_ObjectG_reserve_size_t, std_vectorLcv_DetectionBasedTracker_ObjectG_remove_size_t,
		std_vectorLcv_DetectionBasedTracker_ObjectG_swap_size_t_size_t, std_vectorLcv_DetectionBasedTracker_ObjectG_clear,
		std_vectorLcv_DetectionBasedTracker_ObjectG_get_const_size_t, std_vectorLcv_DetectionBasedTracker_ObjectG_set_size_t_const_Object,
		std_vectorLcv_DetectionBasedTracker_ObjectG_push_const_Object, std_vectorLcv_DetectionBasedTracker_ObjectG_insert_size_t_const_Object,
	}

	vector_non_copy_or_bool! { crate::objdetect::DetectionBasedTracker_Object }


	impl core::Vector<crate::objdetect::DetectionROI> {
		pub fn as_raw_VectorOfDetectionROI(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfDetectionROI(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { crate::objdetect::DetectionROI,
		std_vectorLcv_DetectionROIG_new_const, std_vectorLcv_DetectionROIG_delete,
		std_vectorLcv_DetectionROIG_len_const, std_vectorLcv_DetectionROIG_isEmpty_const,
		std_vectorLcv_DetectionROIG_capacity_const, std_vectorLcv_DetectionROIG_shrinkToFit,
		std_vectorLcv_DetectionROIG_reserve_size_t, std_vectorLcv_DetectionROIG_remove_size_t,
		std_vectorLcv_DetectionROIG_swap_size_t_size_t, std_vectorLcv_DetectionROIG_clear,
		std_vectorLcv_DetectionROIG_get_const_size_t, std_vectorLcv_DetectionROIG_set_size_t_const_DetectionROI,
		std_vectorLcv_DetectionROIG_push_const_DetectionROI, std_vectorLcv_DetectionROIG_insert_size_t_const_DetectionROI,
	}

	vector_non_copy_or_bool! { crate::objdetect::DetectionROI }

	vector_boxed_ref! { crate::objdetect::DetectionROI }

	vector_extern! { BoxedRef<'t, crate::objdetect::DetectionROI>,
		std_vectorLcv_DetectionROIG_new_const, std_vectorLcv_DetectionROIG_delete,
		std_vectorLcv_DetectionROIG_len_const, std_vectorLcv_DetectionROIG_isEmpty_const,
		std_vectorLcv_DetectionROIG_capacity_const, std_vectorLcv_DetectionROIG_shrinkToFit,
		std_vectorLcv_DetectionROIG_reserve_size_t, std_vectorLcv_DetectionROIG_remove_size_t,
		std_vectorLcv_DetectionROIG_swap_size_t_size_t, std_vectorLcv_DetectionROIG_clear,
		std_vectorLcv_DetectionROIG_get_const_size_t, std_vectorLcv_DetectionROIG_set_size_t_const_DetectionROI,
		std_vectorLcv_DetectionROIG_push_const_DetectionROI, std_vectorLcv_DetectionROIG_insert_size_t_const_DetectionROI,
	}


}
pub use objdetect_types::*;

pub use crate::manual::types::*;
