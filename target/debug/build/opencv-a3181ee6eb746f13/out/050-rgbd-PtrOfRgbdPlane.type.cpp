extern "C" {
	const cv::rgbd::RgbdPlane* cv_PtrLcv_rgbd_RgbdPlaneG_getInnerPtr_const(const cv::Ptr<cv::rgbd::RgbdPlane>* instance) {
			return instance->get();
	}
	
	cv::rgbd::RgbdPlane* cv_PtrLcv_rgbd_RgbdPlaneG_getInnerPtrMut(cv::Ptr<cv::rgbd::RgbdPlane>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_rgbd_RgbdPlaneG_delete(cv::Ptr<cv::rgbd::RgbdPlane>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_rgbd_RgbdPlaneG_to_PtrOfAlgorithm(cv::Ptr<cv::rgbd::RgbdPlane>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::rgbd::RgbdPlane>* cv_PtrLcv_rgbd_RgbdPlaneG_new_const_RgbdPlane(cv::rgbd::RgbdPlane* val) {
			return new cv::Ptr<cv::rgbd::RgbdPlane>(val);
	}
	
}

