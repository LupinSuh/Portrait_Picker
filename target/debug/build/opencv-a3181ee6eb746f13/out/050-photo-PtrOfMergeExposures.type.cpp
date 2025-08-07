extern "C" {
	const cv::MergeExposures* cv_PtrLcv_MergeExposuresG_getInnerPtr_const(const cv::Ptr<cv::MergeExposures>* instance) {
			return instance->get();
	}
	
	cv::MergeExposures* cv_PtrLcv_MergeExposuresG_getInnerPtrMut(cv::Ptr<cv::MergeExposures>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_MergeExposuresG_delete(cv::Ptr<cv::MergeExposures>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_MergeExposuresG_to_PtrOfAlgorithm(cv::Ptr<cv::MergeExposures>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

