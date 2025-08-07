extern "C" {
	const cv::ximgproc::GuidedFilter* cv_PtrLcv_ximgproc_GuidedFilterG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::GuidedFilter>* instance) {
			return instance->get();
	}
	
	cv::ximgproc::GuidedFilter* cv_PtrLcv_ximgproc_GuidedFilterG_getInnerPtrMut(cv::Ptr<cv::ximgproc::GuidedFilter>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ximgproc_GuidedFilterG_delete(cv::Ptr<cv::ximgproc::GuidedFilter>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_GuidedFilterG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::GuidedFilter>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

