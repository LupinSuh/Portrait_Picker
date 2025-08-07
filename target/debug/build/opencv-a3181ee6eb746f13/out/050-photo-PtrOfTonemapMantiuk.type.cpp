extern "C" {
	const cv::TonemapMantiuk* cv_PtrLcv_TonemapMantiukG_getInnerPtr_const(const cv::Ptr<cv::TonemapMantiuk>* instance) {
			return instance->get();
	}
	
	cv::TonemapMantiuk* cv_PtrLcv_TonemapMantiukG_getInnerPtrMut(cv::Ptr<cv::TonemapMantiuk>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_TonemapMantiukG_delete(cv::Ptr<cv::TonemapMantiuk>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_TonemapMantiukG_to_PtrOfAlgorithm(cv::Ptr<cv::TonemapMantiuk>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::Tonemap>* cv_PtrLcv_TonemapMantiukG_to_PtrOfTonemap(cv::Ptr<cv::TonemapMantiuk>* instance) {
			return new cv::Ptr<cv::Tonemap>(instance->dynamicCast<cv::Tonemap>());
	}
	
}

