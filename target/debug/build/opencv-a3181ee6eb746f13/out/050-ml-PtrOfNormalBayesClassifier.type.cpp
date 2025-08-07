extern "C" {
	const cv::ml::NormalBayesClassifier* cv_PtrLcv_ml_NormalBayesClassifierG_getInnerPtr_const(const cv::Ptr<cv::ml::NormalBayesClassifier>* instance) {
			return instance->get();
	}
	
	cv::ml::NormalBayesClassifier* cv_PtrLcv_ml_NormalBayesClassifierG_getInnerPtrMut(cv::Ptr<cv::ml::NormalBayesClassifier>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ml_NormalBayesClassifierG_delete(cv::Ptr<cv::ml::NormalBayesClassifier>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ml_NormalBayesClassifierG_to_PtrOfAlgorithm(cv::Ptr<cv::ml::NormalBayesClassifier>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::ml::StatModel>* cv_PtrLcv_ml_NormalBayesClassifierG_to_PtrOfStatModel(cv::Ptr<cv::ml::NormalBayesClassifier>* instance) {
			return new cv::Ptr<cv::ml::StatModel>(instance->dynamicCast<cv::ml::StatModel>());
	}
	
}

