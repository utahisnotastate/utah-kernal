"""
Universal scaffold for all Utah-OS applications.
The OS manifests apps via intent state, not only process spawn.
"""

from __future__ import annotations

import json
import time
from typing import Any


class UtahApp:
    """The universal scaffold for all Utah-OS applications."""

    def __init__(self, name: str) -> None:
        self.name = name
        self.state: dict[str, Any] = {}

    def manifest(self) -> None:
        """Called when the OS invokes the application."""
        print(f"[UTAH-OS] Manifesting {self.name} into VRAM...")
        self.init_logic()

    def init_logic(self) -> None:
        """Override: set up app-specific state."""

    def update(self) -> None:
        """Core loop tick — runs at the refresh rate of reality."""
        pass

    def get_context(self) -> str:
        """Allows the AI assistant to read this app's data."""
        return json.dumps(self.state)

    def run_loop(self, ticks: int = 3, interval_s: float = 0.05) -> None:
        """Simple host-side demo loop."""
        self.manifest()
        for _ in range(ticks):
            self.update()
            time.sleep(interval_s)


class VibeCodeApp(UtahApp):
    """Example: app logic mutated by natural-language intent (vibe-code)."""

    def __init__(self) -> None:
        super().__init__("VibeCode")

    def init_logic(self) -> None:
        self.state = {"mode": "EVOLVING", "entropy": 0.0}

    def evolve(self, user_intent: str) -> None:
        """Evolve the app based on user / assistant commands."""
        self.state["last_thought"] = user_intent
        self.state["entropy"] = min(1.0, float(self.state.get("entropy", 0.0)) + 0.1)
        print(f"[VIBE-CODE] App logic mutated to: {user_intent}")


if __name__ == "__main__":
    app = VibeCodeApp()
    app.manifest()
    app.evolve("render glass calculator with zero latency")
    print(app.get_context())
