class Ccrs < Formula
  desc "A CLI tool for creating conventional commits"
  homepage "https://github.com/tofuuudon/ccrs"
  url "https://github.com/tofuuudon/ccrs/releases/download/v0.2.2/ccrs.tar.gz"
  sha256 "aff008587f1f0760580182d99a4df970dfc4e4ce81d45dccdc7fff75301e5e30"

  def install
    bin.install "ccrs"
    bin.install_symlink "ccrs" => "crs"
  end
end

