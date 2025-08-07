extern "C" {
	const cv::text::TextDetectorCNN* cv_PtrLcv_text_TextDetectorCNNG_getInnerPtr_const(const cv::Ptr<cv::text::TextDetectorCNN>* instance) {
			return instance->get();
	}
	
	cv::text::TextDetectorCNN* cv_PtrLcv_text_TextDetectorCNNG_getInnerPtrMut(cv::Ptr<cv::text::TextDetectorCNN>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_text_TextDetectorCNNG_delete(cv::Ptr<cv::text::TextDetectorCNN>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::text::TextDetector>* cv_PtrLcv_text_TextDetectorCNNG_to_PtrOfTextDetector(cv::Ptr<cv::text::TextDetectorCNN>* instance) {
			return new cv::Ptr<cv::text::TextDetector>(instance->dynamicCast<cv::text::TextDetector>());
	}
	
}

