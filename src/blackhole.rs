use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use std::f32::consts::PI;

pub struct BlackHolePlugin;

impl Plugin for BlackHolePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_blackhole)
            .add_systems(Update, (
                animate_accretion_disk,
                update_grid_warping,
                animate_particles,
                update_particle_temperatures,
                camera_controller,
            ).chain());
    }
}

#[derive(Component)]
pub struct BlackHole {
    pub mass: f32,
    pub schwarzschild_radius: f32,
    pub spin: f32,
}

#[derive(Component)]
pub struct AccretionParticle {
    pub orbital_radius: f32,
    pub angular_velocity: f32,
    pub phase: f32,
    pub temperature: f32,
    pub velocity: Vec3,
    pub last_stable_orbit: f32,
}

#[derive(Component)]
pub struct WarpGrid {
    pub size: usize,
    pub spacing: f32,
}

#[derive(Component)]
pub struct GridMesh;

#[derive(Component)]
pub struct ParticleMaterial;

#[derive(Component)]
pub struct CameraController {
    pub distance: f32,
    pub azimuth: f32,
    pub elevation: f32,
    pub target: Vec3,
    pub auto_rotate: bool,
}

fn setup_blackhole(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const BLACK_HOLE_MASS_SOLAR: f32 = 10.0;
    let rs_scaled = 1.0;

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 8.0, 12.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        CameraController {
            distance: 15.0,
            azimuth: 0.0,
            elevation: 0.3,
            target: Vec3::ZERO,
            auto_rotate: true,
        },
    ));

    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            color: Color::srgb(0.8, 0.9, 1.0),
            illuminance: 100.0,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -PI / 3.0,
            PI / 6.0,
            0.0,
        )),
    ));

    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.05, 0.05, 0.1),
        brightness: 10.0,
        affects_lightmapped_meshes: false,
    });

    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::BLACK,
            emissive: Color::BLACK.into(),
            metallic: 0.0,
            reflectance: 0.0,
            ..default()
        })),
        Transform::from_scale(Vec3::splat(rs_scaled)),
        BlackHole {
            mass: BLACK_HOLE_MASS_SOLAR,
            schwarzschild_radius: rs_scaled,
            spin: 0.7,
        },
    ));

    let grid_mesh_handle = meshes.add(create_warp_grid_mesh(30, 1.0));
    commands.spawn((
        Mesh3d(grid_mesh_handle.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba(0.3, 0.6, 1.0, 0.3),
            emissive: Color::srgb(0.1, 0.2, 0.4).into(),
            unlit: true,
            alpha_mode: AlphaMode::Blend,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        WarpGrid {
            size: 30,
            spacing: 1.0,
        },
        GridMesh,
    ));

    let last_stable_orbit = 3.0 * rs_scaled;
    let outer_disk_radius = 50.0 * rs_scaled;

    for i in 0..500 {
        let progress = (i as f32) / 499.0;
        let radius = last_stable_orbit + (outer_disk_radius - last_stable_orbit) * (progress * progress * progress);
        let phase = (i as f32) * 0.1257;

        let orbital_velocity = (BLACK_HOLE_MASS_SOLAR / radius).sqrt();
        let angular_velocity = orbital_velocity / radius;

        let base_temp = 10000.0 * (BLACK_HOLE_MASS_SOLAR / radius).powf(0.25);
        let temperature = base_temp * (0.5 + 0.5 * fastrand::f32());

        let particle_size = 0.01 * (radius / last_stable_orbit).sqrt().min(3.0);
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(particle_size))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: temperature_to_color(temperature),
                emissive: temperature_to_color(temperature).into(),
                metallic: 0.0,
                ..default()
            })),
            Transform::from_translation(Vec3::new(
                radius * phase.cos(),
                0.0,
                radius * phase.sin(),
            )),
            AccretionParticle {
                orbital_radius: radius,
                angular_velocity,
                phase,
                temperature,
                velocity: Vec3::new(-orbital_velocity * phase.sin(), 0.0, orbital_velocity * phase.cos()),
                last_stable_orbit,
            },
            ParticleMaterial,
        ));
    }

    for _ in 0..200 {
        let distance = 100.0 + fastrand::f32() * 200.0;
        let theta = fastrand::f32() * 2.0 * PI;
        let phi = fastrand::f32() * PI;

        let position = Vec3::new(
            distance * phi.sin() * theta.cos(),
            distance * phi.cos(),
            distance * phi.sin() * theta.sin(),
        );

        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.1))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::WHITE,
                emissive: Color::srgb(0.8, 0.9, 1.0).into(),
                unlit: true,
                ..default()
            })),
            Transform::from_translation(position),
        ));
    }
}

fn temperature_to_color(temp: f32) -> Color {
    let t = temp.clamp(1000.0, 50000.0);

    if t < 3000.0 {
        // Red hot
        Color::srgb(1.0, 0.1 + 0.3 * (t - 1000.0) / 2000.0, 0.0)
    } else if t < 5000.0 {
        // Orange to yellow
        let progress = (t - 3000.0) / 2000.0;
        Color::srgb(1.0, 0.4 + 0.4 * progress, progress * 0.2)
    } else if t < 10000.0 {
        // Yellow to white
        let progress = (t - 5000.0) / 5000.0;
        Color::srgb(1.0, 0.8 + 0.2 * progress, 0.2 + 0.6 * progress)
    } else {
        // Blue-white to blue
        let progress = ((t - 10000.0) / 40000.0).min(1.0);
        Color::srgb(1.0 - 0.2 * progress, 0.9 - 0.1 * progress, 1.0)
    }
}

fn create_warp_grid_mesh(size: usize, spacing: f32) -> Mesh {
    let mut positions = Vec::new();
    let mut indices = Vec::new();

    for i in 0..=size {
        for j in 0..=size {
            let x = (i as f32 - size as f32 / 2.0) * spacing;
            let z = (j as f32 - size as f32 / 2.0) * spacing;
            positions.push([x, 0.0, z]);
        }
    }

    for i in 0..=size {
        for j in 0..size {
            let current = i * (size + 1) + j;
            let next = current + 1;
            indices.extend_from_slice(&[current as u32, next as u32]);
        }
    }

    for i in 0..size {
        for j in 0..=size {
            let current = i * (size + 1) + j;
            let next = current + size + 1;
            indices.extend_from_slice(&[current as u32, next as u32]);
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::LineList, default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![[0.0, 1.0, 0.0]; (size + 1) * (size + 1)],
    );
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

fn animate_accretion_disk(
    time: Res<Time>,
    black_hole_query: Query<&BlackHole>,
    mut particle_query: Query<(&mut Transform, &mut AccretionParticle)>,
) {
    if let Ok(black_hole) = black_hole_query.single() {
        for (mut transform, mut particle) in particle_query.iter_mut() {
            let r = particle.orbital_radius;
            let rs = black_hole.schwarzschild_radius;

            // Time dilation factor: sqrt(1 - rs/r)
            let time_dilation = if r > rs * 1.1 {
                (1.0 - rs / r).sqrt()
            } else {
                0.1
            };

            let relativistic_ang_vel = particle.angular_velocity * time_dilation;
            particle.phase += relativistic_ang_vel * time.delta_secs();

            let x = r * particle.phase.cos();
            let z = r * particle.phase.sin();

            let precession = black_hole.spin * 0.05 * time.elapsed_secs() * time_dilation / r;            let y = 0.05 * (particle.phase * 5.0 + precession).sin() * (rs / r);

            transform.translation = Vec3::new(x, y, z);

            if r > black_hole.schwarzschild_radius * 1.01 {
                particle.orbital_radius -= 0.01 * time.delta_secs() * time_dilation * (rs / r).powf(2.0);            }

            if particle.orbital_radius < black_hole.schwarzschild_radius * 1.01 {
                particle.orbital_radius = particle.last_stable_orbit + fastrand::f32() * 20.0;
                let spawn_dilation = (1.0 - rs / particle.orbital_radius).sqrt();
                particle.angular_velocity *= spawn_dilation;
                particle.phase = fastrand::f32() * 2.0 * PI;
            }
        }
    }
}

fn update_grid_warping(
    black_hole_query: Query<&BlackHole>,
    mut meshes: ResMut<Assets<Mesh>>,
    grid_query: Query<&Mesh3d, With<GridMesh>>,
) {
    if let Ok(black_hole) = black_hole_query.single() {
        for mesh3d in grid_query.iter() {
            if let Some(mesh) = meshes.get_mut(&mesh3d.0) {
                if let Some(positions) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
                    if let bevy::render::mesh::VertexAttributeValues::Float32x3(positions_vec) = positions {
                        for pos in positions_vec.iter_mut() {
                            let x = pos[0];
                            let z = pos[2];
                            let r = (x * x + z * z).sqrt();

                            if r > 0.1 {
                                let rs = black_hole.schwarzschild_radius;
                                let curvature = -(rs / r) * (1.0 + rs / (4.0 * r)).exp();
                                pos[1] = curvature * 2.0;
                            } else {
                                pos[1] = -10.0;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn animate_particles(
    mut materials: ResMut<Assets<StandardMaterial>>,
    camera_query: Query<&Transform, With<Camera3d>>,
    query: Query<(&Transform, &MeshMaterial3d<StandardMaterial>, &AccretionParticle), With<ParticleMaterial>>,
) {
    const C: f32 = 2.998e8;

    if let Ok(camera_transform) = camera_query.single() {
        let camera_pos = camera_transform.translation;

        for (particle_transform, material3d, particle) in query.iter() {
            if let Some(material) = materials.get_mut(&material3d.0) {
                let particle_pos = particle_transform.translation;

                let observer_dir = (camera_pos - particle_pos).normalize();
                let velocity_dir = particle.velocity.normalize();

                let cos_theta = velocity_dir.dot(observer_dir);
                let v = particle.velocity.length();
                let fake_c_scale = 0.1;
                let beta = ((v / C) / fake_c_scale).clamp(0.0, 0.9999);

                let gamma_factor = (1.0 - beta * beta).sqrt();
                let doppler_factor = gamma_factor / (1.0 - beta * cos_theta);

                let shifted_temp = particle.temperature * doppler_factor;
                let base_color = temperature_to_color(shifted_temp);

                let beaming_factor = doppler_factor.powf(3.0);
                let intensity = 0.5 + beaming_factor.clamp(0.1, 5.0);

                let rgb: Vec3 = base_color.to_linear().to_vec3() * intensity;
                material.emissive = Color::linear_rgb(rgb.x, rgb.y, rgb.z).into();
            }
        }
    }
}


fn update_particle_temperatures(
    black_hole_query: Query<&BlackHole>,
    mut particle_query: Query<&mut AccretionParticle>,
) {
    if let Ok(black_hole) = black_hole_query.single() {
        for mut particle in particle_query.iter_mut() {
            // Temperature increases as particles spiral inward due to compression and friction
            let r = particle.orbital_radius;
            let rs = black_hole.schwarzschild_radius;

            // Base temperature from gravitational potential
            let base_temp = 10000.0 * (black_hole.mass / r).powf(0.25);

            // Additional heating from magnetic reconnection and turbulence
            let magnetic_heating = 5000.0 * (rs / r).powf(0.5);

            // Tidal heating
            let tidal_heating = 2000.0 * (rs / r).powf(1.5);
            particle.temperature = base_temp + magnetic_heating + tidal_heating;
        }
    }
}

fn camera_controller(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<(&mut Transform, &mut CameraController), With<Camera3d>>,
) {
    for (mut transform, mut controller) in camera_query.iter_mut() {
        let movement = Vec3::ZERO;
        let mut rotation_changed = false;

        if input.pressed(KeyCode::KeyW) {
            controller.distance = (controller.distance - 5.0 * time.delta_secs()).max(2.0);
        }
        if input.pressed(KeyCode::KeyS) {
            controller.distance += 5.0 * time.delta_secs();
        }
        if input.pressed(KeyCode::KeyA) {
            controller.azimuth -= 1.0 * time.delta_secs();
            rotation_changed = true;
        }
        if input.pressed(KeyCode::KeyD) {
            controller.azimuth += 1.0 * time.delta_secs();
            rotation_changed = true;
        }
        if input.pressed(KeyCode::KeyQ) {
            controller.elevation = (controller.elevation - 1.0 * time.delta_secs()).max(-1.5);
            rotation_changed = true;
        }
        if input.pressed(KeyCode::KeyE) {
            controller.elevation = (controller.elevation + 1.0 * time.delta_secs()).min(1.5);
            rotation_changed = true;
        }

        if input.just_pressed(KeyCode::Space) {
            controller.auto_rotate = !controller.auto_rotate;
        }

        if controller.auto_rotate {
            controller.azimuth += 0.2 * time.delta_secs();
            rotation_changed = true;
        }

        if rotation_changed || movement != Vec3::ZERO {
            let x = controller.distance * controller.elevation.cos() * controller.azimuth.cos();
            let y = controller.distance * controller.elevation.sin();
            let z = controller.distance * controller.elevation.cos() * controller.azimuth.sin();

            transform.translation = controller.target + Vec3::new(x, y, z);
            transform.look_at(controller.target, Vec3::Y);
        }
    }
}
