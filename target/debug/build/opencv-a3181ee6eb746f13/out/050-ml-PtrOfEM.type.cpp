extern "C" {
	const cv::ml::EM* cv_PtrLcv_ml_EMG_getInnerPtr_const(const cv::Ptr<cv::ml::EM>* instance) {
			return instance->get();
	}
	
	cv::ml::EM* cv_PtrLcv_ml_EMG_getInnerPtrMut(cv::Ptr<cv::ml::EM>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ml_EMG_delete(cv::Ptr<cv::ml::EM>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ml_EMG_to_PtrOfAlgorithm(cv::Ptr<cv::ml::EM>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::ml::StatModel>* cv_PtrLcv_ml_EMG_to_PtrOfStatModel(cv::Ptr<cv::ml::EM>* instance) {
			return new cv::Ptr<cv::ml::StatModel>(instance->dynamicCast<cv::ml::StatModel>());
	}
	
}

