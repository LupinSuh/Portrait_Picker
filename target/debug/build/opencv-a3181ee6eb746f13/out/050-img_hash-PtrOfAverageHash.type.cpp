extern "C" {
	const cv::img_hash::AverageHash* cv_PtrLcv_img_hash_AverageHashG_getInnerPtr_const(const cv::Ptr<cv::img_hash::AverageHash>* instance) {
			return instance->get();
	}
	
	cv::img_hash::AverageHash* cv_PtrLcv_img_hash_AverageHashG_getInnerPtrMut(cv::Ptr<cv::img_hash::AverageHash>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_img_hash_AverageHashG_delete(cv::Ptr<cv::img_hash::AverageHash>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_img_hash_AverageHashG_to_PtrOfAlgorithm(cv::Ptr<cv::img_hash::AverageHash>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::img_hash::ImgHashBase>* cv_PtrLcv_img_hash_AverageHashG_to_PtrOfImgHashBase(cv::Ptr<cv::img_hash::AverageHash>* instance) {
			return new cv::Ptr<cv::img_hash::ImgHashBase>(instance->dynamicCast<cv::img_hash::ImgHashBase>());
	}
	
	cv::Ptr<cv::img_hash::AverageHash>* cv_PtrLcv_img_hash_AverageHashG_new_const_AverageHash(cv::img_hash::AverageHash* val) {
			return new cv::Ptr<cv::img_hash::AverageHash>(val);
	}
	
}

