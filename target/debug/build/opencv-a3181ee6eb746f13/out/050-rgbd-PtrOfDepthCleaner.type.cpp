extern "C" {
	const cv::rgbd::DepthCleaner* cv_PtrLcv_rgbd_DepthCleanerG_getInnerPtr_const(const cv::Ptr<cv::rgbd::DepthCleaner>* instance) {
			return instance->get();
	}
	
	cv::rgbd::DepthCleaner* cv_PtrLcv_rgbd_DepthCleanerG_getInnerPtrMut(cv::Ptr<cv::rgbd::DepthCleaner>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_rgbd_DepthCleanerG_delete(cv::Ptr<cv::rgbd::DepthCleaner>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_rgbd_DepthCleanerG_to_PtrOfAlgorithm(cv::Ptr<cv::rgbd::DepthCleaner>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::rgbd::DepthCleaner>* cv_PtrLcv_rgbd_DepthCleanerG_new_const_DepthCleaner(cv::rgbd::DepthCleaner* val) {
			return new cv::Ptr<cv::rgbd::DepthCleaner>(val);
	}
	
}

