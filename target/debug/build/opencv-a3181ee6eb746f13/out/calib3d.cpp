#include "ocvrs_common.hpp"
#include <opencv2/calib3d.hpp>
#include "calib3d_types.hpp"

extern "C" {
	void cv_CirclesGridFinderParameters_CirclesGridFinderParameters(Result<cv::CirclesGridFinderParameters>* ocvrs_return) {
		try {
			cv::CirclesGridFinderParameters ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_LMSolver_run_const_const__InputOutputArrayR(const cv::LMSolver* instance, const cv::_InputOutputArray* param, Result<int>* ocvrs_return) {
		try {
			int ret = instance->run(*param);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_LMSolver_setMaxIters_int(cv::LMSolver* instance, int maxIters, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxIters(maxIters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_LMSolver_getMaxIters_const(const cv::LMSolver* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxIters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_LMSolver_create_const_PtrLCallbackGR_int(const cv::Ptr<cv::LMSolver::Callback>* cb, int maxIters, Result<cv::Ptr<cv::LMSolver>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::LMSolver> ret = cv::LMSolver::create(*cb, maxIters);
			Ok(new cv::Ptr<cv::LMSolver>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_LMSolver_create_const_PtrLCallbackGR_int_double(const cv::Ptr<cv::LMSolver::Callback>* cb, int maxIters, double eps, Result<cv::Ptr<cv::LMSolver>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::LMSolver> ret = cv::LMSolver::create(*cb, maxIters, eps);
			Ok(new cv::Ptr<cv::LMSolver>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_LMSolver_to_Algorithm(cv::LMSolver* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_LMSolver_delete(cv::LMSolver* instance) {
			delete instance;
	}
	
	void cv_LMSolver_Callback_compute_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::LMSolver::Callback* instance, const cv::_InputArray* param, const cv::_OutputArray* err, const cv::_OutputArray* J, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->compute(*param, *err, *J);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_LMSolver_Callback_delete(cv::LMSolver::Callback* instance) {
			delete instance;
	}
	
	void cv_StereoBM_getPreFilterType_const(const cv::StereoBM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPreFilterType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoBM_setPreFilterType_int(cv::StereoBM* instance, int preFilterType, ResultVoid* ocvrs_return) {
		try {
			instance->setPreFilterType(preFilterType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoBM_getPreFilterSize_const(const cv::StereoBM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPreFilterSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoBM_setPreFilterSize_int(cv::StereoBM* instance, int preFilterSize, ResultVoid* ocvrs_return) {
		try {
			instance->setPreFilterSize(preFilterSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoBM_getPreFilterCap_const(const cv::StereoBM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPreFilterCap();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoBM_setPreFilterCap_int(cv::StereoBM* instance, int preFilterCap, ResultVoid* ocvrs_return) {
		try {
			instance->setPreFilterCap(preFilterCap);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoBM_getTextureThreshold_const(const cv::StereoBM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getTextureThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoBM_setTextureThreshold_int(cv::StereoBM* instance, int textureThreshold, ResultVoid* ocvrs_return) {
		try {
			instance->setTextureThreshold(textureThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoBM_getUniquenessRatio_const(const cv::StereoBM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getUniquenessRatio();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoBM_setUniquenessRatio_int(cv::StereoBM* instance, int uniquenessRatio, ResultVoid* ocvrs_return) {
		try {
			instance->setUniquenessRatio(uniquenessRatio);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoBM_getSmallerBlockSize_const(const cv::StereoBM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSmallerBlockSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoBM_setSmallerBlockSize_int(cv::StereoBM* instance, int blockSize, ResultVoid* ocvrs_return) {
		try {
			instance->setSmallerBlockSize(blockSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoBM_getROI1_const(const cv::StereoBM* instance, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->getROI1();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoBM_setROI1_Rect(cv::StereoBM* instance, cv::Rect* roi1, ResultVoid* ocvrs_return) {
		try {
			instance->setROI1(*roi1);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoBM_getROI2_const(const cv::StereoBM* instance, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->getROI2();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoBM_setROI2_Rect(cv::StereoBM* instance, cv::Rect* roi2, ResultVoid* ocvrs_return) {
		try {
			instance->setROI2(*roi2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoBM_create_int_int(int numDisparities, int blockSize, Result<cv::Ptr<cv::StereoBM>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::StereoBM> ret = cv::StereoBM::create(numDisparities, blockSize);
			Ok(new cv::Ptr<cv::StereoBM>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoBM_create(Result<cv::Ptr<cv::StereoBM>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::StereoBM> ret = cv::StereoBM::create();
			Ok(new cv::Ptr<cv::StereoBM>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_StereoBM_to_Algorithm(cv::StereoBM* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::StereoMatcher* cv_StereoBM_to_StereoMatcher(cv::StereoBM* instance) {
			return dynamic_cast<cv::StereoMatcher*>(instance);
	}
	
	void cv_StereoBM_delete(cv::StereoBM* instance) {
			delete instance;
	}
	
	void cv_StereoMatcher_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::StereoMatcher* instance, const cv::_InputArray* left, const cv::_InputArray* right, const cv::_OutputArray* disparity, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*left, *right, *disparity);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoMatcher_getMinDisparity_const(const cv::StereoMatcher* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMinDisparity();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoMatcher_setMinDisparity_int(cv::StereoMatcher* instance, int minDisparity, ResultVoid* ocvrs_return) {
		try {
			instance->setMinDisparity(minDisparity);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoMatcher_getNumDisparities_const(const cv::StereoMatcher* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumDisparities();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoMatcher_setNumDisparities_int(cv::StereoMatcher* instance, int numDisparities, ResultVoid* ocvrs_return) {
		try {
			instance->setNumDisparities(numDisparities);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoMatcher_getBlockSize_const(const cv::StereoMatcher* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getBlockSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoMatcher_setBlockSize_int(cv::StereoMatcher* instance, int blockSize, ResultVoid* ocvrs_return) {
		try {
			instance->setBlockSize(blockSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoMatcher_getSpeckleWindowSize_const(const cv::StereoMatcher* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSpeckleWindowSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoMatcher_setSpeckleWindowSize_int(cv::StereoMatcher* instance, int speckleWindowSize, ResultVoid* ocvrs_return) {
		try {
			instance->setSpeckleWindowSize(speckleWindowSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoMatcher_getSpeckleRange_const(const cv::StereoMatcher* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSpeckleRange();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoMatcher_setSpeckleRange_int(cv::StereoMatcher* instance, int speckleRange, ResultVoid* ocvrs_return) {
		try {
			instance->setSpeckleRange(speckleRange);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoMatcher_getDisp12MaxDiff_const(const cv::StereoMatcher* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDisp12MaxDiff();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoMatcher_setDisp12MaxDiff_int(cv::StereoMatcher* instance, int disp12MaxDiff, ResultVoid* ocvrs_return) {
		try {
			instance->setDisp12MaxDiff(disp12MaxDiff);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::StereoBM* cv_StereoMatcher_to_StereoBM(cv::StereoMatcher* instance) {
			return dynamic_cast<cv::StereoBM*>(instance);
	}
	
	cv::StereoSGBM* cv_StereoMatcher_to_StereoSGBM(cv::StereoMatcher* instance) {
			return dynamic_cast<cv::StereoSGBM*>(instance);
	}
	
	cv::Algorithm* cv_StereoMatcher_to_Algorithm(cv::StereoMatcher* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_StereoMatcher_delete(cv::StereoMatcher* instance) {
			delete instance;
	}
	
	void cv_StereoSGBM_getPreFilterCap_const(const cv::StereoSGBM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPreFilterCap();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoSGBM_setPreFilterCap_int(cv::StereoSGBM* instance, int preFilterCap, ResultVoid* ocvrs_return) {
		try {
			instance->setPreFilterCap(preFilterCap);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoSGBM_getUniquenessRatio_const(const cv::StereoSGBM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getUniquenessRatio();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoSGBM_setUniquenessRatio_int(cv::StereoSGBM* instance, int uniquenessRatio, ResultVoid* ocvrs_return) {
		try {
			instance->setUniquenessRatio(uniquenessRatio);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoSGBM_getP1_const(const cv::StereoSGBM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getP1();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoSGBM_setP1_int(cv::StereoSGBM* instance, int P1, ResultVoid* ocvrs_return) {
		try {
			instance->setP1(P1);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoSGBM_getP2_const(const cv::StereoSGBM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getP2();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoSGBM_setP2_int(cv::StereoSGBM* instance, int P2, ResultVoid* ocvrs_return) {
		try {
			instance->setP2(P2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoSGBM_getMode_const(const cv::StereoSGBM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMode();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoSGBM_setMode_int(cv::StereoSGBM* instance, int mode, ResultVoid* ocvrs_return) {
		try {
			instance->setMode(mode);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoSGBM_create_int_int_int_int_int_int_int_int_int_int_int(int minDisparity, int numDisparities, int blockSize, int P1, int P2, int disp12MaxDiff, int preFilterCap, int uniquenessRatio, int speckleWindowSize, int speckleRange, int mode, Result<cv::Ptr<cv::StereoSGBM>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::StereoSGBM> ret = cv::StereoSGBM::create(minDisparity, numDisparities, blockSize, P1, P2, disp12MaxDiff, preFilterCap, uniquenessRatio, speckleWindowSize, speckleRange, mode);
			Ok(new cv::Ptr<cv::StereoSGBM>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_StereoSGBM_create(Result<cv::Ptr<cv::StereoSGBM>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::StereoSGBM> ret = cv::StereoSGBM::create();
			Ok(new cv::Ptr<cv::StereoSGBM>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_StereoSGBM_to_Algorithm(cv::StereoSGBM* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::StereoMatcher* cv_StereoSGBM_to_StereoMatcher(cv::StereoSGBM* instance) {
			return dynamic_cast<cv::StereoMatcher*>(instance);
	}
	
	void cv_StereoSGBM_delete(cv::StereoSGBM* instance) {
			delete instance;
	}
	
	void cv_UsacParams_UsacParams(Result<cv::UsacParams>* ocvrs_return) {
		try {
			cv::UsacParams ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
}
