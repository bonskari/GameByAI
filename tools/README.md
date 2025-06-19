# AI Texture Generation Tools

This directory contains the AI texture generation tools for the GameByAI project.

## Files

### `sdxl_server.py`
The main SDXL texture generation server that keeps the model loaded in memory for ultra-fast generation.

**Usage:**
```bash
# Generate single texture
python tools/sdxl_server.py single "your prompt here" output.png

# Generate multiple textures in batch
python tools/sdxl_server.py batch "prompt1" "prompt2" "prompt3"
```

**Features:**
- Keeps SDXL model loaded in memory (5-6s initial load, then 1.6-2.3s per texture)
- GPU-optimized with CUDA support
- Unicode-safe output for Windows
- Progress tracking and performance metrics

### `temp_sd_generator.py`
Backup single-shot SDXL script (created automatically if needed).

## Performance

With RTX 3080 Ti (12GB VRAM):
- **Model loading**: ~5-6 seconds (one-time)
- **Generation time**: **1.6-2.3 seconds per texture**
- **GPU utilization**: Properly uses full GPU power
- **Memory usage**: ~2-3GB VRAM

## Requirements

```bash
pip install diffusers torch transformers accelerate pillow
```

For CUDA support (required for speed):
```bash
pip install torch torchvision --index-url https://download.pytorch.org/whl/cu118
```

## Integration

The Rust game automatically calls these tools when generating textures:
```bash
cargo run -- generate-textures --local-only
```

Generated textures are saved directly to `assets/textures/` where the game loads them.

## Quality

These tools generate **real SDXL textures** (300-400KB file sizes) with high detail and quality, not procedural patterns. The system has been cleaned of all fallback procedural generation to ensure only authentic AI-generated textures are used. 