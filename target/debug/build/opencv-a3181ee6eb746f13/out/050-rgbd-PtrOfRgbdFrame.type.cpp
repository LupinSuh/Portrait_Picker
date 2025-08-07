extern "C" {
	const cv::rgbd::RgbdFrame* cv_PtrLcv_rgbd_RgbdFrameG_getInnerPtr_const(const cv::Ptr<cv::rgbd::RgbdFrame>* instance) {
			return instance->get();
	}
	
	cv::rgbd::RgbdFrame* cv_PtrLcv_rgbd_RgbdFrameG_getInnerPtrMut(cv::Ptr<cv::rgbd::RgbdFrame>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_rgbd_RgbdFrameG_delete(cv::Ptr<cv::rgbd::RgbdFrame>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::rgbd::RgbdFrame>* cv_PtrLcv_rgbd_RgbdFrameG_new_const_RgbdFrame(cv::rgbd::RgbdFrame* val) {
			return new cv::Ptr<cv::rgbd::RgbdFrame>(val);
	}
	
}

