# Steam Redirect Executable

A Rust-based wrapper for Steam games that allows you to redirect game execution to another program. Useful for launching mod managers, custom launchers, or alternative executables instead of the original Steam game. Mostly used on Linux to redirect Proton games.

*This project is vibecoded™*

## Features

- 🎮 Redirect Steam game execution to any program
- 📁 Support for relative paths (`./`, `../`) and absolute paths
- 🚀 Handle paths with spaces using quotes
- 🔍 Automatic config file detection (searches current and parent directories)
- 📝 Simple INI-style configuration
- 🔄 Pass arguments to the wrapped program
- ✅ Exit with the same status code as the wrapped program

## Building

```bash
cargo build --release
```

The compiled binary will be in `target/release/steam-redirect`.

## Setup

1. **Build the wrapper:**
   ```bash
   cargo build --release
   ```

2. **Create a config file** named `redirect/config.cfg` in the same directory as your wrapper executable (or in a parent directory).

3. **Configure the wrapper** by specifying the program to execute:
   ```
   program=./ModOrganizer.exe -profile MyProfile
   ```

4. **Replace the Steam game executable** with this wrapper (typically `proton` or the game's main exe).

## Configuration

Create a `redirect/config.cfg` file with the following format:

```ini
program=<path> [arguments]
fallback=<path> [arguments]
```

- `program=` defines the target executable to launch when Steam starts the wrapper.
- `fallback=` defines the original launcher or fallback executable to run when `NO_REDIRECT=1` is set.

### Path Examples

**Relative path (current directory):**
```
program=./game.exe
```

**Relative path (parent directory):**
```
program=../launcher/game.exe
```

**Absolute path:**
```
program=/home/user/games/game.exe
```

**Path with spaces (use quotes):**
```
program="C:\Program Files\Game\game.exe"
```

**With arguments:**
```
program="C:\Program Files\ModOrganizer2\ModOrganizer.exe" -profile Skyrim
```

### Configuration Comments

Lines starting with `#` or `;` are treated as comments:
```ini
# This is a comment
; This is also a comment
program=./game.exe
```

## Usage

### Manual Testing

```bash
# Run the wrapper (it will look for redirect/config.cfg)
./steam-redirect [arguments]

# Any arguments passed to the wrapper are forwarded to the target program
./steam-redirect -arg1 value1
```

### With Steam/Proton

Typically, you would:

1. Locate your Steam game's executable (or Proton wrapper)
2. Backup the original
3. Replace it with this wrapper
4. Place the `redirect/config.cfg` in the game directory or a parent directory
5. When Steam launches the game, it will execute your configured program instead

### Example: Mod Organizer with Skyrim

```bash
# Directory structure:
# /games/skyrim/
#   ├── redirect/config.cfg
#   ├── steam-redirect (or steam-redirect.exe)
#   └── ...game files...

# redirect/config.cfg content:
program="C:\Program Files\ModOrganizer2\ModOrganizer.exe" -profile Skyrim
```

## Advanced Examples

### Mod Organizer with Multiple Profiles
```ini
# skyrim-wrapper.conf
program="D:\Tools\ModOrganizer2\ModOrganizer.exe" -profile "Skyrim SE"

# fallout-wrapper.conf
program="D:\Tools\ModOrganizer2\ModOrganizer.exe" -profile "Fallout 4"
```

### Custom Pre-launch Script
```bash
# launcher.sh
#!/bin/bash
echo "Launching game with custom settings..."
export PROTON_USE_WINED3D=1
exec /path/to/game "$@"

# redirect/config.cfg
program=./launcher.sh
```

### Using Fallback for Direct Game Launch
```ini
# redirect/config.cfg
program="C:\Program Files\ModOrganizer2\ModOrganizer.exe" -profile Skyrim
fallback=./SkyrimSE.exe

# Usage:
# Normal launch: runs Mod Organizer
# NO_REDIRECT=1 ./steam-redirect: runs original SkyrimSE.exe directly
```

### Compatibility Tools
```ini
# Use with Proton GE
program="./proton-ge-8.0/proton" run ./game.exe

# Use with standard Proton
program=/usr/bin/proton run "./game.exe"
```

## Testing Without Steam

Create a simple test setup:

```bash
mkdir test_setup
cd test_setup
cp ../target/release/steam-redirect .

# Create test config that echoes arguments
cat > redirect/config.cfg << 'EOF'
program=/bin/echo
EOF

# Test it
./steam-redirect "Hello World"
# Output: Hello World

# Test with spaces in arguments  
./steam-redirect "Argument with spaces"
# Output: Argument with spaces
```

## Behavior

1. The wrapper searches for `redirect/config.cfg` starting from its own directory and moving up the directory tree
2. It reads the `program=` entry
3. It resolves the program path (relative paths are resolved relative to the config file's directory)
4. It appends any arguments passed to the wrapper
5. It executes the program and forwards its exit code

## Troubleshooting

### Config file not found
- Ensure the config file is named exactly `redirect/config.cfg` (case-sensitive on Linux/Mac)
- Place it in the same directory as the wrapper or a parent directory
- Check file permissions

### program entry not found
- Verify the config file contains a line starting with `program=`
- Check for typos (should be `program=` not `Program=`)

### Failed to execute program
- Verify the program path is correct and the file exists
- On Windows with Proton, use the Drive letter mapping (C:\, D:\, etc.)
- Ensure quotes are used for paths with spaces
- Check file permissions (especially on Linux)

### Program runs but not as expected
- Check that arguments are correctly formatted
- Verify environment variables are set if needed
- Test the program directly to ensure it works

## Building for Production

```bash
cargo build --release --strip
```

This creates an optimized, stripped binary suitable for distribution.

## Performance Notes

- The wrapper is compiled as a release binary, so overhead is minimal (~1-2ms)
- The binary is about 500KB in size
- No dependencies are needed at runtime

## Security Considerations

- The wrapper executes whatever program you specify in the config
- Only use config files from trusted sources
- The wrapper runs with the same permissions as the Steam process
- Be careful with absolute paths - verify they point to legitimate programs

## License

MIT License - AS IS, no warranties or guarantees provided.

## Example Use Cases

### Skyrim/Fallout Modding
Replace the game launcher with Mod Organizer 2, which will handle mod loading before launching the game:
```
program="D:\Tools\ModOrganizer2\ModOrganizer.exe" -profile "Skyrim SE"
```

### Game Launcher Alternative
Route through a custom launcher that sets up the environment:
```
program=./custom_launcher.exe --game skyrim
```

### Compatibility Tool
Use a wrapper for compatibility configurations:
```
program="./proton-ge" run ./game.exe
```
