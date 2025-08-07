extern "C" {
	const cv::ml::SVM* cv_PtrLcv_ml_SVMG_getInnerPtr_const(const cv::Ptr<cv::ml::SVM>* instance) {
			return instance->get();
	}
	
	cv::ml::SVM* cv_PtrLcv_ml_SVMG_getInnerPtrMut(cv::Ptr<cv::ml::SVM>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ml_SVMG_delete(cv::Ptr<cv::ml::SVM>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ml_SVMG_to_PtrOfAlgorithm(cv::Ptr<cv::ml::SVM>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::ml::StatModel>* cv_PtrLcv_ml_SVMG_to_PtrOfStatModel(cv::Ptr<cv::ml::SVM>* instance) {
			return new cv::Ptr<cv::ml::StatModel>(instance->dynamicCast<cv::ml::StatModel>());
	}
	
}

