extern "C" {
	const cv::AlignExposures* cv_PtrLcv_AlignExposuresG_getInnerPtr_const(const cv::Ptr<cv::AlignExposures>* instance) {
			return instance->get();
	}
	
	cv::AlignExposures* cv_PtrLcv_AlignExposuresG_getInnerPtrMut(cv::Ptr<cv::AlignExposures>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_AlignExposuresG_delete(cv::Ptr<cv::AlignExposures>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_AlignExposuresG_to_PtrOfAlgorithm(cv::Ptr<cv::AlignExposures>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

