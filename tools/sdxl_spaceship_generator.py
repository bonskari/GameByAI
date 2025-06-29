#!/usr/bin/env python3
"""
SDXL Spaceship Texture Generator
Generates high-quality realistic spaceship textures (not pixel art)
"""

import sys
import time
import hashlib
from pathlib import Path

try:
    from diffusers import StableDiffusionXLPipeline, EulerAncestralDiscreteScheduler
    import torch
    from PIL import Image
    import gc
except ImportError as e:
    print(f"ERROR: Missing dependencies: {e}")
    print("Install with: pip install diffusers torch transformers accelerate pillow xformers")
    sys.exit(1)

class SDXLSpaceshipTextureServer:
    def __init__(self):
        self.pipe = None
        self.model_loaded = False
        self.generation_count = 0
        
    def load_model(self):
        """Load SDXL model with maximum optimizations"""
        if self.model_loaded:
            return
            
        print(">> Loading SDXL model for realistic spaceship textures...")
        start_time = time.time()
        
        # Load with maximum speed optimizations
        self.pipe = StableDiffusionXLPipeline.from_pretrained(
            "stabilityai/stable-diffusion-xl-base-1.0",
            torch_dtype=torch.float16,
            use_safetensors=True,
            variant="fp16"
        )
        
        if torch.cuda.is_available():
            # Get GPU info
            gpu_name = torch.cuda.get_device_name(0)
            gpu_memory = torch.cuda.get_device_properties(0).total_memory / 1024**3
            print(f">> Detected GPU: {gpu_name} ({gpu_memory:.1f}GB VRAM)")
            
            # Move to GPU
            self.pipe = self.pipe.to("cuda")
            
            # Use faster scheduler
            self.pipe.scheduler = EulerAncestralDiscreteScheduler.from_config(
                self.pipe.scheduler.config
            )
            
            # Enable memory efficient attention
            try:
                self.pipe.enable_xformers_memory_efficient_attention()
                print(">> Enabled xformers memory optimization")
            except ImportError:
                print(">> xformers not available, using standard attention")
            
            # Warm up the pipeline
            print(">> Warming up pipeline...")
            with torch.inference_mode():
                warmup_image = self.pipe(
                    "test",
                    num_inference_steps=1,
                    guidance_scale=1.0,
                    width=512,
                    height=512,
                ).images[0]
            del warmup_image
            torch.cuda.empty_cache()
            
        else:
            print(">> WARNING: CUDA not available, using CPU (will be slow)")
            
        load_time = time.time() - start_time
        print(f">> Model loaded in {load_time:.1f}s")
        self.model_loaded = True
    
    def generate_texture(self, prompt, output_file=None):
        """Generate a single realistic spaceship texture"""
        if not self.model_loaded:
            self.load_model()
        
        start_time = time.time()
        self.generation_count += 1
        
        print(f">> Generation #{self.generation_count}: {prompt}")
        
        # Enhanced prompt for realistic textures with tiling
        enhanced_prompt = f"{prompt}, photorealistic, high quality, 4K texture, seamless tileable, clean surface, no shadows, even lighting, materials"
        
        # Negative prompt to avoid unwanted elements but ALLOW photorealism
        negative_prompt = "pixel art, cartoon, illustration, painting, drawing, sketch, low quality, blurry, noisy, artifacts, text, watermark, logo, signature, people, characters, faces"
        
        print(f">> Enhanced prompt: {enhanced_prompt}")
        
        with torch.inference_mode():
            # Generate with realistic texture optimized settings
            image = self.pipe(
                enhanced_prompt,
                negative_prompt=negative_prompt,
                num_inference_steps=30,  # Higher steps for quality
                guidance_scale=7.5,      # Balanced guidance for realism
                width=512,               # Keep high resolution
                height=512,              # Keep high resolution
                generator=torch.Generator(device="cuda").manual_seed(42 + self.generation_count) if torch.cuda.is_available() else None
            ).images[0]
            
            # Keep the full 512x512 resolution - no downscaling!
        
        # Clean up GPU memory
        if torch.cuda.is_available():
            torch.cuda.empty_cache()
        gc.collect()
        
        # Save the image
        if output_file:
            image.save(output_file)
            print(f">> Saved realistic spaceship texture: {output_file}")
        
        gen_time = time.time() - start_time
        print(f">> Generated realistic texture in {gen_time:.2f}s")
        
        return image
    
    def generate_batch(self, prompts_and_files):
        """Generate multiple textures efficiently"""
        if not self.model_loaded:
            self.load_model()
        
        total_start = time.time()
        results = []
        
        for i, (prompt, output_file) in enumerate(prompts_and_files):
            print(f"\n>> Batch {i+1}/{len(prompts_and_files)}")
            try:
                image = self.generate_texture(prompt, output_file)
                results.append((prompt, output_file, True))
            except Exception as e:
                print(f">> ERROR generating {output_file}: {e}")
                results.append((prompt, output_file, False))
        
        total_time = time.time() - total_start
        successful = sum(1 for _, _, success in results if success)
        print(f"\n>> Batch complete: {successful}/{len(prompts_and_files)} successful in {total_time:.2f}s")
        
        return results

def main():
    if len(sys.argv) < 2:
        print("Usage:")
        print("  python sdxl_spaceship_generator.py single \"prompt\" [output_file]")
        print("  python sdxl_spaceship_generator.py batch \"prompt1\" \"prompt2\" ...")
        sys.exit(1)
    
    server = SDXLSpaceshipTextureServer()
    
    mode = sys.argv[1]
    
    if mode == "single":
        if len(sys.argv) < 3:
            print("ERROR: Need prompt for single generation")
            sys.exit(1)
        
        prompt = sys.argv[2]
        output_file = sys.argv[3] if len(sys.argv) > 3 else "generated_spaceship_texture.png"
        
        try:
            server.generate_texture(prompt, output_file)
            print("SUCCESS: Spaceship texture generation complete")
        except Exception as e:
            print(f"ERROR: Failed to generate texture: {e}")
            sys.exit(1)
    
    elif mode == "batch":
        if len(sys.argv) < 3:
            print("ERROR: Need at least one prompt for batch generation")
            sys.exit(1)
        
        prompts = sys.argv[2:]
        prompts_and_files = []
        
        for i, prompt in enumerate(prompts):
            output_file = f"spaceship_texture_{i:03d}.png"
            prompts_and_files.append((prompt, output_file))
        
        try:
            results = server.generate_batch(prompts_and_files)
            successful = sum(1 for _, _, success in results if success)
            if successful == len(results):
                print("SUCCESS: All spaceship textures generated")
            else:
                print(f"PARTIAL: {successful}/{len(results)} textures generated")
                sys.exit(1)
        except Exception as e:
            print(f"ERROR: Batch generation failed: {e}")
            sys.exit(1)
    
    else:
        print(f"ERROR: Unknown mode '{mode}'")
        sys.exit(1)

if __name__ == "__main__":
    main() 