extern "C" {
	const cv::dnn::BackendWrapper* cv_PtrLcv_dnn_BackendWrapperG_getInnerPtr_const(const cv::Ptr<cv::dnn::BackendWrapper>* instance) {
			return instance->get();
	}
	
	cv::dnn::BackendWrapper* cv_PtrLcv_dnn_BackendWrapperG_getInnerPtrMut(cv::Ptr<cv::dnn::BackendWrapper>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_dnn_BackendWrapperG_delete(cv::Ptr<cv::dnn::BackendWrapper>* instance) {
			delete instance;
	}
	
}

