# rust-3d-render
3D renderer in Rust

The program should use only the basic mathematical operations and drawing one pixel at a time on the screen to recreate a 3D renderer.
The goal is to get a better comprehension of CGI (Computer Generated Imagery).

I will try to recreate a basic version of the following components :
- draw and fill a triangle
- sinus and cosinus functions: a precise version to generate a lookup table, and a quick version that uses the created lookup table.
- matrix operations: addition, multiplication and dot product.
- calculate the rotation and projection matrices, and the normal vectors
- rotate a simple 3D shape with perspective
- allow clipping of the 3D world to the edge of the view cone
- allow moving inside the 3D world
- add simple textures to the 3D shapes
- use multithreading
- use GPU? (I've heard it can get very complicated, plus it doesn't help to understand the maths. But it could be worth trying.)
