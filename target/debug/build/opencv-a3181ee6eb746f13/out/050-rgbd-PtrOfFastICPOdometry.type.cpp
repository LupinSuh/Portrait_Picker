extern "C" {
	const cv::rgbd::FastICPOdometry* cv_PtrLcv_rgbd_FastICPOdometryG_getInnerPtr_const(const cv::Ptr<cv::rgbd::FastICPOdometry>* instance) {
			return instance->get();
	}
	
	cv::rgbd::FastICPOdometry* cv_PtrLcv_rgbd_FastICPOdometryG_getInnerPtrMut(cv::Ptr<cv::rgbd::FastICPOdometry>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_rgbd_FastICPOdometryG_delete(cv::Ptr<cv::rgbd::FastICPOdometry>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_rgbd_FastICPOdometryG_to_PtrOfAlgorithm(cv::Ptr<cv::rgbd::FastICPOdometry>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::rgbd::Odometry>* cv_PtrLcv_rgbd_FastICPOdometryG_to_PtrOfOdometry(cv::Ptr<cv::rgbd::FastICPOdometry>* instance) {
			return new cv::Ptr<cv::rgbd::Odometry>(instance->dynamicCast<cv::rgbd::Odometry>());
	}
	
	cv::Ptr<cv::rgbd::FastICPOdometry>* cv_PtrLcv_rgbd_FastICPOdometryG_new_const_FastICPOdometry(cv::rgbd::FastICPOdometry* val) {
			return new cv::Ptr<cv::rgbd::FastICPOdometry>(val);
	}
	
}

