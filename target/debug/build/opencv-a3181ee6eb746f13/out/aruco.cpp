#include "aruco.hpp"
#include "aruco_types.hpp"

extern "C" {
	void cv_aruco_EstimateParameters_EstimateParameters(Result<cv::aruco::EstimateParameters*>* ocvrs_return) {
		try {
			cv::aruco::EstimateParameters* ret = new cv::aruco::EstimateParameters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::aruco::EstimateParameters* cv_aruco_EstimateParameters_implicitClone_const(const cv::aruco::EstimateParameters* instance) {
			return new cv::aruco::EstimateParameters(*instance);
	}
	
	void cv_aruco_EstimateParameters_propPattern_const(const cv::aruco::EstimateParameters* instance, cv::aruco::PatternPositionType* ocvrs_return) {
			cv::aruco::PatternPositionType ret = instance->pattern;
			*ocvrs_return = ret;
	}
	
	void cv_aruco_EstimateParameters_propPattern_const_PatternPositionType(cv::aruco::EstimateParameters* instance, const cv::aruco::PatternPositionType val) {
			instance->pattern = val;
	}
	
	bool cv_aruco_EstimateParameters_propUseExtrinsicGuess_const(const cv::aruco::EstimateParameters* instance) {
			bool ret = instance->useExtrinsicGuess;
			return ret;
	}
	
	void cv_aruco_EstimateParameters_propUseExtrinsicGuess_const_bool(cv::aruco::EstimateParameters* instance, const bool val) {
			instance->useExtrinsicGuess = val;
	}
	
	int cv_aruco_EstimateParameters_propSolvePnPMethod_const(const cv::aruco::EstimateParameters* instance) {
			int ret = instance->solvePnPMethod;
			return ret;
	}
	
	void cv_aruco_EstimateParameters_propSolvePnPMethod_const_int(cv::aruco::EstimateParameters* instance, const int val) {
			instance->solvePnPMethod = val;
	}
	
	void cv_aruco_EstimateParameters_delete(cv::aruco::EstimateParameters* instance) {
			delete instance;
	}
	
}
