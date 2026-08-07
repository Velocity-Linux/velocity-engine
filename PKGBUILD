# Maintainer: Velocity OS Team <team@velocity-os.org>
pkgname=velocity-engine
pkgver=0.1.0
pkgrel=1
pkgdesc="Core optimization daemon for Velocity OS"
arch=('x86_64')
url="https://github.com/velocity-os/velocity-engine"
license=('GPL-3.0-only')
depends=('dbus' 'power-profiles-daemon')
makedepends=('cargo' 'rust' 'pkg-config' 'dbus')
provides=("${pkgname}")
conflicts=("${pkgname}")
backup=("etc/velocity-engine/default.toml")
install=".packaging/velocity-engine.install"
source=("${pkgname}::git+https://github.com/velocity-os/velocity-engine.git")
sha256sums=('SKIP')

pkgver() {
  cd "${srcdir}/${pkgname}"
  printf "%s" "$(grep -E '^version = "[^"]+"' Cargo.toml | sed -E 's/version = "([^"]+)"/\1/')"
}

build() {
  cd "${srcdir}/${pkgname}"
  cargo build --release
}

check() {
  cd "${srcdir}/${pkgname}"
  CARGO_TARGET_DIR="${srcdir}/${pkgname}/target-test" cargo test --lib
}

package() {
  cd "${srcdir}/${pkgname}"

  install -Dm0755 target/release/velocity-engine -t "${pkgdir}/usr/bin/"
  install -Dm0755 target/release/velocityctl -t "${pkgdir}/usr/bin/"

  install -Dm0644 systemd/velocity-engine.service -t "${pkgdir}/usr/lib/systemd/system/"
  install -Dm0644 config/default.toml -t "${pkgdir}/etc/velocity-engine/"
}
