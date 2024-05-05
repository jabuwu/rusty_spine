# 0.7.2
- Fix `memcpy` crash in Rust 1.78 (when not using `libc` feature)

# 0.7.1
- Fix dark color applying incorrectly with premultiplied alpha (using draw functions)

# 0.7.0
- Add `AttachmentLoader` for creating region attachments
- Upstream fixes
  - Reduce allocations ([EsotericSoftware/spine-runtimes#2325](https://github.com/EsotericSoftware/spine-runtimes/issues/2325))
  - Fix double free of sequences in mesh attachments ([EsotericSoftware/spine-runtimes#2394](https://github.com/EsotericSoftware/spine-runtimes/issues/2394))
  - Fix buffer overflow when loading sequences with less than 10 frames ([EsotericSoftware/spine-runtimes#2397](https://github.com/EsotericSoftware/spine-runtimes/issues/2397))
  - Fix incorrect allocation of TransformTimeline ([EsotericSoftware/spine-runtimes#2401](https://github.com/EsotericSoftware/spine-runtimes/issues/2401))
- Remove unnecessary feature `use_libc`, use `libc` instead
- More clippy fixes
- Fixes for pointer, abi, and memory undefined behavior
- Lots of docs
- Return values now optional:
  - `SkeletonData::audio_path`
  - `SkeletonData::images_path`
  - `SkeletonData::version`
  - `SlotData::attachment_name`
  - `SlotData::blend_mode`
  - `SlotData::color`
  - `TrackEntry::mixing_from`
  - `TrackEntry::mixing_to`
  - `TrackEntry::next`
  - `TrackEntry::previous`
- Remove unnecessary functions:
  - `Atlas::find_page_mut`
  - `Atlas::find_region_mut`
  - `Atlas::pages_mut`
  - `Atlas::regions_mut`
  - `AtlasPage::atlas_mut`
  - `AtlasRegion::page_mut`
  - `BoneData::color_mut`
  - `BoneData::parent_mut`
  - `BoneData::set_length`
  - `BoneData::set_position`
  - `BoneData::set_rotation`
  - `BoneData::set_scale_x`
  - `BoneData::set_scale_y`
  - `BoneData::set_scale`
  - `BoneData::set_shear_x`
  - `BoneData::set_shear_y`
  - `BoneData::set_shear`
  - `BoneData::set_skin_required`
  - `BoneData::set_transform_mode`
  - `BoneData::set_x`
  - `BoneData::set_y`
  - `SkeletonData::animation_at_index_mut`
  - `SkeletonData::animations_mut`
  - `SkeletonData::bone_at_index_mut`
  - `SkeletonData::bones_mut`
  - `SkeletonData::default_skin_mut`
  - `SkeletonData::find_animation_mut`
  - `SkeletonData::find_bone_mut`
  - `SkeletonData::find_skin_mut`
  - `SkeletonData::find_slot_mut`
  - `SkeletonData::skin_at_index_mut`
  - `SkeletonData::skins_mut`
  - `SkeletonData::slot_at_index_mut`
  - `SkeletonData::slots_mut`
  - `SlotData::bone_data_mut`
  - `TrackEntry::set_animation_time`
  - `TrackEntry::set_mixing_from`
  - `TrackEntry::set_mixing_to`
  - `TrackEntry::set_next`
  - `TrackEntry::set_previous`
- Remove unnecessary types:
  - `AtlasPageMutIterator`
  - `AtlasRegionMutIterator`

# 0.6.1
- Compilation fixes for new Rust lints

# 0.6.0
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
