extern "C" {
	const cv::saliency::MotionSaliencyBinWangApr2014* cv_PtrLcv_saliency_MotionSaliencyBinWangApr2014G_getInnerPtr_const(const cv::Ptr<cv::saliency::MotionSaliencyBinWangApr2014>* instance) {
			return instance->get();
	}
	
	cv::saliency::MotionSaliencyBinWangApr2014* cv_PtrLcv_saliency_MotionSaliencyBinWangApr2014G_getInnerPtrMut(cv::Ptr<cv::saliency::MotionSaliencyBinWangApr2014>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_saliency_MotionSaliencyBinWangApr2014G_delete(cv::Ptr<cv::saliency::MotionSaliencyBinWangApr2014>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_saliency_MotionSaliencyBinWangApr2014G_to_PtrOfAlgorithm(cv::Ptr<cv::saliency::MotionSaliencyBinWangApr2014>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::saliency::MotionSaliency>* cv_PtrLcv_saliency_MotionSaliencyBinWangApr2014G_to_PtrOfMotionSaliency(cv::Ptr<cv::saliency::MotionSaliencyBinWangApr2014>* instance) {
			return new cv::Ptr<cv::saliency::MotionSaliency>(instance->dynamicCast<cv::saliency::MotionSaliency>());
	}
	
	cv::Ptr<cv::saliency::Saliency>* cv_PtrLcv_saliency_MotionSaliencyBinWangApr2014G_to_PtrOfSaliency(cv::Ptr<cv::saliency::MotionSaliencyBinWangApr2014>* instance) {
			return new cv::Ptr<cv::saliency::Saliency>(instance->dynamicCast<cv::saliency::Saliency>());
	}
	
	cv::Ptr<cv::saliency::MotionSaliencyBinWangApr2014>* cv_PtrLcv_saliency_MotionSaliencyBinWangApr2014G_new_const_MotionSaliencyBinWangApr2014(cv::saliency::MotionSaliencyBinWangApr2014* val) {
			return new cv::Ptr<cv::saliency::MotionSaliencyBinWangApr2014>(val);
	}
	
}

