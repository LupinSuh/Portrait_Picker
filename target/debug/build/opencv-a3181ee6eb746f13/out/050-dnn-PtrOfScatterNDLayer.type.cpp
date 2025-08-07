extern "C" {
	const cv::dnn::ScatterNDLayer* cv_PtrLcv_dnn_ScatterNDLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::ScatterNDLayer>* instance) {
			return instance->get();
	}
	
	cv::dnn::ScatterNDLayer* cv_PtrLcv_dnn_ScatterNDLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::ScatterNDLayer>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_dnn_ScatterNDLayerG_delete(cv::Ptr<cv::dnn::ScatterNDLayer>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_ScatterNDLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::ScatterNDLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_ScatterNDLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::ScatterNDLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}
	
	cv::Ptr<cv::dnn::ScatterNDLayer>* cv_PtrLcv_dnn_ScatterNDLayerG_new_const_ScatterNDLayer(cv::dnn::ScatterNDLayer* val) {
			return new cv::Ptr<cv::dnn::ScatterNDLayer>(val);
	}
	
}

