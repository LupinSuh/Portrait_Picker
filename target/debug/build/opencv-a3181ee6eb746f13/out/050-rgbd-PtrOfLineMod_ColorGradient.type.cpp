extern "C" {
	const cv::linemod::ColorGradient* cv_PtrLcv_linemod_ColorGradientG_getInnerPtr_const(const cv::Ptr<cv::linemod::ColorGradient>* instance) {
			return instance->get();
	}
	
	cv::linemod::ColorGradient* cv_PtrLcv_linemod_ColorGradientG_getInnerPtrMut(cv::Ptr<cv::linemod::ColorGradient>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_linemod_ColorGradientG_delete(cv::Ptr<cv::linemod::ColorGradient>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::linemod::Modality>* cv_PtrLcv_linemod_ColorGradientG_to_PtrOfLineMod_Modality(cv::Ptr<cv::linemod::ColorGradient>* instance) {
			return new cv::Ptr<cv::linemod::Modality>(instance->dynamicCast<cv::linemod::Modality>());
	}
	
	cv::Ptr<cv::linemod::ColorGradient>* cv_PtrLcv_linemod_ColorGradientG_new_const_ColorGradient(cv::linemod::ColorGradient* val) {
			return new cv::Ptr<cv::linemod::ColorGradient>(val);
	}
	
}

