extern "C" {
	const cv::quality::QualityMSE* cv_PtrLcv_quality_QualityMSEG_getInnerPtr_const(const cv::Ptr<cv::quality::QualityMSE>* instance) {
			return instance->get();
	}
	
	cv::quality::QualityMSE* cv_PtrLcv_quality_QualityMSEG_getInnerPtrMut(cv::Ptr<cv::quality::QualityMSE>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_quality_QualityMSEG_delete(cv::Ptr<cv::quality::QualityMSE>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_quality_QualityMSEG_to_PtrOfAlgorithm(cv::Ptr<cv::quality::QualityMSE>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::quality::QualityBase>* cv_PtrLcv_quality_QualityMSEG_to_PtrOfQualityBase(cv::Ptr<cv::quality::QualityMSE>* instance) {
			return new cv::Ptr<cv::quality::QualityBase>(instance->dynamicCast<cv::quality::QualityBase>());
	}
	
	cv::Ptr<cv::quality::QualityMSE>* cv_PtrLcv_quality_QualityMSEG_new_const_QualityMSE(cv::quality::QualityMSE* val) {
			return new cv::Ptr<cv::quality::QualityMSE>(val);
	}
	
}

