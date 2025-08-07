extern "C" {
	const cv::ml::LogisticRegression* cv_PtrLcv_ml_LogisticRegressionG_getInnerPtr_const(const cv::Ptr<cv::ml::LogisticRegression>* instance) {
			return instance->get();
	}
	
	cv::ml::LogisticRegression* cv_PtrLcv_ml_LogisticRegressionG_getInnerPtrMut(cv::Ptr<cv::ml::LogisticRegression>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ml_LogisticRegressionG_delete(cv::Ptr<cv::ml::LogisticRegression>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ml_LogisticRegressionG_to_PtrOfAlgorithm(cv::Ptr<cv::ml::LogisticRegression>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::ml::StatModel>* cv_PtrLcv_ml_LogisticRegressionG_to_PtrOfStatModel(cv::Ptr<cv::ml::LogisticRegression>* instance) {
			return new cv::Ptr<cv::ml::StatModel>(instance->dynamicCast<cv::ml::StatModel>());
	}
	
}

