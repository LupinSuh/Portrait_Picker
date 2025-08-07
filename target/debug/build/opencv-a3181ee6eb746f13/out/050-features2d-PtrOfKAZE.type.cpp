extern "C" {
	const cv::KAZE* cv_PtrLcv_KAZEG_getInnerPtr_const(const cv::Ptr<cv::KAZE>* instance) {
			return instance->get();
	}
	
	cv::KAZE* cv_PtrLcv_KAZEG_getInnerPtrMut(cv::Ptr<cv::KAZE>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_KAZEG_delete(cv::Ptr<cv::KAZE>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_KAZEG_to_PtrOfAlgorithm(cv::Ptr<cv::KAZE>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_KAZEG_to_PtrOfFeature2D(cv::Ptr<cv::KAZE>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
	
}

