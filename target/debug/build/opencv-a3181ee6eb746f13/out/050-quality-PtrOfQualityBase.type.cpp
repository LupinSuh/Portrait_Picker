extern "C" {
	const cv::quality::QualityBase* cv_PtrLcv_quality_QualityBaseG_getInnerPtr_const(const cv::Ptr<cv::quality::QualityBase>* instance) {
			return instance->get();
	}
	
	cv::quality::QualityBase* cv_PtrLcv_quality_QualityBaseG_getInnerPtrMut(cv::Ptr<cv::quality::QualityBase>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_quality_QualityBaseG_delete(cv::Ptr<cv::quality::QualityBase>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_quality_QualityBaseG_to_PtrOfAlgorithm(cv::Ptr<cv::quality::QualityBase>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

