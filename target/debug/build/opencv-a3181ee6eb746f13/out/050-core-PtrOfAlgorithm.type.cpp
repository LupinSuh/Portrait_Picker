extern "C" {
	const cv::Algorithm* cv_PtrLcv_AlgorithmG_getInnerPtr_const(const cv::Ptr<cv::Algorithm>* instance) {
			return instance->get();
	}
	
	cv::Algorithm* cv_PtrLcv_AlgorithmG_getInnerPtrMut(cv::Ptr<cv::Algorithm>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_AlgorithmG_delete(cv::Ptr<cv::Algorithm>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_AlgorithmG_new_const_Algorithm(cv::Algorithm* val) {
			return new cv::Ptr<cv::Algorithm>(val);
	}
	
}

