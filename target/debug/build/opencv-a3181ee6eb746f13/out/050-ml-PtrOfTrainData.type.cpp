extern "C" {
	const cv::ml::TrainData* cv_PtrLcv_ml_TrainDataG_getInnerPtr_const(const cv::Ptr<cv::ml::TrainData>* instance) {
			return instance->get();
	}
	
	cv::ml::TrainData* cv_PtrLcv_ml_TrainDataG_getInnerPtrMut(cv::Ptr<cv::ml::TrainData>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ml_TrainDataG_delete(cv::Ptr<cv::ml::TrainData>* instance) {
			delete instance;
	}
	
}

