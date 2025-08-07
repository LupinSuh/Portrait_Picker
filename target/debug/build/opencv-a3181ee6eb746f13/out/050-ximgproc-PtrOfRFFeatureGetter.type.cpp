extern "C" {
	const cv::ximgproc::RFFeatureGetter* cv_PtrLcv_ximgproc_RFFeatureGetterG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::RFFeatureGetter>* instance) {
			return instance->get();
	}
	
	cv::ximgproc::RFFeatureGetter* cv_PtrLcv_ximgproc_RFFeatureGetterG_getInnerPtrMut(cv::Ptr<cv::ximgproc::RFFeatureGetter>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ximgproc_RFFeatureGetterG_delete(cv::Ptr<cv::ximgproc::RFFeatureGetter>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_RFFeatureGetterG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::RFFeatureGetter>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

