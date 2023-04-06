cask "nofwl" do
  arch arm: "aarch64", intel: "x86_64"

  version "0.1.0"
  sha256 arm:   "dddf534069abb9445022aa5a3c157bb2565a165cd82d32c51fa9cc118162e128",
         intel: "19b5c1daebf6e954ebd72c5c162da00a3f6a2b9a305e23a9a8dc1cd7e5ee03a4"

  url "https://github.com/lencx/nofwl/releases/download/v#{version}/NoFWL_#{version}_macos_#{arch}.dmg"
  name "NoFWL"
  desc "Desktop application"
  homepage "https://github.com/lencx/nofwl"

  livecheck do
    url :homepage
    strategy :github_latest
  end

  app "NoFWL.app"

  zap trash: [
    "~/.nofwl",
    "~/Library/Caches/com.nofwl.app",
    "~/Library/Preferences/com.nofwl.app.plist",
    "~/Library/Saved Application State/com.nofwl.app.savedState",
    "~/Library/WebKit/com.nofwl.app",
  ]
end
