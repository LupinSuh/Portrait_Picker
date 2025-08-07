extern "C" {
	const cv::MergeMertens* cv_PtrLcv_MergeMertensG_getInnerPtr_const(const cv::Ptr<cv::MergeMertens>* instance) {
			return instance->get();
	}
	
	cv::MergeMertens* cv_PtrLcv_MergeMertensG_getInnerPtrMut(cv::Ptr<cv::MergeMertens>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_MergeMertensG_delete(cv::Ptr<cv::MergeMertens>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_MergeMertensG_to_PtrOfAlgorithm(cv::Ptr<cv::MergeMertens>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::MergeExposures>* cv_PtrLcv_MergeMertensG_to_PtrOfMergeExposures(cv::Ptr<cv::MergeMertens>* instance) {
			return new cv::Ptr<cv::MergeExposures>(instance->dynamicCast<cv::MergeExposures>());
	}
	
}

