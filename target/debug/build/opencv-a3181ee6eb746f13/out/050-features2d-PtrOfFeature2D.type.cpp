extern "C" {
	const cv::Feature2D* cv_PtrLcv_Feature2DG_getInnerPtr_const(const cv::Ptr<cv::Feature2D>* instance) {
			return instance->get();
	}
	
	cv::Feature2D* cv_PtrLcv_Feature2DG_getInnerPtrMut(cv::Ptr<cv::Feature2D>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_Feature2DG_delete(cv::Ptr<cv::Feature2D>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_Feature2DG_to_PtrOfAlgorithm(cv::Ptr<cv::Feature2D>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_Feature2DG_new_const_Feature2D(cv::Feature2D* val) {
			return new cv::Ptr<cv::Feature2D>(val);
	}
	
}

