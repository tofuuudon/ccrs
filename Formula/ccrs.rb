class Ccrs < Formula
  desc "A CLI tool for creating conventional commits"
  homepage "https://github.com/tofuuudon/ccrs"
  url "https://github.com/tofuuudon/ccrs/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "037c9f91b074a01faadc719dc3ba341426fc9a973deaf906cfd7435b2252150f"
  
  depends_on "rust" => :build

  def install
    system "cargo", "build", "--release", "--bin", "ccrs"
    bin.install "target/release/ccrs"
  end
end

