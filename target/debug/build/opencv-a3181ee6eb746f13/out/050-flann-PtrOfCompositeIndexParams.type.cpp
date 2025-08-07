extern "C" {
	const cv::flann::CompositeIndexParams* cv_PtrLcv_flann_CompositeIndexParamsG_getInnerPtr_const(const cv::Ptr<cv::flann::CompositeIndexParams>* instance) {
			return instance->get();
	}
	
	cv::flann::CompositeIndexParams* cv_PtrLcv_flann_CompositeIndexParamsG_getInnerPtrMut(cv::Ptr<cv::flann::CompositeIndexParams>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_flann_CompositeIndexParamsG_delete(cv::Ptr<cv::flann::CompositeIndexParams>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::flann::IndexParams>* cv_PtrLcv_flann_CompositeIndexParamsG_to_PtrOfIndexParams(cv::Ptr<cv::flann::CompositeIndexParams>* instance) {
			return new cv::Ptr<cv::flann::IndexParams>(instance->dynamicCast<cv::flann::IndexParams>());
	}
	
	cv::Ptr<cv::flann::CompositeIndexParams>* cv_PtrLcv_flann_CompositeIndexParamsG_new_const_CompositeIndexParams(cv::flann::CompositeIndexParams* val) {
			return new cv::Ptr<cv::flann::CompositeIndexParams>(val);
	}
	
}

