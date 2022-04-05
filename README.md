# image-stitch

This is a simple program to stitch images together.


## Usage
```
image-stitch -o ./output.png -i ./images -m 16000 -d X
```

## Arguments
| Argument    | Alias | Values                                | Default      | Optional | Description                                    |
|-------------|-------|---------------------------------------|--------------|----------|------------------------------------------------|
| `input`     | `i`   | Path                                  | ./           | Yes      | The source folder for the images.              |
| `output`    | `o`   | Path                                  | ./output.png | Yes      | The output file.                               |
| `max`       | `m`   | Pixels                                | 0            | Yes      | Maximum dimension in the defined direction     |
| `direction` | `d`   | `X`: X-Direction<br>`Y`: Y-Direction  | X            | Yes      | The direction the program processes the images |



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
