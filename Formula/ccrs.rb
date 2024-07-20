class Ccrs < Formula
  desc "A CLI tool for creating conventional commits"
  homepage "https://github.com/tofuuudon/ccrs"
  url "https://github.com/tofuuudon/ccrs/releases/download/v0.2.0/ccrs.tar.gz"
  sha256 "de90655a7028fe9ff0a7816119a32555d5a67418e118370d18db9aeb1fc39b5f"

  def install
    bin.install "ccrs"
    bin.install_symlink "ccrs" => "crs"
  end
end

