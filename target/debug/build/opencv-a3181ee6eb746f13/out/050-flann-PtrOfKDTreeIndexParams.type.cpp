extern "C" {
	const cv::flann::KDTreeIndexParams* cv_PtrLcv_flann_KDTreeIndexParamsG_getInnerPtr_const(const cv::Ptr<cv::flann::KDTreeIndexParams>* instance) {
			return instance->get();
	}
	
	cv::flann::KDTreeIndexParams* cv_PtrLcv_flann_KDTreeIndexParamsG_getInnerPtrMut(cv::Ptr<cv::flann::KDTreeIndexParams>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_flann_KDTreeIndexParamsG_delete(cv::Ptr<cv::flann::KDTreeIndexParams>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::flann::IndexParams>* cv_PtrLcv_flann_KDTreeIndexParamsG_to_PtrOfIndexParams(cv::Ptr<cv::flann::KDTreeIndexParams>* instance) {
			return new cv::Ptr<cv::flann::IndexParams>(instance->dynamicCast<cv::flann::IndexParams>());
	}
	
	cv::Ptr<cv::flann::KDTreeIndexParams>* cv_PtrLcv_flann_KDTreeIndexParamsG_new_const_KDTreeIndexParams(cv::flann::KDTreeIndexParams* val) {
			return new cv::Ptr<cv::flann::KDTreeIndexParams>(val);
	}
	
}

