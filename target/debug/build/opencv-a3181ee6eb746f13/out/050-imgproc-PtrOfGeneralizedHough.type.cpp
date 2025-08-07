extern "C" {
	const cv::GeneralizedHough* cv_PtrLcv_GeneralizedHoughG_getInnerPtr_const(const cv::Ptr<cv::GeneralizedHough>* instance) {
			return instance->get();
	}
	
	cv::GeneralizedHough* cv_PtrLcv_GeneralizedHoughG_getInnerPtrMut(cv::Ptr<cv::GeneralizedHough>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_GeneralizedHoughG_delete(cv::Ptr<cv::GeneralizedHough>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_GeneralizedHoughG_to_PtrOfAlgorithm(cv::Ptr<cv::GeneralizedHough>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

