extern "C" {
	const cv::ximgproc::ContourFitting* cv_PtrLcv_ximgproc_ContourFittingG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::ContourFitting>* instance) {
			return instance->get();
	}
	
	cv::ximgproc::ContourFitting* cv_PtrLcv_ximgproc_ContourFittingG_getInnerPtrMut(cv::Ptr<cv::ximgproc::ContourFitting>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ximgproc_ContourFittingG_delete(cv::Ptr<cv::ximgproc::ContourFitting>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_ContourFittingG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::ContourFitting>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::ximgproc::ContourFitting>* cv_PtrLcv_ximgproc_ContourFittingG_new_const_ContourFitting(cv::ximgproc::ContourFitting* val) {
			return new cv::Ptr<cv::ximgproc::ContourFitting>(val);
	}
	
}

