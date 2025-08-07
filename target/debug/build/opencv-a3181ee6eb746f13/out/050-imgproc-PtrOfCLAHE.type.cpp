extern "C" {
	const cv::CLAHE* cv_PtrLcv_CLAHEG_getInnerPtr_const(const cv::Ptr<cv::CLAHE>* instance) {
			return instance->get();
	}
	
	cv::CLAHE* cv_PtrLcv_CLAHEG_getInnerPtrMut(cv::Ptr<cv::CLAHE>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_CLAHEG_delete(cv::Ptr<cv::CLAHE>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_CLAHEG_to_PtrOfAlgorithm(cv::Ptr<cv::CLAHE>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

