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

//! Low-level bindings for the Bullet C API
//!
//! Some of the functions in `Bullet-C-Api.h` cannot be included because they
//! are only stubs and have not been implemented in Bullet itself. In a custom
//! C API may have to be written in the future to gain access to some of the
//! richer features of the library.
//!
//! The header has this message at the top:
//!
//! > Draft high-level generic physics C-API. For low-level access, use the
//!   physics SDK native API's. Work in progress, functionality will be added on
//!   demand. If possible, use the richer Bullet C++ API, by including
//!   "btBulletDynamicsCommon.h"

use std::libc::*;

macro_rules! pl_declare_handle(
    ($name:ident) => (
        pub struct $name { priv unused: c_int }
    )
)

#[cfg(BT_USE_DOUBLE_PRECISION)]
pub type plReal = c_double;

#[cfg(not(BT_USE_DOUBLE_PRECISION))]
pub type plReal = c_float;

pub type plVector3 = [plReal, ..3];
pub type plQuaternion = [plReal, ..4];

pl_declare_handle!(plPhysicsSdkHandle)
pl_declare_handle!(plDynamicsWorldHandle)
pl_declare_handle!(plRigidBodyHandle)
pl_declare_handle!(plCollisionShapeHandle)
pl_declare_handle!(plConstraintHandle)
// pl_declare_handle!(plMeshInterfaceHandle)
// pl_declare_handle!(plCollisionBroadphaseHandle)
// pl_declare_handle!(plBroadphaseProxyHandle)
pl_declare_handle!(plCollisionWorldHandle)

// typedef void(*btBroadphaseCallback)(void* clientData, void* object1,void* object2);
// pub type btBroadphaseCallback = *u8;

// pub struct plRayCastResult {
//     m_body:             plRigidBodyHandle,
//     m_shape:            plCollisionShapeHandle,
//     m_positionWorld:    *plVector3,
//     m_normalWorld:      *plVector3,
// }

#[link_args="-lBulletCollision -lBulletDynamics -lLinearMath -lc++"]
extern "C" {
    // Create and Delete a Physics SDK
    pub fn plNewBulletSdk() -> plPhysicsSdkHandle;
    pub fn plDeletePhysicsSdk(physicsSdk: plPhysicsSdkHandle);

    // Collision World
    // pub fn plCreateSapBroadphase(beginCallback: btBroadphaseCallback, endCallback: btBroadphaseCallback) -> plCollisionBroadphaseHandle;
    // pub fn plDestroyBroadphase(bp: plCollisionBroadphaseHandle);
    // pub fn plCreateProxy(bp: plCollisionBroadphaseHandle, clientData: *c_void, minX: plReal, minY: plReal, minZ: plReal, maxX: plReal, maxY: plReal, maxZ: plReal) -> plBroadphaseProxyHandle;
    // pub fn plDestroyProxy(bp: plCollisionBroadphaseHandle, proxyHandle: plBroadphaseProxyHandle);
    // pub fn plSetBoundingBox(proxyHandle: plBroadphaseProxyHandle, minX: plReal, minY: plReal, minZ: plReal, maxX: plReal, maxY: plReal, maxZ: plReal);
    // pub fn plCreateCollisionWorld(physicsSdk: plPhysicsSdkHandle) -> plCollisionWorldHandle;

    // Dynamics World
    pub fn plCreateDynamicsWorld(physicsSdk: plPhysicsSdkHandle) -> plDynamicsWorldHandle;
    pub fn plDeleteDynamicsWorld(world: plDynamicsWorldHandle);
    pub fn plStepSimulation(world: plDynamicsWorldHandle, timeStep: plReal);
    pub fn plAddRigidBody(world: plDynamicsWorldHandle, object: plRigidBodyHandle);
    pub fn plRemoveRigidBody(world: plDynamicsWorldHandle, object: plRigidBodyHandle);

    // Rigid Body
    pub fn plCreateRigidBody(user_data: *c_void, mass: c_float, cshape: plCollisionShapeHandle) -> plRigidBodyHandle;
    pub fn plDeleteRigidBody(body: plRigidBodyHandle);

    // Collision Shapes
    pub fn plNewSphereShape(radius: plReal) -> plCollisionShapeHandle;
    pub fn plNewBoxShape(x: plReal, y: plReal, z: plReal) -> plCollisionShapeHandle;
    pub fn plNewCapsuleShape(radius: plReal, height: plReal) -> plCollisionShapeHandle;
    pub fn plNewConeShape(radius: plReal, height: plReal) -> plCollisionShapeHandle;
    pub fn plNewCylinderShape(radius: plReal, height: plReal) -> plCollisionShapeHandle;
    pub fn plNewCompoundShape() -> plCollisionShapeHandle;
    pub fn plAddChildShape(compoundShape: plCollisionShapeHandle, childShape: plCollisionShapeHandle, childPos: *plVector3, childOrn: *plQuaternion);
    pub fn plDeleteShape(shape: plCollisionShapeHandle);

    // Convex Meshes
    pub fn plNewConvexHullShape() -> plCollisionShapeHandle;
    pub fn plAddVertex(convexHull: plCollisionShapeHandle, x: plReal, y: plReal, z: plReal);

    // Concave static triangle meshes
    // pub fn plNewMeshInterface() -> plMeshInterfaceHandle;
    // pub fn plAddTriangle(meshHandle: plMeshInterfaceHandle, v0: *plVector3, v1: *plVector3, v2: *plVector3);
    // pub fn plNewStaticTriangleMeshShape(meshHandle: plMeshInterfaceHandle) -> plCollisionShapeHandle;

    pub fn plSetScaling(shape: plCollisionShapeHandle, scaling: *plVector3);

    // Get world transform
    pub fn plGetOpenGLMatrix(object: plRigidBodyHandle, matrix: *mut plReal);
    pub fn plGetPosition(object: plRigidBodyHandle, position: *mut plVector3);
    pub fn plGetOrientation(object: plRigidBodyHandle, orientation: *mut plQuaternion);

    // Set world transform (position/orientation)
    pub fn plSetPosition(object: plRigidBodyHandle, position: *plVector3);
    pub fn plSetOrientation(object: plRigidBodyHandle, orientation: *plQuaternion);
    pub fn plSetEuler(yaw: plReal, pitch: plReal, roll: plReal, orient: *mut plQuaternion);
    pub fn plSetOpenGLMatrix(object: plRigidBodyHandle, matrix: *plReal);

    // pub fn plRayCast(world: plDynamicsWorldHandle, rayStart: *plVector3, rayEnd: *plVector3, res: plRayCastResult) -> c_int;

    pub fn plNearestPoints(p1: [c_float, ..3], p2: [c_float, ..3], p3: [c_float, ..3],
                           q1: [c_float, ..3], q2: [c_float, ..3], q3: [c_float, ..3],
                           pa: *c_float, pb: *c_float, normal: [c_float, ..3]) -> c_double;
}
