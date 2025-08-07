extern "C" {
	const cv::ml::KNearest* cv_PtrLcv_ml_KNearestG_getInnerPtr_const(const cv::Ptr<cv::ml::KNearest>* instance) {
			return instance->get();
	}
	
	cv::ml::KNearest* cv_PtrLcv_ml_KNearestG_getInnerPtrMut(cv::Ptr<cv::ml::KNearest>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ml_KNearestG_delete(cv::Ptr<cv::ml::KNearest>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ml_KNearestG_to_PtrOfAlgorithm(cv::Ptr<cv::ml::KNearest>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::ml::StatModel>* cv_PtrLcv_ml_KNearestG_to_PtrOfStatModel(cv::Ptr<cv::ml::KNearest>* instance) {
			return new cv::Ptr<cv::ml::StatModel>(instance->dynamicCast<cv::ml::StatModel>());
	}
	
}

