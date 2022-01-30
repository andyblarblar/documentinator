# White_line_detection
# File
`/src/white_line_detection.cpp`

## Summary
This node allows Ohm to avoid the bounding white lines by consuming images published on the image_raw topic and publishing found white lines as point clouds.
This is currently achived using OpenCv, and a small algortithm. The node operates in a pull fashion, only computing when an image is published.
When an image is published, the node converts the image to an OpenCv Umat, then perspective shifting the image, then filtering out 'non-white' pixels,
and finally determining where the remaining pixels are in relation to the robot, and publishing that point as a pointcloud/PC2 to both 'camera_cloud' and 'camera_cloud2'.
Ohm will then see these points as a wall, preventing it from crossing.

## Topics

### Publishes
- camera_cloud: Ohm spesific pointcloud topic.
- camera_cloud2: Ohm spesific pointcloud2d topic.

### Subscribes
- image_raw: The raw image (not-debayerd) from the camera node/gazebo.
- camera_info: The standard camera info topic, used to find things like resolution.

## Params
- kernel_size: Size of the erosion kernel. Default 5

## Potential Improvements
Burn with fire

# Launch
`/launch/WLD.launch.py`
just launch I guess

## Args
- arg1: Some arg to the launchfile
- arg2: Another arg to the launchfile

# Misc
An (optional) description of something else. 
