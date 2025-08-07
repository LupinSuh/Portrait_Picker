extern "C" {
	const cv::ml::DTrees* cv_PtrLcv_ml_DTreesG_getInnerPtr_const(const cv::Ptr<cv::ml::DTrees>* instance) {
			return instance->get();
	}
	
	cv::ml::DTrees* cv_PtrLcv_ml_DTreesG_getInnerPtrMut(cv::Ptr<cv::ml::DTrees>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ml_DTreesG_delete(cv::Ptr<cv::ml::DTrees>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ml_DTreesG_to_PtrOfAlgorithm(cv::Ptr<cv::ml::DTrees>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::ml::StatModel>* cv_PtrLcv_ml_DTreesG_to_PtrOfStatModel(cv::Ptr<cv::ml::DTrees>* instance) {
			return new cv::Ptr<cv::ml::StatModel>(instance->dynamicCast<cv::ml::StatModel>());
	}
	
}

