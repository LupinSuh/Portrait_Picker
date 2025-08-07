extern "C" {
	const cv::flann::AutotunedIndexParams* cv_PtrLcv_flann_AutotunedIndexParamsG_getInnerPtr_const(const cv::Ptr<cv::flann::AutotunedIndexParams>* instance) {
			return instance->get();
	}
	
	cv::flann::AutotunedIndexParams* cv_PtrLcv_flann_AutotunedIndexParamsG_getInnerPtrMut(cv::Ptr<cv::flann::AutotunedIndexParams>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_flann_AutotunedIndexParamsG_delete(cv::Ptr<cv::flann::AutotunedIndexParams>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::flann::IndexParams>* cv_PtrLcv_flann_AutotunedIndexParamsG_to_PtrOfIndexParams(cv::Ptr<cv::flann::AutotunedIndexParams>* instance) {
			return new cv::Ptr<cv::flann::IndexParams>(instance->dynamicCast<cv::flann::IndexParams>());
	}
	
	cv::Ptr<cv::flann::AutotunedIndexParams>* cv_PtrLcv_flann_AutotunedIndexParamsG_new_const_AutotunedIndexParams(cv::flann::AutotunedIndexParams* val) {
			return new cv::Ptr<cv::flann::AutotunedIndexParams>(val);
	}
	
}

