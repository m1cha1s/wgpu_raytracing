@group(0) @binding(0)
var texture: texture_storage_2d<rgba8unorm, write>;

struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>,
}

struct Sphere {
    center: vec3<f32>,
    radius: f32,
}

struct Spheres {
    sphere_count: u32,
    spheres: array<Sphere, 20>,
}

@group(0) @binding(1)
var<storage, read> spheres: Spheres;

struct HitRecord {
    p: vec3<f32>,
    normal: vec3<f32>,
    t: f32,
    hit: bool,
}

fn hash(value: u32) -> u32 {
    var state = value;
    state = state ^ 2747636419u;
    state = state * 2654435769u;
    state = state ^ state >> 16u;
    state = state * 2654435769u;
    state = state ^ state >> 16u;
    state = state * 2654435769u;
    return state;
}

fn randomFloat(value: u32) -> f32 {
    return f32(hash(value)) / 4294967295.0;
}

fn hitSphere(sphere: Sphere, ray: Ray, t_min: f32, t_max: f32) -> HitRecord {
    let oc = ray.origin - sphere.center;
    let a = dot(ray.direction, ray.direction);
    let half_b = dot(oc, ray.direction);
    let c = dot(oc, oc) - sphere.radius * sphere.radius;
    let discriminant = half_b * half_b - a * c;

    let miss = HitRecord(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), 0.0, false);

    if discriminant < 0.0 {
        return miss;
    }

    let sqrtd = sqrt(discriminant);


    var root = (-half_b - sqrtd) / a;
    if root < t_min || t_max < root {
        root = (-half_b + sqrtd) / a;
        if root < t_min || t_max < root {
            return miss;
        }
    }

    let outward_normal = ((ray.origin + (ray.direction * root)) - sphere.center) / sphere.radius;

    let front_face = dot(ray.direction, outward_normal) < 0.0;

    var normal = -outward_normal;
    if front_face {
        normal = outward_normal;
    }

    return HitRecord(ray.origin + (ray.direction * root), normal, root, true);
}

fn hitSpheres(ray: Ray, t_min: f32, t_max: f32) -> HitRecord {
    var temp_rec = HitRecord(vec3<f32>(0.0), vec3<f32>(0.0), 0.0, false);
    var closest_so_far = t_max;

    for (var sphere_idx: u32 = 0u; sphere_idx < spheres.sphere_count; sphere_idx ++) {
        let hit = hitSphere(spheres.spheres[sphere_idx], ray, t_min, closest_so_far);
        if hit.hit {
            temp_rec.hit = true;
            closest_so_far = hit.t;
            temp_rec = hit;
        }
    }

    return temp_rec;
}

fn rayColor(ray: Ray) -> vec4<f32> {
    var hit = hitSpheres(ray, 0.01, 999999999999.0);
    if hit.hit {
        let c = 0.5 * (hit.normal + vec3<f32>(1.0, 1.0, 1.0));
        return vec4<f32>(c.xyz, 1.0);
    }
    let unit_direction = normalize(ray.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * vec4<f32>(1.0, 1.0, 1.0, 1.0) + t * vec4<f32>(0.5, 0.7, 1.0, 1.0);
}

    @compute @workgroup_size(8, 8, 1)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));

    let color = vec4<f32>(0.0, 0.0, 0.0, 1.0);

    textureStore(texture, location, color);
}

    @compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));

    let wh = textureDimensions(texture);

    let aspect_ratio = f32(wh.x) / f32(wh.y);
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_lenght = 1.0;

    let origin = vec3<f32>(0.0, 0.0, 0.0);
    let horizontal = vec3<f32>(viewport_width, 0.0, 0.0);
    let vertical = vec3<f32>(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - vec3<f32>(0.0, 0.0, focal_lenght);

    let u = f32(location.x) / (f32(wh.x) - 1.0);
    let v = 1.0 - f32(location.y) / (f32(wh.y) - 1.0);

    let color = rayColor(Ray(origin, lower_left_corner + u * horizontal + v * vertical - origin));
    // let color = vec4<f32>(u, v, 0.25, 1.0);

    storageBarrier();

    textureStore(texture, location, color);
}