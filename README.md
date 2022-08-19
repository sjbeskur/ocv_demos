# OpenCV Rust Examples
A series of Rust examples on using OpenCV.  These are almost all direct ports of other Python or C++ examples on the internet.

## Prerequisits

``` bash
sudo apt install libclang-dev
sudo apt install clang
sudo apt install libcanberra-gtk-module
```

Of course you will need to install OpenCV

``` bash
git clone https://github.com/opencv/opencv.git
cd opencv/build
cmake ../ && make install
```

Along with the above prerequisits you will need to set the folling env vars in order to compile your rust project

``` bash
# location of your opencv cmake file ( e.g. find /usr -name "OpenCV*cmake")
PKG_CONFIG_PATH=/usr/local/lib/cmake/opencv4/OpenCVConfig.cmake

# location of opencv *.so files
LD_LIBRARY_PATH=/usr/local/lib/
```

## Build project
```
cargo build 
```

## Examples
### Image Filtering Example
This is more an example of Rusts powerful lambda capabilites used to select pixels above a given threshold in an image.

```
cargo run -- filter -f <image_file> -t <threshold_u8>
```



### Finding Centroid / Blobs Example
This example is a Rust port of the following example on [learnopencv.com](https://learnopencv.com/find-center-of-blob-centroid-using-opencv-cpp-python/)

```
cargo run -- centroid -f <image_file>
```




## Notes on Cross Compiling 
e.g. aarch64-unknown-linux-gnu 
### prerequisits
```
sudo apt-get install g++-aarch64-linux-gnu
```

### build (for aarch64)
```
rustup target list
rustup target add aarch64-unknown-linux-gnu 
cargo build --target aarch64-unknown-linux-gnu
```

https://docs.opencv.org/4.x/d0/d76/tutorial_arm_crosscompile_with_cmake.html