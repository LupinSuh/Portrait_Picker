extern "C" {
	const cv::saliency::StaticSaliencyFineGrained* cv_PtrLcv_saliency_StaticSaliencyFineGrainedG_getInnerPtr_const(const cv::Ptr<cv::saliency::StaticSaliencyFineGrained>* instance) {
			return instance->get();
	}
	
	cv::saliency::StaticSaliencyFineGrained* cv_PtrLcv_saliency_StaticSaliencyFineGrainedG_getInnerPtrMut(cv::Ptr<cv::saliency::StaticSaliencyFineGrained>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_saliency_StaticSaliencyFineGrainedG_delete(cv::Ptr<cv::saliency::StaticSaliencyFineGrained>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_saliency_StaticSaliencyFineGrainedG_to_PtrOfAlgorithm(cv::Ptr<cv::saliency::StaticSaliencyFineGrained>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::saliency::Saliency>* cv_PtrLcv_saliency_StaticSaliencyFineGrainedG_to_PtrOfSaliency(cv::Ptr<cv::saliency::StaticSaliencyFineGrained>* instance) {
			return new cv::Ptr<cv::saliency::Saliency>(instance->dynamicCast<cv::saliency::Saliency>());
	}
	
	cv::Ptr<cv::saliency::StaticSaliency>* cv_PtrLcv_saliency_StaticSaliencyFineGrainedG_to_PtrOfStaticSaliency(cv::Ptr<cv::saliency::StaticSaliencyFineGrained>* instance) {
			return new cv::Ptr<cv::saliency::StaticSaliency>(instance->dynamicCast<cv::saliency::StaticSaliency>());
	}
	
	cv::Ptr<cv::saliency::StaticSaliencyFineGrained>* cv_PtrLcv_saliency_StaticSaliencyFineGrainedG_new_const_StaticSaliencyFineGrained(cv::saliency::StaticSaliencyFineGrained* val) {
			return new cv::Ptr<cv::saliency::StaticSaliencyFineGrained>(val);
	}
	
}

