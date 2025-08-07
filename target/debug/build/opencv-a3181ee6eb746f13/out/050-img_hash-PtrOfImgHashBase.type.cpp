extern "C" {
	const cv::img_hash::ImgHashBase* cv_PtrLcv_img_hash_ImgHashBaseG_getInnerPtr_const(const cv::Ptr<cv::img_hash::ImgHashBase>* instance) {
			return instance->get();
	}
	
	cv::img_hash::ImgHashBase* cv_PtrLcv_img_hash_ImgHashBaseG_getInnerPtrMut(cv::Ptr<cv::img_hash::ImgHashBase>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_img_hash_ImgHashBaseG_delete(cv::Ptr<cv::img_hash::ImgHashBase>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_img_hash_ImgHashBaseG_to_PtrOfAlgorithm(cv::Ptr<cv::img_hash::ImgHashBase>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::img_hash::ImgHashBase>* cv_PtrLcv_img_hash_ImgHashBaseG_new_const_ImgHashBase(cv::img_hash::ImgHashBase* val) {
			return new cv::Ptr<cv::img_hash::ImgHashBase>(val);
	}
	
}

