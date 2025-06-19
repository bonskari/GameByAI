# 🎨 AI Texture Generation System

This game includes an AI-powered texture generation system using Hugging Face Stable Diffusion to create unique sci-fi textures for walls and surfaces.

## 🚀 Quick Start

1. **Get a Hugging Face API Token** (FREE):
   - Visit: https://huggingface.co/settings/tokens
   - Create a new token with "Read" permissions
   - Copy the token

2. **Set up your API token** (choose one method):
   ```bash
   # Method 1: Environment variable
   export HUGGINGFACE_API_TOKEN=your_token_here
   
   # Method 2: Create .env file
   echo "HUGGINGFACE_API_TOKEN=your_token_here" > .env
   
   # Method 3: Create token file
   echo "your_token_here" > api_token.txt
   ```

3. **Generate textures**:
   ```bash
   # Test API connection
   cargo run -- generate-textures --test-only
   
   # Generate all texture types
   cargo run -- generate-textures
   
   # Generate specific texture
   cargo run -- generate-textures --texture-type tech-panel
   ```

## 🎯 Available Texture Types

- **tech-panel**: High-tech control interfaces with buttons and screens
- **hull-plating**: Industrial spaceship hull with rivets and weathering
- **control-system**: Advanced holographic displays with circuitry
- **energy-conduit**: Glowing power lines and electrical circuits

## 🛠️ Command Options

```bash
cargo run -- generate-textures [OPTIONS]

Options:
  -o, --output <OUTPUT>              Output directory [default: assets/generated_textures]
  -t, --token <TOKEN>                Hugging Face API token
  -m, --model <MODEL>                Stable Diffusion model [default: stabilityai/stable-diffusion-2-1]
      --test-only                    Test API connection only
      --texture-type <TEXTURE_TYPE>  Generate specific texture type
  -h, --help                         Print help
```

## 🎨 Prompts Used

The system uses carefully crafted prompts for each texture type:

### Tech Panel
```
pixel art sci-fi tech panel, detailed control interface, buttons and screens, 
metallic surface, blue and cyan glowing elements, cyberpunk style, 8-bit retro, 
sharp pixels, seamless tileable texture
```

### Hull Plating
```
pixel art spaceship hull plating, industrial metal panels, rivets and bolts, 
weathered steel texture, grey and dark blue tones, seamless tileable, 
8-bit retro style, sharp pixels
```

### Control System
```
pixel art advanced control system, holographic displays, complex circuitry, 
green and amber lights, high-tech interface, cyberpunk aesthetic, 
seamless tileable texture, 8-bit style
```

### Energy Conduit
```
pixel art energy conduit, glowing power lines, electrical circuits, 
purple and blue energy flowing, neon lights, futuristic piping, 
seamless tileable, 8-bit retro pixel art
```

## 🔧 Integration with Game

Generated textures are automatically used by the game's texture loading system:

1. **Fallback System**: Game loads existing textures if generation fails
2. **Cache System**: Generated textures are saved locally for reuse
3. **Seamless Integration**: No code changes needed in rendering pipeline

## 📁 Output Structure

```
assets/generated_textures/
├── tech_panel.png      # High-tech control interfaces
├── hull_plating.png    # Industrial spaceship hull
├── control_system.png  # Advanced holographic displays  
└── energy_conduit.png  # Glowing power conduits
```

## ⚡ Performance Tips

- **Free Tier**: Hugging Face provides free API access with rate limits
- **Respectful Usage**: System includes 2-second delays between requests
- **Local Caching**: Generated textures are saved for reuse
- **Error Handling**: Graceful fallback if API is unavailable

## 🎮 Using in Game

1. Generate textures: `cargo run -- generate-textures`
2. Copy to main assets: `cp assets/generated_textures/* assets/textures/`
3. Run game: `cargo run`

The new AI-generated textures will be loaded automatically!

## 🛡️ API Security

- **Token Security**: Never commit API tokens to version control
- **Environment Variables**: Use secure token storage methods
- **Rate Limiting**: Built-in delays respect API limits
- **Error Handling**: Graceful degradation if API unavailable

## 🎨 Customization

To modify prompts or add new texture types:

1. Edit `src/game/textures/ai_generator.rs`
2. Update the `prompt_templates` in `AITextureGenerator::new()`
3. Add new wall types in `src/game/map.rs` if needed

## 🆘 Troubleshooting

### No API Token
```
⚠️  No API token provided!
   Set HUGGINGFACE_API_TOKEN environment variable
   OR use --token argument
   OR create api_token.txt file
   OR create .env file with HUGGINGFACE_API_TOKEN=your_token
```

### API Connection Failed
- Check your internet connection
- Verify your API token is valid
- Try `--test-only` flag to test connection

### Generated Images Look Wrong
- Hugging Face models may need "warming up" - try again
- Free tier may have quality limitations
- Consider upgrading to paid tier for better results

---

🚀 **Ready to create unique AI-generated textures for your game!** 