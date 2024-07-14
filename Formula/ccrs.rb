class Ccrs < Formula
  desc "A CLI tool for creating conventional commits"
  homepage "https://github.com/tofuuudon/ccrs"
  url "https://github.com/tofuuudon/ccrs/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "276f6f8aa3f60a1521a81990024de806d65e650145fc15311bf1167d53971257"

  def install
    bin.install "ccrs"
  end
end

