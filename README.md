# Behold, the Documentinator!

Documentinator (aka docTor or doctor) is a Ros2 documentation generator. It takes well-defined and enforced
config files and produces common node documentation in order to ease the burden of managing documentation style and copying.

To install doctor, first [install rust](https://www.rust-lang.org/learn/get-started), then run: ``cargo install documentinator``. 
Doctor should now be on your path. Just run this command again to update when possible.

TODO:

Doctor is currently usable for documentation generation, but nothing else. It also cannot be invoked as doctor by default yet.

- [x] Create init
- [x] Create Generators
- [ ] Alias to doctor
- [ ] Improve CI experience
- [ ] Tidy up and document

docTor has 3 commands:

`doctor init` - creates sample TOML

`doctor gen` - generates documents

### Generating

Doctor generates files by searching in a passed directory for config files. If -r is passed, it will also recurse.

Each config will generate doc for each node. `--readme` can be used to generate a readme.md that links to all the 
generated node docs. This is useful for generating readmes for GitHub repos in a CI action.

### Configuring

Docs will be in nodename.doctor.toml files. Each toml can have n nodes, or each node can have its own file.

### Example Toml file:
```toml
package_name = "white line detection"
repo = "https://github.com/iscumd/white_line_detection"

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

### TOML Format
Config files have the following valid structure.

> Note: tags marked 'Optional' can be omitted in the toml.

- `package_name`: String
- `repo`: String
- `nodes`: Array of
  - `node_name`: String
  - `source_file`: Array of strings
  - `summary`: String
  - `potential_improvements`: Optional string
  - `misc`: Optional string
  - `publishes`: Optional array of
    - `name`: String
    - `description`: String
  - `subscribes`: Optional array of 
    - `name`: String
    - `description`: String
  - `params`: Optional array of
    - `name`: String
    - `description`: String
  - `launch`: Optional array of
    - `file_path`: String
    - `usage`: String
    - `args`: Optional array of 
      - `name`: String
      - `description`: String
      

### Usage examples

Create example config for node called 'node_name':

``` documentinator init node_name```

Generate markdown files for all configs in this directory:

```documentinator gen .```

Generate markdown files for all configs in this and all subdirectories:

```documentinator gen -r .```

Generate a markdown readme that links to all other generated node docs (and also generates them):

```documentinator gen --readme .```