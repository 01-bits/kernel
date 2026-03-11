#!/bin/bash
# QEMU 10.2.1 Build & Install Script (auto-detects source) - Fixed zsh issue
# Run from anywhere - finds QEMU source automatically

set -e  # Exit on any error

PREFIX="$HOME/local/qemu-10.2.1"
NINJA_DIR="$HOME/local/ninja"
TARGETS="x86_64-softmmu,arm-softmmu,aarch64-softmmu"

echo "🚀 QEMU 10.2.1 Build Script"

# Auto-find QEMU source (handles running from anywhere)
find_qemu_src() {
    if [ -f "./configure" ]; then
        echo "$(pwd)"
    elif [ -f "../configure" ]; then
        echo "$(pwd)/.."
    elif [ -f "$HOME/Downloads/qemu-10.2.1/configure" ]; then
        echo "$HOME/Downloads/qemu-10.2.1"
    else
        echo "❌ QEMU source not found!"
        echo "  - Run from qemu-10.2.1/ directory"
        echo "  - OR extract QEMU source to ~/Downloads/qemu-10.2.1/"
        exit 1
    fi
}

QEMU_SRC=$(find_qemu_src)
echo "📁 Found QEMU source: $QEMU_SRC"

# 1. Install tomli dependency
echo "📦 Installing Python tomli..."
pip3 install --user tomli

# 2. Ninja build tool (faster than make)
if [ ! -f "$NINJA_DIR/ninja" ]; then
    echo "⚡ Installing Ninja build tool..."
    mkdir -p "$NINJA_DIR"
    cd "$NINJA_DIR"
    wget https://github.com/ninja-build/ninja/releases/download/v1.12.1/ninja-linux.zip
    unzip ninja-linux.zip
    chmod +x ninja
    rm ninja-linux.zip
    cd -
fi
export PATH="$NINJA_DIR:$PATH"
echo "✅ Ninja ready: $(ninja --version)"

# 3. Clean & configure QEMU
echo "🔧 Configuring QEMU..."
cd "$QEMU_SRC"
rm -rf build
mkdir -p build
cd build
../configure --prefix="$PREFIX" \
              --target-list="$TARGETS" \
              --disable-werror

# 4. Build & Install
echo "🏗️  Building QEMU ($(nproc) cores)..."
ninja -j$(nproc)
echo "📦 Installing to $PREFIX..."
ninja install

# 5. Update shell profile (zsh-safe, no source)
if ! grep -q "QEMU local installation" ~/.zshrc 2>/dev/null; then
    {
        echo ''
        echo '# QEMU local installation'
        echo "export PATH=$PREFIX/bin:\$PATH"
        echo "export LD_LIBRARY_PATH=$PREFIX/lib/x86_64-linux-gnu:\$LD_LIBRARY_PATH"
    } >> ~/.zshrc
    echo "✅ Profile updated → Run: source ~/.zshrc  OR restart terminal"
else
    echo "✅ Profile already exists"
fi

# 6. Verification
echo "✅ BUILD COMPLETE!"
echo ""
echo "📋 Test QEMU:"
echo "  $PREFIX/bin/qemu-system-x86_64 --version"
echo ""
echo "💾 Test Alpine VM (if you have the ISO):"
echo "  qemu-img create -f qcow2 test-disk.qcow2 1G"
echo "  qemu-system-x86_64 -m 512 -cdrom alpine-virt-3.19.1-x86_64.iso \\"
echo "    -hda test-disk.qcow2 -boot d -vga std"
echo ""
echo "🎉 Ready for kernel development!"
