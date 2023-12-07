# Maintainer: Jere Hasu
pkgname=picmaker
pkgver=0.1.1
pkgrel=1
makedepends=('rust' 'cargo')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
pkgdesc="Makes arduino instructions from svg files"
url="https://wiki.aalto.fi/pages/viewpage.action?pageId=234042176"
license=('GPL-3.0-or-later')

build() {
    return 0
}

package() {
    cd $srcdir
    cargo install --root="$pkgdir" --git=https://wiki.aalto.fi/pages/viewpage.action?pageId=234042176
}
