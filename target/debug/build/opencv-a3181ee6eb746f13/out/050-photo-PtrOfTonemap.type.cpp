extern "C" {
	const cv::Tonemap* cv_PtrLcv_TonemapG_getInnerPtr_const(const cv::Ptr<cv::Tonemap>* instance) {
			return instance->get();
	}
	
	cv::Tonemap* cv_PtrLcv_TonemapG_getInnerPtrMut(cv::Ptr<cv::Tonemap>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_TonemapG_delete(cv::Ptr<cv::Tonemap>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_TonemapG_to_PtrOfAlgorithm(cv::Ptr<cv::Tonemap>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

