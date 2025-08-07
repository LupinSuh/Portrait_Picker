extern "C" {
	const cv::img_hash::PHash* cv_PtrLcv_img_hash_PHashG_getInnerPtr_const(const cv::Ptr<cv::img_hash::PHash>* instance) {
			return instance->get();
	}
	
	cv::img_hash::PHash* cv_PtrLcv_img_hash_PHashG_getInnerPtrMut(cv::Ptr<cv::img_hash::PHash>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_img_hash_PHashG_delete(cv::Ptr<cv::img_hash::PHash>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_img_hash_PHashG_to_PtrOfAlgorithm(cv::Ptr<cv::img_hash::PHash>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::img_hash::ImgHashBase>* cv_PtrLcv_img_hash_PHashG_to_PtrOfImgHashBase(cv::Ptr<cv::img_hash::PHash>* instance) {
			return new cv::Ptr<cv::img_hash::ImgHashBase>(instance->dynamicCast<cv::img_hash::ImgHashBase>());
	}
	
	cv::Ptr<cv::img_hash::PHash>* cv_PtrLcv_img_hash_PHashG_new_const_PHash(cv::img_hash::PHash* val) {
			return new cv::Ptr<cv::img_hash::PHash>(val);
	}
	
}

