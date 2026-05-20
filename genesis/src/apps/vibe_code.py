"""CLI demo for VibeCodeApp."""

from __future__ import annotations

import sys
from pathlib import Path

_ROOT = Path(__file__).resolve().parents[2]
if str(_ROOT) not in sys.path:
    sys.path.insert(0, str(_ROOT))

from src.core.base_app import VibeCodeApp  # noqa: E402


def main() -> None:
    app = VibeCodeApp()
    app.manifest()
    if len(sys.argv) > 1:
        app.evolve(" ".join(sys.argv[1:]))
    else:
        app.evolve("open holographic file explorer")
    print(app.get_context())


if __name__ == "__main__":
    main()
