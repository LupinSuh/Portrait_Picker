extern "C" {
	const cv::text::OCRHMMDecoder* cv_PtrLcv_text_OCRHMMDecoderG_getInnerPtr_const(const cv::Ptr<cv::text::OCRHMMDecoder>* instance) {
			return instance->get();
	}
	
	cv::text::OCRHMMDecoder* cv_PtrLcv_text_OCRHMMDecoderG_getInnerPtrMut(cv::Ptr<cv::text::OCRHMMDecoder>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_text_OCRHMMDecoderG_delete(cv::Ptr<cv::text::OCRHMMDecoder>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::text::BaseOCR>* cv_PtrLcv_text_OCRHMMDecoderG_to_PtrOfBaseOCR(cv::Ptr<cv::text::OCRHMMDecoder>* instance) {
			return new cv::Ptr<cv::text::BaseOCR>(instance->dynamicCast<cv::text::BaseOCR>());
	}
	
	cv::Ptr<cv::text::OCRHMMDecoder>* cv_PtrLcv_text_OCRHMMDecoderG_new_const_OCRHMMDecoder(cv::text::OCRHMMDecoder* val) {
			return new cv::Ptr<cv::text::OCRHMMDecoder>(val);
	}
	
}

