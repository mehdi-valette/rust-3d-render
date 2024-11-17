# rust-3d-render
3D renderer in Rust

The goal of this repostory is to go from only the basic operations to a 3D renderer.

I will try to recreate a basic version of the following components :
- draw a triangle
- fill a triangle
- sinus and cosinus functions: a precise version to generate a lookup table, and a quick version that uses the created lookup table.
- matrix operations: addition, multiplication and dot product.
- calculate the rotation and projection matrices, and the normal vectors
- rotate a simple 3D shape with perspective
- allow clipping of the 3D world to the edge of the view cone
- allow moving inside the 3D world
- add simple textures to the 3D shapes
