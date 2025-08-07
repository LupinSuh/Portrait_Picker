extern "C" {
	const cv::Formatted* cv_PtrLcv_FormattedG_getInnerPtr_const(const cv::Ptr<cv::Formatted>* instance) {
			return instance->get();
	}
	
	cv::Formatted* cv_PtrLcv_FormattedG_getInnerPtrMut(cv::Ptr<cv::Formatted>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_FormattedG_delete(cv::Ptr<cv::Formatted>* instance) {
			delete instance;
	}
	
}

