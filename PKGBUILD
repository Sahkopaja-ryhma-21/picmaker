# Maintainer: Jere Hasu
#
# This PKGBUILD was generated by `cargo aur`: https://crates.io/crates/cargo-aur

pkgname=picmaker-bin
pkgver=0.1.0
pkgrel=1
pkgdesc="Makes arduino instructions from svg files"
url="https://wiki.aalto.fi/pages/viewpage.action?pageId=234042176"
license=("GPL-3.0-or-later")
arch=("x86_64")
provides=("picmaker")
conflicts=("picmaker")
source=("https://wiki.aalto.fi/pages/viewpage.action?pageId=234042176/releases/download/v$pkgver/picmaker-$pkgver-x86_64.tar.gz")
sha256sums=("c2b16f014efcf18937f19aa503df6b94ba484ca33908cde77bcc8a80c0f61128")

package() {
    install -Dm755 picmaker -t "$pkgdir/usr/bin"
}
