# Maintainer: WMDE <https://wmde.fun>
# Contributor: System76 <info@system76.com> (original cosmic-panel)
#
# Builds our fork Lin-WMDE/wmde-panel (branch wmde). WMDE panel/dock component:
# installs its own binary wmde-panel and its own config namespace fun.wmde.Panel
# (+ .Panel/.Dock default schemas), so it co-installs beside cosmic-panel with
# NO conflicts/replaces/provides of any cosmic-* package.
pkgname=wmde-panel
pkgver=1.0.0
pkgrel=3
pkgdesc="WMDE panel and dock (fork of cosmic-panel) - owns the fun.wmde.Panel config"
arch=('x86_64')
url="https://wmde.fun"
license=('GPL-3.0-only')
# runtime: wayland client + EGL/GL rendering via smithay (use_system_lib), xkbcommon.
# Verify with namcap after first build.
depends=('glibc' 'gcc-libs' 'wayland' 'libglvnd' 'libxkbcommon')
makedepends=('rust' 'cargo' 'just' 'git' 'wayland' 'clang' 'lld' 'pkgconf' 'libxkbcommon')
source=("$pkgname::git+https://github.com/Lin-WMDE/wmde-panel.git#branch=wmde")
sha256sums=('SKIP')

pkgver() {
  cd "$srcdir/$pkgname"
  git describe --long --tags --abbrev=7 2>/dev/null | sed 's/^epoch-//;s/^v//;s/\([^-]*-g\)/r\1/;s/-/./g' ||
    printf '1.0.0.r%s.g%s' "$(git rev-list --count HEAD)" "$(git rev-parse --short=7 HEAD)"
}

build() {
  cd "$srcdir/$pkgname"
  # x86-64-v3 (AVX2/BMI2) baseline for the WMDE repo; runs on Haswell+ (and the VM).
  export RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }-C target-cpu=x86-64-v3"
  just build-release
}

package() {
  cd "$srcdir/$pkgname"
  # installs /usr/bin/wmde-panel and the default schemas to
  # /usr/share/wmde/fun.wmde.Panel{,.Panel,.Dock}/v1/ (config-root flipped to wmde per H.1)
  just rootdir="$pkgdir" prefix=/usr install
  install -Dm644 LICENSE.md "$pkgdir/usr/share/licenses/$pkgname/LICENSE.md"
}
