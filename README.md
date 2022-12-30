# image-convert.rs

Simple batch image convert binary written in Rust.

## Functionality

- Convert all png images in one directory to jpg, and output in another directory
- Downsample the images

## Limitation

- Currently only support png to jpg
- Only support square images

## Get Started

Use this command to view usage

```bash
jpg-convert -h
```

Avaliable parameters:

- `-i`: specify input directory
- `-o`: specify output directory
- `-s`: set the scale after downsampling
