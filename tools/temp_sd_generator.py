import sys
try:
    from diffusers import StableDiffusionXLPipeline, EulerAncestralDiscreteScheduler
    import torch
    from PIL import Image
    import gc
    
    print("Loading Stable Diffusion XL model for maximum speed...")
    
    # Load with speed optimizations
    pipe = StableDiffusionXLPipeline.from_pretrained(
        "stabilityai/stable-diffusion-xl-base-1.0",
        torch_dtype=torch.float16,
        use_safetensors=True,
        variant="fp16"
    )
    
    if torch.cuda.is_available():
        # Keep everything on GPU - no CPU offloading for speed
        pipe = pipe.to("cuda")
        
        # Use faster scheduler for fewer steps
        pipe.scheduler = EulerAncestralDiscreteScheduler.from_config(pipe.scheduler.config)
        
        # Enable memory efficient attention if available
        try:
            pipe.enable_xformers_memory_efficient_attention()
            print("✓ Enabled xformers optimization")
        except:
            print("⚠ xformers not available, using default attention")
        
        # Compile UNet for faster inference (PyTorch 2.0+)
        try:
            pipe.unet = torch.compile(pipe.unet, mode="reduce-overhead", fullgraph=True)
            print("✓ Compiled UNet for speed")
        except:
            print("⚠ torch.compile not available")
            
        print(f"✓ GPU: {torch.cuda.get_device_name()}")
        print(f"✓ VRAM: {torch.cuda.get_device_properties(0).total_memory // 1024**3}GB")
        
    else:
        print("ERROR: CUDA not available")
        sys.exit(1)
    
    print("SDXL model loaded and optimized!")
    
    # Generate texture with speed-optimized settings
    prompt = "tech panel surface texture, metallic blue industrial design, geometric control interface pattern, electronic components layout, seamless repeating pattern"
    print(f"Generating texture: {prompt[:50]}...")
    
    # Speed-optimized generation
    with torch.inference_mode():  # Disable gradients for faster inference
        image = pipe(
            prompt,
            num_inference_steps=20,          # Reduced from 35 for speed
            guidance_scale=5.0,              # Reduced for faster generation
            width=512,                       # Generate directly at target size
            height=512,
            generator=torch.Generator(device="cuda").manual_seed(42)  # Reproducible + GPU
        ).images[0]
    
    # Clear GPU cache
    torch.cuda.empty_cache()
    gc.collect()
    
    image.save("temp_sd_texture.png")
    print("SUCCESS: Generated 512x512 texture with speed optimizations")
    
except ImportError as e:
    print("ERROR: Missing dependencies")
    print("Install with: pip install diffusers torch transformers accelerate pillow")
    print("For SDXL: Ensure you have diffusers>=0.21.0")
    sys.exit(1)
except Exception as e:
    print(f"ERROR: {e}")
    import traceback
    traceback.print_exc()
    sys.exit(1)
