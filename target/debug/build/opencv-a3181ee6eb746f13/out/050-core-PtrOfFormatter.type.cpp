extern "C" {
	const cv::Formatter* cv_PtrLcv_FormatterG_getInnerPtr_const(const cv::Ptr<cv::Formatter>* instance) {
			return instance->get();
	}
	
	cv::Formatter* cv_PtrLcv_FormatterG_getInnerPtrMut(cv::Ptr<cv::Formatter>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_FormatterG_delete(cv::Ptr<cv::Formatter>* instance) {
			delete instance;
	}
	
}

