use bevy::prelude::*;
use bevy::math::DVec2;
use super::components::*;

pub const GRAVITY: DVec2 = DVec2::new(0.0, -9.800);

pub fn apply_gravity(time: Res<Time>, mut query: Query<(&mut Velocity, &Mass), With<RigidBody>>) {
    let dt = time.delta_secs_f64();
    for (mut velocity, mass) in query.iter_mut() {
        if mass.0 == 0.0 {
            continue;
        }
        velocity.0 += GRAVITY * dt;
    }
}

pub fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    let dt = time.delta_secs_f64();
    for (mut transform, velocity) in query.iter_mut() {
        let displacement = velocity.0 * dt;
        transform.translation.x += displacement.x as f32;
        transform.translation.y += displacement.y as f32;
    }
}

pub fn collision_system(
    mut query: Query<(Entity, &mut Transform, &mut Velocity, &Collider, &Mass, &Restitution)>,
) {
    let mut iter = query.iter_combinations_mut();
    while let Some([(_e1, mut t1, mut v1, c1, m1, r1), (_e2, mut t2, mut v2, c2, m2, r2)]) =
        iter.fetch_next()
    {
        match (c1, c2) {
            (Collider::Circle(r1_val), Collider::Circle(r2_val)) => {
                // Convert translation f32 to DVec2 for physics calcs
                let p1 = DVec2::new(t1.translation.x as f64, t1.translation.y as f64);
                let p2 = DVec2::new(t2.translation.x as f64, t2.translation.y as f64);
                
                let delta = p2 - p1;
                let distance_sq = delta.length_squared();
                let radius_sum = r1_val + r2_val;

                if distance_sq < radius_sum * radius_sum {
                    let distance = distance_sq.sqrt();
                    let normal = if distance > 0.0 {
                        delta / distance
                    } else {
                        DVec2::X
                    };

                    // Positional Correction
                    let penetration = radius_sum - distance;
                    let correction = normal * (penetration / 2.0);
                    
                    let c_vec3 = Vec3::new(correction.x as f32, correction.y as f32, 0.0);
                    t1.translation -= c_vec3;
                    t2.translation += c_vec3;

                    // Velocity Resolution
                    let rel_vel = v2.0 - v1.0;
                    let vel_along_normal = rel_vel.dot(normal);

                    if vel_along_normal > 0.0 {
                        continue;
                    }

                    let e = r1.0.min(r2.0);
                    let j = -(1.0 + e) * vel_along_normal;
                    let inv_mass1 = if m1.0 == 0.0 { 0.0 } else { 1.0 / m1.0 };
                    let inv_mass2 = if m2.0 == 0.0 { 0.0 } else { 1.0 / m2.0 };
                    
                    if inv_mass1 + inv_mass2 == 0.0 {
                         continue;
                    }
                    
                    let impulse = j / (inv_mass1 + inv_mass2);

                    let impulse_vec = normal * impulse;
                    v1.0 -= impulse_vec * inv_mass1;
                    v2.0 += impulse_vec * inv_mass2;
                }
            }
        }
    }
}
