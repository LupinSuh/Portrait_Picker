extern "C" {
	const cv::AlignMTB* cv_PtrLcv_AlignMTBG_getInnerPtr_const(const cv::Ptr<cv::AlignMTB>* instance) {
			return instance->get();
	}
	
	cv::AlignMTB* cv_PtrLcv_AlignMTBG_getInnerPtrMut(cv::Ptr<cv::AlignMTB>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_AlignMTBG_delete(cv::Ptr<cv::AlignMTB>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_AlignMTBG_to_PtrOfAlgorithm(cv::Ptr<cv::AlignMTB>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::AlignExposures>* cv_PtrLcv_AlignMTBG_to_PtrOfAlignExposures(cv::Ptr<cv::AlignMTB>* instance) {
			return new cv::Ptr<cv::AlignExposures>(instance->dynamicCast<cv::AlignExposures>());
	}
	
}

