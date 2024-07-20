class Ccrs < Formula
  desc "A CLI tool for creating conventional commits"
  homepage "https://github.com/tofuuudon/ccrs"
  url "https://github.com/tofuuudon/ccrs/releases/download/v0.2.1/ccrs.tar.gz"
  sha256 "9df7a61d395cfd676320a346c3f95c50be533f470fac9b00bb90fb41b1082490"

  def install
    bin.install "ccrs"
    bin.install_symlink "ccrs" => "crs"
  end
end

