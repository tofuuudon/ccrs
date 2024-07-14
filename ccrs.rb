class Ccrs < Formula
  desc "A CLI tool for creating conventional commits"
  homepage "https://github.com/tofuuudon/homebrew-ccrs"
  url "https://github.com/tofuuudon/homebrew-ccrs/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "b1f7f047e8ec41eb5cab2cc152734ffcc7481ada96ddcbb17f8219e4b494bd48"
  
  depends_on "rust" => :build

  def install
    system "cargo", "build", "--release", "--bin", "ccrs"
    bin.install "target/release/ccrs"
  end
end

