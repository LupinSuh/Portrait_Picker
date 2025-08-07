extern "C" {
	const cv::face::StandardCollector* cv_PtrLcv_face_StandardCollectorG_getInnerPtr_const(const cv::Ptr<cv::face::StandardCollector>* instance) {
			return instance->get();
	}
	
	cv::face::StandardCollector* cv_PtrLcv_face_StandardCollectorG_getInnerPtrMut(cv::Ptr<cv::face::StandardCollector>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_face_StandardCollectorG_delete(cv::Ptr<cv::face::StandardCollector>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::face::PredictCollector>* cv_PtrLcv_face_StandardCollectorG_to_PtrOfPredictCollector(cv::Ptr<cv::face::StandardCollector>* instance) {
			return new cv::Ptr<cv::face::PredictCollector>(instance->dynamicCast<cv::face::PredictCollector>());
	}
	
	cv::Ptr<cv::face::StandardCollector>* cv_PtrLcv_face_StandardCollectorG_new_const_StandardCollector(cv::face::StandardCollector* val) {
			return new cv::Ptr<cv::face::StandardCollector>(val);
	}
	
}

