extern "C" {
	const cv::detail::Blender* cv_PtrLcv_detail_BlenderG_getInnerPtr_const(const cv::Ptr<cv::detail::Blender>* instance) {
			return instance->get();
	}
	
	cv::detail::Blender* cv_PtrLcv_detail_BlenderG_getInnerPtrMut(cv::Ptr<cv::detail::Blender>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_detail_BlenderG_delete(cv::Ptr<cv::detail::Blender>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::detail::Blender>* cv_PtrLcv_detail_BlenderG_new_const_Blender(cv::detail::Blender* val) {
			return new cv::Ptr<cv::detail::Blender>(val);
	}
	
}

