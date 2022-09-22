rusty_spine uses the official C runtime of Spine and doesn't benefit from many safety guarantees that Rust developers are used to. To deal with this, the rusty_spine Rust API tries to put that safety "on top" which works to greatly simplify the effort of dealing with C pointers, but the actual safety guarantee can be dubious at times, as you might expect. This document explains how safety is handled in this project.

Feedback is welcome on how this could be improved further.

## Pointer wrappers

Most structs in this project are a thin wrapper around a C pointer and do not hold much meaningful state of their own. Unfortunately, it's not simple in most cases to upcast to a Rust object, so intead there can be multiple structs instantiated which point to the same pointer. Multiple structs referencing the same pointers is a huge safety concern, particularly when multithreading is involved, so there are a few light restrictions in place to help avoid common safety pitfalls.

## CTmpRef and CTmpMut

`CTmpRef` and `CTMpMut` are by far the most common wrappers used in rusty_spine. The implementation is fairly simple, an object is paired in a struct with a reference to a parent object, and because of this, only accessible so long as the parent's lifetime is guaranteed. A good example of where this is useful is grabbing all the bones of a skeleton. A `Bone` class is created wrapping the underlying `spBone` C struct, but it's returned in a `CTmpRef` or `CTmpMut` which references the `Skeleton`. So long as the `Skeleton` class remains alive, the bones can be accessed without any issue or further checks. `CTmpRef` and `CTmpMut` effectively recreate the safety guarantees of Rust's borrow checker for C pointers.

## CTmp and CMut (Note: Recently replaced with handles)

Sometimes, it is desirable to have a long-term reference to an object, such as caching a `Bone` from a `Skeleton` so it doesn't need to be looked up multiple times. In C, you would store a pointer to `spBone`, but this comes with obvious safety concerns that we try to avoid in Rust. Rather than returning pointers, rusty_spine provides `CTmp` and `CMut` which are wrappers around pointers providing restrictions on their usage. Particularly, a reference counted atomic is stored alongside the pointer which can invalidate the object when the parent is disposed (such as `CTmp<Bone>` becoming invalid after its owning `Skeleton` is dropped). Additionally, to get the child object, the parent object must be passed in either by reference or by mutable reference for `CTmpRef` and `CTmpMut` respectively. This might seem like it defeats the purpose of having a solitary, standalone reference to the object but it's the only way to guarantee that, for example, two `Bone` structs in a `Skeleton` are not mutated simulataneously. This restriction can be bypassed by using the unsafe `get_unchecked` method rather than the safe `get` method.

`CTmp` and `CMut` are acquired from `CTmpRef` and `CTmpMut` respectively using the `keep` method. Please note that this operation is only possible when acquiring a `CTmpRef` or `CTmpMut` from the struct which owns the data. For example, it's possible to get a `CTmp<Bone>` from `Skeleton` but not possible to get a `CTmp<Bone>` from another `CTmp<Bone>` (for example, getting a bone's parent).

## Using pointers directly

If all this stuff about `CTmpRef` and `CTmpMut` seems unnecessary to you, it's possible to get the underlying pointer of any type by using the `c_ptr` method. To "upcast" back to Rust, use the `new_from_ptr` method.

```
let bone = skeleton.bone_at_index(0).unwrap();
let c_bone = bone.c_ptr();
let same_bone = unsafe { Bone::new_from_ptr(c_bone) };
```

## Dispose on drop

In the C runtime, structs are allocated using a `_create` call and are expected to be later freed using `_dispose`. If a Rust struct has called `_create`, it remembers this action (usually in an `owns_memory` boolean) and calls the associated `_dipose` in its `Drop` trait. Structs created with `new_from_ptr` do not call `_dispose` functions.

## Reference counting

Certain types depend on others to remain in existence. For example, a `Skeleton` depends on `SkeletonData`, and so it would be invalid for `SkeletonData` to be dropped if a `Skeleton` exists which still depends on it. As you might expect, this is solved with simple reference counting, and is explicitly bubbled up to the user as some methods require an `Arc` rather than their plain types.

## Work remaining

This project is WIP, and not all safety concerns have been addressed. Particularly, attachments and track entries have tricky lifetimes which are not accurately reflected in the Rust API.

The `CRef` and `CMut` classes can probably be simplified.