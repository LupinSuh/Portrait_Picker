extern "C" {
	const cv::text::OCRTesseract* cv_PtrLcv_text_OCRTesseractG_getInnerPtr_const(const cv::Ptr<cv::text::OCRTesseract>* instance) {
			return instance->get();
	}
	
	cv::text::OCRTesseract* cv_PtrLcv_text_OCRTesseractG_getInnerPtrMut(cv::Ptr<cv::text::OCRTesseract>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_text_OCRTesseractG_delete(cv::Ptr<cv::text::OCRTesseract>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::text::BaseOCR>* cv_PtrLcv_text_OCRTesseractG_to_PtrOfBaseOCR(cv::Ptr<cv::text::OCRTesseract>* instance) {
			return new cv::Ptr<cv::text::BaseOCR>(instance->dynamicCast<cv::text::BaseOCR>());
	}
	
}

