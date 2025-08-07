extern "C" {
	const cv::saliency::StaticSaliency* cv_PtrLcv_saliency_StaticSaliencyG_getInnerPtr_const(const cv::Ptr<cv::saliency::StaticSaliency>* instance) {
			return instance->get();
	}
	
	cv::saliency::StaticSaliency* cv_PtrLcv_saliency_StaticSaliencyG_getInnerPtrMut(cv::Ptr<cv::saliency::StaticSaliency>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_saliency_StaticSaliencyG_delete(cv::Ptr<cv::saliency::StaticSaliency>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_saliency_StaticSaliencyG_to_PtrOfAlgorithm(cv::Ptr<cv::saliency::StaticSaliency>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::saliency::Saliency>* cv_PtrLcv_saliency_StaticSaliencyG_to_PtrOfSaliency(cv::Ptr<cv::saliency::StaticSaliency>* instance) {
			return new cv::Ptr<cv::saliency::Saliency>(instance->dynamicCast<cv::saliency::Saliency>());
	}
	
}

