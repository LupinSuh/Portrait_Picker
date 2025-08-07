extern "C" {
	const cv::quality::QualitySSIM* cv_PtrLcv_quality_QualitySSIMG_getInnerPtr_const(const cv::Ptr<cv::quality::QualitySSIM>* instance) {
			return instance->get();
	}
	
	cv::quality::QualitySSIM* cv_PtrLcv_quality_QualitySSIMG_getInnerPtrMut(cv::Ptr<cv::quality::QualitySSIM>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_quality_QualitySSIMG_delete(cv::Ptr<cv::quality::QualitySSIM>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_quality_QualitySSIMG_to_PtrOfAlgorithm(cv::Ptr<cv::quality::QualitySSIM>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::quality::QualityBase>* cv_PtrLcv_quality_QualitySSIMG_to_PtrOfQualityBase(cv::Ptr<cv::quality::QualitySSIM>* instance) {
			return new cv::Ptr<cv::quality::QualityBase>(instance->dynamicCast<cv::quality::QualityBase>());
	}
	
	cv::Ptr<cv::quality::QualitySSIM>* cv_PtrLcv_quality_QualitySSIMG_new_const_QualitySSIM(cv::quality::QualitySSIM* val) {
			return new cv::Ptr<cv::quality::QualitySSIM>(val);
	}
	
}

