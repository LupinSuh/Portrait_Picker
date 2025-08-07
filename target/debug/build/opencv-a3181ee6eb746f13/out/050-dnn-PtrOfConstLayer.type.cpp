extern "C" {
	const cv::dnn::ConstLayer* cv_PtrLcv_dnn_ConstLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::ConstLayer>* instance) {
			return instance->get();
	}
	
	cv::dnn::ConstLayer* cv_PtrLcv_dnn_ConstLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::ConstLayer>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_dnn_ConstLayerG_delete(cv::Ptr<cv::dnn::ConstLayer>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_ConstLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::ConstLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_ConstLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::ConstLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}
	
	cv::Ptr<cv::dnn::ConstLayer>* cv_PtrLcv_dnn_ConstLayerG_new_const_ConstLayer(cv::dnn::ConstLayer* val) {
			return new cv::Ptr<cv::dnn::ConstLayer>(val);
	}
	
}

