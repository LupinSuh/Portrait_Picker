extern "C" {
	const cv::saliency::Saliency* cv_PtrLcv_saliency_SaliencyG_getInnerPtr_const(const cv::Ptr<cv::saliency::Saliency>* instance) {
			return instance->get();
	}
	
	cv::saliency::Saliency* cv_PtrLcv_saliency_SaliencyG_getInnerPtrMut(cv::Ptr<cv::saliency::Saliency>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_saliency_SaliencyG_delete(cv::Ptr<cv::saliency::Saliency>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_saliency_SaliencyG_to_PtrOfAlgorithm(cv::Ptr<cv::saliency::Saliency>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

