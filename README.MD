# Ray Tracing in One Weekend

![4k ray-traced image](/milestones/image-003.png)

This project is an implementation of the "Ray Tracing in One Weekend" series by Peter Shirley. It serves as an introduction to ray tracing, a rendering technique for generating realistic images by simulating the way light interacts with objects.

## Features

- **Basic Ray Tracing**: Render spheres and planes with realistic lighting.
- **Materials**: Support for Lambertian (diffuse), metal, and dielectric (glass) materials.
- **Camera**: Adjustable camera with depth of field and focus distance.
- **Multithreading**: Faster rendering using parallel processing.


### Prerequisites
- Rust programming language (https://www.rust-lang.org/)

### Installation
1. Clone the repository:
    ```bash
    git clone https://github.com/your-username/raytracing-in-one-weekend.git
    cd raytracing-in-one-weekend
    ```
2. Build the project:
    ```bash
    cargo build --release
    ```

### Running the Renderer
Run the renderer with:
```bash
cargo run --release
```
The output image will be saved as `image[hash].jpg` in the project directory.

## References
- [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) by Peter Shirley


Happy ray tracing!