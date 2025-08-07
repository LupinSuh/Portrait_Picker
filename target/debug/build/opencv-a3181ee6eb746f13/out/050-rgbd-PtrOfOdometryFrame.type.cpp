extern "C" {
	const cv::rgbd::OdometryFrame* cv_PtrLcv_rgbd_OdometryFrameG_getInnerPtr_const(const cv::Ptr<cv::rgbd::OdometryFrame>* instance) {
			return instance->get();
	}
	
	cv::rgbd::OdometryFrame* cv_PtrLcv_rgbd_OdometryFrameG_getInnerPtrMut(cv::Ptr<cv::rgbd::OdometryFrame>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_rgbd_OdometryFrameG_delete(cv::Ptr<cv::rgbd::OdometryFrame>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::rgbd::RgbdFrame>* cv_PtrLcv_rgbd_OdometryFrameG_to_PtrOfRgbdFrame(cv::Ptr<cv::rgbd::OdometryFrame>* instance) {
			return new cv::Ptr<cv::rgbd::RgbdFrame>(instance->dynamicCast<cv::rgbd::RgbdFrame>());
	}
	
	cv::Ptr<cv::rgbd::OdometryFrame>* cv_PtrLcv_rgbd_OdometryFrameG_new_const_OdometryFrame(cv::rgbd::OdometryFrame* val) {
			return new cv::Ptr<cv::rgbd::OdometryFrame>(val);
	}
	
}

