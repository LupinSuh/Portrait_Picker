extern "C" {
	const cv::MergeRobertson* cv_PtrLcv_MergeRobertsonG_getInnerPtr_const(const cv::Ptr<cv::MergeRobertson>* instance) {
			return instance->get();
	}
	
	cv::MergeRobertson* cv_PtrLcv_MergeRobertsonG_getInnerPtrMut(cv::Ptr<cv::MergeRobertson>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_MergeRobertsonG_delete(cv::Ptr<cv::MergeRobertson>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_MergeRobertsonG_to_PtrOfAlgorithm(cv::Ptr<cv::MergeRobertson>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::MergeExposures>* cv_PtrLcv_MergeRobertsonG_to_PtrOfMergeExposures(cv::Ptr<cv::MergeRobertson>* instance) {
			return new cv::Ptr<cv::MergeExposures>(instance->dynamicCast<cv::MergeExposures>());
	}
	
}

