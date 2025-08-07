extern "C" {
	const cv::img_hash::RadialVarianceHash* cv_PtrLcv_img_hash_RadialVarianceHashG_getInnerPtr_const(const cv::Ptr<cv::img_hash::RadialVarianceHash>* instance) {
			return instance->get();
	}
	
	cv::img_hash::RadialVarianceHash* cv_PtrLcv_img_hash_RadialVarianceHashG_getInnerPtrMut(cv::Ptr<cv::img_hash::RadialVarianceHash>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_img_hash_RadialVarianceHashG_delete(cv::Ptr<cv::img_hash::RadialVarianceHash>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_img_hash_RadialVarianceHashG_to_PtrOfAlgorithm(cv::Ptr<cv::img_hash::RadialVarianceHash>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::img_hash::ImgHashBase>* cv_PtrLcv_img_hash_RadialVarianceHashG_to_PtrOfImgHashBase(cv::Ptr<cv::img_hash::RadialVarianceHash>* instance) {
			return new cv::Ptr<cv::img_hash::ImgHashBase>(instance->dynamicCast<cv::img_hash::ImgHashBase>());
	}
	
	cv::Ptr<cv::img_hash::RadialVarianceHash>* cv_PtrLcv_img_hash_RadialVarianceHashG_new_const_RadialVarianceHash(cv::img_hash::RadialVarianceHash* val) {
			return new cv::Ptr<cv::img_hash::RadialVarianceHash>(val);
	}
	
}

