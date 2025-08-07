extern "C" {
	const cv::videostab::FromFileMotionReader* cv_PtrLcv_videostab_FromFileMotionReaderG_getInnerPtr_const(const cv::Ptr<cv::videostab::FromFileMotionReader>* instance) {
			return instance->get();
	}
	
	cv::videostab::FromFileMotionReader* cv_PtrLcv_videostab_FromFileMotionReaderG_getInnerPtrMut(cv::Ptr<cv::videostab::FromFileMotionReader>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_videostab_FromFileMotionReaderG_delete(cv::Ptr<cv::videostab::FromFileMotionReader>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::videostab::ImageMotionEstimatorBase>* cv_PtrLcv_videostab_FromFileMotionReaderG_to_PtrOfImageMotionEstimatorBase(cv::Ptr<cv::videostab::FromFileMotionReader>* instance) {
			return new cv::Ptr<cv::videostab::ImageMotionEstimatorBase>(instance->dynamicCast<cv::videostab::ImageMotionEstimatorBase>());
	}
	
	cv::Ptr<cv::videostab::FromFileMotionReader>* cv_PtrLcv_videostab_FromFileMotionReaderG_new_const_FromFileMotionReader(cv::videostab::FromFileMotionReader* val) {
			return new cv::Ptr<cv::videostab::FromFileMotionReader>(val);
	}
	
}

