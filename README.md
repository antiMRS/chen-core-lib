# **CH**aracter **EN**gine core lib

**chen_core_lib** a set of functions for creating small and stylized games, both in the console and in a separate window. Provides the minimum necessary functionality.

---

## Version generation rule

**0**.**1**.**1**

The number increases if:

First number - major change

Second number - release

Third number - bugfix / issue

---

## Contributing

Delete the file TODO.md in the master branch, since this is a finished release and the tasks in it don't look good.

All major changes need to be made in the dev branch.

---

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
chen_core_lib = { git = "https://github.com/antiMRS/chen-core-lib/" }
```

Enable desired features:

```toml
chen_core_lib = { features = ["colored", "styled", "use_gui"] }
```

Both can be combined

```toml
[dependencies]
chen_core_lib = { git = "https://github.com/antiMRS/chen-core-lib/", features = ["colored", "styled", "use_gui"] }
```

Available features:

- `colored` – enables symbol colors.
- `styled` – enables bold/underline styles.
- `use_gui` – enables `GuiTerminal` and fonts.
- `terminal_color_legacy` – use ANSI colors in terminal.
- `terminal_color_cubes` – use 256‑color palette in terminal.
- `terminal_color_rgb` – use 24‑bit RGB in terminal (Not supported by all terminals).

---

## Usage

A minimal example that creates a window and displays a sprite:

```rust
use chen_core_lib::builtins::*;
use chen_core_lib::system::{GuiTerminal, GuiConfig};

fn main() {
    let mut screen = GuiTerminal::new(40, 20, GuiConfig::default());
    let mut sprite = Sprite::new(5, 5);
    sprite.fill('@');
    sprite.fill_color(Color::new(255, 200, 100));

    screen.blit(&sprite, &Position::new(10, 5));
    screen.render();

    while screen.is_open() {
        let _= screen.poll_events();

        screen.render();

        std::thread::sleep(Duration::from_millis(10));
    }
}
```

For terminal output, use `Terminal` instead:

```rust
use chen_core_lib::system::Terminal;

let mut term = Terminal::new("My Game", 50, 50);
term.blit(&sprite, &Position::new(0, 0));
term.render();
```

---

## Modules and Structures

### Builtins

The `builtins` module contains the core data types.

#### `Position`

Represents a point in 2D space (signed 64 bit integer coordinates).

```rust
let p = Position::new(10, 5);
```

#### `Size`

Stores width and height (unsigned 64 bit integers).

```rust
let s = Size::new(40, 20);
```

#### `Vector`

A 2D vector for movement and arithmetic.

```rust
let v = Vector::new(3, -2);
```

#### `Geometry`

A polygon defined by a list of `Position` vertices.

```rust
let rect = Geometry::new(vec![
    Position::new(0,0), Position::new(5,0),
    Position::new(5,3), Position::new(0,3)
]);
```

#### `Color`

RGB color (24‑bit).

```rust
let custom = Color::new(25, 45, 97);
let white = Color::white();
```

#### `Sprite`

A 2D grid of characters with optional per‑cell colors and styles.

```rust
let mut sp = Sprite::new(10, 10);
```

#### `PixelBuffer` (GUI only)

A raw pixel buffer (32‑bit ARGB) used by `GuiTerminal`. You can render custom pixel art into it and blit it to the screen.
It is used for optimization, as it does not require obtaining glyphs from a font.

```rust
let mut pb = PixelBuffer::new(64, 64);
pb.fill(0xFFFF0000);
screen.blit_buffer(&pb, 10, 10);
```

---

### Terminal

The `Terminal` struct renders the sprite buffer to the standard output (TTY). It clears the screen and prints each cell with optional ANSI escape codes (depending on features).

| Method               | Description                                                   |
| -------------------- | ------------------------------------------------------------- |
| `new(title, w, h)`   | Creates a new terminal screen (title is ignored on Unix).     |
| `blit(&sprite, pos)` | Copies a sprite onto the internal buffer (skips empty chars). |
| `clear()`            | Fills the buffer with spaces and black color.                 |
| `render()`           | Flushes the buffer to stdout with appropriate ANSI codes.     |
| `is_open()`          | Always returns `true` (no window to close).                   |
| `poll_events()`      | Returns `None` (no input in terminal mode).                   |

---

### GuiTerminal

The `GuiTerminal` struct opens a separate window (using `minifb`) and renders characters using an 8×8 pixel font. It handles window resizing and keyboard input.

| Method                       | Description                                        |
| ---------------------------- | -------------------------------------------------- |
| `new(cols, rows, config)`    | Creates a window of the given character grid size. |
| `blit(&sprite, pos)`         | Blits a sprite onto the internal pixel buffer.     |
| `blit_buffer(&buffer, x, y)` | Direct pixel buffer blit.                          |
| `clear()`                    | Clears the entire screen.                          |
| `render()`                   | Updates the window with the current pixel buffer.  |
| `is_open()`                  | Returns `true` while the window is alive.          |
| `poll_events()`              | Returns the latest event (or `None`).              |
| `scale()` / `new_scale()`    | Get/set the pixel scaling factor.                  |

---

## Comparison: Terminal vs GuiTerminal

| Feature              | `Terminal` (TTY)                     | `GuiTerminal` (Window)                       |
| -------------------- | ------------------------------------ | -------------------------------------------- |
| **Output**           | Standard output (ANSI codes)         | Dedicated window with pixel font             |
| **Input**            | None (not implemented)               | Keyboard events with modifiers               |
| **Color depth**      | Configurable (legacy / 256 / 24‑bit) | Always 24‑bit RGB                            |
| **Resolution**       | Fixed character grid (w × h)         | Same grid, but pixels scale with window size |
| **Performance**      | Low overhead, terminal‑dependent     | Rasterizes each frame, uses GPU via minifb   |
| **Platform support** | Any terminal (Windows, Unix)         | depends on minifb                            |
| **Font**             | Terminal's default font              | Built‑in 8×8 bitmap font (font8x8)           |
| **Resizing**         | Not handled                          | Automatic – scales content to fit            |
| **Pixel buffers**    | Not supported                        | Supports raw pixel blitting                  |

---
