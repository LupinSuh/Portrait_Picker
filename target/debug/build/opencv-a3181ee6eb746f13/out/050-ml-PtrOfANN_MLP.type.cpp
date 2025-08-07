extern "C" {
	const cv::ml::ANN_MLP* cv_PtrLcv_ml_ANN_MLPG_getInnerPtr_const(const cv::Ptr<cv::ml::ANN_MLP>* instance) {
			return instance->get();
	}
	
	cv::ml::ANN_MLP* cv_PtrLcv_ml_ANN_MLPG_getInnerPtrMut(cv::Ptr<cv::ml::ANN_MLP>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ml_ANN_MLPG_delete(cv::Ptr<cv::ml::ANN_MLP>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ml_ANN_MLPG_to_PtrOfAlgorithm(cv::Ptr<cv::ml::ANN_MLP>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::ml::StatModel>* cv_PtrLcv_ml_ANN_MLPG_to_PtrOfStatModel(cv::Ptr<cv::ml::ANN_MLP>* instance) {
			return new cv::Ptr<cv::ml::StatModel>(instance->dynamicCast<cv::ml::StatModel>());
	}
	
}

