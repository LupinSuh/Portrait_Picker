extern "C" {
	const cv::ximgproc::EdgeAwareInterpolator* cv_PtrLcv_ximgproc_EdgeAwareInterpolatorG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::EdgeAwareInterpolator>* instance) {
			return instance->get();
	}
	
	cv::ximgproc::EdgeAwareInterpolator* cv_PtrLcv_ximgproc_EdgeAwareInterpolatorG_getInnerPtrMut(cv::Ptr<cv::ximgproc::EdgeAwareInterpolator>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ximgproc_EdgeAwareInterpolatorG_delete(cv::Ptr<cv::ximgproc::EdgeAwareInterpolator>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_EdgeAwareInterpolatorG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::EdgeAwareInterpolator>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::ximgproc::SparseMatchInterpolator>* cv_PtrLcv_ximgproc_EdgeAwareInterpolatorG_to_PtrOfSparseMatchInterpolator(cv::Ptr<cv::ximgproc::EdgeAwareInterpolator>* instance) {
			return new cv::Ptr<cv::ximgproc::SparseMatchInterpolator>(instance->dynamicCast<cv::ximgproc::SparseMatchInterpolator>());
	}
	
}

