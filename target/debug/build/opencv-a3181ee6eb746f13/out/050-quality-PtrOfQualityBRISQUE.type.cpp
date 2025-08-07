extern "C" {
	const cv::quality::QualityBRISQUE* cv_PtrLcv_quality_QualityBRISQUEG_getInnerPtr_const(const cv::Ptr<cv::quality::QualityBRISQUE>* instance) {
			return instance->get();
	}
	
	cv::quality::QualityBRISQUE* cv_PtrLcv_quality_QualityBRISQUEG_getInnerPtrMut(cv::Ptr<cv::quality::QualityBRISQUE>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_quality_QualityBRISQUEG_delete(cv::Ptr<cv::quality::QualityBRISQUE>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_quality_QualityBRISQUEG_to_PtrOfAlgorithm(cv::Ptr<cv::quality::QualityBRISQUE>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::quality::QualityBase>* cv_PtrLcv_quality_QualityBRISQUEG_to_PtrOfQualityBase(cv::Ptr<cv::quality::QualityBRISQUE>* instance) {
			return new cv::Ptr<cv::quality::QualityBase>(instance->dynamicCast<cv::quality::QualityBase>());
	}
	
	cv::Ptr<cv::quality::QualityBRISQUE>* cv_PtrLcv_quality_QualityBRISQUEG_new_const_QualityBRISQUE(cv::quality::QualityBRISQUE* val) {
			return new cv::Ptr<cv::quality::QualityBRISQUE>(val);
	}
	
}

