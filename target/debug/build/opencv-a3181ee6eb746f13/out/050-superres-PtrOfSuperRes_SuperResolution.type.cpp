extern "C" {
	const cv::superres::SuperResolution* cv_PtrLcv_superres_SuperResolutionG_getInnerPtr_const(const cv::Ptr<cv::superres::SuperResolution>* instance) {
			return instance->get();
	}
	
	cv::superres::SuperResolution* cv_PtrLcv_superres_SuperResolutionG_getInnerPtrMut(cv::Ptr<cv::superres::SuperResolution>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_superres_SuperResolutionG_delete(cv::Ptr<cv::superres::SuperResolution>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_superres_SuperResolutionG_to_PtrOfAlgorithm(cv::Ptr<cv::superres::SuperResolution>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::superres::FrameSource>* cv_PtrLcv_superres_SuperResolutionG_to_PtrOfSuperRes_FrameSource(cv::Ptr<cv::superres::SuperResolution>* instance) {
			return new cv::Ptr<cv::superres::FrameSource>(instance->dynamicCast<cv::superres::FrameSource>());
	}
	
}

