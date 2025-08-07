extern "C" {
	const cv::ml::RTrees* cv_PtrLcv_ml_RTreesG_getInnerPtr_const(const cv::Ptr<cv::ml::RTrees>* instance) {
			return instance->get();
	}
	
	cv::ml::RTrees* cv_PtrLcv_ml_RTreesG_getInnerPtrMut(cv::Ptr<cv::ml::RTrees>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ml_RTreesG_delete(cv::Ptr<cv::ml::RTrees>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ml_RTreesG_to_PtrOfAlgorithm(cv::Ptr<cv::ml::RTrees>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::ml::DTrees>* cv_PtrLcv_ml_RTreesG_to_PtrOfDTrees(cv::Ptr<cv::ml::RTrees>* instance) {
			return new cv::Ptr<cv::ml::DTrees>(instance->dynamicCast<cv::ml::DTrees>());
	}
	
	cv::Ptr<cv::ml::StatModel>* cv_PtrLcv_ml_RTreesG_to_PtrOfStatModel(cv::Ptr<cv::ml::RTrees>* instance) {
			return new cv::Ptr<cv::ml::StatModel>(instance->dynamicCast<cv::ml::StatModel>());
	}
	
}

