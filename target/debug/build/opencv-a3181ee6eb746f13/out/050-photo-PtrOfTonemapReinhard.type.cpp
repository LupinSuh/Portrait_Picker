extern "C" {
	const cv::TonemapReinhard* cv_PtrLcv_TonemapReinhardG_getInnerPtr_const(const cv::Ptr<cv::TonemapReinhard>* instance) {
			return instance->get();
	}
	
	cv::TonemapReinhard* cv_PtrLcv_TonemapReinhardG_getInnerPtrMut(cv::Ptr<cv::TonemapReinhard>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_TonemapReinhardG_delete(cv::Ptr<cv::TonemapReinhard>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_TonemapReinhardG_to_PtrOfAlgorithm(cv::Ptr<cv::TonemapReinhard>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::Tonemap>* cv_PtrLcv_TonemapReinhardG_to_PtrOfTonemap(cv::Ptr<cv::TonemapReinhard>* instance) {
			return new cv::Ptr<cv::Tonemap>(instance->dynamicCast<cv::Tonemap>());
	}
	
}

