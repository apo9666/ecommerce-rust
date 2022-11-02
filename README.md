## Windows gnu
Configure rust
```
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

Download MSYS2 https://www.msys2.org/

Install mingw
```shell
pacman -Syu
pacman -S --needed base-devel mingw-w64-x86_64-toolchain
```

Add PATH system variable
```
C:\msys64\mingw64\bin
```

Edit cargo config file

```shell
code %UserProfile%\.cargo\config
```

```
[target.x86_64-pc-windows-gnu]
linker = "C:\\msys64\\mingw64\\bin\\gcc.exe"
ar = "C:\\msys64\\mingw64\\bin\\ar.exe"
```
