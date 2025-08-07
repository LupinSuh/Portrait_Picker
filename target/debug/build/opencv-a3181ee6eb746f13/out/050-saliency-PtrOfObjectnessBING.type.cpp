extern "C" {
	const cv::saliency::ObjectnessBING* cv_PtrLcv_saliency_ObjectnessBINGG_getInnerPtr_const(const cv::Ptr<cv::saliency::ObjectnessBING>* instance) {
			return instance->get();
	}
	
	cv::saliency::ObjectnessBING* cv_PtrLcv_saliency_ObjectnessBINGG_getInnerPtrMut(cv::Ptr<cv::saliency::ObjectnessBING>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_saliency_ObjectnessBINGG_delete(cv::Ptr<cv::saliency::ObjectnessBING>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_saliency_ObjectnessBINGG_to_PtrOfAlgorithm(cv::Ptr<cv::saliency::ObjectnessBING>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::saliency::Objectness>* cv_PtrLcv_saliency_ObjectnessBINGG_to_PtrOfObjectness(cv::Ptr<cv::saliency::ObjectnessBING>* instance) {
			return new cv::Ptr<cv::saliency::Objectness>(instance->dynamicCast<cv::saliency::Objectness>());
	}
	
	cv::Ptr<cv::saliency::Saliency>* cv_PtrLcv_saliency_ObjectnessBINGG_to_PtrOfSaliency(cv::Ptr<cv::saliency::ObjectnessBING>* instance) {
			return new cv::Ptr<cv::saliency::Saliency>(instance->dynamicCast<cv::saliency::Saliency>());
	}
	
	cv::Ptr<cv::saliency::ObjectnessBING>* cv_PtrLcv_saliency_ObjectnessBINGG_new_const_ObjectnessBING(cv::saliency::ObjectnessBING* val) {
			return new cv::Ptr<cv::saliency::ObjectnessBING>(val);
	}
	
}

