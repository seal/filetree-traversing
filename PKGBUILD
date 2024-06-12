# Maintainer: Seal <will@kimbell.uk>
pkgname=filetree-traversing
pkgver=0.1.10
pkgrel=1
pkgdesc="A Rust library for working with file trees"
arch=('x86_64')
url="https://github.com/seal/filetree-traversing"
license=('MIT')
depends=('rust')
makedepends=('cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/seal/filetree-traversing/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
  cd "$pkgname-$pkgver"
  cargo build --release --locked
}

check() {
  cd "$pkgname-$pkgver"
  cargo test --release --locked
}

package() {
  cd "$pkgname-$pkgver"
  install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
  install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
}
