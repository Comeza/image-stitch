# image-stitch

This is a simple program to stitch (hundrets to thousand) images together.


## Usage
```
image-stitch -o ./output.png -i ./images -m 16000 -d X
```

## Arguments
| Argument    | Alias | Optional | Default             | Description                                    |
|-------------|-------|----------|---------------------|------------------------------------------------|
| `input`     | `i`   | Yes      | ./                  | The source folder for the images.              |
| `output`    | `o`   | Yes      | ./output.png        | The output file.                               |
| `max`       | `m`   | Yes      | 0                   | Maximum dimension in the defined direction     |
| `direction` | `d`   | Yes      | X                   | The direction the program processes the images |



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