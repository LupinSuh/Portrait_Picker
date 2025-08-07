extern "C" {
	const cv::img_hash::ColorMomentHash* cv_PtrLcv_img_hash_ColorMomentHashG_getInnerPtr_const(const cv::Ptr<cv::img_hash::ColorMomentHash>* instance) {
			return instance->get();
	}
	
	cv::img_hash::ColorMomentHash* cv_PtrLcv_img_hash_ColorMomentHashG_getInnerPtrMut(cv::Ptr<cv::img_hash::ColorMomentHash>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_img_hash_ColorMomentHashG_delete(cv::Ptr<cv::img_hash::ColorMomentHash>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_img_hash_ColorMomentHashG_to_PtrOfAlgorithm(cv::Ptr<cv::img_hash::ColorMomentHash>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::img_hash::ImgHashBase>* cv_PtrLcv_img_hash_ColorMomentHashG_to_PtrOfImgHashBase(cv::Ptr<cv::img_hash::ColorMomentHash>* instance) {
			return new cv::Ptr<cv::img_hash::ImgHashBase>(instance->dynamicCast<cv::img_hash::ImgHashBase>());
	}
	
	cv::Ptr<cv::img_hash::ColorMomentHash>* cv_PtrLcv_img_hash_ColorMomentHashG_new_const_ColorMomentHash(cv::img_hash::ColorMomentHash* val) {
			return new cv::Ptr<cv::img_hash::ColorMomentHash>(val);
	}
	
}

