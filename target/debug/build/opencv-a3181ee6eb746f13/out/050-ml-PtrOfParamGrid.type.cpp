extern "C" {
	const cv::ml::ParamGrid* cv_PtrLcv_ml_ParamGridG_getInnerPtr_const(const cv::Ptr<cv::ml::ParamGrid>* instance) {
			return instance->get();
	}
	
	cv::ml::ParamGrid* cv_PtrLcv_ml_ParamGridG_getInnerPtrMut(cv::Ptr<cv::ml::ParamGrid>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ml_ParamGridG_delete(cv::Ptr<cv::ml::ParamGrid>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::ml::ParamGrid>* cv_PtrLcv_ml_ParamGridG_new_const_ParamGrid(cv::ml::ParamGrid* val) {
			return new cv::Ptr<cv::ml::ParamGrid>(val);
	}
	
}

