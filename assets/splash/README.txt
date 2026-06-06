Boot splash assets (drop your own files here).

  boot.mp4   — animated splash (Linux only; requires ffmpeg)
  boot.png   — static fallback (Linux + Pico firmware)
  boot.jpg   — same as PNG (also accepts boot.jpeg)

Linux tries boot.mp4 first, then boot.png / boot.jpg / boot.jpeg, then text-only.

Pico embeds the first matching static image at compile time (PNG preferred over JPEG).

Recommended sizes:
  MP4 — H.264, no audio, match layout (960×1280 portrait or 640×480 landscape)
  PNG/JPEG — same aspect as your target layout; resized at runtime / build time