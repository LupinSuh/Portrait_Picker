extern "C" {
	const cv::PlaneWarper* cv_PtrLcv_PlaneWarperG_getInnerPtr_const(const cv::Ptr<cv::PlaneWarper>* instance) {
			return instance->get();
	}
	
	cv::PlaneWarper* cv_PtrLcv_PlaneWarperG_getInnerPtrMut(cv::Ptr<cv::PlaneWarper>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_PlaneWarperG_delete(cv::Ptr<cv::PlaneWarper>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrLcv_PlaneWarperG_to_PtrOfWarperCreator(cv::Ptr<cv::PlaneWarper>* instance) {
			return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
	
	cv::Ptr<cv::PlaneWarper>* cv_PtrLcv_PlaneWarperG_new_const_PlaneWarper(cv::PlaneWarper* val) {
			return new cv::Ptr<cv::PlaneWarper>(val);
	}
	
}

