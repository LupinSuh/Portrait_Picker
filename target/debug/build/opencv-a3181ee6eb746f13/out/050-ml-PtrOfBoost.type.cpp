extern "C" {
	const cv::ml::Boost* cv_PtrLcv_ml_BoostG_getInnerPtr_const(const cv::Ptr<cv::ml::Boost>* instance) {
			return instance->get();
	}
	
	cv::ml::Boost* cv_PtrLcv_ml_BoostG_getInnerPtrMut(cv::Ptr<cv::ml::Boost>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ml_BoostG_delete(cv::Ptr<cv::ml::Boost>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ml_BoostG_to_PtrOfAlgorithm(cv::Ptr<cv::ml::Boost>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::ml::DTrees>* cv_PtrLcv_ml_BoostG_to_PtrOfDTrees(cv::Ptr<cv::ml::Boost>* instance) {
			return new cv::Ptr<cv::ml::DTrees>(instance->dynamicCast<cv::ml::DTrees>());
	}
	
	cv::Ptr<cv::ml::StatModel>* cv_PtrLcv_ml_BoostG_to_PtrOfStatModel(cv::Ptr<cv::ml::Boost>* instance) {
			return new cv::Ptr<cv::ml::StatModel>(instance->dynamicCast<cv::ml::StatModel>());
	}
	
}

