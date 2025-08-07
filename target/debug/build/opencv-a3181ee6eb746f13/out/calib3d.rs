//! # Camera Calibration and 3D Reconstruction
//! 
//! The functions in this section use a so-called pinhole camera model. The view of a scene
//! is obtained by projecting a scene's 3D point ![inline formula](https://latex.codecogs.com/png.latex?P%5Fw) into the image plane using a perspective
//! transformation which forms the corresponding pixel ![inline formula](https://latex.codecogs.com/png.latex?p). Both ![inline formula](https://latex.codecogs.com/png.latex?P%5Fw) and ![inline formula](https://latex.codecogs.com/png.latex?p) are
//! represented in homogeneous coordinates, i.e. as 3D and 2D homogeneous vector respectively. You will
//! find a brief introduction to projective geometry, homogeneous vectors and homogeneous
//! transformations at the end of this section's introduction. For more succinct notation, we often drop
//! the 'homogeneous' and say vector instead of homogeneous vector.
//! 
//! The distortion-free projective transformation given by a  pinhole camera model is shown below.
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?s%20%5C%3B%20p%20%3D%20A%20%5Cbegin%7Bbmatrix%7D%20R%7Ct%20%5Cend%7Bbmatrix%7D%20P%5Fw%2C)
//! 
//! where ![inline formula](https://latex.codecogs.com/png.latex?P%5Fw) is a 3D point expressed with respect to the world coordinate system,
//! ![inline formula](https://latex.codecogs.com/png.latex?p) is a 2D pixel in the image plane, ![inline formula](https://latex.codecogs.com/png.latex?A) is the camera intrinsic matrix,
//! ![inline formula](https://latex.codecogs.com/png.latex?R) and ![inline formula](https://latex.codecogs.com/png.latex?t) are the rotation and translation that describe the change of coordinates from
//! world to camera coordinate systems (or camera frame) and ![inline formula](https://latex.codecogs.com/png.latex?s) is the projective transformation's
//! arbitrary scaling and not part of the camera model.
//! 
//! The camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?A) (notation used as in [Zhang2000](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Zhang2000) and also generally notated
//! as ![inline formula](https://latex.codecogs.com/png.latex?K)) projects 3D points given in the camera coordinate system to 2D pixel coordinates, i.e.
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?p%20%3D%20A%20P%5Fc%2E)
//! 
//! The camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?A) is composed of the focal lengths ![inline formula](https://latex.codecogs.com/png.latex?f%5Fx) and ![inline formula](https://latex.codecogs.com/png.latex?f%5Fy), which are
//! expressed in pixel units, and the principal point ![inline formula](https://latex.codecogs.com/png.latex?%28c%5Fx%2C%20c%5Fy%29), that is usually close to the
//! image center:
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?A%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D%2C)
//! 
//! and thus
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?s%20%5Cbegin%7Bbmatrix%7D%20u%5C%5C%20v%5C%5C%201%20%5Cend%7Bbmatrix%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D%20%5Cbegin%7Bbmatrix%7D%20X%5Fc%5C%5C%20Y%5Fc%5C%5C%20Z%5Fc%20%5Cend%7Bbmatrix%7D%2E)
//! 
//! The matrix of intrinsic parameters does not depend on the scene viewed. So, once estimated, it can
//! be re-used as long as the focal length is fixed (in case of a zoom lens). Thus, if an image from the
//! camera is scaled by a factor, all of these parameters need to be scaled (multiplied/divided,
//! respectively) by the same factor.
//! 
//! The joint rotation-translation matrix ![inline formula](https://latex.codecogs.com/png.latex?%5BR%7Ct%5D) is the matrix product of a projective
//! transformation and a homogeneous transformation. The 3-by-4 projective transformation maps 3D points
//! represented in camera coordinates to 2D points in the image plane and represented in normalized
//! camera coordinates ![inline formula](https://latex.codecogs.com/png.latex?x%27%20%3D%20X%5Fc%20%2F%20Z%5Fc) and ![inline formula](https://latex.codecogs.com/png.latex?y%27%20%3D%20Y%5Fc%20%2F%20Z%5Fc):
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?Z%5Fc%20%5Cbegin%7Bbmatrix%7D%0Ax%27%20%5C%5C%0Ay%27%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0A1%20%26%200%20%26%200%20%26%200%20%5C%5C%0A0%20%26%201%20%26%200%20%26%200%20%5C%5C%0A0%20%26%200%20%26%201%20%26%200%0A%5Cend%7Bbmatrix%7D%0A%5Cbegin%7Bbmatrix%7D%0AX%5Fc%20%5C%5C%0AY%5Fc%20%5C%5C%0AZ%5Fc%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%2E)
//! 
//! The homogeneous transformation is encoded by the extrinsic parameters ![inline formula](https://latex.codecogs.com/png.latex?R) and ![inline formula](https://latex.codecogs.com/png.latex?t) and
//! represents the change of basis from world coordinate system ![inline formula](https://latex.codecogs.com/png.latex?w) to the camera coordinate sytem
//! ![inline formula](https://latex.codecogs.com/png.latex?c). Thus, given the representation of the point ![inline formula](https://latex.codecogs.com/png.latex?P) in world coordinates, ![inline formula](https://latex.codecogs.com/png.latex?P%5Fw), we
//! obtain ![inline formula](https://latex.codecogs.com/png.latex?P)'s representation in the camera coordinate system, ![inline formula](https://latex.codecogs.com/png.latex?P%5Fc), by
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?P%5Fc%20%3D%20%5Cbegin%7Bbmatrix%7D%0AR%20%26%20t%20%5C%5C%0A0%20%26%201%0A%5Cend%7Bbmatrix%7D%20P%5Fw%2C)
//! 
//! This homogeneous transformation is composed out of ![inline formula](https://latex.codecogs.com/png.latex?R), a 3-by-3 rotation matrix, and ![inline formula](https://latex.codecogs.com/png.latex?t), a
//! 3-by-1 translation vector:
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%0AR%20%26%20t%20%5C%5C%0A0%20%26%201%0A%5Cend%7Bbmatrix%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0Ar%5F%7B11%7D%20%26%20r%5F%7B12%7D%20%26%20r%5F%7B13%7D%20%26%20t%5Fx%20%5C%5C%0Ar%5F%7B21%7D%20%26%20r%5F%7B22%7D%20%26%20r%5F%7B23%7D%20%26%20t%5Fy%20%5C%5C%0Ar%5F%7B31%7D%20%26%20r%5F%7B32%7D%20%26%20r%5F%7B33%7D%20%26%20t%5Fz%20%5C%5C%0A0%20%26%200%20%26%200%20%26%201%0A%5Cend%7Bbmatrix%7D%2C%0A)
//! 
//! and therefore
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%0AX%5Fc%20%5C%5C%0AY%5Fc%20%5C%5C%0AZ%5Fc%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0Ar%5F%7B11%7D%20%26%20r%5F%7B12%7D%20%26%20r%5F%7B13%7D%20%26%20t%5Fx%20%5C%5C%0Ar%5F%7B21%7D%20%26%20r%5F%7B22%7D%20%26%20r%5F%7B23%7D%20%26%20t%5Fy%20%5C%5C%0Ar%5F%7B31%7D%20%26%20r%5F%7B32%7D%20%26%20r%5F%7B33%7D%20%26%20t%5Fz%20%5C%5C%0A0%20%26%200%20%26%200%20%26%201%0A%5Cend%7Bbmatrix%7D%0A%5Cbegin%7Bbmatrix%7D%0AX%5Fw%20%5C%5C%0AY%5Fw%20%5C%5C%0AZ%5Fw%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%2E)
//! 
//! Combining the projective transformation and the homogeneous transformation, we obtain the projective
//! transformation that maps 3D points in world coordinates into 2D points in the image plane and in
//! normalized camera coordinates:
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?Z%5Fc%20%5Cbegin%7Bbmatrix%7D%0Ax%27%20%5C%5C%0Ay%27%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%20R%7Ct%20%5Cend%7Bbmatrix%7D%20%5Cbegin%7Bbmatrix%7D%0AX%5Fw%20%5C%5C%0AY%5Fw%20%5C%5C%0AZ%5Fw%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0Ar%5F%7B11%7D%20%26%20r%5F%7B12%7D%20%26%20r%5F%7B13%7D%20%26%20t%5Fx%20%5C%5C%0Ar%5F%7B21%7D%20%26%20r%5F%7B22%7D%20%26%20r%5F%7B23%7D%20%26%20t%5Fy%20%5C%5C%0Ar%5F%7B31%7D%20%26%20r%5F%7B32%7D%20%26%20r%5F%7B33%7D%20%26%20t%5Fz%0A%5Cend%7Bbmatrix%7D%0A%5Cbegin%7Bbmatrix%7D%0AX%5Fw%20%5C%5C%0AY%5Fw%20%5C%5C%0AZ%5Fw%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%2C)
//! 
//! with ![inline formula](https://latex.codecogs.com/png.latex?x%27%20%3D%20X%5Fc%20%2F%20Z%5Fc) and ![inline formula](https://latex.codecogs.com/png.latex?y%27%20%3D%20Y%5Fc%20%2F%20Z%5Fc). Putting the equations for instrincs and extrinsics together, we can write out
//! ![inline formula](https://latex.codecogs.com/png.latex?s%20%5C%3B%20p%20%3D%20A%20%5Cbegin%7Bbmatrix%7D%20R%7Ct%20%5Cend%7Bbmatrix%7D%20P%5Fw) as
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?s%20%5Cbegin%7Bbmatrix%7D%20u%5C%5C%20v%5C%5C%201%20%5Cend%7Bbmatrix%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D%0A%5Cbegin%7Bbmatrix%7D%0Ar%5F%7B11%7D%20%26%20r%5F%7B12%7D%20%26%20r%5F%7B13%7D%20%26%20t%5Fx%20%5C%5C%0Ar%5F%7B21%7D%20%26%20r%5F%7B22%7D%20%26%20r%5F%7B23%7D%20%26%20t%5Fy%20%5C%5C%0Ar%5F%7B31%7D%20%26%20r%5F%7B32%7D%20%26%20r%5F%7B33%7D%20%26%20t%5Fz%0A%5Cend%7Bbmatrix%7D%0A%5Cbegin%7Bbmatrix%7D%0AX%5Fw%20%5C%5C%0AY%5Fw%20%5C%5C%0AZ%5Fw%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%2E)
//! 
//! If ![inline formula](https://latex.codecogs.com/png.latex?Z%5Fc%20%5Cne%200), the transformation above is equivalent to the following,
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%0Au%20%5C%5C%0Av%0A%5Cend%7Bbmatrix%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0Af%5Fx%20X%5Fc%2FZ%5Fc%20%2B%20c%5Fx%20%5C%5C%0Af%5Fy%20Y%5Fc%2FZ%5Fc%20%2B%20c%5Fy%0A%5Cend%7Bbmatrix%7D)
//! 
//! with
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20X%5Fc%5C%5C%20Y%5Fc%5C%5C%20Z%5Fc%20%5Cend%7Bbmatrix%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0AR%7Ct%0A%5Cend%7Bbmatrix%7D%20%5Cbegin%7Bbmatrix%7D%0AX%5Fw%20%5C%5C%0AY%5Fw%20%5C%5C%0AZ%5Fw%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%2E)
//! 
//! The following figure illustrates the pinhole camera model.
//! 
//! ![Pinhole camera model](https://docs.opencv.org/4.12.0/pinhole_camera_model.png)
//! 
//! Real lenses usually have some distortion, mostly radial distortion, and slight tangential distortion.
//! So, the above model is extended as:
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%0Au%20%5C%5C%0Av%0A%5Cend%7Bbmatrix%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0Af%5Fx%20x%27%27%20%2B%20c%5Fx%20%5C%5C%0Af%5Fy%20y%27%27%20%2B%20c%5Fy%0A%5Cend%7Bbmatrix%7D)
//! 
//! where
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%0Ax%27%27%20%5C%5C%0Ay%27%27%0A%5Cend%7Bbmatrix%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0Ax%27%20%5Cfrac%7B1%20%2B%20k%5F1%20r%5E2%20%2B%20k%5F2%20r%5E4%20%2B%20k%5F3%20r%5E6%7D%7B1%20%2B%20k%5F4%20r%5E2%20%2B%20k%5F5%20r%5E4%20%2B%20k%5F6%20r%5E6%7D%20%2B%202%20p%5F1%20x%27%20y%27%20%2B%20p%5F2%28r%5E2%20%2B%202%20x%27%5E2%29%20%2B%20s%5F1%20r%5E2%20%2B%20s%5F2%20r%5E4%20%5C%5C%0Ay%27%20%5Cfrac%7B1%20%2B%20k%5F1%20r%5E2%20%2B%20k%5F2%20r%5E4%20%2B%20k%5F3%20r%5E6%7D%7B1%20%2B%20k%5F4%20r%5E2%20%2B%20k%5F5%20r%5E4%20%2B%20k%5F6%20r%5E6%7D%20%2B%20p%5F1%20%28r%5E2%20%2B%202%20y%27%5E2%29%20%2B%202%20p%5F2%20x%27%20y%27%20%2B%20s%5F3%20r%5E2%20%2B%20s%5F4%20r%5E4%20%5C%5C%0A%5Cend%7Bbmatrix%7D)
//! 
//! with
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?r%5E2%20%3D%20x%27%5E2%20%2B%20y%27%5E2)
//! 
//! and
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%0Ax%27%5C%5C%0Ay%27%0A%5Cend%7Bbmatrix%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0AX%5Fc%2FZ%5Fc%20%5C%5C%0AY%5Fc%2FZ%5Fc%0A%5Cend%7Bbmatrix%7D%2C)
//! 
//! if ![inline formula](https://latex.codecogs.com/png.latex?Z%5Fc%20%5Cne%200).
//! 
//! The distortion parameters are the radial coefficients ![inline formula](https://latex.codecogs.com/png.latex?k%5F1), ![inline formula](https://latex.codecogs.com/png.latex?k%5F2), ![inline formula](https://latex.codecogs.com/png.latex?k%5F3), ![inline formula](https://latex.codecogs.com/png.latex?k%5F4), ![inline formula](https://latex.codecogs.com/png.latex?k%5F5), and ![inline formula](https://latex.codecogs.com/png.latex?k%5F6)
//! ,![inline formula](https://latex.codecogs.com/png.latex?p%5F1) and ![inline formula](https://latex.codecogs.com/png.latex?p%5F2) are the tangential distortion coefficients, and ![inline formula](https://latex.codecogs.com/png.latex?s%5F1), ![inline formula](https://latex.codecogs.com/png.latex?s%5F2), ![inline formula](https://latex.codecogs.com/png.latex?s%5F3), and ![inline formula](https://latex.codecogs.com/png.latex?s%5F4),
//! are the thin prism distortion coefficients. Higher-order coefficients are not considered in OpenCV.
//! 
//! The next figures show two common types of radial distortion: barrel distortion
//! (![inline formula](https://latex.codecogs.com/png.latex?%201%20%2B%20k%5F1%20r%5E2%20%2B%20k%5F2%20r%5E4%20%2B%20k%5F3%20r%5E6%20) monotonically decreasing)
//! and pincushion distortion (![inline formula](https://latex.codecogs.com/png.latex?%201%20%2B%20k%5F1%20r%5E2%20%2B%20k%5F2%20r%5E4%20%2B%20k%5F3%20r%5E6%20) monotonically increasing).
//! Radial distortion is always monotonic for real lenses,
//! and if the estimator produces a non-monotonic result,
//! this should be considered a calibration failure.
//! More generally, radial distortion must be monotonic and the distortion function must be bijective.
//! A failed estimation result may look deceptively good near the image center
//! but will work poorly in e.g. AR/SFM applications.
//! The optimization method used in OpenCV camera calibration does not include these constraints as
//! the framework does not support the required integer programming and polynomial inequalities.
//! See [issue #15992](https://github.com/opencv/opencv/issues/15992) for additional information.
//! 
//! ![](https://docs.opencv.org/4.12.0/distortion_examples.png)
//! ![](https://docs.opencv.org/4.12.0/distortion_examples2.png)
//! 
//! In some cases, the image sensor may be tilted in order to focus an oblique plane in front of the
//! camera (Scheimpflug principle). This can be useful for particle image velocimetry (PIV) or
//! triangulation with a laser fan. The tilt causes a perspective distortion of ![inline formula](https://latex.codecogs.com/png.latex?x%27%27) and
//! ![inline formula](https://latex.codecogs.com/png.latex?y%27%27). This distortion can be modeled in the following way, see e.g. [Louhichi07](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Louhichi07).
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%0Au%20%5C%5C%0Av%0A%5Cend%7Bbmatrix%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0Af%5Fx%20x%27%27%27%20%2B%20c%5Fx%20%5C%5C%0Af%5Fy%20y%27%27%27%20%2B%20c%5Fy%0A%5Cend%7Bbmatrix%7D%2C)
//! 
//! where
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?s%5Cbegin%7Bbmatrix%7D%20x%27%27%27%5C%5C%20y%27%27%27%5C%5C%201%20%5Cend%7Bbmatrix%7D%20%3D%0A%5Cvecthreethree%7BR%5F%7B33%7D%28%5Ctau%5Fx%2C%20%5Ctau%5Fy%29%7D%7B0%7D%7B%2DR%5F%7B13%7D%28%5Ctau%5Fx%2C%20%5Ctau%5Fy%29%7D%0A%7B0%7D%7BR%5F%7B33%7D%28%5Ctau%5Fx%2C%20%5Ctau%5Fy%29%7D%7B%2DR%5F%7B23%7D%28%5Ctau%5Fx%2C%20%5Ctau%5Fy%29%7D%0A%7B0%7D%7B0%7D%7B1%7D%20R%28%5Ctau%5Fx%2C%20%5Ctau%5Fy%29%20%5Cbegin%7Bbmatrix%7D%20x%27%27%5C%5C%20y%27%27%5C%5C%201%20%5Cend%7Bbmatrix%7D)
//! 
//! and the matrix ![inline formula](https://latex.codecogs.com/png.latex?R%28%5Ctau%5Fx%2C%20%5Ctau%5Fy%29) is defined by two rotations with angular parameter
//! ![inline formula](https://latex.codecogs.com/png.latex?%5Ctau%5Fx) and ![inline formula](https://latex.codecogs.com/png.latex?%5Ctau%5Fy), respectively,
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?%0AR%28%5Ctau%5Fx%2C%20%5Ctau%5Fy%29%20%3D%0A%5Cbegin%7Bbmatrix%7D%20%5Ccos%28%5Ctau%5Fy%29%20%26%200%20%26%20%2D%5Csin%28%5Ctau%5Fy%29%5C%5C%200%20%26%201%20%26%200%5C%5C%20%5Csin%28%5Ctau%5Fy%29%20%26%200%20%26%20%5Ccos%28%5Ctau%5Fy%29%20%5Cend%7Bbmatrix%7D%0A%5Cbegin%7Bbmatrix%7D%201%20%26%200%20%26%200%5C%5C%200%20%26%20%5Ccos%28%5Ctau%5Fx%29%20%26%20%5Csin%28%5Ctau%5Fx%29%5C%5C%200%20%26%20%2D%5Csin%28%5Ctau%5Fx%29%20%26%20%5Ccos%28%5Ctau%5Fx%29%20%5Cend%7Bbmatrix%7D%20%3D%0A%5Cbegin%7Bbmatrix%7D%20%5Ccos%28%5Ctau%5Fy%29%20%26%20%5Csin%28%5Ctau%5Fy%29%5Csin%28%5Ctau%5Fx%29%20%26%20%2D%5Csin%28%5Ctau%5Fy%29%5Ccos%28%5Ctau%5Fx%29%5C%5C%200%20%26%20%5Ccos%28%5Ctau%5Fx%29%20%26%20%5Csin%28%5Ctau%5Fx%29%5C%5C%20%5Csin%28%5Ctau%5Fy%29%20%26%20%2D%5Ccos%28%5Ctau%5Fy%29%5Csin%28%5Ctau%5Fx%29%20%26%20%5Ccos%28%5Ctau%5Fy%29%5Ccos%28%5Ctau%5Fx%29%20%5Cend%7Bbmatrix%7D%2E%0A)
//! 
//! In the functions below the coefficients are passed or returned as
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%20%5B%2C%20s%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5B%2C%20%5Ctau%5Fx%2C%20%5Ctau%5Fy%5D%5D%5D%5D%29)
//! 
//! vector. That is, if the vector contains four elements, it means that ![inline formula](https://latex.codecogs.com/png.latex?k%5F3%3D0) . The distortion
//! coefficients do not depend on the scene viewed. Thus, they also belong to the intrinsic camera
//! parameters. And they remain the same regardless of the captured image resolution. If, for example, a
//! camera has been calibrated on images of 320 x 240 resolution, absolutely the same distortion
//! coefficients can be used for 640 x 480 images from the same camera while ![inline formula](https://latex.codecogs.com/png.latex?f%5Fx), ![inline formula](https://latex.codecogs.com/png.latex?f%5Fy),
//! ![inline formula](https://latex.codecogs.com/png.latex?c%5Fx), and ![inline formula](https://latex.codecogs.com/png.latex?c%5Fy) need to be scaled appropriately.
//! 
//! The functions below use the above model to do the following:
//! 
//! *   Project 3D points to the image plane given intrinsic and extrinsic parameters.
//! *   Compute extrinsic parameters given intrinsic parameters, a few 3D points, and their
//! projections.
//! *   Estimate intrinsic and extrinsic camera parameters from several views of a known calibration
//! pattern (every view is described by several 3D-2D point correspondences).
//! *   Estimate the relative position and orientation of the stereo camera "heads" and compute the
//! *rectification* transformation that makes the camera optical axes parallel.
//! 
//! <B> Homogeneous Coordinates </B><br>
//! Homogeneous Coordinates are a system of coordinates that are used in projective geometry. Their use
//! allows to represent points at infinity by finite coordinates and simplifies formulas when compared
//! to the cartesian counterparts, e.g. they have the advantage that affine transformations can be
//! expressed as linear homogeneous transformation.
//! 
//! One obtains the homogeneous vector ![inline formula](https://latex.codecogs.com/png.latex?P%5Fh) by appending a 1 along an n-dimensional cartesian
//! vector ![inline formula](https://latex.codecogs.com/png.latex?P) e.g. for a 3D cartesian vector the mapping ![inline formula](https://latex.codecogs.com/png.latex?P%20%5Crightarrow%20P%5Fh) is:
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%0AX%20%5C%5C%0AY%20%5C%5C%0AZ%0A%5Cend%7Bbmatrix%7D%20%5Crightarrow%20%5Cbegin%7Bbmatrix%7D%0AX%20%5C%5C%0AY%20%5C%5C%0AZ%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%2E)
//! 
//! For the inverse mapping ![inline formula](https://latex.codecogs.com/png.latex?P%5Fh%20%5Crightarrow%20P), one divides all elements of the homogeneous vector
//! by its last element, e.g. for a 3D homogeneous vector one gets its 2D cartesian counterpart by:
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%0AX%20%5C%5C%0AY%20%5C%5C%0AW%0A%5Cend%7Bbmatrix%7D%20%5Crightarrow%20%5Cbegin%7Bbmatrix%7D%0AX%20%2F%20W%20%5C%5C%0AY%20%2F%20W%0A%5Cend%7Bbmatrix%7D%2C)
//! 
//! if ![inline formula](https://latex.codecogs.com/png.latex?W%20%5Cne%200).
//! 
//! Due to this mapping, all multiples ![inline formula](https://latex.codecogs.com/png.latex?k%20P%5Fh), for ![inline formula](https://latex.codecogs.com/png.latex?k%20%5Cne%200), of a homogeneous point represent
//! the same point ![inline formula](https://latex.codecogs.com/png.latex?P%5Fh). An intuitive understanding of this property is that under a projective
//! transformation, all multiples of ![inline formula](https://latex.codecogs.com/png.latex?P%5Fh) are mapped to the same point. This is the physical
//! observation one does for pinhole cameras, as all points along a ray through the camera's pinhole are
//! projected to the same image point, e.g. all points along the red ray in the image of the pinhole
//! camera model above would be mapped to the same image coordinate. This property is also the source
//! for the scale ambiguity s in the equation of the pinhole camera model.
//! 
//! As mentioned, by using homogeneous coordinates we can express any change of basis parameterized by
//! ![inline formula](https://latex.codecogs.com/png.latex?R) and ![inline formula](https://latex.codecogs.com/png.latex?t) as a linear transformation, e.g. for the change of basis from coordinate system
//! 0 to coordinate system 1 becomes:
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?P%5F1%20%3D%20R%20P%5F0%20%2B%20t%20%5Crightarrow%20P%5F%7Bh%5F1%7D%20%3D%20%5Cbegin%7Bbmatrix%7D%0AR%20%26%20t%20%5C%5C%0A0%20%26%201%0A%5Cend%7Bbmatrix%7D%20P%5F%7Bh%5F0%7D%2E)
//! 
//! <B> Homogeneous Transformations, Object frame / Camera frame </B><br>
//! Change of basis or computing the 3D coordinates from one frame to another frame can be achieved easily using
//! the following notation:
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cmathbf%7BX%7D%5Fc%20%3D%20%5Chspace%7B0%2E2em%7D%0A%7B%7D%5E%7Bc%7D%5Cmathbf%7BT%7D%5Fo%20%5Chspace%7B0%2E2em%7D%20%5Cmathbf%7BX%7D%5Fo%0A)
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cbegin%7Bbmatrix%7D%0AX%5Fc%20%5C%5C%0AY%5Fc%20%5C%5C%0AZ%5Fc%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%20%3D%0A%5Cbegin%7Bbmatrix%7D%0A%7B%7D%5E%7Bc%7D%5Cmathbf%7BR%7D%5Fo%20%26%20%7B%7D%5E%7Bc%7D%5Cmathbf%7Bt%7D%5Fo%20%5C%5C%0A0%5F%7B1%20%5Ctimes%203%7D%20%26%201%0A%5Cend%7Bbmatrix%7D%0A%5Cbegin%7Bbmatrix%7D%0AX%5Fo%20%5C%5C%0AY%5Fo%20%5C%5C%0AZ%5Fo%20%5C%5C%0A1%0A%5Cend%7Bbmatrix%7D%0A)
//! 
//! For a 3D points (![inline formula](https://latex.codecogs.com/png.latex?%20%5Cmathbf%7BX%7D%5Fo%20)) expressed in the object frame, the homogeneous transformation matrix
//! ![inline formula](https://latex.codecogs.com/png.latex?%20%7B%7D%5E%7Bc%7D%5Cmathbf%7BT%7D%5Fo%20) allows computing the corresponding coordinate (![inline formula](https://latex.codecogs.com/png.latex?%20%5Cmathbf%7BX%7D%5Fc%20)) in the camera frame.
//! This transformation matrix is composed of a 3x3 rotation matrix ![inline formula](https://latex.codecogs.com/png.latex?%20%7B%7D%5E%7Bc%7D%5Cmathbf%7BR%7D%5Fo%20) and a 3x1 translation vector
//! ![inline formula](https://latex.codecogs.com/png.latex?%20%7B%7D%5E%7Bc%7D%5Cmathbf%7Bt%7D%5Fo%20).
//! The 3x1 translation vector ![inline formula](https://latex.codecogs.com/png.latex?%20%7B%7D%5E%7Bc%7D%5Cmathbf%7Bt%7D%5Fo%20) is the position of the object frame in the camera frame and the
//! 3x3 rotation matrix ![inline formula](https://latex.codecogs.com/png.latex?%20%7B%7D%5E%7Bc%7D%5Cmathbf%7BR%7D%5Fo%20) the orientation of the object frame in the camera frame.
//! 
//! With this simple notation, it is easy to chain the transformations. For instance, to compute the 3D coordinates of a point
//! expressed in the object frame in the world frame can be done with:
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cmathbf%7BX%7D%5Fw%20%3D%20%5Chspace%7B0%2E2em%7D%0A%7B%7D%5E%7Bw%7D%5Cmathbf%7BT%7D%5Fc%20%5Chspace%7B0%2E2em%7D%20%7B%7D%5E%7Bc%7D%5Cmathbf%7BT%7D%5Fo%20%5Chspace%7B0%2E2em%7D%0A%5Cmathbf%7BX%7D%5Fo%20%3D%0A%7B%7D%5E%7Bw%7D%5Cmathbf%7BT%7D%5Fo%20%5Chspace%7B0%2E2em%7D%20%5Cmathbf%7BX%7D%5Fo%0A)
//! 
//! Similarly, computing the inverse transformation can be done with:
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?%0A%5Cmathbf%7BX%7D%5Fo%20%3D%20%5Chspace%7B0%2E2em%7D%0A%7B%7D%5E%7Bo%7D%5Cmathbf%7BT%7D%5Fc%20%5Chspace%7B0%2E2em%7D%20%5Cmathbf%7BX%7D%5Fc%20%3D%0A%5Cleft%28%20%7B%7D%5E%7Bc%7D%5Cmathbf%7BT%7D%5Fo%20%5Cright%29%5E%7B%2D1%7D%20%5Chspace%7B0%2E2em%7D%20%5Cmathbf%7BX%7D%5Fc%0A)
//! 
//! The inverse of an homogeneous transformation matrix is then:
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?%0A%7B%7D%5E%7Bo%7D%5Cmathbf%7BT%7D%5Fc%20%3D%20%5Cleft%28%20%7B%7D%5E%7Bc%7D%5Cmathbf%7BT%7D%5Fo%20%5Cright%29%5E%7B%2D1%7D%20%3D%0A%5Cbegin%7Bbmatrix%7D%0A%7B%7D%5E%7Bc%7D%5Cmathbf%7BR%7D%5E%7B%5Ctop%7D%5Fo%20%26%20%2D%20%5Chspace%7B0%2E2em%7D%20%7B%7D%5E%7Bc%7D%5Cmathbf%7BR%7D%5E%7B%5Ctop%7D%5Fo%20%5Chspace%7B0%2E2em%7D%20%7B%7D%5E%7Bc%7D%5Cmathbf%7Bt%7D%5Fo%20%5C%5C%0A0%5F%7B1%20%5Ctimes%203%7D%20%26%201%0A%5Cend%7Bbmatrix%7D%0A)
//! 
//! One can note that the inverse of a 3x3 rotation matrix is directly its matrix transpose.
//! 
//! ![Perspective projection, from object to camera frame](https://docs.opencv.org/4.12.0/pinhole_homogeneous_transformation.png)
//! 
//! This figure summarizes the whole process. The object pose returned for instance by the [solvePnP] function
//! or pose from fiducial marker detection is this ![inline formula](https://latex.codecogs.com/png.latex?%20%7B%7D%5E%7Bc%7D%5Cmathbf%7BT%7D%5Fo%20) transformation.
//! 
//! The camera intrinsic matrix ![inline formula](https://latex.codecogs.com/png.latex?%20%5Cmathbf%7BK%7D%20) allows projecting the 3D point expressed in the camera frame onto the image plane
//! assuming a perspective projection model (pinhole camera model). Image coordinates extracted from classical image processing functions
//! assume a (u,v) top-left coordinates frame.
//! 
//! \note
//! - for an online video course on this topic, see for instance:
//!   - ["3.3.1. Homogeneous Transformation Matrices", Modern Robotics, Kevin M. Lynch and Frank C. Park](https://modernrobotics.northwestern.edu/nu-gm-book-resource/3-3-1-homogeneous-transformation-matrices/)
//! - the 3x3 rotation matrix is composed of 9 values but describes a 3 dof transformation
//! - some additional properties of the 3x3 rotation matrix are:
//!   - ![inline formula](https://latex.codecogs.com/png.latex?%20%5Cmathrm%7Bdet%7D%20%5Cleft%28%20%5Cmathbf%7BR%7D%20%5Cright%29%20%3D%201%20)
//!   - ![inline formula](https://latex.codecogs.com/png.latex?%20%5Cmathbf%7BR%7D%20%5Cmathbf%7BR%7D%5E%7B%5Ctop%7D%20%3D%20%5Cmathbf%7BR%7D%5E%7B%5Ctop%7D%20%5Cmathbf%7BR%7D%20%3D%20%5Cmathrm%7BI%7D%5F%7B3%20%5Ctimes%203%7D%20)
//!   - interpolating rotation can be done using the [Slerp (spherical linear interpolation)](https://en.wikipedia.org/wiki/Slerp) method
//! - quick conversions between the different rotation formalisms can be done using this [online tool](https://www.andre-gaschler.com/rotationconverter/)
//! 
//! <B> Intrinsic parameters from camera lens specifications </B><br>
//! When dealing with industrial cameras, the camera intrinsic matrix or more precisely ![inline formula](https://latex.codecogs.com/png.latex?%20%5Cleft%28f%5Fx%2C%20f%5Fy%20%5Cright%29%20)
//! can be deduced, approximated from the camera specifications:
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?%0Af%5Fx%20%3D%20%5Cfrac%7Bf%5F%7B%5Ctext%7Bmm%7D%7D%7D%7B%5Ctext%7Bpixel%5Fsize%5Fin%5Fmm%7D%7D%20%3D%20%5Cfrac%7Bf%5F%7B%5Ctext%7Bmm%7D%7D%7D%7B%5Ctext%7Bsensor%5Fsize%5Fin%5Fmm%7D%20%2F%20%5Ctext%7Bnb%5Fpixels%7D%7D%0A)
//! 
//! In a same way, the physical focal length can be deduced from the angular field of view:
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?%0Af%5F%7B%5Ctext%7Bmm%7D%7D%20%3D%20%5Cfrac%7B%5Ctext%7Bsensor%5Fsize%5Fin%5Fmm%7D%7D%7B2%20%5Ctimes%20%5Ctan%7B%5Cfrac%7B%5Ctext%7Bfov%7D%7D%7B2%7D%7D%7D%0A)
//! 
//! This latter conversion can be useful when using a rendering software to mimic a physical camera device.
//! 
//! 
//! Note:
//!    *    See also [calibration_matrix_values]
//! 
//! <B> Additional references, notes </B><br>
//! 
//! Note:
//!    *   Many functions in this module take a camera intrinsic matrix as an input parameter. Although all
//!        functions assume the same structure of this parameter, they may name it differently. The
//!        parameter's description, however, will be clear in that a camera intrinsic matrix with the structure
//!        shown above is required.
//!    *   A calibration sample for 3 cameras in a horizontal position can be found at
//!        opencv_source_code/samples/cpp/3calibration.cpp
//!    *   A calibration sample based on a sequence of images can be found at
//!        opencv_source_code/samples/cpp/calibration.cpp
//!    *   A calibration sample in order to do 3D reconstruction can be found at
//!        opencv_source_code/samples/cpp/build3dmodel.cpp
//!    *   A calibration example on stereo calibration can be found at
//!        opencv_source_code/samples/cpp/stereo_calib.cpp
//!    *   A calibration example on stereo matching can be found at
//!        opencv_source_code/samples/cpp/stereo_match.cpp
//!    *   (Python) A camera calibration sample can be found at
//!        opencv_source_code/samples/python/calibrate.py
//!    # Fisheye camera model
//! 
//!    Definitions: Let P be a point in 3D of coordinates X in the world reference frame (stored in the
//!    matrix X) The coordinate vector of P in the camera reference frame is:
//! 
//!    ![block formula](https://latex.codecogs.com/png.latex?Xc%20%3D%20R%20X%20%2B%20T)
//! 
//!    where R is the rotation matrix corresponding to the rotation vector om: R = rodrigues(om); call x, y
//!    and z the 3 coordinates of Xc:
//! 
//!    ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20x%20%3D%20Xc%5F1%20%5C%5C%20y%20%3D%20Xc%5F2%20%5C%5C%20z%20%3D%20Xc%5F3%20%5Cend%7Barray%7D%20)
//! 
//!    The pinhole projection coordinates of P is [a; b] where
//! 
//!    ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20a%20%3D%20x%20%2F%20z%20%5C%20and%20%5C%20b%20%3D%20y%20%2F%20z%20%5C%5C%20r%5E2%20%3D%20a%5E2%20%2B%20b%5E2%20%5C%5C%20%5Ctheta%20%3D%20atan%28r%29%20%5Cend%7Barray%7D%20)
//! 
//!    Fisheye distortion:
//! 
//!    ![block formula](https://latex.codecogs.com/png.latex?%5Ctheta%5Fd%20%3D%20%5Ctheta%20%281%20%2B%20k%5F1%20%5Ctheta%5E2%20%2B%20k%5F2%20%5Ctheta%5E4%20%2B%20k%5F3%20%5Ctheta%5E6%20%2B%20k%5F4%20%5Ctheta%5E8%29)
//! 
//!    The distorted point coordinates are [x'; y'] where
//! 
//!    ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20x%27%20%3D%20%28%5Ctheta%5Fd%20%2F%20r%29%20a%20%5C%5C%20y%27%20%3D%20%28%5Ctheta%5Fd%20%2F%20r%29%20b%20%5Cend%7Barray%7D%20)
//! 
//!    Finally, conversion into pixel coordinates: The final pixel coordinates vector [u; v] where:
//! 
//!    ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20u%20%3D%20f%5Fx%20%28x%27%20%2B%20%5Calpha%20y%27%29%20%2B%20c%5Fx%20%5C%5C%0A%20%20%20%20v%20%3D%20f%5Fy%20y%27%20%2B%20c%5Fy%20%5Cend%7Barray%7D%20)
//! 
//!    Summary:
//!    Generic camera model [Kannala2006](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Kannala2006) with perspective projection and without distortion correction
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::LMSolver_CallbackTraitConst, super::LMSolver_CallbackTrait, super::LMSolverTraitConst, super::LMSolverTrait, super::StereoMatcherTraitConst, super::StereoMatcherTrait, super::StereoBMTraitConst, super::StereoBMTrait, super::StereoSGBMTraitConst, super::StereoSGBMTrait };
}

pub const CALIB_CB_ACCURACY: i32 = 32;
pub const CALIB_CB_ADAPTIVE_THRESH: i32 = 1;
pub const CALIB_CB_ASYMMETRIC_GRID: i32 = 2;
pub const CALIB_CB_CLUSTERING: i32 = 4;
pub const CALIB_CB_EXHAUSTIVE: i32 = 16;
pub const CALIB_CB_FAST_CHECK: i32 = 8;
pub const CALIB_CB_FILTER_QUADS: i32 = 4;
pub const CALIB_CB_LARGER: i32 = 64;
pub const CALIB_CB_MARKER: i32 = 128;
pub const CALIB_CB_NORMALIZE_IMAGE: i32 = 2;
pub const CALIB_CB_PLAIN: i32 = 256;
pub const CALIB_CB_SYMMETRIC_GRID: i32 = 1;
pub const CALIB_FIX_ASPECT_RATIO: i32 = 2;
pub const CALIB_FIX_FOCAL_LENGTH: i32 = 16;
pub const CALIB_FIX_INTRINSIC: i32 = 256;
pub const CALIB_FIX_K1: i32 = 32;
pub const CALIB_FIX_K2: i32 = 64;
pub const CALIB_FIX_K3: i32 = 128;
pub const CALIB_FIX_K4: i32 = 2048;
pub const CALIB_FIX_K5: i32 = 4096;
pub const CALIB_FIX_K6: i32 = 8192;
pub const CALIB_FIX_PRINCIPAL_POINT: i32 = 4;
pub const CALIB_FIX_S1_S2_S3_S4: i32 = 65536;
pub const CALIB_FIX_TANGENT_DIST: i32 = 2097152;
pub const CALIB_FIX_TAUX_TAUY: i32 = 524288;
/// On-line Hand-Eye Calibration [Andreff99](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Andreff99)
pub const CALIB_HAND_EYE_ANDREFF: i32 = 3;
/// Hand-Eye Calibration Using Dual Quaternions [Daniilidis98](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Daniilidis98)
pub const CALIB_HAND_EYE_DANIILIDIS: i32 = 4;
/// Hand-eye Calibration [Horaud95](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Horaud95)
pub const CALIB_HAND_EYE_HORAUD: i32 = 2;
/// Robot Sensor Calibration: Solving AX = XB on the Euclidean Group [Park94](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Park94)
pub const CALIB_HAND_EYE_PARK: i32 = 1;
/// A New Technique for Fully Autonomous and Efficient 3D Robotics Hand/Eye Calibration [Tsai89](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Tsai89)
pub const CALIB_HAND_EYE_TSAI: i32 = 0;
pub const CALIB_NINTRINSIC: i32 = 18;
pub const CALIB_RATIONAL_MODEL: i32 = 16384;
/// Simultaneous robot-world and hand-eye calibration using dual-quaternions and kronecker product [Li2010SimultaneousRA](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Li2010SimultaneousRA)
pub const CALIB_ROBOT_WORLD_HAND_EYE_LI: i32 = 1;
/// Solving the robot-world/hand-eye calibration problem using the kronecker product [Shah2013SolvingTR](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Shah2013SolvingTR)
pub const CALIB_ROBOT_WORLD_HAND_EYE_SHAH: i32 = 0;
pub const CALIB_SAME_FOCAL_LENGTH: i32 = 512;
pub const CALIB_THIN_PRISM_MODEL: i32 = 32768;
pub const CALIB_TILTED_MODEL: i32 = 262144;
/// for stereoCalibrate
pub const CALIB_USE_EXTRINSIC_GUESS: i32 = 4194304;
pub const CALIB_USE_INTRINSIC_GUESS: i32 = 1;
/// use LU instead of SVD decomposition for solving. much faster but potentially less precise
pub const CALIB_USE_LU: i32 = 131072;
/// use QR instead of SVD decomposition for solving. Faster but potentially less precise
pub const CALIB_USE_QR: i32 = 1048576;
pub const CALIB_ZERO_DISPARITY: i32 = 1024;
pub const CALIB_ZERO_TANGENT_DIST: i32 = 8;
pub const COV_POLISHER: i32 = 3;
pub const CirclesGridFinderParameters_ASYMMETRIC_GRID: i32 = 1;
pub const CirclesGridFinderParameters_SYMMETRIC_GRID: i32 = 0;
/// 7-point algorithm
pub const FM_7POINT: i32 = 1;
/// 8-point algorithm
pub const FM_8POINT: i32 = 2;
/// least-median algorithm. 7-point algorithm is used.
pub const FM_LMEDS: i32 = 4;
/// RANSAC algorithm. It needs at least 15 points. 7-point algorithm is used.
pub const FM_RANSAC: i32 = 8;
pub const Fisheye_CALIB_CHECK_COND: i32 = 4;
pub const Fisheye_CALIB_FIX_FOCAL_LENGTH: i32 = 2048;
pub const Fisheye_CALIB_FIX_INTRINSIC: i32 = 256;
pub const Fisheye_CALIB_FIX_K1: i32 = 16;
pub const Fisheye_CALIB_FIX_K2: i32 = 32;
pub const Fisheye_CALIB_FIX_K3: i32 = 64;
pub const Fisheye_CALIB_FIX_K4: i32 = 128;
pub const Fisheye_CALIB_FIX_PRINCIPAL_POINT: i32 = 512;
pub const Fisheye_CALIB_FIX_SKEW: i32 = 8;
pub const Fisheye_CALIB_RECOMPUTE_EXTRINSIC: i32 = 2;
pub const Fisheye_CALIB_USE_INTRINSIC_GUESS: i32 = 1;
pub const Fisheye_CALIB_ZERO_DISPARITY: i32 = 1024;
/// least-median of squares algorithm
pub const LMEDS: i32 = 4;
pub const LOCAL_OPTIM_GC: i32 = 3;
pub const LOCAL_OPTIM_INNER_AND_ITER_LO: i32 = 2;
pub const LOCAL_OPTIM_INNER_LO: i32 = 1;
pub const LOCAL_OPTIM_NULL: i32 = 0;
pub const LOCAL_OPTIM_SIGMA: i32 = 4;
pub const LSQ_POLISHER: i32 = 1;
pub const MAGSAC: i32 = 2;
pub const NEIGH_FLANN_KNN: i32 = 0;
pub const NEIGH_FLANN_RADIUS: i32 = 2;
pub const NEIGH_GRID: i32 = 1;
pub const NONE_POLISHER: i32 = 0;
pub const PROJ_SPHERICAL_EQRECT: i32 = 1;
pub const PROJ_SPHERICAL_ORTHO: i32 = 0;
/// RANSAC algorithm
pub const RANSAC: i32 = 8;
/// RHO algorithm
pub const RHO: i32 = 16;
pub const SAMPLING_NAPSAC: i32 = 2;
pub const SAMPLING_PROGRESSIVE_NAPSAC: i32 = 1;
pub const SAMPLING_PROSAC: i32 = 3;
pub const SAMPLING_UNIFORM: i32 = 0;
pub const SCORE_METHOD_LMEDS: i32 = 3;
pub const SCORE_METHOD_MAGSAC: i32 = 2;
pub const SCORE_METHOD_MSAC: i32 = 1;
pub const SCORE_METHOD_RANSAC: i32 = 0;
/// An Efficient Algebraic Solution to the Perspective-Three-Point Problem [Ke17](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Ke17)
pub const SOLVEPNP_AP3P: i32 = 5;
/// **Broken implementation. Using this flag will fallback to EPnP.** 
/// 
/// A Direct Least-Squares (DLS) Method for PnP [hesch2011direct](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_hesch2011direct)
pub const SOLVEPNP_DLS: i32 = 3;
/// EPnP: Efficient Perspective-n-Point Camera Pose Estimation [lepetit2009epnp](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_lepetit2009epnp)
pub const SOLVEPNP_EPNP: i32 = 1;
/// Infinitesimal Plane-Based Pose Estimation [Collins14](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Collins14) 
/// 
/// Object points must be coplanar.
pub const SOLVEPNP_IPPE: i32 = 6;
/// Infinitesimal Plane-Based Pose Estimation [Collins14](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Collins14) 
/// 
/// This is a special case suitable for marker pose estimation.
/// 
/// 4 coplanar object points must be defined in the following order:
///   - point 0: [-squareLength / 2,  squareLength / 2, 0]
///   - point 1: [ squareLength / 2,  squareLength / 2, 0]
///   - point 2: [ squareLength / 2, -squareLength / 2, 0]
///   - point 3: [-squareLength / 2, -squareLength / 2, 0]
pub const SOLVEPNP_IPPE_SQUARE: i32 = 7;
/// Pose refinement using non-linear Levenberg-Marquardt minimization scheme [Madsen04](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Madsen04) [Eade13](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Eade13) 
/// 
/// Initial solution for non-planar "objectPoints" needs at least 6 points and uses the DLT algorithm. 
/// 
/// Initial solution for planar "objectPoints" needs at least 4 points and uses pose from homography decomposition.
pub const SOLVEPNP_ITERATIVE: i32 = 0;
/// Used for count
pub const SOLVEPNP_MAX_COUNT: i32 = 9;
/// Complete Solution Classification for the Perspective-Three-Point Problem [gao2003complete](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_gao2003complete)
pub const SOLVEPNP_P3P: i32 = 2;
/// SQPnP: A Consistently Fast and Globally OptimalSolution to the Perspective-n-Point Problem [Terzakis2020SQPnP](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Terzakis2020SQPnP)
pub const SOLVEPNP_SQPNP: i32 = 8;
/// **Broken implementation. Using this flag will fallback to EPnP.** 
/// 
/// Exhaustive Linearization for Robust Camera Pose and Focal Length Estimation [penate2013exhaustive](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_penate2013exhaustive)
pub const SOLVEPNP_UPNP: i32 = 4;
/// Normalized response pre-filter
pub const StereoBM_PREFILTER_NORMALIZED_RESPONSE: i32 = 0;
/// X-Sobel pre-filter
pub const StereoBM_PREFILTER_XSOBEL: i32 = 1;
pub const StereoMatcher_DISP_SCALE: i32 = 16;
pub const StereoMatcher_DISP_SHIFT: i32 = 4;
pub const StereoSGBM_MODE_HH: i32 = 1;
pub const StereoSGBM_MODE_HH4: i32 = 3;
pub const StereoSGBM_MODE_SGBM: i32 = 0;
pub const StereoSGBM_MODE_SGBM_3WAY: i32 = 2;
/// USAC, accurate settings
pub const USAC_ACCURATE: i32 = 36;
/// USAC algorithm, default settings
pub const USAC_DEFAULT: i32 = 32;
/// USAC, fast settings
pub const USAC_FAST: i32 = 35;
/// USAC, fundamental matrix 8 points
pub const USAC_FM_8PTS: i32 = 34;
/// USAC, runs MAGSAC++
pub const USAC_MAGSAC: i32 = 38;
/// USAC, parallel version
pub const USAC_PARALLEL: i32 = 33;
/// USAC, sorted points, runs PROSAC
pub const USAC_PROSAC: i32 = 37;
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CirclesGridFinderParameters_GridType {
	SYMMETRIC_GRID = 0,
	ASYMMETRIC_GRID = 1,
}

opencv_type_enum! { crate::calib3d::CirclesGridFinderParameters_GridType }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HandEyeCalibrationMethod {
	/// A New Technique for Fully Autonomous and Efficient 3D Robotics Hand/Eye Calibration [Tsai89](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Tsai89)
	CALIB_HAND_EYE_TSAI = 0,
	/// Robot Sensor Calibration: Solving AX = XB on the Euclidean Group [Park94](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Park94)
	CALIB_HAND_EYE_PARK = 1,
	/// Hand-eye Calibration [Horaud95](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Horaud95)
	CALIB_HAND_EYE_HORAUD = 2,
	/// On-line Hand-Eye Calibration [Andreff99](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Andreff99)
	CALIB_HAND_EYE_ANDREFF = 3,
	/// Hand-Eye Calibration Using Dual Quaternions [Daniilidis98](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Daniilidis98)
	CALIB_HAND_EYE_DANIILIDIS = 4,
}

opencv_type_enum! { crate::calib3d::HandEyeCalibrationMethod }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LocalOptimMethod {
	LOCAL_OPTIM_NULL = 0,
	LOCAL_OPTIM_INNER_LO = 1,
	LOCAL_OPTIM_INNER_AND_ITER_LO = 2,
	LOCAL_OPTIM_GC = 3,
	LOCAL_OPTIM_SIGMA = 4,
}

opencv_type_enum! { crate::calib3d::LocalOptimMethod }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NeighborSearchMethod {
	NEIGH_FLANN_KNN = 0,
	NEIGH_GRID = 1,
	NEIGH_FLANN_RADIUS = 2,
}

opencv_type_enum! { crate::calib3d::NeighborSearchMethod }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PolishingMethod {
	NONE_POLISHER = 0,
	LSQ_POLISHER = 1,
	MAGSAC = 2,
	COV_POLISHER = 3,
}

opencv_type_enum! { crate::calib3d::PolishingMethod }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RobotWorldHandEyeCalibrationMethod {
	/// Solving the robot-world/hand-eye calibration problem using the kronecker product [Shah2013SolvingTR](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Shah2013SolvingTR)
	CALIB_ROBOT_WORLD_HAND_EYE_SHAH = 0,
	/// Simultaneous robot-world and hand-eye calibration using dual-quaternions and kronecker product [Li2010SimultaneousRA](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Li2010SimultaneousRA)
	CALIB_ROBOT_WORLD_HAND_EYE_LI = 1,
}

opencv_type_enum! { crate::calib3d::RobotWorldHandEyeCalibrationMethod }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SamplingMethod {
	SAMPLING_UNIFORM = 0,
	SAMPLING_PROGRESSIVE_NAPSAC = 1,
	SAMPLING_NAPSAC = 2,
	SAMPLING_PROSAC = 3,
}

opencv_type_enum! { crate::calib3d::SamplingMethod }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ScoreMethod {
	SCORE_METHOD_RANSAC = 0,
	SCORE_METHOD_MSAC = 1,
	SCORE_METHOD_MAGSAC = 2,
	SCORE_METHOD_LMEDS = 3,
}

opencv_type_enum! { crate::calib3d::ScoreMethod }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SolvePnPMethod {
	/// Pose refinement using non-linear Levenberg-Marquardt minimization scheme [Madsen04](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Madsen04) [Eade13](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Eade13) 
	/// 
	/// Initial solution for non-planar "objectPoints" needs at least 6 points and uses the DLT algorithm. 
	/// 
	/// Initial solution for planar "objectPoints" needs at least 4 points and uses pose from homography decomposition.
	SOLVEPNP_ITERATIVE = 0,
	/// EPnP: Efficient Perspective-n-Point Camera Pose Estimation [lepetit2009epnp](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_lepetit2009epnp)
	SOLVEPNP_EPNP = 1,
	/// Complete Solution Classification for the Perspective-Three-Point Problem [gao2003complete](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_gao2003complete)
	SOLVEPNP_P3P = 2,
	/// **Broken implementation. Using this flag will fallback to EPnP.** 
	/// 
	/// A Direct Least-Squares (DLS) Method for PnP [hesch2011direct](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_hesch2011direct)
	SOLVEPNP_DLS = 3,
	/// **Broken implementation. Using this flag will fallback to EPnP.** 
	/// 
	/// Exhaustive Linearization for Robust Camera Pose and Focal Length Estimation [penate2013exhaustive](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_penate2013exhaustive)
	SOLVEPNP_UPNP = 4,
	/// An Efficient Algebraic Solution to the Perspective-Three-Point Problem [Ke17](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Ke17)
	SOLVEPNP_AP3P = 5,
	/// Infinitesimal Plane-Based Pose Estimation [Collins14](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Collins14) 
	/// 
	/// Object points must be coplanar.
	SOLVEPNP_IPPE = 6,
	/// Infinitesimal Plane-Based Pose Estimation [Collins14](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Collins14) 
	/// 
	/// This is a special case suitable for marker pose estimation.
	/// 
	/// 4 coplanar object points must be defined in the following order:
	///   - point 0: [-squareLength / 2,  squareLength / 2, 0]
	///   - point 1: [ squareLength / 2,  squareLength / 2, 0]
	///   - point 2: [ squareLength / 2, -squareLength / 2, 0]
	///   - point 3: [-squareLength / 2, -squareLength / 2, 0]
	SOLVEPNP_IPPE_SQUARE = 7,
	/// SQPnP: A Consistently Fast and Globally OptimalSolution to the Perspective-n-Point Problem [Terzakis2020SQPnP](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Terzakis2020SQPnP)
	SOLVEPNP_SQPNP = 8,
	/// Used for count
	SOLVEPNP_MAX_COUNT = 9,
}

opencv_type_enum! { crate::calib3d::SolvePnPMethod }

/// cv::undistort mode
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UndistortTypes {
	PROJ_SPHERICAL_ORTHO = 0,
	PROJ_SPHERICAL_EQRECT = 1,
}

opencv_type_enum! { crate::calib3d::UndistortTypes }

pub type CirclesGridFinderParameters2 = crate::calib3d::CirclesGridFinderParameters;
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CirclesGridFinderParameters {
	pub density_neighborhood_size: core::Size2f,
	pub min_density: f32,
	pub kmeans_attempts: i32,
	pub min_distance_to_add_keypoint: i32,
	pub keypoint_scale: i32,
	pub min_graph_confidence: f32,
	pub vertex_gain: f32,
	pub vertex_penalty: f32,
	pub existing_vertex_gain: f32,
	pub edge_gain: f32,
	pub edge_penalty: f32,
	pub convex_hull_factor: f32,
	pub min_rng_edge_switch_dist: f32,
	pub grid_type: crate::calib3d::CirclesGridFinderParameters_GridType,
	/// Distance between two adjacent points. Used by CALIB_CB_CLUSTERING.
	pub square_size: f32,
	/// Max deviation from prediction. Used by CALIB_CB_CLUSTERING.
	pub max_rectified_distance: f32,
}

opencv_type_simple! { crate::calib3d::CirclesGridFinderParameters }

impl CirclesGridFinderParameters {
	#[inline]
	pub fn default() -> Result<crate::calib3d::CirclesGridFinderParameters> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CirclesGridFinderParameters_CirclesGridFinderParameters(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Constant methods for [crate::calib3d::LMSolver]
pub trait LMSolverTraitConst: core::AlgorithmTraitConst {
	fn as_raw_LMSolver(&self) -> *const c_void;

	/// Runs Levenberg-Marquardt algorithm using the passed vector of parameters as the start point.
	/// The final vector of parameters (whether the algorithm converged or not) is stored at the same
	/// vector. The method returns the number of iterations used. If it's equal to the previously specified
	/// maxIters, there is a big chance the algorithm did not converge.
	/// 
	/// ## Parameters
	/// * param: initial/final vector of parameters.
	/// 
	/// Note that the dimensionality of parameter space is defined by the size of param vector,
	/// and the dimensionality of optimized criteria is defined by the size of err vector
	/// computed by the callback.
	#[inline]
	fn run(&self, param: &mut impl core::ToInputOutputArray) -> Result<i32> {
		input_output_array_arg!(param);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LMSolver_run_const_const__InputOutputArrayR(self.as_raw_LMSolver(), param.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Retrieves the current maximum number of iterations
	#[inline]
	fn get_max_iters(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LMSolver_getMaxIters_const(self.as_raw_LMSolver(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Mutable methods for [crate::calib3d::LMSolver]
pub trait LMSolverTrait: core::AlgorithmTrait + crate::calib3d::LMSolverTraitConst {
	fn as_raw_mut_LMSolver(&mut self) -> *mut c_void;

	/// Sets the maximum number of iterations
	/// ## Parameters
	/// * maxIters: the number of iterations
	#[inline]
	fn set_max_iters(&mut self, max_iters: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LMSolver_setMaxIters_int(self.as_raw_mut_LMSolver(), max_iters, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Levenberg-Marquardt solver. Starting with the specified vector of parameters it
/// optimizes the target vector criteria "err"
/// (finds local minima of each target vector component absolute value).
/// 
/// When needed, it calls user-provided callback.
pub struct LMSolver {
	ptr: *mut c_void
}

opencv_type_boxed! { LMSolver }

impl Drop for LMSolver {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_LMSolver_delete(self.as_raw_mut_LMSolver()) };
	}
}

unsafe impl Send for LMSolver {}

impl core::AlgorithmTraitConst for LMSolver {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for LMSolver {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::calib3d::LMSolverTraitConst for LMSolver {
	#[inline] fn as_raw_LMSolver(&self) -> *const c_void { self.as_raw() }
}

impl crate::calib3d::LMSolverTrait for LMSolver {
	#[inline] fn as_raw_mut_LMSolver(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl LMSolver {
	/// Creates Levenberg-Marquard solver
	/// 
	/// ## Parameters
	/// * cb: callback
	/// * maxIters: maximum number of iterations that can be further
	///   modified using setMaxIters() method.
	#[inline]
	pub fn create(cb: &core::Ptr<crate::calib3d::LMSolver_Callback>, max_iters: i32) -> Result<core::Ptr<crate::calib3d::LMSolver>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LMSolver_create_const_PtrLCallbackGR_int(cb.as_raw_PtrOfLMSolver_Callback(), max_iters, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::calib3d::LMSolver>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn create_ext(cb: &core::Ptr<crate::calib3d::LMSolver_Callback>, max_iters: i32, eps: f64) -> Result<core::Ptr<crate::calib3d::LMSolver>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LMSolver_create_const_PtrLCallbackGR_int_double(cb.as_raw_PtrOfLMSolver_Callback(), max_iters, eps, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::calib3d::LMSolver>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { LMSolver, core::Algorithm, cv_LMSolver_to_Algorithm }

impl std::fmt::Debug for LMSolver {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("LMSolver")
			.finish()
	}
}

/// Constant methods for [crate::calib3d::LMSolver_Callback]
pub trait LMSolver_CallbackTraitConst {
	fn as_raw_LMSolver_Callback(&self) -> *const c_void;

	/// computes error and Jacobian for the specified vector of parameters
	/// 
	/// ## Parameters
	/// * param: the current vector of parameters
	/// * err: output vector of errors: err_i = actual_f_i - ideal_f_i
	/// * J: output Jacobian: J_ij = d(ideal_f_i)/d(param_j)
	/// 
	/// when J=noArray(), it means that it does not need to be computed.
	/// Dimensionality of error vector and param vector can be different.
	/// The callback should explicitly allocate (with "create" method) each output array
	/// (unless it's noArray()).
	#[inline]
	fn compute(&self, param: &impl core::ToInputArray, err: &mut impl core::ToOutputArray, j: &mut impl core::ToOutputArray) -> Result<bool> {
		input_array_arg!(param);
		output_array_arg!(err);
		output_array_arg!(j);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LMSolver_Callback_compute_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_LMSolver_Callback(), param.as_raw__InputArray(), err.as_raw__OutputArray(), j.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Mutable methods for [crate::calib3d::LMSolver_Callback]
pub trait LMSolver_CallbackTrait: crate::calib3d::LMSolver_CallbackTraitConst {
	fn as_raw_mut_LMSolver_Callback(&mut self) -> *mut c_void;

}

pub struct LMSolver_Callback {
	ptr: *mut c_void
}

opencv_type_boxed! { LMSolver_Callback }

impl Drop for LMSolver_Callback {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_LMSolver_Callback_delete(self.as_raw_mut_LMSolver_Callback()) };
	}
}

unsafe impl Send for LMSolver_Callback {}

impl crate::calib3d::LMSolver_CallbackTraitConst for LMSolver_Callback {
	#[inline] fn as_raw_LMSolver_Callback(&self) -> *const c_void { self.as_raw() }
}

impl crate::calib3d::LMSolver_CallbackTrait for LMSolver_Callback {
	#[inline] fn as_raw_mut_LMSolver_Callback(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl LMSolver_Callback {
}

impl std::fmt::Debug for LMSolver_Callback {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("LMSolver_Callback")
			.finish()
	}
}

/// Constant methods for [crate::calib3d::StereoBM]
pub trait StereoBMTraitConst: crate::calib3d::StereoMatcherTraitConst {
	fn as_raw_StereoBM(&self) -> *const c_void;

	/// Gets the type of pre-filtering currently used in the algorithm.
	/// ## Returns
	/// The current pre-filter type: 0 for PREFILTER_NORMALIZED_RESPONSE or 1 for PREFILTER_XSOBEL.
	#[inline]
	fn get_pre_filter_type(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_getPreFilterType_const(self.as_raw_StereoBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Gets the current size of the pre-filter kernel.
	/// ## Returns
	/// The current pre-filter size.
	#[inline]
	fn get_pre_filter_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_getPreFilterSize_const(self.as_raw_StereoBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Gets the current truncation value for prefiltered pixels.
	/// ## Returns
	/// The current pre-filter cap value.
	#[inline]
	fn get_pre_filter_cap(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_getPreFilterCap_const(self.as_raw_StereoBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Gets the current texture threshold value.
	/// ## Returns
	/// The current texture threshold.
	#[inline]
	fn get_texture_threshold(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_getTextureThreshold_const(self.as_raw_StereoBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Gets the current uniqueness ratio value.
	/// ## Returns
	/// The current uniqueness ratio.
	#[inline]
	fn get_uniqueness_ratio(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_getUniquenessRatio_const(self.as_raw_StereoBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Gets the current size of the smaller block used for texture check.
	/// ## Returns
	/// The current smaller block size.
	#[inline]
	fn get_smaller_block_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_getSmallerBlockSize_const(self.as_raw_StereoBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Gets the current Region of Interest (ROI) for the left image.
	/// ## Returns
	/// The current ROI for the left image.
	#[inline]
	fn get_roi1(&self) -> Result<core::Rect> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_getROI1_const(self.as_raw_StereoBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Gets the current Region of Interest (ROI) for the right image.
	/// ## Returns
	/// The current ROI for the right image.
	#[inline]
	fn get_roi2(&self) -> Result<core::Rect> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_getROI2_const(self.as_raw_StereoBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Mutable methods for [crate::calib3d::StereoBM]
pub trait StereoBMTrait: crate::calib3d::StereoBMTraitConst + crate::calib3d::StereoMatcherTrait {
	fn as_raw_mut_StereoBM(&mut self) -> *mut c_void;

	/// Sets the type of pre-filtering used in the algorithm.
	/// ## Parameters
	/// * preFilterType: The type of pre-filter to use. Possible values are:
	/// - PREFILTER_NORMALIZED_RESPONSE (0): Uses normalized response for pre-filtering.
	/// - PREFILTER_XSOBEL (1): Uses the X-Sobel operator for pre-filtering.
	/// @details The pre-filter type affects how the images are prepared before computing the disparity map. Different pre-filtering methods can enhance specific image features or reduce noise, influencing the quality of the disparity map.
	#[inline]
	fn set_pre_filter_type(&mut self, pre_filter_type: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_setPreFilterType_int(self.as_raw_mut_StereoBM(), pre_filter_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Sets the size of the pre-filter kernel.
	/// ## Parameters
	/// * preFilterSize: The size of the pre-filter kernel. Must be an odd integer, typically between 5 and 255.
	/// @details The pre-filter size determines the spatial extent of the pre-filtering operation, which prepares the images for disparity computation by normalizing brightness and enhancing texture. Larger sizes reduce noise but may blur details, while smaller sizes preserve details but are more susceptible to noise.
	#[inline]
	fn set_pre_filter_size(&mut self, pre_filter_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_setPreFilterSize_int(self.as_raw_mut_StereoBM(), pre_filter_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Sets the truncation value for prefiltered pixels.
	/// ## Parameters
	/// * preFilterCap: The truncation value. Typically in the range [1, 63].
	/// @details This value caps the output of the pre-filter to [-preFilterCap, preFilterCap], helping to reduce the impact of noise and outliers in the pre-filtered image.
	#[inline]
	fn set_pre_filter_cap(&mut self, pre_filter_cap: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_setPreFilterCap_int(self.as_raw_mut_StereoBM(), pre_filter_cap, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Sets the threshold for filtering low-texture regions.
	/// ## Parameters
	/// * textureThreshold: The threshold value. Must be non-negative.
	/// @details This parameter filters out regions with low texture, where establishing correspondences is difficult, thus reducing noise in the disparity map. Higher values filter more aggressively but may discard valid information.
	#[inline]
	fn set_texture_threshold(&mut self, texture_threshold: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_setTextureThreshold_int(self.as_raw_mut_StereoBM(), texture_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Sets the uniqueness ratio for filtering ambiguous matches.
	/// ## Parameters
	/// * uniquenessRatio: The uniqueness ratio value. Typically in the range [5, 15], but can be from 0 to 100.
	/// @details This parameter ensures that the best match is sufficiently better than the next best match, reducing false positives. Higher values are stricter but may filter out valid matches in difficult regions.
	#[inline]
	fn set_uniqueness_ratio(&mut self, uniqueness_ratio: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_setUniquenessRatio_int(self.as_raw_mut_StereoBM(), uniqueness_ratio, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Sets the size of the smaller block used for texture check.
	/// ## Parameters
	/// * blockSize: The size of the smaller block. Must be an odd integer between 5 and 255.
	/// @details This parameter determines the size of the block used to compute texture variance. Smaller blocks capture finer details but are more sensitive to noise, while larger blocks are more robust but may miss fine details.
	#[inline]
	fn set_smaller_block_size(&mut self, block_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_setSmallerBlockSize_int(self.as_raw_mut_StereoBM(), block_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Sets the Region of Interest (ROI) for the left image.
	/// ## Parameters
	/// * roi1: The ROI rectangle for the left image.
	/// @details By setting the ROI, the stereo matching computation is limited to the specified region, improving performance and potentially accuracy by focusing on relevant parts of the image.
	#[inline]
	fn set_roi1(&mut self, roi1: core::Rect) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_setROI1_Rect(self.as_raw_mut_StereoBM(), roi1.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Sets the Region of Interest (ROI) for the right image.
	/// ## Parameters
	/// * roi2: The ROI rectangle for the right image.
	/// @details Similar to setROI1, this limits the computation to the specified region in the right image.
	#[inline]
	fn set_roi2(&mut self, roi2: core::Rect) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_setROI2_Rect(self.as_raw_mut_StereoBM(), roi2.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Class for computing stereo correspondence using the block matching algorithm, introduced and contributed to OpenCV by K. Konolige.
/// @details This class implements a block matching algorithm for stereo correspondence, which is used to compute disparity maps from stereo image pairs. It provides methods to fine-tune parameters such as pre-filtering, texture thresholds, uniqueness ratios, and regions of interest (ROIs) to optimize performance and accuracy.
pub struct StereoBM {
	ptr: *mut c_void
}

opencv_type_boxed! { StereoBM }

impl Drop for StereoBM {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_StereoBM_delete(self.as_raw_mut_StereoBM()) };
	}
}

unsafe impl Send for StereoBM {}

impl core::AlgorithmTraitConst for StereoBM {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for StereoBM {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::calib3d::StereoMatcherTraitConst for StereoBM {
	#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::calib3d::StereoMatcherTrait for StereoBM {
	#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::calib3d::StereoBMTraitConst for StereoBM {
	#[inline] fn as_raw_StereoBM(&self) -> *const c_void { self.as_raw() }
}

impl crate::calib3d::StereoBMTrait for StereoBM {
	#[inline] fn as_raw_mut_StereoBM(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl StereoBM {
	/// Creates StereoBM object
	/// ## Parameters
	/// * numDisparities: The disparity search range. For each pixel, the algorithm will find the best disparity from 0 (default minimum disparity) to numDisparities. The search range can be shifted by changing the minimum disparity.
	/// * blockSize: The linear size of the blocks compared by the algorithm. The size should be odd (as the block is centered at the current pixel). Larger block size implies smoother, though less accurate disparity map. Smaller block size gives more detailed disparity map, but there is a higher chance for the algorithm to find a wrong correspondence.
	/// ## Returns
	/// A pointer to the created StereoBM object.
	/// @details The function creates a StereoBM object. You can then call StereoBM::compute() to compute disparity for a specific stereo pair.
	/// 
	/// ## C++ default parameters
	/// * num_disparities: 0
	/// * block_size: 21
	#[inline]
	pub fn create(num_disparities: i32, block_size: i32) -> Result<core::Ptr<crate::calib3d::StereoBM>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_create_int_int(num_disparities, block_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::calib3d::StereoBM>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates StereoBM object
	/// ## Parameters
	/// * numDisparities: The disparity search range. For each pixel, the algorithm will find the best disparity from 0 (default minimum disparity) to numDisparities. The search range can be shifted by changing the minimum disparity.
	/// * blockSize: The linear size of the blocks compared by the algorithm. The size should be odd (as the block is centered at the current pixel). Larger block size implies smoother, though less accurate disparity map. Smaller block size gives more detailed disparity map, but there is a higher chance for the algorithm to find a wrong correspondence.
	/// ## Returns
	/// A pointer to the created StereoBM object.
	/// @details The function creates a StereoBM object. You can then call StereoBM::compute() to compute disparity for a specific stereo pair.
	/// 
	/// ## Note
	/// This alternative version of [StereoBM::create] function uses the following default values for its arguments:
	/// * num_disparities: 0
	/// * block_size: 21
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::calib3d::StereoBM>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoBM_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::calib3d::StereoBM>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { StereoBM, core::Algorithm, cv_StereoBM_to_Algorithm }

boxed_cast_base! { StereoBM, crate::calib3d::StereoMatcher, cv_StereoBM_to_StereoMatcher }

impl std::fmt::Debug for StereoBM {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("StereoBM")
			.finish()
	}
}

/// Constant methods for [crate::calib3d::StereoMatcher]
pub trait StereoMatcherTraitConst: core::AlgorithmTraitConst {
	fn as_raw_StereoMatcher(&self) -> *const c_void;

	#[inline]
	fn get_min_disparity(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_getMinDisparity_const(self.as_raw_StereoMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_num_disparities(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_getNumDisparities_const(self.as_raw_StereoMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_block_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_getBlockSize_const(self.as_raw_StereoMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_speckle_window_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_getSpeckleWindowSize_const(self.as_raw_StereoMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_speckle_range(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_getSpeckleRange_const(self.as_raw_StereoMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_disp12_max_diff(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_getDisp12MaxDiff_const(self.as_raw_StereoMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Mutable methods for [crate::calib3d::StereoMatcher]
pub trait StereoMatcherTrait: core::AlgorithmTrait + crate::calib3d::StereoMatcherTraitConst {
	fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void;

	/// Computes disparity map for the specified stereo pair
	/// 
	/// ## Parameters
	/// * left: Left 8-bit single-channel image.
	/// * right: Right image of the same size and the same type as the left one.
	/// * disparity: Output disparity map. It has the same size as the input images. Some algorithms,
	/// like StereoBM or StereoSGBM compute 16-bit fixed-point disparity map (where each disparity value
	/// has 4 fractional bits), whereas other algorithms output 32-bit floating-point disparity map.
	#[inline]
	fn compute(&mut self, left: &impl core::ToInputArray, right: &impl core::ToInputArray, disparity: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(left);
		input_array_arg!(right);
		output_array_arg!(disparity);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_StereoMatcher(), left.as_raw__InputArray(), right.as_raw__InputArray(), disparity.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_min_disparity(&mut self, min_disparity: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_setMinDisparity_int(self.as_raw_mut_StereoMatcher(), min_disparity, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_num_disparities(&mut self, num_disparities: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_setNumDisparities_int(self.as_raw_mut_StereoMatcher(), num_disparities, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_block_size(&mut self, block_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_setBlockSize_int(self.as_raw_mut_StereoMatcher(), block_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_speckle_window_size(&mut self, speckle_window_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_setSpeckleWindowSize_int(self.as_raw_mut_StereoMatcher(), speckle_window_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_speckle_range(&mut self, speckle_range: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_setSpeckleRange_int(self.as_raw_mut_StereoMatcher(), speckle_range, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_disp12_max_diff(&mut self, disp12_max_diff: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoMatcher_setDisp12MaxDiff_int(self.as_raw_mut_StereoMatcher(), disp12_max_diff, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// The base class for stereo correspondence algorithms.
pub struct StereoMatcher {
	ptr: *mut c_void
}

opencv_type_boxed! { StereoMatcher }

impl Drop for StereoMatcher {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_StereoMatcher_delete(self.as_raw_mut_StereoMatcher()) };
	}
}

unsafe impl Send for StereoMatcher {}

impl core::AlgorithmTraitConst for StereoMatcher {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for StereoMatcher {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::calib3d::StereoMatcherTraitConst for StereoMatcher {
	#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::calib3d::StereoMatcherTrait for StereoMatcher {
	#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl StereoMatcher {
}

boxed_cast_descendant! { StereoMatcher, crate::calib3d::StereoBM, cv_StereoMatcher_to_StereoBM }

boxed_cast_descendant! { StereoMatcher, crate::calib3d::StereoSGBM, cv_StereoMatcher_to_StereoSGBM }

boxed_cast_base! { StereoMatcher, core::Algorithm, cv_StereoMatcher_to_Algorithm }

impl std::fmt::Debug for StereoMatcher {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("StereoMatcher")
			.finish()
	}
}

/// Constant methods for [crate::calib3d::StereoSGBM]
pub trait StereoSGBMTraitConst: crate::calib3d::StereoMatcherTraitConst {
	fn as_raw_StereoSGBM(&self) -> *const c_void;

	#[inline]
	fn get_pre_filter_cap(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_getPreFilterCap_const(self.as_raw_StereoSGBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_uniqueness_ratio(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_getUniquenessRatio_const(self.as_raw_StereoSGBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_p1(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_getP1_const(self.as_raw_StereoSGBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_p2(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_getP2_const(self.as_raw_StereoSGBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_mode(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_getMode_const(self.as_raw_StereoSGBM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Mutable methods for [crate::calib3d::StereoSGBM]
pub trait StereoSGBMTrait: crate::calib3d::StereoMatcherTrait + crate::calib3d::StereoSGBMTraitConst {
	fn as_raw_mut_StereoSGBM(&mut self) -> *mut c_void;

	#[inline]
	fn set_pre_filter_cap(&mut self, pre_filter_cap: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_setPreFilterCap_int(self.as_raw_mut_StereoSGBM(), pre_filter_cap, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_uniqueness_ratio(&mut self, uniqueness_ratio: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_setUniquenessRatio_int(self.as_raw_mut_StereoSGBM(), uniqueness_ratio, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_p1(&mut self, p1: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_setP1_int(self.as_raw_mut_StereoSGBM(), p1, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_p2(&mut self, p2: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_setP2_int(self.as_raw_mut_StereoSGBM(), p2, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_mode(&mut self, mode: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_setMode_int(self.as_raw_mut_StereoSGBM(), mode, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// The class implements the modified H. Hirschmuller algorithm [HH08](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_HH08) that differs from the original
/// one as follows:
/// 
/// *   By default, the algorithm is single-pass, which means that you consider only 5 directions
/// instead of 8. Set mode=StereoSGBM::MODE_HH in createStereoSGBM to run the full variant of the
/// algorithm but beware that it may consume a lot of memory.
/// *   The algorithm matches blocks, not individual pixels. Though, setting blockSize=1 reduces the
/// blocks to single pixels.
/// *   Mutual information cost function is not implemented. Instead, a simpler Birchfield-Tomasi
/// sub-pixel metric from [BT98](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_BT98) is used. Though, the color images are supported as well.
/// *   Some pre- and post- processing steps from K. Konolige algorithm StereoBM are included, for
/// example: pre-filtering (StereoBM::PREFILTER_XSOBEL type) and post-filtering (uniqueness
/// check, quadratic interpolation and speckle filtering).
/// 
/// 
/// Note:
///    *   (Python) An example illustrating the use of the StereoSGBM matching algorithm can be found
///        at opencv_source_code/samples/python/stereo_match.py
pub struct StereoSGBM {
	ptr: *mut c_void
}

opencv_type_boxed! { StereoSGBM }

impl Drop for StereoSGBM {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_StereoSGBM_delete(self.as_raw_mut_StereoSGBM()) };
	}
}

unsafe impl Send for StereoSGBM {}

impl core::AlgorithmTraitConst for StereoSGBM {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for StereoSGBM {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::calib3d::StereoMatcherTraitConst for StereoSGBM {
	#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::calib3d::StereoMatcherTrait for StereoSGBM {
	#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::calib3d::StereoSGBMTraitConst for StereoSGBM {
	#[inline] fn as_raw_StereoSGBM(&self) -> *const c_void { self.as_raw() }
}

impl crate::calib3d::StereoSGBMTrait for StereoSGBM {
	#[inline] fn as_raw_mut_StereoSGBM(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl StereoSGBM {
	/// Creates StereoSGBM object
	/// 
	/// ## Parameters
	/// * minDisparity: Minimum possible disparity value. Normally, it is zero but sometimes
	/// rectification algorithms can shift images, so this parameter needs to be adjusted accordingly.
	/// * numDisparities: Maximum disparity minus minimum disparity. The value is always greater than
	/// zero. In the current implementation, this parameter must be divisible by 16.
	/// * blockSize: Matched block size. It must be an odd number \>=1 . Normally, it should be
	/// somewhere in the 3..11 range.
	/// * P1: The first parameter controlling the disparity smoothness. See below.
	/// * P2: The second parameter controlling the disparity smoothness. The larger the values are,
	/// the smoother the disparity is. P1 is the penalty on the disparity change by plus or minus 1
	/// between neighbor pixels. P2 is the penalty on the disparity change by more than 1 between neighbor
	/// pixels. The algorithm requires P2 \> P1 . See stereo_match.cpp sample where some reasonably good
	/// P1 and P2 values are shown (like 8\*number_of_image_channels\*blockSize\*blockSize and
	/// 32\*number_of_image_channels\*blockSize\*blockSize , respectively).
	/// * disp12MaxDiff: Maximum allowed difference (in integer pixel units) in the left-right
	/// disparity check. Set it to a non-positive value to disable the check.
	/// * preFilterCap: Truncation value for the prefiltered image pixels. The algorithm first
	/// computes x-derivative at each pixel and clips its value by [-preFilterCap, preFilterCap] interval.
	/// The result values are passed to the Birchfield-Tomasi pixel cost function.
	/// * uniquenessRatio: Margin in percentage by which the best (minimum) computed cost function
	/// value should "win" the second best value to consider the found match correct. Normally, a value
	/// within the 5-15 range is good enough.
	/// * speckleWindowSize: Maximum size of smooth disparity regions to consider their noise speckles
	/// and invalidate. Set it to 0 to disable speckle filtering. Otherwise, set it somewhere in the
	/// 50-200 range.
	/// * speckleRange: Maximum disparity variation within each connected component. If you do speckle
	/// filtering, set the parameter to a positive value, it will be implicitly multiplied by 16.
	/// Normally, 1 or 2 is good enough.
	/// * mode: Set it to StereoSGBM::MODE_HH to run the full-scale two-pass dynamic programming
	/// algorithm. It will consume O(W\*H\*numDisparities) bytes, which is large for 640x480 stereo and
	/// huge for HD-size pictures. By default, it is set to false .
	/// 
	/// The first constructor initializes StereoSGBM with all the default parameters. So, you only have to
	/// set StereoSGBM::numDisparities at minimum. The second constructor enables you to set each parameter
	/// to a custom value.
	/// 
	/// ## C++ default parameters
	/// * min_disparity: 0
	/// * num_disparities: 16
	/// * block_size: 3
	/// * p1: 0
	/// * p2: 0
	/// * disp12_max_diff: 0
	/// * pre_filter_cap: 0
	/// * uniqueness_ratio: 0
	/// * speckle_window_size: 0
	/// * speckle_range: 0
	/// * mode: StereoSGBM::MODE_SGBM
	#[inline]
	pub fn create(min_disparity: i32, num_disparities: i32, block_size: i32, p1: i32, p2: i32, disp12_max_diff: i32, pre_filter_cap: i32, uniqueness_ratio: i32, speckle_window_size: i32, speckle_range: i32, mode: i32) -> Result<core::Ptr<crate::calib3d::StereoSGBM>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_create_int_int_int_int_int_int_int_int_int_int_int(min_disparity, num_disparities, block_size, p1, p2, disp12_max_diff, pre_filter_cap, uniqueness_ratio, speckle_window_size, speckle_range, mode, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::calib3d::StereoSGBM>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates StereoSGBM object
	/// 
	/// ## Parameters
	/// * minDisparity: Minimum possible disparity value. Normally, it is zero but sometimes
	/// rectification algorithms can shift images, so this parameter needs to be adjusted accordingly.
	/// * numDisparities: Maximum disparity minus minimum disparity. The value is always greater than
	/// zero. In the current implementation, this parameter must be divisible by 16.
	/// * blockSize: Matched block size. It must be an odd number \>=1 . Normally, it should be
	/// somewhere in the 3..11 range.
	/// * P1: The first parameter controlling the disparity smoothness. See below.
	/// * P2: The second parameter controlling the disparity smoothness. The larger the values are,
	/// the smoother the disparity is. P1 is the penalty on the disparity change by plus or minus 1
	/// between neighbor pixels. P2 is the penalty on the disparity change by more than 1 between neighbor
	/// pixels. The algorithm requires P2 \> P1 . See stereo_match.cpp sample where some reasonably good
	/// P1 and P2 values are shown (like 8\*number_of_image_channels\*blockSize\*blockSize and
	/// 32\*number_of_image_channels\*blockSize\*blockSize , respectively).
	/// * disp12MaxDiff: Maximum allowed difference (in integer pixel units) in the left-right
	/// disparity check. Set it to a non-positive value to disable the check.
	/// * preFilterCap: Truncation value for the prefiltered image pixels. The algorithm first
	/// computes x-derivative at each pixel and clips its value by [-preFilterCap, preFilterCap] interval.
	/// The result values are passed to the Birchfield-Tomasi pixel cost function.
	/// * uniquenessRatio: Margin in percentage by which the best (minimum) computed cost function
	/// value should "win" the second best value to consider the found match correct. Normally, a value
	/// within the 5-15 range is good enough.
	/// * speckleWindowSize: Maximum size of smooth disparity regions to consider their noise speckles
	/// and invalidate. Set it to 0 to disable speckle filtering. Otherwise, set it somewhere in the
	/// 50-200 range.
	/// * speckleRange: Maximum disparity variation within each connected component. If you do speckle
	/// filtering, set the parameter to a positive value, it will be implicitly multiplied by 16.
	/// Normally, 1 or 2 is good enough.
	/// * mode: Set it to StereoSGBM::MODE_HH to run the full-scale two-pass dynamic programming
	/// algorithm. It will consume O(W\*H\*numDisparities) bytes, which is large for 640x480 stereo and
	/// huge for HD-size pictures. By default, it is set to false .
	/// 
	/// The first constructor initializes StereoSGBM with all the default parameters. So, you only have to
	/// set StereoSGBM::numDisparities at minimum. The second constructor enables you to set each parameter
	/// to a custom value.
	/// 
	/// ## Note
	/// This alternative version of [StereoSGBM::create] function uses the following default values for its arguments:
	/// * min_disparity: 0
	/// * num_disparities: 16
	/// * block_size: 3
	/// * p1: 0
	/// * p2: 0
	/// * disp12_max_diff: 0
	/// * pre_filter_cap: 0
	/// * uniqueness_ratio: 0
	/// * speckle_window_size: 0
	/// * speckle_range: 0
	/// * mode: StereoSGBM::MODE_SGBM
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::calib3d::StereoSGBM>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereoSGBM_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::calib3d::StereoSGBM>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { StereoSGBM, core::Algorithm, cv_StereoSGBM_to_Algorithm }

boxed_cast_base! { StereoSGBM, crate::calib3d::StereoMatcher, cv_StereoSGBM_to_StereoMatcher }

impl std::fmt::Debug for StereoSGBM {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("StereoSGBM")
			.finish()
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UsacParams {
	pub confidence: f64,
	pub is_parallel: bool,
	pub lo_iterations: i32,
	pub lo_method: crate::calib3d::LocalOptimMethod,
	pub lo_sample_size: i32,
	pub max_iterations: i32,
	pub neighbors_search: crate::calib3d::NeighborSearchMethod,
	pub random_generator_state: i32,
	pub sampler: crate::calib3d::SamplingMethod,
	pub score: crate::calib3d::ScoreMethod,
	pub threshold: f64,
	pub final_polisher: crate::calib3d::PolishingMethod,
	pub final_polisher_iterations: i32,
}

opencv_type_simple! { crate::calib3d::UsacParams }

impl UsacParams {
	#[inline]
	pub fn default() -> Result<crate::calib3d::UsacParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_UsacParams_UsacParams(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}
