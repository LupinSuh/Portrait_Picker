extern "C" {
	const cv::Stitcher* cv_PtrLcv_StitcherG_getInnerPtr_const(const cv::Ptr<cv::Stitcher>* instance) {
			return instance->get();
	}
	
	cv::Stitcher* cv_PtrLcv_StitcherG_getInnerPtrMut(cv::Ptr<cv::Stitcher>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_StitcherG_delete(cv::Ptr<cv::Stitcher>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Stitcher>* cv_PtrLcv_StitcherG_new_const_Stitcher(cv::Stitcher* val) {
			return new cv::Ptr<cv::Stitcher>(val);
	}
	
}

