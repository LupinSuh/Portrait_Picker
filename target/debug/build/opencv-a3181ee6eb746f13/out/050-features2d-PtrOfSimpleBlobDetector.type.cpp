extern "C" {
	const cv::SimpleBlobDetector* cv_PtrLcv_SimpleBlobDetectorG_getInnerPtr_const(const cv::Ptr<cv::SimpleBlobDetector>* instance) {
			return instance->get();
	}
	
	cv::SimpleBlobDetector* cv_PtrLcv_SimpleBlobDetectorG_getInnerPtrMut(cv::Ptr<cv::SimpleBlobDetector>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_SimpleBlobDetectorG_delete(cv::Ptr<cv::SimpleBlobDetector>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_SimpleBlobDetectorG_to_PtrOfAlgorithm(cv::Ptr<cv::SimpleBlobDetector>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_SimpleBlobDetectorG_to_PtrOfFeature2D(cv::Ptr<cv::SimpleBlobDetector>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
	
}

