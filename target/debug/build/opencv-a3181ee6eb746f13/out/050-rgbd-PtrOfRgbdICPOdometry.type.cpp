extern "C" {
	const cv::rgbd::RgbdICPOdometry* cv_PtrLcv_rgbd_RgbdICPOdometryG_getInnerPtr_const(const cv::Ptr<cv::rgbd::RgbdICPOdometry>* instance) {
			return instance->get();
	}
	
	cv::rgbd::RgbdICPOdometry* cv_PtrLcv_rgbd_RgbdICPOdometryG_getInnerPtrMut(cv::Ptr<cv::rgbd::RgbdICPOdometry>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_rgbd_RgbdICPOdometryG_delete(cv::Ptr<cv::rgbd::RgbdICPOdometry>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_rgbd_RgbdICPOdometryG_to_PtrOfAlgorithm(cv::Ptr<cv::rgbd::RgbdICPOdometry>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::rgbd::Odometry>* cv_PtrLcv_rgbd_RgbdICPOdometryG_to_PtrOfOdometry(cv::Ptr<cv::rgbd::RgbdICPOdometry>* instance) {
			return new cv::Ptr<cv::rgbd::Odometry>(instance->dynamicCast<cv::rgbd::Odometry>());
	}
	
	cv::Ptr<cv::rgbd::RgbdICPOdometry>* cv_PtrLcv_rgbd_RgbdICPOdometryG_new_const_RgbdICPOdometry(cv::rgbd::RgbdICPOdometry* val) {
			return new cv::Ptr<cv::rgbd::RgbdICPOdometry>(val);
	}
	
}

