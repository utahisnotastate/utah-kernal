"""
Utah-Browser — direct-to-memory rendering scaffold.

Production: Wry engine paints HTML into the Glass-Forge framebuffer without
Chrome/Firefox DOM overhead. This module defines the intent interface.
"""

from __future__ import annotations

import sys
from pathlib import Path

# Allow running as script: python genesis/src/apps/browser.py
_ROOT = Path(__file__).resolve().parents[2]
if str(_ROOT) not in sys.path:
    sys.path.insert(0, str(_ROOT))

from src.core.base_app import UtahApp  # noqa: E402


class UtahBrowser(UtahApp):
    """Minimalist framebuffer browser bridge."""

    def __init__(self) -> None:
        super().__init__("Utah-Browser")
        self.current_url = "about:utah"

    def init_logic(self) -> None:
        self.state = {
            "engine": "glass-forge-vram",
            "wry_ready": False,
            "url": self.current_url,
        }

    def navigate(self, url: str) -> None:
        """Request VRAM render for a URL (host stub; WASM/GPU path in production)."""
        self.current_url = url
        self.state["url"] = url
        print(f"[BROWSER] Rendering VRAM for: {url}")
        print("[BROWSER] Pipeline: intent -> HFS cache -> Glass-Forge blit")
        # Future: wry.WebView -> RGBA buffer -> utah_system.render_interface_node

    def update(self) -> None:
        self.state["frame"] = self.state.get("frame", 0) + 1


def main() -> None:
    browser = UtahBrowser()
    browser.manifest()
    browser.navigate("https://utah-os.local/genesis")
    browser.run_loop(ticks=2, interval_s=0.02)
    print(browser.get_context())


if __name__ == "__main__":
    main()
