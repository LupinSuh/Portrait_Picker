extern "C" {
	const cv::quality::QualityGMSD* cv_PtrLcv_quality_QualityGMSDG_getInnerPtr_const(const cv::Ptr<cv::quality::QualityGMSD>* instance) {
			return instance->get();
	}
	
	cv::quality::QualityGMSD* cv_PtrLcv_quality_QualityGMSDG_getInnerPtrMut(cv::Ptr<cv::quality::QualityGMSD>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_quality_QualityGMSDG_delete(cv::Ptr<cv::quality::QualityGMSD>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_quality_QualityGMSDG_to_PtrOfAlgorithm(cv::Ptr<cv::quality::QualityGMSD>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::quality::QualityBase>* cv_PtrLcv_quality_QualityGMSDG_to_PtrOfQualityBase(cv::Ptr<cv::quality::QualityGMSD>* instance) {
			return new cv::Ptr<cv::quality::QualityBase>(instance->dynamicCast<cv::quality::QualityBase>());
	}
	
	cv::Ptr<cv::quality::QualityGMSD>* cv_PtrLcv_quality_QualityGMSDG_new_const_QualityGMSD(cv::quality::QualityGMSD* val) {
			return new cv::Ptr<cv::quality::QualityGMSD>(val);
	}
	
}

