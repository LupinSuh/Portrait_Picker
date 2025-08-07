#include "ocvrs_common.hpp"
#include <opencv2/ximgproc.hpp>
#include "ximgproc_types.hpp"

extern "C" {
	void cv_ximgproc_AdaptiveManifoldFilter_filter_const__InputArrayR_const__OutputArrayR_const__InputArrayR(cv::ximgproc::AdaptiveManifoldFilter* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* joint, ResultVoid* ocvrs_return) {
		try {
			instance->filter(*src, *dst, *joint);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_AdaptiveManifoldFilter_filter_const__InputArrayR_const__OutputArrayR(cv::ximgproc::AdaptiveManifoldFilter* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->filter(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_AdaptiveManifoldFilter_collectGarbage(cv::ximgproc::AdaptiveManifoldFilter* instance, ResultVoid* ocvrs_return) {
		try {
			instance->collectGarbage();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_AdaptiveManifoldFilter_create(Result<cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter> ret = cv::ximgproc::AdaptiveManifoldFilter::create();
			Ok(new cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_AdaptiveManifoldFilter_getSigmaS_const(const cv::ximgproc::AdaptiveManifoldFilter* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSigmaS();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_AdaptiveManifoldFilter_setSigmaS_double(cv::ximgproc::AdaptiveManifoldFilter* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setSigmaS(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_AdaptiveManifoldFilter_getSigmaR_const(const cv::ximgproc::AdaptiveManifoldFilter* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSigmaR();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_AdaptiveManifoldFilter_setSigmaR_double(cv::ximgproc::AdaptiveManifoldFilter* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setSigmaR(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_AdaptiveManifoldFilter_getTreeHeight_const(const cv::ximgproc::AdaptiveManifoldFilter* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getTreeHeight();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_AdaptiveManifoldFilter_setTreeHeight_int(cv::ximgproc::AdaptiveManifoldFilter* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setTreeHeight(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_AdaptiveManifoldFilter_getPCAIterations_const(const cv::ximgproc::AdaptiveManifoldFilter* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPCAIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_AdaptiveManifoldFilter_setPCAIterations_int(cv::ximgproc::AdaptiveManifoldFilter* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setPCAIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_AdaptiveManifoldFilter_getAdjustOutliers_const(const cv::ximgproc::AdaptiveManifoldFilter* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getAdjustOutliers();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_AdaptiveManifoldFilter_setAdjustOutliers_bool(cv::ximgproc::AdaptiveManifoldFilter* instance, bool val, ResultVoid* ocvrs_return) {
		try {
			instance->setAdjustOutliers(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_AdaptiveManifoldFilter_getUseRNG_const(const cv::ximgproc::AdaptiveManifoldFilter* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseRNG();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_AdaptiveManifoldFilter_setUseRNG_bool(cv::ximgproc::AdaptiveManifoldFilter* instance, bool val, ResultVoid* ocvrs_return) {
		try {
			instance->setUseRNG(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_ximgproc_AdaptiveManifoldFilter_to_Algorithm(cv::ximgproc::AdaptiveManifoldFilter* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_ximgproc_AdaptiveManifoldFilter_delete(cv::ximgproc::AdaptiveManifoldFilter* instance) {
			delete instance;
	}
	
	void cv_ximgproc_ContourFitting_ContourFitting_int_int(int ctr, int fd, Result<cv::ximgproc::ContourFitting*>* ocvrs_return) {
		try {
			cv::ximgproc::ContourFitting* ret = new cv::ximgproc::ContourFitting(ctr, fd);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_ContourFitting_ContourFitting(Result<cv::ximgproc::ContourFitting*>* ocvrs_return) {
		try {
			cv::ximgproc::ContourFitting* ret = new cv::ximgproc::ContourFitting();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_ContourFitting_estimateTransformation_const__InputArrayR_const__InputArrayR_const__OutputArrayR_doubleX_bool(cv::ximgproc::ContourFitting* instance, const cv::_InputArray* src, const cv::_InputArray* dst, const cv::_OutputArray* alphaPhiST, double* dist, bool fdContour, ResultVoid* ocvrs_return) {
		try {
			instance->estimateTransformation(*src, *dst, *alphaPhiST, dist, fdContour);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_ContourFitting_estimateTransformation_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::ximgproc::ContourFitting* instance, const cv::_InputArray* src, const cv::_InputArray* dst, const cv::_OutputArray* alphaPhiST, ResultVoid* ocvrs_return) {
		try {
			instance->estimateTransformation(*src, *dst, *alphaPhiST);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_ContourFitting_estimateTransformation_const__InputArrayR_const__InputArrayR_const__OutputArrayR_doubleR_bool(cv::ximgproc::ContourFitting* instance, const cv::_InputArray* src, const cv::_InputArray* dst, const cv::_OutputArray* alphaPhiST, double* dist, bool fdContour, ResultVoid* ocvrs_return) {
		try {
			instance->estimateTransformation(*src, *dst, *alphaPhiST, *dist, fdContour);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_ContourFitting_estimateTransformation_const__InputArrayR_const__InputArrayR_const__OutputArrayR_doubleR(cv::ximgproc::ContourFitting* instance, const cv::_InputArray* src, const cv::_InputArray* dst, const cv::_OutputArray* alphaPhiST, double* dist, ResultVoid* ocvrs_return) {
		try {
			instance->estimateTransformation(*src, *dst, *alphaPhiST, *dist);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_ContourFitting_setCtrSize_int(cv::ximgproc::ContourFitting* instance, int n, ResultVoid* ocvrs_return) {
		try {
			instance->setCtrSize(n);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_ContourFitting_setFDSize_int(cv::ximgproc::ContourFitting* instance, int n, ResultVoid* ocvrs_return) {
		try {
			instance->setFDSize(n);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_ContourFitting_getCtrSize(cv::ximgproc::ContourFitting* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getCtrSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_ContourFitting_getFDSize(cv::ximgproc::ContourFitting* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFDSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_ximgproc_ContourFitting_to_Algorithm(cv::ximgproc::ContourFitting* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_ximgproc_ContourFitting_delete(cv::ximgproc::ContourFitting* instance) {
			delete instance;
	}
	
	void cv_ximgproc_DTFilter_filter_const__InputArrayR_const__OutputArrayR_int(cv::ximgproc::DTFilter* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, int dDepth, ResultVoid* ocvrs_return) {
		try {
			instance->filter(*src, *dst, dDepth);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_DTFilter_filter_const__InputArrayR_const__OutputArrayR(cv::ximgproc::DTFilter* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->filter(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_ximgproc_DTFilter_to_Algorithm(cv::ximgproc::DTFilter* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_ximgproc_DTFilter_delete(cv::ximgproc::DTFilter* instance) {
			delete instance;
	}
	
	void cv_ximgproc_DisparityFilter_filter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Rect_const__InputArrayR(cv::ximgproc::DisparityFilter* instance, const cv::_InputArray* disparity_map_left, const cv::_InputArray* left_view, const cv::_OutputArray* filtered_disparity_map, const cv::_InputArray* disparity_map_right, cv::Rect* ROI, const cv::_InputArray* right_view, ResultVoid* ocvrs_return) {
		try {
			instance->filter(*disparity_map_left, *left_view, *filtered_disparity_map, *disparity_map_right, *ROI, *right_view);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_DisparityFilter_filter_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::ximgproc::DisparityFilter* instance, const cv::_InputArray* disparity_map_left, const cv::_InputArray* left_view, const cv::_OutputArray* filtered_disparity_map, ResultVoid* ocvrs_return) {
		try {
			instance->filter(*disparity_map_left, *left_view, *filtered_disparity_map);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::ximgproc::DisparityWLSFilter* cv_ximgproc_DisparityFilter_to_DisparityWLSFilter(cv::ximgproc::DisparityFilter* instance) {
			return dynamic_cast<cv::ximgproc::DisparityWLSFilter*>(instance);
	}
	
	cv::Algorithm* cv_ximgproc_DisparityFilter_to_Algorithm(cv::ximgproc::DisparityFilter* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_ximgproc_DisparityFilter_delete(cv::ximgproc::DisparityFilter* instance) {
			delete instance;
	}
	
	void cv_ximgproc_DisparityWLSFilter_getLambda(cv::ximgproc::DisparityWLSFilter* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getLambda();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_DisparityWLSFilter_setLambda_double(cv::ximgproc::DisparityWLSFilter* instance, double _lambda, ResultVoid* ocvrs_return) {
		try {
			instance->setLambda(_lambda);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_DisparityWLSFilter_getSigmaColor(cv::ximgproc::DisparityWLSFilter* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSigmaColor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_DisparityWLSFilter_setSigmaColor_double(cv::ximgproc::DisparityWLSFilter* instance, double _sigma_color, ResultVoid* ocvrs_return) {
		try {
			instance->setSigmaColor(_sigma_color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_DisparityWLSFilter_getLRCthresh(cv::ximgproc::DisparityWLSFilter* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getLRCthresh();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_DisparityWLSFilter_setLRCthresh_int(cv::ximgproc::DisparityWLSFilter* instance, int _LRC_thresh, ResultVoid* ocvrs_return) {
		try {
			instance->setLRCthresh(_LRC_thresh);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_DisparityWLSFilter_getDepthDiscontinuityRadius(cv::ximgproc::DisparityWLSFilter* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDepthDiscontinuityRadius();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_DisparityWLSFilter_setDepthDiscontinuityRadius_int(cv::ximgproc::DisparityWLSFilter* instance, int _disc_radius, ResultVoid* ocvrs_return) {
		try {
			instance->setDepthDiscontinuityRadius(_disc_radius);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_DisparityWLSFilter_getConfidenceMap(cv::ximgproc::DisparityWLSFilter* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getConfidenceMap();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_DisparityWLSFilter_getROI(cv::ximgproc::DisparityWLSFilter* instance, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->getROI();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_ximgproc_DisparityWLSFilter_to_Algorithm(cv::ximgproc::DisparityWLSFilter* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::ximgproc::DisparityFilter* cv_ximgproc_DisparityWLSFilter_to_DisparityFilter(cv::ximgproc::DisparityWLSFilter* instance) {
			return dynamic_cast<cv::ximgproc::DisparityFilter*>(instance);
	}
	
	void cv_ximgproc_DisparityWLSFilter_delete(cv::ximgproc::DisparityWLSFilter* instance) {
			delete instance;
	}
	
	void cv_ximgproc_EdgeAwareInterpolator_setCostMap_const_MatR(cv::ximgproc::EdgeAwareInterpolator* instance, const cv::Mat* _costMap, ResultVoid* ocvrs_return) {
		try {
			instance->setCostMap(*_costMap);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeAwareInterpolator_setK_int(cv::ximgproc::EdgeAwareInterpolator* instance, int _k, ResultVoid* ocvrs_return) {
		try {
			instance->setK(_k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeAwareInterpolator_getK(cv::ximgproc::EdgeAwareInterpolator* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getK();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeAwareInterpolator_setSigma_float(cv::ximgproc::EdgeAwareInterpolator* instance, float _sigma, ResultVoid* ocvrs_return) {
		try {
			instance->setSigma(_sigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeAwareInterpolator_getSigma(cv::ximgproc::EdgeAwareInterpolator* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeAwareInterpolator_setLambda_float(cv::ximgproc::EdgeAwareInterpolator* instance, float _lambda, ResultVoid* ocvrs_return) {
		try {
			instance->setLambda(_lambda);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeAwareInterpolator_getLambda(cv::ximgproc::EdgeAwareInterpolator* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getLambda();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeAwareInterpolator_setUsePostProcessing_bool(cv::ximgproc::EdgeAwareInterpolator* instance, bool _use_post_proc, ResultVoid* ocvrs_return) {
		try {
			instance->setUsePostProcessing(_use_post_proc);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeAwareInterpolator_getUsePostProcessing(cv::ximgproc::EdgeAwareInterpolator* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUsePostProcessing();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeAwareInterpolator_setFGSLambda_float(cv::ximgproc::EdgeAwareInterpolator* instance, float _lambda, ResultVoid* ocvrs_return) {
		try {
			instance->setFGSLambda(_lambda);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeAwareInterpolator_getFGSLambda(cv::ximgproc::EdgeAwareInterpolator* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getFGSLambda();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeAwareInterpolator_setFGSSigma_float(cv::ximgproc::EdgeAwareInterpolator* instance, float _sigma, ResultVoid* ocvrs_return) {
		try {
			instance->setFGSSigma(_sigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeAwareInterpolator_getFGSSigma(cv::ximgproc::EdgeAwareInterpolator* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getFGSSigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_ximgproc_EdgeAwareInterpolator_to_Algorithm(cv::ximgproc::EdgeAwareInterpolator* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::ximgproc::SparseMatchInterpolator* cv_ximgproc_EdgeAwareInterpolator_to_SparseMatchInterpolator(cv::ximgproc::EdgeAwareInterpolator* instance) {
			return dynamic_cast<cv::ximgproc::SparseMatchInterpolator*>(instance);
	}
	
	void cv_ximgproc_EdgeAwareInterpolator_delete(cv::ximgproc::EdgeAwareInterpolator* instance) {
			delete instance;
	}
	
	void cv_ximgproc_EdgeBoxes_getBoundingBoxes_const__InputArrayR_const__InputArrayR_vectorLRectGR_const__OutputArrayR(cv::ximgproc::EdgeBoxes* instance, const cv::_InputArray* edge_map, const cv::_InputArray* orientation_map, std::vector<cv::Rect>* boxes, const cv::_OutputArray* scores, ResultVoid* ocvrs_return) {
		try {
			instance->getBoundingBoxes(*edge_map, *orientation_map, *boxes, *scores);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeBoxes_getBoundingBoxes_const__InputArrayR_const__InputArrayR_vectorLRectGR(cv::ximgproc::EdgeBoxes* instance, const cv::_InputArray* edge_map, const cv::_InputArray* orientation_map, std::vector<cv::Rect>* boxes, ResultVoid* ocvrs_return) {
		try {
			instance->getBoundingBoxes(*edge_map, *orientation_map, *boxes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeBoxes_getAlpha_const(const cv::ximgproc::EdgeBoxes* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getAlpha();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeBoxes_setAlpha_float(cv::ximgproc::EdgeBoxes* instance, float value, ResultVoid* ocvrs_return) {
		try {
			instance->setAlpha(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeBoxes_getBeta_const(const cv::ximgproc::EdgeBoxes* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getBeta();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeBoxes_setBeta_float(cv::ximgproc::EdgeBoxes* instance, float value, ResultVoid* ocvrs_return) {
		try {
			instance->setBeta(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeBoxes_getEta_const(const cv::ximgproc::EdgeBoxes* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getEta();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeBoxes_setEta_float(cv::ximgproc::EdgeBoxes* instance, float value, ResultVoid* ocvrs_return) {
		try {
			instance->setEta(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeBoxes_getMinScore_const(const cv::ximgproc::EdgeBoxes* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMinScore();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeBoxes_setMinScore_float(cv::ximgproc::EdgeBoxes* instance, float value, ResultVoid* ocvrs_return) {
		try {
			instance->setMinScore(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeBoxes_getMaxBoxes_const(const cv::ximgproc::EdgeBoxes* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxBoxes();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeBoxes_setMaxBoxes_int(cv::ximgproc::EdgeBoxes* instance, int value, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxBoxes(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeBoxes_getEdgeMinMag_const(const cv::ximgproc::EdgeBoxes* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getEdgeMinMag();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeBoxes_setEdgeMinMag_float(cv::ximgproc::EdgeBoxes* instance, float value, ResultVoid* ocvrs_return) {
		try {
			instance->setEdgeMinMag(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeBoxes_getEdgeMergeThr_const(const cv::ximgproc::EdgeBoxes* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getEdgeMergeThr();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeBoxes_setEdgeMergeThr_float(cv::ximgproc::EdgeBoxes* instance, float value, ResultVoid* ocvrs_return) {
		try {
			instance->setEdgeMergeThr(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeBoxes_getClusterMinMag_const(const cv::ximgproc::EdgeBoxes* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getClusterMinMag();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeBoxes_setClusterMinMag_float(cv::ximgproc::EdgeBoxes* instance, float value, ResultVoid* ocvrs_return) {
		try {
			instance->setClusterMinMag(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeBoxes_getMaxAspectRatio_const(const cv::ximgproc::EdgeBoxes* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMaxAspectRatio();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeBoxes_setMaxAspectRatio_float(cv::ximgproc::EdgeBoxes* instance, float value, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxAspectRatio(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeBoxes_getMinBoxArea_const(const cv::ximgproc::EdgeBoxes* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMinBoxArea();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeBoxes_setMinBoxArea_float(cv::ximgproc::EdgeBoxes* instance, float value, ResultVoid* ocvrs_return) {
		try {
			instance->setMinBoxArea(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeBoxes_getGamma_const(const cv::ximgproc::EdgeBoxes* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getGamma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeBoxes_setGamma_float(cv::ximgproc::EdgeBoxes* instance, float value, ResultVoid* ocvrs_return) {
		try {
			instance->setGamma(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeBoxes_getKappa_const(const cv::ximgproc::EdgeBoxes* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getKappa();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeBoxes_setKappa_float(cv::ximgproc::EdgeBoxes* instance, float value, ResultVoid* ocvrs_return) {
		try {
			instance->setKappa(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_ximgproc_EdgeBoxes_to_Algorithm(cv::ximgproc::EdgeBoxes* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_ximgproc_EdgeBoxes_delete(cv::ximgproc::EdgeBoxes* instance) {
			delete instance;
	}
	
	void cv_ximgproc_EdgeDrawing_detectEdges_const__InputArrayR(cv::ximgproc::EdgeDrawing* instance, const cv::_InputArray* src, ResultVoid* ocvrs_return) {
		try {
			instance->detectEdges(*src);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeDrawing_getEdgeImage_const__OutputArrayR(cv::ximgproc::EdgeDrawing* instance, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->getEdgeImage(*dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeDrawing_getGradientImage_const__OutputArrayR(cv::ximgproc::EdgeDrawing* instance, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->getGradientImage(*dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeDrawing_getSegments(cv::ximgproc::EdgeDrawing* instance, Result<std::vector<std::vector<cv::Point>>*>* ocvrs_return) {
		try {
			std::vector<std::vector<cv::Point>> ret = instance->getSegments();
			Ok(new std::vector<std::vector<cv::Point>>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeDrawing_getSegmentIndicesOfLines_const(const cv::ximgproc::EdgeDrawing* instance, Result<std::vector<int>*>* ocvrs_return) {
		try {
			std::vector<int> ret = instance->getSegmentIndicesOfLines();
			Ok(new std::vector<int>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeDrawing_detectLines_const__OutputArrayR(cv::ximgproc::EdgeDrawing* instance, const cv::_OutputArray* lines, ResultVoid* ocvrs_return) {
		try {
			instance->detectLines(*lines);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeDrawing_detectEllipses_const__OutputArrayR(cv::ximgproc::EdgeDrawing* instance, const cv::_OutputArray* ellipses, ResultVoid* ocvrs_return) {
		try {
			instance->detectEllipses(*ellipses);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeDrawing_setParams_const_ParamsR(cv::ximgproc::EdgeDrawing* instance, const cv::ximgproc::EdgeDrawing::Params* parameters, ResultVoid* ocvrs_return) {
		try {
			instance->setParams(*parameters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeDrawing_propParams_const(const cv::ximgproc::EdgeDrawing* instance, cv::ximgproc::EdgeDrawing::Params* ocvrs_return) {
			cv::ximgproc::EdgeDrawing::Params ret = instance->params;
			*ocvrs_return = ret;
	}
	
	void cv_ximgproc_EdgeDrawing_propParams_const_Params(cv::ximgproc::EdgeDrawing* instance, const cv::ximgproc::EdgeDrawing::Params* val) {
			instance->params = *val;
	}
	
	cv::Algorithm* cv_ximgproc_EdgeDrawing_to_Algorithm(cv::ximgproc::EdgeDrawing* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_ximgproc_EdgeDrawing_delete(cv::ximgproc::EdgeDrawing* instance) {
			delete instance;
	}
	
	void cv_ximgproc_EdgeDrawing_Params_Params(Result<cv::ximgproc::EdgeDrawing::Params>* ocvrs_return) {
		try {
			cv::ximgproc::EdgeDrawing::Params ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeDrawing_Params_read_const_FileNodeR(cv::ximgproc::EdgeDrawing::Params* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_EdgeDrawing_Params_write_const_FileStorageR(const cv::ximgproc::EdgeDrawing::Params* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_FastBilateralSolverFilter_filter_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::ximgproc::FastBilateralSolverFilter* instance, const cv::_InputArray* src, const cv::_InputArray* confidence, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->filter(*src, *confidence, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_ximgproc_FastBilateralSolverFilter_to_Algorithm(cv::ximgproc::FastBilateralSolverFilter* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_ximgproc_FastBilateralSolverFilter_delete(cv::ximgproc::FastBilateralSolverFilter* instance) {
			delete instance;
	}
	
	void cv_ximgproc_FastGlobalSmootherFilter_filter_const__InputArrayR_const__OutputArrayR(cv::ximgproc::FastGlobalSmootherFilter* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->filter(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_ximgproc_FastGlobalSmootherFilter_to_Algorithm(cv::ximgproc::FastGlobalSmootherFilter* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_ximgproc_FastGlobalSmootherFilter_delete(cv::ximgproc::FastGlobalSmootherFilter* instance) {
			delete instance;
	}
	
	void cv_ximgproc_FastLineDetector_detect_const__InputArrayR_const__OutputArrayR(cv::ximgproc::FastLineDetector* instance, const cv::_InputArray* image, const cv::_OutputArray* lines, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*image, *lines);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_FastLineDetector_drawSegments_const__InputOutputArrayR_const__InputArrayR_bool_Scalar_int(cv::ximgproc::FastLineDetector* instance, const cv::_InputOutputArray* image, const cv::_InputArray* lines, bool draw_arrow, cv::Scalar* linecolor, int linethickness, ResultVoid* ocvrs_return) {
		try {
			instance->drawSegments(*image, *lines, draw_arrow, *linecolor, linethickness);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_FastLineDetector_drawSegments_const__InputOutputArrayR_const__InputArrayR(cv::ximgproc::FastLineDetector* instance, const cv::_InputOutputArray* image, const cv::_InputArray* lines, ResultVoid* ocvrs_return) {
		try {
			instance->drawSegments(*image, *lines);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_ximgproc_FastLineDetector_to_Algorithm(cv::ximgproc::FastLineDetector* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_ximgproc_FastLineDetector_delete(cv::ximgproc::FastLineDetector* instance) {
			delete instance;
	}
	
	void cv_ximgproc_GuidedFilter_filter_const__InputArrayR_const__OutputArrayR_int(cv::ximgproc::GuidedFilter* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, int dDepth, ResultVoid* ocvrs_return) {
		try {
			instance->filter(*src, *dst, dDepth);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_GuidedFilter_filter_const__InputArrayR_const__OutputArrayR(cv::ximgproc::GuidedFilter* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->filter(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_ximgproc_GuidedFilter_to_Algorithm(cv::ximgproc::GuidedFilter* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_ximgproc_GuidedFilter_delete(cv::ximgproc::GuidedFilter* instance) {
			delete instance;
	}
	
	void cv_ximgproc_RFFeatureGetter_getFeatures_const_const_MatR_MatR_const_int_const_int_const_int_const_int_const_int(const cv::ximgproc::RFFeatureGetter* instance, const cv::Mat* src, cv::Mat* features, const int gnrmRad, const int gsmthRad, const int shrink, const int outNum, const int gradNum, ResultVoid* ocvrs_return) {
		try {
			instance->getFeatures(*src, *features, gnrmRad, gsmthRad, shrink, outNum, gradNum);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_ximgproc_RFFeatureGetter_to_Algorithm(cv::ximgproc::RFFeatureGetter* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_ximgproc_RFFeatureGetter_delete(cv::ximgproc::RFFeatureGetter* instance) {
			delete instance;
	}
	
	void cv_ximgproc_RICInterpolator_setK_int(cv::ximgproc::RICInterpolator* instance, int k, ResultVoid* ocvrs_return) {
		try {
			instance->setK(k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_setK(cv::ximgproc::RICInterpolator* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setK();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_getK_const(const cv::ximgproc::RICInterpolator* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getK();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_setCostMap_const_MatR(cv::ximgproc::RICInterpolator* instance, const cv::Mat* costMap, ResultVoid* ocvrs_return) {
		try {
			instance->setCostMap(*costMap);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_setSuperpixelSize_int(cv::ximgproc::RICInterpolator* instance, int spSize, ResultVoid* ocvrs_return) {
		try {
			instance->setSuperpixelSize(spSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_setSuperpixelSize(cv::ximgproc::RICInterpolator* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setSuperpixelSize();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_getSuperpixelSize_const(const cv::ximgproc::RICInterpolator* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSuperpixelSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_setSuperpixelNNCnt_int(cv::ximgproc::RICInterpolator* instance, int spNN, ResultVoid* ocvrs_return) {
		try {
			instance->setSuperpixelNNCnt(spNN);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_setSuperpixelNNCnt(cv::ximgproc::RICInterpolator* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setSuperpixelNNCnt();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_getSuperpixelNNCnt_const(const cv::ximgproc::RICInterpolator* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSuperpixelNNCnt();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_setSuperpixelRuler_float(cv::ximgproc::RICInterpolator* instance, float ruler, ResultVoid* ocvrs_return) {
		try {
			instance->setSuperpixelRuler(ruler);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_setSuperpixelRuler(cv::ximgproc::RICInterpolator* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setSuperpixelRuler();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_getSuperpixelRuler_const(const cv::ximgproc::RICInterpolator* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSuperpixelRuler();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_setSuperpixelMode_int(cv::ximgproc::RICInterpolator* instance, int mode, ResultVoid* ocvrs_return) {
		try {
			instance->setSuperpixelMode(mode);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_setSuperpixelMode(cv::ximgproc::RICInterpolator* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setSuperpixelMode();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_getSuperpixelMode_const(const cv::ximgproc::RICInterpolator* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSuperpixelMode();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_setAlpha_float(cv::ximgproc::RICInterpolator* instance, float alpha, ResultVoid* ocvrs_return) {
		try {
			instance->setAlpha(alpha);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_setAlpha(cv::ximgproc::RICInterpolator* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setAlpha();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_getAlpha_const(const cv::ximgproc::RICInterpolator* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getAlpha();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_setModelIter_int(cv::ximgproc::RICInterpolator* instance, int modelIter, ResultVoid* ocvrs_return) {
		try {
			instance->setModelIter(modelIter);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_setModelIter(cv::ximgproc::RICInterpolator* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setModelIter();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_getModelIter_const(const cv::ximgproc::RICInterpolator* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getModelIter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_setRefineModels_bool(cv::ximgproc::RICInterpolator* instance, bool refineModles, ResultVoid* ocvrs_return) {
		try {
			instance->setRefineModels(refineModles);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_setRefineModels(cv::ximgproc::RICInterpolator* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setRefineModels();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_getRefineModels_const(const cv::ximgproc::RICInterpolator* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getRefineModels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_setMaxFlow_float(cv::ximgproc::RICInterpolator* instance, float maxFlow, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxFlow(maxFlow);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_setMaxFlow(cv::ximgproc::RICInterpolator* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxFlow();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_getMaxFlow_const(const cv::ximgproc::RICInterpolator* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMaxFlow();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_setUseVariationalRefinement_bool(cv::ximgproc::RICInterpolator* instance, bool use_variational_refinement, ResultVoid* ocvrs_return) {
		try {
			instance->setUseVariationalRefinement(use_variational_refinement);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_setUseVariationalRefinement(cv::ximgproc::RICInterpolator* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setUseVariationalRefinement();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_getUseVariationalRefinement_const(const cv::ximgproc::RICInterpolator* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseVariationalRefinement();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_setUseGlobalSmootherFilter_bool(cv::ximgproc::RICInterpolator* instance, bool use_FGS, ResultVoid* ocvrs_return) {
		try {
			instance->setUseGlobalSmootherFilter(use_FGS);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_setUseGlobalSmootherFilter(cv::ximgproc::RICInterpolator* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setUseGlobalSmootherFilter();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_getUseGlobalSmootherFilter_const(const cv::ximgproc::RICInterpolator* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseGlobalSmootherFilter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_setFGSLambda_float(cv::ximgproc::RICInterpolator* instance, float lambda, ResultVoid* ocvrs_return) {
		try {
			instance->setFGSLambda(lambda);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_setFGSLambda(cv::ximgproc::RICInterpolator* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setFGSLambda();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_getFGSLambda_const(const cv::ximgproc::RICInterpolator* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getFGSLambda();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_setFGSSigma_float(cv::ximgproc::RICInterpolator* instance, float sigma, ResultVoid* ocvrs_return) {
		try {
			instance->setFGSSigma(sigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_setFGSSigma(cv::ximgproc::RICInterpolator* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setFGSSigma();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RICInterpolator_getFGSSigma_const(const cv::ximgproc::RICInterpolator* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getFGSSigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_ximgproc_RICInterpolator_to_Algorithm(cv::ximgproc::RICInterpolator* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::ximgproc::SparseMatchInterpolator* cv_ximgproc_RICInterpolator_to_SparseMatchInterpolator(cv::ximgproc::RICInterpolator* instance) {
			return dynamic_cast<cv::ximgproc::SparseMatchInterpolator*>(instance);
	}
	
	void cv_ximgproc_RICInterpolator_delete(cv::ximgproc::RICInterpolator* instance) {
			delete instance;
	}
	
	void cv_ximgproc_RidgeDetectionFilter_create_int_int_int_int_int_double_double_int(int ddepth, int dx, int dy, int ksize, int out_dtype, double scale, double delta, int borderType, Result<cv::Ptr<cv::ximgproc::RidgeDetectionFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::RidgeDetectionFilter> ret = cv::ximgproc::RidgeDetectionFilter::create(ddepth, dx, dy, ksize, out_dtype, scale, delta, borderType);
			Ok(new cv::Ptr<cv::ximgproc::RidgeDetectionFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RidgeDetectionFilter_create(Result<cv::Ptr<cv::ximgproc::RidgeDetectionFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::RidgeDetectionFilter> ret = cv::ximgproc::RidgeDetectionFilter::create();
			Ok(new cv::Ptr<cv::ximgproc::RidgeDetectionFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_RidgeDetectionFilter_getRidgeFilteredImage_const__InputArrayR_const__OutputArrayR(cv::ximgproc::RidgeDetectionFilter* instance, const cv::_InputArray* _img, const cv::_OutputArray* out, ResultVoid* ocvrs_return) {
		try {
			instance->getRidgeFilteredImage(*_img, *out);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_ximgproc_RidgeDetectionFilter_to_Algorithm(cv::ximgproc::RidgeDetectionFilter* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_ximgproc_RidgeDetectionFilter_delete(cv::ximgproc::RidgeDetectionFilter* instance) {
			delete instance;
	}
	
	void cv_ximgproc_ScanSegment_getNumberOfSuperpixels(cv::ximgproc::ScanSegment* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumberOfSuperpixels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_ScanSegment_iterate_const__InputArrayR(cv::ximgproc::ScanSegment* instance, const cv::_InputArray* img, ResultVoid* ocvrs_return) {
		try {
			instance->iterate(*img);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_ScanSegment_getLabels_const__OutputArrayR(cv::ximgproc::ScanSegment* instance, const cv::_OutputArray* labels_out, ResultVoid* ocvrs_return) {
		try {
			instance->getLabels(*labels_out);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_ScanSegment_getLabelContourMask_const__OutputArrayR_bool(cv::ximgproc::ScanSegment* instance, const cv::_OutputArray* image, bool thick_line, ResultVoid* ocvrs_return) {
		try {
			instance->getLabelContourMask(*image, thick_line);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_ScanSegment_getLabelContourMask_const__OutputArrayR(cv::ximgproc::ScanSegment* instance, const cv::_OutputArray* image, ResultVoid* ocvrs_return) {
		try {
			instance->getLabelContourMask(*image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_ximgproc_ScanSegment_to_Algorithm(cv::ximgproc::ScanSegment* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_ximgproc_ScanSegment_delete(cv::ximgproc::ScanSegment* instance) {
			delete instance;
	}
	
	void cv_ximgproc_SparseMatchInterpolator_interpolate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::ximgproc::SparseMatchInterpolator* instance, const cv::_InputArray* from_image, const cv::_InputArray* from_points, const cv::_InputArray* to_image, const cv::_InputArray* to_points, const cv::_OutputArray* dense_flow, ResultVoid* ocvrs_return) {
		try {
			instance->interpolate(*from_image, *from_points, *to_image, *to_points, *dense_flow);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::ximgproc::EdgeAwareInterpolator* cv_ximgproc_SparseMatchInterpolator_to_EdgeAwareInterpolator(cv::ximgproc::SparseMatchInterpolator* instance) {
			return dynamic_cast<cv::ximgproc::EdgeAwareInterpolator*>(instance);
	}
	
	cv::ximgproc::RICInterpolator* cv_ximgproc_SparseMatchInterpolator_to_RICInterpolator(cv::ximgproc::SparseMatchInterpolator* instance) {
			return dynamic_cast<cv::ximgproc::RICInterpolator*>(instance);
	}
	
	cv::Algorithm* cv_ximgproc_SparseMatchInterpolator_to_Algorithm(cv::ximgproc::SparseMatchInterpolator* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_ximgproc_SparseMatchInterpolator_delete(cv::ximgproc::SparseMatchInterpolator* instance) {
			delete instance;
	}
	
	void cv_ximgproc_StructuredEdgeDetection_detectEdges_const_const__InputArrayR_const__OutputArrayR(const cv::ximgproc::StructuredEdgeDetection* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->detectEdges(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_StructuredEdgeDetection_computeOrientation_const_const__InputArrayR_const__OutputArrayR(const cv::ximgproc::StructuredEdgeDetection* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->computeOrientation(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_StructuredEdgeDetection_edgesNms_const_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_float_bool(const cv::ximgproc::StructuredEdgeDetection* instance, const cv::_InputArray* edge_image, const cv::_InputArray* orientation_image, const cv::_OutputArray* dst, int r, int s, float m, bool isParallel, ResultVoid* ocvrs_return) {
		try {
			instance->edgesNms(*edge_image, *orientation_image, *dst, r, s, m, isParallel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_StructuredEdgeDetection_edgesNms_const_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::ximgproc::StructuredEdgeDetection* instance, const cv::_InputArray* edge_image, const cv::_InputArray* orientation_image, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->edgesNms(*edge_image, *orientation_image, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_ximgproc_StructuredEdgeDetection_to_Algorithm(cv::ximgproc::StructuredEdgeDetection* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_ximgproc_StructuredEdgeDetection_delete(cv::ximgproc::StructuredEdgeDetection* instance) {
			delete instance;
	}
	
	void cv_ximgproc_SuperpixelLSC_getNumberOfSuperpixels_const(const cv::ximgproc::SuperpixelLSC* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumberOfSuperpixels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_SuperpixelLSC_iterate_int(cv::ximgproc::SuperpixelLSC* instance, int num_iterations, ResultVoid* ocvrs_return) {
		try {
			instance->iterate(num_iterations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_SuperpixelLSC_iterate(cv::ximgproc::SuperpixelLSC* instance, ResultVoid* ocvrs_return) {
		try {
			instance->iterate();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_SuperpixelLSC_getLabels_const_const__OutputArrayR(const cv::ximgproc::SuperpixelLSC* instance, const cv::_OutputArray* labels_out, ResultVoid* ocvrs_return) {
		try {
			instance->getLabels(*labels_out);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_SuperpixelLSC_getLabelContourMask_const_const__OutputArrayR_bool(const cv::ximgproc::SuperpixelLSC* instance, const cv::_OutputArray* image, bool thick_line, ResultVoid* ocvrs_return) {
		try {
			instance->getLabelContourMask(*image, thick_line);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_SuperpixelLSC_getLabelContourMask_const_const__OutputArrayR(const cv::ximgproc::SuperpixelLSC* instance, const cv::_OutputArray* image, ResultVoid* ocvrs_return) {
		try {
			instance->getLabelContourMask(*image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_SuperpixelLSC_enforceLabelConnectivity_int(cv::ximgproc::SuperpixelLSC* instance, int min_element_size, ResultVoid* ocvrs_return) {
		try {
			instance->enforceLabelConnectivity(min_element_size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_SuperpixelLSC_enforceLabelConnectivity(cv::ximgproc::SuperpixelLSC* instance, ResultVoid* ocvrs_return) {
		try {
			instance->enforceLabelConnectivity();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_ximgproc_SuperpixelLSC_to_Algorithm(cv::ximgproc::SuperpixelLSC* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_ximgproc_SuperpixelLSC_delete(cv::ximgproc::SuperpixelLSC* instance) {
			delete instance;
	}
	
	void cv_ximgproc_SuperpixelSEEDS_getNumberOfSuperpixels(cv::ximgproc::SuperpixelSEEDS* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumberOfSuperpixels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_SuperpixelSEEDS_iterate_const__InputArrayR_int(cv::ximgproc::SuperpixelSEEDS* instance, const cv::_InputArray* img, int num_iterations, ResultVoid* ocvrs_return) {
		try {
			instance->iterate(*img, num_iterations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_SuperpixelSEEDS_iterate_const__InputArrayR(cv::ximgproc::SuperpixelSEEDS* instance, const cv::_InputArray* img, ResultVoid* ocvrs_return) {
		try {
			instance->iterate(*img);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_SuperpixelSEEDS_getLabels_const__OutputArrayR(cv::ximgproc::SuperpixelSEEDS* instance, const cv::_OutputArray* labels_out, ResultVoid* ocvrs_return) {
		try {
			instance->getLabels(*labels_out);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_SuperpixelSEEDS_getLabelContourMask_const__OutputArrayR_bool(cv::ximgproc::SuperpixelSEEDS* instance, const cv::_OutputArray* image, bool thick_line, ResultVoid* ocvrs_return) {
		try {
			instance->getLabelContourMask(*image, thick_line);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_SuperpixelSEEDS_getLabelContourMask_const__OutputArrayR(cv::ximgproc::SuperpixelSEEDS* instance, const cv::_OutputArray* image, ResultVoid* ocvrs_return) {
		try {
			instance->getLabelContourMask(*image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_ximgproc_SuperpixelSEEDS_to_Algorithm(cv::ximgproc::SuperpixelSEEDS* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_ximgproc_SuperpixelSEEDS_delete(cv::ximgproc::SuperpixelSEEDS* instance) {
			delete instance;
	}
	
	void cv_ximgproc_SuperpixelSLIC_getNumberOfSuperpixels_const(const cv::ximgproc::SuperpixelSLIC* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumberOfSuperpixels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_SuperpixelSLIC_iterate_int(cv::ximgproc::SuperpixelSLIC* instance, int num_iterations, ResultVoid* ocvrs_return) {
		try {
			instance->iterate(num_iterations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_SuperpixelSLIC_iterate(cv::ximgproc::SuperpixelSLIC* instance, ResultVoid* ocvrs_return) {
		try {
			instance->iterate();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_SuperpixelSLIC_getLabels_const_const__OutputArrayR(const cv::ximgproc::SuperpixelSLIC* instance, const cv::_OutputArray* labels_out, ResultVoid* ocvrs_return) {
		try {
			instance->getLabels(*labels_out);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_SuperpixelSLIC_getLabelContourMask_const_const__OutputArrayR_bool(const cv::ximgproc::SuperpixelSLIC* instance, const cv::_OutputArray* image, bool thick_line, ResultVoid* ocvrs_return) {
		try {
			instance->getLabelContourMask(*image, thick_line);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_SuperpixelSLIC_getLabelContourMask_const_const__OutputArrayR(const cv::ximgproc::SuperpixelSLIC* instance, const cv::_OutputArray* image, ResultVoid* ocvrs_return) {
		try {
			instance->getLabelContourMask(*image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_SuperpixelSLIC_enforceLabelConnectivity_int(cv::ximgproc::SuperpixelSLIC* instance, int min_element_size, ResultVoid* ocvrs_return) {
		try {
			instance->enforceLabelConnectivity(min_element_size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_SuperpixelSLIC_enforceLabelConnectivity(cv::ximgproc::SuperpixelSLIC* instance, ResultVoid* ocvrs_return) {
		try {
			instance->enforceLabelConnectivity();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_ximgproc_SuperpixelSLIC_to_Algorithm(cv::ximgproc::SuperpixelSLIC* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_ximgproc_SuperpixelSLIC_delete(cv::ximgproc::SuperpixelSLIC* instance) {
			delete instance;
	}
	
	void cv_ximgproc_segmentation_GraphSegmentation_processImage_const__InputArrayR_const__OutputArrayR(cv::ximgproc::segmentation::GraphSegmentation* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->processImage(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_segmentation_GraphSegmentation_setSigma_double(cv::ximgproc::segmentation::GraphSegmentation* instance, double sigma, ResultVoid* ocvrs_return) {
		try {
			instance->setSigma(sigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_segmentation_GraphSegmentation_getSigma(cv::ximgproc::segmentation::GraphSegmentation* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_segmentation_GraphSegmentation_setK_float(cv::ximgproc::segmentation::GraphSegmentation* instance, float k, ResultVoid* ocvrs_return) {
		try {
			instance->setK(k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_segmentation_GraphSegmentation_getK(cv::ximgproc::segmentation::GraphSegmentation* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getK();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_segmentation_GraphSegmentation_setMinSize_int(cv::ximgproc::segmentation::GraphSegmentation* instance, int min_size, ResultVoid* ocvrs_return) {
		try {
			instance->setMinSize(min_size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_segmentation_GraphSegmentation_getMinSize(cv::ximgproc::segmentation::GraphSegmentation* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMinSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_ximgproc_segmentation_GraphSegmentation_to_Algorithm(cv::ximgproc::segmentation::GraphSegmentation* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_ximgproc_segmentation_GraphSegmentation_delete(cv::ximgproc::segmentation::GraphSegmentation* instance) {
			delete instance;
	}
	
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_setBaseImage_const__InputArrayR(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, const cv::_InputArray* img, ResultVoid* ocvrs_return) {
		try {
			instance->setBaseImage(*img);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_switchToSingleStrategy_int_float(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, int k, float sigma, ResultVoid* ocvrs_return) {
		try {
			instance->switchToSingleStrategy(k, sigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_switchToSingleStrategy(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, ResultVoid* ocvrs_return) {
		try {
			instance->switchToSingleStrategy();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_switchToSelectiveSearchFast_int_int_float(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, int base_k, int inc_k, float sigma, ResultVoid* ocvrs_return) {
		try {
			instance->switchToSelectiveSearchFast(base_k, inc_k, sigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_switchToSelectiveSearchFast(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, ResultVoid* ocvrs_return) {
		try {
			instance->switchToSelectiveSearchFast();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_switchToSelectiveSearchQuality_int_int_float(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, int base_k, int inc_k, float sigma, ResultVoid* ocvrs_return) {
		try {
			instance->switchToSelectiveSearchQuality(base_k, inc_k, sigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_switchToSelectiveSearchQuality(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, ResultVoid* ocvrs_return) {
		try {
			instance->switchToSelectiveSearchQuality();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_addImage_const__InputArrayR(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, const cv::_InputArray* img, ResultVoid* ocvrs_return) {
		try {
			instance->addImage(*img);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_clearImages(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, ResultVoid* ocvrs_return) {
		try {
			instance->clearImages();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_addGraphSegmentation_PtrLGraphSegmentationG(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>* g, ResultVoid* ocvrs_return) {
		try {
			instance->addGraphSegmentation(*g);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_clearGraphSegmentations(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, ResultVoid* ocvrs_return) {
		try {
			instance->clearGraphSegmentations();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_addStrategy_PtrLSelectiveSearchSegmentationStrategyG(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* s, ResultVoid* ocvrs_return) {
		try {
			instance->addStrategy(*s);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_clearStrategies(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, ResultVoid* ocvrs_return) {
		try {
			instance->clearStrategies();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_process_vectorLRectGR(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, std::vector<cv::Rect>* rects, ResultVoid* ocvrs_return) {
		try {
			instance->process(*rects);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_ximgproc_segmentation_SelectiveSearchSegmentation_to_Algorithm(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_delete(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance) {
			delete instance;
	}
	
	void cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_setImage_const__InputArrayR_const__InputArrayR_const__InputArrayR_int(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* instance, const cv::_InputArray* img, const cv::_InputArray* regions, const cv::_InputArray* sizes, int image_id, ResultVoid* ocvrs_return) {
		try {
			instance->setImage(*img, *regions, *sizes, image_id);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_setImage_const__InputArrayR_const__InputArrayR_const__InputArrayR(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* instance, const cv::_InputArray* img, const cv::_InputArray* regions, const cv::_InputArray* sizes, ResultVoid* ocvrs_return) {
		try {
			instance->setImage(*img, *regions, *sizes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_get_int_int(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* instance, int r1, int r2, Result<float>* ocvrs_return) {
		try {
			float ret = instance->get(r1, r2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_merge_int_int(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* instance, int r1, int r2, ResultVoid* ocvrs_return) {
		try {
			instance->merge(r1, r2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor* cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_to_SelectiveSearchSegmentationStrategyColor(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* instance) {
			return dynamic_cast<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor*>(instance);
	}
	
	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill* cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_to_SelectiveSearchSegmentationStrategyFill(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* instance) {
			return dynamic_cast<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill*>(instance);
	}
	
	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple* cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_to_SelectiveSearchSegmentationStrategyMultiple(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* instance) {
			return dynamic_cast<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple*>(instance);
	}
	
	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize* cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_to_SelectiveSearchSegmentationStrategySize(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* instance) {
			return dynamic_cast<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize*>(instance);
	}
	
	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture* cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_to_SelectiveSearchSegmentationStrategyTexture(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* instance) {
			return dynamic_cast<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture*>(instance);
	}
	
	cv::Algorithm* cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_to_Algorithm(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_delete(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* instance) {
			delete instance;
	}
	
	cv::Algorithm* cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyColor_to_Algorithm(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyColor_to_SelectiveSearchSegmentationStrategy(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor* instance) {
			return dynamic_cast<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy*>(instance);
	}
	
	void cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyColor_delete(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor* instance) {
			delete instance;
	}
	
	cv::Algorithm* cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyFill_to_Algorithm(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyFill_to_SelectiveSearchSegmentationStrategy(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill* instance) {
			return dynamic_cast<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy*>(instance);
	}
	
	void cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyFill_delete(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill* instance) {
			delete instance;
	}
	
	void cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultiple_addStrategy_PtrLSelectiveSearchSegmentationStrategyG_float(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple* instance, cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* g, float weight, ResultVoid* ocvrs_return) {
		try {
			instance->addStrategy(*g, weight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultiple_clearStrategies(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple* instance, ResultVoid* ocvrs_return) {
		try {
			instance->clearStrategies();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultiple_to_Algorithm(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultiple_to_SelectiveSearchSegmentationStrategy(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple* instance) {
			return dynamic_cast<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy*>(instance);
	}
	
	void cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultiple_delete(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple* instance) {
			delete instance;
	}
	
	cv::Algorithm* cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategySize_to_Algorithm(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategySize_to_SelectiveSearchSegmentationStrategy(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize* instance) {
			return dynamic_cast<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy*>(instance);
	}
	
	void cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategySize_delete(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize* instance) {
			delete instance;
	}
	
	cv::Algorithm* cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyTexture_to_Algorithm(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyTexture_to_SelectiveSearchSegmentationStrategy(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture* instance) {
			return dynamic_cast<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy*>(instance);
	}
	
	void cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyTexture_delete(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture* instance) {
			delete instance;
	}
	
}
