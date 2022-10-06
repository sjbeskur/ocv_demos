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

### Cross-Compile / Build (for aarch64)

To cross-compile for aarch64 you will need to edit the following:
~/.cargo/config

```bash
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"

```


```bash
rustup target list
rustup target add aarch64-unknown-linux-gnu 
cargo build --target aarch64-unknown-linux-gnu

```

### Cross-Compile / OpenCV (for aarch64)

To cross-compile OpenCV for aarch64 you can do the following from the OpenCV source folder:

```bash
cmake -B build -S. \ 
    -DCMAKE_BUILD_TYPE=RELEASE \
    -DCMAKE_INSTALL_PREFIX=/usr/local \ -DOPENCV_GENERATE_PKGCONFIG=ON  \
    -DCMAKE_TOOLCHAIN_FILE=../platforms/linux/aarch64-gnu.toolchain.cmake .. \
    && cd build \
    && make -j16 
```

### Armv7 / armhf 

sudo dpkg --add-architecture armhf

~.cargo/config
```
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
```

```
rustup target add armv7-unknown-linux-gnueabihf
cargo build --target armv7-unknown-linux-gnueabihf

```






https://docs.opencv.org/4.x/d0/d76/tutorial_arm_crosscompile_with_cmake.html

https://jensd.be/1126/linux/cross-compiling-for-arm-or-aarch64-on-debian-or-ubuntu

