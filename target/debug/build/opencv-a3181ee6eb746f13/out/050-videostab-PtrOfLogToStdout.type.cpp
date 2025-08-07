extern "C" {
	const cv::videostab::LogToStdout* cv_PtrLcv_videostab_LogToStdoutG_getInnerPtr_const(const cv::Ptr<cv::videostab::LogToStdout>* instance) {
			return instance->get();
	}
	
	cv::videostab::LogToStdout* cv_PtrLcv_videostab_LogToStdoutG_getInnerPtrMut(cv::Ptr<cv::videostab::LogToStdout>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_videostab_LogToStdoutG_delete(cv::Ptr<cv::videostab::LogToStdout>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::videostab::ILog>* cv_PtrLcv_videostab_LogToStdoutG_to_PtrOfILog(cv::Ptr<cv::videostab::LogToStdout>* instance) {
			return new cv::Ptr<cv::videostab::ILog>(instance->dynamicCast<cv::videostab::ILog>());
	}
	
	cv::Ptr<cv::videostab::LogToStdout>* cv_PtrLcv_videostab_LogToStdoutG_new_const_LogToStdout(cv::videostab::LogToStdout* val) {
			return new cv::Ptr<cv::videostab::LogToStdout>(val);
	}
	
}

