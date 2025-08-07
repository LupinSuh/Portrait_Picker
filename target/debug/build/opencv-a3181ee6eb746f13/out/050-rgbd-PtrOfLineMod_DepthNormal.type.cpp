extern "C" {
	const cv::linemod::DepthNormal* cv_PtrLcv_linemod_DepthNormalG_getInnerPtr_const(const cv::Ptr<cv::linemod::DepthNormal>* instance) {
			return instance->get();
	}
	
	cv::linemod::DepthNormal* cv_PtrLcv_linemod_DepthNormalG_getInnerPtrMut(cv::Ptr<cv::linemod::DepthNormal>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_linemod_DepthNormalG_delete(cv::Ptr<cv::linemod::DepthNormal>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::linemod::Modality>* cv_PtrLcv_linemod_DepthNormalG_to_PtrOfLineMod_Modality(cv::Ptr<cv::linemod::DepthNormal>* instance) {
			return new cv::Ptr<cv::linemod::Modality>(instance->dynamicCast<cv::linemod::Modality>());
	}
	
	cv::Ptr<cv::linemod::DepthNormal>* cv_PtrLcv_linemod_DepthNormalG_new_const_DepthNormal(cv::linemod::DepthNormal* val) {
			return new cv::Ptr<cv::linemod::DepthNormal>(val);
	}
	
}

