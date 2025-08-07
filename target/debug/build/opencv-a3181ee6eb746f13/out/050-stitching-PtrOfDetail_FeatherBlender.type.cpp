extern "C" {
	const cv::detail::FeatherBlender* cv_PtrLcv_detail_FeatherBlenderG_getInnerPtr_const(const cv::Ptr<cv::detail::FeatherBlender>* instance) {
			return instance->get();
	}
	
	cv::detail::FeatherBlender* cv_PtrLcv_detail_FeatherBlenderG_getInnerPtrMut(cv::Ptr<cv::detail::FeatherBlender>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_detail_FeatherBlenderG_delete(cv::Ptr<cv::detail::FeatherBlender>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::detail::Blender>* cv_PtrLcv_detail_FeatherBlenderG_to_PtrOfDetail_Blender(cv::Ptr<cv::detail::FeatherBlender>* instance) {
			return new cv::Ptr<cv::detail::Blender>(instance->dynamicCast<cv::detail::Blender>());
	}
	
	cv::Ptr<cv::detail::FeatherBlender>* cv_PtrLcv_detail_FeatherBlenderG_new_const_FeatherBlender(cv::detail::FeatherBlender* val) {
			return new cv::Ptr<cv::detail::FeatherBlender>(val);
	}
	
}

