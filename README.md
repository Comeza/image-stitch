# image-stitch

This is a simple program to stitch (hundrets to thousand) images together.


## Usage
The program will use the current working dir as source folder.
```
image-stitch
```
This will generate an `output/output.png` file

## Arguments
| Argument    | Alias | Optional | Default             | Description                                    |
|-------------|-------|----------|---------------------|------------------------------------------------|
| `input`     | `i`   | Yes      | ./                  | The source folder for the images.              |
| `output`    | `o`   | Yes      | ./output/output.png | The output file.                               |
| `max`       | `m`   | Yes      |                     | Maximum dimension in the defined direction     |
| `direction` | `d`   | Yes      | X                   | The direction the program processes the images |

*Right now buggy


## Todo
#### General
- [x] Image stitching

#### Program args
- [x] Set output file
- [x] Set source dir
- [x] Print help
- [ ] Print supported file formats

#### File Formats
- [ ] Gif