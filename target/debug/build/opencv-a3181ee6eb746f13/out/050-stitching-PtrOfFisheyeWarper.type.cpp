extern "C" {
	const cv::FisheyeWarper* cv_PtrLcv_FisheyeWarperG_getInnerPtr_const(const cv::Ptr<cv::FisheyeWarper>* instance) {
			return instance->get();
	}
	
	cv::FisheyeWarper* cv_PtrLcv_FisheyeWarperG_getInnerPtrMut(cv::Ptr<cv::FisheyeWarper>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_FisheyeWarperG_delete(cv::Ptr<cv::FisheyeWarper>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrLcv_FisheyeWarperG_to_PtrOfWarperCreator(cv::Ptr<cv::FisheyeWarper>* instance) {
			return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
	
	cv::Ptr<cv::FisheyeWarper>* cv_PtrLcv_FisheyeWarperG_new_const_FisheyeWarper(cv::FisheyeWarper* val) {
			return new cv::Ptr<cv::FisheyeWarper>(val);
	}
	
}

