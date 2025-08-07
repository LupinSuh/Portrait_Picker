extern "C" {
	const cv::ximgproc::SuperpixelSLIC* cv_PtrLcv_ximgproc_SuperpixelSLICG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::SuperpixelSLIC>* instance) {
			return instance->get();
	}
	
	cv::ximgproc::SuperpixelSLIC* cv_PtrLcv_ximgproc_SuperpixelSLICG_getInnerPtrMut(cv::Ptr<cv::ximgproc::SuperpixelSLIC>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ximgproc_SuperpixelSLICG_delete(cv::Ptr<cv::ximgproc::SuperpixelSLIC>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_SuperpixelSLICG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::SuperpixelSLIC>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

