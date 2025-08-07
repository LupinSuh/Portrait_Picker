extern "C" {
	const cv::ximgproc::SparseMatchInterpolator* cv_PtrLcv_ximgproc_SparseMatchInterpolatorG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::SparseMatchInterpolator>* instance) {
			return instance->get();
	}
	
	cv::ximgproc::SparseMatchInterpolator* cv_PtrLcv_ximgproc_SparseMatchInterpolatorG_getInnerPtrMut(cv::Ptr<cv::ximgproc::SparseMatchInterpolator>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ximgproc_SparseMatchInterpolatorG_delete(cv::Ptr<cv::ximgproc::SparseMatchInterpolator>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_SparseMatchInterpolatorG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::SparseMatchInterpolator>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

