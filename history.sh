~ history
    1  ls
    2  mkdir -p ~/local/qemu-10.2.1\n
    3  ./configure --prefix=$HOME/local/qemu-10.2.1 \\n            --target-list=x86_64-softmmu,arm-softmmu,aarch64-softmmu \\n            --disable-werror
    4  pip3 install --user tomli
    5  ./configure --prefix=$HOME/local/qemu-10.2.1 \\n            --target-list=x86_64-softmmu,arm-softmmu,aarch64-softmmu \\n            --disable-werror
    6  mkdir -p ~/local/ninja\ncd ~/local/ninja
    7  wget https://github.com/ninja-build/ninja/releases/download/v1.12.1/ninja-linux.zip\n
    8  unzip ninja-linux.zip\n
    9  chmod +x ninja
   10  export PATH=$HOME/local/ninja:$PATH
   11  ninja --version
   12  cd -
   13  ./configure --prefix=$HOME/local/qemu-10.2.1 \\n            --target-list=x86_64-softmmu,arm-softmmu,aarch64-softmmu \\n            --disable-werror
   14  cd /home/muboutoub/Downloads/qemu-10.2.1/build\n
   15  make -j$(nproc)
   16  make install
   17  ls -la ~/local/qemu-10.2.1/bin/
   18  ~/local/qemu-10.2.1/bin/qemu-system-x86_64 --version
   19  echo '# QEMU local installation' >> ~/.zshrc\necho 'export PATH=$HOME/local/qemu-10.2.1/bin:$PATH' >> ~/.zshrc\necho 'export LD_LIBRARY_PATH=$HOME/local/qemu-10.2.1/lib/x86_64-linux-gnu:$LD_LIBRARY_PATH' >> ~/.zshrc\n\n# Reload your shell configuration\nsource ~/.zshrc
   20  cd
   21  wget https://dl-cdn.alpinelinux.org/alpine/v3.19/releases/x86_64/alpine-virt-3.19.1-x86_64.iso
   22  ls
   23  qemu-img create -f qcow2 test-disk.qcow2 1G
   24  qemu-system-x86_64 \\n  -m 512 \\n  -cdrom alpine-virt-3.19.1-x86_64.iso \\n  -hda test-disk.qcow2 \\n  -boot d \\n  -vga std
   25  gcl https://github.com/01-bits/kernel.git
   26  cd kernel
   27  code .
   28  git switch idt
   29  git remote add z https://github.com/zone-01oujda/kernel.git
   30  gl z
   31  gl z idt
   32  gl z gdt
   33  gl z develop
   34  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
➜  ~ 

