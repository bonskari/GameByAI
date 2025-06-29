#!/usr/bin/env python3
"""
SDXL PBR Texture Generator for Spaceship Materials
Generates high-quality base color, normal, and metallic-roughness maps
"""

import sys
import time
from pathlib import Path

try:
    from diffusers import StableDiffusionXLPipeline, EulerAncestralDiscreteScheduler
    import torch
    from PIL import Image, ImageFilter
    import numpy as np
    import gc
except ImportError as e:
    print(f"ERROR: Missing dependencies: {e}")
    print("Install with: pip install diffusers torch transformers accelerate pillow xformers numpy")
    sys.exit(1)

class SDXLPBRGenerator:
    def __init__(self):
        self.pipe = None
        self.model_loaded = False
        self.generation_count = 0
        
    def load_model(self):
        """Load SDXL model with maximum optimizations"""
        if self.model_loaded:
            return
            
        print(">> Loading SDXL model for PBR generation...")
        start_time = time.time()
        
        self.pipe = StableDiffusionXLPipeline.from_pretrained(
            "stabilityai/stable-diffusion-xl-base-1.0",
            torch_dtype=torch.float16,
            use_safetensors=True,
            variant="fp16"
        )
        
        if torch.cuda.is_available():
            gpu_name = torch.cuda.get_device_name(0)
            gpu_memory = torch.cuda.get_device_properties(0).total_memory / 1024**3
            print(f">> Detected GPU: {gpu_name} ({gpu_memory:.1f}GB VRAM)")
            
            self.pipe = self.pipe.to("cuda")
            self.pipe.scheduler = EulerAncestralDiscreteScheduler.from_config(
                self.pipe.scheduler.config
            )
            
            try:
                self.pipe.enable_xformers_memory_efficient_attention()
                print(">> Enabled xformers memory optimization")
            except ImportError:
                print(">> xformers not available, using standard attention")
            
            print(">> Warming up pipeline...")
            with torch.inference_mode():
                warmup_image = self.pipe(
                    "test", num_inference_steps=1, guidance_scale=1.0,
                    width=512, height=512,
                ).images[0]
            del warmup_image
            torch.cuda.empty_cache()
        else:
            print(">> WARNING: CUDA not available, using CPU (will be slow)")
            
        load_time = time.time() - start_time
        print(f">> Model loaded in {load_time:.1f}s")
        self.model_loaded = True
    
    def generate_base_color(self, material_type, output_file=None):
        """Generate base color (albedo) texture"""
        if not self.model_loaded:
            self.load_model()
        
        start_time = time.time()
        self.generation_count += 1
        
        material_prompts = {
            "tech_panel": "futuristic spaceship tech panel, metallic blue panels, glowing circuits, clean geometric design, seamless tileable texture, high tech interface, brushed metal finish",
            "hull_plating": "spaceship hull plating, weathered grey metal plates, rivets, industrial design, seamless tileable texture, worn steel panels, space vessel exterior",
            "control_system": "spaceship control system interface, metallic orange accent panels, digital displays, buttons and switches, seamless tileable texture, command center design",
            "energy_conduit": "spaceship energy conduit, metallic green power channels, glowing energy lines, technical patterns, seamless tileable texture, power distribution system",
            "floor": "spaceship floor plating, dark grey metal grating, anti-slip texture, industrial flooring, seamless tileable texture, worn metal walkway",
            "ceiling": "spaceship ceiling panels, clean white metal tiles, ventilation grilles, overhead lighting strips, seamless tileable texture, sterile interior design"
        }
        
        prompt = material_prompts.get(material_type, material_prompts["hull_plating"])
        print(f">> Generation #{self.generation_count}: {material_type} base color")
        
        enhanced_prompt = f"{prompt}, photorealistic PBR texture, 4K quality, seamless tiling, clean surface, no shadows, diffuse lighting"
        negative_prompt = "pixel art, cartoon, illustration, painting, drawing, sketch, low quality, blurry, noisy, artifacts, text, watermark, logo, signature"
        
        with torch.inference_mode():
            image = self.pipe(
                enhanced_prompt,
                negative_prompt=negative_prompt,
                num_inference_steps=30,
                guidance_scale=7.5,
                width=512, height=512,
                generator=torch.Generator(device="cuda").manual_seed(42 + self.generation_count) if torch.cuda.is_available() else None
            ).images[0]
        
        if torch.cuda.is_available():
            torch.cuda.empty_cache()
        gc.collect()
        
        if output_file:
            image.save(output_file)
            print(f">> Saved base color texture: {output_file}")
        
        gen_time = time.time() - start_time
        print(f">> Generated base color texture in {gen_time:.2f}s")
        return image
    
    def generate_normal_map(self, base_color_image, output_file=None):
        """Generate normal map from base color texture"""
        print(">> Generating normal map from base color...")
        
        height_map = base_color_image.convert('L')
        height_map = height_map.filter(ImageFilter.GaussianBlur(radius=0.5))
        height_array = np.array(height_map, dtype=np.float32) / 255.0
        
        height, width = height_array.shape
        padded = np.pad(height_array, 1, mode='edge')
        
        dx = (padded[1:-1, 2:] - padded[1:-1, :-2]) * 0.5
        dy = (padded[2:, 1:-1] - padded[:-2, 1:-1]) * 0.5
        
        strength = 3.0
        dz = 1.0 / strength
        
        length = np.sqrt(dx*dx + dy*dy + dz*dz)
        nx = dx / length
        ny = dy / length
        nz = dz / length
        
        normal_r = ((nx + 1.0) * 127.5).astype(np.uint8)
        normal_g = ((ny + 1.0) * 127.5).astype(np.uint8)
        normal_b = ((nz + 1.0) * 127.5).astype(np.uint8)
        
        normal_map = Image.fromarray(np.stack([normal_r, normal_g, normal_b], axis=2))
        
        if output_file:
            normal_map.save(output_file)
            print(f">> Saved normal map: {output_file}")
        
        return normal_map
    
    def generate_metallic_roughness(self, material_type, output_file=None):
        """Generate metallic-roughness map"""
        print(f">> Generating metallic-roughness map for {material_type}...")
        
        material_properties = {
            "tech_panel": {"metallic": 0.8, "roughness": 0.3},
            "hull_plating": {"metallic": 0.9, "roughness": 0.4},
            "control_system": {"metallic": 0.7, "roughness": 0.2},
            "energy_conduit": {"metallic": 0.8, "roughness": 0.3},
            "floor": {"metallic": 0.6, "roughness": 0.7},
            "ceiling": {"metallic": 0.9, "roughness": 0.1}
        }
        
        props = material_properties.get(material_type, {"metallic": 0.8, "roughness": 0.4})
        
        width, height = 512, 512
        metallic_base = int(props["metallic"] * 255)
        roughness_base = int(props["roughness"] * 255)
        
        metallic_array = np.full((height, width), metallic_base, dtype=np.uint8)
        roughness_array = np.full((height, width), roughness_base, dtype=np.uint8)
        
        noise_scale = 10
        metallic_noise = np.random.randint(-noise_scale, noise_scale, (height, width))
        roughness_noise = np.random.randint(-noise_scale, noise_scale, (height, width))
        
        metallic_array = np.clip(metallic_array + metallic_noise, 0, 255).astype(np.uint8)
        roughness_array = np.clip(roughness_array + roughness_noise, 0, 255).astype(np.uint8)
        blue_array = np.zeros((height, width), dtype=np.uint8)
        
        metallic_roughness = Image.fromarray(np.stack([metallic_array, roughness_array, blue_array], axis=2))
        
        if output_file:
            metallic_roughness.save(output_file)
            print(f">> Saved metallic-roughness map: {output_file}")
        
        return metallic_roughness
    
    def generate_pbr_set(self, material_type, output_dir="assets/textures"):
        """Generate complete PBR texture set for a material"""
        output_path = Path(output_dir)
        output_path.mkdir(exist_ok=True)
        
        print(f"\n>> Generating PBR texture set for: {material_type}")
        
        base_color_file = output_path / f"{material_type}_BaseColor.png"
        base_color = self.generate_base_color(material_type, base_color_file)
        
        normal_file = output_path / f"{material_type}_Normal.png"
        normal_map = self.generate_normal_map(base_color, normal_file)
        
        metallic_roughness_file = output_path / f"{material_type}_MetallicRoughness.png"
        metallic_roughness = self.generate_metallic_roughness(material_type, metallic_roughness_file)
        
        print(f">> PBR set complete for {material_type}:")
        print(f"   - Base Color: {base_color_file}")
        print(f"   - Normal: {normal_file}")
        print(f"   - Metallic-Roughness: {metallic_roughness_file}")
        
        return {
            "base_color": base_color_file,
            "normal": normal_file,
            "metallic_roughness": metallic_roughness_file
        }

def main():
    if len(sys.argv) < 2:
        print("Usage:")
        print("  python sdxl_pbr_generator.py single <material_type>")
        print("  python sdxl_pbr_generator.py batch")
        print("\nMaterial types: tech_panel, hull_plating, control_system, energy_conduit, floor, ceiling")
        sys.exit(1)
    
    generator = SDXLPBRGenerator()
    mode = sys.argv[1]
    
    if mode == "single":
        if len(sys.argv) < 3:
            print("ERROR: Need material type for single generation")
            sys.exit(1)
        
        material_type = sys.argv[2]
        try:
            result = generator.generate_pbr_set(material_type)
            print("SUCCESS: PBR texture set generated")
        except Exception as e:
            print(f"ERROR: Failed to generate PBR textures: {e}")
            sys.exit(1)
    
    elif mode == "batch":
        materials = ["tech_panel", "hull_plating", "control_system", "energy_conduit", "floor", "ceiling"]
        
        print(f">> Generating PBR sets for {len(materials)} materials...")
        
        try:
            results = {}
            for material in materials:
                results[material] = generator.generate_pbr_set(material)
            
            print("\n>> Batch generation complete!")
            print(">> Generated PBR sets:")
            for material, files in results.items():
                print(f"   {material}:")
                for map_type, file_path in files.items():
                    print(f"     - {map_type}: {file_path}")
            
        except Exception as e:
            print(f"ERROR: Batch generation failed: {e}")
            sys.exit(1)
    
    else:
        print(f"ERROR: Unknown mode '{mode}'")
        sys.exit(1)

if __name__ == "__main__":
    main() 