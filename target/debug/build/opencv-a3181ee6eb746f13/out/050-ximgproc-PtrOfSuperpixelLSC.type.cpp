extern "C" {
	const cv::ximgproc::SuperpixelLSC* cv_PtrLcv_ximgproc_SuperpixelLSCG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::SuperpixelLSC>* instance) {
			return instance->get();
	}
	
	cv::ximgproc::SuperpixelLSC* cv_PtrLcv_ximgproc_SuperpixelLSCG_getInnerPtrMut(cv::Ptr<cv::ximgproc::SuperpixelLSC>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ximgproc_SuperpixelLSCG_delete(cv::Ptr<cv::ximgproc::SuperpixelLSC>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_SuperpixelLSCG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::SuperpixelLSC>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

