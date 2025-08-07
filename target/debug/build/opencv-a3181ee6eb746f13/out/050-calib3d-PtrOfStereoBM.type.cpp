extern "C" {
	const cv::StereoBM* cv_PtrLcv_StereoBMG_getInnerPtr_const(const cv::Ptr<cv::StereoBM>* instance) {
			return instance->get();
	}
	
	cv::StereoBM* cv_PtrLcv_StereoBMG_getInnerPtrMut(cv::Ptr<cv::StereoBM>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_StereoBMG_delete(cv::Ptr<cv::StereoBM>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_StereoBMG_to_PtrOfAlgorithm(cv::Ptr<cv::StereoBM>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::StereoMatcher>* cv_PtrLcv_StereoBMG_to_PtrOfStereoMatcher(cv::Ptr<cv::StereoBM>* instance) {
			return new cv::Ptr<cv::StereoMatcher>(instance->dynamicCast<cv::StereoMatcher>());
	}
	
}

