# Maintainer: Gaurav Atreya <allmanpride@gmail.com>
pkgname=artha
pkgver=0.1.0
pkgrel=1
pkgdesc="Minimal Nepali Dictionary Desktop Application"
arch=('x86_64')
license=('GPL3')
depends=('gcc-libs')
makedepends=('rust' 'cargo')

build() {
	cargo build --release
}

package() {
    cd "$srcdir"
    mkdir -p "$pkgdir/usr/bin" "$pkgdir/usr/share/applications/"
    cp "../target/release/${pkgname}" "$pkgdir/usr/bin/${pkgname}"
    cp "../artha.desktop" "$pkgdir/usr/share/applications/"
}
