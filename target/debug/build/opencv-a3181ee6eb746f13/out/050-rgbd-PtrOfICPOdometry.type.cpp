extern "C" {
	const cv::rgbd::ICPOdometry* cv_PtrLcv_rgbd_ICPOdometryG_getInnerPtr_const(const cv::Ptr<cv::rgbd::ICPOdometry>* instance) {
			return instance->get();
	}
	
	cv::rgbd::ICPOdometry* cv_PtrLcv_rgbd_ICPOdometryG_getInnerPtrMut(cv::Ptr<cv::rgbd::ICPOdometry>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_rgbd_ICPOdometryG_delete(cv::Ptr<cv::rgbd::ICPOdometry>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_rgbd_ICPOdometryG_to_PtrOfAlgorithm(cv::Ptr<cv::rgbd::ICPOdometry>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::rgbd::Odometry>* cv_PtrLcv_rgbd_ICPOdometryG_to_PtrOfOdometry(cv::Ptr<cv::rgbd::ICPOdometry>* instance) {
			return new cv::Ptr<cv::rgbd::Odometry>(instance->dynamicCast<cv::rgbd::Odometry>());
	}
	
	cv::Ptr<cv::rgbd::ICPOdometry>* cv_PtrLcv_rgbd_ICPOdometryG_new_const_ICPOdometry(cv::rgbd::ICPOdometry* val) {
			return new cv::Ptr<cv::rgbd::ICPOdometry>(val);
	}
	
}

