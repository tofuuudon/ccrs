class Ccrs < Formula
  desc "A CLI tool for creating conventional commits"
  homepage "https://github.com/tofuuudon/ccrs"
  url "https://github.com/tofuuudon/ccrs/releases/download/v0.2.2/ccrs.tar.gz"
  sha256 "166108f48615cab16b6e4be4ae7d20da9eca44b9d70a51ac6f54a264f5fdeecb"

  def install
    bin.install "ccrs"
    bin.install_symlink "ccrs" => "crs"
  end
end

