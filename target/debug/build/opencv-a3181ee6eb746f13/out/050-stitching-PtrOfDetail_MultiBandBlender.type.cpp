extern "C" {
	const cv::detail::MultiBandBlender* cv_PtrLcv_detail_MultiBandBlenderG_getInnerPtr_const(const cv::Ptr<cv::detail::MultiBandBlender>* instance) {
			return instance->get();
	}
	
	cv::detail::MultiBandBlender* cv_PtrLcv_detail_MultiBandBlenderG_getInnerPtrMut(cv::Ptr<cv::detail::MultiBandBlender>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_detail_MultiBandBlenderG_delete(cv::Ptr<cv::detail::MultiBandBlender>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::detail::Blender>* cv_PtrLcv_detail_MultiBandBlenderG_to_PtrOfDetail_Blender(cv::Ptr<cv::detail::MultiBandBlender>* instance) {
			return new cv::Ptr<cv::detail::Blender>(instance->dynamicCast<cv::detail::Blender>());
	}
	
	cv::Ptr<cv::detail::MultiBandBlender>* cv_PtrLcv_detail_MultiBandBlenderG_new_const_MultiBandBlender(cv::detail::MultiBandBlender* val) {
			return new cv::Ptr<cv::detail::MultiBandBlender>(val);
	}
	
}

