// Copyright 2013 The BULLET-RS Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[link(name = "bullet",
       vers = "0.1",
       author = "Brendan Zabarauskas",
       url = "https://github.com/bjz/bullet-rs")];

#[comment = "Bindings and wrapper for the Bullet physics C API."];
#[license = "ASL2"];
#[crate_type = "lib"];

use std::cast::transmute;
use std::libc::c_float;

pub mod ffi;

pub type Real = ffi::plReal;
pub type Vector3 = [Real, ..3];
pub type Quaternion = [Real, ..4];

macro_rules! declare_handle(
    ($Self:ident, $H:ident) => (
        pub struct $Self { priv handle: ffi::$H }
    )
)

pub trait Handle<H> {
    pub fn get_handle(&self) -> H;
}

macro_rules! impl_handle(
    ($Self:ident, $H:ident) => (
        impl Handle<ffi::$H> for $Self {
            pub fn get_handle(&self) -> ffi::$H { self.handle }
        }
    )
)

// Pysics SDK

declare_handle!(PhysicsSDK, plPhysicsSdkHandle)
impl_handle!(PhysicsSDK, plPhysicsSdkHandle)

impl PhysicsSDK {
    pub fn new() -> PhysicsSDK {
        unsafe {
            PhysicsSDK { handle: ffi::plNewBulletSdk() }
        }
    }

    pub fn delete(&self) {
        unsafe { ffi::plDeletePhysicsSdk(self.get_handle()); }
    }
}

// Dynamics World

declare_handle!(DynamicsWorld, plDynamicsWorldHandle)
impl_handle!(DynamicsWorld, plDynamicsWorldHandle)

impl DynamicsWorld {
    pub fn create(sdk: &PhysicsSDK) -> DynamicsWorld {
        unsafe {
            DynamicsWorld { handle: ffi::plCreateDynamicsWorld(sdk.get_handle()) }
        }
    }

    pub fn step_simulation(&self, time_step: Real) {
        unsafe { ffi::plStepSimulation(self.get_handle(), time_step); }
    }

    pub fn add_rigid_body(&self, object: &RigidBody) {
        unsafe { ffi::plAddRigidBody(self.get_handle(), object.get_handle()); }
    }

    pub fn remove_rigid_body(&self, object: &RigidBody) {
        unsafe { ffi::plRemoveRigidBody(self.get_handle(), object.get_handle()); }
    }

    pub fn delete(&self) {
        unsafe { ffi::plDeleteDynamicsWorld(self.get_handle()); }
    }
}

// Rigid body

declare_handle!(RigidBody, plRigidBodyHandle)
impl_handle!(RigidBody, plRigidBodyHandle)

impl RigidBody {
    pub fn create<T, CS: CollisionShape>(user_data: *T, mass: float, cshape: &CS) -> RigidBody {
        unsafe {
            RigidBody {
                handle: ffi::plCreateRigidBody(transmute(user_data), mass as c_float, cshape.get_handle())
            }
        }
    }

    pub fn delete(&self) {
        unsafe { ffi::plDeleteRigidBody(self.get_handle()); }
    }

    /// matrix: a 15 element array (12 rotation(row major padded on the right by 1), and 3 translation
    pub fn get_opengl_matrix(&self, matrix: &mut [Real, ..15]) {
        unsafe { ffi::plGetOpenGLMatrix(self.get_handle(), transmute(matrix)); }
    }

    pub fn get_position(&self, position: &mut Vector3) {
        unsafe { ffi::plGetPosition(self.get_handle(), position); }
    }

    pub fn get_orientation(&self, orientation: &mut Quaternion) {
        unsafe { ffi::plGetOrientation(self.get_handle(), orientation); }
    }

    pub fn set_position(&self, position: &Vector3) {
        unsafe { ffi::plSetPosition(self.get_handle(), position); }
    }

    pub fn set_orientation(&self, orientation: &Quaternion) {
        unsafe { ffi::plSetOrientation(self.get_handle(), orientation); }
    }

    /// matrix: a 15 element array (12 rotation(row major padded on the right by 1), and 3 translation
    pub fn set_opengl_matrix(&self, matrix: &[Real, ..15]) {
        unsafe { ffi::plSetOpenGLMatrix(self.get_handle(), transmute(matrix)); }
    }

    // pub fn ray_cast(&self, rayStart: &Vector3, rayEnd: &Vector3) -> Option<~RayCastResult> {}
}

// Collision shapes

pub trait CollisionShape: Handle<ffi::plCollisionShapeHandle> {
    pub fn set_scaling(&self, scaling: &Vector3) {
        unsafe { ffi::plSetScaling(self.get_handle(), scaling); }
    }

    pub fn delete(&self) {
        unsafe { ffi::plDeleteShape(self.get_handle()); }
    }
}

declare_handle!(SphereShape, plCollisionShapeHandle)
impl_handle!(SphereShape, plCollisionShapeHandle)
impl CollisionShape for SphereShape;

impl SphereShape {
    pub fn new(radius: Real) -> SphereShape {
        unsafe {
            SphereShape {
                handle: ffi::plNewSphereShape(radius)
            }
        }
    }
}

declare_handle!(BoxShape, plCollisionShapeHandle)
impl_handle!(BoxShape, plCollisionShapeHandle)
impl CollisionShape for BoxShape;

impl BoxShape {
    pub fn new(x: Real, y: Real, z: Real) -> BoxShape {
        unsafe {
            BoxShape {
                handle: ffi::plNewBoxShape(x, y, z)
            }
        }
    }
}

declare_handle!(CapsuleShape, plCollisionShapeHandle)
impl_handle!(CapsuleShape, plCollisionShapeHandle)
impl CollisionShape for CapsuleShape;

impl CapsuleShape {
    pub fn new(radius: Real, height: Real) -> CapsuleShape {
        unsafe {
            CapsuleShape {
                handle: ffi::plNewCapsuleShape(radius, height)
            }
        }
    }
}

declare_handle!(ConeShape, plCollisionShapeHandle)
impl_handle!(ConeShape, plCollisionShapeHandle)
impl CollisionShape for ConeShape;

impl ConeShape {
    pub fn new(radius: Real, height: Real) -> ConeShape {
        unsafe {
            ConeShape {
                handle: ffi::plNewConeShape(radius, height)
            }
        }
    }
}

declare_handle!(CylinderShape, plCollisionShapeHandle)
impl_handle!(CylinderShape, plCollisionShapeHandle)
impl CollisionShape for CylinderShape;

impl CylinderShape {
    pub fn new(radius: Real, height: Real) -> CylinderShape {
        unsafe {
            CylinderShape {
                handle: ffi::plNewCylinderShape(radius, height)
            }
        }
    }
}

declare_handle!(CompoundShape, plCollisionShapeHandle)
impl_handle!(CompoundShape, plCollisionShapeHandle)
impl CollisionShape for CompoundShape;

impl CompoundShape {
    pub fn new() -> CompoundShape {
        unsafe {
            CompoundShape {
                handle: ffi::plNewCompoundShape()
            }
        }
    }

    pub fn add_child<CS: CollisionShape>(&self, child: &CS, child_pos: &Vector3, child_orn: &Quaternion) {
        unsafe { ffi::plAddChildShape(self.get_handle(), child.get_handle(), child_pos, child_orn); }
    }
}

declare_handle!(ConvexHullShape, plCollisionShapeHandle)
impl_handle!(ConvexHullShape, plCollisionShapeHandle)
impl CollisionShape for ConvexHullShape;

impl ConvexHullShape {
    pub fn new() -> ConvexHullShape {
        unsafe {
            ConvexHullShape {
                handle: ffi::plNewConvexHullShape()
            }
        }
    }

    pub fn add_vertex(&self, x: Real, y: Real, z: Real) {
        unsafe { ffi::plAddVertex(self.get_handle(), x, y, z); }
    }
}

// declare_handle!(TriangleMeshInterface, plMeshInterfaceHandle)
// impl_handle!(TriangleMeshInterface, plMeshInterfaceHandle)

// impl TriangleMeshInterface {
//     pub fn new() -> TriangleMeshInterface {
//         unsafe {
//             TriangleMeshInterface {
//                 handle: ffi::plNewMeshInterface()
//             }
//         }
//     }

//     pub fn add_triangle(&self, v0: &Vector3, v1: &Vector3, v2: &Vector3) {
//         unsafe { ffi::plAddTriangle(self.get_handle(), v0, v1, v2); }
//     }
// }

// declare_handle!(TriangleMeshShape, plCollisionShapeHandle)
// impl_handle!(TriangleMeshShape, plCollisionShapeHandle)
// impl CollisionShape for TriangleMeshShape;

// impl TriangleMeshShape {
//     pub fn new(mesh: TriangleMeshInterface) -> TriangleMeshShape {
//         unsafe {
//             TriangleMeshShape {
//                 handle: ffi::plNewStaticTriangleMeshShape(mesh.get_handle())
//             }
//         }
//     }
// }
