extern "C" {
	const cv::ml::SVMSGD* cv_PtrLcv_ml_SVMSGDG_getInnerPtr_const(const cv::Ptr<cv::ml::SVMSGD>* instance) {
			return instance->get();
	}
	
	cv::ml::SVMSGD* cv_PtrLcv_ml_SVMSGDG_getInnerPtrMut(cv::Ptr<cv::ml::SVMSGD>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ml_SVMSGDG_delete(cv::Ptr<cv::ml::SVMSGD>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ml_SVMSGDG_to_PtrOfAlgorithm(cv::Ptr<cv::ml::SVMSGD>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::ml::StatModel>* cv_PtrLcv_ml_SVMSGDG_to_PtrOfStatModel(cv::Ptr<cv::ml::SVMSGD>* instance) {
			return new cv::Ptr<cv::ml::StatModel>(instance->dynamicCast<cv::ml::StatModel>());
	}
	
}

