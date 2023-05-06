# 0.6.0 (unreleased)
- All indices are now `usize` (previously some were `i32` which lead to inconsistent APIs)
- Events are nicer to work with using the new `AnimationEvent` enum.
- `Skeleton::set_skin` and `Skin::add_skin` marked unsafe (with safety comments)
  - Add `Skeleton::set_skins_by_name` as a safe alternative for the primary use case (conglomerate skins)
- New error type: `SpineError::PathNotUtf8` (removes some internal `unwrap` calls)
- Lots of clippy fixes
  - Many functions now marked `#[must_use]`
- The following functions are no longer `unsafe`:
  - `MeshAttachment::new_linked_mesh`
  - `MeshAttachment::update_region`
  - `RegionAttachment::update_region`
- Upstream fixes
  - Fix IK constraint NaN when a parent bone has zero scale
  - Handle skeleton loading problems without crashes ([EsotericSoftware/spine-runtimes#2276](https://github.com/EsotericSoftware/spine-runtimes/pull/2276))
  - Fix crash when skeleton `hash` or `spine` doesn't exist in JSON file ([EsotericSoftware/spine-runtimes#2270](https://github.com/EsotericSoftware/spine-runtimes/pull/2270))
  - Fix integer values in events broken with binary loader ([EsotericSoftware/spine-runtimes#2281](https://github.com/EsotericSoftware/spine-runtimes/issues/2281))

# 0.5.2
- Upstream fixes
  - Dark color alpha inconsistent between binary and JSON formats ([EsotericSoftware/spine-runtimes#2263](https://github.com/EsotericSoftware/spine-runtimes/issues/2263))
  - Atlas defaults are not handled ([EsotericSoftware/spine-runtimes#2264](https://github.com/EsotericSoftware/spine-runtimes/issues/2264))
- Set dark color's alpha to 0 for non-PMA and 1 for PMA textures in drawers (the dark color's alpha is not typically used, so this is merely a convenience)

# 0.5.1
- Fixes [#5](https://github.com/jabuwu/rusty_spine/issues/5)
- Adds `SkeletonClipping::clip_triangles`
- Improved docs & tests