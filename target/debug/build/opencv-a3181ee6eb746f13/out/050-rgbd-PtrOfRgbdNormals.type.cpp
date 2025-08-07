extern "C" {
	const cv::rgbd::RgbdNormals* cv_PtrLcv_rgbd_RgbdNormalsG_getInnerPtr_const(const cv::Ptr<cv::rgbd::RgbdNormals>* instance) {
			return instance->get();
	}
	
	cv::rgbd::RgbdNormals* cv_PtrLcv_rgbd_RgbdNormalsG_getInnerPtrMut(cv::Ptr<cv::rgbd::RgbdNormals>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_rgbd_RgbdNormalsG_delete(cv::Ptr<cv::rgbd::RgbdNormals>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_rgbd_RgbdNormalsG_to_PtrOfAlgorithm(cv::Ptr<cv::rgbd::RgbdNormals>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::rgbd::RgbdNormals>* cv_PtrLcv_rgbd_RgbdNormalsG_new_const_RgbdNormals(cv::rgbd::RgbdNormals* val) {
			return new cv::Ptr<cv::rgbd::RgbdNormals>(val);
	}
	
}

