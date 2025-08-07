extern "C" {
	const cv::saliency::MotionSaliency* cv_PtrLcv_saliency_MotionSaliencyG_getInnerPtr_const(const cv::Ptr<cv::saliency::MotionSaliency>* instance) {
			return instance->get();
	}
	
	cv::saliency::MotionSaliency* cv_PtrLcv_saliency_MotionSaliencyG_getInnerPtrMut(cv::Ptr<cv::saliency::MotionSaliency>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_saliency_MotionSaliencyG_delete(cv::Ptr<cv::saliency::MotionSaliency>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_saliency_MotionSaliencyG_to_PtrOfAlgorithm(cv::Ptr<cv::saliency::MotionSaliency>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::saliency::Saliency>* cv_PtrLcv_saliency_MotionSaliencyG_to_PtrOfSaliency(cv::Ptr<cv::saliency::MotionSaliency>* instance) {
			return new cv::Ptr<cv::saliency::Saliency>(instance->dynamicCast<cv::saliency::Saliency>());
	}
	
}

