extern "C" {
	const cv::img_hash::BlockMeanHash* cv_PtrLcv_img_hash_BlockMeanHashG_getInnerPtr_const(const cv::Ptr<cv::img_hash::BlockMeanHash>* instance) {
			return instance->get();
	}
	
	cv::img_hash::BlockMeanHash* cv_PtrLcv_img_hash_BlockMeanHashG_getInnerPtrMut(cv::Ptr<cv::img_hash::BlockMeanHash>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_img_hash_BlockMeanHashG_delete(cv::Ptr<cv::img_hash::BlockMeanHash>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_img_hash_BlockMeanHashG_to_PtrOfAlgorithm(cv::Ptr<cv::img_hash::BlockMeanHash>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::img_hash::ImgHashBase>* cv_PtrLcv_img_hash_BlockMeanHashG_to_PtrOfImgHashBase(cv::Ptr<cv::img_hash::BlockMeanHash>* instance) {
			return new cv::Ptr<cv::img_hash::ImgHashBase>(instance->dynamicCast<cv::img_hash::ImgHashBase>());
	}
	
	cv::Ptr<cv::img_hash::BlockMeanHash>* cv_PtrLcv_img_hash_BlockMeanHashG_new_const_BlockMeanHash(cv::img_hash::BlockMeanHash* val) {
			return new cv::Ptr<cv::img_hash::BlockMeanHash>(val);
	}
	
}

