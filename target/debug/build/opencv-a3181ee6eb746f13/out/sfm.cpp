#include "sfm.hpp"
#include "sfm_types.hpp"

extern "C" {
	void cv_sfm_BaseSFM_run_const__InputArrayR(cv::sfm::BaseSFM* instance, const cv::_InputArray* points2d, ResultVoid* ocvrs_return) {
		try {
			instance->run(*points2d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_sfm_BaseSFM_run_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(cv::sfm::BaseSFM* instance, const cv::_InputArray* points2d, const cv::_InputOutputArray* K, const cv::_OutputArray* Rs, const cv::_OutputArray* Ts, const cv::_OutputArray* points3d, ResultVoid* ocvrs_return) {
		try {
			instance->run(*points2d, *K, *Rs, *Ts, *points3d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_sfm_BaseSFM_run_const_vectorLStringGR(cv::sfm::BaseSFM* instance, const std::vector<cv::String>* images, ResultVoid* ocvrs_return) {
		try {
			instance->run(*images);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_sfm_BaseSFM_run_const_vectorLStringGR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(cv::sfm::BaseSFM* instance, const std::vector<cv::String>* images, const cv::_InputOutputArray* K, const cv::_OutputArray* Rs, const cv::_OutputArray* Ts, const cv::_OutputArray* points3d, ResultVoid* ocvrs_return) {
		try {
			instance->run(*images, *K, *Rs, *Ts, *points3d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_sfm_BaseSFM_getError_const(const cv::sfm::BaseSFM* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getError();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_sfm_BaseSFM_getPoints_const__OutputArrayR(cv::sfm::BaseSFM* instance, const cv::_OutputArray* points3d, ResultVoid* ocvrs_return) {
		try {
			instance->getPoints(*points3d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_sfm_BaseSFM_getIntrinsics_const(const cv::sfm::BaseSFM* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getIntrinsics();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_sfm_BaseSFM_getCameras_const__OutputArrayR_const__OutputArrayR(cv::sfm::BaseSFM* instance, const cv::_OutputArray* Rs, const cv::_OutputArray* Ts, ResultVoid* ocvrs_return) {
		try {
			instance->getCameras(*Rs, *Ts);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_sfm_BaseSFM_setReconstructionOptions_const_libmv_ReconstructionOptionsR(cv::sfm::BaseSFM* instance, const cv::sfm::libmv_ReconstructionOptions* libmv_reconstruction_options, ResultVoid* ocvrs_return) {
		try {
			instance->setReconstructionOptions(*libmv_reconstruction_options);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_sfm_BaseSFM_setCameraIntrinsicOptions_const_libmv_CameraIntrinsicsOptionsR(cv::sfm::BaseSFM* instance, const cv::sfm::libmv_CameraIntrinsicsOptions* libmv_camera_intrinsics_options, ResultVoid* ocvrs_return) {
		try {
			instance->setCameraIntrinsicOptions(*libmv_camera_intrinsics_options);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::sfm::SFMLibmvEuclideanReconstruction* cv_sfm_BaseSFM_to_SFMLibmvEuclideanReconstruction(cv::sfm::BaseSFM* instance) {
			return dynamic_cast<cv::sfm::SFMLibmvEuclideanReconstruction*>(instance);
	}
	
	void cv_sfm_BaseSFM_delete(cv::sfm::BaseSFM* instance) {
			delete instance;
	}
	
	void cv_sfm_SFMLibmvEuclideanReconstruction_run_const__InputArrayR(cv::sfm::SFMLibmvEuclideanReconstruction* instance, const cv::_InputArray* points2d, ResultVoid* ocvrs_return) {
		try {
			instance->run(*points2d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_sfm_SFMLibmvEuclideanReconstruction_run_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(cv::sfm::SFMLibmvEuclideanReconstruction* instance, const cv::_InputArray* points2d, const cv::_InputOutputArray* K, const cv::_OutputArray* Rs, const cv::_OutputArray* Ts, const cv::_OutputArray* points3d, ResultVoid* ocvrs_return) {
		try {
			instance->run(*points2d, *K, *Rs, *Ts, *points3d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_sfm_SFMLibmvEuclideanReconstruction_run_const_vectorLStringGR(cv::sfm::SFMLibmvEuclideanReconstruction* instance, const std::vector<cv::String>* images, ResultVoid* ocvrs_return) {
		try {
			instance->run(*images);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_sfm_SFMLibmvEuclideanReconstruction_run_const_vectorLStringGR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(cv::sfm::SFMLibmvEuclideanReconstruction* instance, const std::vector<cv::String>* images, const cv::_InputOutputArray* K, const cv::_OutputArray* Rs, const cv::_OutputArray* Ts, const cv::_OutputArray* points3d, ResultVoid* ocvrs_return) {
		try {
			instance->run(*images, *K, *Rs, *Ts, *points3d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_sfm_SFMLibmvEuclideanReconstruction_getError_const(const cv::sfm::SFMLibmvEuclideanReconstruction* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getError();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_sfm_SFMLibmvEuclideanReconstruction_getPoints_const__OutputArrayR(cv::sfm::SFMLibmvEuclideanReconstruction* instance, const cv::_OutputArray* points3d, ResultVoid* ocvrs_return) {
		try {
			instance->getPoints(*points3d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_sfm_SFMLibmvEuclideanReconstruction_getIntrinsics_const(const cv::sfm::SFMLibmvEuclideanReconstruction* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getIntrinsics();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_sfm_SFMLibmvEuclideanReconstruction_getCameras_const__OutputArrayR_const__OutputArrayR(cv::sfm::SFMLibmvEuclideanReconstruction* instance, const cv::_OutputArray* Rs, const cv::_OutputArray* Ts, ResultVoid* ocvrs_return) {
		try {
			instance->getCameras(*Rs, *Ts);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_sfm_SFMLibmvEuclideanReconstruction_setReconstructionOptions_const_libmv_ReconstructionOptionsR(cv::sfm::SFMLibmvEuclideanReconstruction* instance, const cv::sfm::libmv_ReconstructionOptions* libmv_reconstruction_options, ResultVoid* ocvrs_return) {
		try {
			instance->setReconstructionOptions(*libmv_reconstruction_options);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_sfm_SFMLibmvEuclideanReconstruction_setCameraIntrinsicOptions_const_libmv_CameraIntrinsicsOptionsR(cv::sfm::SFMLibmvEuclideanReconstruction* instance, const cv::sfm::libmv_CameraIntrinsicsOptions* libmv_camera_intrinsics_options, ResultVoid* ocvrs_return) {
		try {
			instance->setCameraIntrinsicOptions(*libmv_camera_intrinsics_options);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_sfm_SFMLibmvEuclideanReconstruction_create_const_libmv_CameraIntrinsicsOptionsR_const_libmv_ReconstructionOptionsR(const cv::sfm::libmv_CameraIntrinsicsOptions* camera_instrinsic_options, const cv::sfm::libmv_ReconstructionOptions* reconstruction_options, Result<cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction> ret = cv::sfm::SFMLibmvEuclideanReconstruction::create(*camera_instrinsic_options, *reconstruction_options);
			Ok(new cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_sfm_SFMLibmvEuclideanReconstruction_create(Result<cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction> ret = cv::sfm::SFMLibmvEuclideanReconstruction::create();
			Ok(new cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::sfm::BaseSFM* cv_sfm_SFMLibmvEuclideanReconstruction_to_BaseSFM(cv::sfm::SFMLibmvEuclideanReconstruction* instance) {
			return dynamic_cast<cv::sfm::BaseSFM*>(instance);
	}
	
	void cv_sfm_SFMLibmvEuclideanReconstruction_delete(cv::sfm::SFMLibmvEuclideanReconstruction* instance) {
			delete instance;
	}
	
	void cv_sfm_libmv_CameraIntrinsicsOptions_libmv_CameraIntrinsicsOptions_const_int_const_double_const_double_const_double_const_double_const_double_const_double_const_double_const_double_const_double(const int _distortion_model, const double _focal_length_x, const double _focal_length_y, const double _principal_point_x, const double _principal_point_y, const double _polynomial_k1, const double _polynomial_k2, const double _polynomial_k3, const double _polynomial_p1, const double _polynomial_p2, Result<cv::sfm::libmv_CameraIntrinsicsOptions>* ocvrs_return) {
		try {
			cv::sfm::libmv_CameraIntrinsicsOptions ret(_distortion_model, _focal_length_x, _focal_length_y, _principal_point_x, _principal_point_y, _polynomial_k1, _polynomial_k2, _polynomial_k3, _polynomial_p1, _polynomial_p2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_sfm_libmv_CameraIntrinsicsOptions_libmv_CameraIntrinsicsOptions(Result<cv::sfm::libmv_CameraIntrinsicsOptions>* ocvrs_return) {
		try {
			cv::sfm::libmv_CameraIntrinsicsOptions ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_sfm_libmv_ReconstructionOptions_libmv_ReconstructionOptions_const_int_const_int_const_int_const_int_const_int(const int _keyframe1, const int _keyframe2, const int _refine_intrinsics, const int _select_keyframes, const int _verbosity_level, Result<cv::sfm::libmv_ReconstructionOptions>* ocvrs_return) {
		try {
			cv::sfm::libmv_ReconstructionOptions ret(_keyframe1, _keyframe2, _refine_intrinsics, _select_keyframes, _verbosity_level);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_sfm_libmv_ReconstructionOptions_libmv_ReconstructionOptions(Result<cv::sfm::libmv_ReconstructionOptions>* ocvrs_return) {
		try {
			cv::sfm::libmv_ReconstructionOptions ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
}
