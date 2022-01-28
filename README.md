# development overview

docTor has 3 commands:

`doctor init` - creates sample TOML

`doctor gen` - generates documents

`doctor validate` - runs  linter on code to validate docs are correct


Docs will be in nodename.doctor.toml files. Each toml can have n nodes, or each node can have its own file.

Doctor is technically agnostic to document type for both input and output, although Im focusing on TOML and md for now.

### Example Toml file:
```toml
[[nodes]]
node_name = 'White_line_detection'
source_file = ['/src/white_line_detection.cpp']
summary = '''
This node allows Ohm to avoid the bounding white lines by consuming images published on the image_raw topic and publishing found white lines as point clouds.
This is currently achived using OpenCv, and a small algortithm. The node operates in a pull fashion, only computing when an image is published.
When an image is published, the node converts the image to an OpenCv Umat, then perspective shifting the image, then filtering out 'non-white' pixels,
and finally determining where the remaining pixels are in relation to the robot, and publishing that point as a pointcloud/PC2 to both 'camera_cloud' and 'camera_cloud2'.
Ohm will then see these points as a wall, preventing it from crossing.
'''
potential_improvements = 'Burn with fire'
misc = 'An (optional) description of something else.'

[[nodes.publishes]]
name = 'camera_cloud'
description = 'Ohm spesific pointcloud topic.'

[[nodes.publishes]]
name = 'camera_cloud2'
description = 'Ohm spesific pointcloud2d topic.'

[[nodes.subscribes]]
name = 'image_raw'
description = 'The raw image (not-debayerd) from the camera node/gazebo.'

[[nodes.subscribes]]
name = 'camera_info'
description = 'The standard camera info topic, used to find things like resolution.'

[[nodes.params]]
name = 'kernel_size'
description = 'Size of the erosion kernel. Default 5'

[nodes.launch]
file_path = '/launch/WLD.launch.py'
usage = 'just launch I guess'

[[nodes.launch.args]]
name = 'arg1'
description = 'Some arg to the launchfile'

[[nodes.launch.args]]
name = 'arg2'
description = 'Another arg to the launchfile'

```