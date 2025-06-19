#!/usr/bin/env python3
"""
Persistent SDXL Texture Generation Server
Keeps model loaded in memory for ultra-fast generation
"""

import sys
import json
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

class SDXLTextureServer:
    def __init__(self):
        self.pipe = None
        self.model_loaded = False
        self.generation_count = 0
        
    def load_model(self):
        """Load SDXL model with maximum optimizations"""
        if self.model_loaded:
            return
            
        print(">> Loading SDXL model with speed optimizations...")
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
            
            # Compile UNet for maximum speed (PyTorch 2.0+)
            try:
                # Try torch.compile only if triton is available
                import triton  # Test if triton is available
                self.pipe.unet = torch.compile(
                    self.pipe.unet, 
                    mode="reduce-overhead", 
                    fullgraph=True
                )
                print(">> Compiled UNet with torch.compile")
            except (ImportError, AttributeError):
                print(">> torch.compile/triton not available, using standard UNet (still fast)")
            
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
        """Generate a single texture"""
        if not self.model_loaded:
            self.load_model()
        
        start_time = time.time()
        self.generation_count += 1
        
        print(f">> Generation #{self.generation_count}: {prompt}")
        
        with torch.inference_mode():
            # Generate with optimized settings
            image = self.pipe(
                prompt,
                num_inference_steps=15,  # Reduced for speed
                guidance_scale=5.0,
                width=512,
                height=512,
                generator=torch.Generator(device="cuda").manual_seed(42) if torch.cuda.is_available() else None
            ).images[0]
        
        # Clean up GPU memory
        if torch.cuda.is_available():
            torch.cuda.empty_cache()
        gc.collect()
        
        # Save the image
        if output_file:
            image.save(output_file)
            print(f">> Saved: {output_file}")
        
        gen_time = time.time() - start_time
        print(f">> Generated texture in {gen_time:.2f}s")
        
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
        print("  python sdxl_server.py single \"prompt\" [output_file]")
        print("  python sdxl_server.py batch \"prompt1\" \"prompt2\" ...")
        print("  python sdxl_server.py server  # Start persistent server")
        sys.exit(1)
    
    server = SDXLTextureServer()
    
    mode = sys.argv[1]
    
    if mode == "single":
        if len(sys.argv) < 3:
            print("ERROR: Need prompt for single generation")
            sys.exit(1)
        
        prompt = sys.argv[2]
        output_file = sys.argv[3] if len(sys.argv) > 3 else "generated_texture.png"
        
        try:
            server.generate_texture(prompt, output_file)
            print("SUCCESS: Single texture generation complete")
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
            output_file = f"batch_texture_{i:03d}.png"
            prompts_and_files.append((prompt, output_file))
        
        try:
            results = server.generate_batch(prompts_and_files)
            successful = sum(1 for _, _, success in results if success)
            if successful == len(results):
                print("SUCCESS: All batch textures generated")
            else:
                print(f"PARTIAL: {successful}/{len(results)} textures generated")
                sys.exit(1)
        except Exception as e:
            print(f"ERROR: Batch generation failed: {e}")
            sys.exit(1)
    
    elif mode == "server":
        print(">> Starting persistent SDXL server...")
        print(">> Send JSON requests to stdin")
        server.load_model()
        
        # TODO: Implement persistent server mode
        print(">> Persistent server mode not yet implemented")
        sys.exit(1)
    
    else:
        print(f"ERROR: Unknown mode '{mode}'")
        sys.exit(1)

if __name__ == "__main__":
    main() 