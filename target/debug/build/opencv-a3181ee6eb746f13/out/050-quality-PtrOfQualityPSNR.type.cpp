extern "C" {
	const cv::quality::QualityPSNR* cv_PtrLcv_quality_QualityPSNRG_getInnerPtr_const(const cv::Ptr<cv::quality::QualityPSNR>* instance) {
			return instance->get();
	}
	
	cv::quality::QualityPSNR* cv_PtrLcv_quality_QualityPSNRG_getInnerPtrMut(cv::Ptr<cv::quality::QualityPSNR>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_quality_QualityPSNRG_delete(cv::Ptr<cv::quality::QualityPSNR>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_quality_QualityPSNRG_to_PtrOfAlgorithm(cv::Ptr<cv::quality::QualityPSNR>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::quality::QualityBase>* cv_PtrLcv_quality_QualityPSNRG_to_PtrOfQualityBase(cv::Ptr<cv::quality::QualityPSNR>* instance) {
			return new cv::Ptr<cv::quality::QualityBase>(instance->dynamicCast<cv::quality::QualityBase>());
	}
	
	cv::Ptr<cv::quality::QualityPSNR>* cv_PtrLcv_quality_QualityPSNRG_new_const_QualityPSNR(cv::quality::QualityPSNR* val) {
			return new cv::Ptr<cv::quality::QualityPSNR>(val);
	}
	
}

