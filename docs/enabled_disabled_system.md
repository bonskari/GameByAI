# Enabled/Disabled System Documentation

## Overview

The GameByAI ECS now includes a comprehensive enabled/disabled system that provides fine-grained control over entity and component activity. This system operates at two levels:

1. **Entity Level**: Control whether an entire entity is active
2. **Component Level**: Control whether individual components are active

## Entity-Level Control

### Enabled Component

The `Enabled` component controls whether an entire entity is active in the game world.

```rust
use crate::ecs::{World, Enabled};

// Create an entity that's enabled by default
let entity = world.spawn()
    .with(Transform::new(Vec3::new(0.0, 0.0, 0.0)))
    .with(StaticRenderer::wall("wall.png".to_string()))
    .build();

// Entity is enabled by default (no Enabled component = enabled)

// Explicitly add Enabled component
world.add(entity, Enabled::new()); // enabled by default

// Or create disabled
world.add(entity, Enabled::disabled());
```

### World Helper Methods

The `World` provides convenient methods for entity-level control:

```rust
// Enable an entity
world.enable_entity(entity);

// Disable an entity  
world.disable_entity(entity);

// Toggle entity state
world.toggle_entity(entity);

// Check if entity is enabled
let is_enabled = world.is_entity_enabled(entity);

// Check if systems should process this entity's components
let should_process = world.should_process_component::<Transform>(entity);
```

## Component-Level Control

### All Components Have Enabled Fields

Every component now has an `enabled: bool` field with consistent methods:

```rust
// Create components with enabled state
let transform = Transform::new(Vec3::ZERO)
    .with_enabled(true);

let renderer = StaticRenderer::wall("texture.png".to_string())
    .with_enabled(false); // Start disabled

let collider = Collider::static_solid(ColliderShape::Box { size: Vec3::ONE })
    .with_enabled(true);

// Enable/disable components after creation
if let Some(mut transform) = world.get_mut::<Transform>(entity) {
    transform.disable();
    // or
    transform.enable();
    // or check
    if transform.is_enabled() {
        // component is active
    }
}
```

### Component Trait Methods

All components implement consistent enabled/disabled methods via the `Component` trait:

```rust
pub trait Component: 'static + Send + Sync {
    fn is_enabled(&self) -> bool { true }
    fn enable(&mut self) { }
    fn disable(&mut self) { }
}
```

Components with enabled fields override these methods:

```rust
// Works with any component type
if let Some(mut component) = world.get_mut::<SomeComponent>(entity) {
    component.enable();   // Via trait method
    component.disable();  // Via trait method
    let enabled = component.is_enabled(); // Via trait method
}
```

## System Integration

### PathfindingSystem Example

The pathfinding system respects both entity and component enabled states:

```rust
// In PathfindingSystem::process_entity_pathfinding
pub fn process_entity_pathfinding(&mut self, world: &mut World, entity: Entity, delta_time: f32) {
    // Check if entity is enabled
    if !world.is_entity_enabled(entity) {
        return; // Skip disabled entities
    }

    // Check if transform component is enabled
    if let Some(transform) = world.get::<Transform>(entity) {
        if !transform.is_enabled() {
            return; // Skip if transform is disabled
        }
        // Use transform...
    }

    // Check if pathfinder component is enabled
    if let Some(pathfinder) = world.get::<Pathfinder>(entity) {
        if !pathfinder.is_enabled() {
            return; // Skip if pathfinder is disabled
        }
        // Process pathfinding...
    }
}
```

### Rendering System Integration

The `StaticRenderer` has special rendering logic:

```rust
impl StaticRenderer {
    /// Check if this renderer should actually render (both enabled and visible)
    pub fn should_render(&self) -> bool {
        self.enabled && self.visible
    }
}

// In rendering system
for (entity, transform, renderer) in world.query_2::<Transform, StaticRenderer>() {
    // Check entity enabled state
    if !world.is_entity_enabled(entity) {
        continue; // Skip disabled entities
    }
    
    // Check if renderer should render
    if !renderer.should_render() {
        continue; // Skip if renderer disabled or invisible
    }
    
    // Render the entity...
}
```

## Use Cases

### 1. Temporarily Disable Entities

```rust
// Disable an enemy during a cutscene
world.disable_entity(enemy_entity);

// Re-enable after cutscene
world.enable_entity(enemy_entity);
```

### 2. Component-Level Control

```rust
// Disable collision but keep rendering
if let Some(mut collider) = world.get_mut::<Collider>(entity) {
    collider.disable();
}

// Disable rendering but keep physics
if let Some(mut renderer) = world.get_mut::<StaticRenderer>(entity) {
    renderer.disable();
}
```

### 3. Debug Mode Toggle

```rust
// Toggle pathfinding visualization
for (entity, mut pathfinder) in world.query_1_mut::<Pathfinder>() {
    if debug_mode {
        pathfinder.enable();
    } else {
        pathfinder.disable();
    }
}
```

### 4. Dynamic Level Loading

```rust
// Disable all entities in a level section
for entity in level_section_entities {
    world.disable_entity(entity);
}

// Enable when player approaches
for entity in level_section_entities {
    world.enable_entity(entity);
}
```

## Performance Benefits

### System Optimization

Systems can early-exit for disabled entities/components:

```rust
pub fn update_physics(&mut self, world: &mut World, delta_time: f32) {
    for (entity, transform, velocity) in world.query_2::<Transform, Velocity>() {
        // Quick entity check
        if !world.is_entity_enabled(entity) {
            continue;
        }
        
        // Component-level checks
        if !transform.is_enabled() || !velocity.is_enabled() {
            continue;
        }
        
        // Expensive physics calculations only for active entities
        apply_physics(transform, velocity, delta_time);
    }
}
```

### Memory Efficiency

- No additional allocations for disabled entities
- Components remain in memory but are skipped during processing
- Efficient query filtering

## Best Practices

### 1. Entity vs Component Level

- **Entity Level**: Use for major state changes (object activation/deactivation)
- **Component Level**: Use for fine-grained control (disable rendering, collision, etc.)

### 2. System Design

```rust
// Good: Check entity enabled state early
if !world.is_entity_enabled(entity) {
    continue;
}

// Good: Check component enabled state
if !component.is_enabled() {
    continue;
}

// Good: Use should_process_component for common patterns
if !world.should_process_component::<Transform>(entity) {
    continue;
}
```

### 3. Default Behavior

- All components default to `enabled: true`
- Entities without `Enabled` component are considered enabled
- This ensures backward compatibility

### 4. Component Creation

```rust
// Use builder pattern for clean component creation
let component = SomeComponent::new()
    .with_enabled(false)
    .with_other_property(value);

// Or create and modify
let mut component = SomeComponent::new();
component.disable();
```

## Examples

### Example 1: Toggle Enemy AI

```rust
fn toggle_enemy_ai(&mut self, world: &mut World, enable: bool) {
    for (entity, mut pathfinder) in world.query_1_mut::<Pathfinder>() {
        // Only affect entities that also have an Enemy component
        if world.has::<Enemy>(entity) {
            if enable {
                pathfinder.enable();
            } else {
                pathfinder.disable();
            }
        }
    }
}
```

### Example 2: Level Streaming

```rust
struct LevelSection {
    entities: Vec<Entity>,
    enabled: bool,
}

impl LevelSection {
    fn set_enabled(&mut self, world: &mut World, enabled: bool) {
        self.enabled = enabled;
        for &entity in &self.entities {
            if enabled {
                world.enable_entity(entity);
            } else {
                world.disable_entity(entity);
            }
        }
    }
}
```

### Example 3: Debug Visualization

```rust
fn toggle_debug_visualization(&mut self, world: &mut World, show_debug: bool) {
    // Toggle pathfinding visualization
    for (entity, mut pathfinder) in world.query_1_mut::<Pathfinder>() {
        pathfinder.enabled = show_debug;
    }
    
    // Toggle collision box rendering
    for (entity, mut collider) in world.query_1_mut::<Collider>() {
        if world.has::<DebugRenderer>(entity) {
            if let Some(mut debug_renderer) = world.get_mut::<DebugRenderer>(entity) {
                if show_debug {
                    debug_renderer.enable();
                } else {
                    debug_renderer.disable();
                }
            }
        }
    }
}
```

This enabled/disabled system provides powerful control over your ECS entities and components while maintaining excellent performance through early filtering and consistent APIs. 