#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct World {
    pub scenes: Vec<Scene>,
    pub meshes: MeshRegistry,
}

pub type Scene = petgraph::Graph<Node, ()>;

// For data references in Node components, store the data offset into resource buffers
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Node {
    #[default]
    Empty,
    Viewport(Viewport),
    Node3D {
        transform: Transform3D,
        node: Node3D,
    },
    VisualInstance3D(VisualInstance3D),
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Viewport {
    #[default]
    Empty,
    // Primary Viewport, child cameras register to this
    Main {
        dimension: ViewportDimension,
    },
    // Secondary Viewport, used to render 2D UI over 3D world viewport
    Sub {
        dimension: ViewportDimension,
    },
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ViewportDimension {
    pub width: u32,
    pub height: u32,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Node3D {
    #[default]
    Empty,

    Camera3D {
        camera: Camera3D,
    },
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum VisualInstance3D {
    #[default]
    Empty,
    Geometry(Geometry),
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Geometry {
    #[default]
    Empty,
    Label3D,                        // TODO: 3D text rendering
    SpriteBase3D(SpriteBase3D),     // TODO: 2D sprites rendered in 3D world
    MeshInstance3D(MeshInstance3D), // TODO: 3D meshes rendering
    MultiMeshInstance3D,            // TODO: instanced 3D rendering
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SpriteBase3D {
    #[default]
    Empty,
    Sprite3D,         // TODO: billboard rendering
    AnimatedSprite3D, // TODO: instanced rendering
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Sprite3D {
    billboard: bool, // always facing camera
}

// Mesh is a type of Resource that contains vertex array-based geometry, divided in surfaces. Each surface contains a completely separate array and a material used to draw it. Design wise, a mesh with multiple surfaces is preferred to a single surface, because objects created in 3D editing software commonly contain multiple materials.
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MeshInstance3D {
    pub mesh_reference: Option<MeshId>,
}

pub type MeshId = String;
pub type MeshRegistry = std::collections::HashMap<MeshId, Mesh>;

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Mesh {
    #[default]
    Empty,
    // TODO: implement placeholder meshes
    Placeholder, // Used if primary mesh is unavailable for any reason
    /// Used to draw immediate mode style geometry, highly inefficient to use for anything complex.
    /// Intended for a small amount of geometry that is expected to change frequently.
    // TODO: implement implement immediate mode meshes
    Immediate,
    // TODO: implement array meshes
    // Used to construct a mesh from a set of vertices and indices
    ArrayMesh,
    PrimitiveMesh(PrimitiveMesh),
}

#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize, bytemuck::Zeroable)]
pub struct Transform3D {
    pub translation: nalgebra_glm::Vec3,
    pub rotation: nalgebra_glm::Quat,
    pub scale: nalgebra_glm::Vec3,
}

impl Default for Transform3D {
    fn default() -> Self {
        Self {
            translation: nalgebra_glm::Vec3::new(0.0, 0.0, 0.0),
            rotation: nalgebra_glm::Quat::identity(),
            scale: nalgebra_glm::Vec3::new(1.0, 1.0, 1.0),
        }
    }
}

impl From<([f32; 3], [f32; 4], [f32; 3])> for Transform3D {
    fn from((translation, rotation, scale): ([f32; 3], [f32; 4], [f32; 3])) -> Self {
        Self {
            translation: nalgebra_glm::vec3(translation[0], translation[1], translation[2]),
            rotation: nalgebra_glm::quat(rotation[0], rotation[1], rotation[2], rotation[3]),
            scale: nalgebra_glm::vec3(scale[0], scale[1], scale[2]),
        }
    }
}

impl From<Transform3D> for nalgebra_glm::Mat4 {
    fn from(transform: Transform3D) -> Self {
        nalgebra_glm::translation(&transform.translation)
            * nalgebra_glm::quat_to_mat4(&transform.rotation)
            * nalgebra_glm::scaling(&transform.scale)
    }
}

pub fn decompose_matrix(
    matrix: &nalgebra_glm::Mat4,
) -> (nalgebra_glm::Vec3, nalgebra_glm::Quat, nalgebra_glm::Vec3) {
    let translation = nalgebra_glm::Vec3::new(matrix.m14, matrix.m24, matrix.m34);

    let (scale_x, scale_y, scale_z) = (
        nalgebra_glm::length(&nalgebra_glm::Vec3::new(matrix.m11, matrix.m12, matrix.m13)),
        nalgebra_glm::length(&nalgebra_glm::Vec3::new(matrix.m21, matrix.m22, matrix.m23)),
        nalgebra_glm::length(&nalgebra_glm::Vec3::new(matrix.m31, matrix.m32, matrix.m33)),
    );

    let scale = nalgebra_glm::Vec3::new(scale_x, scale_y, scale_z);

    // Normalize the matrix to extract rotation
    let rotation_matrix = nalgebra_glm::mat3(
        matrix.m11 / scale_x,
        matrix.m12 / scale_y,
        matrix.m13 / scale_z,
        matrix.m21 / scale_x,
        matrix.m22 / scale_y,
        matrix.m23 / scale_z,
        matrix.m31 / scale_x,
        matrix.m32 / scale_y,
        matrix.m33 / scale_z,
    );

    let rotation = nalgebra_glm::mat3_to_quat(&rotation_matrix);

    (translation, rotation, scale)
}

impl From<nalgebra_glm::Mat4> for Transform3D {
    fn from(matrix: nalgebra_glm::Mat4) -> Self {
        let (translation, rotation, scale) = decompose_matrix(&matrix);
        Self {
            translation,
            rotation,
            scale,
        }
    }
}

impl Transform3D {
    pub fn matrix(&self) -> nalgebra_glm::Mat4 {
        nalgebra_glm::Mat4::from(*self)
    }
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Camera3D {
    pub projection: Projection,
}

impl Camera3D {
    pub fn projection_matrix(&self, aspect_ratio: f32) -> nalgebra_glm::Mat4 {
        match &self.projection {
            Projection::Perspective(camera) => camera.matrix(aspect_ratio),
            Projection::Orthographic(camera) => camera.matrix(),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub enum Projection {
    Perspective(PerspectiveCamera),
    Orthographic(OrthographicCamera),
}

impl Default for Projection {
    fn default() -> Self {
        Self::Perspective(PerspectiveCamera::default())
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct PerspectiveCamera {
    pub aspect_ratio: Option<f32>,
    pub y_fov_rad: f32,
    pub z_far: Option<f32>,
    pub z_near: f32,
}
impl Default for PerspectiveCamera {
    fn default() -> Self {
        Self {
            aspect_ratio: None,
            y_fov_rad: 90_f32.to_radians(),
            z_far: None,
            z_near: 0.01,
        }
    }
}

impl PerspectiveCamera {
    pub fn matrix(&self, viewport_aspect_ratio: f32) -> nalgebra_glm::Mat4 {
        let aspect_ratio = if let Some(aspect_ratio) = self.aspect_ratio {
            aspect_ratio
        } else {
            viewport_aspect_ratio
        };

        if let Some(z_far) = self.z_far {
            nalgebra_glm::perspective_zo(aspect_ratio, self.y_fov_rad, self.z_near, z_far)
        } else {
            nalgebra_glm::infinite_perspective_rh_zo(aspect_ratio, self.y_fov_rad, self.z_near)
        }
    }
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct OrthographicCamera {
    pub x_mag: f32,
    pub y_mag: f32,
    pub z_far: f32,
    pub z_near: f32,
}

impl OrthographicCamera {
    pub fn matrix(&self) -> nalgebra_glm::Mat4 {
        let z_sum = self.z_near + self.z_far;
        let z_diff = self.z_near - self.z_far;
        nalgebra_glm::Mat4::new(
            1.0 / self.x_mag,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0 / self.y_mag,
            0.0,
            0.0,
            0.0,
            0.0,
            2.0 / z_diff,
            0.0,
            0.0,
            0.0,
            z_sum / z_diff,
            1.0,
        )
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Orientation {
    pub min_radius: f32,
    pub max_radius: f32,
    pub radius: f32,
    pub offset: nalgebra_glm::Vec3,
    pub sensitivity: nalgebra_glm::Vec2,
    pub direction: nalgebra_glm::Vec2,
}

impl Orientation {
    pub fn direction(&self) -> nalgebra_glm::Vec3 {
        nalgebra_glm::vec3(
            self.direction.y.sin() * self.direction.x.sin(),
            self.direction.y.cos(),
            self.direction.y.sin() * self.direction.x.cos(),
        )
    }

    pub fn rotate(&mut self, position_delta: &nalgebra_glm::Vec2) {
        let delta = position_delta.component_mul(&self.sensitivity);
        self.direction.x += delta.x;
        self.direction.y = nalgebra_glm::clamp_scalar(
            self.direction.y + delta.y,
            10.0_f32.to_radians(),
            170.0_f32.to_radians(),
        );
    }

    pub fn up(&self) -> nalgebra_glm::Vec3 {
        self.right().cross(&self.direction())
    }

    pub fn right(&self) -> nalgebra_glm::Vec3 {
        self.direction().cross(&nalgebra_glm::Vec3::y()).normalize()
    }

    pub fn pan(&mut self, offset: &nalgebra_glm::Vec2) {
        self.offset += self.right() * offset.x;
        self.offset += self.up() * offset.y;
    }

    pub fn position(&self) -> nalgebra_glm::Vec3 {
        (self.direction() * self.radius) + self.offset
    }

    pub fn zoom(&mut self, distance: f32) {
        self.radius -= distance;
        if self.radius < self.min_radius {
            self.radius = self.min_radius;
        }
        if self.radius > self.max_radius {
            self.radius = self.max_radius;
        }
    }

    pub fn look_at_offset(&self) -> nalgebra_glm::Quat {
        self.look(self.offset - self.position())
    }

    pub fn look_forward(&self) -> nalgebra_glm::Quat {
        self.look(-self.direction())
    }

    fn look(&self, point: nalgebra_glm::Vec3) -> nalgebra_glm::Quat {
        nalgebra_glm::quat_conjugate(&nalgebra_glm::quat_look_at(
            &point,
            &nalgebra_glm::Vec3::y(),
        ))
    }
}

impl Default for Orientation {
    fn default() -> Self {
        Self {
            min_radius: 1.0,
            max_radius: 100.0,
            radius: 5.0,
            offset: nalgebra_glm::vec3(0.0, 0.0, 0.0),
            sensitivity: nalgebra_glm::vec2(1.0, 1.0),
            direction: nalgebra_glm::Vec2::new(0.0, 1.0),
        }
    }
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PrimitiveMesh {
    pub shape: PrimitiveShape,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum PrimitiveShape {
    #[default]
    Triangle,
    Box,
    Capsule,
    Cylinder,
    Plane,
    Point,
    Sphere,
}
