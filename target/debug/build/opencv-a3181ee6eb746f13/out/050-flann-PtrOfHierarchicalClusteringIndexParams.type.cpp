extern "C" {
	const cv::flann::HierarchicalClusteringIndexParams* cv_PtrLcv_flann_HierarchicalClusteringIndexParamsG_getInnerPtr_const(const cv::Ptr<cv::flann::HierarchicalClusteringIndexParams>* instance) {
			return instance->get();
	}
	
	cv::flann::HierarchicalClusteringIndexParams* cv_PtrLcv_flann_HierarchicalClusteringIndexParamsG_getInnerPtrMut(cv::Ptr<cv::flann::HierarchicalClusteringIndexParams>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_flann_HierarchicalClusteringIndexParamsG_delete(cv::Ptr<cv::flann::HierarchicalClusteringIndexParams>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::flann::IndexParams>* cv_PtrLcv_flann_HierarchicalClusteringIndexParamsG_to_PtrOfIndexParams(cv::Ptr<cv::flann::HierarchicalClusteringIndexParams>* instance) {
			return new cv::Ptr<cv::flann::IndexParams>(instance->dynamicCast<cv::flann::IndexParams>());
	}
	
	cv::Ptr<cv::flann::HierarchicalClusteringIndexParams>* cv_PtrLcv_flann_HierarchicalClusteringIndexParamsG_new_const_HierarchicalClusteringIndexParams(cv::flann::HierarchicalClusteringIndexParams* val) {
			return new cv::Ptr<cv::flann::HierarchicalClusteringIndexParams>(val);
	}
	
}

