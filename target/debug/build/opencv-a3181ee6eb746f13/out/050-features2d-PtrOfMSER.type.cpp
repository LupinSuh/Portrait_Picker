extern "C" {
	const cv::MSER* cv_PtrLcv_MSERG_getInnerPtr_const(const cv::Ptr<cv::MSER>* instance) {
			return instance->get();
	}
	
	cv::MSER* cv_PtrLcv_MSERG_getInnerPtrMut(cv::Ptr<cv::MSER>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_MSERG_delete(cv::Ptr<cv::MSER>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_MSERG_to_PtrOfAlgorithm(cv::Ptr<cv::MSER>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_MSERG_to_PtrOfFeature2D(cv::Ptr<cv::MSER>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
	
}

