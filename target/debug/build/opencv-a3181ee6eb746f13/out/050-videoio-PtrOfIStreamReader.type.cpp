extern "C" {
	const cv::IStreamReader* cv_PtrLcv_IStreamReaderG_getInnerPtr_const(const cv::Ptr<cv::IStreamReader>* instance) {
			return instance->get();
	}
	
	cv::IStreamReader* cv_PtrLcv_IStreamReaderG_getInnerPtrMut(cv::Ptr<cv::IStreamReader>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_IStreamReaderG_delete(cv::Ptr<cv::IStreamReader>* instance) {
			delete instance;
	}
	
}

