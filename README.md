# Black Hole Simulation
> Real-time, interactive black hole visualization written in Rust

<img width="1401" height="735" alt="swappy-20250813-153940" src="https://github.com/user-attachments/assets/584c9aa1-2a83-4c5d-803d-d3070af5940e" />

## Overview
This project simulates an [accretion disk](https://en.wikipedia.org/wiki/Accretion_disk), particle heating, and stylized [spacetime curvature](https://en.wikipedia.org/wiki/Curved_spacetime) (grid warping) around the **black hole**, complete with camera controls.

## ✨ Features
- **Accretion Disk Animation**: Particles orbit around the black hole with relativistic time dilation effects.
- **Doppler Shift Rendering**: Colors shift based on relative motion using simplified relativistic formula.
- **Temperature-Driven Emission**: Particles emit light based on dynamic heating models (gravitational, magnetic, and tidal).
- **Spacetime Warping Grid**: A visible mesh is distorted according to Schwarzschild radius curvature.
- **Configurable Black Hole Parameters**: Mass, spin, and scale are adjustable in code.
- **Camera Controls**: Zoom, rotate, tilt, and toggle auto-rotation in real time.
- **Background stars**: Static star field for reference.

## Controls
- `W`: Zoom in
- `S`: Zoom out
- `A`: Rotate left
- `D`: Rotate right
- `Q`: Tilt down
- `E`: Tilt up
- `Space`: Toggle auto-rotation

## ⚙️ Installation
1. Clone the Repository:
```bash
git clone https://github.com/ecnivs/blackhole.git
cd blackhole
```
2. Run the simulation:
```bash
cargo run --release
```
## How It Works
- The black hole's [Schwarzschild radius](https://en.wikipedia.org/wiki/Schwarzschild_radius) is computed from its mass.
- [Accretion](https://en.wikipedia.org/wiki/Accretion_(astrophysics)) particles orbit the hole, with thier orbital radius decreasing over time.
- Relativistic [Doppler effect](https://en.wikipedia.org/wiki/Doppler_effect) alters their emission color depending on motion relative to camera.
- The grid mesh is dynamically warped to show spacetime curvature.
- A simple star field provides background reference.

## Possible Improvements
- [ ] Add [photon ring](https://en.wikipedia.org/wiki/Photon_sphere) and lensing visuals via [ray marching](https://en.wikipedia.org/wiki/Ray_marching)
- [ ] Implement [Kerr metric](https://en.wikipedia.org/wiki/Kerr_metric) spin-based light bending
- [ ] Better color grading for cinematic presentation
- [ ] UI silders for real-time parameter changes

## 🙌 Contributing
We appreciate any feedback or code reviews! Feel free to:
1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Submit a pull request

### I'd appreciate any feedback or code reviews you might have!
