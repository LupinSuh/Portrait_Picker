extern "C" {
	const cv::saliency::StaticSaliencySpectralResidual* cv_PtrLcv_saliency_StaticSaliencySpectralResidualG_getInnerPtr_const(const cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>* instance) {
			return instance->get();
	}
	
	cv::saliency::StaticSaliencySpectralResidual* cv_PtrLcv_saliency_StaticSaliencySpectralResidualG_getInnerPtrMut(cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_saliency_StaticSaliencySpectralResidualG_delete(cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_saliency_StaticSaliencySpectralResidualG_to_PtrOfAlgorithm(cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::saliency::Saliency>* cv_PtrLcv_saliency_StaticSaliencySpectralResidualG_to_PtrOfSaliency(cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>* instance) {
			return new cv::Ptr<cv::saliency::Saliency>(instance->dynamicCast<cv::saliency::Saliency>());
	}
	
	cv::Ptr<cv::saliency::StaticSaliency>* cv_PtrLcv_saliency_StaticSaliencySpectralResidualG_to_PtrOfStaticSaliency(cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>* instance) {
			return new cv::Ptr<cv::saliency::StaticSaliency>(instance->dynamicCast<cv::saliency::StaticSaliency>());
	}
	
	cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>* cv_PtrLcv_saliency_StaticSaliencySpectralResidualG_new_const_StaticSaliencySpectralResidual(cv::saliency::StaticSaliencySpectralResidual* val) {
			return new cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>(val);
	}
	
}

