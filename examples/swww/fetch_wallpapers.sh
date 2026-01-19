#!/bin/bash

# Directory containing wallpapers
WALLPAPER_DIR="$HOME/Pictures"

# Start JSON structure
cat << 'EOF'
{
  "name": "swww-wallpaper-picker",
  "view": {
    "layout": "grid"
  },
  "items": [
EOF

# Find all image files and generate JSON entries
first=true
find "$WALLPAPER_DIR" -type f \( -iname "*.jpg" -o -iname "*.jpeg" -o -iname "*.png" -o -iname "*.webp" -o -iname "*.gif" \) | sort | while read -r file; do
    # Get just the filename without path
    filename=$(basename "$file")
    # Remove extension for ID
    id="${filename%.*}"
    
    # Add comma before each item except the first
    if [ "$first" = true ]; then
        first=false
    else
        echo ","
    fi
    
    # Generate JSON entry
    cat << EOF
    {
      "id": "$id",
      "label": "$filename",
      "icon": {
        "type": "path",
        "value": "$file"
      },
      "action": {
        "action": "exec",
        "cmd": "swww img \"${file}\""
      }
    }
EOF
done

# Close JSON structure
cat << 'EOF'

  ]
}
EOF
