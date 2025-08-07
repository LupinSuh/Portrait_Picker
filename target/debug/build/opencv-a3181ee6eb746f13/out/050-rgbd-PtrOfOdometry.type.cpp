extern "C" {
	const cv::rgbd::Odometry* cv_PtrLcv_rgbd_OdometryG_getInnerPtr_const(const cv::Ptr<cv::rgbd::Odometry>* instance) {
			return instance->get();
	}
	
	cv::rgbd::Odometry* cv_PtrLcv_rgbd_OdometryG_getInnerPtrMut(cv::Ptr<cv::rgbd::Odometry>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_rgbd_OdometryG_delete(cv::Ptr<cv::rgbd::Odometry>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_rgbd_OdometryG_to_PtrOfAlgorithm(cv::Ptr<cv::rgbd::Odometry>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

