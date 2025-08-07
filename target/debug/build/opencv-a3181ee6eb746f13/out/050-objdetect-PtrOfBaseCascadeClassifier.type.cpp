extern "C" {
	const cv::BaseCascadeClassifier* cv_PtrLcv_BaseCascadeClassifierG_getInnerPtr_const(const cv::Ptr<cv::BaseCascadeClassifier>* instance) {
			return instance->get();
	}
	
	cv::BaseCascadeClassifier* cv_PtrLcv_BaseCascadeClassifierG_getInnerPtrMut(cv::Ptr<cv::BaseCascadeClassifier>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_BaseCascadeClassifierG_delete(cv::Ptr<cv::BaseCascadeClassifier>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_BaseCascadeClassifierG_to_PtrOfAlgorithm(cv::Ptr<cv::BaseCascadeClassifier>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

