extern "C" {
	const cv::ximgproc::SuperpixelSEEDS* cv_PtrLcv_ximgproc_SuperpixelSEEDSG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::SuperpixelSEEDS>* instance) {
			return instance->get();
	}
	
	cv::ximgproc::SuperpixelSEEDS* cv_PtrLcv_ximgproc_SuperpixelSEEDSG_getInnerPtrMut(cv::Ptr<cv::ximgproc::SuperpixelSEEDS>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ximgproc_SuperpixelSEEDSG_delete(cv::Ptr<cv::ximgproc::SuperpixelSEEDS>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_SuperpixelSEEDSG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::SuperpixelSEEDS>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

