# Zysics

Zysics is a lightweight, custom 2D physics engine sandbox built using Rust and the Bevy game engine. It serves as a playground for exploring rigid body dynamics, collision resolution, and Entity-Component-System (ECS) architecture.
## Key Features
ECS-Based Architecture: Physics logic is decoupled into discrete systems (Gravity, Velocity, Collisions) utilizing Bevy's ECS.
Rigid Body Dynamics: accurate simulation of velocity, mass, and restitution (bounciness).
Double Precision: Uses DVec2 (f64) for physics calculations to minimize floating-point errors, mapped to Vec3 for rendering.
Impulse-Based Collision Resolution: Handles velocity changes and positional correction to prevent sinking objects.
Interactive Sandbox: Real-time object spawning with mouse input.

## Getting Started
### Prerequisites:
Rust & Cargo: Ensure you have the latest nightly version of Rust installed. You can install it via rustup.rs. (last test: 1.94 nightly)

### Installation & Running
* Clone the repository:
  
  `git clone https://github.com/yourusername/zysics.git`

  `cd zysics`
* Run the project:
  
  `cargo run`

(Note: The first build may take a moment to compile Bevy dependencies. took about 45 seconds on my 265k)

### Controls
Left Mouse Click: Spawn a new physics circle with random colors at the cursor location.
(Note: The floor is currently simulated as a static, infinite-mass object. moves if to many objects are on it trying to fix this atm)

### Technical Overview

The engine currently implements the following physics pipeline:

* Gravity System: Applies constant downward acceleration to all dynamic bodies.
* Velocity System: updates positions based on current velocities (Explicit Euler integration).
* Collision System:
    * Detection: Checks for overlapping radii between Circle colliders.
    * Resolution: Calculates the collision normal and applies an impulse based on the coefficient of restitution and the relative mass of the objects.
    * Positional Correction: Separates overlapping objects to prevent "sinking" or instability.

### Future Goals

This project is under active development. The roadmap focuses on stability, features, and accuracy:

  * Expanded Collider Support:

      Implement AABB (Axis-Aligned Bounding Box) and OBB (Oriented Bounding Box) for rectangle collisions.

      Add support for arbitrary polygons.

      Implement broad-phase collision detection (e.g., Quadtrees or Spatial Hashing) to optimize performance as the object count grows.

  * Improved Integrator:

      Move from the current Explicit Euler method to Verlet or RK4 (Runge-Kutta) integration. This will significantly improve energy conservation and simulation stability at lower frame rates.

  * Enhanced Interactivity:

      Add "Mouse Joints" to allow dragging and throwing objects with the cursor.

      UI controls to adjust gravity, restitution, and time scale in real-time.

      Visual debugging (rendering velocity vectors and collision normals).
    
![ezgif com-video-to-gif-converter](https://github.com/user-attachments/assets/d3825437-035d-4930-84c0-09e8b5aa128a)

