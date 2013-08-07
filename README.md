# bullet-rs

Bindings and wrapper for the [Bullet Physics](http://bulletphysics.org) C API.

## Note

Unfortunately Bullet's [C API](http://code.google.com/p/bullet/source/browse/trunk/src/Bullet-C-Api.h)
is extremely limited, and is not regularly updated. The header has this
message at the top:

> Draft high-level generic physics C-API. For low-level access, use the
physics SDK native API's. Work in progress, functionality will be added on
demand. If possible, use the richer Bullet C++ API, by including
btBulletDynamicsCommon.h"

Which doesn't really help folks who work in other languages! In the future it
might be better to implement a custom C API in order to provide a more
comprehensive wrapper for Bullet.
