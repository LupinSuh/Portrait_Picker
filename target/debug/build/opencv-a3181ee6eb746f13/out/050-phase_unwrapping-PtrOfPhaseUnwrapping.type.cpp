extern "C" {
	const cv::phase_unwrapping::PhaseUnwrapping* cv_PtrLcv_phase_unwrapping_PhaseUnwrappingG_getInnerPtr_const(const cv::Ptr<cv::phase_unwrapping::PhaseUnwrapping>* instance) {
			return instance->get();
	}
	
	cv::phase_unwrapping::PhaseUnwrapping* cv_PtrLcv_phase_unwrapping_PhaseUnwrappingG_getInnerPtrMut(cv::Ptr<cv::phase_unwrapping::PhaseUnwrapping>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_phase_unwrapping_PhaseUnwrappingG_delete(cv::Ptr<cv::phase_unwrapping::PhaseUnwrapping>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_phase_unwrapping_PhaseUnwrappingG_to_PtrOfAlgorithm(cv::Ptr<cv::phase_unwrapping::PhaseUnwrapping>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

