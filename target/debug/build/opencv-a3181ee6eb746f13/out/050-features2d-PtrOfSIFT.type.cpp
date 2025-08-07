extern "C" {
	const cv::SIFT* cv_PtrLcv_SIFTG_getInnerPtr_const(const cv::Ptr<cv::SIFT>* instance) {
			return instance->get();
	}
	
	cv::SIFT* cv_PtrLcv_SIFTG_getInnerPtrMut(cv::Ptr<cv::SIFT>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_SIFTG_delete(cv::Ptr<cv::SIFT>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_SIFTG_to_PtrOfAlgorithm(cv::Ptr<cv::SIFT>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_SIFTG_to_PtrOfFeature2D(cv::Ptr<cv::SIFT>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
	
}

