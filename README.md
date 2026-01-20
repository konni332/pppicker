# pppicker

A generic, highly customizable picker application for Wayland compositors. Inspired by tools like `rofi` and `wofi`, but with a focus on extreme stylability through CSS and JSON configuration.

## Features

- **JSON-driven content**: Define your picker items in simple JSON format
- **CSS styling**: Full control over appearance through CSS
- **Multiple layouts**: Grid and list views
- **Image support**: Display images as icons (perfect for wallpaper pickers)
- **Action types**: Execute commands, print values, or exit with codes
- **Per-picker styling**: Different styles for different use cases
- **Wayland native**: Built with GTK4 for modern Wayland compositors

## Installation

### Prerequisites

- GTK4
- Rust toolchain (for building from source)

### From Source

```bash
git clone https://github.com/konni332/pppicker
cd pppicker
cargo build --release
sudo cp target/release/pppicker /usr/local/bin/
```

## Usage

pppicker reads JSON input either from a file or stdin:

```bash
# From file
pppicker config.json

# From stdin
echo '{"name": "test", "items": [...]}' | pppicker

# Using a script
./wallpaper-picker.sh | pppicker
```

### Input Format

```json
{
  "name": "example-picker",
  "view": {
    "layout": "grid"
  },
  "items": [
    {
      "id": "item1",
      "label": "Example Item",
      "icon": {
        "type": "unicode",
        "value": "🎨"
      },
      "action": {
        "action": "exec",
        "cmd": "notify-send 'Selected!'"
      }
    }
  ]
}
```

#### View Layouts

- `"list"` - Vertical list layout (default)
- `"grid"` - Grid layout for visual content

#### Search Bar (Optional)

Add a search bar to filter items:
```json
{
  "name": "example-picker",
  "search_bar": {
    "placeholder": "Search..."
  },
  "view": {
    "layout": "grid"
  },
  "items": [...]
}
```

The search bar filters items by their labels as you type.

#### Icon Types

```json
// Unicode emoji or character
"icon": {
  "type": "unicode",
  "value": "🖼️"
}

// Image file path
"icon": {
  "type": "path",
  "value": "/path/to/image.png"
}

// No icon
"icon": null
```

#### Action Types

```json
// Execute a shell command
"action": {
  "action": "exec",
  "cmd": "hyprctl dispatch ..."
}

// Print value to stdout
"action": {
  "action": "print",
  "value": "selected-value"
}

// Exit with code
"action": {
  "action": "exit",
  "code": 0
}
```

### Keyboard Controls

- **Arrow Keys / Page Up/Down / Home/End**: Navigate items
- **Type to search**: When search bar is enabled, typing filters items
- **Backspace**: Delete last character in search (when search bar is enabled)
- **Enter**: Select item and execute action
- **Escape**: Close picker

## Styling

pppicker looks for CSS files in the following order:

1. `~/.config/pppicker/{name}.css` - Picker-specific style
2. `./style.css` - Current directory (for testing, only when built with debug profile)
3. `~/.config/pppicker/style.css` - Global user style
4. Built-in default (Gruvbox Dark)

The `{name}` is taken from the JSON input's `"name"` field.

### Available CSS Classes

```css
.picker-window          /* Main window */
.picker-search           /* Search entry */
.picker-scrolled        /* Scrolled container */
.picker-list            /* List view container */
.picker-grid            /* Grid view container */
.picker-row             /* List row item */
.picker-grid-item       /* Grid item */
.picker-grid-item-box   /* Grid item inner box */
.picker-label           /* Item label text */
.picker-icon            /* Icon container */
.picker-icon-unicode    /* Unicode icons */
.picker-icon-path       /* Image icons */
```

### Example Styles

#### Minimal Dark (Nord)

```css
.picker-window {
    background-color: #2e3440;
}

.picker-grid-item {
    padding: 16px;
    margin: 8px;
    background-color: #3b4252;
    border-radius: 12px;
}

.picker-grid-item:selected {
    background-color: #88c0d0;
}

.picker-grid .picker-icon-path {
    min-width: 100px;
    min-height: 100px;
    -gtk-icon-size: 100px;
    border-radius: 8px;
}
```

#### Single Preview Style

```css
.picker-window {
    min-width: 700px;
    min-height: 800px;
}

.picker-grid-item {
    padding: 60px;
    margin: 0;
    min-height: 800px;
}

.picker-grid-item-box {
    min-width: 700px;
}

.picker-grid .picker-icon-path {
    min-width: 500px;
    min-height: 500px;
    -gtk-icon-size: 500px;
}
```

## Compositor Configuration

### Hyprland

For floating window behavior without layer-shell:

```conf
# ~/.config/hypr/hyprland.conf

windowrulev2 = float, title:^(pppicker)$
windowrulev2 = center, title:^(pppicker)$
windowrulev2 = size 600 400, title:^(pppicker)$
windowrulev2 = noborder, title:^(pppicker)$
```

### Sway

```conf
# ~/.config/sway/config

for_window [title="pppicker"] floating enable
for_window [title="pppicker"] resize set 600 400
for_window [title="pppicker"] move position center
```

### River

```bash
riverctl rule-add float -title "pppicker"
riverctl rule-add dimensions 600 400 -title "pppicker"
riverctl rule-add position center -title "pppicker"
```

## Example Use Cases

### Wallpaper Picker (hyprpaper)

Create a script `wallpaper-picker.sh`:

```bash
#!/bin/bash

WALLPAPER_DIR="$HOME/Pictures"

cat << 'EOF'
{
  "name": "hyprpaper-wallpaper-picker",
  "view": { "layout": "grid" },
  "items": [
EOF

first=true
find "$WALLPAPER_DIR" -type f \( -iname "*.jpg" -o -iname "*.png" \) | sort | while read -r file; do
    filename=$(basename "$file")
    id="${filename%.*}"
    
    [ "$first" = true ] && first=false || echo ","
    
    cat << EOF
    {
      "id": "$id",
      "label": "$filename",
      "icon": { "type": "path", "value": "$file" },
      "action": {
        "action": "exec",
        "cmd": "hyprctl hyprpaper preload \\"${file}\\" && hyprctl hyprpaper wallpaper \\",${file}\\""
      }
    }
EOF
done

echo '  ]
}'
```

Usage:
```bash
chmod +x wallpaper-picker.sh
./wallpaper-picker.sh | pppicker
```

### Application Launcher

```json
{
  "name": "app-launcher",
  "view": { "layout": "list" },
  "items": [
    {
      "id": "firefox",
      "label": "Firefox",
      "icon": { "type": "unicode", "value": "🌐" },
      "action": { "action": "exec", "cmd": "firefox" }
    },
    {
      "id": "terminal",
      "label": "Terminal",
      "icon": { "type": "unicode", "value": "💻" },
      "action": { "action": "exec", "cmd": "kitty" }
    }
  ]
}
```

### Theme Selector

```json
{
  "name": "theme-picker",
  "view": { "layout": "grid" },
  "items": [
    {
      "id": "gruvbox",
      "label": "Gruvbox Dark",
      "icon": { "type": "unicode", "value": "🌲" },
      "action": { "action": "print", "value": "gruvbox-dark" }
    },
    {
      "id": "nord",
      "label": "Nord",
      "icon": { "type": "unicode", "value": "❄️" },
      "action": { "action": "print", "value": "nord" }
    }
  ]
}
```

Then capture the output:
```bash
theme=$(echo '...' | pppicker)
echo "Selected theme: $theme"
```

## Compatibility

### Tested Compositors

- Hyprland
- Sway
- River (limited testing)
- Wayfire (limited testing)

### Display Servers

- ✅ Wayland (primary target)
- ❌ X11 (not supported)

### GTK Version

- Requires GTK4 (4.0+)

## Logging

Logs are written to:
- `~/.cache/pppicker/pppicker.log` (file, no colors)
- stderr (with ANSI colors)

Control log level with `RUST_LOG`:
```bash
RUST_LOG=debug pppicker config.json
```

## Troubleshooting

### Window not floating on Hyprland

Ensure you have the windowrules configured (see [Compositor Configuration](#compositor-configuration)).

### Images not displaying

- Check file paths are absolute
- Verify image formats (PNG, JPG, WEBP supported)
- Check file permissions

### CSS not loading

Check load order:
1. Verify `"name"` field in JSON matches your CSS filename
2. Check `~/.config/pppicker/` directory exists
3. View logs at `~/.cache/pppicker/pppicker.log`

### Icons cut off or clipped

Adjust sizing in CSS:
```css
.picker-grid .picker-icon-path {
    min-width: 80px;
    min-height: 80px;
    -gtk-icon-size: 80px;
}
```

And ensure proper padding:
```css
.picker-grid-item {
    padding: 12px;
    margin: 6px;
}
```

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

[MIT License](./LICENSE.md)

## Credits

Inspired by:
- rofi
- wofi
