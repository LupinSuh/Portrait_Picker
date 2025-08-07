extern "C" {
	const cv::kinfu::VolumeParams* cv_PtrLcv_kinfu_VolumeParamsG_getInnerPtr_const(const cv::Ptr<cv::kinfu::VolumeParams>* instance) {
			return instance->get();
	}
	
	cv::kinfu::VolumeParams* cv_PtrLcv_kinfu_VolumeParamsG_getInnerPtrMut(cv::Ptr<cv::kinfu::VolumeParams>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_kinfu_VolumeParamsG_delete(cv::Ptr<cv::kinfu::VolumeParams>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::kinfu::VolumeParams>* cv_PtrLcv_kinfu_VolumeParamsG_new_const_VolumeParams(cv::kinfu::VolumeParams* val) {
			return new cv::Ptr<cv::kinfu::VolumeParams>(val);
	}
	
}

