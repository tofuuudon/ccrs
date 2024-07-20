class Ccrs < Formula
  desc "A CLI tool for creating conventional commits"
  homepage "https://github.com/tofuuudon/ccrs"
  url "https://github.com/tofuuudon/ccrs/releases/download/v0.2.1/ccrs.tar.gz"
  sha256 "da93bd8c49e80bf760f125f3c96c2aefd7be68e77dfc382ca09811b4daf02d3e"

  def install
    bin.install "ccrs"
    bin.install_symlink "ccrs" => "crs"
  end
end

