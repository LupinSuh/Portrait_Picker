extern "C" {
	const cv::text::TextDetector* cv_PtrLcv_text_TextDetectorG_getInnerPtr_const(const cv::Ptr<cv::text::TextDetector>* instance) {
			return instance->get();
	}
	
	cv::text::TextDetector* cv_PtrLcv_text_TextDetectorG_getInnerPtrMut(cv::Ptr<cv::text::TextDetector>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_text_TextDetectorG_delete(cv::Ptr<cv::text::TextDetector>* instance) {
			delete instance;
	}
	
}

