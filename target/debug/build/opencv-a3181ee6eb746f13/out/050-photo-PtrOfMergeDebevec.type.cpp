extern "C" {
	const cv::MergeDebevec* cv_PtrLcv_MergeDebevecG_getInnerPtr_const(const cv::Ptr<cv::MergeDebevec>* instance) {
			return instance->get();
	}
	
	cv::MergeDebevec* cv_PtrLcv_MergeDebevecG_getInnerPtrMut(cv::Ptr<cv::MergeDebevec>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_MergeDebevecG_delete(cv::Ptr<cv::MergeDebevec>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_MergeDebevecG_to_PtrOfAlgorithm(cv::Ptr<cv::MergeDebevec>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::MergeExposures>* cv_PtrLcv_MergeDebevecG_to_PtrOfMergeExposures(cv::Ptr<cv::MergeDebevec>* instance) {
			return new cv::Ptr<cv::MergeExposures>(instance->dynamicCast<cv::MergeExposures>());
	}
	
}

