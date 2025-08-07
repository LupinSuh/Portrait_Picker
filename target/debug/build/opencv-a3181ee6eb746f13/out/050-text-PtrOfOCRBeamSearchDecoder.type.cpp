extern "C" {
	const cv::text::OCRBeamSearchDecoder* cv_PtrLcv_text_OCRBeamSearchDecoderG_getInnerPtr_const(const cv::Ptr<cv::text::OCRBeamSearchDecoder>* instance) {
			return instance->get();
	}
	
	cv::text::OCRBeamSearchDecoder* cv_PtrLcv_text_OCRBeamSearchDecoderG_getInnerPtrMut(cv::Ptr<cv::text::OCRBeamSearchDecoder>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_text_OCRBeamSearchDecoderG_delete(cv::Ptr<cv::text::OCRBeamSearchDecoder>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::text::BaseOCR>* cv_PtrLcv_text_OCRBeamSearchDecoderG_to_PtrOfBaseOCR(cv::Ptr<cv::text::OCRBeamSearchDecoder>* instance) {
			return new cv::Ptr<cv::text::BaseOCR>(instance->dynamicCast<cv::text::BaseOCR>());
	}
	
	cv::Ptr<cv::text::OCRBeamSearchDecoder>* cv_PtrLcv_text_OCRBeamSearchDecoderG_new_const_OCRBeamSearchDecoder(cv::text::OCRBeamSearchDecoder* val) {
			return new cv::Ptr<cv::text::OCRBeamSearchDecoder>(val);
	}
	
}

