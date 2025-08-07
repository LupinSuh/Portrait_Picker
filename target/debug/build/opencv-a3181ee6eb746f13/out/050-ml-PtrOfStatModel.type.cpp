extern "C" {
	const cv::ml::StatModel* cv_PtrLcv_ml_StatModelG_getInnerPtr_const(const cv::Ptr<cv::ml::StatModel>* instance) {
			return instance->get();
	}
	
	cv::ml::StatModel* cv_PtrLcv_ml_StatModelG_getInnerPtrMut(cv::Ptr<cv::ml::StatModel>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ml_StatModelG_delete(cv::Ptr<cv::ml::StatModel>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ml_StatModelG_to_PtrOfAlgorithm(cv::Ptr<cv::ml::StatModel>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

