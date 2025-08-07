extern "C" {
	const cv::img_hash::MarrHildrethHash* cv_PtrLcv_img_hash_MarrHildrethHashG_getInnerPtr_const(const cv::Ptr<cv::img_hash::MarrHildrethHash>* instance) {
			return instance->get();
	}
	
	cv::img_hash::MarrHildrethHash* cv_PtrLcv_img_hash_MarrHildrethHashG_getInnerPtrMut(cv::Ptr<cv::img_hash::MarrHildrethHash>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_img_hash_MarrHildrethHashG_delete(cv::Ptr<cv::img_hash::MarrHildrethHash>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_img_hash_MarrHildrethHashG_to_PtrOfAlgorithm(cv::Ptr<cv::img_hash::MarrHildrethHash>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::img_hash::ImgHashBase>* cv_PtrLcv_img_hash_MarrHildrethHashG_to_PtrOfImgHashBase(cv::Ptr<cv::img_hash::MarrHildrethHash>* instance) {
			return new cv::Ptr<cv::img_hash::ImgHashBase>(instance->dynamicCast<cv::img_hash::ImgHashBase>());
	}
	
	cv::Ptr<cv::img_hash::MarrHildrethHash>* cv_PtrLcv_img_hash_MarrHildrethHashG_new_const_MarrHildrethHash(cv::img_hash::MarrHildrethHash* val) {
			return new cv::Ptr<cv::img_hash::MarrHildrethHash>(val);
	}
	
}

