extern "C" {
	const cv::ximgproc::RICInterpolator* cv_PtrLcv_ximgproc_RICInterpolatorG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::RICInterpolator>* instance) {
			return instance->get();
	}
	
	cv::ximgproc::RICInterpolator* cv_PtrLcv_ximgproc_RICInterpolatorG_getInnerPtrMut(cv::Ptr<cv::ximgproc::RICInterpolator>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ximgproc_RICInterpolatorG_delete(cv::Ptr<cv::ximgproc::RICInterpolator>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_RICInterpolatorG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::RICInterpolator>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::ximgproc::SparseMatchInterpolator>* cv_PtrLcv_ximgproc_RICInterpolatorG_to_PtrOfSparseMatchInterpolator(cv::Ptr<cv::ximgproc::RICInterpolator>* instance) {
			return new cv::Ptr<cv::ximgproc::SparseMatchInterpolator>(instance->dynamicCast<cv::ximgproc::SparseMatchInterpolator>());
	}
	
}

