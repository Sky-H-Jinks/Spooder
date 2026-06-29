# Spooder — hardware & maths understanding

A personal memory bank for how the robotics stack works: Raspberry Pi 5,
Robot HAT V4, the MCU, PWM, I²C, and the servo maths. Written in my own words to
cement understanding.

---

## Acronyms

- **GPIO** — General Purpose Input/Output (a single wire the Pi can drive high or low)
- **MCU** — Microcontroller Unit (the chip on the HAT that generates the servo pulses)
- **PWM** — Pulse Width Modulation (encoding position as the width of a repeating pulse)
- **I²C** — Inter-Integrated Circuit (the 2-wire bus the Pi uses to talk to the MCU)

---

## The big picture (who does what)

- **Pi 5** — decides *what* I want ("knee to 30°"). High-level thinking.
- **MCU (on the HAT)** — generates the relentless, precise PWM pulses that hold each
  servo in position. The Pi can't do this reliably because Linux timing is jittery.
- The Pi talks to the MCU over **I²C**; the **reset** is the one exception — it goes
  over a dedicated **GPIO** wire, not the bus.

---

## GPIO reset

Purpose: flush the MCU into a clean, known state at startup so no leftover config
from a previous run is in effect.

On the Robot HAT V4 the reset line is **GPIO pin 5**. Sequence:

1. Drive GPIO5 **low** for ~**10ms** — this is the reset pulse (like holding a reset button).
2. Drive GPIO5 **high** — release the reset. High is the resting state.
3. **Wait ~200ms** — give the MCU time to boot before sending it anything.

---

## MCU

A small chip that generates **12 PWM "heartbeats" at once** — one per channel, one
channel per servo. It produces precise, steady pulses that both *move* a servo to a
position and *hold* it there.

- **Address: `0x14`** on I²C bus 1. Set the bus's slave address to `0x14` before talking.

Confirmed present on the bus with `i2cdetect`:

```
sudo i2cdetect -y 1
10: -- -- -- -- 14 -- -- ...     ← MCU answering at 0x14
```

---

## I²C (the bus)

I²C is the **2-wire bus/protocol** the Pi uses to reach the MCU — a shared "road."
Every chip on the bus sees every message, but only the chip whose **address** is
named acts on it (so `0x14` = the MCU).

- The Pi has multiple I²C buses; I'm using **bus 1**, exposed by Linux as the file
  **`/dev/i2c-1`**.
- Writing to the bus = writing bytes to that file. The MCU then interprets them.

---

### Message format (the MCU's command language)

Every write is **3 bytes**, interpreted by position:

```
[ register , value_high , value_low ]
   byte 0      byte 1       byte 2
```

- **byte 0 — register**: which "mailbox" inside the MCU (where the value goes).
- **bytes 1–2 — value**: a 16-bit number, **big-endian** (high byte first).

Example: `i2c.write(&[0x20, 0x01, 0x01])` → "register `0x20`, value **257**"
(257 = `0x0101` = high `0x01`, low `0x01`).

---

## Registers (the MCU's mailboxes)

| Register | Purpose | Value I write | Notes |
|----------|---------|---------------|-------|
| `0x40` | PWM frequency (prescaler) | `350` | → ~50Hz. Empirical, from working C++. |
| `0x44` | PWM resolution | `4095` | 4096 steps (0–4095). 1 tick ≈ 4.88µs. |
| `0x20`–`0x2B` | Channel positions (0–11) | tick count | `0x20 + channel`. Holds servo position. |

**Init order:** reset MCU → write `0x40` (freq) → write `0x44` (resolution) → then
positions. Frequency sets the 20ms cycle (50Hz); resolution slices that cycle into
4096 ticks. The two together define **1 tick = 20000µs ÷ 4096 ≈ 4.88µs**.

---

## Servo channel mapping

Servo position registers start at `0x20`, one per channel. Each leg has 3 servos
(ankle, knee, hip), grouped so channels line up per leg.

**My wiring (NOTE: offsets differ from the original C++ — ankle/knee/hip, not hip/knee/ankle):**

```
Leg 0 (id = 0):
  Ankle (offset 0) → channel 0 → 0x20
  Knee  (offset 1) → channel 1 → 0x21
  Hip   (offset 2) → channel 2 → 0x22
```

Channel for any joint:

```
channel = leg.id * 3 + joint.offset
register = 0x20 + channel
```

---

## Servo maths

Servos accept **−90° … +90°**, instructed via pulse width:

```
500µs → −90°    1500µs → 0° (centre)    2500µs → +90°
```

The chain from a desired angle to bytes on the wire is **three** steps:

### Step 1 — angle → pulse width (µs)

Clamp the angle to ±90°, then map linearly (centre 1500µs, ±1000µs swing):

```
pulse_us = 1500 + (angle / 90) * 1000
```

- 0°  → 1500 + (0/90)*1000   = **1500µs**
- 75° → 1500 + (75/90)*1000  = 1500 + 833.3 = **2333µs**
- −90°→ 1500 + (−1)*1000     = **500µs**   (do the maths in f32 — a negative
  intermediate cast straight to an unsigned int saturates to 0!)

### Step 2 — pulse width → ticks

The MCU's position registers speak **ticks**, not microseconds. Convert using the
resolution (4095) over the 20ms (20000µs) frame:

```
ticks = (pulse_us * 4095) / 20000
```

- 1500µs → ~**307** ticks (= 1500 / 4.88)
- 500µs  → ~102 ticks
- 2500µs → ~512 ticks

### Step 3 — ticks → bytes (big-endian)

Split the 16-bit tick value into two bytes:

```
high_byte = ticks / 256   (integer division)  — "how many 256s"  → (ticks >> 8) & 0xFF
low_byte  = ticks % 256   (remainder)          — "what's left"     → ticks & 0xFF
```

> The shift/mask form (`>> 8`, `& 0xFF`) is just the fast way to write ÷256 and
> remainder. Bytes are digits in base-256: `high * 256 + low` rebuilds the value.

Example, ticks = 307:

```
307 / 256 = 1   remainder 51
high_byte = 1  = 0x01
low_byte  = 51 = 0x33
→ write [0x20, 0x01, 0x33]   (channel 0 to centre)
```

---

## Full chain, one line

```
angle → (clamp, ×11.1µs/° from 1500) → pulse_us → (×4095/20000) → ticks → (÷256, %256) → [reg, high, low] → i2c.write
```

| angle | pulse µs | ticks | bytes (chan 0) |
|-------|----------|-------|----------------|
| −90°  | 500      | ~102  | `0x20, 0x00, 0x66` |
| 0°    | 1500     | ~307  | `0x20, 0x01, 0x33` |
| +45°  | 2000     | ~410  | `0x20, 0x01, 0x9A` |
| +90°  | 2500     | ~512  | `0x20, 0x02, 0x00` |

---

## Why a servo "fries"

Not an electrical thing — driving a servo past its **mechanical stop** makes it
stall against the end and draw stall current continuously, overheating the windings
/ stripping gears. So the **angle clamp** (keeping within physical travel) is the
real protection; a pulse-width clamp is downstream insurance. Proper per-servo
min/max limits (alongside calibration offsets) are the mature version of this — a
later concern.
