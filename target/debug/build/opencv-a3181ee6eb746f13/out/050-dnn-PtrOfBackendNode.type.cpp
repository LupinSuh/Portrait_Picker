extern "C" {
	const cv::dnn::BackendNode* cv_PtrLcv_dnn_BackendNodeG_getInnerPtr_const(const cv::Ptr<cv::dnn::BackendNode>* instance) {
			return instance->get();
	}
	
	cv::dnn::BackendNode* cv_PtrLcv_dnn_BackendNodeG_getInnerPtrMut(cv::Ptr<cv::dnn::BackendNode>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_dnn_BackendNodeG_delete(cv::Ptr<cv::dnn::BackendNode>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::dnn::BackendNode>* cv_PtrLcv_dnn_BackendNodeG_new_const_BackendNode(cv::dnn::BackendNode* val) {
			return new cv::Ptr<cv::dnn::BackendNode>(val);
	}
	
}

