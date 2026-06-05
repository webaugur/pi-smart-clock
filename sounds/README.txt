Place short WAV files here for the Linux/SDL2 build:

  tick.wav      - second tick (even seconds)
  tock.wav      - second tock (odd seconds)
  quarter.wav   - quarter-hour chime
  half.wav      - half-hour chime
  bell.wav      - top of hour

Alarm sounds (also referenced in config/alarms.csv):
  cuckoo.wav, chime.wav, alarm.wav

Keep tick/tock clips under ~500ms to avoid overlap.
Chimes use SDL_mixer Chunk on channel 0; alarms loop on channel 1.