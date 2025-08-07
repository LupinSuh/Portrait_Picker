extern "C" {
	const cv::rgbd::RgbdOdometry* cv_PtrLcv_rgbd_RgbdOdometryG_getInnerPtr_const(const cv::Ptr<cv::rgbd::RgbdOdometry>* instance) {
			return instance->get();
	}
	
	cv::rgbd::RgbdOdometry* cv_PtrLcv_rgbd_RgbdOdometryG_getInnerPtrMut(cv::Ptr<cv::rgbd::RgbdOdometry>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_rgbd_RgbdOdometryG_delete(cv::Ptr<cv::rgbd::RgbdOdometry>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_rgbd_RgbdOdometryG_to_PtrOfAlgorithm(cv::Ptr<cv::rgbd::RgbdOdometry>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::rgbd::Odometry>* cv_PtrLcv_rgbd_RgbdOdometryG_to_PtrOfOdometry(cv::Ptr<cv::rgbd::RgbdOdometry>* instance) {
			return new cv::Ptr<cv::rgbd::Odometry>(instance->dynamicCast<cv::rgbd::Odometry>());
	}
	
	cv::Ptr<cv::rgbd::RgbdOdometry>* cv_PtrLcv_rgbd_RgbdOdometryG_new_const_RgbdOdometry(cv::rgbd::RgbdOdometry* val) {
			return new cv::Ptr<cv::rgbd::RgbdOdometry>(val);
	}
	
}

