extern "C" {
	const cv::text::OCRHolisticWordRecognizer* cv_PtrLcv_text_OCRHolisticWordRecognizerG_getInnerPtr_const(const cv::Ptr<cv::text::OCRHolisticWordRecognizer>* instance) {
			return instance->get();
	}
	
	cv::text::OCRHolisticWordRecognizer* cv_PtrLcv_text_OCRHolisticWordRecognizerG_getInnerPtrMut(cv::Ptr<cv::text::OCRHolisticWordRecognizer>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_text_OCRHolisticWordRecognizerG_delete(cv::Ptr<cv::text::OCRHolisticWordRecognizer>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::text::BaseOCR>* cv_PtrLcv_text_OCRHolisticWordRecognizerG_to_PtrOfBaseOCR(cv::Ptr<cv::text::OCRHolisticWordRecognizer>* instance) {
			return new cv::Ptr<cv::text::BaseOCR>(instance->dynamicCast<cv::text::BaseOCR>());
	}
	
}

