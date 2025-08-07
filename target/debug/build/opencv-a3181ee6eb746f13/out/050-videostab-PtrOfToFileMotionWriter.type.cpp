extern "C" {
	const cv::videostab::ToFileMotionWriter* cv_PtrLcv_videostab_ToFileMotionWriterG_getInnerPtr_const(const cv::Ptr<cv::videostab::ToFileMotionWriter>* instance) {
			return instance->get();
	}
	
	cv::videostab::ToFileMotionWriter* cv_PtrLcv_videostab_ToFileMotionWriterG_getInnerPtrMut(cv::Ptr<cv::videostab::ToFileMotionWriter>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_videostab_ToFileMotionWriterG_delete(cv::Ptr<cv::videostab::ToFileMotionWriter>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::videostab::ImageMotionEstimatorBase>* cv_PtrLcv_videostab_ToFileMotionWriterG_to_PtrOfImageMotionEstimatorBase(cv::Ptr<cv::videostab::ToFileMotionWriter>* instance) {
			return new cv::Ptr<cv::videostab::ImageMotionEstimatorBase>(instance->dynamicCast<cv::videostab::ImageMotionEstimatorBase>());
	}
	
	cv::Ptr<cv::videostab::ToFileMotionWriter>* cv_PtrLcv_videostab_ToFileMotionWriterG_new_const_ToFileMotionWriter(cv::videostab::ToFileMotionWriter* val) {
			return new cv::Ptr<cv::videostab::ToFileMotionWriter>(val);
	}
	
}

