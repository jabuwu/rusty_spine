#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {

    fn spine_memcpy(__dest: *mut c_void, __src: *const c_void, __n: size_t) -> *mut c_void;
    fn spine_memmove(__dest: *mut c_void, __src: *const c_void, __n: size_t) -> *mut c_void;
    fn acosf(_: c_float) -> c_float;
    fn atan2f(_: c_float, _: c_float) -> c_float;
    fn spine_memset(__s: *mut c_void, __c: c_int, __n: size_t) -> *mut c_void;
    fn cosf(_: c_float) -> c_float;
    fn sinf(_: c_float) -> c_float;
    fn spine_strcasecmp(__s1: *const c_char, __s2: *const c_char) -> c_int;
    fn spine_strcpy(__dest: *mut c_char, __src: *const c_char) -> *mut c_char;
    fn spine_strncat(__dest: *mut c_char, __src: *const c_char, __n: size_t) -> *mut c_char;
    fn spine_strcmp(__s1: *const c_char, __s2: *const c_char) -> c_int;
    fn spine_strncmp(__s1: *const c_char, __s2: *const c_char, __n: size_t) -> c_int;
    fn pow(_: c_double, _: c_double) -> c_double;
    fn spine_sqrtf(__x: c_float) -> c_float;
    fn _spAtlasPage_createTexture(self_0: *mut spAtlasPage, path: *const c_char);
    fn _spAtlasPage_disposeTexture(self_0: *mut spAtlasPage);
    fn _spUtil_readFile(path: *const c_char, length: *mut c_int) -> *mut c_char;
    fn fmodf(_: c_float, _: c_float) -> c_float;
    fn spine_strtol(__nptr: *const c_char, __endptr: *mut *mut c_char, __base: c_int) -> c_long;
    fn spine_strtoul(__nptr: *const c_char, __endptr: *mut *mut c_char, __base: c_int) -> c_ulong;
    fn spine_fclose(__stream: *mut FILE) -> c_int;
    fn spine_fopen(__filename: *const c_char, __modes: *const c_char) -> *mut FILE;
    fn spine_strrchr(__s: *const c_char, __c: c_int) -> *mut c_char;

    fn spine_strlen(__s: *const c_char) -> size_t;

    fn spine_rand() -> c_int;
    fn spine_malloc(__size: size_t) -> *mut c_void;
    fn spine_realloc(__ptr: *mut c_void, __size: size_t) -> *mut c_void;
    fn spine_free(__ptr: *mut c_void);
    fn spine_fread(__ptr: *mut c_void, __size: size_t, __n: size_t, __stream: *mut FILE) -> size_t;
    fn spine_fseek(__stream: *mut FILE, __off: c_long, __whence: c_int) -> c_int;
    fn spine_ftell(__stream: *mut FILE) -> c_long;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spEventData {
    pub name: *const c_char,
    pub intValue: c_int,
    pub floatValue: c_float,
    pub stringValue: *const c_char,
    pub audioPath: *const c_char,
    pub volume: c_float,
    pub balance: c_float,
}
pub type size_t = c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spEvent {
    pub data: *mut spEventData,
    pub time: c_float,
    pub intValue: c_int,
    pub floatValue: c_float,
    pub stringValue: *const c_char,
    pub volume: c_float,
    pub balance: c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spAttachmentLoader {
    pub error1: *const c_char,
    pub error2: *const c_char,
    pub vtable: *const c_void,
}
pub type spAttachmentType = c_uint;
pub const SP_ATTACHMENT_CLIPPING: spAttachmentType = 6;
pub const SP_ATTACHMENT_POINT: spAttachmentType = 5;
pub const SP_ATTACHMENT_PATH: spAttachmentType = 4;
pub const SP_ATTACHMENT_LINKED_MESH: spAttachmentType = 3;
pub const SP_ATTACHMENT_MESH: spAttachmentType = 2;
pub const SP_ATTACHMENT_BOUNDING_BOX: spAttachmentType = 1;
pub const SP_ATTACHMENT_REGION: spAttachmentType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spAttachment {
    pub name: *const c_char,
    pub type_0: spAttachmentType,
    pub vtable: *const c_void,
    pub refCount: c_int,
    pub attachmentLoader: *mut spAttachmentLoader,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _spAttachmentVtable {
    pub dispose: Option<unsafe extern "C" fn(*mut spAttachment) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut spAttachment) -> *mut spAttachment>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spColor {
    pub r: c_float,
    pub g: c_float,
    pub b: c_float,
    pub a: c_float,
}
pub type spTransformMode = c_uint;
pub const SP_TRANSFORMMODE_NOSCALEORREFLECTION: spTransformMode = 4;
pub const SP_TRANSFORMMODE_NOSCALE: spTransformMode = 3;
pub const SP_TRANSFORMMODE_NOROTATIONORREFLECTION: spTransformMode = 2;
pub const SP_TRANSFORMMODE_ONLYTRANSLATION: spTransformMode = 1;
pub const SP_TRANSFORMMODE_NORMAL: spTransformMode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spBoneData {
    pub index: c_int,
    pub name: *const c_char,
    pub parent: *mut spBoneData,
    pub length: c_float,
    pub x: c_float,
    pub y: c_float,
    pub rotation: c_float,
    pub scaleX: c_float,
    pub scaleY: c_float,
    pub shearX: c_float,
    pub shearY: c_float,
    pub transformMode: spTransformMode,
    pub skinRequired: c_int,
    pub color: spColor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spSkeleton {
    pub data: *mut spSkeletonData,
    pub bonesCount: c_int,
    pub bones: *mut *mut spBone,
    pub root: *mut spBone,
    pub slotsCount: c_int,
    pub slots: *mut *mut spSlot,
    pub drawOrder: *mut *mut spSlot,
    pub ikConstraintsCount: c_int,
    pub ikConstraints: *mut *mut spIkConstraint,
    pub transformConstraintsCount: c_int,
    pub transformConstraints: *mut *mut spTransformConstraint,
    pub pathConstraintsCount: c_int,
    pub pathConstraints: *mut *mut spPathConstraint,
    pub skin: *mut spSkin,
    pub color: spColor,
    pub scaleX: c_float,
    pub scaleY: c_float,
    pub x: c_float,
    pub y: c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spSkin {
    pub name: *const c_char,
    pub bones: *mut spBoneDataArray,
    pub ikConstraints: *mut spIkConstraintDataArray,
    pub transformConstraints: *mut spTransformConstraintDataArray,
    pub pathConstraints: *mut spPathConstraintDataArray,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spPathConstraintDataArray {
    pub size: c_int,
    pub capacity: c_int,
    pub items: *mut *mut spPathConstraintData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spPathConstraintData {
    pub name: *const c_char,
    pub order: c_int,
    pub skinRequired: c_int,
    pub bonesCount: c_int,
    pub bones: *mut *mut spBoneData,
    pub target: *mut spSlotData,
    pub positionMode: spPositionMode,
    pub spacingMode: spSpacingMode,
    pub rotateMode: spRotateMode,
    pub offsetRotation: c_float,
    pub position: c_float,
    pub spacing: c_float,
    pub mixRotate: c_float,
    pub mixX: c_float,
    pub mixY: c_float,
}
pub type spRotateMode = c_uint;
pub const SP_ROTATE_MODE_CHAIN_SCALE: spRotateMode = 2;
pub const SP_ROTATE_MODE_CHAIN: spRotateMode = 1;
pub const SP_ROTATE_MODE_TANGENT: spRotateMode = 0;
pub type spSpacingMode = c_uint;
pub const SP_SPACING_MODE_PROPORTIONAL: spSpacingMode = 3;
pub const SP_SPACING_MODE_PERCENT: spSpacingMode = 2;
pub const SP_SPACING_MODE_FIXED: spSpacingMode = 1;
pub const SP_SPACING_MODE_LENGTH: spSpacingMode = 0;
pub type spPositionMode = c_uint;
pub const SP_POSITION_MODE_PERCENT: spPositionMode = 1;
pub const SP_POSITION_MODE_FIXED: spPositionMode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spSlotData {
    pub index: c_int,
    pub name: *const c_char,
    pub boneData: *const spBoneData,
    pub attachmentName: *const c_char,
    pub color: spColor,
    pub darkColor: *mut spColor,
    pub blendMode: spBlendMode,
}
pub type spBlendMode = c_uint;
pub const SP_BLEND_MODE_SCREEN: spBlendMode = 3;
pub const SP_BLEND_MODE_MULTIPLY: spBlendMode = 2;
pub const SP_BLEND_MODE_ADDITIVE: spBlendMode = 1;
pub const SP_BLEND_MODE_NORMAL: spBlendMode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spTransformConstraintDataArray {
    pub size: c_int,
    pub capacity: c_int,
    pub items: *mut *mut spTransformConstraintData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spTransformConstraintData {
    pub name: *const c_char,
    pub order: c_int,
    pub skinRequired: c_int,
    pub bonesCount: c_int,
    pub bones: *mut *mut spBoneData,
    pub target: *mut spBoneData,
    pub mixRotate: c_float,
    pub mixX: c_float,
    pub mixY: c_float,
    pub mixScaleX: c_float,
    pub mixScaleY: c_float,
    pub mixShearY: c_float,
    pub offsetRotation: c_float,
    pub offsetX: c_float,
    pub offsetY: c_float,
    pub offsetScaleX: c_float,
    pub offsetScaleY: c_float,
    pub offsetShearY: c_float,
    pub relative: c_int,
    pub local: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spIkConstraintDataArray {
    pub size: c_int,
    pub capacity: c_int,
    pub items: *mut *mut spIkConstraintData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spIkConstraintData {
    pub name: *const c_char,
    pub order: c_int,
    pub skinRequired: c_int,
    pub bonesCount: c_int,
    pub bones: *mut *mut spBoneData,
    pub target: *mut spBoneData,
    pub bendDirection: c_int,
    pub compress: c_int,
    pub stretch: c_int,
    pub uniform: c_int,
    pub mix: c_float,
    pub softness: c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spBoneDataArray {
    pub size: c_int,
    pub capacity: c_int,
    pub items: *mut *mut spBoneData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spPathConstraint {
    pub data: *mut spPathConstraintData,
    pub bonesCount: c_int,
    pub bones: *mut *mut spBone,
    pub target: *mut spSlot,
    pub position: c_float,
    pub spacing: c_float,
    pub mixRotate: c_float,
    pub mixX: c_float,
    pub mixY: c_float,
    pub spacesCount: c_int,
    pub spaces: *mut c_float,
    pub positionsCount: c_int,
    pub positions: *mut c_float,
    pub worldCount: c_int,
    pub world: *mut c_float,
    pub curvesCount: c_int,
    pub curves: *mut c_float,
    pub lengthsCount: c_int,
    pub lengths: *mut c_float,
    pub segments: [c_float; 10],
    pub active: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spSlot {
    pub data: *mut spSlotData,
    pub bone: *mut spBone,
    pub color: spColor,
    pub darkColor: *mut spColor,
    pub attachment: *mut spAttachment,
    pub attachmentState: c_int,
    pub deformCapacity: c_int,
    pub deformCount: c_int,
    pub deform: *mut c_float,
    pub sequenceIndex: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spBone {
    pub data: *mut spBoneData,
    pub skeleton: *mut spSkeleton,
    pub parent: *mut spBone,
    pub childrenCount: c_int,
    pub children: *mut *mut spBone,
    pub x: c_float,
    pub y: c_float,
    pub rotation: c_float,
    pub scaleX: c_float,
    pub scaleY: c_float,
    pub shearX: c_float,
    pub shearY: c_float,
    pub ax: c_float,
    pub ay: c_float,
    pub arotation: c_float,
    pub ascaleX: c_float,
    pub ascaleY: c_float,
    pub ashearX: c_float,
    pub ashearY: c_float,
    pub a: c_float,
    pub b: c_float,
    pub worldX: c_float,
    pub c: c_float,
    pub d: c_float,
    pub worldY: c_float,
    pub sorted: c_int,
    pub active: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spTransformConstraint {
    pub data: *mut spTransformConstraintData,
    pub bonesCount: c_int,
    pub bones: *mut *mut spBone,
    pub target: *mut spBone,
    pub mixRotate: c_float,
    pub mixX: c_float,
    pub mixY: c_float,
    pub mixScaleX: c_float,
    pub mixScaleY: c_float,
    pub mixShearY: c_float,
    pub active: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spIkConstraint {
    pub data: *mut spIkConstraintData,
    pub bonesCount: c_int,
    pub bones: *mut *mut spBone,
    pub target: *mut spBone,
    pub bendDirection: c_int,
    pub compress: c_int,
    pub stretch: c_int,
    pub mix: c_float,
    pub softness: c_float,
    pub active: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spSkeletonData {
    pub version: *const c_char,
    pub hash: *const c_char,
    pub x: c_float,
    pub y: c_float,
    pub width: c_float,
    pub height: c_float,
    pub fps: c_float,
    pub imagesPath: *const c_char,
    pub audioPath: *const c_char,
    pub stringsCount: c_int,
    pub strings: *mut *mut c_char,
    pub bonesCount: c_int,
    pub bones: *mut *mut spBoneData,
    pub slotsCount: c_int,
    pub slots: *mut *mut spSlotData,
    pub skinsCount: c_int,
    pub skins: *mut *mut spSkin,
    pub defaultSkin: *mut spSkin,
    pub eventsCount: c_int,
    pub events: *mut *mut spEventData,
    pub animationsCount: c_int,
    pub animations: *mut *mut spAnimation,
    pub ikConstraintsCount: c_int,
    pub ikConstraints: *mut *mut spIkConstraintData,
    pub transformConstraintsCount: c_int,
    pub transformConstraints: *mut *mut spTransformConstraintData,
    pub pathConstraintsCount: c_int,
    pub pathConstraints: *mut *mut spPathConstraintData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spAnimation {
    pub name: *const c_char,
    pub duration: c_float,
    pub timelines: *mut spTimelineArray,
    pub timelineIds: *mut spPropertyIdArray,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spPropertyIdArray {
    pub size: c_int,
    pub capacity: c_int,
    pub items: *mut spPropertyId,
}
pub type spPropertyId = uint64_t;
pub type uint64_t = __uint64_t;
pub type __uint64_t = c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spTimelineArray {
    pub size: c_int,
    pub capacity: c_int,
    pub items: *mut *mut spTimeline,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spTimeline {
    pub vtable: _spTimelineVtable,
    pub propertyIds: [spPropertyId; 3],
    pub propertyIdsCount: c_int,
    pub frames: *mut spFloatArray,
    pub frameCount: c_int,
    pub frameEntries: c_int,
    pub type_0: spTimelineType,
}
pub type spTimelineType = c_uint;
pub const SP_TIMELINE_EVENT: spTimelineType = 24;
pub const SP_TIMELINE_DRAWORDER: spTimelineType = 23;
pub const SP_TIMELINE_TRANSFORMCONSTRAINT: spTimelineType = 22;
pub const SP_TIMELINE_RGB: spTimelineType = 21;
pub const SP_TIMELINE_RGBA: spTimelineType = 20;
pub const SP_TIMELINE_RGBA2: spTimelineType = 19;
pub const SP_TIMELINE_RGB2: spTimelineType = 18;
pub const SP_TIMELINE_PATHCONSTRAINTMIX: spTimelineType = 17;
pub const SP_TIMELINE_IKCONSTRAINT: spTimelineType = 16;
pub const SP_TIMELINE_SEQUENCE: spTimelineType = 15;
pub const SP_TIMELINE_DEFORM: spTimelineType = 14;
pub const SP_TIMELINE_TRANSLATE: spTimelineType = 13;
pub const SP_TIMELINE_SHEAR: spTimelineType = 12;
pub const SP_TIMELINE_SCALE: spTimelineType = 11;
pub const SP_TIMELINE_TRANSLATEY: spTimelineType = 10;
pub const SP_TIMELINE_TRANSLATEX: spTimelineType = 9;
pub const SP_TIMELINE_SHEARY: spTimelineType = 8;
pub const SP_TIMELINE_SHEARX: spTimelineType = 7;
pub const SP_TIMELINE_SCALEY: spTimelineType = 6;
pub const SP_TIMELINE_SCALEX: spTimelineType = 5;
pub const SP_TIMELINE_ROTATE: spTimelineType = 4;
pub const SP_TIMELINE_PATHCONSTRAINTSPACING: spTimelineType = 3;
pub const SP_TIMELINE_PATHCONSTRAINTPOSITION: spTimelineType = 2;
pub const SP_TIMELINE_ALPHA: spTimelineType = 1;
pub const SP_TIMELINE_ATTACHMENT: spTimelineType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spFloatArray {
    pub size: c_int,
    pub capacity: c_int,
    pub items: *mut c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _spTimelineVtable {
    pub apply: Option<
        unsafe extern "C" fn(
            *mut spTimeline,
            *mut spSkeleton,
            c_float,
            c_float,
            *mut *mut spEvent,
            *mut c_int,
            c_float,
            spMixBlend,
            spMixDirection,
        ) -> (),
    >,
    pub dispose: Option<unsafe extern "C" fn(*mut spTimeline) -> ()>,
    pub setBezier: Option<
        unsafe extern "C" fn(
            *mut spTimeline,
            c_int,
            c_int,
            c_float,
            c_float,
            c_float,
            c_float,
            c_float,
            c_float,
            c_float,
            c_float,
            c_float,
        ) -> (),
    >,
}
pub type spMixDirection = c_uint;
pub const SP_MIX_DIRECTION_OUT: spMixDirection = 1;
pub const SP_MIX_DIRECTION_IN: spMixDirection = 0;
pub type spMixBlend = c_uint;
pub const SP_MIX_BLEND_ADD: spMixBlend = 3;
pub const SP_MIX_BLEND_REPLACE: spMixBlend = 2;
pub const SP_MIX_BLEND_FIRST: spMixBlend = 1;
pub const SP_MIX_BLEND_SETUP: spMixBlend = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _SkinHashTableEntry {
    pub entry: *mut _Entry,
    pub next: *mut _SkinHashTableEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _Entry {
    pub slotIndex: c_int,
    pub name: *const c_char,
    pub attachment: *mut spAttachment,
    pub next: *mut _Entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _spSkin {
    pub super_0: spSkin,
    pub entries: *mut _Entry,
    pub entriesHashTable: [*mut _SkinHashTableEntry; 100],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spVertexAttachment {
    pub super_0: spAttachment,
    pub bonesCount: c_int,
    pub bones: *mut c_int,
    pub verticesCount: c_int,
    pub vertices: *mut c_float,
    pub worldVerticesLength: c_int,
    pub timelineAttachment: *mut spAttachment,
    pub id: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spSequence {
    pub id: c_int,
    pub start: c_int,
    pub digits: c_int,
    pub setupIndex: c_int,
    pub regions: *mut spTextureRegionArray,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spTextureRegionArray {
    pub size: c_int,
    pub capacity: c_int,
    pub items: *mut *mut spTextureRegion,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spTextureRegion {
    pub rendererObject: *mut c_void,
    pub u: c_float,
    pub v: c_float,
    pub u2: c_float,
    pub v2: c_float,
    pub degrees: c_int,
    pub offsetX: c_float,
    pub offsetY: c_float,
    pub width: c_int,
    pub height: c_int,
    pub originalWidth: c_int,
    pub originalHeight: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spMeshAttachment {
    pub super_0: spVertexAttachment,
    pub rendererObject: *mut c_void,
    pub region: *mut spTextureRegion,
    pub sequence: *mut spSequence,
    pub path: *const c_char,
    pub regionUVs: *mut c_float,
    pub uvs: *mut c_float,
    pub trianglesCount: c_int,
    pub triangles: *mut c_ushort,
    pub color: spColor,
    pub hullLength: c_int,
    pub parentMesh: *mut spMeshAttachment,
    pub edgesCount: c_int,
    pub edges: *mut c_int,
    pub width: c_float,
    pub height: c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spRegionAttachment {
    pub super_0: spAttachment,
    pub path: *const c_char,
    pub x: c_float,
    pub y: c_float,
    pub scaleX: c_float,
    pub scaleY: c_float,
    pub rotation: c_float,
    pub width: c_float,
    pub height: c_float,
    pub color: spColor,
    pub rendererObject: *mut c_void,
    pub region: *mut spTextureRegion,
    pub sequence: *mut spSequence,
    pub offset: [c_float; 8],
    pub uvs: [c_float; 8],
}
pub const BLY: C2RustUnnamed_1 = 1;
pub const BLX: C2RustUnnamed_1 = 0;
pub const BRY: C2RustUnnamed_1 = 7;
pub const BRX: C2RustUnnamed_1 = 6;
pub const URY: C2RustUnnamed_1 = 5;
pub const URX: C2RustUnnamed_1 = 4;
pub const ULY: C2RustUnnamed_1 = 3;
pub const ULX: C2RustUnnamed_1 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spIntArray {
    pub size: c_int,
    pub capacity: c_int,
    pub items: *mut c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spShortArray {
    pub size: c_int,
    pub capacity: c_int,
    pub items: *mut c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spUnsignedShortArray {
    pub size: c_int,
    pub capacity: c_int,
    pub items: *mut c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spArrayFloatArray {
    pub size: c_int,
    pub capacity: c_int,
    pub items: *mut *mut spFloatArray,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spArrayShortArray {
    pub size: c_int,
    pub capacity: c_int,
    pub items: *mut *mut spShortArray,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spAtlas {
    pub pages: *mut spAtlasPage,
    pub regions: *mut spAtlasRegion,
    pub rendererObject: *mut c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spAtlasRegion {
    pub super_0: spTextureRegion,
    pub name: *const c_char,
    pub x: c_int,
    pub y: c_int,
    pub index: c_int,
    pub splits: *mut c_int,
    pub pads: *mut c_int,
    pub keyValues: *mut spKeyValueArray,
    pub page: *mut spAtlasPage,
    pub next: *mut spAtlasRegion,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spAtlasPage {
    pub atlas: *const spAtlas,
    pub name: *const c_char,
    pub format: spAtlasFormat,
    pub minFilter: spAtlasFilter,
    pub magFilter: spAtlasFilter,
    pub uWrap: spAtlasWrap,
    pub vWrap: spAtlasWrap,
    pub rendererObject: *mut c_void,
    pub width: c_int,
    pub height: c_int,
    pub pma: c_int,
    pub next: *mut spAtlasPage,
}
pub type spAtlasWrap = c_uint;
pub const SP_ATLAS_REPEAT: spAtlasWrap = 2;
pub const SP_ATLAS_CLAMPTOEDGE: spAtlasWrap = 1;
pub const SP_ATLAS_MIRROREDREPEAT: spAtlasWrap = 0;
pub type spAtlasFilter = c_uint;
pub const SP_ATLAS_MIPMAP_LINEAR_LINEAR: spAtlasFilter = 7;
pub const SP_ATLAS_MIPMAP_NEAREST_LINEAR: spAtlasFilter = 6;
pub const SP_ATLAS_MIPMAP_LINEAR_NEAREST: spAtlasFilter = 5;
pub const SP_ATLAS_MIPMAP_NEAREST_NEAREST: spAtlasFilter = 4;
pub const SP_ATLAS_MIPMAP: spAtlasFilter = 3;
pub const SP_ATLAS_LINEAR: spAtlasFilter = 2;
pub const SP_ATLAS_NEAREST: spAtlasFilter = 1;
pub const SP_ATLAS_UNKNOWN_FILTER: spAtlasFilter = 0;
pub type spAtlasFormat = c_uint;
pub const SP_ATLAS_RGBA8888: spAtlasFormat = 7;
pub const SP_ATLAS_RGB888: spAtlasFormat = 6;
pub const SP_ATLAS_RGBA4444: spAtlasFormat = 5;
pub const SP_ATLAS_RGB565: spAtlasFormat = 4;
pub const SP_ATLAS_LUMINANCE_ALPHA: spAtlasFormat = 3;
pub const SP_ATLAS_INTENSITY: spAtlasFormat = 2;
pub const SP_ATLAS_ALPHA: spAtlasFormat = 1;
pub const SP_ATLAS_UNKNOWN_FORMAT: spAtlasFormat = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spKeyValueArray {
    pub size: c_int,
    pub capacity: c_int,
    pub items: *mut spKeyValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spKeyValue {
    pub name: *mut c_char,
    pub values: [c_float; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SimpleString {
    pub start: *mut c_char,
    pub end: *mut c_char,
    pub length: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AtlasInput {
    pub start: *const c_char,
    pub end: *const c_char,
    pub index: *mut c_char,
    pub length: c_int,
    pub line: SimpleString,
}
pub type __uint32_t = c_uint;
pub type __off_t = c_long;
pub type __off64_t = c_long;
pub type uint32_t = __uint32_t;
pub type C2RustUnnamed = c_uint;
pub const SP_PROPERTY_SEQUENCE: C2RustUnnamed = 524288;
pub const SP_PROPERTY_PATHCONSTRAINT_MIX: C2RustUnnamed = 262144;
pub const SP_PROPERTY_PATHCONSTRAINT_SPACING: C2RustUnnamed = 131072;
pub const SP_PROPERTY_PATHCONSTRAINT_POSITION: C2RustUnnamed = 65536;
pub const SP_PROPERTY_TRANSFORMCONSTRAINT: C2RustUnnamed = 32768;
pub const SP_PROPERTY_IKCONSTRAINT: C2RustUnnamed = 16384;
pub const SP_PROPERTY_DRAWORDER: C2RustUnnamed = 8192;
pub const SP_PROPERTY_EVENT: C2RustUnnamed = 4096;
pub const SP_PROPERTY_DEFORM: C2RustUnnamed = 2048;
pub const SP_PROPERTY_ATTACHMENT: C2RustUnnamed = 1024;
pub const SP_PROPERTY_RGB2: C2RustUnnamed = 512;
pub const SP_PROPERTY_ALPHA: C2RustUnnamed = 256;
pub const SP_PROPERTY_RGB: C2RustUnnamed = 128;
pub const SP_PROPERTY_SHEARY: C2RustUnnamed = 64;
pub const SP_PROPERTY_SHEARX: C2RustUnnamed = 32;
pub const SP_PROPERTY_SCALEY: C2RustUnnamed = 16;
pub const SP_PROPERTY_SCALEX: C2RustUnnamed = 8;
pub const SP_PROPERTY_Y: C2RustUnnamed = 4;
pub const SP_PROPERTY_X: C2RustUnnamed = 2;
pub const SP_PROPERTY_ROTATE: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spCurveTimeline {
    pub super_0: spTimeline,
    pub curves: *mut spFloatArray,
}
pub type spCurveTimeline1 = spCurveTimeline;
pub type spCurveTimeline2 = spCurveTimeline;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spRotateTimeline {
    pub super_0: spCurveTimeline1,
    pub boneIndex: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spTranslateTimeline {
    pub super_0: spCurveTimeline2,
    pub boneIndex: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spTranslateXTimeline {
    pub super_0: spCurveTimeline1,
    pub boneIndex: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spTranslateYTimeline {
    pub super_0: spCurveTimeline1,
    pub boneIndex: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spScaleTimeline {
    pub super_0: spCurveTimeline2,
    pub boneIndex: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spScaleXTimeline {
    pub super_0: spCurveTimeline1,
    pub boneIndex: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spScaleYTimeline {
    pub super_0: spCurveTimeline1,
    pub boneIndex: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spShearTimeline {
    pub super_0: spCurveTimeline2,
    pub boneIndex: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spShearXTimeline {
    pub super_0: spCurveTimeline1,
    pub boneIndex: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spShearYTimeline {
    pub super_0: spCurveTimeline1,
    pub boneIndex: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spRGBATimeline {
    pub super_0: spCurveTimeline2,
    pub slotIndex: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spRGBTimeline {
    pub super_0: spCurveTimeline2,
    pub slotIndex: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spAlphaTimeline {
    pub super_0: spCurveTimeline1,
    pub slotIndex: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spRGBA2Timeline {
    pub super_0: spCurveTimeline,
    pub slotIndex: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spRGB2Timeline {
    pub super_0: spCurveTimeline,
    pub slotIndex: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spAttachmentTimeline {
    pub super_0: spTimeline,
    pub slotIndex: c_int,
    pub attachmentNames: *mut *const c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spDeformTimeline {
    pub super_0: spCurveTimeline,
    pub frameVerticesCount: c_int,
    pub frameVertices: *mut *const c_float,
    pub slotIndex: c_int,
    pub attachment: *mut spAttachment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spSequenceTimeline {
    pub super_0: spTimeline,
    pub slotIndex: c_int,
    pub attachment: *mut spAttachment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spEventTimeline {
    pub super_0: spTimeline,
    pub events: *mut *mut spEvent,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spDrawOrderTimeline {
    pub super_0: spTimeline,
    pub drawOrders: *mut *const c_int,
    pub slotsCount: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spIkConstraintTimeline {
    pub super_0: spCurveTimeline,
    pub ikConstraintIndex: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spTransformConstraintTimeline {
    pub super_0: spCurveTimeline,
    pub transformConstraintIndex: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spPathConstraintPositionTimeline {
    pub super_0: spCurveTimeline,
    pub pathConstraintIndex: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spPathConstraintSpacingTimeline {
    pub super_0: spCurveTimeline,
    pub pathConstraintIndex: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spPathConstraintMixTimeline {
    pub super_0: spCurveTimeline,
    pub pathConstraintIndex: c_int,
}
pub type spSkinEntry = _Entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _spAttachmentLoaderVtable {
    pub createAttachment: Option<
        unsafe extern "C" fn(
            *mut spAttachmentLoader,
            *mut spSkin,
            spAttachmentType,
            *const c_char,
            *const c_char,
            *mut spSequence,
        ) -> *mut spAttachment,
    >,
    pub configureAttachment:
        Option<unsafe extern "C" fn(*mut spAttachmentLoader, *mut spAttachment) -> ()>,
    pub disposeAttachment:
        Option<unsafe extern "C" fn(*mut spAttachmentLoader, *mut spAttachment) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut spAttachmentLoader) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spPathAttachment {
    pub super_0: spVertexAttachment,
    pub lengthsLength: c_int,
    pub lengths: *mut c_float,
    pub closed: c_int,
    pub constantSpeed: c_int,
    pub color: spColor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _spSkeleton {
    pub super_0: spSkeleton,
    pub updateCacheCount: c_int,
    pub updateCacheCapacity: c_int,
    pub updateCache: *mut _spUpdate,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _spUpdate {
    pub type_0: _spUpdateType,
    pub object: *mut c_void,
}
pub type _spUpdateType = c_uint;
pub const SP_UPDATE_TRANSFORM_CONSTRAINT: _spUpdateType = 3;
pub const SP_UPDATE_PATH_CONSTRAINT: _spUpdateType = 2;
pub const SP_UPDATE_IK_CONSTRAINT: _spUpdateType = 1;
pub const SP_UPDATE_BONE: _spUpdateType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spBoundingBoxAttachment {
    pub super_0: spVertexAttachment,
    pub color: spColor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spClippingAttachment {
    pub super_0: spVertexAttachment,
    pub endSlot: *mut spSlotData,
    pub color: spColor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spPointAttachment {
    pub super_0: spAttachment,
    pub x: c_float,
    pub y: c_float,
    pub rotation: c_float,
    pub color: spColor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spAnimationStateData {
    pub skeletonData: *mut spSkeletonData,
    pub defaultMix: c_float,
    pub entries: *const c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FromEntry {
    pub animation: *mut spAnimation,
    pub toEntries: *mut _ToEntry,
    pub next: *mut _FromEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ToEntry {
    pub animation: *mut spAnimation,
    pub duration: c_float,
    pub next: *mut _ToEntry,
}
pub type spEventType = c_uint;
pub const SP_ANIMATION_EVENT: spEventType = 5;
pub const SP_ANIMATION_DISPOSE: spEventType = 4;
pub const SP_ANIMATION_COMPLETE: spEventType = 3;
pub const SP_ANIMATION_END: spEventType = 2;
pub const SP_ANIMATION_INTERRUPT: spEventType = 1;
pub const SP_ANIMATION_START: spEventType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spAnimationState {
    pub data: *mut spAnimationStateData,
    pub tracksCount: c_int,
    pub tracks: *mut *mut spTrackEntry,
    pub listener: spAnimationStateListener,
    pub timeScale: c_float,
    pub rendererObject: *mut c_void,
    pub userData: *mut c_void,
    pub unkeyedState: c_int,
}
pub type spAnimationStateListener = Option<
    unsafe extern "C" fn(*mut spAnimationState, spEventType, *mut spTrackEntry, *mut spEvent) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spTrackEntry {
    pub animation: *mut spAnimation,
    pub previous: *mut spTrackEntry,
    pub next: *mut spTrackEntry,
    pub mixingFrom: *mut spTrackEntry,
    pub mixingTo: *mut spTrackEntry,
    pub listener: spAnimationStateListener,
    pub trackIndex: c_int,
    pub loop_0: c_int,
    pub holdPrevious: c_int,
    pub reverse: c_int,
    pub shortestRotation: c_int,
    pub eventThreshold: c_float,
    pub attachmentThreshold: c_float,
    pub drawOrderThreshold: c_float,
    pub animationStart: c_float,
    pub animationEnd: c_float,
    pub animationLast: c_float,
    pub nextAnimationLast: c_float,
    pub delay: c_float,
    pub trackTime: c_float,
    pub trackLast: c_float,
    pub nextTrackLast: c_float,
    pub trackEnd: c_float,
    pub timeScale: c_float,
    pub alpha: c_float,
    pub mixTime: c_float,
    pub mixDuration: c_float,
    pub interruptAlpha: c_float,
    pub totalAlpha: c_float,
    pub mixBlend: spMixBlend,
    pub timelineMode: *mut spIntArray,
    pub timelineHoldMix: *mut spTrackEntryArray,
    pub timelinesRotation: *mut c_float,
    pub timelinesRotationCount: c_int,
    pub rendererObject: *mut c_void,
    pub userData: *mut c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spTrackEntryArray {
    pub size: c_int,
    pub capacity: c_int,
    pub items: *mut *mut spTrackEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _spAnimationState {
    pub super_0: spAnimationState,
    pub eventsCount: c_int,
    pub events: *mut *mut spEvent,
    pub queue: *mut _spEventQueue,
    pub propertyIDs: *mut spPropertyId,
    pub propertyIDsCount: c_int,
    pub propertyIDsCapacity: c_int,
    pub animationsChanged: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _spEventQueue {
    pub state: *mut _spAnimationState,
    pub objects: *mut _spEventQueueItem,
    pub objectsCount: c_int,
    pub objectsCapacity: c_int,
    pub drainDisabled: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union _spEventQueueItem {
    pub type_0: c_int,
    pub entry: *mut spTrackEntry,
    pub event: *mut spEvent,
}
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: c_int,
    pub _IO_read_ptr: *mut c_char,
    pub _IO_read_end: *mut c_char,
    pub _IO_read_base: *mut c_char,
    pub _IO_write_base: *mut c_char,
    pub _IO_write_ptr: *mut c_char,
    pub _IO_write_end: *mut c_char,
    pub _IO_buf_base: *mut c_char,
    pub _IO_buf_end: *mut c_char,
    pub _IO_save_base: *mut c_char,
    pub _IO_backup_base: *mut c_char,
    pub _IO_save_end: *mut c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: c_int,
    pub _flags2: c_int,
    pub _old_offset: __off_t,
    pub _cur_column: c_ushort,
    pub _vtable_offset: c_schar,
    pub _shortbuf: [c_char; 1],
    pub _lock: *mut c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut c_void,
    pub __pad5: size_t,
    pub _mode: c_int,
    pub _unused2: [c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spAtlasAttachmentLoader {
    pub super_0: spAttachmentLoader,
    pub atlas: *mut spAtlas,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spPolygon {
    pub vertices: *mut c_float,
    pub count: c_int,
    pub capacity: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spSkeletonBounds {
    pub count: c_int,
    pub boundingBoxes: *mut *mut spBoundingBoxAttachment,
    pub polygons: *mut *mut spPolygon,
    pub minX: c_float,
    pub minY: c_float,
    pub maxX: c_float,
    pub maxY: c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _spSkeletonBounds {
    pub super_0: spSkeletonBounds,
    pub capacity: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spSkeletonBinary {
    pub scale: c_float,
    pub attachmentLoader: *mut spAttachmentLoader,
    pub error: *const c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _spSkeletonBinary {
    pub super_0: spSkeletonBinary,
    pub ownsLoader: c_int,
    pub linkedMeshCount: c_int,
    pub linkedMeshCapacity: c_int,
    pub linkedMeshes: *mut _spLinkedMeshBinary,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _spLinkedMeshBinary {
    pub parent: *const c_char,
    pub skin: *const c_char,
    pub slotIndex: c_int,
    pub mesh: *mut spMeshAttachment,
    pub inheritTimeline: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dataInput {
    pub cursor: *const c_uchar,
    pub end: *const c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub intValue: c_int,
    pub floatValue: c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spSkeletonJson {
    pub scale: c_float,
    pub attachmentLoader: *mut spAttachmentLoader,
    pub error: *const c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _spSkeletonJson {
    pub super_0: spSkeletonJson,
    pub ownsLoader: c_int,
    pub linkedMeshCount: c_int,
    pub linkedMeshCapacity: c_int,
    pub linkedMeshes: *mut _spLinkedMeshJson,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _spLinkedMeshJson {
    pub parent: *const c_char,
    pub skin: *const c_char,
    pub slotIndex: c_int,
    pub mesh: *mut spMeshAttachment,
    pub inheritTimeline: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Json {
    pub next: *mut Json,
    pub child: *mut Json,
    pub type_0: c_int,
    pub size: c_int,
    pub valueString: *const c_char,
    pub valueInt: c_int,
    pub valueFloat: c_float,
    pub name: *const c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spTriangulator {
    pub convexPolygons: *mut spArrayFloatArray,
    pub convexPolygonsIndices: *mut spArrayShortArray,
    pub indicesArray: *mut spShortArray,
    pub isConcaveArray: *mut spIntArray,
    pub triangles: *mut spShortArray,
    pub polygonPool: *mut spArrayFloatArray,
    pub polygonIndicesPool: *mut spArrayShortArray,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spSkeletonClipping {
    pub triangulator: *mut spTriangulator,
    pub clippingPolygon: *mut spFloatArray,
    pub clipOutput: *mut spFloatArray,
    pub clippedVertices: *mut spFloatArray,
    pub clippedUVs: *mut spFloatArray,
    pub clippedTriangles: *mut spUnsignedShortArray,
    pub scratch: *mut spFloatArray,
    pub clipAttachment: *mut spClippingAttachment,
    pub clippingPolygons: *mut spArrayFloatArray,
}
pub type C2RustUnnamed_1 = c_uint;
#[no_mangle]
pub unsafe extern "C" fn isspace_(mut x: c_int) -> c_int {
    return (x <= 32 as c_int) as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spPropertyIdArray_create(
    mut initialCapacity: c_int,
) -> *mut spPropertyIdArray {
    let mut array: *mut spPropertyIdArray = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spPropertyIdArray>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        37 as c_int,
    ) as *mut spPropertyIdArray;
    (*array).size = 0 as c_int;
    (*array).capacity = initialCapacity;
    (*array).items = _spCalloc(
        initialCapacity as size_t,
        ::core::mem::size_of::<spPropertyId>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        37 as c_int,
    ) as *mut spPropertyId;
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn spPropertyIdArray_dispose(mut self_0: *mut spPropertyIdArray) {
    _spFree((*self_0).items as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spPropertyIdArray_clear(mut self_0: *mut spPropertyIdArray) {
    (*self_0).size = 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spPropertyIdArray_setSize(
    mut self_0: *mut spPropertyIdArray,
    mut newSize: c_int,
) -> *mut spPropertyIdArray {
    (*self_0).size = newSize;
    if (*self_0).capacity < newSize {
        (*self_0).capacity = if 8 as c_int > ((*self_0).size as c_float * 1.75f32) as c_int {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0).items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<spPropertyId>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut spPropertyId;
    }
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spPropertyIdArray_ensureCapacity(
    mut self_0: *mut spPropertyIdArray,
    mut newCapacity: c_int,
) {
    if (*self_0).capacity >= newCapacity {
        return;
    }
    (*self_0).capacity = newCapacity;
    (*self_0).items = _spRealloc(
        (*self_0).items as *mut c_void,
        (::core::mem::size_of::<spPropertyId>() as c_ulong)
            .wrapping_mul((*self_0).capacity as c_ulong),
    ) as *mut spPropertyId;
}
#[no_mangle]
pub unsafe extern "C" fn spPropertyIdArray_add(
    mut self_0: *mut spPropertyIdArray,
    mut value: spPropertyId,
) {
    if (*self_0).size == (*self_0).capacity {
        (*self_0).capacity = if 8 as c_int > ((*self_0).size as c_float * 1.75f32) as c_int {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0).items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<spPropertyId>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut spPropertyId;
    }
    let fresh0 = (*self_0).size;
    (*self_0).size = (*self_0).size + 1;
    *((*self_0).items).offset(fresh0 as isize) = value;
}
#[no_mangle]
pub unsafe extern "C" fn spPropertyIdArray_addAll(
    mut self_0: *mut spPropertyIdArray,
    mut other: *mut spPropertyIdArray,
) {
    let mut i: c_int = 0 as c_int;
    while i < (*other).size {
        spPropertyIdArray_add(self_0, *((*other).items).offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spPropertyIdArray_addAllValues(
    mut self_0: *mut spPropertyIdArray,
    mut values: *mut spPropertyId,
    mut offset: c_int,
    mut count: c_int,
) {
    let mut i: c_int = offset;
    let mut n: c_int = offset + count;
    while i < n {
        spPropertyIdArray_add(self_0, *values.offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spPropertyIdArray_removeAt(
    mut self_0: *mut spPropertyIdArray,
    mut index: c_int,
) {
    (*self_0).size -= 1;
    spine_memmove(
        ((*self_0).items).offset(index as isize) as *mut c_void,
        ((*self_0).items)
            .offset(index as isize)
            .offset(1 as c_int as isize) as *const c_void,
        (::core::mem::size_of::<spPropertyId>() as c_ulong)
            .wrapping_mul(((*self_0).size - index) as c_ulong),
    );
}
#[no_mangle]
pub unsafe extern "C" fn spPropertyIdArray_contains(
    mut self_0: *mut spPropertyIdArray,
    mut value: spPropertyId,
) -> c_int {
    let mut items: *mut spPropertyId = (*self_0).items;
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    i = 0 as c_int;
    n = (*self_0).size;
    while i < n {
        if *items.offset(i as isize) == value {
            return -(1 as c_int);
        }
        i += 1;
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spPropertyIdArray_pop(mut self_0: *mut spPropertyIdArray) -> spPropertyId {
    (*self_0).size -= 1;
    let mut item: spPropertyId = *((*self_0).items).offset((*self_0).size as isize);
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn spPropertyIdArray_peek(
    mut self_0: *mut spPropertyIdArray,
) -> spPropertyId {
    return *((*self_0).items).offset(((*self_0).size - 1 as c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn spTimelineArray_create(
    mut initialCapacity: c_int,
) -> *mut spTimelineArray {
    let mut array: *mut spTimelineArray = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spTimelineArray>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        39 as c_int,
    ) as *mut spTimelineArray;
    (*array).size = 0 as c_int;
    (*array).capacity = initialCapacity;
    (*array).items = _spCalloc(
        initialCapacity as size_t,
        ::core::mem::size_of::<*mut spTimeline>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        39 as c_int,
    ) as *mut *mut spTimeline;
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn spTimelineArray_dispose(mut self_0: *mut spTimelineArray) {
    _spFree((*self_0).items as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spTimelineArray_clear(mut self_0: *mut spTimelineArray) {
    (*self_0).size = 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spTimelineArray_setSize(
    mut self_0: *mut spTimelineArray,
    mut newSize: c_int,
) -> *mut spTimelineArray {
    (*self_0).size = newSize;
    if (*self_0).capacity < newSize {
        (*self_0).capacity = if 8 as c_int > ((*self_0).size as c_float * 1.75f32) as c_int {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0).items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<*mut spTimeline>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut *mut spTimeline;
    }
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spTimelineArray_ensureCapacity(
    mut self_0: *mut spTimelineArray,
    mut newCapacity: c_int,
) {
    if (*self_0).capacity >= newCapacity {
        return;
    }
    (*self_0).capacity = newCapacity;
    (*self_0).items = _spRealloc(
        (*self_0).items as *mut c_void,
        (::core::mem::size_of::<*mut spTimeline>() as c_ulong)
            .wrapping_mul((*self_0).capacity as c_ulong),
    ) as *mut *mut spTimeline;
}
#[no_mangle]
pub unsafe extern "C" fn spTimelineArray_add(
    mut self_0: *mut spTimelineArray,
    mut value: *mut spTimeline,
) {
    if (*self_0).size == (*self_0).capacity {
        (*self_0).capacity = if 8 as c_int > ((*self_0).size as c_float * 1.75f32) as c_int {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0).items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<*mut spTimeline>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut *mut spTimeline;
    }
    let fresh1 = (*self_0).size;
    (*self_0).size = (*self_0).size + 1;
    let ref mut fresh2 = *((*self_0).items).offset(fresh1 as isize);
    *fresh2 = value;
}
#[no_mangle]
pub unsafe extern "C" fn spTimelineArray_addAll(
    mut self_0: *mut spTimelineArray,
    mut other: *mut spTimelineArray,
) {
    let mut i: c_int = 0 as c_int;
    while i < (*other).size {
        spTimelineArray_add(self_0, *((*other).items).offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spTimelineArray_addAllValues(
    mut self_0: *mut spTimelineArray,
    mut values: *mut *mut spTimeline,
    mut offset: c_int,
    mut count: c_int,
) {
    let mut i: c_int = offset;
    let mut n: c_int = offset + count;
    while i < n {
        spTimelineArray_add(self_0, *values.offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spTimelineArray_removeAt(
    mut self_0: *mut spTimelineArray,
    mut index: c_int,
) {
    (*self_0).size -= 1;
    spine_memmove(
        ((*self_0).items).offset(index as isize) as *mut c_void,
        ((*self_0).items)
            .offset(index as isize)
            .offset(1 as c_int as isize) as *const c_void,
        (::core::mem::size_of::<*mut spTimeline>() as c_ulong)
            .wrapping_mul(((*self_0).size - index) as c_ulong),
    );
}
#[no_mangle]
pub unsafe extern "C" fn spTimelineArray_contains(
    mut self_0: *mut spTimelineArray,
    mut value: *mut spTimeline,
) -> c_int {
    let mut items: *mut *mut spTimeline = (*self_0).items;
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    i = 0 as c_int;
    n = (*self_0).size;
    while i < n {
        if *items.offset(i as isize) == value {
            return -(1 as c_int);
        }
        i += 1;
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spTimelineArray_pop(mut self_0: *mut spTimelineArray) -> *mut spTimeline {
    (*self_0).size -= 1;
    let mut item: *mut spTimeline = *((*self_0).items).offset((*self_0).size as isize);
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn spTimelineArray_peek(mut self_0: *mut spTimelineArray) -> *mut spTimeline {
    return *((*self_0).items).offset(((*self_0).size - 1 as c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn spAnimation_create(
    mut name: *const c_char,
    mut timelines: *mut spTimelineArray,
    mut duration: c_float,
) -> *mut spAnimation {
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    let mut self_0: *mut spAnimation = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spAnimation>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        43 as c_int,
    ) as *mut spAnimation;
    let ref mut fresh3 = *(&(*self_0).name as *const *const c_char as *mut *mut c_char);
    *fresh3 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul((spine_strlen(name)).wrapping_add(1 as c_int as c_ulong)),
        b"spine.c\0" as *const u8 as *const c_char,
        44 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh3, name);
    (*self_0).timelines = if !timelines.is_null() {
        timelines
    } else {
        spTimelineArray_create(1 as c_int)
    };
    timelines = (*self_0).timelines;
    (*self_0).timelineIds = spPropertyIdArray_create(16 as c_int);
    i = 0 as c_int;
    n = (*timelines).size;
    while i < n {
        spPropertyIdArray_addAllValues(
            (*self_0).timelineIds,
            ((**((*timelines).items).offset(i as isize)).propertyIds).as_mut_ptr(),
            0 as c_int,
            (**((*timelines).items).offset(i as isize)).propertyIdsCount,
        );
        i += 1;
    }
    (*self_0).duration = duration;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spAnimation_dispose(mut self_0: *mut spAnimation) {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*(*self_0).timelines).size {
        spTimeline_dispose(*((*(*self_0).timelines).items).offset(i as isize));
        i += 1;
    }
    spTimelineArray_dispose((*self_0).timelines);
    spPropertyIdArray_dispose((*self_0).timelineIds);
    _spFree((*self_0).name as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spAnimation_hasTimeline(
    mut self_0: *mut spAnimation,
    mut ids: *mut spPropertyId,
    mut idsCount: c_int,
) -> c_int {
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    let mut ii: c_int = 0;
    i = 0 as c_int;
    n = (*(*self_0).timelineIds).size;
    while i < n {
        ii = 0 as c_int;
        while ii < idsCount {
            if *((*(*self_0).timelineIds).items).offset(i as isize) == *ids.offset(ii as isize) {
                return 1 as c_int;
            }
            ii += 1;
        }
        i += 1;
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spAnimation_apply(
    mut self_0: *const spAnimation,
    mut skeleton: *mut spSkeleton,
    mut lastTime: c_float,
    mut time: c_float,
    mut loop_0: c_int,
    mut events: *mut *mut spEvent,
    mut eventsCount: *mut c_int,
    mut alpha: c_float,
    mut blend: spMixBlend,
    mut direction: spMixDirection,
) {
    let mut i: c_int = 0;
    let mut n: c_int = (*(*self_0).timelines).size;
    if loop_0 != 0 && (*self_0).duration != 0. {
        time = fmodf(time, (*self_0).duration);
        if lastTime > 0 as c_int as c_float {
            lastTime = fmodf(lastTime, (*self_0).duration);
        }
    }
    i = 0 as c_int;
    while i < n {
        spTimeline_apply(
            *((*(*self_0).timelines).items).offset(i as isize),
            skeleton,
            lastTime,
            time,
            events,
            eventsCount,
            alpha,
            blend,
            direction,
        );
        i += 1;
    }
}
unsafe extern "C" fn search(mut values: *mut spFloatArray, mut time: c_float) -> c_int {
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    let mut items: *mut c_float = (*values).items;
    i = 1 as c_int;
    n = (*values).size;
    while i < n {
        if *items.offset(i as isize) > time {
            return i - 1 as c_int;
        }
        i += 1;
    }
    return (*values).size - 1 as c_int;
}
unsafe extern "C" fn search2(
    mut values: *mut spFloatArray,
    mut time: c_float,
    mut step: c_int,
) -> c_int {
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    let mut items: *mut c_float = (*values).items;
    i = step;
    n = (*values).size;
    while i < n {
        if *items.offset(i as isize) > time {
            return i - step;
        }
        i += step;
    }
    return (*values).size - step;
}
#[no_mangle]
pub unsafe extern "C" fn _spTimeline_init(
    mut self_0: *mut spTimeline,
    mut frameCount: c_int,
    mut frameEntries: c_int,
    mut propertyIds: *mut spPropertyId,
    mut propertyIdsCount: c_int,
    mut type_0: spTimelineType,
    mut dispose: Option<unsafe extern "C" fn(*mut spTimeline) -> ()>,
    mut apply: Option<
        unsafe extern "C" fn(
            *mut spTimeline,
            *mut spSkeleton,
            c_float,
            c_float,
            *mut *mut spEvent,
            *mut c_int,
            c_float,
            spMixBlend,
            spMixDirection,
        ) -> (),
    >,
    mut setBezier: Option<
        unsafe extern "C" fn(
            *mut spTimeline,
            c_int,
            c_int,
            c_float,
            c_float,
            c_float,
            c_float,
            c_float,
            c_float,
            c_float,
            c_float,
            c_float,
        ) -> (),
    >,
) {
    let mut i: c_int = 0;
    (*self_0).frames = spFloatArray_create(frameCount * frameEntries);
    (*(*self_0).frames).size = frameCount * frameEntries;
    (*self_0).frameCount = frameCount;
    (*self_0).frameEntries = frameEntries;
    i = 0 as c_int;
    while i < propertyIdsCount {
        (*self_0).propertyIds[i as usize] = *propertyIds.offset(i as isize);
        i += 1;
    }
    (*self_0).propertyIdsCount = propertyIdsCount;
    (*self_0).type_0 = type_0;
    (*self_0).vtable.dispose = dispose;
    (*self_0).vtable.apply = apply;
    (*self_0).vtable.setBezier = setBezier;
}
#[no_mangle]
pub unsafe extern "C" fn spTimeline_dispose(mut self_0: *mut spTimeline) {
    ((*self_0).vtable.dispose).expect("non-null function pointer")(self_0);
    spFloatArray_dispose((*self_0).frames);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spTimeline_apply(
    mut self_0: *mut spTimeline,
    mut skeleton: *mut spSkeleton,
    mut lastTime: c_float,
    mut time: c_float,
    mut firedEvents: *mut *mut spEvent,
    mut eventsCount: *mut c_int,
    mut alpha: c_float,
    mut blend: spMixBlend,
    mut direction: spMixDirection,
) {
    ((*self_0).vtable.apply).expect("non-null function pointer")(
        self_0,
        skeleton,
        lastTime,
        time,
        firedEvents,
        eventsCount,
        alpha,
        blend,
        direction,
    );
}
#[no_mangle]
pub unsafe extern "C" fn spTimeline_setBezier(
    mut self_0: *mut spTimeline,
    mut bezier: c_int,
    mut frame: c_int,
    mut value: c_float,
    mut time1: c_float,
    mut value1: c_float,
    mut cx1: c_float,
    mut cy1: c_float,
    mut cx2: c_float,
    mut cy2: c_float,
    mut time2: c_float,
    mut value2: c_float,
) {
    if ((*self_0).vtable.setBezier).is_some() {
        ((*self_0).vtable.setBezier).expect("non-null function pointer")(
            self_0, bezier, frame, value, time1, value1, cx1, cy1, cx2, cy2, time2, value2,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn spTimeline_getDuration(mut self_0: *const spTimeline) -> c_float {
    return *((*(*self_0).frames).items)
        .offset(((*(*self_0).frames).size - (*self_0).frameEntries) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn _spCurveTimeline_init(
    mut self_0: *mut spCurveTimeline,
    mut frameCount: c_int,
    mut frameEntries: c_int,
    mut bezierCount: c_int,
    mut propertyIds: *mut spPropertyId,
    mut propertyIdsCount: c_int,
    mut type_0: spTimelineType,
    mut dispose: Option<unsafe extern "C" fn(*mut spTimeline) -> ()>,
    mut apply: Option<
        unsafe extern "C" fn(
            *mut spTimeline,
            *mut spSkeleton,
            c_float,
            c_float,
            *mut *mut spEvent,
            *mut c_int,
            c_float,
            spMixBlend,
            spMixDirection,
        ) -> (),
    >,
    mut setBezier: Option<
        unsafe extern "C" fn(
            *mut spTimeline,
            c_int,
            c_int,
            c_float,
            c_float,
            c_float,
            c_float,
            c_float,
            c_float,
            c_float,
            c_float,
            c_float,
        ) -> (),
    >,
) {
    _spTimeline_init(
        &mut (*self_0).super_0,
        frameCount,
        frameEntries,
        propertyIds,
        propertyIdsCount,
        type_0,
        dispose,
        apply,
        setBezier,
    );
    (*self_0).curves = spFloatArray_create(frameCount + bezierCount * 18 as c_int);
    (*(*self_0).curves).size = frameCount + bezierCount * 18 as c_int;
    *((*(*self_0).curves).items).offset((frameCount - 1 as c_int) as isize) = 1 as c_int as c_float;
}
#[no_mangle]
pub unsafe extern "C" fn _spCurveTimeline_dispose(mut self_0: *mut spTimeline) {
    spFloatArray_dispose((*(self_0 as *mut spCurveTimeline)).curves);
}
#[no_mangle]
pub unsafe extern "C" fn _spCurveTimeline_setBezier(
    mut timeline: *mut spTimeline,
    mut bezier: c_int,
    mut frame: c_int,
    mut value: c_float,
    mut time1: c_float,
    mut value1: c_float,
    mut cx1: c_float,
    mut cy1: c_float,
    mut cx2: c_float,
    mut cy2: c_float,
    mut time2: c_float,
    mut value2: c_float,
) {
    let mut self_0: *mut spCurveTimeline = timeline as *mut spCurveTimeline;
    let mut tmpx: c_float = 0.;
    let mut tmpy: c_float = 0.;
    let mut dddx: c_float = 0.;
    let mut dddy: c_float = 0.;
    let mut ddx: c_float = 0.;
    let mut ddy: c_float = 0.;
    let mut dx: c_float = 0.;
    let mut dy: c_float = 0.;
    let mut x: c_float = 0.;
    let mut y: c_float = 0.;
    let mut i: c_int = (*self_0).super_0.frameCount + bezier * 18 as c_int;
    let mut n: c_int = 0;
    let mut curves: *mut c_float = (*(*self_0).curves).items;
    if value == 0 as c_int as c_float {
        *curves.offset(frame as isize) = (2 as c_int + i) as c_float;
    }
    tmpx = ((time1 - cx1 * 2 as c_int as c_float + cx2) as c_double * 0.03f64) as c_float;
    tmpy = ((value1 - cy1 * 2 as c_int as c_float + cy2) as c_double * 0.03f64) as c_float;
    dddx =
        (((cx1 - cx2) * 3 as c_int as c_float - time1 + time2) as c_double * 0.006f64) as c_float;
    dddy =
        (((cy1 - cy2) * 3 as c_int as c_float - value1 + value2) as c_double * 0.006f64) as c_float;
    ddx = tmpx * 2 as c_int as c_float + dddx;
    ddy = tmpy * 2 as c_int as c_float + dddy;
    dx = ((cx1 - time1) as c_double * 0.3f64 + tmpx as c_double + dddx as c_double * 0.16666667f64)
        as c_float;
    dy = ((cy1 - value1) as c_double * 0.3f64 + tmpy as c_double + dddy as c_double * 0.16666667f64)
        as c_float;
    x = time1 + dx;
    y = value1 + dy;
    n = i + 18 as c_int;
    while i < n {
        *curves.offset(i as isize) = x;
        *curves.offset((i + 1 as c_int) as isize) = y;
        dx += ddx;
        dy += ddy;
        ddx += dddx;
        ddy += dddy;
        x += dx;
        y += dy;
        i += 2 as c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _spCurveTimeline_getBezierValue(
    mut self_0: *mut spCurveTimeline,
    mut time: c_float,
    mut frameIndex: c_int,
    mut valueOffset: c_int,
    mut i: c_int,
) -> c_float {
    let mut curves: *mut c_float = (*(*self_0).curves).items;
    let mut frames: *mut c_float = (*(*self_0).super_0.frames).items;
    let mut x: c_float = 0.;
    let mut y: c_float = 0.;
    let mut n: c_int = 0;
    if *curves.offset(i as isize) > time {
        x = *frames.offset(frameIndex as isize);
        y = *frames.offset((frameIndex + valueOffset) as isize);
        return y
            + (time - x) / (*curves.offset(i as isize) - x)
                * (*curves.offset((i + 1 as c_int) as isize) - y);
    }
    n = i + 18 as c_int;
    i += 2 as c_int;
    while i < n {
        if *curves.offset(i as isize) >= time {
            x = *curves.offset((i - 2 as c_int) as isize);
            y = *curves.offset((i - 1 as c_int) as isize);
            return y
                + (time - x) / (*curves.offset(i as isize) - x)
                    * (*curves.offset((i + 1 as c_int) as isize) - y);
        }
        i += 2 as c_int;
    }
    frameIndex += (*self_0).super_0.frameEntries;
    x = *curves.offset((n - 2 as c_int) as isize);
    y = *curves.offset((n - 1 as c_int) as isize);
    return y
        + (time - x) / (*frames.offset(frameIndex as isize) - x)
            * (*frames.offset((frameIndex + valueOffset) as isize) - y);
}
#[no_mangle]
pub unsafe extern "C" fn spCurveTimeline_setLinear(
    mut self_0: *mut spCurveTimeline,
    mut frame: c_int,
) {
    *((*(*self_0).curves).items).offset(frame as isize) = 0 as c_int as c_float;
}
#[no_mangle]
pub unsafe extern "C" fn spCurveTimeline_setStepped(
    mut self_0: *mut spCurveTimeline,
    mut frame: c_int,
) {
    *((*(*self_0).curves).items).offset(frame as isize) = 1 as c_int as c_float;
}
#[no_mangle]
pub unsafe extern "C" fn spCurveTimeline1_setFrame(
    mut self_0: *mut spCurveTimeline1,
    mut frame: c_int,
    mut time: c_float,
    mut value: c_float,
) {
    let mut frames: *mut c_float = (*(*self_0).super_0.frames).items;
    frame <<= 1 as c_int;
    *frames.offset(frame as isize) = time;
    *frames.offset((frame + 1 as c_int) as isize) = value;
}
#[no_mangle]
pub unsafe extern "C" fn spCurveTimeline1_getCurveValue(
    mut self_0: *mut spCurveTimeline1,
    mut time: c_float,
) -> c_float {
    let mut frames: *mut c_float = (*(*self_0).super_0.frames).items;
    let mut curves: *mut c_float = (*(*self_0).curves).items;
    let mut i: c_int = (*(*self_0).super_0.frames).size - 2 as c_int;
    let mut ii: c_int = 0;
    let mut curveType: c_int = 0;
    ii = 2 as c_int;
    while ii <= i {
        if *frames.offset(ii as isize) > time {
            i = ii - 2 as c_int;
            break;
        } else {
            ii += 2 as c_int;
        }
    }
    curveType = *curves.offset((i >> 1 as c_int) as isize) as c_int;
    match curveType {
        0 => {
            let mut before: c_float = *frames.offset(i as isize);
            let mut value: c_float = *frames.offset((i + 1 as c_int) as isize);
            return value
                + (time - before) / (*frames.offset((i + 2 as c_int) as isize) - before)
                    * (*frames.offset((i + 2 as c_int + 1 as c_int) as isize) - value);
        }
        1 => return *frames.offset((i + 1 as c_int) as isize),
        _ => {}
    }
    return _spCurveTimeline_getBezierValue(self_0, time, i, 1 as c_int, curveType - 2 as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn spCurveTimeline2_setFrame(
    mut self_0: *mut spCurveTimeline1,
    mut frame: c_int,
    mut time: c_float,
    mut value1: c_float,
    mut value2: c_float,
) {
    let mut frames: *mut c_float = (*(*self_0).super_0.frames).items;
    frame *= 3 as c_int;
    *frames.offset(frame as isize) = time;
    *frames.offset((frame + 1 as c_int) as isize) = value1;
    *frames.offset((frame + 2 as c_int) as isize) = value2;
}
#[no_mangle]
pub unsafe extern "C" fn _spRotateTimeline_apply(
    mut timeline: *mut spTimeline,
    mut skeleton: *mut spSkeleton,
    mut _lastTime: c_float,
    mut time: c_float,
    mut _firedEvents: *mut *mut spEvent,
    mut _eventsCount: *mut c_int,
    mut alpha: c_float,
    mut blend: spMixBlend,
    mut _direction: spMixDirection,
) {
    let mut bone: *mut spBone = 0 as *mut spBone;
    let mut r: c_float = 0.;
    let mut self_0: *mut spRotateTimeline = timeline as *mut spRotateTimeline;
    let mut frames: *mut c_float = (*(*self_0).super_0.super_0.frames).items;
    bone = *((*skeleton).bones).offset((*self_0).boneIndex as isize);
    if (*bone).active == 0 {
        return;
    }
    if time < *frames.offset(0 as c_int as isize) {
        match blend as c_uint {
            0 => {
                (*bone).rotation = (*(*bone).data).rotation;
                return;
            }
            1 => {
                (*bone).rotation += ((*(*bone).data).rotation - (*bone).rotation) * alpha;
            }
            _ => {}
        }
        return;
    }
    r = spCurveTimeline1_getCurveValue(&mut (*self_0).super_0, time);
    let mut current_block_14: u64;
    match blend as c_uint {
        0 => {
            (*bone).rotation = (*(*bone).data).rotation + r * alpha;
            current_block_14 = 12039483399334584727;
        }
        1 | 2 => {
            r += (*(*bone).data).rotation - (*bone).rotation;
            current_block_14 = 17903053923152385536;
        }
        3 => {
            current_block_14 = 17903053923152385536;
        }
        _ => {
            current_block_14 = 12039483399334584727;
        }
    }
    match current_block_14 {
        17903053923152385536 => {
            (*bone).rotation += r * alpha;
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn spRotateTimeline_create(
    mut frameCount: c_int,
    mut bezierCount: c_int,
    mut boneIndex: c_int,
) -> *mut spRotateTimeline {
    let mut timeline: *mut spRotateTimeline = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spRotateTimeline>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        355 as c_int,
    ) as *mut spRotateTimeline;
    let mut ids: [spPropertyId; 1] = [0; 1];
    ids[0 as c_int as usize] =
        (SP_PROPERTY_ROTATE as c_int as spPropertyId) << 32 as c_int | boneIndex as c_ulong;
    _spCurveTimeline_init(
        &mut (*timeline).super_0,
        frameCount,
        2 as c_int,
        bezierCount,
        ids.as_mut_ptr(),
        1 as c_int,
        SP_TIMELINE_ROTATE,
        Some(_spCurveTimeline_dispose as unsafe extern "C" fn(*mut spTimeline) -> ()),
        Some(
            _spRotateTimeline_apply
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    *mut spSkeleton,
                    c_float,
                    c_float,
                    *mut *mut spEvent,
                    *mut c_int,
                    c_float,
                    spMixBlend,
                    spMixDirection,
                ) -> (),
        ),
        Some(
            _spCurveTimeline_setBezier
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    c_int,
                    c_int,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                ) -> (),
        ),
    );
    (*timeline).boneIndex = boneIndex;
    return timeline;
}
#[no_mangle]
pub unsafe extern "C" fn spRotateTimeline_setFrame(
    mut self_0: *mut spRotateTimeline,
    mut frame: c_int,
    mut time: c_float,
    mut degrees: c_float,
) {
    spCurveTimeline1_setFrame(&mut (*self_0).super_0, frame, time, degrees);
}
#[no_mangle]
pub unsafe extern "C" fn _spTranslateTimeline_apply(
    mut timeline: *mut spTimeline,
    mut skeleton: *mut spSkeleton,
    mut _lastTime: c_float,
    mut time: c_float,
    mut _firedEvents: *mut *mut spEvent,
    mut _eventsCount: *mut c_int,
    mut alpha: c_float,
    mut blend: spMixBlend,
    mut _direction: spMixDirection,
) {
    let mut bone: *mut spBone = 0 as *mut spBone;
    let mut x: c_float = 0.;
    let mut y: c_float = 0.;
    let mut t: c_float = 0.;
    let mut i: c_int = 0;
    let mut curveType: c_int = 0;
    let mut self_0: *mut spTranslateTimeline = timeline as *mut spTranslateTimeline;
    let mut frames: *mut c_float = (*(*self_0).super_0.super_0.frames).items;
    let mut curves: *mut c_float = (*(*self_0).super_0.curves).items;
    bone = *((*skeleton).bones).offset((*self_0).boneIndex as isize);
    if (*bone).active == 0 {
        return;
    }
    if time < *frames.offset(0 as c_int as isize) {
        match blend as c_uint {
            0 => {
                (*bone).x = (*(*bone).data).x;
                (*bone).y = (*(*bone).data).y;
                return;
            }
            1 => {
                (*bone).x += ((*(*bone).data).x - (*bone).x) * alpha;
                (*bone).y += ((*(*bone).data).y - (*bone).y) * alpha;
            }
            _ => {}
        }
        return;
    }
    i = search2((*self_0).super_0.super_0.frames, time, 3 as c_int);
    curveType = *curves.offset((i / 3 as c_int) as isize) as c_int;
    match curveType {
        0 => {
            let mut before: c_float = *frames.offset(i as isize);
            x = *frames.offset((i + 1 as c_int) as isize);
            y = *frames.offset((i + 2 as c_int) as isize);
            t = (time - before) / (*frames.offset((i + 3 as c_int) as isize) - before);
            x += (*frames.offset((i + 3 as c_int + 1 as c_int) as isize) - x) * t;
            y += (*frames.offset((i + 3 as c_int + 2 as c_int) as isize) - y) * t;
        }
        1 => {
            x = *frames.offset((i + 1 as c_int) as isize);
            y = *frames.offset((i + 2 as c_int) as isize);
        }
        _ => {
            x = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                1 as c_int,
                curveType - 2 as c_int,
            );
            y = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                2 as c_int,
                curveType + 18 as c_int - 2 as c_int,
            );
        }
    }
    match blend as c_uint {
        0 => {
            (*bone).x = (*(*bone).data).x + x * alpha;
            (*bone).y = (*(*bone).data).y + y * alpha;
        }
        1 | 2 => {
            (*bone).x += ((*(*bone).data).x + x - (*bone).x) * alpha;
            (*bone).y += ((*(*bone).data).y + y - (*bone).y) * alpha;
        }
        3 => {
            (*bone).x += x * alpha;
            (*bone).y += y * alpha;
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn spTranslateTimeline_create(
    mut frameCount: c_int,
    mut bezierCount: c_int,
    mut boneIndex: c_int,
) -> *mut spTranslateTimeline {
    let mut timeline: *mut spTranslateTimeline = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spTranslateTimeline>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        445 as c_int,
    ) as *mut spTranslateTimeline;
    let mut ids: [spPropertyId; 2] = [0; 2];
    ids[0 as c_int as usize] =
        (SP_PROPERTY_X as c_int as spPropertyId) << 32 as c_int | boneIndex as c_ulong;
    ids[1 as c_int as usize] =
        (SP_PROPERTY_Y as c_int as spPropertyId) << 32 as c_int | boneIndex as c_ulong;
    _spCurveTimeline_init(
        &mut (*timeline).super_0,
        frameCount,
        3 as c_int,
        bezierCount,
        ids.as_mut_ptr(),
        2 as c_int,
        SP_TIMELINE_TRANSLATE,
        Some(_spCurveTimeline_dispose as unsafe extern "C" fn(*mut spTimeline) -> ()),
        Some(
            _spTranslateTimeline_apply
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    *mut spSkeleton,
                    c_float,
                    c_float,
                    *mut *mut spEvent,
                    *mut c_int,
                    c_float,
                    spMixBlend,
                    spMixDirection,
                ) -> (),
        ),
        Some(
            _spCurveTimeline_setBezier
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    c_int,
                    c_int,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                ) -> (),
        ),
    );
    (*timeline).boneIndex = boneIndex;
    return timeline;
}
#[no_mangle]
pub unsafe extern "C" fn spTranslateTimeline_setFrame(
    mut self_0: *mut spTranslateTimeline,
    mut frame: c_int,
    mut time: c_float,
    mut x: c_float,
    mut y: c_float,
) {
    spCurveTimeline2_setFrame(&mut (*self_0).super_0, frame, time, x, y);
}
#[no_mangle]
pub unsafe extern "C" fn _spTranslateXTimeline_apply(
    mut timeline: *mut spTimeline,
    mut skeleton: *mut spSkeleton,
    mut _lastTime: c_float,
    mut time: c_float,
    mut _firedEvents: *mut *mut spEvent,
    mut _eventsCount: *mut c_int,
    mut alpha: c_float,
    mut blend: spMixBlend,
    mut _direction: spMixDirection,
) {
    let mut bone: *mut spBone = 0 as *mut spBone;
    let mut x: c_float = 0.;
    let mut self_0: *mut spTranslateXTimeline = timeline as *mut spTranslateXTimeline;
    let mut frames: *mut c_float = (*(*self_0).super_0.super_0.frames).items;
    bone = *((*skeleton).bones).offset((*self_0).boneIndex as isize);
    if (*bone).active == 0 {
        return;
    }
    if time < *frames.offset(0 as c_int as isize) {
        match blend as c_uint {
            0 => {
                (*bone).x = (*(*bone).data).x;
                return;
            }
            1 => {
                (*bone).x += ((*(*bone).data).x - (*bone).x) * alpha;
            }
            _ => {}
        }
        return;
    }
    x = spCurveTimeline1_getCurveValue(&mut (*self_0).super_0, time);
    match blend as c_uint {
        0 => {
            (*bone).x = (*(*bone).data).x + x * alpha;
        }
        1 | 2 => {
            (*bone).x += ((*(*bone).data).x + x - (*bone).x) * alpha;
        }
        3 => {
            (*bone).x += x * alpha;
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn spTranslateXTimeline_create(
    mut frameCount: c_int,
    mut bezierCount: c_int,
    mut boneIndex: c_int,
) -> *mut spTranslateXTimeline {
    let mut timeline: *mut spTranslateXTimeline = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spTranslateXTimeline>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        506 as c_int,
    ) as *mut spTranslateXTimeline;
    let mut ids: [spPropertyId; 1] = [0; 1];
    ids[0 as c_int as usize] =
        (SP_PROPERTY_X as c_int as spPropertyId) << 32 as c_int | boneIndex as c_ulong;
    _spCurveTimeline_init(
        &mut (*timeline).super_0,
        frameCount,
        2 as c_int,
        bezierCount,
        ids.as_mut_ptr(),
        1 as c_int,
        SP_TIMELINE_TRANSLATEX,
        Some(_spCurveTimeline_dispose as unsafe extern "C" fn(*mut spTimeline) -> ()),
        Some(
            _spTranslateXTimeline_apply
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    *mut spSkeleton,
                    c_float,
                    c_float,
                    *mut *mut spEvent,
                    *mut c_int,
                    c_float,
                    spMixBlend,
                    spMixDirection,
                ) -> (),
        ),
        Some(
            _spCurveTimeline_setBezier
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    c_int,
                    c_int,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                ) -> (),
        ),
    );
    (*timeline).boneIndex = boneIndex;
    return timeline;
}
#[no_mangle]
pub unsafe extern "C" fn spTranslateXTimeline_setFrame(
    mut self_0: *mut spTranslateXTimeline,
    mut frame: c_int,
    mut time: c_float,
    mut x: c_float,
) {
    spCurveTimeline1_setFrame(&mut (*self_0).super_0, frame, time, x);
}
#[no_mangle]
pub unsafe extern "C" fn _spTranslateYTimeline_apply(
    mut timeline: *mut spTimeline,
    mut skeleton: *mut spSkeleton,
    mut _lastTime: c_float,
    mut time: c_float,
    mut _firedEvents: *mut *mut spEvent,
    mut _eventsCount: *mut c_int,
    mut alpha: c_float,
    mut blend: spMixBlend,
    mut _direction: spMixDirection,
) {
    let mut bone: *mut spBone = 0 as *mut spBone;
    let mut y: c_float = 0.;
    let mut self_0: *mut spTranslateYTimeline = timeline as *mut spTranslateYTimeline;
    let mut frames: *mut c_float = (*(*self_0).super_0.super_0.frames).items;
    bone = *((*skeleton).bones).offset((*self_0).boneIndex as isize);
    if (*bone).active == 0 {
        return;
    }
    if time < *frames.offset(0 as c_int as isize) {
        match blend as c_uint {
            0 => {
                (*bone).y = (*(*bone).data).y;
                return;
            }
            1 => {
                (*bone).y += ((*(*bone).data).y - (*bone).y) * alpha;
            }
            _ => {}
        }
        return;
    }
    y = spCurveTimeline1_getCurveValue(&mut (*self_0).super_0, time);
    match blend as c_uint {
        0 => {
            (*bone).y = (*(*bone).data).y + y * alpha;
        }
        1 | 2 => {
            (*bone).y += ((*(*bone).data).y + y - (*bone).y) * alpha;
        }
        3 => {
            (*bone).y += y * alpha;
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn spTranslateYTimeline_create(
    mut frameCount: c_int,
    mut bezierCount: c_int,
    mut boneIndex: c_int,
) -> *mut spTranslateYTimeline {
    let mut timeline: *mut spTranslateYTimeline = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spTranslateYTimeline>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        566 as c_int,
    ) as *mut spTranslateYTimeline;
    let mut ids: [spPropertyId; 1] = [0; 1];
    ids[0 as c_int as usize] =
        (SP_PROPERTY_Y as c_int as spPropertyId) << 32 as c_int | boneIndex as c_ulong;
    _spCurveTimeline_init(
        &mut (*timeline).super_0,
        frameCount,
        2 as c_int,
        bezierCount,
        ids.as_mut_ptr(),
        1 as c_int,
        SP_TIMELINE_TRANSLATEY,
        Some(_spCurveTimeline_dispose as unsafe extern "C" fn(*mut spTimeline) -> ()),
        Some(
            _spTranslateYTimeline_apply
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    *mut spSkeleton,
                    c_float,
                    c_float,
                    *mut *mut spEvent,
                    *mut c_int,
                    c_float,
                    spMixBlend,
                    spMixDirection,
                ) -> (),
        ),
        Some(
            _spCurveTimeline_setBezier
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    c_int,
                    c_int,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                ) -> (),
        ),
    );
    (*timeline).boneIndex = boneIndex;
    return timeline;
}
#[no_mangle]
pub unsafe extern "C" fn spTranslateYTimeline_setFrame(
    mut self_0: *mut spTranslateYTimeline,
    mut frame: c_int,
    mut time: c_float,
    mut y: c_float,
) {
    spCurveTimeline1_setFrame(&mut (*self_0).super_0, frame, time, y);
}
#[no_mangle]
pub unsafe extern "C" fn _spScaleTimeline_apply(
    mut timeline: *mut spTimeline,
    mut skeleton: *mut spSkeleton,
    mut _lastTime: c_float,
    mut time: c_float,
    mut _firedEvents: *mut *mut spEvent,
    mut _eventsCount: *mut c_int,
    mut alpha: c_float,
    mut blend: spMixBlend,
    mut direction: spMixDirection,
) {
    let mut bone: *mut spBone = 0 as *mut spBone;
    let mut i: c_int = 0;
    let mut curveType: c_int = 0;
    let mut x: c_float = 0.;
    let mut y: c_float = 0.;
    let mut t: c_float = 0.;
    let mut self_0: *mut spScaleTimeline = timeline as *mut spScaleTimeline;
    let mut frames: *mut c_float = (*(*self_0).super_0.super_0.frames).items;
    let mut curves: *mut c_float = (*(*self_0).super_0.curves).items;
    bone = *((*skeleton).bones).offset((*self_0).boneIndex as isize);
    if (*bone).active == 0 {
        return;
    }
    if time < *frames.offset(0 as c_int as isize) {
        match blend as c_uint {
            0 => {
                (*bone).scaleX = (*(*bone).data).scaleX;
                (*bone).scaleY = (*(*bone).data).scaleY;
                return;
            }
            1 => {
                (*bone).scaleX += ((*(*bone).data).scaleX - (*bone).scaleX) * alpha;
                (*bone).scaleY += ((*(*bone).data).scaleY - (*bone).scaleY) * alpha;
            }
            _ => {}
        }
        return;
    }
    i = search2((*self_0).super_0.super_0.frames, time, 3 as c_int);
    curveType = *curves.offset((i / 3 as c_int) as isize) as c_int;
    match curveType {
        0 => {
            let mut before: c_float = *frames.offset(i as isize);
            x = *frames.offset((i + 1 as c_int) as isize);
            y = *frames.offset((i + 2 as c_int) as isize);
            t = (time - before) / (*frames.offset((i + 3 as c_int) as isize) - before);
            x += (*frames.offset((i + 3 as c_int + 1 as c_int) as isize) - x) * t;
            y += (*frames.offset((i + 3 as c_int + 2 as c_int) as isize) - y) * t;
        }
        1 => {
            x = *frames.offset((i + 1 as c_int) as isize);
            y = *frames.offset((i + 2 as c_int) as isize);
        }
        _ => {
            x = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                1 as c_int,
                curveType - 2 as c_int,
            );
            y = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                2 as c_int,
                curveType + 18 as c_int - 2 as c_int,
            );
        }
    }
    x *= (*(*bone).data).scaleX;
    y *= (*(*bone).data).scaleY;
    if alpha == 1 as c_int as c_float {
        if blend as c_uint == SP_MIX_BLEND_ADD as c_int as c_uint {
            (*bone).scaleX += x - (*(*bone).data).scaleX;
            (*bone).scaleY += y - (*(*bone).data).scaleY;
        } else {
            (*bone).scaleX = x;
            (*bone).scaleY = y;
        }
    } else {
        let mut bx: c_float = 0.;
        let mut by: c_float = 0.;
        if direction as c_uint == SP_MIX_DIRECTION_OUT as c_int as c_uint {
            match blend as c_uint {
                0 => {
                    bx = (*(*bone).data).scaleX;
                    by = (*(*bone).data).scaleY;
                    (*bone).scaleX = bx
                        + ((if x < 0 as c_int as c_float { -x } else { x })
                            * (if bx < 0 as c_int as c_float {
                                -1.0f32
                            } else {
                                if bx > 0 as c_int as c_float {
                                    1.0f32
                                } else {
                                    0.0f32
                                }
                            })
                            - bx)
                            * alpha;
                    (*bone).scaleY = by
                        + ((if y < 0 as c_int as c_float { -y } else { y })
                            * (if by < 0 as c_int as c_float {
                                -1.0f32
                            } else {
                                if by > 0 as c_int as c_float {
                                    1.0f32
                                } else {
                                    0.0f32
                                }
                            })
                            - by)
                            * alpha;
                }
                1 | 2 => {
                    bx = (*bone).scaleX;
                    by = (*bone).scaleY;
                    (*bone).scaleX = bx
                        + ((if x < 0 as c_int as c_float { -x } else { x })
                            * (if bx < 0 as c_int as c_float {
                                -1.0f32
                            } else {
                                if bx > 0 as c_int as c_float {
                                    1.0f32
                                } else {
                                    0.0f32
                                }
                            })
                            - bx)
                            * alpha;
                    (*bone).scaleY = by
                        + ((if y < 0 as c_int as c_float { -y } else { y })
                            * (if by < 0 as c_int as c_float {
                                -1.0f32
                            } else {
                                if by > 0 as c_int as c_float {
                                    1.0f32
                                } else {
                                    0.0f32
                                }
                            })
                            - by)
                            * alpha;
                }
                3 => {
                    (*bone).scaleX += (x - (*(*bone).data).scaleX) * alpha;
                    (*bone).scaleY += (y - (*(*bone).data).scaleY) * alpha;
                }
                _ => {}
            }
        } else {
            match blend as c_uint {
                0 => {
                    bx = (if (*(*bone).data).scaleX < 0 as c_int as c_float {
                        -(*(*bone).data).scaleX
                    } else {
                        (*(*bone).data).scaleX
                    }) * (if x < 0 as c_int as c_float {
                        -1.0f32
                    } else {
                        if x > 0 as c_int as c_float {
                            1.0f32
                        } else {
                            0.0f32
                        }
                    });
                    by = (if (*(*bone).data).scaleY < 0 as c_int as c_float {
                        -(*(*bone).data).scaleY
                    } else {
                        (*(*bone).data).scaleY
                    }) * (if y < 0 as c_int as c_float {
                        -1.0f32
                    } else {
                        if y > 0 as c_int as c_float {
                            1.0f32
                        } else {
                            0.0f32
                        }
                    });
                    (*bone).scaleX = bx + (x - bx) * alpha;
                    (*bone).scaleY = by + (y - by) * alpha;
                }
                1 | 2 => {
                    bx = (if (*bone).scaleX < 0 as c_int as c_float {
                        -(*bone).scaleX
                    } else {
                        (*bone).scaleX
                    }) * (if x < 0 as c_int as c_float {
                        -1.0f32
                    } else {
                        if x > 0 as c_int as c_float {
                            1.0f32
                        } else {
                            0.0f32
                        }
                    });
                    by = (if (*bone).scaleY < 0 as c_int as c_float {
                        -(*bone).scaleY
                    } else {
                        (*bone).scaleY
                    }) * (if y < 0 as c_int as c_float {
                        -1.0f32
                    } else {
                        if y > 0 as c_int as c_float {
                            1.0f32
                        } else {
                            0.0f32
                        }
                    });
                    (*bone).scaleX = bx + (x - bx) * alpha;
                    (*bone).scaleY = by + (y - by) * alpha;
                }
                3 => {
                    (*bone).scaleX += (x - (*(*bone).data).scaleX) * alpha;
                    (*bone).scaleY += (y - (*(*bone).data).scaleY) * alpha;
                }
                _ => {}
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn spScaleTimeline_create(
    mut frameCount: c_int,
    mut bezierCount: c_int,
    mut boneIndex: c_int,
) -> *mut spScaleTimeline {
    let mut timeline: *mut spScaleTimeline = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spScaleTimeline>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        691 as c_int,
    ) as *mut spScaleTimeline;
    let mut ids: [spPropertyId; 2] = [0; 2];
    ids[0 as c_int as usize] =
        (SP_PROPERTY_SCALEX as c_int as spPropertyId) << 32 as c_int | boneIndex as c_ulong;
    ids[1 as c_int as usize] =
        (SP_PROPERTY_SCALEY as c_int as spPropertyId) << 32 as c_int | boneIndex as c_ulong;
    _spCurveTimeline_init(
        &mut (*timeline).super_0,
        frameCount,
        3 as c_int,
        bezierCount,
        ids.as_mut_ptr(),
        2 as c_int,
        SP_TIMELINE_SCALE,
        Some(_spCurveTimeline_dispose as unsafe extern "C" fn(*mut spTimeline) -> ()),
        Some(
            _spScaleTimeline_apply
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    *mut spSkeleton,
                    c_float,
                    c_float,
                    *mut *mut spEvent,
                    *mut c_int,
                    c_float,
                    spMixBlend,
                    spMixDirection,
                ) -> (),
        ),
        Some(
            _spCurveTimeline_setBezier
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    c_int,
                    c_int,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                ) -> (),
        ),
    );
    (*timeline).boneIndex = boneIndex;
    return timeline;
}
#[no_mangle]
pub unsafe extern "C" fn spScaleTimeline_setFrame(
    mut self_0: *mut spScaleTimeline,
    mut frame: c_int,
    mut time: c_float,
    mut x: c_float,
    mut y: c_float,
) {
    spCurveTimeline2_setFrame(&mut (*self_0).super_0, frame, time, x, y);
}
#[no_mangle]
pub unsafe extern "C" fn _spScaleXTimeline_apply(
    mut timeline: *mut spTimeline,
    mut skeleton: *mut spSkeleton,
    mut _lastTime: c_float,
    mut time: c_float,
    mut _firedEvents: *mut *mut spEvent,
    mut _eventsCount: *mut c_int,
    mut alpha: c_float,
    mut blend: spMixBlend,
    mut direction: spMixDirection,
) {
    let mut bone: *mut spBone = 0 as *mut spBone;
    let mut x: c_float = 0.;
    let mut self_0: *mut spScaleXTimeline = timeline as *mut spScaleXTimeline;
    let mut frames: *mut c_float = (*(*self_0).super_0.super_0.frames).items;
    bone = *((*skeleton).bones).offset((*self_0).boneIndex as isize);
    if (*bone).active == 0 {
        return;
    }
    if time < *frames.offset(0 as c_int as isize) {
        match blend as c_uint {
            0 => {
                (*bone).scaleX = (*(*bone).data).scaleX;
                return;
            }
            1 => {
                (*bone).scaleX += ((*(*bone).data).scaleX - (*bone).scaleX) * alpha;
            }
            _ => {}
        }
        return;
    }
    x = spCurveTimeline1_getCurveValue(&mut (*self_0).super_0, time) * (*(*bone).data).scaleX;
    if alpha == 1 as c_int as c_float {
        if blend as c_uint == SP_MIX_BLEND_ADD as c_int as c_uint {
            (*bone).scaleX += x - (*(*bone).data).scaleX;
        } else {
            (*bone).scaleX = x;
        }
    } else {
        let mut bx: c_float = 0.;
        if direction as c_uint == SP_MIX_DIRECTION_OUT as c_int as c_uint {
            match blend as c_uint {
                0 => {
                    bx = (*(*bone).data).scaleX;
                    (*bone).scaleX = bx
                        + ((if x < 0 as c_int as c_float { -x } else { x })
                            * (if bx < 0 as c_int as c_float {
                                -1.0f32
                            } else {
                                if bx > 0 as c_int as c_float {
                                    1.0f32
                                } else {
                                    0.0f32
                                }
                            })
                            - bx)
                            * alpha;
                }
                1 | 2 => {
                    bx = (*bone).scaleX;
                    (*bone).scaleX = bx
                        + ((if x < 0 as c_int as c_float { -x } else { x })
                            * (if bx < 0 as c_int as c_float {
                                -1.0f32
                            } else {
                                if bx > 0 as c_int as c_float {
                                    1.0f32
                                } else {
                                    0.0f32
                                }
                            })
                            - bx)
                            * alpha;
                }
                3 => {
                    (*bone).scaleX += (x - (*(*bone).data).scaleX) * alpha;
                }
                _ => {}
            }
        } else {
            match blend as c_uint {
                0 => {
                    bx = (if (*(*bone).data).scaleX < 0 as c_int as c_float {
                        -(*(*bone).data).scaleX
                    } else {
                        (*(*bone).data).scaleX
                    }) * (if x < 0 as c_int as c_float {
                        -1.0f32
                    } else {
                        if x > 0 as c_int as c_float {
                            1.0f32
                        } else {
                            0.0f32
                        }
                    });
                    (*bone).scaleX = bx + (x - bx) * alpha;
                }
                1 | 2 => {
                    bx = (if (*bone).scaleX < 0 as c_int as c_float {
                        -(*bone).scaleX
                    } else {
                        (*bone).scaleX
                    }) * (if x < 0 as c_int as c_float {
                        -1.0f32
                    } else {
                        if x > 0 as c_int as c_float {
                            1.0f32
                        } else {
                            0.0f32
                        }
                    });
                    (*bone).scaleX = bx + (x - bx) * alpha;
                }
                3 => {
                    (*bone).scaleX += (x - (*(*bone).data).scaleX) * alpha;
                }
                _ => {}
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn spScaleXTimeline_create(
    mut frameCount: c_int,
    mut bezierCount: c_int,
    mut boneIndex: c_int,
) -> *mut spScaleXTimeline {
    let mut timeline: *mut spScaleXTimeline = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spScaleXTimeline>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        778 as c_int,
    ) as *mut spScaleXTimeline;
    let mut ids: [spPropertyId; 1] = [0; 1];
    ids[0 as c_int as usize] =
        (SP_PROPERTY_SCALEX as c_int as spPropertyId) << 32 as c_int | boneIndex as c_ulong;
    _spCurveTimeline_init(
        &mut (*timeline).super_0,
        frameCount,
        2 as c_int,
        bezierCount,
        ids.as_mut_ptr(),
        1 as c_int,
        SP_TIMELINE_SCALEX,
        Some(_spCurveTimeline_dispose as unsafe extern "C" fn(*mut spTimeline) -> ()),
        Some(
            _spScaleXTimeline_apply
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    *mut spSkeleton,
                    c_float,
                    c_float,
                    *mut *mut spEvent,
                    *mut c_int,
                    c_float,
                    spMixBlend,
                    spMixDirection,
                ) -> (),
        ),
        Some(
            _spCurveTimeline_setBezier
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    c_int,
                    c_int,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                ) -> (),
        ),
    );
    (*timeline).boneIndex = boneIndex;
    return timeline;
}
#[no_mangle]
pub unsafe extern "C" fn spScaleXTimeline_setFrame(
    mut self_0: *mut spScaleXTimeline,
    mut frame: c_int,
    mut time: c_float,
    mut y: c_float,
) {
    spCurveTimeline1_setFrame(&mut (*self_0).super_0, frame, time, y);
}
#[no_mangle]
pub unsafe extern "C" fn _spScaleYTimeline_apply(
    mut timeline: *mut spTimeline,
    mut skeleton: *mut spSkeleton,
    mut _lastTime: c_float,
    mut time: c_float,
    mut _firedEvents: *mut *mut spEvent,
    mut _eventsCount: *mut c_int,
    mut alpha: c_float,
    mut blend: spMixBlend,
    mut direction: spMixDirection,
) {
    let mut bone: *mut spBone = 0 as *mut spBone;
    let mut y: c_float = 0.;
    let mut self_0: *mut spScaleYTimeline = timeline as *mut spScaleYTimeline;
    let mut frames: *mut c_float = (*(*self_0).super_0.super_0.frames).items;
    bone = *((*skeleton).bones).offset((*self_0).boneIndex as isize);
    if (*bone).active == 0 {
        return;
    }
    if time < *frames.offset(0 as c_int as isize) {
        match blend as c_uint {
            0 => {
                (*bone).scaleY = (*(*bone).data).scaleY;
                return;
            }
            1 => {
                (*bone).scaleY += ((*(*bone).data).scaleY - (*bone).scaleY) * alpha;
            }
            _ => {}
        }
        return;
    }
    y = spCurveTimeline1_getCurveValue(&mut (*self_0).super_0, time) * (*(*bone).data).scaleY;
    if alpha == 1 as c_int as c_float {
        if blend as c_uint == SP_MIX_BLEND_ADD as c_int as c_uint {
            (*bone).scaleY += y - (*(*bone).data).scaleY;
        } else {
            (*bone).scaleY = y;
        }
    } else {
        let mut by: c_float = 0 as c_int as c_float;
        if direction as c_uint == SP_MIX_DIRECTION_OUT as c_int as c_uint {
            match blend as c_uint {
                0 => {
                    by = (*(*bone).data).scaleY;
                    (*bone).scaleY = by
                        + ((if y < 0 as c_int as c_float { -y } else { y })
                            * (if by < 0 as c_int as c_float {
                                -1.0f32
                            } else {
                                if by > 0 as c_int as c_float {
                                    1.0f32
                                } else {
                                    0.0f32
                                }
                            })
                            - by)
                            * alpha;
                }
                1 | 2 => {
                    by = (*bone).scaleY;
                    (*bone).scaleY = by
                        + ((if y < 0 as c_int as c_float { -y } else { y })
                            * (if by < 0 as c_int as c_float {
                                -1.0f32
                            } else {
                                if by > 0 as c_int as c_float {
                                    1.0f32
                                } else {
                                    0.0f32
                                }
                            })
                            - by)
                            * alpha;
                }
                3 => {
                    (*bone).scaleY += (y - (*(*bone).data).scaleY) * alpha;
                }
                _ => {}
            }
        } else {
            match blend as c_uint {
                0 => {
                    by = (if (*(*bone).data).scaleY < 0 as c_int as c_float {
                        -(*(*bone).data).scaleY
                    } else {
                        (*(*bone).data).scaleY
                    }) * (if y < 0 as c_int as c_float {
                        -1.0f32
                    } else {
                        if y > 0 as c_int as c_float {
                            1.0f32
                        } else {
                            0.0f32
                        }
                    });
                    (*bone).scaleY = by + (y - by) * alpha;
                }
                1 | 2 => {
                    by = (if (*bone).scaleY < 0 as c_int as c_float {
                        -(*bone).scaleY
                    } else {
                        (*bone).scaleY
                    }) * (if y < 0 as c_int as c_float {
                        -1.0f32
                    } else {
                        if y > 0 as c_int as c_float {
                            1.0f32
                        } else {
                            0.0f32
                        }
                    });
                    (*bone).scaleY = by + (y - by) * alpha;
                }
                3 => {
                    (*bone).scaleY += (y - (*(*bone).data).scaleY) * alpha;
                }
                _ => {}
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn spScaleYTimeline_create(
    mut frameCount: c_int,
    mut bezierCount: c_int,
    mut boneIndex: c_int,
) -> *mut spScaleYTimeline {
    let mut timeline: *mut spScaleYTimeline = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spScaleYTimeline>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        864 as c_int,
    ) as *mut spScaleYTimeline;
    let mut ids: [spPropertyId; 1] = [0; 1];
    ids[0 as c_int as usize] =
        (SP_PROPERTY_SCALEY as c_int as spPropertyId) << 32 as c_int | boneIndex as c_ulong;
    _spCurveTimeline_init(
        &mut (*timeline).super_0,
        frameCount,
        2 as c_int,
        bezierCount,
        ids.as_mut_ptr(),
        1 as c_int,
        SP_TIMELINE_SCALEY,
        Some(_spCurveTimeline_dispose as unsafe extern "C" fn(*mut spTimeline) -> ()),
        Some(
            _spScaleYTimeline_apply
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    *mut spSkeleton,
                    c_float,
                    c_float,
                    *mut *mut spEvent,
                    *mut c_int,
                    c_float,
                    spMixBlend,
                    spMixDirection,
                ) -> (),
        ),
        Some(
            _spCurveTimeline_setBezier
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    c_int,
                    c_int,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                ) -> (),
        ),
    );
    (*timeline).boneIndex = boneIndex;
    return timeline;
}
#[no_mangle]
pub unsafe extern "C" fn spScaleYTimeline_setFrame(
    mut self_0: *mut spScaleYTimeline,
    mut frame: c_int,
    mut time: c_float,
    mut y: c_float,
) {
    spCurveTimeline1_setFrame(&mut (*self_0).super_0, frame, time, y);
}
#[no_mangle]
pub unsafe extern "C" fn _spShearTimeline_apply(
    mut timeline: *mut spTimeline,
    mut skeleton: *mut spSkeleton,
    mut _lastTime: c_float,
    mut time: c_float,
    mut _firedEvents: *mut *mut spEvent,
    mut _eventsCount: *mut c_int,
    mut alpha: c_float,
    mut blend: spMixBlend,
    mut _direction: spMixDirection,
) {
    let mut bone: *mut spBone = 0 as *mut spBone;
    let mut x: c_float = 0.;
    let mut y: c_float = 0.;
    let mut t: c_float = 0.;
    let mut i: c_int = 0;
    let mut curveType: c_int = 0;
    let mut self_0: *mut spShearTimeline = timeline as *mut spShearTimeline;
    let mut frames: *mut c_float = (*(*self_0).super_0.super_0.frames).items;
    let mut curves: *mut c_float = (*(*self_0).super_0.curves).items;
    bone = *((*skeleton).bones).offset((*self_0).boneIndex as isize);
    if (*bone).active == 0 {
        return;
    }
    if time < *frames.offset(0 as c_int as isize) {
        match blend as c_uint {
            0 => {
                (*bone).shearX = (*(*bone).data).shearX;
                (*bone).shearY = (*(*bone).data).shearY;
                return;
            }
            1 => {
                (*bone).shearX += ((*(*bone).data).shearX - (*bone).shearX) * alpha;
                (*bone).shearY += ((*(*bone).data).shearY - (*bone).shearY) * alpha;
            }
            _ => {}
        }
        return;
    }
    i = search2((*self_0).super_0.super_0.frames, time, 3 as c_int);
    curveType = *curves.offset((i / 3 as c_int) as isize) as c_int;
    match curveType {
        0 => {
            let mut before: c_float = *frames.offset(i as isize);
            x = *frames.offset((i + 1 as c_int) as isize);
            y = *frames.offset((i + 2 as c_int) as isize);
            t = (time - before) / (*frames.offset((i + 3 as c_int) as isize) - before);
            x += (*frames.offset((i + 3 as c_int + 1 as c_int) as isize) - x) * t;
            y += (*frames.offset((i + 3 as c_int + 2 as c_int) as isize) - y) * t;
        }
        1 => {
            x = *frames.offset((i + 1 as c_int) as isize);
            y = *frames.offset((i + 2 as c_int) as isize);
        }
        _ => {
            x = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                1 as c_int,
                curveType - 2 as c_int,
            );
            y = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                2 as c_int,
                curveType + 18 as c_int - 2 as c_int,
            );
        }
    }
    match blend as c_uint {
        0 => {
            (*bone).shearX = (*(*bone).data).shearX + x * alpha;
            (*bone).shearY = (*(*bone).data).shearY + y * alpha;
        }
        1 | 2 => {
            (*bone).shearX += ((*(*bone).data).shearX + x - (*bone).shearX) * alpha;
            (*bone).shearY += ((*(*bone).data).shearY + y - (*bone).shearY) * alpha;
        }
        3 => {
            (*bone).shearX += x * alpha;
            (*bone).shearY += y * alpha;
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn spShearTimeline_create(
    mut frameCount: c_int,
    mut bezierCount: c_int,
    mut boneIndex: c_int,
) -> *mut spShearTimeline {
    let mut timeline: *mut spShearTimeline = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spShearTimeline>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        952 as c_int,
    ) as *mut spShearTimeline;
    let mut ids: [spPropertyId; 2] = [0; 2];
    ids[0 as c_int as usize] =
        (SP_PROPERTY_SHEARX as c_int as spPropertyId) << 32 as c_int | boneIndex as c_ulong;
    ids[1 as c_int as usize] =
        (SP_PROPERTY_SHEARY as c_int as spPropertyId) << 32 as c_int | boneIndex as c_ulong;
    _spCurveTimeline_init(
        &mut (*timeline).super_0,
        frameCount,
        3 as c_int,
        bezierCount,
        ids.as_mut_ptr(),
        2 as c_int,
        SP_TIMELINE_SHEAR,
        Some(_spCurveTimeline_dispose as unsafe extern "C" fn(*mut spTimeline) -> ()),
        Some(
            _spShearTimeline_apply
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    *mut spSkeleton,
                    c_float,
                    c_float,
                    *mut *mut spEvent,
                    *mut c_int,
                    c_float,
                    spMixBlend,
                    spMixDirection,
                ) -> (),
        ),
        Some(
            _spCurveTimeline_setBezier
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    c_int,
                    c_int,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                ) -> (),
        ),
    );
    (*timeline).boneIndex = boneIndex;
    return timeline;
}
#[no_mangle]
pub unsafe extern "C" fn spShearTimeline_setFrame(
    mut self_0: *mut spShearTimeline,
    mut frame: c_int,
    mut time: c_float,
    mut x: c_float,
    mut y: c_float,
) {
    spCurveTimeline2_setFrame(&mut (*self_0).super_0, frame, time, x, y);
}
#[no_mangle]
pub unsafe extern "C" fn _spShearXTimeline_apply(
    mut timeline: *mut spTimeline,
    mut skeleton: *mut spSkeleton,
    mut _lastTime: c_float,
    mut time: c_float,
    mut _firedEvents: *mut *mut spEvent,
    mut _eventsCount: *mut c_int,
    mut alpha: c_float,
    mut blend: spMixBlend,
    mut _direction: spMixDirection,
) {
    let mut bone: *mut spBone = 0 as *mut spBone;
    let mut x: c_float = 0.;
    let mut self_0: *mut spShearXTimeline = timeline as *mut spShearXTimeline;
    let mut frames: *mut c_float = (*(*self_0).super_0.super_0.frames).items;
    bone = *((*skeleton).bones).offset((*self_0).boneIndex as isize);
    if (*bone).active == 0 {
        return;
    }
    if time < *frames.offset(0 as c_int as isize) {
        match blend as c_uint {
            0 => {
                (*bone).shearX = (*(*bone).data).shearX;
                return;
            }
            1 => {
                (*bone).shearX += ((*(*bone).data).shearX - (*bone).shearX) * alpha;
            }
            _ => {}
        }
        return;
    }
    x = spCurveTimeline1_getCurveValue(&mut (*self_0).super_0, time);
    match blend as c_uint {
        0 => {
            (*bone).shearX = (*(*bone).data).shearX + x * alpha;
        }
        1 | 2 => {
            (*bone).shearX += ((*(*bone).data).shearX + x - (*bone).shearX) * alpha;
        }
        3 => {
            (*bone).shearX += x * alpha;
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn spShearXTimeline_create(
    mut frameCount: c_int,
    mut bezierCount: c_int,
    mut boneIndex: c_int,
) -> *mut spShearXTimeline {
    let mut timeline: *mut spShearXTimeline = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spShearXTimeline>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        1012 as c_int,
    ) as *mut spShearXTimeline;
    let mut ids: [spPropertyId; 1] = [0; 1];
    ids[0 as c_int as usize] =
        (SP_PROPERTY_SHEARX as c_int as spPropertyId) << 32 as c_int | boneIndex as c_ulong;
    _spCurveTimeline_init(
        &mut (*timeline).super_0,
        frameCount,
        2 as c_int,
        bezierCount,
        ids.as_mut_ptr(),
        1 as c_int,
        SP_TIMELINE_SHEARX,
        Some(_spCurveTimeline_dispose as unsafe extern "C" fn(*mut spTimeline) -> ()),
        Some(
            _spShearXTimeline_apply
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    *mut spSkeleton,
                    c_float,
                    c_float,
                    *mut *mut spEvent,
                    *mut c_int,
                    c_float,
                    spMixBlend,
                    spMixDirection,
                ) -> (),
        ),
        Some(
            _spCurveTimeline_setBezier
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    c_int,
                    c_int,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                ) -> (),
        ),
    );
    (*timeline).boneIndex = boneIndex;
    return timeline;
}
#[no_mangle]
pub unsafe extern "C" fn spShearXTimeline_setFrame(
    mut self_0: *mut spShearXTimeline,
    mut frame: c_int,
    mut time: c_float,
    mut x: c_float,
) {
    spCurveTimeline1_setFrame(&mut (*self_0).super_0, frame, time, x);
}
#[no_mangle]
pub unsafe extern "C" fn _spShearYTimeline_apply(
    mut timeline: *mut spTimeline,
    mut skeleton: *mut spSkeleton,
    mut _lastTime: c_float,
    mut time: c_float,
    mut _firedEvents: *mut *mut spEvent,
    mut _eventsCount: *mut c_int,
    mut alpha: c_float,
    mut blend: spMixBlend,
    mut _direction: spMixDirection,
) {
    let mut bone: *mut spBone = 0 as *mut spBone;
    let mut y: c_float = 0.;
    let mut self_0: *mut spShearYTimeline = timeline as *mut spShearYTimeline;
    let mut frames: *mut c_float = (*(*self_0).super_0.super_0.frames).items;
    bone = *((*skeleton).bones).offset((*self_0).boneIndex as isize);
    if (*bone).active == 0 {
        return;
    }
    if time < *frames.offset(0 as c_int as isize) {
        match blend as c_uint {
            0 => {
                (*bone).shearY = (*(*bone).data).shearY;
                return;
            }
            1 => {
                (*bone).shearY += ((*(*bone).data).shearY - (*bone).shearY) * alpha;
            }
            _ => {}
        }
        return;
    }
    y = spCurveTimeline1_getCurveValue(&mut (*self_0).super_0, time);
    match blend as c_uint {
        0 => {
            (*bone).shearY = (*(*bone).data).shearY + y * alpha;
        }
        1 | 2 => {
            (*bone).shearY += ((*(*bone).data).shearY + y - (*bone).shearY) * alpha;
        }
        3 => {
            (*bone).shearY += y * alpha;
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn spShearYTimeline_create(
    mut frameCount: c_int,
    mut bezierCount: c_int,
    mut boneIndex: c_int,
) -> *mut spShearYTimeline {
    let mut timeline: *mut spShearYTimeline = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spShearYTimeline>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        1072 as c_int,
    ) as *mut spShearYTimeline;
    let mut ids: [spPropertyId; 1] = [0; 1];
    ids[0 as c_int as usize] =
        (SP_PROPERTY_SHEARY as c_int as spPropertyId) << 32 as c_int | boneIndex as c_ulong;
    _spCurveTimeline_init(
        &mut (*timeline).super_0,
        frameCount,
        2 as c_int,
        bezierCount,
        ids.as_mut_ptr(),
        1 as c_int,
        SP_TIMELINE_SHEARY,
        Some(_spCurveTimeline_dispose as unsafe extern "C" fn(*mut spTimeline) -> ()),
        Some(
            _spShearYTimeline_apply
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    *mut spSkeleton,
                    c_float,
                    c_float,
                    *mut *mut spEvent,
                    *mut c_int,
                    c_float,
                    spMixBlend,
                    spMixDirection,
                ) -> (),
        ),
        Some(
            _spCurveTimeline_setBezier
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    c_int,
                    c_int,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                ) -> (),
        ),
    );
    (*timeline).boneIndex = boneIndex;
    return timeline;
}
#[no_mangle]
pub unsafe extern "C" fn spShearYTimeline_setFrame(
    mut self_0: *mut spShearYTimeline,
    mut frame: c_int,
    mut time: c_float,
    mut y: c_float,
) {
    spCurveTimeline1_setFrame(&mut (*self_0).super_0, frame, time, y);
}
static mut RGBA_ENTRIES: c_int = 5 as c_int;
static mut COLOR_R: c_int = 1 as c_int;
static mut COLOR_G: c_int = 2 as c_int;
static mut COLOR_B: c_int = 3 as c_int;
static mut COLOR_A: c_int = 4 as c_int;
#[no_mangle]
pub unsafe extern "C" fn _spRGBATimeline_apply(
    mut timeline: *mut spTimeline,
    mut skeleton: *mut spSkeleton,
    mut _lastTime: c_float,
    mut time: c_float,
    mut _firedEvents: *mut *mut spEvent,
    mut _eventsCount: *mut c_int,
    mut alpha: c_float,
    mut blend: spMixBlend,
    mut _direction: spMixDirection,
) {
    let mut slot: *mut spSlot = 0 as *mut spSlot;
    let mut i: c_int = 0;
    let mut curveType: c_int = 0;
    let mut r: c_float = 0.;
    let mut g: c_float = 0.;
    let mut b: c_float = 0.;
    let mut a: c_float = 0.;
    let mut t: c_float = 0.;
    let mut color: *mut spColor = 0 as *mut spColor;
    let mut setup: *mut spColor = 0 as *mut spColor;
    let mut self_0: *mut spRGBATimeline = timeline as *mut spRGBATimeline;
    let mut frames: *mut c_float = (*(*self_0).super_0.super_0.frames).items;
    let mut curves: *mut c_float = (*(*self_0).super_0.curves).items;
    slot = *((*skeleton).slots).offset((*self_0).slotIndex as isize);
    if (*(*slot).bone).active == 0 {
        return;
    }
    if time < *frames.offset(0 as c_int as isize) {
        color = &mut (*slot).color;
        setup = &mut (*(*slot).data).color;
        match blend as c_uint {
            0 => {
                spColor_setFromColor(color, setup);
                return;
            }
            1 => {
                spColor_addFloats(
                    color,
                    ((*setup).r - (*color).r) * alpha,
                    ((*setup).g - (*color).g) * alpha,
                    ((*setup).b - (*color).b) * alpha,
                    ((*setup).a - (*color).a) * alpha,
                );
            }
            _ => {}
        }
        return;
    }
    i = search2((*self_0).super_0.super_0.frames, time, RGBA_ENTRIES);
    curveType = *curves.offset((i / RGBA_ENTRIES) as isize) as c_int;
    match curveType {
        0 => {
            let mut before: c_float = *frames.offset(i as isize);
            r = *frames.offset((i + COLOR_R) as isize);
            g = *frames.offset((i + COLOR_G) as isize);
            b = *frames.offset((i + COLOR_B) as isize);
            a = *frames.offset((i + COLOR_A) as isize);
            t = (time - before) / (*frames.offset((i + RGBA_ENTRIES) as isize) - before);
            r += (*frames.offset((i + RGBA_ENTRIES + COLOR_R) as isize) - r) * t;
            g += (*frames.offset((i + RGBA_ENTRIES + COLOR_G) as isize) - g) * t;
            b += (*frames.offset((i + RGBA_ENTRIES + COLOR_B) as isize) - b) * t;
            a += (*frames.offset((i + RGBA_ENTRIES + COLOR_A) as isize) - a) * t;
        }
        1 => {
            r = *frames.offset((i + COLOR_R) as isize);
            g = *frames.offset((i + COLOR_G) as isize);
            b = *frames.offset((i + COLOR_B) as isize);
            a = *frames.offset((i + COLOR_A) as isize);
        }
        _ => {
            r = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                COLOR_R,
                curveType - 2 as c_int,
            );
            g = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                COLOR_G,
                curveType + 18 as c_int - 2 as c_int,
            );
            b = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                COLOR_B,
                curveType + 18 as c_int * 2 as c_int - 2 as c_int,
            );
            a = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                COLOR_A,
                curveType + 18 as c_int * 3 as c_int - 2 as c_int,
            );
        }
    }
    color = &mut (*slot).color;
    if alpha == 1 as c_int as c_float {
        spColor_setFromFloats(color, r, g, b, a);
    } else {
        if blend as c_uint == SP_MIX_BLEND_SETUP as c_int as c_uint {
            spColor_setFromColor(color, &mut (*(*slot).data).color);
        }
        spColor_addFloats(
            color,
            (r - (*color).r) * alpha,
            (g - (*color).g) * alpha,
            (b - (*color).b) * alpha,
            (a - (*color).a) * alpha,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn spRGBATimeline_create(
    mut framesCount: c_int,
    mut bezierCount: c_int,
    mut slotIndex: c_int,
) -> *mut spRGBATimeline {
    let mut timeline: *mut spRGBATimeline = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spRGBATimeline>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        1169 as c_int,
    ) as *mut spRGBATimeline;
    let mut ids: [spPropertyId; 2] = [0; 2];
    ids[0 as c_int as usize] =
        (SP_PROPERTY_RGB as c_int as spPropertyId) << 32 as c_int | slotIndex as c_ulong;
    ids[1 as c_int as usize] =
        (SP_PROPERTY_ALPHA as c_int as spPropertyId) << 32 as c_int | slotIndex as c_ulong;
    _spCurveTimeline_init(
        &mut (*timeline).super_0,
        framesCount,
        RGBA_ENTRIES,
        bezierCount,
        ids.as_mut_ptr(),
        2 as c_int,
        SP_TIMELINE_RGBA,
        Some(_spCurveTimeline_dispose as unsafe extern "C" fn(*mut spTimeline) -> ()),
        Some(
            _spRGBATimeline_apply
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    *mut spSkeleton,
                    c_float,
                    c_float,
                    *mut *mut spEvent,
                    *mut c_int,
                    c_float,
                    spMixBlend,
                    spMixDirection,
                ) -> (),
        ),
        Some(
            _spCurveTimeline_setBezier
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    c_int,
                    c_int,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                ) -> (),
        ),
    );
    (*timeline).slotIndex = slotIndex;
    return timeline;
}
#[no_mangle]
pub unsafe extern "C" fn spRGBATimeline_setFrame(
    mut self_0: *mut spRGBATimeline,
    mut frame: c_int,
    mut time: c_float,
    mut r: c_float,
    mut g: c_float,
    mut b: c_float,
    mut a: c_float,
) {
    let mut frames: *mut c_float = (*(*self_0).super_0.super_0.frames).items;
    frame *= RGBA_ENTRIES;
    *frames.offset(frame as isize) = time;
    *frames.offset((frame + COLOR_R) as isize) = r;
    *frames.offset((frame + COLOR_G) as isize) = g;
    *frames.offset((frame + COLOR_B) as isize) = b;
    *frames.offset((frame + COLOR_A) as isize) = a;
}
#[no_mangle]
pub unsafe extern "C" fn _spRGBTimeline_apply(
    mut timeline: *mut spTimeline,
    mut skeleton: *mut spSkeleton,
    mut _lastTime: c_float,
    mut time: c_float,
    mut _firedEvents: *mut *mut spEvent,
    mut _eventsCount: *mut c_int,
    mut alpha: c_float,
    mut blend: spMixBlend,
    mut _direction: spMixDirection,
) {
    let mut slot: *mut spSlot = 0 as *mut spSlot;
    let mut i: c_int = 0;
    let mut curveType: c_int = 0;
    let mut r: c_float = 0.;
    let mut g: c_float = 0.;
    let mut b: c_float = 0.;
    let mut t: c_float = 0.;
    let mut color: *mut spColor = 0 as *mut spColor;
    let mut setup: *mut spColor = 0 as *mut spColor;
    let mut self_0: *mut spRGBTimeline = timeline as *mut spRGBTimeline;
    let mut frames: *mut c_float = (*(*self_0).super_0.super_0.frames).items;
    let mut curves: *mut c_float = (*(*self_0).super_0.curves).items;
    slot = *((*skeleton).slots).offset((*self_0).slotIndex as isize);
    if (*(*slot).bone).active == 0 {
        return;
    }
    if time < *frames.offset(0 as c_int as isize) {
        color = &mut (*slot).color;
        setup = &mut (*(*slot).data).color;
        match blend as c_uint {
            0 => {
                spColor_setFromColor(color, setup);
                return;
            }
            1 => {
                spColor_addFloats(
                    color,
                    ((*setup).r - (*color).r) * alpha,
                    ((*setup).g - (*color).g) * alpha,
                    ((*setup).b - (*color).b) * alpha,
                    ((*setup).a - (*color).a) * alpha,
                );
            }
            _ => {}
        }
        return;
    }
    i = search2((*self_0).super_0.super_0.frames, time, 4 as c_int);
    curveType = *curves.offset((i / 4 as c_int) as isize) as c_int;
    match curveType {
        0 => {
            let mut before: c_float = *frames.offset(i as isize);
            r = *frames.offset((i + COLOR_R) as isize);
            g = *frames.offset((i + COLOR_G) as isize);
            b = *frames.offset((i + COLOR_B) as isize);
            t = (time - before) / (*frames.offset((i + 4 as c_int) as isize) - before);
            r += (*frames.offset((i + 4 as c_int + COLOR_R) as isize) - r) * t;
            g += (*frames.offset((i + 4 as c_int + COLOR_G) as isize) - g) * t;
            b += (*frames.offset((i + 4 as c_int + COLOR_B) as isize) - b) * t;
        }
        1 => {
            r = *frames.offset((i + COLOR_R) as isize);
            g = *frames.offset((i + COLOR_G) as isize);
            b = *frames.offset((i + COLOR_B) as isize);
        }
        _ => {
            r = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                COLOR_R,
                curveType - 2 as c_int,
            );
            g = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                COLOR_G,
                curveType + 18 as c_int - 2 as c_int,
            );
            b = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                COLOR_B,
                curveType + 18 as c_int * 2 as c_int - 2 as c_int,
            );
        }
    }
    color = &mut (*slot).color;
    if alpha == 1 as c_int as c_float {
        (*color).r = r;
        (*color).g = g;
        (*color).b = b;
    } else {
        if blend as c_uint == SP_MIX_BLEND_SETUP as c_int as c_uint {
            (*color).r = (*(*slot).data).color.r;
            (*color).g = (*(*slot).data).color.g;
            (*color).b = (*(*slot).data).color.b;
        }
        (*color).r += (r - (*color).r) * alpha;
        (*color).g += (g - (*color).g) * alpha;
        (*color).b += (b - (*color).b) * alpha;
    };
}
#[no_mangle]
pub unsafe extern "C" fn spRGBTimeline_create(
    mut framesCount: c_int,
    mut bezierCount: c_int,
    mut slotIndex: c_int,
) -> *mut spRGBTimeline {
    let mut timeline: *mut spRGBTimeline = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spRGBTimeline>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        1275 as c_int,
    ) as *mut spRGBTimeline;
    let mut ids: [spPropertyId; 1] = [0; 1];
    ids[0 as c_int as usize] =
        (SP_PROPERTY_RGB as c_int as spPropertyId) << 32 as c_int | slotIndex as c_ulong;
    _spCurveTimeline_init(
        &mut (*timeline).super_0,
        framesCount,
        4 as c_int,
        bezierCount,
        ids.as_mut_ptr(),
        1 as c_int,
        SP_TIMELINE_RGB,
        Some(_spCurveTimeline_dispose as unsafe extern "C" fn(*mut spTimeline) -> ()),
        Some(
            _spRGBTimeline_apply
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    *mut spSkeleton,
                    c_float,
                    c_float,
                    *mut *mut spEvent,
                    *mut c_int,
                    c_float,
                    spMixBlend,
                    spMixDirection,
                ) -> (),
        ),
        Some(
            _spCurveTimeline_setBezier
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    c_int,
                    c_int,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                ) -> (),
        ),
    );
    (*timeline).slotIndex = slotIndex;
    return timeline;
}
#[no_mangle]
pub unsafe extern "C" fn spRGBTimeline_setFrame(
    mut self_0: *mut spRGBTimeline,
    mut frame: c_int,
    mut time: c_float,
    mut r: c_float,
    mut g: c_float,
    mut b: c_float,
) {
    let mut frames: *mut c_float = (*(*self_0).super_0.super_0.frames).items;
    frame *= 4 as c_int;
    *frames.offset(frame as isize) = time;
    *frames.offset((frame + COLOR_R) as isize) = r;
    *frames.offset((frame + COLOR_G) as isize) = g;
    *frames.offset((frame + COLOR_B) as isize) = b;
}
#[no_mangle]
pub unsafe extern "C" fn _spAlphaTimeline_apply(
    mut timeline: *mut spTimeline,
    mut skeleton: *mut spSkeleton,
    mut _lastTime: c_float,
    mut time: c_float,
    mut _firedEvents: *mut *mut spEvent,
    mut _eventsCount: *mut c_int,
    mut alpha: c_float,
    mut blend: spMixBlend,
    mut _direction: spMixDirection,
) {
    let mut slot: *mut spSlot = 0 as *mut spSlot;
    let mut a: c_float = 0.;
    let mut color: *mut spColor = 0 as *mut spColor;
    let mut setup: *mut spColor = 0 as *mut spColor;
    let mut self_0: *mut spAlphaTimeline = timeline as *mut spAlphaTimeline;
    let mut frames: *mut c_float = (*(*self_0).super_0.super_0.frames).items;
    slot = *((*skeleton).slots).offset((*self_0).slotIndex as isize);
    if (*(*slot).bone).active == 0 {
        return;
    }
    if time < *frames.offset(0 as c_int as isize) {
        color = &mut (*slot).color;
        setup = &mut (*(*slot).data).color;
        match blend as c_uint {
            0 => {
                (*color).a = (*setup).a;
                return;
            }
            1 => {
                (*color).a += ((*setup).a - (*color).a) * alpha;
            }
            _ => {}
        }
        return;
    }
    a = spCurveTimeline1_getCurveValue(&mut (*self_0).super_0, time);
    if alpha == 1 as c_int as c_float {
        (*slot).color.a = a;
    } else {
        if blend as c_uint == SP_MIX_BLEND_SETUP as c_int as c_uint {
            (*slot).color.a = (*(*slot).data).color.a;
        }
        (*slot).color.a += (a - (*slot).color.a) * alpha;
    };
}
#[no_mangle]
pub unsafe extern "C" fn spAlphaTimeline_create(
    mut frameCount: c_int,
    mut bezierCount: c_int,
    mut slotIndex: c_int,
) -> *mut spAlphaTimeline {
    let mut timeline: *mut spAlphaTimeline = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spAlphaTimeline>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        1338 as c_int,
    ) as *mut spAlphaTimeline;
    let mut ids: [spPropertyId; 1] = [0; 1];
    ids[0 as c_int as usize] =
        (SP_PROPERTY_ALPHA as c_int as spPropertyId) << 32 as c_int | slotIndex as c_ulong;
    _spCurveTimeline_init(
        &mut (*timeline).super_0,
        frameCount,
        2 as c_int,
        bezierCount,
        ids.as_mut_ptr(),
        1 as c_int,
        SP_TIMELINE_ALPHA,
        Some(_spCurveTimeline_dispose as unsafe extern "C" fn(*mut spTimeline) -> ()),
        Some(
            _spAlphaTimeline_apply
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    *mut spSkeleton,
                    c_float,
                    c_float,
                    *mut *mut spEvent,
                    *mut c_int,
                    c_float,
                    spMixBlend,
                    spMixDirection,
                ) -> (),
        ),
        Some(
            _spCurveTimeline_setBezier
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    c_int,
                    c_int,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                ) -> (),
        ),
    );
    (*timeline).slotIndex = slotIndex;
    return timeline;
}
#[no_mangle]
pub unsafe extern "C" fn spAlphaTimeline_setFrame(
    mut self_0: *mut spAlphaTimeline,
    mut frame: c_int,
    mut time: c_float,
    mut alpha: c_float,
) {
    spCurveTimeline1_setFrame(&mut (*self_0).super_0, frame, time, alpha);
}
static mut RGBA2_ENTRIES: c_int = 8 as c_int;
static mut COLOR_R2: c_int = 5 as c_int;
static mut COLOR_G2: c_int = 6 as c_int;
static mut COLOR_B2: c_int = 7 as c_int;
#[no_mangle]
pub unsafe extern "C" fn _spRGBA2Timeline_apply(
    mut timeline: *mut spTimeline,
    mut skeleton: *mut spSkeleton,
    mut _lastTime: c_float,
    mut time: c_float,
    mut _firedEvents: *mut *mut spEvent,
    mut _eventsCount: *mut c_int,
    mut alpha: c_float,
    mut blend: spMixBlend,
    mut _direction: spMixDirection,
) {
    let mut slot: *mut spSlot = 0 as *mut spSlot;
    let mut i: c_int = 0;
    let mut curveType: c_int = 0;
    let mut r: c_float = 0.;
    let mut g: c_float = 0.;
    let mut b: c_float = 0.;
    let mut a: c_float = 0.;
    let mut r2: c_float = 0.;
    let mut g2: c_float = 0.;
    let mut b2: c_float = 0.;
    let mut t: c_float = 0.;
    let mut light: *mut spColor = 0 as *mut spColor;
    let mut setupLight: *mut spColor = 0 as *mut spColor;
    let mut dark: *mut spColor = 0 as *mut spColor;
    let mut setupDark: *mut spColor = 0 as *mut spColor;
    let mut self_0: *mut spRGBA2Timeline = timeline as *mut spRGBA2Timeline;
    let mut frames: *mut c_float = (*(*self_0).super_0.super_0.frames).items;
    let mut curves: *mut c_float = (*(*self_0).super_0.curves).items;
    slot = *((*skeleton).slots).offset((*self_0).slotIndex as isize);
    if (*(*slot).bone).active == 0 {
        return;
    }
    if time < *frames.offset(0 as c_int as isize) {
        light = &mut (*slot).color;
        dark = (*slot).darkColor;
        setupLight = &mut (*(*slot).data).color;
        setupDark = (*(*slot).data).darkColor;
        match blend as c_uint {
            0 => {
                spColor_setFromColor(light, setupLight);
                spColor_setFromFloats3(dark, (*setupDark).r, (*setupDark).g, (*setupDark).b);
                return;
            }
            1 => {
                spColor_addFloats(
                    light,
                    ((*setupLight).r - (*light).r) * alpha,
                    ((*setupLight).g - (*light).g) * alpha,
                    ((*setupLight).b - (*light).b) * alpha,
                    ((*setupLight).a - (*light).a) * alpha,
                );
                (*dark).r += ((*setupDark).r - (*dark).r) * alpha;
                (*dark).g += ((*setupDark).g - (*dark).g) * alpha;
                (*dark).b += ((*setupDark).b - (*dark).b) * alpha;
            }
            _ => {}
        }
        return;
    }
    r = 0 as c_int as c_float;
    g = 0 as c_int as c_float;
    b = 0 as c_int as c_float;
    a = 0 as c_int as c_float;
    r2 = 0 as c_int as c_float;
    g2 = 0 as c_int as c_float;
    b2 = 0 as c_int as c_float;
    i = search2((*self_0).super_0.super_0.frames, time, RGBA2_ENTRIES);
    curveType = *curves.offset((i / RGBA2_ENTRIES) as isize) as c_int;
    match curveType {
        0 => {
            let mut before: c_float = *frames.offset(i as isize);
            r = *frames.offset((i + COLOR_R) as isize);
            g = *frames.offset((i + COLOR_G) as isize);
            b = *frames.offset((i + COLOR_B) as isize);
            a = *frames.offset((i + COLOR_A) as isize);
            r2 = *frames.offset((i + COLOR_R2) as isize);
            g2 = *frames.offset((i + COLOR_G2) as isize);
            b2 = *frames.offset((i + COLOR_B2) as isize);
            t = (time - before) / (*frames.offset((i + RGBA2_ENTRIES) as isize) - before);
            r += (*frames.offset((i + RGBA2_ENTRIES + COLOR_R) as isize) - r) * t;
            g += (*frames.offset((i + RGBA2_ENTRIES + COLOR_G) as isize) - g) * t;
            b += (*frames.offset((i + RGBA2_ENTRIES + COLOR_B) as isize) - b) * t;
            a += (*frames.offset((i + RGBA2_ENTRIES + COLOR_A) as isize) - a) * t;
            r2 += (*frames.offset((i + RGBA2_ENTRIES + COLOR_R2) as isize) - r2) * t;
            g2 += (*frames.offset((i + RGBA2_ENTRIES + COLOR_G2) as isize) - g2) * t;
            b2 += (*frames.offset((i + RGBA2_ENTRIES + COLOR_B2) as isize) - b2) * t;
        }
        1 => {
            r = *frames.offset((i + COLOR_R) as isize);
            g = *frames.offset((i + COLOR_G) as isize);
            b = *frames.offset((i + COLOR_B) as isize);
            a = *frames.offset((i + COLOR_A) as isize);
            r2 = *frames.offset((i + COLOR_R2) as isize);
            g2 = *frames.offset((i + COLOR_G2) as isize);
            b2 = *frames.offset((i + COLOR_B2) as isize);
        }
        _ => {
            r = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                COLOR_R,
                curveType - 2 as c_int,
            );
            g = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                COLOR_G,
                curveType + 18 as c_int - 2 as c_int,
            );
            b = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                COLOR_B,
                curveType + 18 as c_int * 2 as c_int - 2 as c_int,
            );
            a = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                COLOR_A,
                curveType + 18 as c_int * 3 as c_int - 2 as c_int,
            );
            r2 = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                COLOR_R2,
                curveType + 18 as c_int * 4 as c_int - 2 as c_int,
            );
            g2 = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                COLOR_G2,
                curveType + 18 as c_int * 5 as c_int - 2 as c_int,
            );
            b2 = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                COLOR_B2,
                curveType + 18 as c_int * 6 as c_int - 2 as c_int,
            );
        }
    }
    light = &mut (*slot).color;
    dark = (*slot).darkColor;
    if alpha == 1 as c_int as c_float {
        spColor_setFromFloats(light, r, g, b, a);
        spColor_setFromFloats3(dark, r2, g2, b2);
    } else {
        if blend as c_uint == SP_MIX_BLEND_SETUP as c_int as c_uint {
            spColor_setFromColor(light, &mut (*(*slot).data).color);
            spColor_setFromColor(dark, (*(*slot).data).darkColor);
        }
        spColor_addFloats(
            light,
            (r - (*light).r) * alpha,
            (g - (*light).g) * alpha,
            (b - (*light).b) * alpha,
            (a - (*light).a) * alpha,
        );
        (*dark).r += (r2 - (*dark).r) * alpha;
        (*dark).g += (g2 - (*dark).g) * alpha;
        (*dark).b += (b2 - (*dark).b) * alpha;
    };
}
#[no_mangle]
pub unsafe extern "C" fn spRGBA2Timeline_create(
    mut framesCount: c_int,
    mut bezierCount: c_int,
    mut slotIndex: c_int,
) -> *mut spRGBA2Timeline {
    let mut timeline: *mut spRGBA2Timeline = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spRGBA2Timeline>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        1465 as c_int,
    ) as *mut spRGBA2Timeline;
    let mut ids: [spPropertyId; 3] = [0; 3];
    ids[0 as c_int as usize] =
        (SP_PROPERTY_RGB as c_int as spPropertyId) << 32 as c_int | slotIndex as c_ulong;
    ids[1 as c_int as usize] =
        (SP_PROPERTY_ALPHA as c_int as spPropertyId) << 32 as c_int | slotIndex as c_ulong;
    ids[2 as c_int as usize] =
        (SP_PROPERTY_RGB2 as c_int as spPropertyId) << 32 as c_int | slotIndex as c_ulong;
    _spCurveTimeline_init(
        &mut (*timeline).super_0,
        framesCount,
        RGBA2_ENTRIES,
        bezierCount,
        ids.as_mut_ptr(),
        3 as c_int,
        SP_TIMELINE_RGBA2,
        Some(_spCurveTimeline_dispose as unsafe extern "C" fn(*mut spTimeline) -> ()),
        Some(
            _spRGBA2Timeline_apply
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    *mut spSkeleton,
                    c_float,
                    c_float,
                    *mut *mut spEvent,
                    *mut c_int,
                    c_float,
                    spMixBlend,
                    spMixDirection,
                ) -> (),
        ),
        Some(
            _spCurveTimeline_setBezier
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    c_int,
                    c_int,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                ) -> (),
        ),
    );
    (*timeline).slotIndex = slotIndex;
    return timeline;
}
#[no_mangle]
pub unsafe extern "C" fn spRGBA2Timeline_setFrame(
    mut self_0: *mut spRGBA2Timeline,
    mut frame: c_int,
    mut time: c_float,
    mut r: c_float,
    mut g: c_float,
    mut b: c_float,
    mut a: c_float,
    mut r2: c_float,
    mut g2: c_float,
    mut b2: c_float,
) {
    let mut frames: *mut c_float = (*(*self_0).super_0.super_0.frames).items;
    frame *= RGBA2_ENTRIES;
    *frames.offset(frame as isize) = time;
    *frames.offset((frame + COLOR_R) as isize) = r;
    *frames.offset((frame + COLOR_G) as isize) = g;
    *frames.offset((frame + COLOR_B) as isize) = b;
    *frames.offset((frame + COLOR_A) as isize) = a;
    *frames.offset((frame + COLOR_R2) as isize) = r2;
    *frames.offset((frame + COLOR_G2) as isize) = g2;
    *frames.offset((frame + COLOR_B2) as isize) = b2;
}
static mut RGB2_ENTRIES: c_int = 7 as c_int;
static mut COLOR2_R2: c_int = 5 as c_int;
static mut COLOR2_G2: c_int = 6 as c_int;
static mut COLOR2_B2: c_int = 7 as c_int;
#[no_mangle]
pub unsafe extern "C" fn _spRGB2Timeline_apply(
    mut timeline: *mut spTimeline,
    mut skeleton: *mut spSkeleton,
    mut _lastTime: c_float,
    mut time: c_float,
    mut _firedEvents: *mut *mut spEvent,
    mut _eventsCount: *mut c_int,
    mut alpha: c_float,
    mut blend: spMixBlend,
    mut _direction: spMixDirection,
) {
    let mut slot: *mut spSlot = 0 as *mut spSlot;
    let mut i: c_int = 0;
    let mut curveType: c_int = 0;
    let mut r: c_float = 0.;
    let mut g: c_float = 0.;
    let mut b: c_float = 0.;
    let mut r2: c_float = 0.;
    let mut g2: c_float = 0.;
    let mut b2: c_float = 0.;
    let mut t: c_float = 0.;
    let mut light: *mut spColor = 0 as *mut spColor;
    let mut setupLight: *mut spColor = 0 as *mut spColor;
    let mut dark: *mut spColor = 0 as *mut spColor;
    let mut setupDark: *mut spColor = 0 as *mut spColor;
    let mut self_0: *mut spRGB2Timeline = timeline as *mut spRGB2Timeline;
    let mut frames: *mut c_float = (*(*self_0).super_0.super_0.frames).items;
    let mut curves: *mut c_float = (*(*self_0).super_0.curves).items;
    slot = *((*skeleton).slots).offset((*self_0).slotIndex as isize);
    if (*(*slot).bone).active == 0 {
        return;
    }
    if time < *frames.offset(0 as c_int as isize) {
        light = &mut (*slot).color;
        dark = (*slot).darkColor;
        setupLight = &mut (*(*slot).data).color;
        setupDark = (*(*slot).data).darkColor;
        match blend as c_uint {
            0 => {
                spColor_setFromColor3(light, setupLight);
                spColor_setFromColor3(dark, setupDark);
                return;
            }
            1 => {
                spColor_addFloats3(
                    light,
                    ((*setupLight).r - (*light).r) * alpha,
                    ((*setupLight).g - (*light).g) * alpha,
                    ((*setupLight).b - (*light).b) * alpha,
                );
                (*dark).r += ((*setupDark).r - (*dark).r) * alpha;
                (*dark).g += ((*setupDark).g - (*dark).g) * alpha;
                (*dark).b += ((*setupDark).b - (*dark).b) * alpha;
            }
            _ => {}
        }
        return;
    }
    r = 0 as c_int as c_float;
    g = 0 as c_int as c_float;
    b = 0 as c_int as c_float;
    r2 = 0 as c_int as c_float;
    g2 = 0 as c_int as c_float;
    b2 = 0 as c_int as c_float;
    i = search2((*self_0).super_0.super_0.frames, time, RGB2_ENTRIES);
    curveType = *curves.offset((i / RGB2_ENTRIES) as isize) as c_int;
    match curveType {
        0 => {
            let mut before: c_float = *frames.offset(i as isize);
            r = *frames.offset((i + COLOR_R) as isize);
            g = *frames.offset((i + COLOR_G) as isize);
            b = *frames.offset((i + COLOR_B) as isize);
            r2 = *frames.offset((i + COLOR2_R2) as isize);
            g2 = *frames.offset((i + COLOR2_G2) as isize);
            b2 = *frames.offset((i + COLOR2_B2) as isize);
            t = (time - before) / (*frames.offset((i + RGB2_ENTRIES) as isize) - before);
            r += (*frames.offset((i + RGB2_ENTRIES + COLOR_R) as isize) - r) * t;
            g += (*frames.offset((i + RGB2_ENTRIES + COLOR_G) as isize) - g) * t;
            b += (*frames.offset((i + RGB2_ENTRIES + COLOR_B) as isize) - b) * t;
            r2 += (*frames.offset((i + RGB2_ENTRIES + COLOR2_R2) as isize) - r2) * t;
            g2 += (*frames.offset((i + RGB2_ENTRIES + COLOR2_G2) as isize) - g2) * t;
            b2 += (*frames.offset((i + RGB2_ENTRIES + COLOR2_B2) as isize) - b2) * t;
        }
        1 => {
            r = *frames.offset((i + COLOR_R) as isize);
            g = *frames.offset((i + COLOR_G) as isize);
            b = *frames.offset((i + COLOR_B) as isize);
            r2 = *frames.offset((i + COLOR2_R2) as isize);
            g2 = *frames.offset((i + COLOR2_G2) as isize);
            b2 = *frames.offset((i + COLOR2_B2) as isize);
        }
        _ => {
            r = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                COLOR_R,
                curveType - 2 as c_int,
            );
            g = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                COLOR_G,
                curveType + 18 as c_int - 2 as c_int,
            );
            b = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                COLOR_B,
                curveType + 18 as c_int * 2 as c_int - 2 as c_int,
            );
            r2 = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                COLOR2_R2,
                curveType + 18 as c_int * 3 as c_int - 2 as c_int,
            );
            g2 = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                COLOR2_G2,
                curveType + 18 as c_int * 4 as c_int - 2 as c_int,
            );
            b2 = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                COLOR2_B2,
                curveType + 18 as c_int * 5 as c_int - 2 as c_int,
            );
        }
    }
    light = &mut (*slot).color;
    dark = (*slot).darkColor;
    if alpha == 1 as c_int as c_float {
        spColor_setFromFloats3(light, r, g, b);
        spColor_setFromFloats3(dark, r2, g2, b2);
    } else {
        if blend as c_uint == SP_MIX_BLEND_SETUP as c_int as c_uint {
            spColor_setFromColor3(light, &mut (*(*slot).data).color);
            spColor_setFromColor3(dark, (*(*slot).data).darkColor);
        }
        spColor_addFloats3(
            light,
            (r - (*light).r) * alpha,
            (g - (*light).g) * alpha,
            (b - (*light).b) * alpha,
        );
        (*dark).r += (r2 - (*dark).r) * alpha;
        (*dark).g += (g2 - (*dark).g) * alpha;
        (*dark).b += (b2 - (*dark).b) * alpha;
    };
}
#[no_mangle]
pub unsafe extern "C" fn spRGB2Timeline_create(
    mut framesCount: c_int,
    mut bezierCount: c_int,
    mut slotIndex: c_int,
) -> *mut spRGB2Timeline {
    let mut timeline: *mut spRGB2Timeline = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spRGB2Timeline>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        1598 as c_int,
    ) as *mut spRGB2Timeline;
    let mut ids: [spPropertyId; 2] = [0; 2];
    ids[0 as c_int as usize] =
        (SP_PROPERTY_RGB as c_int as spPropertyId) << 32 as c_int | slotIndex as c_ulong;
    ids[1 as c_int as usize] =
        (SP_PROPERTY_RGB2 as c_int as spPropertyId) << 32 as c_int | slotIndex as c_ulong;
    _spCurveTimeline_init(
        &mut (*timeline).super_0,
        framesCount,
        RGB2_ENTRIES,
        bezierCount,
        ids.as_mut_ptr(),
        2 as c_int,
        SP_TIMELINE_RGB2,
        Some(_spCurveTimeline_dispose as unsafe extern "C" fn(*mut spTimeline) -> ()),
        Some(
            _spRGB2Timeline_apply
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    *mut spSkeleton,
                    c_float,
                    c_float,
                    *mut *mut spEvent,
                    *mut c_int,
                    c_float,
                    spMixBlend,
                    spMixDirection,
                ) -> (),
        ),
        Some(
            _spCurveTimeline_setBezier
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    c_int,
                    c_int,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                ) -> (),
        ),
    );
    (*timeline).slotIndex = slotIndex;
    return timeline;
}
#[no_mangle]
pub unsafe extern "C" fn spRGB2Timeline_setFrame(
    mut self_0: *mut spRGB2Timeline,
    mut frame: c_int,
    mut time: c_float,
    mut r: c_float,
    mut g: c_float,
    mut b: c_float,
    mut r2: c_float,
    mut g2: c_float,
    mut b2: c_float,
) {
    let mut frames: *mut c_float = (*(*self_0).super_0.super_0.frames).items;
    frame *= RGB2_ENTRIES;
    *frames.offset(frame as isize) = time;
    *frames.offset((frame + COLOR_R) as isize) = r;
    *frames.offset((frame + COLOR_G) as isize) = g;
    *frames.offset((frame + COLOR_B) as isize) = b;
    *frames.offset((frame + COLOR2_R2) as isize) = r2;
    *frames.offset((frame + COLOR2_G2) as isize) = g2;
    *frames.offset((frame + COLOR2_B2) as isize) = b2;
}
unsafe extern "C" fn _spSetAttachment(
    mut timeline: *mut spAttachmentTimeline,
    mut skeleton: *mut spSkeleton,
    mut slot: *mut spSlot,
    mut attachmentName: *const c_char,
) {
    spSlot_setAttachment(
        slot,
        if attachmentName.is_null() {
            0 as *mut spAttachment
        } else {
            spSkeleton_getAttachmentForSlotIndex(skeleton, (*timeline).slotIndex, attachmentName)
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn _spAttachmentTimeline_apply(
    mut timeline: *mut spTimeline,
    mut skeleton: *mut spSkeleton,
    mut _lastTime: c_float,
    mut time: c_float,
    mut _firedEvents: *mut *mut spEvent,
    mut _eventsCount: *mut c_int,
    mut _alpha: c_float,
    mut blend: spMixBlend,
    mut direction: spMixDirection,
) {
    let mut attachmentName: *const c_char = 0 as *const c_char;
    let mut self_0: *mut spAttachmentTimeline = timeline as *mut spAttachmentTimeline;
    let mut frames: *mut c_float = (*(*self_0).super_0.frames).items;
    let mut slot: *mut spSlot = *((*skeleton).slots).offset((*self_0).slotIndex as isize);
    if (*(*slot).bone).active == 0 {
        return;
    }
    if direction as c_uint == SP_MIX_DIRECTION_OUT as c_int as c_uint {
        if blend as c_uint == SP_MIX_BLEND_SETUP as c_int as c_uint {
            _spSetAttachment(self_0, skeleton, slot, (*(*slot).data).attachmentName);
        }
        return;
    }
    if time < *frames.offset(0 as c_int as isize) {
        if blend as c_uint == SP_MIX_BLEND_SETUP as c_int as c_uint
            || blend as c_uint == SP_MIX_BLEND_FIRST as c_int as c_uint
        {
            _spSetAttachment(self_0, skeleton, slot, (*(*slot).data).attachmentName);
        }
        return;
    }
    if time < *frames.offset(0 as c_int as isize) {
        if blend as c_uint == SP_MIX_BLEND_SETUP as c_int as c_uint
            || blend as c_uint == SP_MIX_BLEND_FIRST as c_int as c_uint
        {
            _spSetAttachment(self_0, skeleton, slot, (*(*slot).data).attachmentName);
        }
        return;
    }
    attachmentName =
        *((*self_0).attachmentNames).offset(search((*self_0).super_0.frames, time) as isize);
    _spSetAttachment(self_0, skeleton, slot, attachmentName);
}
#[no_mangle]
pub unsafe extern "C" fn _spAttachmentTimeline_dispose(mut timeline: *mut spTimeline) {
    let mut self_0: *mut spAttachmentTimeline = timeline as *mut spAttachmentTimeline;
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*(*self_0).super_0.frames).size {
        _spFree(*((*self_0).attachmentNames).offset(i as isize) as *mut c_void);
        i += 1;
    }
    _spFree((*self_0).attachmentNames as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spAttachmentTimeline_create(
    mut framesCount: c_int,
    mut slotIndex: c_int,
) -> *mut spAttachmentTimeline {
    let mut self_0: *mut spAttachmentTimeline = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spAttachmentTimeline>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        1675 as c_int,
    ) as *mut spAttachmentTimeline;
    let mut ids: [spPropertyId; 1] = [0; 1];
    ids[0 as c_int as usize] =
        (SP_PROPERTY_ATTACHMENT as c_int as spPropertyId) << 32 as c_int | slotIndex as c_ulong;
    _spTimeline_init(
        &mut (*self_0).super_0,
        framesCount,
        1 as c_int,
        ids.as_mut_ptr(),
        1 as c_int,
        SP_TIMELINE_ATTACHMENT,
        Some(_spAttachmentTimeline_dispose as unsafe extern "C" fn(*mut spTimeline) -> ()),
        Some(
            _spAttachmentTimeline_apply
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    *mut spSkeleton,
                    c_float,
                    c_float,
                    *mut *mut spEvent,
                    *mut c_int,
                    c_float,
                    spMixBlend,
                    spMixDirection,
                ) -> (),
        ),
        None,
    );
    let ref mut fresh4 =
        *(&(*self_0).attachmentNames as *const *mut *const c_char as *mut *mut *mut c_char);
    *fresh4 = _spCalloc(
        framesCount as size_t,
        ::core::mem::size_of::<*mut c_char>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        1680 as c_int,
    ) as *mut *mut c_char;
    (*self_0).slotIndex = slotIndex;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spAttachmentTimeline_setFrame(
    mut self_0: *mut spAttachmentTimeline,
    mut frame: c_int,
    mut time: c_float,
    mut attachmentName: *const c_char,
) {
    *((*(*self_0).super_0.frames).items).offset(frame as isize) = time;
    _spFree(*((*self_0).attachmentNames).offset(frame as isize) as *mut c_void);
    if !attachmentName.is_null() {
        let ref mut fresh5 = *(&mut *((*self_0).attachmentNames).offset(frame as isize)
            as *mut *const c_char as *mut *mut c_char);
        *fresh5 = _spMalloc(
            (::core::mem::size_of::<c_char>() as c_ulong)
                .wrapping_mul((spine_strlen(attachmentName)).wrapping_add(1 as c_int as c_ulong)),
            b"spine.c\0" as *const u8 as *const c_char,
            1690 as c_int,
        ) as *mut c_char;
        spine_strcpy(*fresh5, attachmentName);
    } else {
        let ref mut fresh6 = *((*self_0).attachmentNames).offset(frame as isize);
        *fresh6 = 0 as *const c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn _spDeformTimeline_setBezier(
    mut timeline: *mut spTimeline,
    mut bezier: c_int,
    mut frame: c_int,
    mut value: c_float,
    mut time1: c_float,
    mut _value1: c_float,
    mut cx1: c_float,
    mut cy1: c_float,
    mut cx2: c_float,
    mut cy2: c_float,
    mut time2: c_float,
    mut _value2: c_float,
) {
    let mut self_0: *mut spDeformTimeline = timeline as *mut spDeformTimeline;
    let mut n: c_int = 0;
    let mut i: c_int = (*self_0).super_0.super_0.frameCount + bezier * 18 as c_int;
    let mut curves: *mut c_float = (*(*self_0).super_0.curves).items;
    let mut tmpx: c_float =
        ((time1 - cx1 * 2 as c_int as c_float + cx2) as c_double * 0.03f64) as c_float;
    let mut tmpy: c_float = (cy2 as c_double * 0.03f64 - cy1 as c_double * 0.06f64) as c_float;
    let mut dddx: c_float =
        (((cx1 - cx2) * 3 as c_int as c_float - time1 + time2) as c_double * 0.006f64) as c_float;
    let mut dddy: c_float = (((cy1 - cy2) as c_double + 0.33333333f64) * 0.018f64) as c_float;
    let mut ddx: c_float = tmpx * 2 as c_int as c_float + dddx;
    let mut ddy: c_float = tmpy * 2 as c_int as c_float + dddy;
    let mut dx: c_float = ((cx1 - time1) as c_double * 0.3f64
        + tmpx as c_double
        + dddx as c_double * 0.16666667f64) as c_float;
    let mut dy: c_float =
        (cy1 as c_double * 0.3f64 + tmpy as c_double + dddy as c_double * 0.16666667f64) as c_float;
    let mut x: c_float = time1 + dx;
    let mut y: c_float = dy;
    if value == 0 as c_int as c_float {
        *curves.offset(frame as isize) = (2 as c_int + i) as c_float;
    }
    n = i + 18 as c_int;
    while i < n {
        *curves.offset(i as isize) = x;
        *curves.offset((i + 1 as c_int) as isize) = y;
        dx += ddx;
        dy += ddy;
        ddx += dddx;
        ddy += dddy;
        x += dx;
        y += dy;
        i += 2 as c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _spDeformTimeline_getCurvePercent(
    mut self_0: *mut spDeformTimeline,
    mut time: c_float,
    mut frame: c_int,
) -> c_float {
    let mut curves: *mut c_float = (*(*self_0).super_0.curves).items;
    let mut frames: *mut c_float = (*(*self_0).super_0.super_0.frames).items;
    let mut n: c_int = 0;
    let mut i: c_int = *curves.offset(frame as isize) as c_int;
    let mut frameEntries: c_int = (*self_0).super_0.super_0.frameEntries;
    let mut x: c_float = 0.;
    let mut y: c_float = 0.;
    match i {
        0 => {
            x = *frames.offset(frame as isize);
            return (time - x) / (*frames.offset((frame + frameEntries) as isize) - x);
        }
        1 => return 0 as c_int as c_float,
        _ => {}
    }
    i -= 2 as c_int;
    if *curves.offset(i as isize) > time {
        x = *frames.offset(frame as isize);
        return *curves.offset((i + 1 as c_int) as isize) * (time - x)
            / (*curves.offset(i as isize) - x);
    }
    n = i + 18 as c_int;
    i += 2 as c_int;
    while i < n {
        if *curves.offset(i as isize) >= time {
            x = *curves.offset((i - 2 as c_int) as isize);
            y = *curves.offset((i - 1 as c_int) as isize);
            return y
                + (time - x) / (*curves.offset(i as isize) - x)
                    * (*curves.offset((i + 1 as c_int) as isize) - y);
        }
        i += 2 as c_int;
    }
    x = *curves.offset((n - 2 as c_int) as isize);
    y = *curves.offset((n - 1 as c_int) as isize);
    return y
        + (1 as c_int as c_float - y) * (time - x)
            / (*frames.offset((frame + frameEntries) as isize) - x);
}
#[no_mangle]
pub unsafe extern "C" fn _spDeformTimeline_apply(
    mut timeline: *mut spTimeline,
    mut skeleton: *mut spSkeleton,
    mut _lastTime: c_float,
    mut time: c_float,
    mut _firedEvents: *mut *mut spEvent,
    mut _eventsCount: *mut c_int,
    mut alpha: c_float,
    mut blend: spMixBlend,
    mut _direction: spMixDirection,
) {
    let mut frame: c_int = 0;
    let mut i: c_int = 0;
    let mut vertexCount: c_int = 0;
    let mut percent: c_float = 0.;
    let mut prevVertices: *const c_float = 0 as *const c_float;
    let mut nextVertices: *const c_float = 0 as *const c_float;
    let mut frames: *mut c_float = 0 as *mut c_float;
    let mut framesCount: c_int = 0;
    let mut frameVertices: *mut *const c_float = 0 as *mut *const c_float;
    let mut deformArray: *mut c_float = 0 as *mut c_float;
    let mut self_0: *mut spDeformTimeline = timeline as *mut spDeformTimeline;
    let mut slot: *mut spSlot = *((*skeleton).slots).offset((*self_0).slotIndex as isize);
    if (*(*slot).bone).active == 0 {
        return;
    }
    if ((*slot).attachment).is_null() {
        return;
    }
    match (*(*slot).attachment).type_0 as c_uint {
        1 | 6 | 2 | 4 => {
            let mut vertexAttachment: *mut spVertexAttachment =
                (*slot).attachment as *mut spVertexAttachment;
            if (*vertexAttachment).timelineAttachment != (*self_0).attachment {
                return;
            }
        }
        _ => return,
    }
    frames = (*(*self_0).super_0.super_0.frames).items;
    framesCount = (*(*self_0).super_0.super_0.frames).size;
    vertexCount = (*self_0).frameVerticesCount;
    if (*slot).deformCount < vertexCount {
        if (*slot).deformCapacity < vertexCount {
            _spFree((*slot).deform as *mut c_void);
            (*slot).deform = _spMalloc(
                (::core::mem::size_of::<c_float>() as c_ulong).wrapping_mul(vertexCount as c_ulong),
                b"spine.c\0" as *const u8 as *const c_char,
                1793 as c_int,
            ) as *mut c_float;
            (*slot).deformCapacity = vertexCount;
        }
    }
    if (*slot).deformCount == 0 as c_int {
        blend = SP_MIX_BLEND_SETUP;
    }
    frameVertices = (*self_0).frameVertices;
    deformArray = (*slot).deform;
    if time < *frames.offset(0 as c_int as isize) {
        let mut vertexAttachment_0: *mut spVertexAttachment =
            (*slot).attachment as *mut spVertexAttachment;
        match blend as c_uint {
            0 => {
                (*slot).deformCount = 0 as c_int;
                return;
            }
            1 => {
                if alpha == 1 as c_int as c_float {
                    (*slot).deformCount = 0 as c_int;
                    return;
                }
                (*slot).deformCount = vertexCount;
                if ((*vertexAttachment_0).bones).is_null() {
                    let mut setupVertices: *mut c_float = (*vertexAttachment_0).vertices;
                    i = 0 as c_int;
                    while i < vertexCount {
                        *deformArray.offset(i as isize) += (*setupVertices.offset(i as isize)
                            - *deformArray.offset(i as isize))
                            * alpha;
                        i += 1;
                    }
                } else {
                    alpha = 1 as c_int as c_float - alpha;
                    i = 0 as c_int;
                    while i < vertexCount {
                        *deformArray.offset(i as isize) *= alpha;
                        i += 1;
                    }
                }
            }
            2 | 3 | _ => {}
        }
        return;
    }
    (*slot).deformCount = vertexCount;
    if time >= *frames.offset((framesCount - 1 as c_int) as isize) {
        let mut lastVertices: *const c_float =
            *((*self_0).frameVertices).offset((framesCount - 1 as c_int) as isize);
        if alpha == 1 as c_int as c_float {
            if blend as c_uint == SP_MIX_BLEND_ADD as c_int as c_uint {
                let mut vertexAttachment_1: *mut spVertexAttachment =
                    (*slot).attachment as *mut spVertexAttachment;
                if ((*vertexAttachment_1).bones).is_null() {
                    let mut setupVertices_0: *mut c_float = (*vertexAttachment_1).vertices;
                    i = 0 as c_int;
                    while i < vertexCount {
                        *deformArray.offset(i as isize) +=
                            *lastVertices.offset(i as isize) - *setupVertices_0.offset(i as isize);
                        i += 1;
                    }
                } else {
                    i = 0 as c_int;
                    while i < vertexCount {
                        *deformArray.offset(i as isize) += *lastVertices.offset(i as isize);
                        i += 1;
                    }
                }
            } else {
                spine_memcpy(
                    deformArray as *mut c_void,
                    lastVertices as *const c_void,
                    (vertexCount as c_ulong)
                        .wrapping_mul(::core::mem::size_of::<c_float>() as c_ulong),
                );
            }
        } else {
            let mut vertexAttachment_2: *mut spVertexAttachment = 0 as *mut spVertexAttachment;
            let mut current_block_86: u64;
            match blend as c_uint {
                0 => {
                    vertexAttachment_2 = (*slot).attachment as *mut spVertexAttachment;
                    if ((*vertexAttachment_2).bones).is_null() {
                        let mut setupVertices_1: *mut c_float = (*vertexAttachment_2).vertices;
                        i = 0 as c_int;
                        while i < vertexCount {
                            let mut setup: c_float = *setupVertices_1.offset(i as isize);
                            *deformArray.offset(i as isize) =
                                setup + (*lastVertices.offset(i as isize) - setup) * alpha;
                            i += 1;
                        }
                    } else {
                        i = 0 as c_int;
                        while i < vertexCount {
                            *deformArray.offset(i as isize) =
                                *lastVertices.offset(i as isize) * alpha;
                            i += 1;
                        }
                    }
                    current_block_86 = 15864857819291709765;
                }
                1 | 2 => {
                    i = 0 as c_int;
                    while i < vertexCount {
                        *deformArray.offset(i as isize) += (*lastVertices.offset(i as isize)
                            - *deformArray.offset(i as isize))
                            * alpha;
                        i += 1;
                    }
                    current_block_86 = 17011199535497144483;
                }
                3 => {
                    current_block_86 = 17011199535497144483;
                }
                _ => {
                    current_block_86 = 15864857819291709765;
                }
            }
            match current_block_86 {
                17011199535497144483 => {
                    vertexAttachment_2 = (*slot).attachment as *mut spVertexAttachment;
                    if ((*vertexAttachment_2).bones).is_null() {
                        let mut setupVertices_2: *mut c_float = (*vertexAttachment_2).vertices;
                        i = 0 as c_int;
                        while i < vertexCount {
                            *deformArray.offset(i as isize) += (*lastVertices.offset(i as isize)
                                - *setupVertices_2.offset(i as isize))
                                * alpha;
                            i += 1;
                        }
                    } else {
                        i = 0 as c_int;
                        while i < vertexCount {
                            *deformArray.offset(i as isize) +=
                                *lastVertices.offset(i as isize) * alpha;
                            i += 1;
                        }
                    }
                }
                _ => {}
            }
        }
        return;
    }
    frame = search((*self_0).super_0.super_0.frames, time);
    percent = _spDeformTimeline_getCurvePercent(self_0, time, frame);
    prevVertices = *frameVertices.offset(frame as isize);
    nextVertices = *frameVertices.offset((frame + 1 as c_int) as isize);
    if alpha == 1 as c_int as c_float {
        if blend as c_uint == SP_MIX_BLEND_ADD as c_int as c_uint {
            let mut vertexAttachment_3: *mut spVertexAttachment =
                (*slot).attachment as *mut spVertexAttachment;
            if ((*vertexAttachment_3).bones).is_null() {
                let mut setupVertices_3: *mut c_float = (*vertexAttachment_3).vertices;
                i = 0 as c_int;
                while i < vertexCount {
                    let mut prev: c_float = *prevVertices.offset(i as isize);
                    *deformArray.offset(i as isize) += prev
                        + (*nextVertices.offset(i as isize) - prev) * percent
                        - *setupVertices_3.offset(i as isize);
                    i += 1;
                }
            } else {
                i = 0 as c_int;
                while i < vertexCount {
                    let mut prev_0: c_float = *prevVertices.offset(i as isize);
                    *deformArray.offset(i as isize) +=
                        prev_0 + (*nextVertices.offset(i as isize) - prev_0) * percent;
                    i += 1;
                }
            }
        } else {
            i = 0 as c_int;
            while i < vertexCount {
                let mut prev_1: c_float = *prevVertices.offset(i as isize);
                *deformArray.offset(i as isize) =
                    prev_1 + (*nextVertices.offset(i as isize) - prev_1) * percent;
                i += 1;
            }
        }
    } else {
        let mut vertexAttachment_4: *mut spVertexAttachment = 0 as *mut spVertexAttachment;
        match blend as c_uint {
            0 => {
                vertexAttachment_4 = (*slot).attachment as *mut spVertexAttachment;
                if ((*vertexAttachment_4).bones).is_null() {
                    let mut setupVertices_4: *mut c_float = (*vertexAttachment_4).vertices;
                    i = 0 as c_int;
                    while i < vertexCount {
                        let mut prev_2: c_float = *prevVertices.offset(i as isize);
                        let mut setup_0: c_float = *setupVertices_4.offset(i as isize);
                        *deformArray.offset(i as isize) = setup_0
                            + (prev_2 + (*nextVertices.offset(i as isize) - prev_2) * percent
                                - setup_0)
                                * alpha;
                        i += 1;
                    }
                } else {
                    i = 0 as c_int;
                    while i < vertexCount {
                        let mut prev_3: c_float = *prevVertices.offset(i as isize);
                        *deformArray.offset(i as isize) = (prev_3
                            + (*nextVertices.offset(i as isize) - prev_3) * percent)
                            * alpha;
                        i += 1;
                    }
                }
            }
            1 | 2 => {
                i = 0 as c_int;
                while i < vertexCount {
                    let mut prev_4: c_float = *prevVertices.offset(i as isize);
                    *deformArray.offset(i as isize) += (prev_4
                        + (*nextVertices.offset(i as isize) - prev_4) * percent
                        - *deformArray.offset(i as isize))
                        * alpha;
                    i += 1;
                }
            }
            3 => {
                vertexAttachment_4 = (*slot).attachment as *mut spVertexAttachment;
                if ((*vertexAttachment_4).bones).is_null() {
                    let mut setupVertices_5: *mut c_float = (*vertexAttachment_4).vertices;
                    i = 0 as c_int;
                    while i < vertexCount {
                        let mut prev_5: c_float = *prevVertices.offset(i as isize);
                        *deformArray.offset(i as isize) += (prev_5
                            + (*nextVertices.offset(i as isize) - prev_5) * percent
                            - *setupVertices_5.offset(i as isize))
                            * alpha;
                        i += 1;
                    }
                } else {
                    i = 0 as c_int;
                    while i < vertexCount {
                        let mut prev_6: c_float = *prevVertices.offset(i as isize);
                        *deformArray.offset(i as isize) += (prev_6
                            + (*nextVertices.offset(i as isize) - prev_6) * percent)
                            * alpha;
                        i += 1;
                    }
                }
            }
            _ => {}
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _spDeformTimeline_dispose(mut timeline: *mut spTimeline) {
    let mut self_0: *mut spDeformTimeline = timeline as *mut spDeformTimeline;
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*(*self_0).super_0.super_0.frames).size {
        _spFree(*((*self_0).frameVertices).offset(i as isize) as *mut c_void);
        i += 1;
    }
    _spFree((*self_0).frameVertices as *mut c_void);
    _spCurveTimeline_dispose(timeline);
}
#[no_mangle]
pub unsafe extern "C" fn spDeformTimeline_create(
    mut framesCount: c_int,
    mut frameVerticesCount: c_int,
    mut bezierCount: c_int,
    mut slotIndex: c_int,
    mut attachment: *mut spVertexAttachment,
) -> *mut spDeformTimeline {
    let mut self_0: *mut spDeformTimeline = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spDeformTimeline>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        1978 as c_int,
    ) as *mut spDeformTimeline;
    let mut ids: [spPropertyId; 1] = [0; 1];
    ids[0 as c_int as usize] = (SP_PROPERTY_DEFORM as c_int as spPropertyId) << 32 as c_int
        | ((slotIndex << 16 as c_int | (*attachment).id) as c_uint & 0xffffffff as c_uint)
            as c_ulong;
    _spCurveTimeline_init(
        &mut (*self_0).super_0,
        framesCount,
        1 as c_int,
        bezierCount,
        ids.as_mut_ptr(),
        1 as c_int,
        SP_TIMELINE_DEFORM,
        Some(_spDeformTimeline_dispose as unsafe extern "C" fn(*mut spTimeline) -> ()),
        Some(
            _spDeformTimeline_apply
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    *mut spSkeleton,
                    c_float,
                    c_float,
                    *mut *mut spEvent,
                    *mut c_int,
                    c_float,
                    spMixBlend,
                    spMixDirection,
                ) -> (),
        ),
        Some(
            _spDeformTimeline_setBezier
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    c_int,
                    c_int,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                ) -> (),
        ),
    );
    let ref mut fresh7 =
        *(&(*self_0).frameVertices as *const *mut *const c_float as *mut *mut *mut c_float);
    *fresh7 = _spCalloc(
        framesCount as size_t,
        ::core::mem::size_of::<*mut c_float>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        1983 as c_int,
    ) as *mut *mut c_float;
    *(&(*self_0).frameVerticesCount as *const c_int as *mut c_int) = frameVerticesCount;
    (*self_0).slotIndex = slotIndex;
    (*self_0).attachment = &mut (*attachment).super_0;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spDeformTimeline_setFrame(
    mut self_0: *mut spDeformTimeline,
    mut frame: c_int,
    mut time: c_float,
    mut vertices: *mut c_float,
) {
    *((*(*self_0).super_0.super_0.frames).items).offset(frame as isize) = time;
    _spFree(*((*self_0).frameVertices).offset(frame as isize) as *mut c_void);
    if vertices.is_null() {
        let ref mut fresh8 = *((*self_0).frameVertices).offset(frame as isize);
        *fresh8 = 0 as *const c_float;
    } else {
        let ref mut fresh9 = *((*self_0).frameVertices).offset(frame as isize);
        *fresh9 = _spMalloc(
            (::core::mem::size_of::<c_float>() as c_ulong)
                .wrapping_mul((*self_0).frameVerticesCount as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            1997 as c_int,
        ) as *mut c_float;
        spine_memcpy(
            *(&mut *((*self_0).frameVertices).offset(frame as isize) as *mut *const c_float
                as *mut *mut c_float) as *mut c_void,
            vertices as *const c_void,
            ((*self_0).frameVerticesCount as c_ulong)
                .wrapping_mul(::core::mem::size_of::<c_float>() as c_ulong),
        );
    };
}
static mut SEQUENCE_ENTRIES: c_int = 3 as c_int;
static mut MODE: c_int = 1 as c_int;
static mut DELAY: c_int = 2 as c_int;
#[no_mangle]
pub unsafe extern "C" fn _spSequenceTimeline_apply(
    mut timeline: *mut spTimeline,
    mut skeleton: *mut spSkeleton,
    mut _lastTime: c_float,
    mut time: c_float,
    mut _firedEvents: *mut *mut spEvent,
    mut _eventsCount: *mut c_int,
    mut _alpha: c_float,
    mut blend: spMixBlend,
    mut _direction: spMixDirection,
) {
    let mut self_0: *mut spSequenceTimeline = timeline as *mut spSequenceTimeline;
    let mut slot: *mut spSlot = *((*skeleton).slots).offset((*self_0).slotIndex as isize);
    let mut slotAttachment: *mut spAttachment = 0 as *mut spAttachment;
    let mut frames: *mut c_float = 0 as *mut c_float;
    let mut i: c_int = 0;
    let mut modeAndIndex: c_int = 0;
    let mut count: c_int = 0;
    let mut index: c_int = 0;
    let mut mode: c_int = 0;
    let mut before: c_float = 0.;
    let mut delay: c_float = 0.;
    let mut sequence: *mut spSequence = 0 as *mut spSequence;
    if (*(*slot).bone).active == 0 {
        return;
    }
    slotAttachment = (*slot).attachment;
    if slotAttachment != (*self_0).attachment {
        match (*(*slot).attachment).type_0 as c_uint {
            1 | 6 | 2 | 4 => {
                let mut vertexAttachment: *mut spVertexAttachment =
                    (*slot).attachment as *mut spVertexAttachment;
                if (*vertexAttachment).timelineAttachment != (*self_0).attachment {
                    return;
                }
            }
            _ => return,
        }
    }
    frames = (*(*self_0).super_0.frames).items;
    if time < *frames.offset(0 as c_int as isize) {
        if blend as c_uint == SP_MIX_BLEND_SETUP as c_int as c_uint
            || blend as c_uint == SP_MIX_BLEND_FIRST as c_int as c_uint
        {
            (*slot).sequenceIndex = -(1 as c_int);
        }
        return;
    }
    i = search2((*self_0).super_0.frames, time, SEQUENCE_ENTRIES);
    before = *frames.offset(i as isize);
    modeAndIndex = *frames.offset((i + MODE) as isize) as c_int;
    delay = *frames.offset((i + DELAY) as isize);
    if (*(*self_0).attachment).type_0 as c_uint == SP_ATTACHMENT_REGION as c_int as c_uint {
        sequence = (*((*self_0).attachment as *mut spRegionAttachment)).sequence;
    }
    if (*(*self_0).attachment).type_0 as c_uint == SP_ATTACHMENT_MESH as c_int as c_uint {
        sequence = (*((*self_0).attachment as *mut spMeshAttachment)).sequence;
    }
    if sequence.is_null() {
        return;
    }
    index = modeAndIndex >> 4 as c_int;
    count = (*(*sequence).regions).size;
    mode = modeAndIndex & 0xf as c_int;
    if mode != 0 as c_int {
        index += (((time - before) / delay) as c_double + 0.00001f64) as c_int;
        match mode {
            1 => {
                index = if (count - 1 as c_int) < index {
                    count - 1 as c_int
                } else {
                    index
                };
            }
            2 => {
                index %= count;
            }
            3 => {
                let mut n: c_int = (count << 1 as c_int) - 2 as c_int;
                index = if n == 0 as c_int {
                    0 as c_int
                } else {
                    index % n
                };
                if index >= count {
                    index = n - index;
                }
            }
            4 => {
                index = if count - 1 as c_int - index > 0 as c_int {
                    count - 1 as c_int - index
                } else {
                    0 as c_int
                };
            }
            5 => {
                index = count - 1 as c_int - index % count;
            }
            6 => {
                let mut n_0: c_int = (count << 1 as c_int) - 2 as c_int;
                index = if n_0 == 0 as c_int {
                    0 as c_int
                } else {
                    (index + count - 1 as c_int) % n_0
                };
                if index >= count {
                    index = n_0 - index;
                }
            }
            _ => {}
        }
    }
    (*slot).sequenceIndex = index;
}
#[no_mangle]
pub unsafe extern "C" fn _spSequenceTimeline_dispose(mut _timeline: *mut spTimeline) {}
#[no_mangle]
pub unsafe extern "C" fn spSequenceTimeline_create(
    mut framesCount: c_int,
    mut slotIndex: c_int,
    mut attachment: *mut spAttachment,
) -> *mut spSequenceTimeline {
    let mut sequenceId: c_int = 0 as c_int;
    let mut self_0: *mut spSequenceTimeline = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spSequenceTimeline>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        2095 as c_int,
    ) as *mut spSequenceTimeline;
    let mut ids: [spPropertyId; 1] = [0; 1];
    if (*attachment).type_0 as c_uint == SP_ATTACHMENT_REGION as c_int as c_uint {
        sequenceId = (*(*(attachment as *mut spRegionAttachment)).sequence).id;
    }
    if (*attachment).type_0 as c_uint == SP_ATTACHMENT_MESH as c_int as c_uint {
        sequenceId = (*(*(attachment as *mut spMeshAttachment)).sequence).id;
    }
    ids[0 as c_int as usize] = (SP_PROPERTY_SEQUENCE as c_int as spPropertyId) << 32 as c_int
        | ((slotIndex << 16 as c_int | sequenceId) as c_uint & 0xffffffff as c_uint) as c_ulong;
    _spTimeline_init(
        &mut (*self_0).super_0,
        framesCount,
        SEQUENCE_ENTRIES,
        ids.as_mut_ptr(),
        1 as c_int,
        SP_TIMELINE_SEQUENCE,
        Some(_spSequenceTimeline_dispose as unsafe extern "C" fn(*mut spTimeline) -> ()),
        Some(
            _spSequenceTimeline_apply
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    *mut spSkeleton,
                    c_float,
                    c_float,
                    *mut *mut spEvent,
                    *mut c_int,
                    c_float,
                    spMixBlend,
                    spMixDirection,
                ) -> (),
        ),
        None,
    );
    (*self_0).slotIndex = slotIndex;
    (*self_0).attachment = attachment;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spSequenceTimeline_setFrame(
    mut self_0: *mut spSequenceTimeline,
    mut frame: c_int,
    mut time: c_float,
    mut mode: c_int,
    mut index: c_int,
    mut delay: c_float,
) {
    let mut frames: *mut c_float = (*(*self_0).super_0.frames).items;
    frame *= SEQUENCE_ENTRIES;
    *frames.offset(frame as isize) = time;
    *frames.offset((frame + MODE) as isize) = (mode | index << 4 as c_int) as c_float;
    *frames.offset((frame + DELAY) as isize) = delay;
}
#[no_mangle]
pub unsafe extern "C" fn _spEventTimeline_apply(
    mut timeline: *mut spTimeline,
    mut skeleton: *mut spSkeleton,
    mut lastTime: c_float,
    mut time: c_float,
    mut firedEvents: *mut *mut spEvent,
    mut eventsCount: *mut c_int,
    mut alpha: c_float,
    mut blend: spMixBlend,
    mut direction: spMixDirection,
) {
    let mut self_0: *mut spEventTimeline = timeline as *mut spEventTimeline;
    let mut frames: *mut c_float = (*(*self_0).super_0.frames).items;
    let mut framesCount: c_int = (*(*self_0).super_0.frames).size;
    let mut i: c_int = 0;
    if firedEvents.is_null() {
        return;
    }
    if lastTime > time {
        _spEventTimeline_apply(
            timeline,
            skeleton,
            lastTime,
            0x7fffffff as c_int as c_float,
            firedEvents,
            eventsCount,
            alpha,
            blend,
            direction,
        );
        lastTime = -(1 as c_int) as c_float;
    } else if lastTime >= *frames.offset((framesCount - 1 as c_int) as isize) {
        return;
    }
    if time < *frames.offset(0 as c_int as isize) {
        return;
    }
    if lastTime < *frames.offset(0 as c_int as isize) {
        i = 0 as c_int;
    } else {
        let mut frameTime: c_float = 0.;
        i = search((*self_0).super_0.frames, lastTime) + 1 as c_int;
        frameTime = *frames.offset(i as isize);
        while i > 0 as c_int {
            if *frames.offset((i - 1 as c_int) as isize) != frameTime {
                break;
            }
            i -= 1;
        }
    }
    while i < framesCount && time >= *frames.offset(i as isize) {
        let ref mut fresh10 = *firedEvents.offset(*eventsCount as isize);
        *fresh10 = *((*self_0).events).offset(i as isize);
        *eventsCount += 1;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _spEventTimeline_dispose(mut timeline: *mut spTimeline) {
    let mut self_0: *mut spEventTimeline = timeline as *mut spEventTimeline;
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*(*self_0).super_0.frames).size {
        spEvent_dispose(*((*self_0).events).offset(i as isize));
        i += 1;
    }
    _spFree((*self_0).events as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spEventTimeline_create(mut framesCount: c_int) -> *mut spEventTimeline {
    let mut self_0: *mut spEventTimeline = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spEventTimeline>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        2165 as c_int,
    ) as *mut spEventTimeline;
    let mut ids: [spPropertyId; 1] = [0; 1];
    ids[0 as c_int as usize] = (SP_PROPERTY_EVENT as c_int as spPropertyId) << 32 as c_int;
    _spTimeline_init(
        &mut (*self_0).super_0,
        framesCount,
        1 as c_int,
        ids.as_mut_ptr(),
        1 as c_int,
        SP_TIMELINE_EVENT,
        Some(_spEventTimeline_dispose as unsafe extern "C" fn(*mut spTimeline) -> ()),
        Some(
            _spEventTimeline_apply
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    *mut spSkeleton,
                    c_float,
                    c_float,
                    *mut *mut spEvent,
                    *mut c_int,
                    c_float,
                    spMixBlend,
                    spMixDirection,
                ) -> (),
        ),
        None,
    );
    let ref mut fresh11 =
        *(&(*self_0).events as *const *mut *mut spEvent as *mut *mut *mut spEvent);
    *fresh11 = _spCalloc(
        framesCount as size_t,
        ::core::mem::size_of::<*mut spEvent>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        2170 as c_int,
    ) as *mut *mut spEvent;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spEventTimeline_setFrame(
    mut self_0: *mut spEventTimeline,
    mut frame: c_int,
    mut event: *mut spEvent,
) {
    *((*(*self_0).super_0.frames).items).offset(frame as isize) = (*event).time;
    _spFree(*((*self_0).events).offset(frame as isize) as *mut c_void);
    let ref mut fresh12 = *((*self_0).events).offset(frame as isize);
    *fresh12 = event;
}
#[no_mangle]
pub unsafe extern "C" fn _spDrawOrderTimeline_apply(
    mut timeline: *mut spTimeline,
    mut skeleton: *mut spSkeleton,
    mut _lastTime: c_float,
    mut time: c_float,
    mut _firedEvents: *mut *mut spEvent,
    mut _eventsCount: *mut c_int,
    mut _alpha: c_float,
    mut blend: spMixBlend,
    mut direction: spMixDirection,
) {
    let mut i: c_int = 0;
    let mut drawOrderToSetupIndex: *const c_int = 0 as *const c_int;
    let mut self_0: *mut spDrawOrderTimeline = timeline as *mut spDrawOrderTimeline;
    let mut frames: *mut c_float = (*(*self_0).super_0.frames).items;
    if direction as c_uint == SP_MIX_DIRECTION_OUT as c_int as c_uint {
        if blend as c_uint == SP_MIX_BLEND_SETUP as c_int as c_uint {
            spine_memcpy(
                (*skeleton).drawOrder as *mut c_void,
                (*skeleton).slots as *const c_void,
                ((*self_0).slotsCount as c_ulong)
                    .wrapping_mul(::core::mem::size_of::<*mut spSlot>() as c_ulong),
            );
        }
        return;
    }
    if time < *frames.offset(0 as c_int as isize) {
        if blend as c_uint == SP_MIX_BLEND_SETUP as c_int as c_uint
            || blend as c_uint == SP_MIX_BLEND_FIRST as c_int as c_uint
        {
            spine_memcpy(
                (*skeleton).drawOrder as *mut c_void,
                (*skeleton).slots as *const c_void,
                ((*self_0).slotsCount as c_ulong)
                    .wrapping_mul(::core::mem::size_of::<*mut spSlot>() as c_ulong),
            );
        }
        return;
    }
    drawOrderToSetupIndex =
        *((*self_0).drawOrders).offset(search((*self_0).super_0.frames, time) as isize);
    if drawOrderToSetupIndex.is_null() {
        spine_memcpy(
            (*skeleton).drawOrder as *mut c_void,
            (*skeleton).slots as *const c_void,
            ((*self_0).slotsCount as c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut spSlot>() as c_ulong),
        );
    } else {
        i = 0 as c_int;
        while i < (*self_0).slotsCount {
            let ref mut fresh13 = *((*skeleton).drawOrder).offset(i as isize);
            *fresh13 =
                *((*skeleton).slots).offset(*drawOrderToSetupIndex.offset(i as isize) as isize);
            i += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _spDrawOrderTimeline_dispose(mut timeline: *mut spTimeline) {
    let mut self_0: *mut spDrawOrderTimeline = timeline as *mut spDrawOrderTimeline;
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*(*self_0).super_0.frames).size {
        _spFree(*((*self_0).drawOrders).offset(i as isize) as *mut c_void);
        i += 1;
    }
    _spFree((*self_0).drawOrders as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spDrawOrderTimeline_create(
    mut framesCount: c_int,
    mut slotsCount: c_int,
) -> *mut spDrawOrderTimeline {
    let mut self_0: *mut spDrawOrderTimeline = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spDrawOrderTimeline>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        2227 as c_int,
    ) as *mut spDrawOrderTimeline;
    let mut ids: [spPropertyId; 1] = [0; 1];
    ids[0 as c_int as usize] = (SP_PROPERTY_DRAWORDER as c_int as spPropertyId) << 32 as c_int;
    _spTimeline_init(
        &mut (*self_0).super_0,
        framesCount,
        1 as c_int,
        ids.as_mut_ptr(),
        1 as c_int,
        SP_TIMELINE_DRAWORDER,
        Some(_spDrawOrderTimeline_dispose as unsafe extern "C" fn(*mut spTimeline) -> ()),
        Some(
            _spDrawOrderTimeline_apply
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    *mut spSkeleton,
                    c_float,
                    c_float,
                    *mut *mut spEvent,
                    *mut c_int,
                    c_float,
                    spMixBlend,
                    spMixDirection,
                ) -> (),
        ),
        None,
    );
    let ref mut fresh14 =
        *(&(*self_0).drawOrders as *const *mut *const c_int as *mut *mut *mut c_int);
    *fresh14 = _spCalloc(
        framesCount as size_t,
        ::core::mem::size_of::<*mut c_int>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        2233 as c_int,
    ) as *mut *mut c_int;
    *(&(*self_0).slotsCount as *const c_int as *mut c_int) = slotsCount;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spDrawOrderTimeline_setFrame(
    mut self_0: *mut spDrawOrderTimeline,
    mut frame: c_int,
    mut time: c_float,
    mut drawOrder: *const c_int,
) {
    *((*(*self_0).super_0.frames).items).offset(frame as isize) = time;
    _spFree(*((*self_0).drawOrders).offset(frame as isize) as *mut c_void);
    if drawOrder.is_null() {
        let ref mut fresh15 = *((*self_0).drawOrders).offset(frame as isize);
        *fresh15 = 0 as *const c_int;
    } else {
        let ref mut fresh16 = *((*self_0).drawOrders).offset(frame as isize);
        *fresh16 = _spMalloc(
            (::core::mem::size_of::<c_int>() as c_ulong)
                .wrapping_mul((*self_0).slotsCount as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            2246 as c_int,
        ) as *mut c_int;
        spine_memcpy(
            *(&mut *((*self_0).drawOrders).offset(frame as isize) as *mut *const c_int
                as *mut *mut c_int) as *mut c_void,
            drawOrder as *const c_void,
            ((*self_0).slotsCount as c_ulong)
                .wrapping_mul(::core::mem::size_of::<c_int>() as c_ulong),
        );
    };
}
static mut IKCONSTRAINT_ENTRIES: c_int = 6 as c_int;
static mut IKCONSTRAINT_MIX: c_int = 1 as c_int;
static mut IKCONSTRAINT_SOFTNESS: c_int = 2 as c_int;
static mut IKCONSTRAINT_BEND_DIRECTION: c_int = 3 as c_int;
static mut IKCONSTRAINT_COMPRESS: c_int = 4 as c_int;
static mut IKCONSTRAINT_STRETCH: c_int = 5 as c_int;
#[no_mangle]
pub unsafe extern "C" fn _spIkConstraintTimeline_apply(
    mut timeline: *mut spTimeline,
    mut skeleton: *mut spSkeleton,
    mut _lastTime: c_float,
    mut time: c_float,
    mut _firedEvents: *mut *mut spEvent,
    mut _eventsCount: *mut c_int,
    mut alpha: c_float,
    mut blend: spMixBlend,
    mut direction: spMixDirection,
) {
    let mut i: c_int = 0;
    let mut curveType: c_int = 0;
    let mut mix: c_float = 0.;
    let mut softness: c_float = 0.;
    let mut t: c_float = 0.;
    let mut constraint: *mut spIkConstraint = 0 as *mut spIkConstraint;
    let mut self_0: *mut spIkConstraintTimeline = timeline as *mut spIkConstraintTimeline;
    let mut frames: *mut c_float = (*(*self_0).super_0.super_0.frames).items;
    let mut curves: *mut c_float = (*(*self_0).super_0.curves).items;
    constraint = *((*skeleton).ikConstraints).offset((*self_0).ikConstraintIndex as isize);
    if (*constraint).active == 0 {
        return;
    }
    if time < *frames.offset(0 as c_int as isize) {
        match blend as c_uint {
            0 => {
                (*constraint).mix = (*(*constraint).data).mix;
                (*constraint).softness = (*(*constraint).data).softness;
                (*constraint).bendDirection = (*(*constraint).data).bendDirection;
                (*constraint).compress = (*(*constraint).data).compress;
                (*constraint).stretch = (*(*constraint).data).stretch;
                return;
            }
            1 => {
                (*constraint).mix += ((*(*constraint).data).mix - (*constraint).mix) * alpha;
                (*constraint).softness +=
                    ((*(*constraint).data).softness - (*constraint).softness) * alpha;
                (*constraint).bendDirection = (*(*constraint).data).bendDirection;
                (*constraint).compress = (*(*constraint).data).compress;
                (*constraint).stretch = (*(*constraint).data).stretch;
                return;
            }
            _ => return,
        }
    }
    i = search2((*self_0).super_0.super_0.frames, time, IKCONSTRAINT_ENTRIES);
    curveType = *curves.offset((i / IKCONSTRAINT_ENTRIES) as isize) as c_int;
    match curveType {
        0 => {
            let mut before: c_float = *frames.offset(i as isize);
            mix = *frames.offset((i + IKCONSTRAINT_MIX) as isize);
            softness = *frames.offset((i + IKCONSTRAINT_SOFTNESS) as isize);
            t = (time - before) / (*frames.offset((i + IKCONSTRAINT_ENTRIES) as isize) - before);
            mix +=
                (*frames.offset((i + IKCONSTRAINT_ENTRIES + IKCONSTRAINT_MIX) as isize) - mix) * t;
            softness += (*frames
                .offset((i + IKCONSTRAINT_ENTRIES + IKCONSTRAINT_SOFTNESS) as isize)
                - softness)
                * t;
        }
        1 => {
            mix = *frames.offset((i + IKCONSTRAINT_MIX) as isize);
            softness = *frames.offset((i + IKCONSTRAINT_SOFTNESS) as isize);
        }
        _ => {
            mix = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                IKCONSTRAINT_MIX,
                curveType - 2 as c_int,
            );
            softness = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                IKCONSTRAINT_SOFTNESS,
                curveType + 18 as c_int - 2 as c_int,
            );
        }
    }
    if blend as c_uint == SP_MIX_BLEND_SETUP as c_int as c_uint {
        (*constraint).mix = (*(*constraint).data).mix + (mix - (*(*constraint).data).mix) * alpha;
        (*constraint).softness =
            (*(*constraint).data).softness + (softness - (*(*constraint).data).softness) * alpha;
        if direction as c_uint == SP_MIX_DIRECTION_OUT as c_int as c_uint {
            (*constraint).bendDirection = (*(*constraint).data).bendDirection;
            (*constraint).compress = (*(*constraint).data).compress;
            (*constraint).stretch = (*(*constraint).data).stretch;
        } else {
            (*constraint).bendDirection =
                *frames.offset((i + IKCONSTRAINT_BEND_DIRECTION) as isize) as c_int;
            (*constraint).compress = (*frames.offset((i + IKCONSTRAINT_COMPRESS) as isize)
                != 0 as c_int as c_float) as c_int;
            (*constraint).stretch = (*frames.offset((i + IKCONSTRAINT_STRETCH) as isize)
                != 0 as c_int as c_float) as c_int;
        }
    } else {
        (*constraint).mix += (mix - (*constraint).mix) * alpha;
        (*constraint).softness += (softness - (*constraint).softness) * alpha;
        if direction as c_uint == SP_MIX_DIRECTION_IN as c_int as c_uint {
            (*constraint).bendDirection =
                *frames.offset((i + IKCONSTRAINT_BEND_DIRECTION) as isize) as c_int;
            (*constraint).compress = (*frames.offset((i + IKCONSTRAINT_COMPRESS) as isize)
                != 0 as c_int as c_float) as c_int;
            (*constraint).stretch = (*frames.offset((i + IKCONSTRAINT_STRETCH) as isize)
                != 0 as c_int as c_float) as c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn spIkConstraintTimeline_create(
    mut framesCount: c_int,
    mut bezierCount: c_int,
    mut ikConstraintIndex: c_int,
) -> *mut spIkConstraintTimeline {
    let mut timeline: *mut spIkConstraintTimeline = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spIkConstraintTimeline>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        2343 as c_int,
    ) as *mut spIkConstraintTimeline;
    let mut ids: [spPropertyId; 1] = [0; 1];
    ids[0 as c_int as usize] = (SP_PROPERTY_IKCONSTRAINT as c_int as spPropertyId) << 32 as c_int
        | ikConstraintIndex as c_ulong;
    _spCurveTimeline_init(
        &mut (*timeline).super_0,
        framesCount,
        IKCONSTRAINT_ENTRIES,
        bezierCount,
        ids.as_mut_ptr(),
        1 as c_int,
        SP_TIMELINE_IKCONSTRAINT,
        Some(_spCurveTimeline_dispose as unsafe extern "C" fn(*mut spTimeline) -> ()),
        Some(
            _spIkConstraintTimeline_apply
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    *mut spSkeleton,
                    c_float,
                    c_float,
                    *mut *mut spEvent,
                    *mut c_int,
                    c_float,
                    spMixBlend,
                    spMixDirection,
                ) -> (),
        ),
        Some(
            _spCurveTimeline_setBezier
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    c_int,
                    c_int,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                ) -> (),
        ),
    );
    (*timeline).ikConstraintIndex = ikConstraintIndex;
    return timeline;
}
#[no_mangle]
pub unsafe extern "C" fn spIkConstraintTimeline_setFrame(
    mut self_0: *mut spIkConstraintTimeline,
    mut frame: c_int,
    mut time: c_float,
    mut mix: c_float,
    mut softness: c_float,
    mut bendDirection: c_int,
    mut compress: c_int,
    mut stretch: c_int,
) {
    let mut frames: *mut c_float = (*(*self_0).super_0.super_0.frames).items;
    frame *= IKCONSTRAINT_ENTRIES;
    *frames.offset(frame as isize) = time;
    *frames.offset((frame + IKCONSTRAINT_MIX) as isize) = mix;
    *frames.offset((frame + IKCONSTRAINT_SOFTNESS) as isize) = softness;
    *frames.offset((frame + IKCONSTRAINT_BEND_DIRECTION) as isize) = bendDirection as c_float;
    *frames.offset((frame + IKCONSTRAINT_COMPRESS) as isize) = (if compress != 0 {
        1 as c_int
    } else {
        0 as c_int
    }) as c_float;
    *frames.offset((frame + IKCONSTRAINT_STRETCH) as isize) =
        (if stretch != 0 { 1 as c_int } else { 0 as c_int }) as c_float;
}
static mut TRANSFORMCONSTRAINT_ENTRIES: c_int = 7 as c_int;
static mut TRANSFORMCONSTRAINT_ROTATE: c_int = 1 as c_int;
static mut TRANSFORMCONSTRAINT_X: c_int = 2 as c_int;
static mut TRANSFORMCONSTRAINT_Y: c_int = 3 as c_int;
static mut TRANSFORMCONSTRAINT_SCALEX: c_int = 4 as c_int;
static mut TRANSFORMCONSTRAINT_SCALEY: c_int = 5 as c_int;
static mut TRANSFORMCONSTRAINT_SHEARY: c_int = 6 as c_int;
#[no_mangle]
pub unsafe extern "C" fn _spTransformConstraintTimeline_apply(
    mut timeline: *mut spTimeline,
    mut skeleton: *mut spSkeleton,
    mut _lastTime: c_float,
    mut time: c_float,
    mut _firedEvents: *mut *mut spEvent,
    mut _eventsCount: *mut c_int,
    mut alpha: c_float,
    mut blend: spMixBlend,
    mut _direction: spMixDirection,
) {
    let mut i: c_int = 0;
    let mut curveType: c_int = 0;
    let mut rotate: c_float = 0.;
    let mut x: c_float = 0.;
    let mut y: c_float = 0.;
    let mut scaleX: c_float = 0.;
    let mut scaleY: c_float = 0.;
    let mut shearY: c_float = 0.;
    let mut t: c_float = 0.;
    let mut constraint: *mut spTransformConstraint = 0 as *mut spTransformConstraint;
    let mut self_0: *mut spTransformConstraintTimeline =
        timeline as *mut spTransformConstraintTimeline;
    let mut frames: *mut c_float = 0 as *mut c_float;
    let mut curves: *mut c_float = 0 as *mut c_float;
    let mut data: *mut spTransformConstraintData = 0 as *mut spTransformConstraintData;
    constraint =
        *((*skeleton).transformConstraints).offset((*self_0).transformConstraintIndex as isize);
    if (*constraint).active == 0 {
        return;
    }
    frames = (*(*self_0).super_0.super_0.frames).items;
    curves = (*(*self_0).super_0.curves).items;
    data = (*constraint).data;
    if time < *frames.offset(0 as c_int as isize) {
        match blend as c_uint {
            0 => {
                (*constraint).mixRotate = (*data).mixRotate;
                (*constraint).mixX = (*data).mixX;
                (*constraint).mixY = (*data).mixY;
                (*constraint).mixScaleX = (*data).mixScaleX;
                (*constraint).mixScaleY = (*data).mixScaleY;
                (*constraint).mixShearY = (*data).mixShearY;
                return;
            }
            1 => {
                (*constraint).mixRotate += ((*data).mixRotate - (*constraint).mixRotate) * alpha;
                (*constraint).mixX += ((*data).mixX - (*constraint).mixX) * alpha;
                (*constraint).mixY += ((*data).mixY - (*constraint).mixY) * alpha;
                (*constraint).mixScaleX += ((*data).mixScaleX - (*constraint).mixScaleX) * alpha;
                (*constraint).mixScaleY += ((*data).mixScaleY - (*constraint).mixScaleY) * alpha;
                (*constraint).mixShearY += ((*data).mixShearY - (*constraint).mixShearY) * alpha;
                return;
            }
            _ => return,
        }
    }
    i = search2(
        (*self_0).super_0.super_0.frames,
        time,
        TRANSFORMCONSTRAINT_ENTRIES,
    );
    curveType = *curves.offset((i / TRANSFORMCONSTRAINT_ENTRIES) as isize) as c_int;
    match curveType {
        0 => {
            let mut before: c_float = *frames.offset(i as isize);
            rotate = *frames.offset((i + TRANSFORMCONSTRAINT_ROTATE) as isize);
            x = *frames.offset((i + TRANSFORMCONSTRAINT_X) as isize);
            y = *frames.offset((i + TRANSFORMCONSTRAINT_Y) as isize);
            scaleX = *frames.offset((i + TRANSFORMCONSTRAINT_SCALEX) as isize);
            scaleY = *frames.offset((i + TRANSFORMCONSTRAINT_SCALEY) as isize);
            shearY = *frames.offset((i + TRANSFORMCONSTRAINT_SHEARY) as isize);
            t = (time - before)
                / (*frames.offset((i + TRANSFORMCONSTRAINT_ENTRIES) as isize) - before);
            rotate += (*frames
                .offset((i + TRANSFORMCONSTRAINT_ENTRIES + TRANSFORMCONSTRAINT_ROTATE) as isize)
                - rotate)
                * t;
            x += (*frames
                .offset((i + TRANSFORMCONSTRAINT_ENTRIES + TRANSFORMCONSTRAINT_X) as isize)
                - x)
                * t;
            y += (*frames
                .offset((i + TRANSFORMCONSTRAINT_ENTRIES + TRANSFORMCONSTRAINT_Y) as isize)
                - y)
                * t;
            scaleX += (*frames
                .offset((i + TRANSFORMCONSTRAINT_ENTRIES + TRANSFORMCONSTRAINT_SCALEX) as isize)
                - scaleX)
                * t;
            scaleY += (*frames
                .offset((i + TRANSFORMCONSTRAINT_ENTRIES + TRANSFORMCONSTRAINT_SCALEY) as isize)
                - scaleY)
                * t;
            shearY += (*frames
                .offset((i + TRANSFORMCONSTRAINT_ENTRIES + TRANSFORMCONSTRAINT_SHEARY) as isize)
                - shearY)
                * t;
        }
        1 => {
            rotate = *frames.offset((i + TRANSFORMCONSTRAINT_ROTATE) as isize);
            x = *frames.offset((i + TRANSFORMCONSTRAINT_X) as isize);
            y = *frames.offset((i + TRANSFORMCONSTRAINT_Y) as isize);
            scaleX = *frames.offset((i + TRANSFORMCONSTRAINT_SCALEX) as isize);
            scaleY = *frames.offset((i + TRANSFORMCONSTRAINT_SCALEY) as isize);
            shearY = *frames.offset((i + TRANSFORMCONSTRAINT_SHEARY) as isize);
        }
        _ => {
            rotate = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                TRANSFORMCONSTRAINT_ROTATE,
                curveType - 2 as c_int,
            );
            x = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                TRANSFORMCONSTRAINT_X,
                curveType + 18 as c_int - 2 as c_int,
            );
            y = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                TRANSFORMCONSTRAINT_Y,
                curveType + 18 as c_int * 2 as c_int - 2 as c_int,
            );
            scaleX = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                TRANSFORMCONSTRAINT_SCALEX,
                curveType + 18 as c_int * 3 as c_int - 2 as c_int,
            );
            scaleY = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                TRANSFORMCONSTRAINT_SCALEY,
                curveType + 18 as c_int * 4 as c_int - 2 as c_int,
            );
            shearY = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                TRANSFORMCONSTRAINT_SHEARY,
                curveType + 18 as c_int * 5 as c_int - 2 as c_int,
            );
        }
    }
    if blend as c_uint == SP_MIX_BLEND_SETUP as c_int as c_uint {
        (*constraint).mixRotate = (*data).mixRotate + (rotate - (*data).mixRotate) * alpha;
        (*constraint).mixX = (*data).mixX + (x - (*data).mixX) * alpha;
        (*constraint).mixY = (*data).mixY + (y - (*data).mixY) * alpha;
        (*constraint).mixScaleX = (*data).mixScaleX + (scaleX - (*data).mixScaleX) * alpha;
        (*constraint).mixScaleY = (*data).mixScaleY + (scaleY - (*data).mixScaleY) * alpha;
        (*constraint).mixShearY = (*data).mixShearY + (shearY - (*data).mixShearY) * alpha;
    } else {
        (*constraint).mixRotate += (rotate - (*constraint).mixRotate) * alpha;
        (*constraint).mixX += (x - (*constraint).mixX) * alpha;
        (*constraint).mixY += (y - (*constraint).mixY) * alpha;
        (*constraint).mixScaleX += (scaleX - (*constraint).mixScaleX) * alpha;
        (*constraint).mixScaleY += (scaleY - (*constraint).mixScaleY) * alpha;
        (*constraint).mixShearY += (shearY - (*constraint).mixShearY) * alpha;
    };
}
#[no_mangle]
pub unsafe extern "C" fn spTransformConstraintTimeline_create(
    mut framesCount: c_int,
    mut bezierCount: c_int,
    mut transformConstraintIndex: c_int,
) -> *mut spTransformConstraintTimeline {
    let mut timeline: *mut spTransformConstraintTimeline = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spTransformConstraintTimeline>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        2484 as c_int,
    )
        as *mut spTransformConstraintTimeline;
    let mut ids: [spPropertyId; 1] = [0; 1];
    ids[0 as c_int as usize] = (SP_PROPERTY_TRANSFORMCONSTRAINT as c_int as spPropertyId)
        << 32 as c_int
        | transformConstraintIndex as c_ulong;
    _spCurveTimeline_init(
        &mut (*timeline).super_0,
        framesCount,
        TRANSFORMCONSTRAINT_ENTRIES,
        bezierCount,
        ids.as_mut_ptr(),
        1 as c_int,
        SP_TIMELINE_TRANSFORMCONSTRAINT,
        Some(_spCurveTimeline_dispose as unsafe extern "C" fn(*mut spTimeline) -> ()),
        Some(
            _spTransformConstraintTimeline_apply
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    *mut spSkeleton,
                    c_float,
                    c_float,
                    *mut *mut spEvent,
                    *mut c_int,
                    c_float,
                    spMixBlend,
                    spMixDirection,
                ) -> (),
        ),
        Some(
            _spCurveTimeline_setBezier
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    c_int,
                    c_int,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                ) -> (),
        ),
    );
    (*timeline).transformConstraintIndex = transformConstraintIndex;
    return timeline;
}
#[no_mangle]
pub unsafe extern "C" fn spTransformConstraintTimeline_setFrame(
    mut self_0: *mut spTransformConstraintTimeline,
    mut frame: c_int,
    mut time: c_float,
    mut mixRotate: c_float,
    mut mixX: c_float,
    mut mixY: c_float,
    mut mixScaleX: c_float,
    mut mixScaleY: c_float,
    mut mixShearY: c_float,
) {
    let mut frames: *mut c_float = (*(*self_0).super_0.super_0.frames).items;
    frame *= TRANSFORMCONSTRAINT_ENTRIES;
    *frames.offset(frame as isize) = time;
    *frames.offset((frame + TRANSFORMCONSTRAINT_ROTATE) as isize) = mixRotate;
    *frames.offset((frame + TRANSFORMCONSTRAINT_X) as isize) = mixX;
    *frames.offset((frame + TRANSFORMCONSTRAINT_Y) as isize) = mixY;
    *frames.offset((frame + TRANSFORMCONSTRAINT_SCALEX) as isize) = mixScaleX;
    *frames.offset((frame + TRANSFORMCONSTRAINT_SCALEY) as isize) = mixScaleY;
    *frames.offset((frame + TRANSFORMCONSTRAINT_SHEARY) as isize) = mixShearY;
}
static mut PATHCONSTRAINTPOSITION_ENTRIES: c_int = 2 as c_int;
static mut PATHCONSTRAINTPOSITION_VALUE: c_int = 1 as c_int;
#[no_mangle]
pub unsafe extern "C" fn _spPathConstraintPositionTimeline_apply(
    mut timeline: *mut spTimeline,
    mut skeleton: *mut spSkeleton,
    mut _lastTime: c_float,
    mut time: c_float,
    mut _firedEvents: *mut *mut spEvent,
    mut _eventsCount: *mut c_int,
    mut alpha: c_float,
    mut blend: spMixBlend,
    mut _direction: spMixDirection,
) {
    let mut position: c_float = 0.;
    let mut constraint: *mut spPathConstraint = 0 as *mut spPathConstraint;
    let mut self_0: *mut spPathConstraintPositionTimeline =
        timeline as *mut spPathConstraintPositionTimeline;
    let mut frames: *mut c_float = 0 as *mut c_float;
    constraint = *((*skeleton).pathConstraints).offset((*self_0).pathConstraintIndex as isize);
    if (*constraint).active == 0 {
        return;
    }
    frames = (*(*self_0).super_0.super_0.frames).items;
    if time < *frames.offset(0 as c_int as isize) {
        match blend as c_uint {
            0 => {
                (*constraint).position = (*(*constraint).data).position;
                return;
            }
            1 => {
                (*constraint).position +=
                    ((*(*constraint).data).position - (*constraint).position) * alpha;
                return;
            }
            _ => return,
        }
    }
    position = spCurveTimeline1_getCurveValue(&mut (*self_0).super_0, time);
    if blend as c_uint == SP_MIX_BLEND_SETUP as c_int as c_uint {
        (*constraint).position =
            (*(*constraint).data).position + (position - (*(*constraint).data).position) * alpha;
    } else {
        (*constraint).position += (position - (*constraint).position) * alpha;
    };
}
#[no_mangle]
pub unsafe extern "C" fn spPathConstraintPositionTimeline_create(
    mut framesCount: c_int,
    mut bezierCount: c_int,
    mut pathConstraintIndex: c_int,
) -> *mut spPathConstraintPositionTimeline {
    let mut timeline: *mut spPathConstraintPositionTimeline = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spPathConstraintPositionTimeline>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        2552 as c_int,
    )
        as *mut spPathConstraintPositionTimeline;
    let mut ids: [spPropertyId; 1] = [0; 1];
    ids[0 as c_int as usize] = (SP_PROPERTY_PATHCONSTRAINT_POSITION as c_int as spPropertyId)
        << 32 as c_int
        | pathConstraintIndex as c_ulong;
    _spCurveTimeline_init(
        &mut (*timeline).super_0,
        framesCount,
        PATHCONSTRAINTPOSITION_ENTRIES,
        bezierCount,
        ids.as_mut_ptr(),
        1 as c_int,
        SP_TIMELINE_PATHCONSTRAINTPOSITION,
        Some(_spCurveTimeline_dispose as unsafe extern "C" fn(*mut spTimeline) -> ()),
        Some(
            _spPathConstraintPositionTimeline_apply
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    *mut spSkeleton,
                    c_float,
                    c_float,
                    *mut *mut spEvent,
                    *mut c_int,
                    c_float,
                    spMixBlend,
                    spMixDirection,
                ) -> (),
        ),
        Some(
            _spCurveTimeline_setBezier
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    c_int,
                    c_int,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                ) -> (),
        ),
    );
    (*timeline).pathConstraintIndex = pathConstraintIndex;
    return timeline;
}
#[no_mangle]
pub unsafe extern "C" fn spPathConstraintPositionTimeline_setFrame(
    mut self_0: *mut spPathConstraintPositionTimeline,
    mut frame: c_int,
    mut time: c_float,
    mut value: c_float,
) {
    let mut frames: *mut c_float = (*(*self_0).super_0.super_0.frames).items;
    frame *= PATHCONSTRAINTPOSITION_ENTRIES;
    *frames.offset(frame as isize) = time;
    *frames.offset((frame + PATHCONSTRAINTPOSITION_VALUE) as isize) = value;
}
static mut PATHCONSTRAINTSPACING_ENTRIES: c_int = 2 as c_int;
static mut PATHCONSTRAINTSPACING_VALUE: c_int = 1 as c_int;
#[no_mangle]
pub unsafe extern "C" fn _spPathConstraintSpacingTimeline_apply(
    mut timeline: *mut spTimeline,
    mut skeleton: *mut spSkeleton,
    mut _lastTime: c_float,
    mut time: c_float,
    mut _firedEvents: *mut *mut spEvent,
    mut _eventsCount: *mut c_int,
    mut alpha: c_float,
    mut blend: spMixBlend,
    mut _direction: spMixDirection,
) {
    let mut spacing: c_float = 0.;
    let mut constraint: *mut spPathConstraint = 0 as *mut spPathConstraint;
    let mut self_0: *mut spPathConstraintSpacingTimeline =
        timeline as *mut spPathConstraintSpacingTimeline;
    let mut frames: *mut c_float = 0 as *mut c_float;
    constraint = *((*skeleton).pathConstraints).offset((*self_0).pathConstraintIndex as isize);
    if (*constraint).active == 0 {
        return;
    }
    frames = (*(*self_0).super_0.super_0.frames).items;
    if time < *frames.offset(0 as c_int as isize) {
        match blend as c_uint {
            0 => {
                (*constraint).spacing = (*(*constraint).data).spacing;
                return;
            }
            1 => {
                (*constraint).spacing +=
                    ((*(*constraint).data).spacing - (*constraint).spacing) * alpha;
                return;
            }
            _ => return,
        }
    }
    spacing = spCurveTimeline1_getCurveValue(&mut (*self_0).super_0, time);
    if blend as c_uint == SP_MIX_BLEND_SETUP as c_int as c_uint {
        (*constraint).spacing =
            (*(*constraint).data).spacing + (spacing - (*(*constraint).data).spacing) * alpha;
    } else {
        (*constraint).spacing += (spacing - (*constraint).spacing) * alpha;
    };
}
#[no_mangle]
pub unsafe extern "C" fn spPathConstraintSpacingTimeline_create(
    mut framesCount: c_int,
    mut bezierCount: c_int,
    mut pathConstraintIndex: c_int,
) -> *mut spPathConstraintSpacingTimeline {
    let mut timeline: *mut spPathConstraintSpacingTimeline = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spPathConstraintSpacingTimeline>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        2614 as c_int,
    )
        as *mut spPathConstraintSpacingTimeline;
    let mut ids: [spPropertyId; 1] = [0; 1];
    ids[0 as c_int as usize] = (SP_PROPERTY_PATHCONSTRAINT_SPACING as c_int as spPropertyId)
        << 32 as c_int
        | pathConstraintIndex as c_ulong;
    _spCurveTimeline_init(
        &mut (*timeline).super_0,
        framesCount,
        PATHCONSTRAINTSPACING_ENTRIES,
        bezierCount,
        ids.as_mut_ptr(),
        1 as c_int,
        SP_TIMELINE_PATHCONSTRAINTSPACING,
        Some(_spCurveTimeline_dispose as unsafe extern "C" fn(*mut spTimeline) -> ()),
        Some(
            _spPathConstraintSpacingTimeline_apply
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    *mut spSkeleton,
                    c_float,
                    c_float,
                    *mut *mut spEvent,
                    *mut c_int,
                    c_float,
                    spMixBlend,
                    spMixDirection,
                ) -> (),
        ),
        Some(
            _spCurveTimeline_setBezier
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    c_int,
                    c_int,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                ) -> (),
        ),
    );
    (*timeline).pathConstraintIndex = pathConstraintIndex;
    return timeline;
}
#[no_mangle]
pub unsafe extern "C" fn spPathConstraintSpacingTimeline_setFrame(
    mut self_0: *mut spPathConstraintSpacingTimeline,
    mut frame: c_int,
    mut time: c_float,
    mut value: c_float,
) {
    let mut frames: *mut c_float = (*(*self_0).super_0.super_0.frames).items;
    frame *= PATHCONSTRAINTSPACING_ENTRIES;
    *frames.offset(frame as isize) = time;
    *frames.offset((frame + PATHCONSTRAINTSPACING_VALUE) as isize) = value;
}
static mut PATHCONSTRAINTMIX_ENTRIES: c_int = 4 as c_int;
static mut PATHCONSTRAINTMIX_ROTATE: c_int = 1 as c_int;
static mut PATHCONSTRAINTMIX_X: c_int = 2 as c_int;
static mut PATHCONSTRAINTMIX_Y: c_int = 3 as c_int;
#[no_mangle]
pub unsafe extern "C" fn _spPathConstraintMixTimeline_apply(
    mut timeline: *mut spTimeline,
    mut skeleton: *mut spSkeleton,
    mut _lastTime: c_float,
    mut time: c_float,
    mut _firedEvents: *mut *mut spEvent,
    mut _eventsCount: *mut c_int,
    mut alpha: c_float,
    mut blend: spMixBlend,
    mut _direction: spMixDirection,
) {
    let mut i: c_int = 0;
    let mut curveType: c_int = 0;
    let mut rotate: c_float = 0.;
    let mut x: c_float = 0.;
    let mut y: c_float = 0.;
    let mut t: c_float = 0.;
    let mut constraint: *mut spPathConstraint = 0 as *mut spPathConstraint;
    let mut self_0: *mut spPathConstraintMixTimeline = timeline as *mut spPathConstraintMixTimeline;
    let mut frames: *mut c_float = 0 as *mut c_float;
    let mut curves: *mut c_float = 0 as *mut c_float;
    constraint = *((*skeleton).pathConstraints).offset((*self_0).pathConstraintIndex as isize);
    if (*constraint).active == 0 {
        return;
    }
    frames = (*(*self_0).super_0.super_0.frames).items;
    curves = (*(*self_0).super_0.curves).items;
    if time < *frames.offset(0 as c_int as isize) {
        match blend as c_uint {
            0 => {
                (*constraint).mixRotate = (*(*constraint).data).mixRotate;
                (*constraint).mixX = (*(*constraint).data).mixX;
                (*constraint).mixY = (*(*constraint).data).mixY;
                return;
            }
            1 => {
                (*constraint).mixRotate +=
                    ((*(*constraint).data).mixRotate - (*constraint).mixRotate) * alpha;
                (*constraint).mixX += ((*(*constraint).data).mixX - (*constraint).mixX) * alpha;
                (*constraint).mixY += ((*(*constraint).data).mixY - (*constraint).mixY) * alpha;
            }
            _ => {}
        }
        return;
    }
    i = search2(
        (*self_0).super_0.super_0.frames,
        time,
        PATHCONSTRAINTMIX_ENTRIES,
    );
    curveType = *curves.offset((i >> 2 as c_int) as isize) as c_int;
    match curveType {
        0 => {
            let mut before: c_float = *frames.offset(i as isize);
            rotate = *frames.offset((i + PATHCONSTRAINTMIX_ROTATE) as isize);
            x = *frames.offset((i + PATHCONSTRAINTMIX_X) as isize);
            y = *frames.offset((i + PATHCONSTRAINTMIX_Y) as isize);
            t = (time - before)
                / (*frames.offset((i + PATHCONSTRAINTMIX_ENTRIES) as isize) - before);
            rotate += (*frames
                .offset((i + PATHCONSTRAINTMIX_ENTRIES + PATHCONSTRAINTMIX_ROTATE) as isize)
                - rotate)
                * t;
            x += (*frames.offset((i + PATHCONSTRAINTMIX_ENTRIES + PATHCONSTRAINTMIX_X) as isize)
                - x)
                * t;
            y += (*frames.offset((i + PATHCONSTRAINTMIX_ENTRIES + PATHCONSTRAINTMIX_Y) as isize)
                - y)
                * t;
        }
        1 => {
            rotate = *frames.offset((i + PATHCONSTRAINTMIX_ROTATE) as isize);
            x = *frames.offset((i + PATHCONSTRAINTMIX_X) as isize);
            y = *frames.offset((i + PATHCONSTRAINTMIX_Y) as isize);
        }
        _ => {
            rotate = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                PATHCONSTRAINTMIX_ROTATE,
                curveType - 2 as c_int,
            );
            x = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                PATHCONSTRAINTMIX_X,
                curveType + 18 as c_int - 2 as c_int,
            );
            y = _spCurveTimeline_getBezierValue(
                &mut (*self_0).super_0,
                time,
                i,
                PATHCONSTRAINTMIX_Y,
                curveType + 18 as c_int * 2 as c_int - 2 as c_int,
            );
        }
    }
    if blend as c_uint == SP_MIX_BLEND_SETUP as c_int as c_uint {
        let mut data: *mut spPathConstraintData = (*constraint).data;
        (*constraint).mixRotate = (*data).mixRotate + (rotate - (*data).mixRotate) * alpha;
        (*constraint).mixX = (*data).mixX + (x - (*data).mixX) * alpha;
        (*constraint).mixY = (*data).mixY + (y - (*data).mixY) * alpha;
    } else {
        (*constraint).mixRotate += (rotate - (*constraint).mixRotate) * alpha;
        (*constraint).mixX += (x - (*constraint).mixX) * alpha;
        (*constraint).mixY += (y - (*constraint).mixY) * alpha;
    };
}
#[no_mangle]
pub unsafe extern "C" fn spPathConstraintMixTimeline_create(
    mut framesCount: c_int,
    mut bezierCount: c_int,
    mut pathConstraintIndex: c_int,
) -> *mut spPathConstraintMixTimeline {
    let mut timeline: *mut spPathConstraintMixTimeline = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spPathConstraintMixTimeline>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        2720 as c_int,
    ) as *mut spPathConstraintMixTimeline;
    let mut ids: [spPropertyId; 1] = [0; 1];
    ids[0 as c_int as usize] = (SP_PROPERTY_PATHCONSTRAINT_MIX as c_int as spPropertyId)
        << 32 as c_int
        | pathConstraintIndex as c_ulong;
    _spCurveTimeline_init(
        &mut (*timeline).super_0,
        framesCount,
        PATHCONSTRAINTMIX_ENTRIES,
        bezierCount,
        ids.as_mut_ptr(),
        1 as c_int,
        SP_TIMELINE_PATHCONSTRAINTMIX,
        Some(_spCurveTimeline_dispose as unsafe extern "C" fn(*mut spTimeline) -> ()),
        Some(
            _spPathConstraintMixTimeline_apply
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    *mut spSkeleton,
                    c_float,
                    c_float,
                    *mut *mut spEvent,
                    *mut c_int,
                    c_float,
                    spMixBlend,
                    spMixDirection,
                ) -> (),
        ),
        Some(
            _spCurveTimeline_setBezier
                as unsafe extern "C" fn(
                    *mut spTimeline,
                    c_int,
                    c_int,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                    c_float,
                ) -> (),
        ),
    );
    (*timeline).pathConstraintIndex = pathConstraintIndex;
    return timeline;
}
#[no_mangle]
pub unsafe extern "C" fn spPathConstraintMixTimeline_setFrame(
    mut self_0: *mut spPathConstraintMixTimeline,
    mut frame: c_int,
    mut time: c_float,
    mut mixRotate: c_float,
    mut mixX: c_float,
    mut mixY: c_float,
) {
    let mut frames: *mut c_float = (*(*self_0).super_0.super_0.frames).items;
    frame *= PATHCONSTRAINTMIX_ENTRIES;
    *frames.offset(frame as isize) = time;
    *frames.offset((frame + PATHCONSTRAINTMIX_ROTATE) as isize) = mixRotate;
    *frames.offset((frame + PATHCONSTRAINTMIX_X) as isize) = mixX;
    *frames.offset((frame + PATHCONSTRAINTMIX_Y) as isize) = mixY;
}
#[no_mangle]
pub unsafe extern "C" fn spTrackEntryArray_create(
    mut initialCapacity: c_int,
) -> *mut spTrackEntryArray {
    let mut array: *mut spTrackEntryArray = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spTrackEntryArray>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        2781 as c_int,
    ) as *mut spTrackEntryArray;
    (*array).size = 0 as c_int;
    (*array).capacity = initialCapacity;
    (*array).items = _spCalloc(
        initialCapacity as size_t,
        ::core::mem::size_of::<*mut spTrackEntry>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        2781 as c_int,
    ) as *mut *mut spTrackEntry;
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn spTrackEntryArray_dispose(mut self_0: *mut spTrackEntryArray) {
    _spFree((*self_0).items as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spTrackEntryArray_clear(mut self_0: *mut spTrackEntryArray) {
    (*self_0).size = 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spTrackEntryArray_setSize(
    mut self_0: *mut spTrackEntryArray,
    mut newSize: c_int,
) -> *mut spTrackEntryArray {
    (*self_0).size = newSize;
    if (*self_0).capacity < newSize {
        (*self_0).capacity = if 8 as c_int > ((*self_0).size as c_float * 1.75f32) as c_int {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0).items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<*mut spTrackEntry>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut *mut spTrackEntry;
    }
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spTrackEntryArray_ensureCapacity(
    mut self_0: *mut spTrackEntryArray,
    mut newCapacity: c_int,
) {
    if (*self_0).capacity >= newCapacity {
        return;
    }
    (*self_0).capacity = newCapacity;
    (*self_0).items = _spRealloc(
        (*self_0).items as *mut c_void,
        (::core::mem::size_of::<*mut spTrackEntry>() as c_ulong)
            .wrapping_mul((*self_0).capacity as c_ulong),
    ) as *mut *mut spTrackEntry;
}
#[no_mangle]
pub unsafe extern "C" fn spTrackEntryArray_add(
    mut self_0: *mut spTrackEntryArray,
    mut value: *mut spTrackEntry,
) {
    if (*self_0).size == (*self_0).capacity {
        (*self_0).capacity = if 8 as c_int > ((*self_0).size as c_float * 1.75f32) as c_int {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0).items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<*mut spTrackEntry>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut *mut spTrackEntry;
    }
    let fresh17 = (*self_0).size;
    (*self_0).size = (*self_0).size + 1;
    let ref mut fresh18 = *((*self_0).items).offset(fresh17 as isize);
    *fresh18 = value;
}
#[no_mangle]
pub unsafe extern "C" fn spTrackEntryArray_addAll(
    mut self_0: *mut spTrackEntryArray,
    mut other: *mut spTrackEntryArray,
) {
    let mut i: c_int = 0 as c_int;
    while i < (*other).size {
        spTrackEntryArray_add(self_0, *((*other).items).offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spTrackEntryArray_addAllValues(
    mut self_0: *mut spTrackEntryArray,
    mut values: *mut *mut spTrackEntry,
    mut offset: c_int,
    mut count: c_int,
) {
    let mut i: c_int = offset;
    let mut n: c_int = offset + count;
    while i < n {
        spTrackEntryArray_add(self_0, *values.offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spTrackEntryArray_removeAt(
    mut self_0: *mut spTrackEntryArray,
    mut index: c_int,
) {
    (*self_0).size -= 1;
    spine_memmove(
        ((*self_0).items).offset(index as isize) as *mut c_void,
        ((*self_0).items)
            .offset(index as isize)
            .offset(1 as c_int as isize) as *const c_void,
        (::core::mem::size_of::<*mut spTrackEntry>() as c_ulong)
            .wrapping_mul(((*self_0).size - index) as c_ulong),
    );
}
#[no_mangle]
pub unsafe extern "C" fn spTrackEntryArray_contains(
    mut self_0: *mut spTrackEntryArray,
    mut value: *mut spTrackEntry,
) -> c_int {
    let mut items: *mut *mut spTrackEntry = (*self_0).items;
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    i = 0 as c_int;
    n = (*self_0).size;
    while i < n {
        if *items.offset(i as isize) == value {
            return -(1 as c_int);
        }
        i += 1;
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spTrackEntryArray_pop(
    mut self_0: *mut spTrackEntryArray,
) -> *mut spTrackEntry {
    (*self_0).size -= 1;
    let mut item: *mut spTrackEntry = *((*self_0).items).offset((*self_0).size as isize);
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn spTrackEntryArray_peek(
    mut self_0: *mut spTrackEntryArray,
) -> *mut spTrackEntry {
    return *((*self_0).items).offset(((*self_0).size - 1 as c_int) as isize);
}
static mut SP_EMPTY_ANIMATION: *mut spAnimation = 0 as *const spAnimation as *mut spAnimation;
#[no_mangle]
pub unsafe extern "C" fn spAnimationState_disposeStatics() {
    if !SP_EMPTY_ANIMATION.is_null() {
        spAnimation_dispose(SP_EMPTY_ANIMATION);
    }
    SP_EMPTY_ANIMATION = 0 as *mut spAnimation;
}
#[no_mangle]
pub unsafe extern "C" fn _spEventQueue_create(
    mut state: *mut _spAnimationState,
) -> *mut _spEventQueue {
    let mut self_0: *mut _spEventQueue = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<_spEventQueue>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        2829 as c_int,
    ) as *mut _spEventQueue;
    (*self_0).state = state;
    (*self_0).objectsCount = 0 as c_int;
    (*self_0).objectsCapacity = 16 as c_int;
    (*self_0).objects = _spCalloc(
        (*self_0).objectsCapacity as size_t,
        ::core::mem::size_of::<_spEventQueueItem>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        2833 as c_int,
    ) as *mut _spEventQueueItem;
    (*self_0).drainDisabled = 0 as c_int;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn _spEventQueue_free(mut self_0: *mut _spEventQueue) {
    _spFree((*self_0).objects as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _spEventQueue_ensureCapacity(
    mut self_0: *mut _spEventQueue,
    mut newElements: c_int,
) {
    if (*self_0).objectsCount + newElements > (*self_0).objectsCapacity {
        let mut newObjects: *mut _spEventQueueItem = 0 as *mut _spEventQueueItem;
        (*self_0).objectsCapacity <<= 1 as c_int;
        newObjects = _spCalloc(
            (*self_0).objectsCapacity as size_t,
            ::core::mem::size_of::<_spEventQueueItem>() as c_ulong,
            b"spine.c\0" as *const u8 as *const c_char,
            2847 as c_int,
        ) as *mut _spEventQueueItem;
        spine_memcpy(
            newObjects as *mut c_void,
            (*self_0).objects as *const c_void,
            (::core::mem::size_of::<_spEventQueueItem>() as c_ulong)
                .wrapping_mul((*self_0).objectsCount as c_ulong),
        );
        _spFree((*self_0).objects as *mut c_void);
        (*self_0).objects = newObjects;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _spEventQueue_addType(
    mut self_0: *mut _spEventQueue,
    mut type_0: spEventType,
) {
    _spEventQueue_ensureCapacity(self_0, 1 as c_int);
    let fresh19 = (*self_0).objectsCount;
    (*self_0).objectsCount = (*self_0).objectsCount + 1;
    (*((*self_0).objects).offset(fresh19 as isize)).type_0 = type_0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _spEventQueue_addEntry(
    mut self_0: *mut _spEventQueue,
    mut entry: *mut spTrackEntry,
) {
    _spEventQueue_ensureCapacity(self_0, 1 as c_int);
    let fresh20 = (*self_0).objectsCount;
    (*self_0).objectsCount = (*self_0).objectsCount + 1;
    let ref mut fresh21 = (*((*self_0).objects).offset(fresh20 as isize)).entry;
    *fresh21 = entry;
}
#[no_mangle]
pub unsafe extern "C" fn _spEventQueue_addEvent(
    mut self_0: *mut _spEventQueue,
    mut event: *mut spEvent,
) {
    _spEventQueue_ensureCapacity(self_0, 1 as c_int);
    let fresh22 = (*self_0).objectsCount;
    (*self_0).objectsCount = (*self_0).objectsCount + 1;
    let ref mut fresh23 = (*((*self_0).objects).offset(fresh22 as isize)).event;
    *fresh23 = event;
}
#[no_mangle]
pub unsafe extern "C" fn _spEventQueue_start(
    mut self_0: *mut _spEventQueue,
    mut entry: *mut spTrackEntry,
) {
    _spEventQueue_addType(self_0, SP_ANIMATION_START);
    _spEventQueue_addEntry(self_0, entry);
    (*(*self_0).state).animationsChanged = 1 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _spEventQueue_interrupt(
    mut self_0: *mut _spEventQueue,
    mut entry: *mut spTrackEntry,
) {
    _spEventQueue_addType(self_0, SP_ANIMATION_INTERRUPT);
    _spEventQueue_addEntry(self_0, entry);
}
#[no_mangle]
pub unsafe extern "C" fn _spEventQueue_end(
    mut self_0: *mut _spEventQueue,
    mut entry: *mut spTrackEntry,
) {
    _spEventQueue_addType(self_0, SP_ANIMATION_END);
    _spEventQueue_addEntry(self_0, entry);
    (*(*self_0).state).animationsChanged = 1 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _spEventQueue_dispose(
    mut self_0: *mut _spEventQueue,
    mut entry: *mut spTrackEntry,
) {
    _spEventQueue_addType(self_0, SP_ANIMATION_DISPOSE);
    _spEventQueue_addEntry(self_0, entry);
}
#[no_mangle]
pub unsafe extern "C" fn _spEventQueue_complete(
    mut self_0: *mut _spEventQueue,
    mut entry: *mut spTrackEntry,
) {
    _spEventQueue_addType(self_0, SP_ANIMATION_COMPLETE);
    _spEventQueue_addEntry(self_0, entry);
}
#[no_mangle]
pub unsafe extern "C" fn _spEventQueue_event(
    mut self_0: *mut _spEventQueue,
    mut entry: *mut spTrackEntry,
    mut event: *mut spEvent,
) {
    _spEventQueue_addType(self_0, SP_ANIMATION_EVENT);
    _spEventQueue_addEntry(self_0, entry);
    _spEventQueue_addEvent(self_0, event);
}
#[no_mangle]
pub unsafe extern "C" fn _spEventQueue_clear(mut self_0: *mut _spEventQueue) {
    (*self_0).objectsCount = 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _spEventQueue_drain(mut self_0: *mut _spEventQueue) {
    let mut i: c_int = 0;
    if (*self_0).drainDisabled != 0 {
        return;
    }
    (*self_0).drainDisabled = 1 as c_int;
    i = 0 as c_int;
    while i < (*self_0).objectsCount {
        let mut type_0: spEventType =
            (*((*self_0).objects).offset(i as isize)).type_0 as spEventType;
        let mut entry: *mut spTrackEntry =
            (*((*self_0).objects).offset((i + 1 as c_int) as isize)).entry;
        let mut event: *mut spEvent = 0 as *mut spEvent;
        let mut current_block_22: u64;
        match type_0 as c_uint {
            0 | 1 | 3 => {
                if ((*entry).listener).is_some() {
                    ((*entry).listener).expect("non-null function pointer")(
                        &mut (*(*self_0).state).super_0,
                        type_0,
                        entry,
                        0 as *mut spEvent,
                    );
                }
                if ((*(*self_0).state).super_0.listener).is_some() {
                    ((*(*self_0).state).super_0.listener).expect("non-null function pointer")(
                        &mut (*(*self_0).state).super_0,
                        type_0,
                        entry,
                        0 as *mut spEvent,
                    );
                }
                current_block_22 = 10043043949733653460;
            }
            2 => {
                if ((*entry).listener).is_some() {
                    ((*entry).listener).expect("non-null function pointer")(
                        &mut (*(*self_0).state).super_0,
                        type_0,
                        entry,
                        0 as *mut spEvent,
                    );
                }
                if ((*(*self_0).state).super_0.listener).is_some() {
                    ((*(*self_0).state).super_0.listener).expect("non-null function pointer")(
                        &mut (*(*self_0).state).super_0,
                        type_0,
                        entry,
                        0 as *mut spEvent,
                    );
                }
                current_block_22 = 1019940178787807919;
            }
            4 => {
                current_block_22 = 1019940178787807919;
            }
            5 => {
                event = (*((*self_0).objects).offset((i + 2 as c_int) as isize)).event;
                if ((*entry).listener).is_some() {
                    ((*entry).listener).expect("non-null function pointer")(
                        &mut (*(*self_0).state).super_0,
                        type_0,
                        entry,
                        event,
                    );
                }
                if ((*(*self_0).state).super_0.listener).is_some() {
                    ((*(*self_0).state).super_0.listener).expect("non-null function pointer")(
                        &mut (*(*self_0).state).super_0,
                        type_0,
                        entry,
                        event,
                    );
                }
                i += 1;
                current_block_22 = 10043043949733653460;
            }
            _ => {
                current_block_22 = 10043043949733653460;
            }
        }
        match current_block_22 {
            1019940178787807919 => {
                if ((*entry).listener).is_some() {
                    ((*entry).listener).expect("non-null function pointer")(
                        &mut (*(*self_0).state).super_0,
                        SP_ANIMATION_DISPOSE,
                        entry,
                        0 as *mut spEvent,
                    );
                }
                if ((*(*self_0).state).super_0.listener).is_some() {
                    ((*(*self_0).state).super_0.listener).expect("non-null function pointer")(
                        &mut (*(*self_0).state).super_0,
                        SP_ANIMATION_DISPOSE,
                        entry,
                        0 as *mut spEvent,
                    );
                }
                _spAnimationState_disposeTrackEntry(entry);
            }
            _ => {}
        }
        i += 2 as c_int;
    }
    _spEventQueue_clear(self_0);
    (*self_0).drainDisabled = 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _spAnimationState_enableQueue(mut self_0: *mut spAnimationState) {
    let mut internal: *mut _spAnimationState = self_0 as *mut _spAnimationState;
    (*(*internal).queue).drainDisabled = 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _spAnimationState_disableQueue(mut self_0: *mut spAnimationState) {
    let mut internal: *mut _spAnimationState = self_0 as *mut _spAnimationState;
    (*(*internal).queue).drainDisabled = 1 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _spAnimationState_disposeTrackEntry(mut entry: *mut spTrackEntry) {
    spIntArray_dispose((*entry).timelineMode);
    spTrackEntryArray_dispose((*entry).timelineHoldMix);
    _spFree((*entry).timelinesRotation as *mut c_void);
    _spFree(entry as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _spAnimationState_disposeTrackEntries(
    mut state: *mut spAnimationState,
    mut entry: *mut spTrackEntry,
) {
    while !entry.is_null() {
        let mut next: *mut spTrackEntry = (*entry).next;
        let mut from: *mut spTrackEntry = (*entry).mixingFrom;
        while !from.is_null() {
            let mut nextFrom: *mut spTrackEntry = (*from).mixingFrom;
            if ((*entry).listener).is_some() {
                ((*entry).listener).expect("non-null function pointer")(
                    state,
                    SP_ANIMATION_DISPOSE,
                    from,
                    0 as *mut spEvent,
                );
            }
            if ((*state).listener).is_some() {
                ((*state).listener).expect("non-null function pointer")(
                    state,
                    SP_ANIMATION_DISPOSE,
                    from,
                    0 as *mut spEvent,
                );
            }
            _spAnimationState_disposeTrackEntry(from);
            from = nextFrom;
        }
        if ((*entry).listener).is_some() {
            ((*entry).listener).expect("non-null function pointer")(
                state,
                SP_ANIMATION_DISPOSE,
                entry,
                0 as *mut spEvent,
            );
        }
        if ((*state).listener).is_some() {
            ((*state).listener).expect("non-null function pointer")(
                state,
                SP_ANIMATION_DISPOSE,
                entry,
                0 as *mut spEvent,
            );
        }
        _spAnimationState_disposeTrackEntry(entry);
        entry = next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spAnimationState_create(
    mut data: *mut spAnimationStateData,
) -> *mut spAnimationState {
    let mut internal: *mut _spAnimationState = 0 as *mut _spAnimationState;
    let mut self_0: *mut spAnimationState = 0 as *mut spAnimationState;
    if SP_EMPTY_ANIMATION.is_null() {
        SP_EMPTY_ANIMATION = 1 as c_int as *mut spAnimation;
        SP_EMPTY_ANIMATION = spAnimation_create(
            b"<empty>\0" as *const u8 as *const c_char,
            0 as *mut spTimelineArray,
            0 as c_int as c_float,
        );
    }
    internal = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<_spAnimationState>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        2989 as c_int,
    ) as *mut _spAnimationState;
    self_0 = &mut (*internal).super_0;
    let ref mut fresh24 =
        *(&(*self_0).data as *const *mut spAnimationStateData as *mut *mut spAnimationStateData);
    *fresh24 = data;
    (*self_0).timeScale = 1 as c_int as c_float;
    (*internal).queue = _spEventQueue_create(internal);
    (*internal).events = _spCalloc(
        128 as c_int as size_t,
        ::core::mem::size_of::<*mut spEvent>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        2996 as c_int,
    ) as *mut *mut spEvent;
    (*internal).propertyIDs = _spCalloc(
        128 as c_int as size_t,
        ::core::mem::size_of::<spPropertyId>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        2998 as c_int,
    ) as *mut spPropertyId;
    (*internal).propertyIDsCapacity = 128 as c_int;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spAnimationState_dispose(mut self_0: *mut spAnimationState) {
    let mut i: c_int = 0;
    let mut internal: *mut _spAnimationState = self_0 as *mut _spAnimationState;
    i = 0 as c_int;
    while i < (*self_0).tracksCount {
        _spAnimationState_disposeTrackEntries(self_0, *((*self_0).tracks).offset(i as isize));
        i += 1;
    }
    _spFree((*self_0).tracks as *mut c_void);
    _spEventQueue_free((*internal).queue);
    _spFree((*internal).events as *mut c_void);
    _spFree((*internal).propertyIDs as *mut c_void);
    _spFree(internal as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spAnimationState_update(
    mut self_0: *mut spAnimationState,
    mut delta: c_float,
) {
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    let mut internal: *mut _spAnimationState = self_0 as *mut _spAnimationState;
    delta *= (*self_0).timeScale;
    let mut current_block_29: u64;
    i = 0 as c_int;
    n = (*self_0).tracksCount;
    while i < n {
        let mut currentDelta: c_float = 0.;
        let mut current: *mut spTrackEntry = *((*self_0).tracks).offset(i as isize);
        let mut next: *mut spTrackEntry = 0 as *mut spTrackEntry;
        if !current.is_null() {
            (*current).animationLast = (*current).nextAnimationLast;
            (*current).trackLast = (*current).nextTrackLast;
            currentDelta = delta * (*current).timeScale;
            if (*current).delay > 0 as c_int as c_float {
                (*current).delay -= currentDelta;
                if (*current).delay > 0 as c_int as c_float {
                    current_block_29 = 16559507199688588974;
                } else {
                    currentDelta = -(*current).delay;
                    (*current).delay = 0 as c_int as c_float;
                    current_block_29 = 17965632435239708295;
                }
            } else {
                current_block_29 = 17965632435239708295;
            }
            match current_block_29 {
                16559507199688588974 => {}
                _ => {
                    next = (*current).next;
                    if !next.is_null() {
                        let mut nextTime: c_float = (*current).trackLast - (*next).delay;
                        if nextTime >= 0 as c_int as c_float {
                            (*next).delay = 0 as c_int as c_float;
                            (*next).trackTime += if (*current).timeScale == 0 as c_int as c_float {
                                0 as c_int as c_float
                            } else {
                                (nextTime / (*current).timeScale + delta) * (*next).timeScale
                            };
                            (*current).trackTime += currentDelta;
                            _spAnimationState_setCurrent(self_0, i, next, 1 as c_int);
                            while !((*next).mixingFrom).is_null() {
                                (*next).mixTime += delta;
                                next = (*next).mixingFrom;
                            }
                            current_block_29 = 16559507199688588974;
                        } else {
                            current_block_29 = 17478428563724192186;
                        }
                    } else if (*current).trackLast >= (*current).trackEnd
                        && ((*current).mixingFrom).is_null()
                    {
                        let ref mut fresh25 = *((*self_0).tracks).offset(i as isize);
                        *fresh25 = 0 as *mut spTrackEntry;
                        _spEventQueue_end((*internal).queue, current);
                        spAnimationState_clearNext(self_0, current);
                        current_block_29 = 16559507199688588974;
                    } else {
                        current_block_29 = 17478428563724192186;
                    }
                    match current_block_29 {
                        16559507199688588974 => {}
                        _ => {
                            if !((*current).mixingFrom).is_null()
                                && _spAnimationState_updateMixingFrom(self_0, current, delta) != 0
                            {
                                let mut from: *mut spTrackEntry = (*current).mixingFrom;
                                (*current).mixingFrom = 0 as *mut spTrackEntry;
                                if !from.is_null() {
                                    (*from).mixingTo = 0 as *mut spTrackEntry;
                                }
                                while !from.is_null() {
                                    _spEventQueue_end((*internal).queue, from);
                                    from = (*from).mixingFrom;
                                }
                            }
                            (*current).trackTime += currentDelta;
                        }
                    }
                }
            }
        }
        i += 1;
    }
    _spEventQueue_drain((*internal).queue);
}
#[no_mangle]
pub unsafe extern "C" fn _spAnimationState_updateMixingFrom(
    mut self_0: *mut spAnimationState,
    mut to: *mut spTrackEntry,
    mut delta: c_float,
) -> c_int {
    let mut from: *mut spTrackEntry = (*to).mixingFrom;
    let mut finished: c_int = 0;
    let mut internal: *mut _spAnimationState = self_0 as *mut _spAnimationState;
    if from.is_null() {
        return -(1 as c_int);
    }
    finished = _spAnimationState_updateMixingFrom(self_0, from, delta);
    (*from).animationLast = (*from).nextAnimationLast;
    (*from).trackLast = (*from).nextTrackLast;
    if (*to).mixTime > 0 as c_int as c_float && (*to).mixTime >= (*to).mixDuration {
        if (*from).totalAlpha == 0 as c_int as c_float || (*to).mixDuration == 0 as c_int as c_float
        {
            (*to).mixingFrom = (*from).mixingFrom;
            if !((*from).mixingFrom).is_null() {
                (*(*from).mixingFrom).mixingTo = to;
            }
            (*to).interruptAlpha = (*from).interruptAlpha;
            _spEventQueue_end((*internal).queue, from);
        }
        return finished;
    }
    (*from).trackTime += delta * (*from).timeScale;
    (*to).mixTime += delta;
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spAnimationState_apply(
    mut self_0: *mut spAnimationState,
    mut skeleton: *mut spSkeleton,
) -> c_int {
    let mut internal: *mut _spAnimationState = self_0 as *mut _spAnimationState;
    let mut current: *mut spTrackEntry = 0 as *mut spTrackEntry;
    let mut i: c_int = 0;
    let mut ii: c_int = 0;
    let mut n: c_int = 0;
    let mut animationLast: c_float = 0.;
    let mut animationTime: c_float = 0.;
    let mut timelineCount: c_int = 0;
    let mut timelines: *mut *mut spTimeline = 0 as *mut *mut spTimeline;
    let mut firstFrame: c_int = 0;
    let mut shortestRotation: c_int = 0;
    let mut timelinesRotation: *mut c_float = 0 as *mut c_float;
    let mut timeline: *mut spTimeline = 0 as *mut spTimeline;
    let mut applied: c_int = 0 as c_int;
    let mut blend: spMixBlend = SP_MIX_BLEND_SETUP;
    let mut timelineBlend: spMixBlend = SP_MIX_BLEND_SETUP;
    let mut setupState: c_int = 0 as c_int;
    let mut slots: *mut *mut spSlot = 0 as *mut *mut spSlot;
    let mut slot: *mut spSlot = 0 as *mut spSlot;
    let mut attachmentName: *const c_char = 0 as *const c_char;
    let mut applyEvents: *mut *mut spEvent = 0 as *mut *mut spEvent;
    let mut applyTime: c_float = 0.;
    if (*internal).animationsChanged != 0 {
        _spAnimationState_animationsChanged(self_0);
    }
    i = 0 as c_int;
    n = (*self_0).tracksCount;
    while i < n {
        let mut mix: c_float = 0.;
        current = *((*self_0).tracks).offset(i as isize);
        if !(current.is_null() || (*current).delay > 0 as c_int as c_float) {
            applied = -(1 as c_int);
            blend = (if i == 0 as c_int {
                SP_MIX_BLEND_FIRST as c_int as c_uint
            } else {
                (*current).mixBlend as c_uint
            }) as spMixBlend;
            mix = (*current).alpha;
            if !((*current).mixingFrom).is_null() {
                mix *= _spAnimationState_applyMixingFrom(self_0, current, skeleton, blend);
            } else if (*current).trackTime >= (*current).trackEnd && ((*current).next).is_null() {
                mix = 0 as c_int as c_float;
            }
            animationLast = (*current).animationLast;
            animationTime = spTrackEntry_getAnimationTime(current);
            timelineCount = (*(*(*current).animation).timelines).size;
            applyEvents = (*internal).events;
            applyTime = animationTime;
            if (*current).reverse != 0 {
                applyTime = (*(*current).animation).duration - applyTime;
                applyEvents = 0 as *mut *mut spEvent;
            }
            timelines = (*(*(*current).animation).timelines).items;
            if i == 0 as c_int && mix == 1 as c_int as c_float
                || blend as c_uint == SP_MIX_BLEND_ADD as c_int as c_uint
            {
                ii = 0 as c_int;
                while ii < timelineCount {
                    timeline = *timelines.offset(ii as isize);
                    if (*timeline).propertyIds[0 as c_int as usize]
                        == SP_PROPERTY_ATTACHMENT as c_int as c_ulong
                    {
                        _spAnimationState_applyAttachmentTimeline(
                            self_0,
                            timeline,
                            skeleton,
                            applyTime,
                            blend,
                            -(1 as c_int),
                        );
                    } else {
                        spTimeline_apply(
                            *timelines.offset(ii as isize),
                            skeleton,
                            animationLast,
                            applyTime,
                            applyEvents,
                            &mut (*internal).eventsCount,
                            mix,
                            blend,
                            SP_MIX_DIRECTION_IN,
                        );
                    }
                    ii += 1;
                }
            } else {
                let mut timelineMode: *mut spIntArray = (*current).timelineMode;
                shortestRotation = (*current).shortestRotation;
                firstFrame = (shortestRotation == 0
                    && (*current).timelinesRotationCount != timelineCount << 1 as c_int)
                    as c_int;
                if firstFrame != 0 {
                    _spAnimationState_resizeTimelinesRotation(current, timelineCount << 1 as c_int);
                }
                timelinesRotation = (*current).timelinesRotation;
                ii = 0 as c_int;
                while ii < timelineCount {
                    timeline = *timelines.offset(ii as isize);
                    timelineBlend = (if *((*timelineMode).items).offset(ii as isize) == 0 as c_int {
                        blend as c_uint
                    } else {
                        SP_MIX_BLEND_SETUP as c_int as c_uint
                    }) as spMixBlend;
                    if shortestRotation == 0
                        && (*timeline).propertyIds[0 as c_int as usize]
                            == SP_PROPERTY_ROTATE as c_int as c_ulong
                    {
                        _spAnimationState_applyRotateTimeline(
                            self_0,
                            timeline,
                            skeleton,
                            applyTime,
                            mix,
                            timelineBlend,
                            timelinesRotation,
                            ii << 1 as c_int,
                            firstFrame,
                        );
                    } else if (*timeline).propertyIds[0 as c_int as usize]
                        == SP_PROPERTY_ATTACHMENT as c_int as c_ulong
                    {
                        _spAnimationState_applyAttachmentTimeline(
                            self_0,
                            timeline,
                            skeleton,
                            applyTime,
                            timelineBlend,
                            -(1 as c_int),
                        );
                    } else {
                        spTimeline_apply(
                            timeline,
                            skeleton,
                            animationLast,
                            applyTime,
                            applyEvents,
                            &mut (*internal).eventsCount,
                            mix,
                            timelineBlend,
                            SP_MIX_DIRECTION_IN,
                        );
                    }
                    ii += 1;
                }
            }
            _spAnimationState_queueEvents(self_0, current, animationTime);
            (*internal).eventsCount = 0 as c_int;
            (*current).nextAnimationLast = animationTime;
            (*current).nextTrackLast = (*current).trackTime;
        }
        i += 1;
    }
    setupState = (*self_0).unkeyedState + 1 as c_int;
    slots = (*skeleton).slots;
    i = 0 as c_int;
    n = (*skeleton).slotsCount;
    while i < n {
        slot = *slots.offset(i as isize);
        if (*slot).attachmentState == setupState {
            attachmentName = (*(*slot).data).attachmentName;
            spSlot_setAttachment(
                slot,
                if attachmentName.is_null() {
                    0 as *mut spAttachment
                } else {
                    spSkeleton_getAttachmentForSlotIndex(
                        skeleton,
                        (*(*slot).data).index,
                        attachmentName,
                    )
                },
            );
        }
        i += 1;
    }
    (*self_0).unkeyedState += 2 as c_int;
    _spEventQueue_drain((*internal).queue);
    return applied;
}
#[no_mangle]
pub unsafe extern "C" fn _spAnimationState_applyMixingFrom(
    mut self_0: *mut spAnimationState,
    mut to: *mut spTrackEntry,
    mut skeleton: *mut spSkeleton,
    mut blend: spMixBlend,
) -> c_float {
    let mut internal: *mut _spAnimationState = self_0 as *mut _spAnimationState;
    let mut mix: c_float = 0.;
    let mut events: *mut *mut spEvent = 0 as *mut *mut spEvent;
    let mut attachments: c_int = 0;
    let mut drawOrder: c_int = 0;
    let mut animationLast: c_float = 0.;
    let mut animationTime: c_float = 0.;
    let mut timelineCount: c_int = 0;
    let mut timelines: *mut *mut spTimeline = 0 as *mut *mut spTimeline;
    let mut timelineMode: *mut spIntArray = 0 as *mut spIntArray;
    let mut timelineHoldMix: *mut spTrackEntryArray = 0 as *mut spTrackEntryArray;
    let mut timelineBlend: spMixBlend = SP_MIX_BLEND_SETUP;
    let mut alphaHold: c_float = 0.;
    let mut alphaMix: c_float = 0.;
    let mut alpha: c_float = 0.;
    let mut firstFrame: c_int = 0;
    let mut shortestRotation: c_int = 0;
    let mut timelinesRotation: *mut c_float = 0 as *mut c_float;
    let mut i: c_int = 0;
    let mut holdMix: *mut spTrackEntry = 0 as *mut spTrackEntry;
    let mut applyTime: c_float = 0.;
    let mut from: *mut spTrackEntry = (*to).mixingFrom;
    if !((*from).mixingFrom).is_null() {
        _spAnimationState_applyMixingFrom(self_0, from, skeleton, blend);
    }
    if (*to).mixDuration == 0 as c_int as c_float {
        mix = 1 as c_int as c_float;
        if blend as c_uint == SP_MIX_BLEND_FIRST as c_int as c_uint {
            blend = SP_MIX_BLEND_SETUP;
        }
    } else {
        mix = (*to).mixTime / (*to).mixDuration;
        if mix > 1 as c_int as c_float {
            mix = 1 as c_int as c_float;
        }
        if blend as c_uint != SP_MIX_BLEND_FIRST as c_int as c_uint {
            blend = (*from).mixBlend;
        }
    }
    attachments = (mix < (*from).attachmentThreshold) as c_int;
    drawOrder = (mix < (*from).drawOrderThreshold) as c_int;
    timelineCount = (*(*(*from).animation).timelines).size;
    timelines = (*(*(*from).animation).timelines).items;
    alphaHold = (*from).alpha * (*to).interruptAlpha;
    alphaMix = alphaHold * (1 as c_int as c_float - mix);
    animationLast = (*from).animationLast;
    animationTime = spTrackEntry_getAnimationTime(from);
    applyTime = animationTime;
    events = 0 as *mut *mut spEvent;
    if (*from).reverse != 0 {
        applyTime = (*(*from).animation).duration - applyTime;
    } else if mix < (*from).eventThreshold {
        events = (*internal).events;
    }
    if blend as c_uint == SP_MIX_BLEND_ADD as c_int as c_uint {
        i = 0 as c_int;
        while i < timelineCount {
            let mut timeline: *mut spTimeline = *timelines.offset(i as isize);
            spTimeline_apply(
                timeline,
                skeleton,
                animationLast,
                applyTime,
                events,
                &mut (*internal).eventsCount,
                alphaMix,
                blend,
                SP_MIX_DIRECTION_OUT,
            );
            i += 1;
        }
    } else {
        timelineMode = (*from).timelineMode;
        timelineHoldMix = (*from).timelineHoldMix;
        shortestRotation = (*from).shortestRotation;
        firstFrame = (shortestRotation == 0
            && (*from).timelinesRotationCount != timelineCount << 1 as c_int)
            as c_int;
        if firstFrame != 0 {
            _spAnimationState_resizeTimelinesRotation(from, timelineCount << 1 as c_int);
        }
        timelinesRotation = (*from).timelinesRotation;
        (*from).totalAlpha = 0 as c_int as c_float;
        let mut current_block_62: u64;
        i = 0 as c_int;
        while i < timelineCount {
            let mut direction: spMixDirection = SP_MIX_DIRECTION_OUT;
            let mut timeline_0: *mut spTimeline = *timelines.offset(i as isize);
            match *((*timelineMode).items).offset(i as isize) {
                0 => {
                    if drawOrder == 0
                        && (*timeline_0).propertyIds[0 as c_int as usize]
                            == SP_PROPERTY_DRAWORDER as c_int as c_ulong
                    {
                        current_block_62 = 572715077006366937;
                    } else {
                        timelineBlend = blend;
                        alpha = alphaMix;
                        current_block_62 = 12829669402821218572;
                    }
                }
                1 => {
                    timelineBlend = SP_MIX_BLEND_SETUP;
                    alpha = alphaMix;
                    current_block_62 = 12829669402821218572;
                }
                2 => {
                    timelineBlend = blend;
                    alpha = alphaHold;
                    current_block_62 = 12829669402821218572;
                }
                3 => {
                    timelineBlend = SP_MIX_BLEND_SETUP;
                    alpha = alphaHold;
                    current_block_62 = 12829669402821218572;
                }
                _ => {
                    timelineBlend = SP_MIX_BLEND_SETUP;
                    holdMix = *((*timelineHoldMix).items).offset(i as isize);
                    alpha = alphaHold
                        * (if 0 as c_int as c_float
                            > 1 as c_int as c_float - (*holdMix).mixTime / (*holdMix).mixDuration
                        {
                            0 as c_int as c_float
                        } else {
                            1 as c_int as c_float - (*holdMix).mixTime / (*holdMix).mixDuration
                        });
                    current_block_62 = 12829669402821218572;
                }
            }
            match current_block_62 {
                12829669402821218572 => {
                    (*from).totalAlpha += alpha;
                    if shortestRotation == 0
                        && (*timeline_0).propertyIds[0 as c_int as usize]
                            == SP_PROPERTY_ROTATE as c_int as c_ulong
                    {
                        _spAnimationState_applyRotateTimeline(
                            self_0,
                            timeline_0,
                            skeleton,
                            applyTime,
                            alpha,
                            timelineBlend,
                            timelinesRotation,
                            i << 1 as c_int,
                            firstFrame,
                        );
                    } else if (*timeline_0).propertyIds[0 as c_int as usize]
                        == SP_PROPERTY_ATTACHMENT as c_int as c_ulong
                    {
                        _spAnimationState_applyAttachmentTimeline(
                            self_0,
                            timeline_0,
                            skeleton,
                            applyTime,
                            timelineBlend,
                            attachments,
                        );
                    } else {
                        if drawOrder != 0
                            && (*timeline_0).propertyIds[0 as c_int as usize]
                                == SP_PROPERTY_DRAWORDER as c_int as c_ulong
                            && timelineBlend as c_uint == SP_MIX_BLEND_SETUP as c_int as c_uint
                        {
                            direction = SP_MIX_DIRECTION_IN;
                        }
                        spTimeline_apply(
                            timeline_0,
                            skeleton,
                            animationLast,
                            applyTime,
                            events,
                            &mut (*internal).eventsCount,
                            alpha,
                            timelineBlend,
                            direction,
                        );
                    }
                }
                _ => {}
            }
            i += 1;
        }
    }
    if (*to).mixDuration > 0 as c_int as c_float {
        _spAnimationState_queueEvents(self_0, from, animationTime);
    }
    (*internal).eventsCount = 0 as c_int;
    (*from).nextAnimationLast = animationTime;
    (*from).nextTrackLast = (*from).trackTime;
    return mix;
}
unsafe extern "C" fn _spAnimationState_setAttachment(
    mut self_0: *mut spAnimationState,
    mut skeleton: *mut spSkeleton,
    mut slot: *mut spSlot,
    mut attachmentName: *const c_char,
    mut attachments: c_int,
) {
    spSlot_setAttachment(
        slot,
        if attachmentName.is_null() {
            0 as *mut spAttachment
        } else {
            spSkeleton_getAttachmentForSlotIndex(skeleton, (*(*slot).data).index, attachmentName)
        },
    );
    if attachments != 0 {
        (*slot).attachmentState = (*self_0).unkeyedState + 2 as c_int;
    }
}
unsafe extern "C" fn binarySearch1(
    mut values: *mut c_float,
    mut valuesLength: c_int,
    mut target: c_float,
) -> c_int {
    let mut low: c_int = 0 as c_int;
    let mut current: c_int = 0;
    let mut high: c_int = valuesLength - 2 as c_int;
    if high == 0 as c_int {
        return 1 as c_int;
    }
    current = high >> 1 as c_int;
    loop {
        if *values.offset((current + 1 as c_int) as isize) <= target {
            low = current + 1 as c_int;
        } else {
            high = current;
        }
        if low == high {
            return low + 1 as c_int;
        }
        current = low + high >> 1 as c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _spAnimationState_applyAttachmentTimeline(
    mut self_0: *mut spAnimationState,
    mut timeline: *mut spTimeline,
    mut skeleton: *mut spSkeleton,
    mut time: c_float,
    mut blend: spMixBlend,
    mut attachments: c_int,
) {
    let mut attachmentTimeline: *mut spAttachmentTimeline = 0 as *mut spAttachmentTimeline;
    let mut slot: *mut spSlot = 0 as *mut spSlot;
    let mut frames: *mut c_float = 0 as *mut c_float;
    attachmentTimeline = timeline as *mut spAttachmentTimeline;
    slot = *((*skeleton).slots).offset((*attachmentTimeline).slotIndex as isize);
    if (*(*slot).bone).active == 0 {
        return;
    }
    frames = (*(*attachmentTimeline).super_0.frames).items;
    if time < *frames.offset(0 as c_int as isize) {
        if blend as c_uint == SP_MIX_BLEND_SETUP as c_int as c_uint
            || blend as c_uint == SP_MIX_BLEND_FIRST as c_int as c_uint
        {
            _spAnimationState_setAttachment(
                self_0,
                skeleton,
                slot,
                (*(*slot).data).attachmentName,
                attachments,
            );
        }
    } else {
        _spAnimationState_setAttachment(
            self_0,
            skeleton,
            slot,
            *((*attachmentTimeline).attachmentNames).offset(binarySearch1(
                frames,
                (*(*attachmentTimeline).super_0.frames).size,
                time,
            ) as isize),
            attachments,
        );
    }
    if (*slot).attachmentState <= (*self_0).unkeyedState {
        (*slot).attachmentState = (*self_0).unkeyedState + 1 as c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _spAnimationState_applyRotateTimeline(
    mut _self_0: *mut spAnimationState,
    mut timeline: *mut spTimeline,
    mut skeleton: *mut spSkeleton,
    mut time: c_float,
    mut alpha: c_float,
    mut blend: spMixBlend,
    mut timelinesRotation: *mut c_float,
    mut i: c_int,
    mut firstFrame: c_int,
) {
    let mut rotateTimeline: *mut spRotateTimeline = 0 as *mut spRotateTimeline;
    let mut frames: *mut c_float = 0 as *mut c_float;
    let mut bone: *mut spBone = 0 as *mut spBone;
    let mut r1: c_float = 0.;
    let mut r2: c_float = 0.;
    let mut total: c_float = 0.;
    let mut diff: c_float = 0.;
    let mut current: c_int = 0;
    let mut dir: c_int = 0;
    if firstFrame != 0 {
        *timelinesRotation.offset(i as isize) = 0 as c_int as c_float;
    }
    if alpha == 1 as c_int as c_float {
        spTimeline_apply(
            timeline,
            skeleton,
            0 as c_int as c_float,
            time,
            0 as *mut *mut spEvent,
            0 as *mut c_int,
            1 as c_int as c_float,
            blend,
            SP_MIX_DIRECTION_IN,
        );
        return;
    }
    rotateTimeline = timeline as *mut spRotateTimeline;
    frames = (*(*rotateTimeline).super_0.super_0.frames).items;
    bone = *((*skeleton).bones).offset((*rotateTimeline).boneIndex as isize);
    if (*bone).active == 0 {
        return;
    }
    if time < *frames.offset(0 as c_int as isize) {
        {
            match blend as c_uint {
                0 => {
                    (*bone).rotation = (*(*bone).data).rotation;
                    return;
                }
                1 => {
                    r1 = (*bone).rotation;
                    r2 = (*(*bone).data).rotation;
                }
                _ => {
                    return;
                }
            }
        }
    } else {
        r1 = if blend as c_uint == SP_MIX_BLEND_SETUP as c_int as c_uint {
            (*(*bone).data).rotation
        } else {
            (*bone).rotation
        };
        r2 = (*(*bone).data).rotation
            + spCurveTimeline1_getCurveValue(&mut (*rotateTimeline).super_0, time);
    }
    diff = r2 - r1;
    diff -= ((16384 as c_int
        - (16384.499999999996f64 - (diff / 360 as c_int as c_float) as c_double) as c_int)
        * 360 as c_int) as c_float;
    if diff == 0 as c_int as c_float {
        total = *timelinesRotation.offset(i as isize);
    } else {
        let mut lastTotal: c_float = 0.;
        let mut lastDiff: c_float = 0.;
        if firstFrame != 0 {
            lastTotal = 0 as c_int as c_float;
            lastDiff = diff;
        } else {
            lastTotal = *timelinesRotation.offset(i as isize);
            lastDiff = *timelinesRotation.offset((i + 1 as c_int) as isize);
        }
        current = (diff > 0 as c_int as c_float) as c_int;
        dir = (lastTotal >= 0 as c_int as c_float) as c_int;
        if (if lastDiff < 0 as c_int as c_float {
            -1.0f32
        } else {
            if lastDiff > 0 as c_int as c_float {
                1.0f32
            } else {
                0.0f32
            }
        }) != (if diff < 0 as c_int as c_float {
            -1.0f32
        } else {
            if diff > 0 as c_int as c_float {
                1.0f32
            } else {
                0.0f32
            }
        }) && (if lastDiff < 0 as c_int as c_float {
            -lastDiff
        } else {
            lastDiff
        }) <= 90 as c_int as c_float
        {
            if (if lastTotal < 0 as c_int as c_float {
                -lastTotal
            } else {
                lastTotal
            }) > 180 as c_int as c_float
            {
                lastTotal += 360 as c_int as c_float
                    * (if lastTotal < 0 as c_int as c_float {
                        -1.0f32
                    } else {
                        if lastTotal > 0 as c_int as c_float {
                            1.0f32
                        } else {
                            0.0f32
                        }
                    });
            }
            dir = current;
        }
        total = diff + lastTotal - fmodf(lastTotal, 360 as c_int as c_float);
        if dir != current {
            total += 360 as c_int as c_float
                * (if lastTotal < 0 as c_int as c_float {
                    -1.0f32
                } else {
                    if lastTotal > 0 as c_int as c_float {
                        1.0f32
                    } else {
                        0.0f32
                    }
                });
        }
        *timelinesRotation.offset(i as isize) = total;
    }
    *timelinesRotation.offset((i + 1 as c_int) as isize) = diff;
    (*bone).rotation = r1 + total * alpha;
}
#[no_mangle]
pub unsafe extern "C" fn _spAnimationState_queueEvents(
    mut self_0: *mut spAnimationState,
    mut entry: *mut spTrackEntry,
    mut animationTime: c_float,
) {
    let mut events: *mut *mut spEvent = 0 as *mut *mut spEvent;
    let mut event: *mut spEvent = 0 as *mut spEvent;
    let mut internal: *mut _spAnimationState = self_0 as *mut _spAnimationState;
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    let mut complete: c_int = 0;
    let mut animationStart: c_float = (*entry).animationStart;
    let mut animationEnd: c_float = (*entry).animationEnd;
    let mut duration: c_float = animationEnd - animationStart;
    let mut trackLastWrapped: c_float = fmodf((*entry).trackLast, duration);
    events = (*internal).events;
    i = 0 as c_int;
    n = (*internal).eventsCount;
    while i < n {
        event = *events.offset(i as isize);
        if (*event).time < trackLastWrapped {
            break;
        }
        if !((*event).time > animationEnd) {
            _spEventQueue_event((*internal).queue, entry, event);
        }
        i += 1;
    }
    if (*entry).loop_0 != 0 {
        complete = (duration == 0 as c_int as c_float
            || trackLastWrapped > fmodf((*entry).trackTime, duration)) as c_int;
    } else {
        complete =
            (animationTime >= animationEnd && (*entry).animationLast < animationEnd) as c_int;
    }
    if complete != 0 {
        _spEventQueue_complete((*internal).queue, entry);
    }
    while i < n {
        event = *events.offset(i as isize);
        if !((*event).time < animationStart) {
            _spEventQueue_event((*internal).queue, entry, event);
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spAnimationState_clearTracks(mut self_0: *mut spAnimationState) {
    let mut internal: *mut _spAnimationState = self_0 as *mut _spAnimationState;
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    let mut oldDrainDisabled: c_int = 0;
    oldDrainDisabled = (*(*internal).queue).drainDisabled;
    (*(*internal).queue).drainDisabled = 1 as c_int;
    i = 0 as c_int;
    n = (*self_0).tracksCount;
    while i < n {
        spAnimationState_clearTrack(self_0, i);
        i += 1;
    }
    (*self_0).tracksCount = 0 as c_int;
    (*(*internal).queue).drainDisabled = oldDrainDisabled;
    _spEventQueue_drain((*internal).queue);
}
#[no_mangle]
pub unsafe extern "C" fn spAnimationState_clearTrack(
    mut self_0: *mut spAnimationState,
    mut trackIndex: c_int,
) {
    let mut current: *mut spTrackEntry = 0 as *mut spTrackEntry;
    let mut entry: *mut spTrackEntry = 0 as *mut spTrackEntry;
    let mut from: *mut spTrackEntry = 0 as *mut spTrackEntry;
    let mut internal: *mut _spAnimationState = self_0 as *mut _spAnimationState;
    if trackIndex >= (*self_0).tracksCount {
        return;
    }
    current = *((*self_0).tracks).offset(trackIndex as isize);
    if current.is_null() {
        return;
    }
    _spEventQueue_end((*internal).queue, current);
    spAnimationState_clearNext(self_0, current);
    entry = current;
    loop {
        from = (*entry).mixingFrom;
        if from.is_null() {
            break;
        }
        _spEventQueue_end((*internal).queue, from);
        (*entry).mixingFrom = 0 as *mut spTrackEntry;
        (*entry).mixingTo = 0 as *mut spTrackEntry;
        entry = from;
    }
    let ref mut fresh26 = *((*self_0).tracks).offset((*current).trackIndex as isize);
    *fresh26 = 0 as *mut spTrackEntry;
    _spEventQueue_drain((*internal).queue);
}
#[no_mangle]
pub unsafe extern "C" fn _spAnimationState_setCurrent(
    mut self_0: *mut spAnimationState,
    mut index: c_int,
    mut current: *mut spTrackEntry,
    mut interrupt: c_int,
) {
    let mut internal: *mut _spAnimationState = self_0 as *mut _spAnimationState;
    let mut from: *mut spTrackEntry = _spAnimationState_expandToIndex(self_0, index);
    let ref mut fresh27 = *((*self_0).tracks).offset(index as isize);
    *fresh27 = current;
    (*current).previous = 0 as *mut spTrackEntry;
    if !from.is_null() {
        if interrupt != 0 {
            _spEventQueue_interrupt((*internal).queue, from);
        }
        (*current).mixingFrom = from;
        (*from).mixingTo = current;
        (*current).mixTime = 0 as c_int as c_float;
        if !((*from).mixingFrom).is_null() && (*from).mixDuration > 0 as c_int as c_float {
            (*current).interruptAlpha *=
                if (1 as c_int as c_float) < (*from).mixTime / (*from).mixDuration {
                    1 as c_int as c_float
                } else {
                    (*from).mixTime / (*from).mixDuration
                };
        }
        (*from).timelinesRotationCount = 0 as c_int;
    }
    _spEventQueue_start((*internal).queue, current);
}
#[no_mangle]
pub unsafe extern "C" fn spAnimationState_setAnimationByName(
    mut self_0: *mut spAnimationState,
    mut trackIndex: c_int,
    mut animationName: *const c_char,
    mut loop_0: c_int,
) -> *mut spTrackEntry {
    let mut animation: *mut spAnimation =
        spSkeletonData_findAnimation((*(*self_0).data).skeletonData, animationName);
    return spAnimationState_setAnimation(self_0, trackIndex, animation, loop_0);
}
#[no_mangle]
pub unsafe extern "C" fn spAnimationState_setAnimation(
    mut self_0: *mut spAnimationState,
    mut trackIndex: c_int,
    mut animation: *mut spAnimation,
    mut loop_0: c_int,
) -> *mut spTrackEntry {
    let mut entry: *mut spTrackEntry = 0 as *mut spTrackEntry;
    let mut internal: *mut _spAnimationState = self_0 as *mut _spAnimationState;
    let mut interrupt: c_int = 1 as c_int;
    let mut current: *mut spTrackEntry = _spAnimationState_expandToIndex(self_0, trackIndex);
    if !current.is_null() {
        if (*current).nextTrackLast == -(1 as c_int) as c_float {
            let ref mut fresh28 = *((*self_0).tracks).offset(trackIndex as isize);
            *fresh28 = (*current).mixingFrom;
            _spEventQueue_interrupt((*internal).queue, current);
            _spEventQueue_end((*internal).queue, current);
            spAnimationState_clearNext(self_0, current);
            current = (*current).mixingFrom;
            interrupt = 0 as c_int;
        } else {
            spAnimationState_clearNext(self_0, current);
        }
    }
    entry = _spAnimationState_trackEntry(self_0, trackIndex, animation, loop_0, current);
    _spAnimationState_setCurrent(self_0, trackIndex, entry, interrupt);
    _spEventQueue_drain((*internal).queue);
    return entry;
}
#[no_mangle]
pub unsafe extern "C" fn spAnimationState_addAnimationByName(
    mut self_0: *mut spAnimationState,
    mut trackIndex: c_int,
    mut animationName: *const c_char,
    mut loop_0: c_int,
    mut delay: c_float,
) -> *mut spTrackEntry {
    let mut animation: *mut spAnimation =
        spSkeletonData_findAnimation((*(*self_0).data).skeletonData, animationName);
    return spAnimationState_addAnimation(self_0, trackIndex, animation, loop_0, delay);
}
#[no_mangle]
pub unsafe extern "C" fn spAnimationState_addAnimation(
    mut self_0: *mut spAnimationState,
    mut trackIndex: c_int,
    mut animation: *mut spAnimation,
    mut loop_0: c_int,
    mut delay: c_float,
) -> *mut spTrackEntry {
    let mut entry: *mut spTrackEntry = 0 as *mut spTrackEntry;
    let mut internal: *mut _spAnimationState = self_0 as *mut _spAnimationState;
    let mut last: *mut spTrackEntry = _spAnimationState_expandToIndex(self_0, trackIndex);
    if !last.is_null() {
        while !((*last).next).is_null() {
            last = (*last).next;
        }
    }
    entry = _spAnimationState_trackEntry(self_0, trackIndex, animation, loop_0, last);
    if last.is_null() {
        _spAnimationState_setCurrent(self_0, trackIndex, entry, 1 as c_int);
        _spEventQueue_drain((*internal).queue);
    } else {
        (*last).next = entry;
        (*entry).previous = last;
        if delay <= 0 as c_int as c_float {
            delay += spTrackEntry_getTrackComplete(last) - (*entry).mixDuration;
        }
    }
    (*entry).delay = delay;
    return entry;
}
#[no_mangle]
pub unsafe extern "C" fn spAnimationState_setEmptyAnimation(
    mut self_0: *mut spAnimationState,
    mut trackIndex: c_int,
    mut mixDuration: c_float,
) -> *mut spTrackEntry {
    let mut entry: *mut spTrackEntry =
        spAnimationState_setAnimation(self_0, trackIndex, SP_EMPTY_ANIMATION, 0 as c_int);
    (*entry).mixDuration = mixDuration;
    (*entry).trackEnd = mixDuration;
    return entry;
}
#[no_mangle]
pub unsafe extern "C" fn spAnimationState_addEmptyAnimation(
    mut self_0: *mut spAnimationState,
    mut trackIndex: c_int,
    mut mixDuration: c_float,
    mut delay: c_float,
) -> *mut spTrackEntry {
    let mut entry: *mut spTrackEntry =
        spAnimationState_addAnimation(self_0, trackIndex, SP_EMPTY_ANIMATION, 0 as c_int, delay);
    if delay <= 0 as c_int as c_float {
        (*entry).delay += (*entry).mixDuration - mixDuration;
    }
    (*entry).mixDuration = mixDuration;
    (*entry).trackEnd = mixDuration;
    return entry;
}
#[no_mangle]
pub unsafe extern "C" fn spAnimationState_setEmptyAnimations(
    mut self_0: *mut spAnimationState,
    mut mixDuration: c_float,
) {
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    let mut oldDrainDisabled: c_int = 0;
    let mut current: *mut spTrackEntry = 0 as *mut spTrackEntry;
    let mut internal: *mut _spAnimationState = self_0 as *mut _spAnimationState;
    oldDrainDisabled = (*(*internal).queue).drainDisabled;
    (*(*internal).queue).drainDisabled = 1 as c_int;
    i = 0 as c_int;
    n = (*self_0).tracksCount;
    while i < n {
        current = *((*self_0).tracks).offset(i as isize);
        if !current.is_null() {
            spAnimationState_setEmptyAnimation(self_0, (*current).trackIndex, mixDuration);
        }
        i += 1;
    }
    (*(*internal).queue).drainDisabled = oldDrainDisabled;
    _spEventQueue_drain((*internal).queue);
}
#[no_mangle]
pub unsafe extern "C" fn _spAnimationState_expandToIndex(
    mut self_0: *mut spAnimationState,
    mut index: c_int,
) -> *mut spTrackEntry {
    let mut newTracks: *mut *mut spTrackEntry = 0 as *mut *mut spTrackEntry;
    if index < (*self_0).tracksCount {
        return *((*self_0).tracks).offset(index as isize);
    }
    newTracks = _spCalloc(
        (index + 1 as c_int) as size_t,
        ::core::mem::size_of::<*mut spTrackEntry>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        3634 as c_int,
    ) as *mut *mut spTrackEntry;
    spine_memcpy(
        newTracks as *mut c_void,
        (*self_0).tracks as *const c_void,
        ((*self_0).tracksCount as c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut spTrackEntry>() as c_ulong),
    );
    _spFree((*self_0).tracks as *mut c_void);
    (*self_0).tracks = newTracks;
    (*self_0).tracksCount = index + 1 as c_int;
    return 0 as *mut spTrackEntry;
}
#[no_mangle]
pub unsafe extern "C" fn _spAnimationState_trackEntry(
    mut self_0: *mut spAnimationState,
    mut trackIndex: c_int,
    mut animation: *mut spAnimation,
    mut loop_0: c_int,
    mut last: *mut spTrackEntry,
) -> *mut spTrackEntry {
    let mut entry: *mut spTrackEntry = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spTrackEntry>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        3645 as c_int,
    ) as *mut spTrackEntry;
    (*entry).trackIndex = trackIndex;
    (*entry).animation = animation;
    (*entry).loop_0 = loop_0;
    (*entry).holdPrevious = 0 as c_int;
    (*entry).reverse = 0 as c_int;
    (*entry).shortestRotation = 0 as c_int;
    (*entry).previous = 0 as *mut spTrackEntry;
    (*entry).next = 0 as *mut spTrackEntry;
    (*entry).eventThreshold = 0 as c_int as c_float;
    (*entry).attachmentThreshold = 0 as c_int as c_float;
    (*entry).drawOrderThreshold = 0 as c_int as c_float;
    (*entry).animationStart = 0 as c_int as c_float;
    (*entry).animationEnd = (*animation).duration;
    (*entry).animationLast = -(1 as c_int) as c_float;
    (*entry).nextAnimationLast = -(1 as c_int) as c_float;
    (*entry).delay = 0 as c_int as c_float;
    (*entry).trackTime = 0 as c_int as c_float;
    (*entry).trackLast = -(1 as c_int) as c_float;
    (*entry).nextTrackLast = -(1 as c_int) as c_float;
    (*entry).trackEnd = 0x7fffffff as c_int as c_float;
    (*entry).timeScale = 1 as c_int as c_float;
    (*entry).alpha = 1 as c_int as c_float;
    (*entry).mixTime = 0 as c_int as c_float;
    (*entry).mixDuration = if last.is_null() {
        0 as c_int as c_float
    } else {
        spAnimationStateData_getMix((*self_0).data, (*last).animation, animation)
    };
    (*entry).interruptAlpha = 1 as c_int as c_float;
    (*entry).totalAlpha = 0 as c_int as c_float;
    (*entry).mixBlend = SP_MIX_BLEND_REPLACE;
    (*entry).timelineMode = spIntArray_create(16 as c_int);
    (*entry).timelineHoldMix = spTrackEntryArray_create(16 as c_int);
    return entry;
}
#[no_mangle]
pub unsafe extern "C" fn spAnimationState_clearNext(
    mut self_0: *mut spAnimationState,
    mut entry: *mut spTrackEntry,
) {
    let mut internal: *mut _spAnimationState = self_0 as *mut _spAnimationState;
    let mut next: *mut spTrackEntry = (*entry).next;
    while !next.is_null() {
        _spEventQueue_dispose((*internal).queue, next);
        next = (*next).next;
    }
    (*entry).next = 0 as *mut spTrackEntry;
}
#[no_mangle]
pub unsafe extern "C" fn _spAnimationState_animationsChanged(mut self_0: *mut spAnimationState) {
    let mut internal: *mut _spAnimationState = self_0 as *mut _spAnimationState;
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    let mut entry: *mut spTrackEntry = 0 as *mut spTrackEntry;
    (*internal).animationsChanged = 0 as c_int;
    (*internal).propertyIDsCount = 0 as c_int;
    i = 0 as c_int;
    n = (*self_0).tracksCount;
    while i < n {
        entry = *((*self_0).tracks).offset(i as isize);
        if !entry.is_null() {
            while !((*entry).mixingFrom).is_null() {
                entry = (*entry).mixingFrom;
            }
            loop {
                if ((*entry).mixingTo).is_null()
                    || (*entry).mixBlend as c_uint != SP_MIX_BLEND_ADD as c_int as c_uint
                {
                    _spTrackEntry_computeHold(entry, self_0);
                }
                entry = (*entry).mixingTo;
                if entry.is_null() {
                    break;
                }
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _spAnimationState_resizeTimelinesRotation(
    mut entry: *mut spTrackEntry,
    mut newSize: c_int,
) -> *mut c_float {
    if (*entry).timelinesRotationCount != newSize {
        let mut newTimelinesRotation: *mut c_float = _spCalloc(
            newSize as size_t,
            ::core::mem::size_of::<c_float>() as c_ulong,
            b"spine.c\0" as *const u8 as *const c_char,
            3718 as c_int,
        ) as *mut c_float;
        _spFree((*entry).timelinesRotation as *mut c_void);
        (*entry).timelinesRotation = newTimelinesRotation;
        (*entry).timelinesRotationCount = newSize;
    }
    return (*entry).timelinesRotation;
}
#[no_mangle]
pub unsafe extern "C" fn _spAnimationState_ensureCapacityPropertyIDs(
    mut self_0: *mut spAnimationState,
    mut capacity: c_int,
) {
    let mut internal: *mut _spAnimationState = self_0 as *mut _spAnimationState;
    if (*internal).propertyIDsCapacity < capacity {
        let mut newPropertyIDs: *mut spPropertyId = _spCalloc(
            (capacity << 1 as c_int) as size_t,
            ::core::mem::size_of::<spPropertyId>() as c_ulong,
            b"spine.c\0" as *const u8 as *const c_char,
            3729 as c_int,
        ) as *mut spPropertyId;
        spine_memcpy(
            newPropertyIDs as *mut c_void,
            (*internal).propertyIDs as *const c_void,
            (::core::mem::size_of::<spPropertyId>() as c_ulong)
                .wrapping_mul((*internal).propertyIDsCount as c_ulong),
        );
        _spFree((*internal).propertyIDs as *mut c_void);
        (*internal).propertyIDs = newPropertyIDs;
        (*internal).propertyIDsCapacity = capacity << 1 as c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _spAnimationState_addPropertyID(
    mut self_0: *mut spAnimationState,
    mut id: spPropertyId,
) -> c_int {
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    let mut internal: *mut _spAnimationState = self_0 as *mut _spAnimationState;
    i = 0 as c_int;
    n = (*internal).propertyIDsCount;
    while i < n {
        if *((*internal).propertyIDs).offset(i as isize) == id {
            return 0 as c_int;
        }
        i += 1;
    }
    _spAnimationState_ensureCapacityPropertyIDs(self_0, (*internal).propertyIDsCount + 1 as c_int);
    *((*internal).propertyIDs).offset((*internal).propertyIDsCount as isize) = id;
    (*internal).propertyIDsCount += 1;
    return 1 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _spAnimationState_addPropertyIDs(
    mut self_0: *mut spAnimationState,
    mut ids: *mut spPropertyId,
    mut numIds: c_int,
) -> c_int {
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    let mut internal: *mut _spAnimationState = self_0 as *mut _spAnimationState;
    let mut oldSize: c_int = (*internal).propertyIDsCount;
    i = 0 as c_int;
    n = numIds;
    while i < n {
        _spAnimationState_addPropertyID(self_0, *ids.offset(i as isize));
        i += 1;
    }
    return ((*internal).propertyIDsCount != oldSize) as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spAnimationState_getCurrent(
    mut self_0: *mut spAnimationState,
    mut trackIndex: c_int,
) -> *mut spTrackEntry {
    if trackIndex >= (*self_0).tracksCount {
        return 0 as *mut spTrackEntry;
    }
    return *((*self_0).tracks).offset(trackIndex as isize);
}
#[no_mangle]
pub unsafe extern "C" fn spAnimationState_clearListenerNotifications(
    mut self_0: *mut spAnimationState,
) {
    let mut internal: *mut _spAnimationState = self_0 as *mut _spAnimationState;
    _spEventQueue_clear((*internal).queue);
}
#[no_mangle]
pub unsafe extern "C" fn spTrackEntry_getAnimationTime(mut entry: *mut spTrackEntry) -> c_float {
    if (*entry).loop_0 != 0 {
        let mut duration: c_float = (*entry).animationEnd - (*entry).animationStart;
        if duration == 0 as c_int as c_float {
            return (*entry).animationStart;
        }
        return fmodf((*entry).trackTime, duration) + (*entry).animationStart;
    }
    return if (*entry).trackTime + (*entry).animationStart < (*entry).animationEnd {
        (*entry).trackTime + (*entry).animationStart
    } else {
        (*entry).animationEnd
    };
}
#[no_mangle]
pub unsafe extern "C" fn spTrackEntry_getTrackComplete(mut entry: *mut spTrackEntry) -> c_float {
    let mut duration: c_float = (*entry).animationEnd - (*entry).animationStart;
    if duration != 0 as c_int as c_float {
        if (*entry).loop_0 != 0 {
            return duration * (1 as c_int + ((*entry).trackTime / duration) as c_int) as c_float;
        }
        if (*entry).trackTime < duration {
            return duration;
        }
    }
    return (*entry).trackTime;
}
#[no_mangle]
pub unsafe extern "C" fn _spTrackEntry_computeHold(
    mut entry: *mut spTrackEntry,
    mut state: *mut spAnimationState,
) {
    let mut to: *mut spTrackEntry = 0 as *mut spTrackEntry;
    let mut timelines: *mut *mut spTimeline = 0 as *mut *mut spTimeline;
    let mut timelinesCount: c_int = 0;
    let mut timelineMode: *mut c_int = 0 as *mut c_int;
    let mut timelineHoldMix: *mut *mut spTrackEntry = 0 as *mut *mut spTrackEntry;
    let mut next: *mut spTrackEntry = 0 as *mut spTrackEntry;
    let mut i: c_int = 0;
    to = (*entry).mixingTo;
    timelines = (*(*(*entry).animation).timelines).items;
    timelinesCount = (*(*(*entry).animation).timelines).size;
    timelineMode = (*spIntArray_setSize((*entry).timelineMode, timelinesCount)).items;
    spTrackEntryArray_clear((*entry).timelineHoldMix);
    timelineHoldMix = (*spTrackEntryArray_setSize((*entry).timelineHoldMix, timelinesCount)).items;
    if !to.is_null() && (*to).holdPrevious != 0 {
        i = 0 as c_int;
        while i < timelinesCount {
            let mut ids: *mut spPropertyId =
                ((**timelines.offset(i as isize)).propertyIds).as_mut_ptr();
            let mut numIds: c_int = (**timelines.offset(i as isize)).propertyIdsCount;
            *timelineMode.offset(i as isize) =
                if _spAnimationState_addPropertyIDs(state, ids, numIds) != 0 {
                    3 as c_int
                } else {
                    2 as c_int
                };
            i += 1;
        }
        return;
    }
    i = 0 as c_int;
    's_69: while i < timelinesCount {
        let mut timeline: *mut spTimeline = *timelines.offset(i as isize);
        let mut ids_0: *mut spPropertyId = ((*timeline).propertyIds).as_mut_ptr();
        let mut numIds_0: c_int = (*timeline).propertyIdsCount;
        if _spAnimationState_addPropertyIDs(state, ids_0, numIds_0) == 0 {
            *timelineMode.offset(i as isize) = 0 as c_int;
        } else if to.is_null()
            || (*timeline).propertyIds[0 as c_int as usize]
                == SP_PROPERTY_ATTACHMENT as c_int as c_ulong
            || (*timeline).propertyIds[0 as c_int as usize]
                == SP_PROPERTY_DRAWORDER as c_int as c_ulong
            || (*timeline).propertyIds[0 as c_int as usize] == SP_PROPERTY_EVENT as c_int as c_ulong
            || spAnimation_hasTimeline((*to).animation, ids_0, numIds_0) == 0
        {
            *timelineMode.offset(i as isize) = 1 as c_int;
        } else {
            next = (*to).mixingTo;
            while !next.is_null() {
                if spAnimation_hasTimeline((*next).animation, ids_0, numIds_0) != 0 {
                    next = (*next).mixingTo;
                } else {
                    if !((*next).mixDuration > 0 as c_int as c_float) {
                        break;
                    }
                    *timelineMode.offset(i as isize) = 4 as c_int;
                    let ref mut fresh29 = *timelineHoldMix.offset(i as isize);
                    *fresh29 = next;
                    i += 1;
                    continue 's_69;
                }
            }
            *timelineMode.offset(i as isize) = 3 as c_int;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _ToEntry_create(
    mut to: *mut spAnimation,
    mut duration: c_float,
) -> *mut _ToEntry {
    let mut self_0: *mut _ToEntry = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<_ToEntry>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        3884 as c_int,
    ) as *mut _ToEntry;
    (*self_0).animation = to;
    (*self_0).duration = duration;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn _ToEntry_dispose(mut self_0: *mut _ToEntry) {
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _FromEntry_create(mut from: *mut spAnimation) -> *mut _FromEntry {
    let mut self_0: *mut _FromEntry = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<_FromEntry>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        3904 as c_int,
    ) as *mut _FromEntry;
    (*self_0).animation = from;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn _FromEntry_dispose(mut self_0: *mut _FromEntry) {
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spAnimationStateData_create(
    mut skeletonData: *mut spSkeletonData,
) -> *mut spAnimationStateData {
    let mut self_0: *mut spAnimationStateData = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spAnimationStateData>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        3916 as c_int,
    ) as *mut spAnimationStateData;
    let ref mut fresh30 =
        *(&(*self_0).skeletonData as *const *mut spSkeletonData as *mut *mut spSkeletonData);
    *fresh30 = skeletonData;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spAnimationStateData_dispose(mut self_0: *mut spAnimationStateData) {
    let mut toEntry: *mut _ToEntry = 0 as *mut _ToEntry;
    let mut nextToEntry: *mut _ToEntry = 0 as *mut _ToEntry;
    let mut nextFromEntry: *mut _FromEntry = 0 as *mut _FromEntry;
    let mut fromEntry: *mut _FromEntry = (*self_0).entries as *mut _FromEntry;
    while !fromEntry.is_null() {
        toEntry = (*fromEntry).toEntries;
        while !toEntry.is_null() {
            nextToEntry = (*toEntry).next;
            _ToEntry_dispose(toEntry);
            toEntry = nextToEntry;
        }
        nextFromEntry = (*fromEntry).next;
        _FromEntry_dispose(fromEntry);
        fromEntry = nextFromEntry;
    }
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spAnimationStateData_setMixByName(
    mut self_0: *mut spAnimationStateData,
    mut fromName: *const c_char,
    mut toName: *const c_char,
    mut duration: c_float,
) {
    let mut to: *mut spAnimation = 0 as *mut spAnimation;
    let mut from: *mut spAnimation = spSkeletonData_findAnimation((*self_0).skeletonData, fromName);
    if from.is_null() {
        return;
    }
    to = spSkeletonData_findAnimation((*self_0).skeletonData, toName);
    if to.is_null() {
        return;
    }
    spAnimationStateData_setMix(self_0, from, to, duration);
}
#[no_mangle]
pub unsafe extern "C" fn spAnimationStateData_setMix(
    mut self_0: *mut spAnimationStateData,
    mut from: *mut spAnimation,
    mut to: *mut spAnimation,
    mut duration: c_float,
) {
    let mut toEntry: *mut _ToEntry = 0 as *mut _ToEntry;
    let mut fromEntry: *mut _FromEntry = (*self_0).entries as *mut _FromEntry;
    while !fromEntry.is_null() {
        if (*fromEntry).animation == from {
            toEntry = (*fromEntry).toEntries;
            while !toEntry.is_null() {
                if (*toEntry).animation == to {
                    (*toEntry).duration = duration;
                    return;
                }
                toEntry = (*toEntry).next;
            }
            break;
        } else {
            fromEntry = (*fromEntry).next;
        }
    }
    if fromEntry.is_null() {
        fromEntry = _FromEntry_create(from);
        (*fromEntry).next = (*self_0).entries as *mut _FromEntry;
        let ref mut fresh31 = *(&(*self_0).entries as *const *const c_void as *mut *mut _FromEntry);
        *fresh31 = fromEntry;
    }
    toEntry = _ToEntry_create(to, duration);
    (*toEntry).next = (*fromEntry).toEntries;
    (*fromEntry).toEntries = toEntry;
}
#[no_mangle]
pub unsafe extern "C" fn spAnimationStateData_getMix(
    mut self_0: *mut spAnimationStateData,
    mut from: *mut spAnimation,
    mut to: *mut spAnimation,
) -> c_float {
    let mut fromEntry: *mut _FromEntry = (*self_0).entries as *mut _FromEntry;
    while !fromEntry.is_null() {
        if (*fromEntry).animation == from {
            let mut toEntry: *mut _ToEntry = (*fromEntry).toEntries;
            while !toEntry.is_null() {
                if (*toEntry).animation == to {
                    return (*toEntry).duration;
                }
                toEntry = (*toEntry).next;
            }
        }
        fromEntry = (*fromEntry).next;
    }
    return (*self_0).defaultMix;
}
#[no_mangle]
pub unsafe extern "C" fn spFloatArray_create(mut initialCapacity: c_int) -> *mut spFloatArray {
    let mut array: *mut spFloatArray = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spFloatArray>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        4027 as c_int,
    ) as *mut spFloatArray;
    (*array).size = 0 as c_int;
    (*array).capacity = initialCapacity;
    (*array).items = _spCalloc(
        initialCapacity as size_t,
        ::core::mem::size_of::<c_float>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        4027 as c_int,
    ) as *mut c_float;
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn spFloatArray_dispose(mut self_0: *mut spFloatArray) {
    _spFree((*self_0).items as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spFloatArray_clear(mut self_0: *mut spFloatArray) {
    (*self_0).size = 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spFloatArray_setSize(
    mut self_0: *mut spFloatArray,
    mut newSize: c_int,
) -> *mut spFloatArray {
    (*self_0).size = newSize;
    if (*self_0).capacity < newSize {
        (*self_0).capacity = if 8 as c_int > ((*self_0).size as c_float * 1.75f32) as c_int {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0).items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<c_float>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut c_float;
    }
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spFloatArray_ensureCapacity(
    mut self_0: *mut spFloatArray,
    mut newCapacity: c_int,
) {
    if (*self_0).capacity >= newCapacity {
        return;
    }
    (*self_0).capacity = newCapacity;
    (*self_0).items = _spRealloc(
        (*self_0).items as *mut c_void,
        (::core::mem::size_of::<c_float>() as c_ulong).wrapping_mul((*self_0).capacity as c_ulong),
    ) as *mut c_float;
}
#[no_mangle]
pub unsafe extern "C" fn spFloatArray_add(mut self_0: *mut spFloatArray, mut value: c_float) {
    if (*self_0).size == (*self_0).capacity {
        (*self_0).capacity = if 8 as c_int > ((*self_0).size as c_float * 1.75f32) as c_int {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0).items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<c_float>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut c_float;
    }
    let fresh32 = (*self_0).size;
    (*self_0).size = (*self_0).size + 1;
    *((*self_0).items).offset(fresh32 as isize) = value;
}
#[no_mangle]
pub unsafe extern "C" fn spFloatArray_addAll(
    mut self_0: *mut spFloatArray,
    mut other: *mut spFloatArray,
) {
    let mut i: c_int = 0 as c_int;
    while i < (*other).size {
        spFloatArray_add(self_0, *((*other).items).offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spFloatArray_addAllValues(
    mut self_0: *mut spFloatArray,
    mut values: *mut c_float,
    mut offset: c_int,
    mut count: c_int,
) {
    let mut i: c_int = offset;
    let mut n: c_int = offset + count;
    while i < n {
        spFloatArray_add(self_0, *values.offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spFloatArray_removeAt(mut self_0: *mut spFloatArray, mut index: c_int) {
    (*self_0).size -= 1;
    spine_memmove(
        ((*self_0).items).offset(index as isize) as *mut c_void,
        ((*self_0).items)
            .offset(index as isize)
            .offset(1 as c_int as isize) as *const c_void,
        (::core::mem::size_of::<c_float>() as c_ulong)
            .wrapping_mul(((*self_0).size - index) as c_ulong),
    );
}
#[no_mangle]
pub unsafe extern "C" fn spFloatArray_contains(
    mut self_0: *mut spFloatArray,
    mut value: c_float,
) -> c_int {
    let mut items: *mut c_float = (*self_0).items;
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    i = 0 as c_int;
    n = (*self_0).size;
    while i < n {
        if *items.offset(i as isize) == value {
            return -(1 as c_int);
        }
        i += 1;
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spFloatArray_pop(mut self_0: *mut spFloatArray) -> c_float {
    (*self_0).size -= 1;
    let mut item: c_float = *((*self_0).items).offset((*self_0).size as isize);
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn spFloatArray_peek(mut self_0: *mut spFloatArray) -> c_float {
    return *((*self_0).items).offset(((*self_0).size - 1 as c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn spIntArray_create(mut initialCapacity: c_int) -> *mut spIntArray {
    let mut array: *mut spIntArray = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spIntArray>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        4029 as c_int,
    ) as *mut spIntArray;
    (*array).size = 0 as c_int;
    (*array).capacity = initialCapacity;
    (*array).items = _spCalloc(
        initialCapacity as size_t,
        ::core::mem::size_of::<c_int>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        4029 as c_int,
    ) as *mut c_int;
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn spIntArray_dispose(mut self_0: *mut spIntArray) {
    _spFree((*self_0).items as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spIntArray_clear(mut self_0: *mut spIntArray) {
    (*self_0).size = 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spIntArray_setSize(
    mut self_0: *mut spIntArray,
    mut newSize: c_int,
) -> *mut spIntArray {
    (*self_0).size = newSize;
    if (*self_0).capacity < newSize {
        (*self_0).capacity = if 8 as c_int > ((*self_0).size as c_float * 1.75f32) as c_int {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0).items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<c_int>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut c_int;
    }
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spIntArray_ensureCapacity(
    mut self_0: *mut spIntArray,
    mut newCapacity: c_int,
) {
    if (*self_0).capacity >= newCapacity {
        return;
    }
    (*self_0).capacity = newCapacity;
    (*self_0).items = _spRealloc(
        (*self_0).items as *mut c_void,
        (::core::mem::size_of::<c_int>() as c_ulong).wrapping_mul((*self_0).capacity as c_ulong),
    ) as *mut c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spIntArray_add(mut self_0: *mut spIntArray, mut value: c_int) {
    if (*self_0).size == (*self_0).capacity {
        (*self_0).capacity = if 8 as c_int > ((*self_0).size as c_float * 1.75f32) as c_int {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0).items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<c_int>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut c_int;
    }
    let fresh33 = (*self_0).size;
    (*self_0).size = (*self_0).size + 1;
    *((*self_0).items).offset(fresh33 as isize) = value;
}
#[no_mangle]
pub unsafe extern "C" fn spIntArray_addAll(
    mut self_0: *mut spIntArray,
    mut other: *mut spIntArray,
) {
    let mut i: c_int = 0 as c_int;
    while i < (*other).size {
        spIntArray_add(self_0, *((*other).items).offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spIntArray_addAllValues(
    mut self_0: *mut spIntArray,
    mut values: *mut c_int,
    mut offset: c_int,
    mut count: c_int,
) {
    let mut i: c_int = offset;
    let mut n: c_int = offset + count;
    while i < n {
        spIntArray_add(self_0, *values.offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spIntArray_removeAt(mut self_0: *mut spIntArray, mut index: c_int) {
    (*self_0).size -= 1;
    spine_memmove(
        ((*self_0).items).offset(index as isize) as *mut c_void,
        ((*self_0).items)
            .offset(index as isize)
            .offset(1 as c_int as isize) as *const c_void,
        (::core::mem::size_of::<c_int>() as c_ulong)
            .wrapping_mul(((*self_0).size - index) as c_ulong),
    );
}
#[no_mangle]
pub unsafe extern "C" fn spIntArray_contains(
    mut self_0: *mut spIntArray,
    mut value: c_int,
) -> c_int {
    let mut items: *mut c_int = (*self_0).items;
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    i = 0 as c_int;
    n = (*self_0).size;
    while i < n {
        if *items.offset(i as isize) == value {
            return -(1 as c_int);
        }
        i += 1;
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spIntArray_pop(mut self_0: *mut spIntArray) -> c_int {
    (*self_0).size -= 1;
    let mut item: c_int = *((*self_0).items).offset((*self_0).size as isize);
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn spIntArray_peek(mut self_0: *mut spIntArray) -> c_int {
    return *((*self_0).items).offset(((*self_0).size - 1 as c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn spShortArray_create(mut initialCapacity: c_int) -> *mut spShortArray {
    let mut array: *mut spShortArray = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spShortArray>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        4031 as c_int,
    ) as *mut spShortArray;
    (*array).size = 0 as c_int;
    (*array).capacity = initialCapacity;
    (*array).items = _spCalloc(
        initialCapacity as size_t,
        ::core::mem::size_of::<c_short>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        4031 as c_int,
    ) as *mut c_short;
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn spShortArray_dispose(mut self_0: *mut spShortArray) {
    _spFree((*self_0).items as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spShortArray_clear(mut self_0: *mut spShortArray) {
    (*self_0).size = 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spShortArray_setSize(
    mut self_0: *mut spShortArray,
    mut newSize: c_int,
) -> *mut spShortArray {
    (*self_0).size = newSize;
    if (*self_0).capacity < newSize {
        (*self_0).capacity = if 8 as c_int > ((*self_0).size as c_float * 1.75f32) as c_int {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0).items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<c_short>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut c_short;
    }
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spShortArray_ensureCapacity(
    mut self_0: *mut spShortArray,
    mut newCapacity: c_int,
) {
    if (*self_0).capacity >= newCapacity {
        return;
    }
    (*self_0).capacity = newCapacity;
    (*self_0).items = _spRealloc(
        (*self_0).items as *mut c_void,
        (::core::mem::size_of::<c_short>() as c_ulong).wrapping_mul((*self_0).capacity as c_ulong),
    ) as *mut c_short;
}
#[no_mangle]
pub unsafe extern "C" fn spShortArray_add(mut self_0: *mut spShortArray, mut value: c_short) {
    if (*self_0).size == (*self_0).capacity {
        (*self_0).capacity = if 8 as c_int > ((*self_0).size as c_float * 1.75f32) as c_int {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0).items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<c_short>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut c_short;
    }
    let fresh34 = (*self_0).size;
    (*self_0).size = (*self_0).size + 1;
    *((*self_0).items).offset(fresh34 as isize) = value;
}
#[no_mangle]
pub unsafe extern "C" fn spShortArray_addAll(
    mut self_0: *mut spShortArray,
    mut other: *mut spShortArray,
) {
    let mut i: c_int = 0 as c_int;
    while i < (*other).size {
        spShortArray_add(self_0, *((*other).items).offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spShortArray_addAllValues(
    mut self_0: *mut spShortArray,
    mut values: *mut c_short,
    mut offset: c_int,
    mut count: c_int,
) {
    let mut i: c_int = offset;
    let mut n: c_int = offset + count;
    while i < n {
        spShortArray_add(self_0, *values.offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spShortArray_removeAt(mut self_0: *mut spShortArray, mut index: c_int) {
    (*self_0).size -= 1;
    spine_memmove(
        ((*self_0).items).offset(index as isize) as *mut c_void,
        ((*self_0).items)
            .offset(index as isize)
            .offset(1 as c_int as isize) as *const c_void,
        (::core::mem::size_of::<c_short>() as c_ulong)
            .wrapping_mul(((*self_0).size - index) as c_ulong),
    );
}
#[no_mangle]
pub unsafe extern "C" fn spShortArray_contains(
    mut self_0: *mut spShortArray,
    mut value: c_short,
) -> c_int {
    let mut items: *mut c_short = (*self_0).items;
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    i = 0 as c_int;
    n = (*self_0).size;
    while i < n {
        if *items.offset(i as isize) as c_int == value as c_int {
            return -(1 as c_int);
        }
        i += 1;
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spShortArray_pop(mut self_0: *mut spShortArray) -> c_short {
    (*self_0).size -= 1;
    let mut item: c_short = *((*self_0).items).offset((*self_0).size as isize);
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn spShortArray_peek(mut self_0: *mut spShortArray) -> c_short {
    return *((*self_0).items).offset(((*self_0).size - 1 as c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn spUnsignedShortArray_create(
    mut initialCapacity: c_int,
) -> *mut spUnsignedShortArray {
    let mut array: *mut spUnsignedShortArray = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spUnsignedShortArray>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        4033 as c_int,
    ) as *mut spUnsignedShortArray;
    (*array).size = 0 as c_int;
    (*array).capacity = initialCapacity;
    (*array).items = _spCalloc(
        initialCapacity as size_t,
        ::core::mem::size_of::<c_ushort>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        4033 as c_int,
    ) as *mut c_ushort;
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn spUnsignedShortArray_dispose(mut self_0: *mut spUnsignedShortArray) {
    _spFree((*self_0).items as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spUnsignedShortArray_clear(mut self_0: *mut spUnsignedShortArray) {
    (*self_0).size = 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spUnsignedShortArray_setSize(
    mut self_0: *mut spUnsignedShortArray,
    mut newSize: c_int,
) -> *mut spUnsignedShortArray {
    (*self_0).size = newSize;
    if (*self_0).capacity < newSize {
        (*self_0).capacity = if 8 as c_int > ((*self_0).size as c_float * 1.75f32) as c_int {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0).items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<c_ushort>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut c_ushort;
    }
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spUnsignedShortArray_ensureCapacity(
    mut self_0: *mut spUnsignedShortArray,
    mut newCapacity: c_int,
) {
    if (*self_0).capacity >= newCapacity {
        return;
    }
    (*self_0).capacity = newCapacity;
    (*self_0).items = _spRealloc(
        (*self_0).items as *mut c_void,
        (::core::mem::size_of::<c_ushort>() as c_ulong).wrapping_mul((*self_0).capacity as c_ulong),
    ) as *mut c_ushort;
}
#[no_mangle]
pub unsafe extern "C" fn spUnsignedShortArray_add(
    mut self_0: *mut spUnsignedShortArray,
    mut value: c_ushort,
) {
    if (*self_0).size == (*self_0).capacity {
        (*self_0).capacity = if 8 as c_int > ((*self_0).size as c_float * 1.75f32) as c_int {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0).items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<c_ushort>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut c_ushort;
    }
    let fresh35 = (*self_0).size;
    (*self_0).size = (*self_0).size + 1;
    *((*self_0).items).offset(fresh35 as isize) = value;
}
#[no_mangle]
pub unsafe extern "C" fn spUnsignedShortArray_addAll(
    mut self_0: *mut spUnsignedShortArray,
    mut other: *mut spUnsignedShortArray,
) {
    let mut i: c_int = 0 as c_int;
    while i < (*other).size {
        spUnsignedShortArray_add(self_0, *((*other).items).offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spUnsignedShortArray_addAllValues(
    mut self_0: *mut spUnsignedShortArray,
    mut values: *mut c_ushort,
    mut offset: c_int,
    mut count: c_int,
) {
    let mut i: c_int = offset;
    let mut n: c_int = offset + count;
    while i < n {
        spUnsignedShortArray_add(self_0, *values.offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spUnsignedShortArray_removeAt(
    mut self_0: *mut spUnsignedShortArray,
    mut index: c_int,
) {
    (*self_0).size -= 1;
    spine_memmove(
        ((*self_0).items).offset(index as isize) as *mut c_void,
        ((*self_0).items)
            .offset(index as isize)
            .offset(1 as c_int as isize) as *const c_void,
        (::core::mem::size_of::<c_ushort>() as c_ulong)
            .wrapping_mul(((*self_0).size - index) as c_ulong),
    );
}
#[no_mangle]
pub unsafe extern "C" fn spUnsignedShortArray_contains(
    mut self_0: *mut spUnsignedShortArray,
    mut value: c_ushort,
) -> c_int {
    let mut items: *mut c_ushort = (*self_0).items;
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    i = 0 as c_int;
    n = (*self_0).size;
    while i < n {
        if *items.offset(i as isize) as c_int == value as c_int {
            return -(1 as c_int);
        }
        i += 1;
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spUnsignedShortArray_pop(
    mut self_0: *mut spUnsignedShortArray,
) -> c_ushort {
    (*self_0).size -= 1;
    let mut item: c_ushort = *((*self_0).items).offset((*self_0).size as isize);
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn spUnsignedShortArray_peek(
    mut self_0: *mut spUnsignedShortArray,
) -> c_ushort {
    return *((*self_0).items).offset(((*self_0).size - 1 as c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn spArrayFloatArray_create(
    mut initialCapacity: c_int,
) -> *mut spArrayFloatArray {
    let mut array: *mut spArrayFloatArray = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spArrayFloatArray>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        4035 as c_int,
    ) as *mut spArrayFloatArray;
    (*array).size = 0 as c_int;
    (*array).capacity = initialCapacity;
    (*array).items = _spCalloc(
        initialCapacity as size_t,
        ::core::mem::size_of::<*mut spFloatArray>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        4035 as c_int,
    ) as *mut *mut spFloatArray;
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn spArrayFloatArray_dispose(mut self_0: *mut spArrayFloatArray) {
    _spFree((*self_0).items as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spArrayFloatArray_clear(mut self_0: *mut spArrayFloatArray) {
    (*self_0).size = 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spArrayFloatArray_setSize(
    mut self_0: *mut spArrayFloatArray,
    mut newSize: c_int,
) -> *mut spArrayFloatArray {
    (*self_0).size = newSize;
    if (*self_0).capacity < newSize {
        (*self_0).capacity = if 8 as c_int > ((*self_0).size as c_float * 1.75f32) as c_int {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0).items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<*mut spFloatArray>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut *mut spFloatArray;
    }
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spArrayFloatArray_ensureCapacity(
    mut self_0: *mut spArrayFloatArray,
    mut newCapacity: c_int,
) {
    if (*self_0).capacity >= newCapacity {
        return;
    }
    (*self_0).capacity = newCapacity;
    (*self_0).items = _spRealloc(
        (*self_0).items as *mut c_void,
        (::core::mem::size_of::<*mut spFloatArray>() as c_ulong)
            .wrapping_mul((*self_0).capacity as c_ulong),
    ) as *mut *mut spFloatArray;
}
#[no_mangle]
pub unsafe extern "C" fn spArrayFloatArray_add(
    mut self_0: *mut spArrayFloatArray,
    mut value: *mut spFloatArray,
) {
    if (*self_0).size == (*self_0).capacity {
        (*self_0).capacity = if 8 as c_int > ((*self_0).size as c_float * 1.75f32) as c_int {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0).items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<*mut spFloatArray>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut *mut spFloatArray;
    }
    let fresh36 = (*self_0).size;
    (*self_0).size = (*self_0).size + 1;
    let ref mut fresh37 = *((*self_0).items).offset(fresh36 as isize);
    *fresh37 = value;
}
#[no_mangle]
pub unsafe extern "C" fn spArrayFloatArray_addAll(
    mut self_0: *mut spArrayFloatArray,
    mut other: *mut spArrayFloatArray,
) {
    let mut i: c_int = 0 as c_int;
    while i < (*other).size {
        spArrayFloatArray_add(self_0, *((*other).items).offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spArrayFloatArray_addAllValues(
    mut self_0: *mut spArrayFloatArray,
    mut values: *mut *mut spFloatArray,
    mut offset: c_int,
    mut count: c_int,
) {
    let mut i: c_int = offset;
    let mut n: c_int = offset + count;
    while i < n {
        spArrayFloatArray_add(self_0, *values.offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spArrayFloatArray_removeAt(
    mut self_0: *mut spArrayFloatArray,
    mut index: c_int,
) {
    (*self_0).size -= 1;
    spine_memmove(
        ((*self_0).items).offset(index as isize) as *mut c_void,
        ((*self_0).items)
            .offset(index as isize)
            .offset(1 as c_int as isize) as *const c_void,
        (::core::mem::size_of::<*mut spFloatArray>() as c_ulong)
            .wrapping_mul(((*self_0).size - index) as c_ulong),
    );
}
#[no_mangle]
pub unsafe extern "C" fn spArrayFloatArray_contains(
    mut self_0: *mut spArrayFloatArray,
    mut value: *mut spFloatArray,
) -> c_int {
    let mut items: *mut *mut spFloatArray = (*self_0).items;
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    i = 0 as c_int;
    n = (*self_0).size;
    while i < n {
        if *items.offset(i as isize) == value {
            return -(1 as c_int);
        }
        i += 1;
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spArrayFloatArray_pop(
    mut self_0: *mut spArrayFloatArray,
) -> *mut spFloatArray {
    (*self_0).size -= 1;
    let mut item: *mut spFloatArray = *((*self_0).items).offset((*self_0).size as isize);
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn spArrayFloatArray_peek(
    mut self_0: *mut spArrayFloatArray,
) -> *mut spFloatArray {
    return *((*self_0).items).offset(((*self_0).size - 1 as c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn spArrayShortArray_create(
    mut initialCapacity: c_int,
) -> *mut spArrayShortArray {
    let mut array: *mut spArrayShortArray = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spArrayShortArray>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        4037 as c_int,
    ) as *mut spArrayShortArray;
    (*array).size = 0 as c_int;
    (*array).capacity = initialCapacity;
    (*array).items = _spCalloc(
        initialCapacity as size_t,
        ::core::mem::size_of::<*mut spShortArray>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        4037 as c_int,
    ) as *mut *mut spShortArray;
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn spArrayShortArray_dispose(mut self_0: *mut spArrayShortArray) {
    _spFree((*self_0).items as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spArrayShortArray_clear(mut self_0: *mut spArrayShortArray) {
    (*self_0).size = 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spArrayShortArray_setSize(
    mut self_0: *mut spArrayShortArray,
    mut newSize: c_int,
) -> *mut spArrayShortArray {
    (*self_0).size = newSize;
    if (*self_0).capacity < newSize {
        (*self_0).capacity = if 8 as c_int > ((*self_0).size as c_float * 1.75f32) as c_int {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0).items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<*mut spShortArray>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut *mut spShortArray;
    }
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spArrayShortArray_ensureCapacity(
    mut self_0: *mut spArrayShortArray,
    mut newCapacity: c_int,
) {
    if (*self_0).capacity >= newCapacity {
        return;
    }
    (*self_0).capacity = newCapacity;
    (*self_0).items = _spRealloc(
        (*self_0).items as *mut c_void,
        (::core::mem::size_of::<*mut spShortArray>() as c_ulong)
            .wrapping_mul((*self_0).capacity as c_ulong),
    ) as *mut *mut spShortArray;
}
#[no_mangle]
pub unsafe extern "C" fn spArrayShortArray_add(
    mut self_0: *mut spArrayShortArray,
    mut value: *mut spShortArray,
) {
    if (*self_0).size == (*self_0).capacity {
        (*self_0).capacity = if 8 as c_int > ((*self_0).size as c_float * 1.75f32) as c_int {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0).items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<*mut spShortArray>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut *mut spShortArray;
    }
    let fresh38 = (*self_0).size;
    (*self_0).size = (*self_0).size + 1;
    let ref mut fresh39 = *((*self_0).items).offset(fresh38 as isize);
    *fresh39 = value;
}
#[no_mangle]
pub unsafe extern "C" fn spArrayShortArray_addAll(
    mut self_0: *mut spArrayShortArray,
    mut other: *mut spArrayShortArray,
) {
    let mut i: c_int = 0 as c_int;
    while i < (*other).size {
        spArrayShortArray_add(self_0, *((*other).items).offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spArrayShortArray_addAllValues(
    mut self_0: *mut spArrayShortArray,
    mut values: *mut *mut spShortArray,
    mut offset: c_int,
    mut count: c_int,
) {
    let mut i: c_int = offset;
    let mut n: c_int = offset + count;
    while i < n {
        spArrayShortArray_add(self_0, *values.offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spArrayShortArray_removeAt(
    mut self_0: *mut spArrayShortArray,
    mut index: c_int,
) {
    (*self_0).size -= 1;
    spine_memmove(
        ((*self_0).items).offset(index as isize) as *mut c_void,
        ((*self_0).items)
            .offset(index as isize)
            .offset(1 as c_int as isize) as *const c_void,
        (::core::mem::size_of::<*mut spShortArray>() as c_ulong)
            .wrapping_mul(((*self_0).size - index) as c_ulong),
    );
}
#[no_mangle]
pub unsafe extern "C" fn spArrayShortArray_contains(
    mut self_0: *mut spArrayShortArray,
    mut value: *mut spShortArray,
) -> c_int {
    let mut items: *mut *mut spShortArray = (*self_0).items;
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    i = 0 as c_int;
    n = (*self_0).size;
    while i < n {
        if *items.offset(i as isize) == value {
            return -(1 as c_int);
        }
        i += 1;
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spArrayShortArray_pop(
    mut self_0: *mut spArrayShortArray,
) -> *mut spShortArray {
    (*self_0).size -= 1;
    let mut item: *mut spShortArray = *((*self_0).items).offset((*self_0).size as isize);
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn spArrayShortArray_peek(
    mut self_0: *mut spArrayShortArray,
) -> *mut spShortArray {
    return *((*self_0).items).offset(((*self_0).size - 1 as c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn spKeyValueArray_create(
    mut initialCapacity: c_int,
) -> *mut spKeyValueArray {
    let mut array: *mut spKeyValueArray = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spKeyValueArray>() as c_ulong,
        b"_file_name_\0" as *const u8 as *const c_char,
        39 as c_int,
    ) as *mut spKeyValueArray;
    (*array).size = 0 as c_int;
    (*array).capacity = initialCapacity;
    (*array).items = _spCalloc(
        initialCapacity as size_t,
        ::core::mem::size_of::<spKeyValue>() as c_ulong,
        b"_file_name_\0" as *const u8 as *const c_char,
        39 as c_int,
    ) as *mut spKeyValue;
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn spKeyValueArray_dispose(mut self_0: *mut spKeyValueArray) {
    _spFree((*self_0).items as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spKeyValueArray_clear(mut self_0: *mut spKeyValueArray) {
    (*self_0).size = 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spKeyValueArray_setSize(
    mut self_0: *mut spKeyValueArray,
    mut newSize: c_int,
) -> *mut spKeyValueArray {
    (*self_0).size = newSize;
    if (*self_0).capacity < newSize {
        (*self_0).capacity = if 8 as c_int > ((*self_0).size as c_float * 1.75f32) as c_int {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0).items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<spKeyValue>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut spKeyValue;
    }
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spKeyValueArray_ensureCapacity(
    mut self_0: *mut spKeyValueArray,
    mut newCapacity: c_int,
) {
    if (*self_0).capacity >= newCapacity {
        return;
    }
    (*self_0).capacity = newCapacity;
    (*self_0).items = _spRealloc(
        (*self_0).items as *mut c_void,
        (::core::mem::size_of::<spKeyValue>() as c_ulong)
            .wrapping_mul((*self_0).capacity as c_ulong),
    ) as *mut spKeyValue;
}
#[no_mangle]
pub unsafe extern "C" fn spKeyValueArray_add(
    mut self_0: *mut spKeyValueArray,
    mut value: spKeyValue,
) {
    if (*self_0).size == (*self_0).capacity {
        (*self_0).capacity = if 8 as c_int > ((*self_0).size as c_float * 1.75f32) as c_int {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0).items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<spKeyValue>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut spKeyValue;
    }
    let fresh40 = (*self_0).size;
    (*self_0).size = (*self_0).size + 1;
    *((*self_0).items).offset(fresh40 as isize) = value;
}
#[no_mangle]
pub unsafe extern "C" fn spKeyValueArray_addAll(
    mut self_0: *mut spKeyValueArray,
    mut other: *mut spKeyValueArray,
) {
    let mut i: c_int = 0 as c_int;
    while i < (*other).size {
        spKeyValueArray_add(self_0, *((*other).items).offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spKeyValueArray_addAllValues(
    mut self_0: *mut spKeyValueArray,
    mut values: *mut spKeyValue,
    mut offset: c_int,
    mut count: c_int,
) {
    let mut i: c_int = offset;
    let mut n: c_int = offset + count;
    while i < n {
        spKeyValueArray_add(self_0, *values.offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spKeyValueArray_contains(
    mut self_0: *mut spKeyValueArray,
    mut value: spKeyValue,
) -> c_int {
    let mut items: *mut spKeyValue = (*self_0).items;
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    i = 0 as c_int;
    n = (*self_0).size;
    while i < n {
        if spine_strcmp((*items.offset(i as isize)).name, value.name) == 0 {
            return -(1 as c_int);
        }
        i += 1;
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spKeyValueArray_pop(mut self_0: *mut spKeyValueArray) -> spKeyValue {
    (*self_0).size -= 1;
    let mut item: spKeyValue = *((*self_0).items).offset((*self_0).size as isize);
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn spKeyValueArray_peek(mut self_0: *mut spKeyValueArray) -> spKeyValue {
    return *((*self_0).items).offset(((*self_0).size - 1 as c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn spAtlasPage_create(
    mut atlas: *mut spAtlas,
    mut name: *const c_char,
) -> *mut spAtlasPage {
    let mut self_0: *mut spAtlasPage = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spAtlasPage>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        4136 as c_int,
    ) as *mut spAtlasPage;
    let ref mut fresh41 = *(&mut (*self_0).atlas as *mut *const spAtlas as *mut *mut spAtlas);
    *fresh41 = atlas;
    let ref mut fresh42 = *(&mut (*self_0).name as *mut *const c_char as *mut *mut c_char);
    *fresh42 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul((spine_strlen(name)).wrapping_add(1 as c_int as c_ulong)),
        b"spine.c\0" as *const u8 as *const c_char,
        4138 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh42, name);
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spAtlasPage_dispose(mut self_0: *mut spAtlasPage) {
    _spAtlasPage_disposeTexture(self_0);
    _spFree((*self_0).name as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spAtlasRegion_create() -> *mut spAtlasRegion {
    let mut region: *mut spAtlasRegion = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spAtlasRegion>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        4151 as c_int,
    ) as *mut spAtlasRegion;
    (*region).keyValues = spKeyValueArray_create(2 as c_int);
    return region;
}
#[no_mangle]
pub unsafe extern "C" fn spAtlasRegion_dispose(mut self_0: *mut spAtlasRegion) {
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    _spFree((*self_0).name as *mut c_void);
    _spFree((*self_0).splits as *mut c_void);
    _spFree((*self_0).pads as *mut c_void);
    i = 0 as c_int;
    n = (*(*self_0).keyValues).size;
    while i < n {
        _spFree((*((*(*self_0).keyValues).items).offset(i as isize)).name as *mut c_void);
        i += 1;
    }
    spKeyValueArray_dispose((*self_0).keyValues);
    _spFree(self_0 as *mut c_void);
}
unsafe extern "C" fn ss_trim(mut self_0: *mut SimpleString) -> *mut SimpleString {
    while isspace_(*(*self_0).start as c_uchar as c_int) != 0 && (*self_0).start < (*self_0).end {
        (*self_0).start = ((*self_0).start).offset(1);
    }
    if (*self_0).start == (*self_0).end {
        (*self_0).length = ((*self_0).end).offset_from((*self_0).start) as c_long as c_int;
        return self_0;
    }
    (*self_0).end = ((*self_0).end).offset(-1);
    while *(*self_0).end as c_uchar as c_int == '\r' as i32 && (*self_0).end >= (*self_0).start {
        (*self_0).end = ((*self_0).end).offset(-1);
    }
    (*self_0).end = ((*self_0).end).offset(1);
    (*self_0).length = ((*self_0).end).offset_from((*self_0).start) as c_long as c_int;
    return self_0;
}
unsafe extern "C" fn ss_indexOf(mut self_0: *mut SimpleString, mut needle: c_char) -> c_int {
    let mut c: *mut c_char = (*self_0).start;
    while c < (*self_0).end {
        if *c as c_int == needle as c_int {
            return c.offset_from((*self_0).start) as c_long as c_int;
        }
        c = c.offset(1);
    }
    return -(1 as c_int);
}
unsafe extern "C" fn ss_indexOf2(
    mut self_0: *mut SimpleString,
    mut needle: c_char,
    mut at: c_int,
) -> c_int {
    let mut c: *mut c_char = ((*self_0).start).offset(at as isize);
    while c < (*self_0).end {
        if *c as c_int == needle as c_int {
            return c.offset_from((*self_0).start) as c_long as c_int;
        }
        c = c.offset(1);
    }
    return -(1 as c_int);
}
unsafe extern "C" fn ss_substr(
    mut self_0: *mut SimpleString,
    mut s: c_int,
    mut e: c_int,
) -> SimpleString {
    let mut result: SimpleString = SimpleString {
        start: 0 as *mut c_char,
        end: 0 as *mut c_char,
        length: 0,
    };
    e = s + e;
    result.start = ((*self_0).start).offset(s as isize);
    result.end = ((*self_0).start).offset(e as isize);
    result.length = e - s;
    return result;
}
unsafe extern "C" fn ss_substr2(mut self_0: *mut SimpleString, mut s: c_int) -> SimpleString {
    let mut result: SimpleString = SimpleString {
        start: 0 as *mut c_char,
        end: 0 as *mut c_char,
        length: 0,
    };
    result.start = ((*self_0).start).offset(s as isize);
    result.end = (*self_0).end;
    result.length = (result.end).offset_from(result.start) as c_long as c_int;
    return result;
}
unsafe extern "C" fn ss_equals(mut self_0: *mut SimpleString, mut str: *const c_char) -> c_int {
    let mut i: c_int = 0;
    let mut otherLen: c_int = spine_strlen(str) as c_int;
    if (*self_0).length != otherLen {
        return 0 as c_int;
    }
    i = 0 as c_int;
    while i < (*self_0).length {
        if *((*self_0).start).offset(i as isize) as c_int != *str.offset(i as isize) as c_int {
            return 0 as c_int;
        }
        i += 1;
    }
    return -(1 as c_int);
}
unsafe extern "C" fn ss_copy(mut self_0: *mut SimpleString) -> *mut c_char {
    let mut string: *mut c_char = _spCalloc(
        ((*self_0).length + 1 as c_int) as size_t,
        ::core::mem::size_of::<c_char>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        4237 as c_int,
    ) as *mut c_char;
    spine_memcpy(
        string as *mut c_void,
        (*self_0).start as *const c_void,
        (*self_0).length as size_t,
    );
    *string.offset((*self_0).length as isize) = '\0' as i32 as c_char;
    return string;
}
unsafe extern "C" fn ss_toInt(mut self_0: *mut SimpleString) -> c_int {
    return spine_strtol((*self_0).start, &mut (*self_0).end, 10 as c_int) as c_int;
}
unsafe extern "C" fn ai_readLine(mut self_0: *mut AtlasInput) -> *mut SimpleString {
    if (*self_0).index >= (*self_0).end as *mut c_char {
        return 0 as *mut SimpleString;
    }
    (*self_0).line.start = (*self_0).index;
    while (*self_0).index < (*self_0).end as *mut c_char && *(*self_0).index as c_int != '\n' as i32
    {
        (*self_0).index = ((*self_0).index).offset(1);
    }
    (*self_0).line.end = (*self_0).index;
    if (*self_0).index != (*self_0).end as *mut c_char {
        (*self_0).index = ((*self_0).index).offset(1);
    }
    (*self_0).line = *ss_trim(&mut (*self_0).line);
    (*self_0).line.length =
        ((*self_0).line.end).offset_from((*self_0).line.start) as c_long as c_int;
    return &mut (*self_0).line;
}
unsafe extern "C" fn ai_readEntry(
    mut entry: *mut SimpleString,
    mut line: *mut SimpleString,
) -> c_int {
    let mut colon: c_int = 0;
    let mut i: c_int = 0;
    let mut lastMatch: c_int = 0;
    let mut substr: SimpleString = SimpleString {
        start: 0 as *mut c_char,
        end: 0 as *mut c_char,
        length: 0,
    };
    if line.is_null() {
        return 0 as c_int;
    }
    ss_trim(line);
    if (*line).length == 0 as c_int {
        return 0 as c_int;
    }
    colon = ss_indexOf(line, ':' as i32 as c_char);
    if colon == -(1 as c_int) {
        return 0 as c_int;
    }
    substr = ss_substr(line, 0 as c_int, colon);
    *entry.offset(0 as c_int as isize) = *ss_trim(&mut substr);
    i = 1 as c_int;
    lastMatch = colon + 1 as c_int;
    loop {
        let mut comma: c_int = ss_indexOf2(line, ',' as i32 as c_char, lastMatch);
        if comma == -(1 as c_int) {
            substr = ss_substr2(line, lastMatch);
            *entry.offset(i as isize) = *ss_trim(&mut substr);
            return i;
        }
        substr = ss_substr(line, lastMatch, comma - lastMatch);
        *entry.offset(i as isize) = *ss_trim(&mut substr);
        lastMatch = comma + 1 as c_int;
        if i == 4 as c_int {
            return 4 as c_int;
        }
        i += 1;
    }
}
static mut formatNames: [*const c_char; 8] = [
    b"\0" as *const u8 as *const c_char,
    b"Alpha\0" as *const u8 as *const c_char,
    b"Intensity\0" as *const u8 as *const c_char,
    b"LuminanceAlpha\0" as *const u8 as *const c_char,
    b"RGB565\0" as *const u8 as *const c_char,
    b"RGBA4444\0" as *const u8 as *const c_char,
    b"RGB888\0" as *const u8 as *const c_char,
    b"RGBA8888\0" as *const u8 as *const c_char,
];
static mut textureFilterNames: [*const c_char; 8] = [
    b"\0" as *const u8 as *const c_char,
    b"Nearest\0" as *const u8 as *const c_char,
    b"Linear\0" as *const u8 as *const c_char,
    b"MipMap\0" as *const u8 as *const c_char,
    b"MipMapNearestNearest\0" as *const u8 as *const c_char,
    b"MipMapLinearNearest\0" as *const u8 as *const c_char,
    b"MipMapNearestLinear\0" as *const u8 as *const c_char,
    b"MipMapLinearLinear\0" as *const u8 as *const c_char,
];
#[no_mangle]
pub unsafe extern "C" fn indexOf(
    mut array: *mut *const c_char,
    mut count: c_int,
    mut str: *mut SimpleString,
) -> c_int {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < count {
        if ss_equals(str, *array.offset(i as isize)) != 0 {
            return i;
        }
        i += 1;
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spAtlas_create(
    mut begin: *const c_char,
    mut length: c_int,
    mut dir: *const c_char,
    mut rendererObject: *mut c_void,
) -> *mut spAtlas {
    let mut self_0: *mut spAtlas = 0 as *mut spAtlas;
    let mut reader: AtlasInput = AtlasInput {
        start: 0 as *const c_char,
        end: 0 as *const c_char,
        index: 0 as *mut c_char,
        length: 0,
        line: SimpleString {
            start: 0 as *mut c_char,
            end: 0 as *mut c_char,
            length: 0,
        },
    };
    let mut line: *mut SimpleString = 0 as *mut SimpleString;
    let mut entry: [SimpleString; 5] = [SimpleString {
        start: 0 as *mut c_char,
        end: 0 as *mut c_char,
        length: 0,
    }; 5];
    let mut page: *mut spAtlasPage = 0 as *mut spAtlasPage;
    let mut lastPage: *mut spAtlasPage = 0 as *mut spAtlasPage;
    let mut lastRegion: *mut spAtlasRegion = 0 as *mut spAtlasRegion;
    let mut count: c_int = 0;
    let mut dirLength: c_int = spine_strlen(dir) as c_int;
    let mut needsSlash: c_int = (dirLength > 0 as c_int
        && *dir.offset((dirLength - 1 as c_int) as isize) as c_int != '/' as i32
        && *dir.offset((dirLength - 1 as c_int) as isize) as c_int != '\\' as i32)
        as c_int;
    self_0 = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spAtlas>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        4318 as c_int,
    ) as *mut spAtlas;
    (*self_0).rendererObject = rendererObject;
    reader.start = begin;
    reader.end = begin.offset(length as isize);
    reader.index = begin as *mut c_char;
    reader.length = length;
    line = ai_readLine(&mut reader);
    while !line.is_null() && (*line).length == 0 as c_int {
        line = ai_readLine(&mut reader);
    }
    while -(1 as c_int) != 0 {
        if line.is_null() || (*line).length == 0 as c_int {
            break;
        }
        if ai_readEntry(entry.as_mut_ptr(), line) == 0 as c_int {
            break;
        }
        line = ai_readLine(&mut reader);
    }
    while -(1 as c_int) != 0 {
        if line.is_null() {
            break;
        }
        if (*ss_trim(line)).length == 0 as c_int {
            page = 0 as *mut spAtlasPage;
            line = ai_readLine(&mut reader);
        } else if page.is_null() {
            let mut name: *mut c_char = ss_copy(line);
            let mut path: *mut c_char = _spCalloc(
                ((dirLength + needsSlash) as c_ulong)
                    .wrapping_add(spine_strlen(name))
                    .wrapping_add(1 as c_int as c_ulong),
                ::core::mem::size_of::<c_char>() as c_ulong,
                b"spine.c\0" as *const u8 as *const c_char,
                4343 as c_int,
            ) as *mut c_char;
            spine_memcpy(
                path as *mut c_void,
                dir as *const c_void,
                dirLength as size_t,
            );
            if needsSlash != 0 {
                *path.offset(dirLength as isize) = '/' as i32 as c_char;
            }
            spine_strcpy(
                path.offset(dirLength as isize).offset(needsSlash as isize),
                name,
            );
            page = spAtlasPage_create(self_0, name);
            _spFree(name as *mut c_void);
            if !lastPage.is_null() {
                (*lastPage).next = page;
            } else {
                (*self_0).pages = page;
            }
            lastPage = page;
            while -(1 as c_int) != 0 {
                line = ai_readLine(&mut reader);
                if ai_readEntry(entry.as_mut_ptr(), line) == 0 as c_int {
                    break;
                }
                if ss_equals(
                    &mut *entry.as_mut_ptr().offset(0 as c_int as isize),
                    b"size\0" as *const u8 as *const c_char,
                ) != 0
                {
                    (*page).width = ss_toInt(&mut *entry.as_mut_ptr().offset(1 as c_int as isize));
                    (*page).height = ss_toInt(&mut *entry.as_mut_ptr().offset(2 as c_int as isize));
                } else if ss_equals(
                    &mut *entry.as_mut_ptr().offset(0 as c_int as isize),
                    b"format\0" as *const u8 as *const c_char,
                ) != 0
                {
                    (*page).format = indexOf(
                        formatNames.as_mut_ptr(),
                        8 as c_int,
                        &mut *entry.as_mut_ptr().offset(1 as c_int as isize),
                    ) as spAtlasFormat;
                } else if ss_equals(
                    &mut *entry.as_mut_ptr().offset(0 as c_int as isize),
                    b"filter\0" as *const u8 as *const c_char,
                ) != 0
                {
                    (*page).minFilter = indexOf(
                        textureFilterNames.as_mut_ptr(),
                        8 as c_int,
                        &mut *entry.as_mut_ptr().offset(1 as c_int as isize),
                    ) as spAtlasFilter;
                    (*page).magFilter = indexOf(
                        textureFilterNames.as_mut_ptr(),
                        8 as c_int,
                        &mut *entry.as_mut_ptr().offset(2 as c_int as isize),
                    ) as spAtlasFilter;
                } else if ss_equals(
                    &mut *entry.as_mut_ptr().offset(0 as c_int as isize),
                    b"repeat\0" as *const u8 as *const c_char,
                ) != 0
                {
                    (*page).uWrap = SP_ATLAS_CLAMPTOEDGE;
                    (*page).vWrap = SP_ATLAS_CLAMPTOEDGE;
                    if ss_indexOf(
                        &mut *entry.as_mut_ptr().offset(1 as c_int as isize),
                        'x' as i32 as c_char,
                    ) != -(1 as c_int)
                    {
                        (*page).uWrap = SP_ATLAS_REPEAT;
                    }
                    if ss_indexOf(
                        &mut *entry.as_mut_ptr().offset(1 as c_int as isize),
                        'y' as i32 as c_char,
                    ) != -(1 as c_int)
                    {
                        (*page).vWrap = SP_ATLAS_REPEAT;
                    }
                } else if ss_equals(
                    &mut *entry.as_mut_ptr().offset(0 as c_int as isize),
                    b"pma\0" as *const u8 as *const c_char,
                ) != 0
                {
                    (*page).pma = ss_equals(
                        &mut *entry.as_mut_ptr().offset(1 as c_int as isize),
                        b"true\0" as *const u8 as *const c_char,
                    );
                }
            }
            _spAtlasPage_createTexture(page, path);
            _spFree(path as *mut c_void);
        } else {
            let mut region: *mut spAtlasRegion = spAtlasRegion_create();
            if !lastRegion.is_null() {
                (*lastRegion).next = region;
            } else {
                (*self_0).regions = region;
            }
            lastRegion = region;
            (*region).page = page;
            (*region).name = ss_copy(line);
            while -(1 as c_int) != 0 {
                line = ai_readLine(&mut reader);
                count = ai_readEntry(entry.as_mut_ptr(), line);
                if count == 0 as c_int {
                    break;
                }
                if ss_equals(
                    &mut *entry.as_mut_ptr().offset(0 as c_int as isize),
                    b"xy\0" as *const u8 as *const c_char,
                ) != 0
                {
                    (*region).x = ss_toInt(&mut *entry.as_mut_ptr().offset(1 as c_int as isize));
                    (*region).y = ss_toInt(&mut *entry.as_mut_ptr().offset(2 as c_int as isize));
                } else if ss_equals(
                    &mut *entry.as_mut_ptr().offset(0 as c_int as isize),
                    b"size\0" as *const u8 as *const c_char,
                ) != 0
                {
                    (*region).super_0.width =
                        ss_toInt(&mut *entry.as_mut_ptr().offset(1 as c_int as isize));
                    (*region).super_0.height =
                        ss_toInt(&mut *entry.as_mut_ptr().offset(2 as c_int as isize));
                } else if ss_equals(
                    &mut *entry.as_mut_ptr().offset(0 as c_int as isize),
                    b"bounds\0" as *const u8 as *const c_char,
                ) != 0
                {
                    (*region).x = ss_toInt(&mut *entry.as_mut_ptr().offset(1 as c_int as isize));
                    (*region).y = ss_toInt(&mut *entry.as_mut_ptr().offset(2 as c_int as isize));
                    (*region).super_0.width =
                        ss_toInt(&mut *entry.as_mut_ptr().offset(3 as c_int as isize));
                    (*region).super_0.height =
                        ss_toInt(&mut *entry.as_mut_ptr().offset(4 as c_int as isize));
                } else if ss_equals(
                    &mut *entry.as_mut_ptr().offset(0 as c_int as isize),
                    b"offset\0" as *const u8 as *const c_char,
                ) != 0
                {
                    (*region).super_0.offsetX =
                        ss_toInt(&mut *entry.as_mut_ptr().offset(1 as c_int as isize)) as c_float;
                    (*region).super_0.offsetY =
                        ss_toInt(&mut *entry.as_mut_ptr().offset(2 as c_int as isize)) as c_float;
                } else if ss_equals(
                    &mut *entry.as_mut_ptr().offset(0 as c_int as isize),
                    b"orig\0" as *const u8 as *const c_char,
                ) != 0
                {
                    (*region).super_0.originalWidth =
                        ss_toInt(&mut *entry.as_mut_ptr().offset(1 as c_int as isize));
                    (*region).super_0.originalHeight =
                        ss_toInt(&mut *entry.as_mut_ptr().offset(2 as c_int as isize));
                } else if ss_equals(
                    &mut *entry.as_mut_ptr().offset(0 as c_int as isize),
                    b"offsets\0" as *const u8 as *const c_char,
                ) != 0
                {
                    (*region).super_0.offsetX =
                        ss_toInt(&mut *entry.as_mut_ptr().offset(1 as c_int as isize)) as c_float;
                    (*region).super_0.offsetY =
                        ss_toInt(&mut *entry.as_mut_ptr().offset(2 as c_int as isize)) as c_float;
                    (*region).super_0.originalWidth =
                        ss_toInt(&mut *entry.as_mut_ptr().offset(3 as c_int as isize));
                    (*region).super_0.originalHeight =
                        ss_toInt(&mut *entry.as_mut_ptr().offset(4 as c_int as isize));
                } else if ss_equals(
                    &mut *entry.as_mut_ptr().offset(0 as c_int as isize),
                    b"rotate\0" as *const u8 as *const c_char,
                ) != 0
                {
                    if ss_equals(
                        &mut *entry.as_mut_ptr().offset(1 as c_int as isize),
                        b"true\0" as *const u8 as *const c_char,
                    ) != 0
                    {
                        (*region).super_0.degrees = 90 as c_int;
                    } else if ss_equals(
                        &mut *entry.as_mut_ptr().offset(1 as c_int as isize),
                        b"false\0" as *const u8 as *const c_char,
                    ) == 0
                    {
                        (*region).super_0.degrees =
                            ss_toInt(&mut *entry.as_mut_ptr().offset(1 as c_int as isize));
                    }
                } else if ss_equals(
                    &mut *entry.as_mut_ptr().offset(0 as c_int as isize),
                    b"index\0" as *const u8 as *const c_char,
                ) != 0
                {
                    (*region).index =
                        ss_toInt(&mut *entry.as_mut_ptr().offset(1 as c_int as isize));
                } else {
                    let mut i: c_int = 0 as c_int;
                    let mut keyValue: spKeyValue = spKeyValue {
                        name: 0 as *mut c_char,
                        values: [0.; 5],
                    };
                    keyValue.name = ss_copy(&mut *entry.as_mut_ptr().offset(0 as c_int as isize));
                    i = 0 as c_int;
                    while i < count {
                        keyValue.values[i as usize] =
                            ss_toInt(&mut *entry.as_mut_ptr().offset((i + 1 as c_int) as isize))
                                as c_float;
                        i += 1;
                    }
                    spKeyValueArray_add((*region).keyValues, keyValue);
                }
            }
            if (*region).super_0.originalWidth == 0 as c_int
                && (*region).super_0.originalHeight == 0 as c_int
            {
                (*region).super_0.originalWidth = (*region).super_0.width;
                (*region).super_0.originalHeight = (*region).super_0.height;
            }
            (*region).super_0.u = (*region).x as c_float / (*page).width as c_float;
            (*region).super_0.v = (*region).y as c_float / (*page).height as c_float;
            if (*region).super_0.degrees == 90 as c_int {
                (*region).super_0.u2 =
                    ((*region).x + (*region).super_0.height) as c_float / (*page).width as c_float;
                (*region).super_0.v2 =
                    ((*region).y + (*region).super_0.width) as c_float / (*page).height as c_float;
            } else {
                (*region).super_0.u2 =
                    ((*region).x + (*region).super_0.width) as c_float / (*page).width as c_float;
                (*region).super_0.v2 =
                    ((*region).y + (*region).super_0.height) as c_float / (*page).height as c_float;
            }
        }
    }
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spAtlas_createFromFile(
    mut path: *const c_char,
    mut rendererObject: *mut c_void,
) -> *mut spAtlas {
    let mut dirLength: c_int = 0;
    let mut dir: *mut c_char = 0 as *mut c_char;
    let mut length: c_int = 0;
    let mut data: *const c_char = 0 as *const c_char;
    let mut atlas: *mut spAtlas = 0 as *mut spAtlas;
    let mut lastForwardSlash: *const c_char = spine_strrchr(path, '/' as i32);
    let mut lastBackwardSlash: *const c_char = spine_strrchr(path, '\\' as i32);
    let mut lastSlash: *const c_char = if lastForwardSlash > lastBackwardSlash {
        lastForwardSlash
    } else {
        lastBackwardSlash
    };
    if lastSlash == path {
        lastSlash = lastSlash.offset(1);
    }
    dirLength = (if !lastSlash.is_null() {
        lastSlash.offset_from(path) as c_long
    } else {
        0 as c_int as c_long
    }) as c_int;
    dir = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul((dirLength + 1 as c_int) as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        4466 as c_int,
    ) as *mut c_char;
    spine_memcpy(
        dir as *mut c_void,
        path as *const c_void,
        dirLength as size_t,
    );
    *dir.offset(dirLength as isize) = '\0' as i32 as c_char;
    data = _spUtil_readFile(path, &mut length);
    if !data.is_null() {
        atlas = spAtlas_create(data, length, dir, rendererObject);
    }
    _spFree(data as *mut c_void);
    _spFree(dir as *mut c_void);
    return atlas;
}
#[no_mangle]
pub unsafe extern "C" fn spAtlas_dispose(mut self_0: *mut spAtlas) {
    let mut region: *mut spAtlasRegion = 0 as *mut spAtlasRegion;
    let mut nextRegion: *mut spAtlasRegion = 0 as *mut spAtlasRegion;
    let mut page: *mut spAtlasPage = (*self_0).pages;
    while !page.is_null() {
        let mut nextPage: *mut spAtlasPage = (*page).next;
        spAtlasPage_dispose(page);
        page = nextPage;
    }
    region = (*self_0).regions;
    while !region.is_null() {
        nextRegion = (*region).next;
        spAtlasRegion_dispose(region);
        region = nextRegion;
    }
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spAtlas_findRegion(
    mut self_0: *const spAtlas,
    mut name: *const c_char,
) -> *mut spAtlasRegion {
    let mut region: *mut spAtlasRegion = (*self_0).regions;
    while !region.is_null() {
        if spine_strcmp((*region).name, name) == 0 as c_int {
            return region;
        }
        region = (*region).next;
    }
    return 0 as *mut spAtlasRegion;
}
unsafe extern "C" fn loadSequence(
    mut atlas: *mut spAtlas,
    mut basePath: *const c_char,
    mut sequence: *mut spSequence,
) -> c_int {
    let mut regions: *mut spTextureRegionArray = (*sequence).regions;
    let mut path: *mut c_char = _spCalloc(
        (spine_strlen(basePath))
            .wrapping_add((*sequence).digits as c_ulong)
            .wrapping_add(1 as c_int as c_ulong),
        ::core::mem::size_of::<c_char>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        4540 as c_int,
    ) as *mut c_char;
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*regions).size {
        spSequence_getPath(sequence, basePath, i, path);
        let ref mut fresh43 = *((*regions).items).offset(i as isize);
        *fresh43 = &mut (*(spAtlas_findRegion
            as unsafe extern "C" fn(*const spAtlas, *const c_char) -> *mut spAtlasRegion)(
            atlas, path,
        ))
        .super_0;
        if (*((*regions).items).offset(i as isize)).is_null() {
            _spFree(path as *mut c_void);
            return 0 as c_int;
        }
        let ref mut fresh44 = (**((*regions).items).offset(i as isize)).rendererObject;
        *fresh44 = *((*regions).items).offset(i as isize) as *mut c_void;
        i += 1;
    }
    _spFree(path as *mut c_void);
    return -(1 as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn _spAtlasAttachmentLoader_createAttachment(
    mut loader: *mut spAttachmentLoader,
    mut _skin: *mut spSkin,
    mut type_0: spAttachmentType,
    mut name: *const c_char,
    mut path: *const c_char,
    mut sequence: *mut spSequence,
) -> *mut spAttachment {
    let mut self_0: *mut spAtlasAttachmentLoader = loader as *mut spAtlasAttachmentLoader;
    match type_0 as c_uint {
        0 => {
            let mut attachment: *mut spRegionAttachment = spRegionAttachment_create(name);
            if !sequence.is_null() {
                if loadSequence((*self_0).atlas, path, sequence) == 0 {
                    spAttachment_dispose(&mut (*attachment).super_0);
                    _spAttachmentLoader_setError(
                        loader,
                        b"Couldn't load sequence for region attachment: \0" as *const u8
                            as *const c_char,
                        path,
                    );
                    return 0 as *mut spAttachment;
                }
            } else {
                let mut region: *mut spAtlasRegion = spAtlas_findRegion((*self_0).atlas, path);
                if region.is_null() {
                    spAttachment_dispose(&mut (*attachment).super_0);
                    _spAttachmentLoader_setError(
                        loader,
                        b"Region not found: \0" as *const u8 as *const c_char,
                        path,
                    );
                    return 0 as *mut spAttachment;
                }
                (*attachment).rendererObject = region as *mut c_void;
                (*attachment).region = &mut (*region).super_0;
            }
            return &mut (*attachment).super_0;
        }
        2 | 3 => {
            let mut attachment_0: *mut spMeshAttachment = spMeshAttachment_create(name);
            if !sequence.is_null() {
                if loadSequence((*self_0).atlas, path, sequence) == 0 {
                    spAttachment_dispose(&mut (*attachment_0).super_0.super_0);
                    _spAttachmentLoader_setError(
                        loader,
                        b"Couldn't load sequence for mesh attachment: \0" as *const u8
                            as *const c_char,
                        path,
                    );
                    return 0 as *mut spAttachment;
                }
            } else {
                let mut region_0: *mut spAtlasRegion = spAtlas_findRegion((*self_0).atlas, path);
                if region_0.is_null() {
                    _spAttachmentLoader_setError(
                        loader,
                        b"Region not found: \0" as *const u8 as *const c_char,
                        path,
                    );
                    return 0 as *mut spAttachment;
                }
                (*attachment_0).rendererObject = region_0 as *mut c_void;
                (*attachment_0).region = &mut (*region_0).super_0;
            }
            return &mut (*attachment_0).super_0.super_0;
        }
        1 => {
            return &mut (*(spBoundingBoxAttachment_create
                as unsafe extern "C" fn(*const c_char) -> *mut spBoundingBoxAttachment)(
                name
            ))
            .super_0
            .super_0;
        }
        4 => {
            return &mut (*(spPathAttachment_create
                as unsafe extern "C" fn(*const c_char) -> *mut spPathAttachment)(
                name
            ))
            .super_0
            .super_0;
        }
        5 => {
            return &mut (*(spPointAttachment_create
                as unsafe extern "C" fn(*const c_char) -> *mut spPointAttachment)(
                name
            ))
            .super_0;
        }
        6 => {
            return &mut (*(spClippingAttachment_create
                as unsafe extern "C" fn(*const c_char) -> *mut spClippingAttachment)(
                name
            ))
            .super_0
            .super_0;
        }
        _ => {
            _spAttachmentLoader_setUnknownTypeError(loader, type_0);
            return 0 as *mut spAttachment;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn spAtlasAttachmentLoader_create(
    mut atlas: *mut spAtlas,
) -> *mut spAtlasAttachmentLoader {
    let mut self_0: *mut spAtlasAttachmentLoader = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spAtlasAttachmentLoader>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        4617 as c_int,
    ) as *mut spAtlasAttachmentLoader;
    _spAttachmentLoader_init(
        &mut (*self_0).super_0,
        Some(_spAttachmentLoader_deinit as unsafe extern "C" fn(*mut spAttachmentLoader) -> ()),
        Some(
            _spAtlasAttachmentLoader_createAttachment
                as unsafe extern "C" fn(
                    *mut spAttachmentLoader,
                    *mut spSkin,
                    spAttachmentType,
                    *const c_char,
                    *const c_char,
                    *mut spSequence,
                ) -> *mut spAttachment,
        ),
        None,
        None,
    );
    (*self_0).atlas = atlas;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn _spAttachment_init(
    mut self_0: *mut spAttachment,
    mut name: *const c_char,
    mut type_0: spAttachmentType,
    mut dispose: Option<unsafe extern "C" fn(*mut spAttachment) -> ()>,
    mut copy: Option<unsafe extern "C" fn(*mut spAttachment) -> *mut spAttachment>,
) {
    let ref mut fresh45 =
        *(&(*self_0).vtable as *const *const c_void as *mut *mut _spAttachmentVtable);
    *fresh45 = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<_spAttachmentVtable>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        4664 as c_int,
    ) as *mut _spAttachmentVtable;
    let ref mut fresh46 = (*((*self_0).vtable as *mut _spAttachmentVtable)).dispose;
    *fresh46 = dispose;
    let ref mut fresh47 = (*((*self_0).vtable as *mut _spAttachmentVtable)).copy;
    *fresh47 = copy;
    let ref mut fresh48 = *(&(*self_0).name as *const *const c_char as *mut *mut c_char);
    *fresh48 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul((spine_strlen(name)).wrapping_add(1 as c_int as c_ulong)),
        b"spine.c\0" as *const u8 as *const c_char,
        4668 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh48, name);
    *(&(*self_0).type_0 as *const spAttachmentType as *mut spAttachmentType) = type_0;
}
#[no_mangle]
pub unsafe extern "C" fn _spAttachment_deinit(mut self_0: *mut spAttachment) {
    if !((*self_0).attachmentLoader).is_null() {
        spAttachmentLoader_disposeAttachment((*self_0).attachmentLoader, self_0);
    }
    _spFree((*self_0).vtable as *mut c_void);
    _spFree((*self_0).name as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spAttachment_copy(mut self_0: *mut spAttachment) -> *mut spAttachment {
    return ((*((*self_0).vtable as *mut _spAttachmentVtable)).copy)
        .expect("non-null function pointer")(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn spAttachment_dispose(mut self_0: *mut spAttachment) {
    (*self_0).refCount -= 1;
    if (*self_0).refCount <= 0 as c_int {
        ((*((*self_0).vtable as *mut _spAttachmentVtable)).dispose)
            .expect("non-null function pointer")(self_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _spAttachmentLoader_init(
    mut self_0: *mut spAttachmentLoader,
    mut dispose: Option<unsafe extern "C" fn(*mut spAttachmentLoader) -> ()>,
    mut createAttachment: Option<
        unsafe extern "C" fn(
            *mut spAttachmentLoader,
            *mut spSkin,
            spAttachmentType,
            *const c_char,
            *const c_char,
            *mut spSequence,
        ) -> *mut spAttachment,
    >,
    mut configureAttachment: Option<
        unsafe extern "C" fn(*mut spAttachmentLoader, *mut spAttachment) -> (),
    >,
    mut disposeAttachment: Option<
        unsafe extern "C" fn(*mut spAttachmentLoader, *mut spAttachment) -> (),
    >,
) {
    let ref mut fresh49 =
        *(&(*self_0).vtable as *const *const c_void as *mut *mut _spAttachmentLoaderVtable);
    *fresh49 = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<_spAttachmentLoaderVtable>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        4738 as c_int,
    ) as *mut _spAttachmentLoaderVtable;
    let ref mut fresh50 = (*((*self_0).vtable as *mut _spAttachmentLoaderVtable)).dispose;
    *fresh50 = dispose;
    let ref mut fresh51 = (*((*self_0).vtable as *mut _spAttachmentLoaderVtable)).createAttachment;
    *fresh51 = createAttachment;
    let ref mut fresh52 =
        (*((*self_0).vtable as *mut _spAttachmentLoaderVtable)).configureAttachment;
    *fresh52 = configureAttachment;
    let ref mut fresh53 = (*((*self_0).vtable as *mut _spAttachmentLoaderVtable)).disposeAttachment;
    *fresh53 = disposeAttachment;
}
#[no_mangle]
pub unsafe extern "C" fn _spAttachmentLoader_deinit(mut self_0: *mut spAttachmentLoader) {
    _spFree((*self_0).vtable as *mut c_void);
    _spFree((*self_0).error1 as *mut c_void);
    _spFree((*self_0).error2 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spAttachmentLoader_dispose(mut self_0: *mut spAttachmentLoader) {
    ((*((*self_0).vtable as *mut _spAttachmentLoaderVtable)).dispose)
        .expect("non-null function pointer")(self_0);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spAttachmentLoader_createAttachment(
    mut self_0: *mut spAttachmentLoader,
    mut skin: *mut spSkin,
    mut type_0: spAttachmentType,
    mut name: *const c_char,
    mut path: *const c_char,
    mut sequence: *mut spSequence,
) -> *mut spAttachment {
    _spFree((*self_0).error1 as *mut c_void);
    _spFree((*self_0).error2 as *mut c_void);
    (*self_0).error1 = 0 as *const c_char;
    (*self_0).error2 = 0 as *const c_char;
    return ((*((*self_0).vtable as *mut _spAttachmentLoaderVtable)).createAttachment)
        .expect("non-null function pointer")(self_0, skin, type_0, name, path, sequence);
}
#[no_mangle]
pub unsafe extern "C" fn spAttachmentLoader_configureAttachment(
    mut self_0: *mut spAttachmentLoader,
    mut attachment: *mut spAttachment,
) {
    if ((*((*self_0).vtable as *mut _spAttachmentLoaderVtable)).configureAttachment).is_none() {
        return;
    }
    ((*((*self_0).vtable as *mut _spAttachmentLoaderVtable)).configureAttachment)
        .expect("non-null function pointer")(self_0, attachment);
}
#[no_mangle]
pub unsafe extern "C" fn spAttachmentLoader_disposeAttachment(
    mut self_0: *mut spAttachmentLoader,
    mut attachment: *mut spAttachment,
) {
    if ((*((*self_0).vtable as *mut _spAttachmentLoaderVtable)).disposeAttachment).is_none() {
        return;
    }
    ((*((*self_0).vtable as *mut _spAttachmentLoaderVtable)).disposeAttachment)
        .expect("non-null function pointer")(self_0, attachment);
}
#[no_mangle]
pub unsafe extern "C" fn _spAttachmentLoader_setError(
    mut self_0: *mut spAttachmentLoader,
    mut error1: *const c_char,
    mut error2: *const c_char,
) {
    _spFree((*self_0).error1 as *mut c_void);
    _spFree((*self_0).error2 as *mut c_void);
    let ref mut fresh54 = *(&mut (*self_0).error1 as *mut *const c_char as *mut *mut c_char);
    *fresh54 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul((spine_strlen(error1)).wrapping_add(1 as c_int as c_ulong)),
        b"spine.c\0" as *const u8 as *const c_char,
        4779 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh54, error1);
    let ref mut fresh55 = *(&mut (*self_0).error2 as *mut *const c_char as *mut *mut c_char);
    *fresh55 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul((spine_strlen(error2)).wrapping_add(1 as c_int as c_ulong)),
        b"spine.c\0" as *const u8 as *const c_char,
        4780 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh55, error2);
}
#[no_mangle]
pub unsafe extern "C" fn _spAttachmentLoader_setUnknownTypeError(
    mut self_0: *mut spAttachmentLoader,
    mut type_0: spAttachmentType,
) {
    let mut buffer: [c_char; 16] = [0; 16];
    spine_sprintf!(
        buffer.as_mut_ptr(),
        b"%d\0" as *const u8 as *const c_char,
        type_0 as c_uint,
    );
    _spAttachmentLoader_setError(
        self_0,
        b"Unknown attachment type: \0" as *const u8 as *const c_char,
        buffer.as_mut_ptr(),
    );
}
static mut yDown: c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn spBone_setYDown(mut value: c_int) {
    yDown = value;
}
#[no_mangle]
pub unsafe extern "C" fn spBone_isYDown() -> c_int {
    return yDown;
}
#[no_mangle]
pub unsafe extern "C" fn spBone_create(
    mut data: *mut spBoneData,
    mut skeleton: *mut spSkeleton,
    mut parent: *mut spBone,
) -> *mut spBone {
    let mut self_0: *mut spBone = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spBone>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        4832 as c_int,
    ) as *mut spBone;
    let ref mut fresh56 = *(&(*self_0).data as *const *mut spBoneData as *mut *mut spBoneData);
    *fresh56 = data;
    let ref mut fresh57 = *(&(*self_0).skeleton as *const *mut spSkeleton as *mut *mut spSkeleton);
    *fresh57 = skeleton;
    let ref mut fresh58 = *(&(*self_0).parent as *const *mut spBone as *mut *mut spBone);
    *fresh58 = parent;
    *(&(*self_0).a as *const c_float as *mut c_float) = 1.0f32;
    *(&(*self_0).d as *const c_float as *mut c_float) = 1.0f32;
    spBone_setToSetupPose(self_0);
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spBone_dispose(mut self_0: *mut spBone) {
    _spFree((*self_0).children as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spBone_update(mut self_0: *mut spBone) {
    spBone_updateWorldTransformWith(
        self_0,
        (*self_0).ax,
        (*self_0).ay,
        (*self_0).arotation,
        (*self_0).ascaleX,
        (*self_0).ascaleY,
        (*self_0).ashearX,
        (*self_0).ashearY,
    );
}
#[no_mangle]
pub unsafe extern "C" fn spBone_updateWorldTransform(mut self_0: *mut spBone) {
    spBone_updateWorldTransformWith(
        self_0,
        (*self_0).x,
        (*self_0).y,
        (*self_0).rotation,
        (*self_0).scaleX,
        (*self_0).scaleY,
        (*self_0).shearX,
        (*self_0).shearY,
    );
}
#[no_mangle]
pub unsafe extern "C" fn spBone_updateWorldTransformWith(
    mut self_0: *mut spBone,
    mut x: c_float,
    mut y: c_float,
    mut rotation: c_float,
    mut scaleX: c_float,
    mut scaleY: c_float,
    mut shearX: c_float,
    mut shearY: c_float,
) {
    let mut cosine: c_float = 0.;
    let mut sine: c_float = 0.;
    let mut pa: c_float = 0.;
    let mut pb: c_float = 0.;
    let mut pc: c_float = 0.;
    let mut pd: c_float = 0.;
    let mut parent: *mut spBone = (*self_0).parent;
    let mut sx: c_float = (*(*self_0).skeleton).scaleX;
    let mut sy: c_float = (*(*self_0).skeleton).scaleY
        * (if spBone_isYDown() != 0 {
            -(1 as c_int)
        } else {
            1 as c_int
        }) as c_float;
    (*self_0).ax = x;
    (*self_0).ay = y;
    (*self_0).arotation = rotation;
    (*self_0).ascaleX = scaleX;
    (*self_0).ascaleY = scaleY;
    (*self_0).ashearX = shearX;
    (*self_0).ashearY = shearY;
    if parent.is_null() {
        let mut rotationY: c_float = rotation + 90 as c_int as c_float + shearY;
        *(&(*self_0).a as *const c_float as *mut c_float) =
            cosf((rotation + shearX) * (3.1415926535897932385f32 / 180 as c_int as c_float))
                * scaleX
                * sx;
        *(&(*self_0).b as *const c_float as *mut c_float) =
            cosf(rotationY * (3.1415926535897932385f32 / 180 as c_int as c_float)) * scaleY * sx;
        *(&(*self_0).c as *const c_float as *mut c_float) =
            sinf((rotation + shearX) * (3.1415926535897932385f32 / 180 as c_int as c_float))
                * scaleX
                * sy;
        *(&(*self_0).d as *const c_float as *mut c_float) =
            sinf(rotationY * (3.1415926535897932385f32 / 180 as c_int as c_float)) * scaleY * sy;
        *(&(*self_0).worldX as *const c_float as *mut c_float) = x * sx + (*(*self_0).skeleton).x;
        *(&(*self_0).worldY as *const c_float as *mut c_float) = y * sy + (*(*self_0).skeleton).y;
        return;
    }
    pa = (*parent).a;
    pb = (*parent).b;
    pc = (*parent).c;
    pd = (*parent).d;
    *(&(*self_0).worldX as *const c_float as *mut c_float) = pa * x + pb * y + (*parent).worldX;
    *(&(*self_0).worldY as *const c_float as *mut c_float) = pc * x + pd * y + (*parent).worldY;
    match (*(*self_0).data).transformMode as c_uint {
        0 => {
            let mut rotationY_0: c_float = rotation + 90 as c_int as c_float + shearY;
            let mut la: c_float =
                cosf((rotation + shearX) * (3.1415926535897932385f32 / 180 as c_int as c_float))
                    * scaleX;
            let mut lb: c_float =
                cosf(rotationY_0 * (3.1415926535897932385f32 / 180 as c_int as c_float)) * scaleY;
            let mut lc: c_float =
                sinf((rotation + shearX) * (3.1415926535897932385f32 / 180 as c_int as c_float))
                    * scaleX;
            let mut ld: c_float =
                sinf(rotationY_0 * (3.1415926535897932385f32 / 180 as c_int as c_float)) * scaleY;
            *(&(*self_0).a as *const c_float as *mut c_float) = pa * la + pb * lc;
            *(&(*self_0).b as *const c_float as *mut c_float) = pa * lb + pb * ld;
            *(&(*self_0).c as *const c_float as *mut c_float) = pc * la + pd * lc;
            *(&(*self_0).d as *const c_float as *mut c_float) = pc * lb + pd * ld;
            return;
        }
        1 => {
            let mut rotationY_1: c_float = rotation + 90 as c_int as c_float + shearY;
            *(&(*self_0).a as *const c_float as *mut c_float) =
                cosf((rotation + shearX) * (3.1415926535897932385f32 / 180 as c_int as c_float))
                    * scaleX;
            *(&(*self_0).b as *const c_float as *mut c_float) =
                cosf(rotationY_1 * (3.1415926535897932385f32 / 180 as c_int as c_float)) * scaleY;
            *(&(*self_0).c as *const c_float as *mut c_float) =
                sinf((rotation + shearX) * (3.1415926535897932385f32 / 180 as c_int as c_float))
                    * scaleX;
            *(&(*self_0).d as *const c_float as *mut c_float) =
                sinf(rotationY_1 * (3.1415926535897932385f32 / 180 as c_int as c_float)) * scaleY;
        }
        2 => {
            let mut s: c_float = pa * pa + pc * pc;
            let mut prx: c_float = 0.;
            let mut rx: c_float = 0.;
            let mut ry: c_float = 0.;
            let mut la_0: c_float = 0.;
            let mut lb_0: c_float = 0.;
            let mut lc_0: c_float = 0.;
            let mut ld_0: c_float = 0.;
            if s > 0.0001f32 {
                s = (if pa * pd - pb * pc < 0 as c_int as c_float {
                    -(pa * pd - pb * pc)
                } else {
                    pa * pd - pb * pc
                }) / s;
                pa /= (*(*self_0).skeleton).scaleX;
                pc /= (*(*self_0).skeleton).scaleY;
                pb = pc * s;
                pd = pa * s;
                prx = atan2f(pc, pa) * (180 as c_int as c_float / 3.1415926535897932385f32);
            } else {
                pa = 0 as c_int as c_float;
                pc = 0 as c_int as c_float;
                prx = 90 as c_int as c_float
                    - atan2f(pd, pb) * (180 as c_int as c_float / 3.1415926535897932385f32);
            }
            rx = rotation + shearX - prx;
            ry = rotation + shearY - prx + 90 as c_int as c_float;
            la_0 = cosf(rx * (3.1415926535897932385f32 / 180 as c_int as c_float)) * scaleX;
            lb_0 = cosf(ry * (3.1415926535897932385f32 / 180 as c_int as c_float)) * scaleY;
            lc_0 = sinf(rx * (3.1415926535897932385f32 / 180 as c_int as c_float)) * scaleX;
            ld_0 = sinf(ry * (3.1415926535897932385f32 / 180 as c_int as c_float)) * scaleY;
            *(&(*self_0).a as *const c_float as *mut c_float) = pa * la_0 - pb * lc_0;
            *(&(*self_0).b as *const c_float as *mut c_float) = pa * lb_0 - pb * ld_0;
            *(&(*self_0).c as *const c_float as *mut c_float) = pc * la_0 + pd * lc_0;
            *(&(*self_0).d as *const c_float as *mut c_float) = pc * lb_0 + pd * ld_0;
        }
        3 | 4 => {
            let mut za: c_float = 0.;
            let mut zc: c_float = 0.;
            let mut s_0: c_float = 0.;
            let mut r: c_float = 0.;
            let mut zb: c_float = 0.;
            let mut zd: c_float = 0.;
            let mut la_1: c_float = 0.;
            let mut lb_1: c_float = 0.;
            let mut lc_1: c_float = 0.;
            let mut ld_1: c_float = 0.;
            cosine = cosf(rotation * (3.1415926535897932385f32 / 180 as c_int as c_float));
            sine = sinf(rotation * (3.1415926535897932385f32 / 180 as c_int as c_float));
            za = (pa * cosine + pb * sine) / sx;
            zc = (pc * cosine + pd * sine) / sy;
            s_0 = spine_sqrtf(za * za + zc * zc);
            if s_0 > 0.00001f32 {
                s_0 = 1 as c_int as c_float / s_0;
            }
            za *= s_0;
            zc *= s_0;
            s_0 = spine_sqrtf(za * za + zc * zc);
            if (*(*self_0).data).transformMode as c_uint
                == SP_TRANSFORMMODE_NOSCALE as c_int as c_uint
                && (pa * pd - pb * pc < 0 as c_int as c_float) as c_int
                    != ((sx < 0 as c_int as c_float) as c_int
                        != (sy < 0 as c_int as c_float) as c_int) as c_int
            {
                s_0 = -s_0;
            }
            r = 3.1415926535897932385f32 / 2 as c_int as c_float + atan2f(zc, za);
            zb = cosf(r) * s_0;
            zd = sinf(r) * s_0;
            la_1 = cosf(shearX * (3.1415926535897932385f32 / 180 as c_int as c_float)) * scaleX;
            lb_1 = cosf(
                (90 as c_int as c_float + shearY)
                    * (3.1415926535897932385f32 / 180 as c_int as c_float),
            ) * scaleY;
            lc_1 = sinf(shearX * (3.1415926535897932385f32 / 180 as c_int as c_float)) * scaleX;
            ld_1 = sinf(
                (90 as c_int as c_float + shearY)
                    * (3.1415926535897932385f32 / 180 as c_int as c_float),
            ) * scaleY;
            *(&(*self_0).a as *const c_float as *mut c_float) = za * la_1 + zb * lc_1;
            *(&(*self_0).b as *const c_float as *mut c_float) = za * lb_1 + zb * ld_1;
            *(&(*self_0).c as *const c_float as *mut c_float) = zc * la_1 + zd * lc_1;
            *(&(*self_0).d as *const c_float as *mut c_float) = zc * lb_1 + zd * ld_1;
        }
        _ => {}
    }
    *(&(*self_0).a as *const c_float as *mut c_float) *= sx;
    *(&(*self_0).b as *const c_float as *mut c_float) *= sx;
    *(&(*self_0).c as *const c_float as *mut c_float) *= sy;
    *(&(*self_0).d as *const c_float as *mut c_float) *= sy;
}
#[no_mangle]
pub unsafe extern "C" fn spBone_setToSetupPose(mut self_0: *mut spBone) {
    (*self_0).x = (*(*self_0).data).x;
    (*self_0).y = (*(*self_0).data).y;
    (*self_0).rotation = (*(*self_0).data).rotation;
    (*self_0).scaleX = (*(*self_0).data).scaleX;
    (*self_0).scaleY = (*(*self_0).data).scaleY;
    (*self_0).shearX = (*(*self_0).data).shearX;
    (*self_0).shearY = (*(*self_0).data).shearY;
}
#[no_mangle]
pub unsafe extern "C" fn spBone_getWorldRotationX(mut self_0: *mut spBone) -> c_float {
    return atan2f((*self_0).c, (*self_0).a) * (180 as c_int as c_float / 3.1415926535897932385f32);
}
#[no_mangle]
pub unsafe extern "C" fn spBone_getWorldRotationY(mut self_0: *mut spBone) -> c_float {
    return atan2f((*self_0).d, (*self_0).b) * (180 as c_int as c_float / 3.1415926535897932385f32);
}
#[no_mangle]
pub unsafe extern "C" fn spBone_getWorldScaleX(mut self_0: *mut spBone) -> c_float {
    return spine_sqrtf((*self_0).a * (*self_0).a + (*self_0).c * (*self_0).c);
}
#[no_mangle]
pub unsafe extern "C" fn spBone_getWorldScaleY(mut self_0: *mut spBone) -> c_float {
    return spine_sqrtf((*self_0).b * (*self_0).b + (*self_0).d * (*self_0).d);
}
#[no_mangle]
pub unsafe extern "C" fn spBone_updateAppliedTransform(mut self_0: *mut spBone) {
    let mut parent: *mut spBone = (*self_0).parent;
    if parent.is_null() {
        (*self_0).ax = (*self_0).worldX - (*(*self_0).skeleton).x;
        (*self_0).ay = (*self_0).worldY - (*(*self_0).skeleton).y;
        (*self_0).arotation =
            atan2f((*self_0).c, (*self_0).a) * (180 as c_int as c_float / 3.1415926535897932385f32);
        (*self_0).ascaleX = spine_sqrtf((*self_0).a * (*self_0).a + (*self_0).c * (*self_0).c);
        (*self_0).ascaleY = spine_sqrtf((*self_0).b * (*self_0).b + (*self_0).d * (*self_0).d);
        (*self_0).ashearX = 0 as c_int as c_float;
        (*self_0).ashearY = atan2f(
            (*self_0).a * (*self_0).b + (*self_0).c * (*self_0).d,
            (*self_0).a * (*self_0).d - (*self_0).b * (*self_0).c,
        ) * (180 as c_int as c_float / 3.1415926535897932385f32);
    } else {
        let mut pa: c_float = (*parent).a;
        let mut pb: c_float = (*parent).b;
        let mut pc: c_float = (*parent).c;
        let mut pd: c_float = (*parent).d;
        let mut pid: c_float = 1 as c_int as c_float / (pa * pd - pb * pc);
        let mut dx: c_float = (*self_0).worldX - (*parent).worldX;
        let mut dy: c_float = (*self_0).worldY - (*parent).worldY;
        let mut ia: c_float = pid * pd;
        let mut id: c_float = pid * pa;
        let mut ib: c_float = pid * pb;
        let mut ic: c_float = pid * pc;
        let mut ra: c_float = ia * (*self_0).a - ib * (*self_0).c;
        let mut rb: c_float = ia * (*self_0).b - ib * (*self_0).d;
        let mut rc: c_float = id * (*self_0).c - ic * (*self_0).a;
        let mut rd: c_float = id * (*self_0).d - ic * (*self_0).b;
        (*self_0).ax = dx * pd * pid - dy * pb * pid;
        (*self_0).ay = dy * pa * pid - dx * pc * pid;
        (*self_0).ashearX = 0 as c_int as c_float;
        (*self_0).ascaleX = spine_sqrtf(ra * ra + rc * rc);
        if (*self_0).ascaleX > 0.0001f32 {
            let mut det: c_float = ra * rd - rb * rc;
            (*self_0).ascaleY = det / (*self_0).ascaleX;
            (*self_0).ashearY = atan2f(ra * rb + rc * rd, det)
                * (180 as c_int as c_float / 3.1415926535897932385f32);
            (*self_0).arotation =
                atan2f(rc, ra) * (180 as c_int as c_float / 3.1415926535897932385f32);
        } else {
            (*self_0).ascaleX = 0 as c_int as c_float;
            (*self_0).ascaleY = spine_sqrtf(rb * rb + rd * rd);
            (*self_0).ashearY = 0 as c_int as c_float;
            (*self_0).arotation = 90 as c_int as c_float
                - atan2f(rd, rb) * (180 as c_int as c_float / 3.1415926535897932385f32);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn spBone_worldToLocal(
    mut self_0: *mut spBone,
    mut worldX: c_float,
    mut worldY: c_float,
    mut localX: *mut c_float,
    mut localY: *mut c_float,
) {
    let mut invDet: c_float =
        1 as c_int as c_float / ((*self_0).a * (*self_0).d - (*self_0).b * (*self_0).c);
    let mut x: c_float = worldX - (*self_0).worldX;
    let mut y: c_float = worldY - (*self_0).worldY;
    *localX = x * (*self_0).d * invDet - y * (*self_0).b * invDet;
    *localY = y * (*self_0).a * invDet - x * (*self_0).c * invDet;
}
#[no_mangle]
pub unsafe extern "C" fn spBone_localToWorld(
    mut self_0: *mut spBone,
    mut localX: c_float,
    mut localY: c_float,
    mut worldX: *mut c_float,
    mut worldY: *mut c_float,
) {
    let mut x: c_float = localX;
    let mut y: c_float = localY;
    *worldX = x * (*self_0).a + y * (*self_0).b + (*self_0).worldX;
    *worldY = x * (*self_0).c + y * (*self_0).d + (*self_0).worldY;
}
#[no_mangle]
pub unsafe extern "C" fn spBone_worldToLocalRotation(
    mut self_0: *mut spBone,
    mut worldRotation: c_float,
) -> c_float {
    let mut sine: c_float = 0.;
    let mut cosine: c_float = 0.;
    sine = sinf(worldRotation * (3.1415926535897932385f32 / 180 as c_int as c_float));
    cosine = cosf(worldRotation * (3.1415926535897932385f32 / 180 as c_int as c_float));
    return atan2f(
        (*self_0).a * sine - (*self_0).c * cosine,
        (*self_0).d * cosine - (*self_0).b * sine,
    ) * (180 as c_int as c_float / 3.1415926535897932385f32)
        + (*self_0).rotation
        - (*self_0).shearX;
}
#[no_mangle]
pub unsafe extern "C" fn spBone_localToWorldRotation(
    mut self_0: *mut spBone,
    mut localRotation: c_float,
) -> c_float {
    let mut sine: c_float = 0.;
    let mut cosine: c_float = 0.;
    localRotation -= (*self_0).rotation - (*self_0).shearX;
    sine = sinf(localRotation * (3.1415926535897932385f32 / 180 as c_int as c_float));
    cosine = cosf(localRotation * (3.1415926535897932385f32 / 180 as c_int as c_float));
    return atan2f(
        cosine * (*self_0).c + sine * (*self_0).d,
        cosine * (*self_0).a + sine * (*self_0).b,
    ) * (180 as c_int as c_float / 3.1415926535897932385f32);
}
#[no_mangle]
pub unsafe extern "C" fn spBone_rotateWorld(mut self_0: *mut spBone, mut degrees: c_float) {
    let mut a: c_float = (*self_0).a;
    let mut b: c_float = (*self_0).b;
    let mut c: c_float = (*self_0).c;
    let mut d: c_float = (*self_0).d;
    let mut cosine: c_float = cosf(degrees * (3.1415926535897932385f32 / 180 as c_int as c_float));
    let mut sine: c_float = sinf(degrees * (3.1415926535897932385f32 / 180 as c_int as c_float));
    *(&(*self_0).a as *const c_float as *mut c_float) = cosine * a - sine * c;
    *(&(*self_0).b as *const c_float as *mut c_float) = cosine * b - sine * d;
    *(&(*self_0).c as *const c_float as *mut c_float) = sine * a + cosine * c;
    *(&(*self_0).d as *const c_float as *mut c_float) = sine * b + cosine * d;
}
#[no_mangle]
pub unsafe extern "C" fn spBoneData_create(
    mut index: c_int,
    mut name: *const c_char,
    mut parent: *mut spBoneData,
) -> *mut spBoneData {
    let mut self_0: *mut spBoneData = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spBoneData>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        5115 as c_int,
    ) as *mut spBoneData;
    *(&(*self_0).index as *const c_int as *mut c_int) = index;
    let ref mut fresh59 = *(&(*self_0).name as *const *const c_char as *mut *mut c_char);
    *fresh59 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul((spine_strlen(name)).wrapping_add(1 as c_int as c_ulong)),
        b"spine.c\0" as *const u8 as *const c_char,
        5117 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh59, name);
    let ref mut fresh60 = *(&(*self_0).parent as *const *mut spBoneData as *mut *mut spBoneData);
    *fresh60 = parent;
    (*self_0).scaleX = 1 as c_int as c_float;
    (*self_0).scaleY = 1 as c_int as c_float;
    (*self_0).transformMode = SP_TRANSFORMMODE_NORMAL;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spBoneData_dispose(mut self_0: *mut spBoneData) {
    _spFree((*self_0).name as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _spBoundingBoxAttachment_dispose(mut attachment: *mut spAttachment) {
    let mut self_0: *mut spBoundingBoxAttachment = attachment as *mut spBoundingBoxAttachment;
    _spVertexAttachment_deinit(&mut (*self_0).super_0);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _spBoundingBoxAttachment_copy(
    mut attachment: *mut spAttachment,
) -> *mut spAttachment {
    let mut copy: *mut spBoundingBoxAttachment = spBoundingBoxAttachment_create((*attachment).name);
    let mut self_0: *mut spBoundingBoxAttachment = attachment as *mut spBoundingBoxAttachment;
    spVertexAttachment_copyTo(&mut (*self_0).super_0, &mut (*copy).super_0);
    return &mut (*copy).super_0.super_0;
}
#[no_mangle]
pub unsafe extern "C" fn spBoundingBoxAttachment_create(
    mut name: *const c_char,
) -> *mut spBoundingBoxAttachment {
    let mut self_0: *mut spBoundingBoxAttachment = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spBoundingBoxAttachment>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        5177 as c_int,
    ) as *mut spBoundingBoxAttachment;
    _spVertexAttachment_init(&mut (*self_0).super_0);
    _spAttachment_init(
        &mut (*self_0).super_0.super_0,
        name,
        SP_ATTACHMENT_BOUNDING_BOX,
        Some(_spBoundingBoxAttachment_dispose as unsafe extern "C" fn(*mut spAttachment) -> ()),
        Some(
            _spBoundingBoxAttachment_copy
                as unsafe extern "C" fn(*mut spAttachment) -> *mut spAttachment,
        ),
    );
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn _spClippingAttachment_dispose(mut attachment: *mut spAttachment) {
    let mut self_0: *mut spClippingAttachment = attachment as *mut spClippingAttachment;
    _spVertexAttachment_deinit(&mut (*self_0).super_0);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _spClippingAttachment_copy(
    mut attachment: *mut spAttachment,
) -> *mut spAttachment {
    let mut copy: *mut spClippingAttachment = spClippingAttachment_create((*attachment).name);
    let mut self_0: *mut spClippingAttachment = attachment as *mut spClippingAttachment;
    spVertexAttachment_copyTo(&mut (*self_0).super_0, &mut (*copy).super_0);
    (*copy).endSlot = (*self_0).endSlot;
    return &mut (*copy).super_0.super_0;
}
#[no_mangle]
pub unsafe extern "C" fn spClippingAttachment_create(
    mut name: *const c_char,
) -> *mut spClippingAttachment {
    let mut self_0: *mut spClippingAttachment = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spClippingAttachment>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        5232 as c_int,
    ) as *mut spClippingAttachment;
    _spVertexAttachment_init(&mut (*self_0).super_0);
    _spAttachment_init(
        &mut (*self_0).super_0.super_0,
        name,
        SP_ATTACHMENT_CLIPPING,
        Some(_spClippingAttachment_dispose as unsafe extern "C" fn(*mut spAttachment) -> ()),
        Some(
            _spClippingAttachment_copy
                as unsafe extern "C" fn(*mut spAttachment) -> *mut spAttachment,
        ),
    );
    (*self_0).endSlot = 0 as *mut spSlotData;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spColor_create() -> *mut spColor {
    return _spMalloc(
        (::core::mem::size_of::<spColor>() as c_ulong).wrapping_mul(1 as c_int as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        5272 as c_int,
    ) as *mut spColor;
}
#[no_mangle]
pub unsafe extern "C" fn spColor_dispose(mut self_0: *mut spColor) {
    if !self_0.is_null() {
        _spFree(self_0 as *mut c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn spColor_setFromFloats(
    mut self_0: *mut spColor,
    mut r: c_float,
    mut g: c_float,
    mut b: c_float,
    mut a: c_float,
) {
    (*self_0).r = r;
    (*self_0).g = g;
    (*self_0).b = b;
    (*self_0).a = a;
    spColor_clamp(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn spColor_setFromFloats3(
    mut self_0: *mut spColor,
    mut r: c_float,
    mut g: c_float,
    mut b: c_float,
) {
    (*self_0).r = r;
    (*self_0).g = g;
    (*self_0).b = b;
    spColor_clamp(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn spColor_setFromColor(
    mut self_0: *mut spColor,
    mut otherColor: *mut spColor,
) {
    (*self_0).r = (*otherColor).r;
    (*self_0).g = (*otherColor).g;
    (*self_0).b = (*otherColor).b;
    (*self_0).a = (*otherColor).a;
}
#[no_mangle]
pub unsafe extern "C" fn spColor_setFromColor3(
    mut self_0: *mut spColor,
    mut otherColor: *mut spColor,
) {
    (*self_0).r = (*otherColor).r;
    (*self_0).g = (*otherColor).g;
    (*self_0).b = (*otherColor).b;
}
#[no_mangle]
pub unsafe extern "C" fn spColor_addColor(mut self_0: *mut spColor, mut otherColor: *mut spColor) {
    (*self_0).r += (*otherColor).r;
    (*self_0).g += (*otherColor).g;
    (*self_0).b += (*otherColor).b;
    (*self_0).a += (*otherColor).a;
    spColor_clamp(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn spColor_addFloats(
    mut self_0: *mut spColor,
    mut r: c_float,
    mut g: c_float,
    mut b: c_float,
    mut a: c_float,
) {
    (*self_0).r += r;
    (*self_0).g += g;
    (*self_0).b += b;
    (*self_0).a += a;
    spColor_clamp(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn spColor_addFloats3(
    mut self_0: *mut spColor,
    mut r: c_float,
    mut g: c_float,
    mut b: c_float,
) {
    (*self_0).r += r;
    (*self_0).g += g;
    (*self_0).b += b;
    spColor_clamp(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn spColor_clamp(mut self_0: *mut spColor) {
    if (*self_0).r < 0 as c_int as c_float {
        (*self_0).r = 0 as c_int as c_float;
    } else if (*self_0).r > 1 as c_int as c_float {
        (*self_0).r = 1 as c_int as c_float;
    }
    if (*self_0).g < 0 as c_int as c_float {
        (*self_0).g = 0 as c_int as c_float;
    } else if (*self_0).g > 1 as c_int as c_float {
        (*self_0).g = 1 as c_int as c_float;
    }
    if (*self_0).b < 0 as c_int as c_float {
        (*self_0).b = 0 as c_int as c_float;
    } else if (*self_0).b > 1 as c_int as c_float {
        (*self_0).b = 1 as c_int as c_float;
    }
    if (*self_0).a < 0 as c_int as c_float {
        (*self_0).a = 0 as c_int as c_float;
    } else if (*self_0).a > 1 as c_int as c_float {
        (*self_0).a = 1 as c_int as c_float;
    }
}
static mut _spTimelineTypeNames: [*const c_char; 24] = [
    b"Attachment\0" as *const u8 as *const c_char,
    b"Alpha\0" as *const u8 as *const c_char,
    b"PathConstraintPosition\0" as *const u8 as *const c_char,
    b"PathConstraintSpace\0" as *const u8 as *const c_char,
    b"Rotate\0" as *const u8 as *const c_char,
    b"ScaleX\0" as *const u8 as *const c_char,
    b"ScaleY\0" as *const u8 as *const c_char,
    b"ShearX\0" as *const u8 as *const c_char,
    b"ShearY\0" as *const u8 as *const c_char,
    b"TranslateX\0" as *const u8 as *const c_char,
    b"TranslateY\0" as *const u8 as *const c_char,
    b"Scale\0" as *const u8 as *const c_char,
    b"Shear\0" as *const u8 as *const c_char,
    b"Translate\0" as *const u8 as *const c_char,
    b"Deform\0" as *const u8 as *const c_char,
    b"IkConstraint\0" as *const u8 as *const c_char,
    b"PathConstraintMix\0" as *const u8 as *const c_char,
    b"Rgb2\0" as *const u8 as *const c_char,
    b"Rgba2\0" as *const u8 as *const c_char,
    b"Rgba\0" as *const u8 as *const c_char,
    b"Rgb\0" as *const u8 as *const c_char,
    b"TransformConstraint\0" as *const u8 as *const c_char,
    b"DrawOrder\0" as *const u8 as *const c_char,
    b"Event\0" as *const u8 as *const c_char,
];
#[no_mangle]
pub unsafe extern "C" fn spDebug_printSkeletonData(mut skeletonData: *mut spSkeletonData) {
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    spDebug_printBoneDatas((*skeletonData).bones, (*skeletonData).bonesCount);
    i = 0 as c_int;
    n = (*skeletonData).animationsCount;
    while i < n {
        spDebug_printAnimation(*((*skeletonData).animations).offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _spDebug_printTimelineBase(mut timeline: *mut spTimeline) {
    spine_printf!(
        b"   Timeline %s:\n\0" as *const u8 as *const c_char,
        _spTimelineTypeNames[(*timeline).type_0 as usize],
    );
    spine_printf!(
        b"      frame count: %i\n\0" as *const u8 as *const c_char,
        (*timeline).frameCount,
    );
    spine_printf!(
        b"      frame entries: %i\n\0" as *const u8 as *const c_char,
        (*timeline).frameEntries,
    );
    spine_printf!(b"      frames: \0" as *const u8 as *const c_char);
    spDebug_printFloats((*(*timeline).frames).items, (*(*timeline).frames).size);
    spine_printf!(b"\n\0" as *const u8 as *const c_char);
}
#[no_mangle]
pub unsafe extern "C" fn _spDebug_printCurveTimeline(mut timeline: *mut spCurveTimeline) {
    _spDebug_printTimelineBase(&mut (*timeline).super_0);
    spine_printf!(b"      curves: \0" as *const u8 as *const c_char);
    spDebug_printFloats((*(*timeline).curves).items, (*(*timeline).curves).size);
    spine_printf!(b"\n\0" as *const u8 as *const c_char);
}
#[no_mangle]
pub unsafe extern "C" fn spDebug_printTimeline(mut timeline: *mut spTimeline) {
    match (*timeline).type_0 as c_uint {
        0 => {
            let mut t: *mut spAttachmentTimeline = timeline as *mut spAttachmentTimeline;
            _spDebug_printTimelineBase(&mut (*t).super_0);
        }
        1 => {
            let mut t_0: *mut spAlphaTimeline = timeline as *mut spAlphaTimeline;
            _spDebug_printCurveTimeline(&mut (*t_0).super_0);
        }
        2 => {
            let mut t_1: *mut spPathConstraintPositionTimeline =
                timeline as *mut spPathConstraintPositionTimeline;
            _spDebug_printCurveTimeline(&mut (*t_1).super_0);
        }
        3 => {
            let mut t_2: *mut spPathConstraintMixTimeline =
                timeline as *mut spPathConstraintMixTimeline;
            _spDebug_printCurveTimeline(&mut (*t_2).super_0);
        }
        4 => {
            let mut t_3: *mut spRotateTimeline = timeline as *mut spRotateTimeline;
            _spDebug_printCurveTimeline(&mut (*t_3).super_0);
        }
        5 => {
            let mut t_4: *mut spScaleXTimeline = timeline as *mut spScaleXTimeline;
            _spDebug_printCurveTimeline(&mut (*t_4).super_0);
        }
        6 => {
            let mut t_5: *mut spScaleYTimeline = timeline as *mut spScaleYTimeline;
            _spDebug_printCurveTimeline(&mut (*t_5).super_0);
        }
        7 => {
            let mut t_6: *mut spShearXTimeline = timeline as *mut spShearXTimeline;
            _spDebug_printCurveTimeline(&mut (*t_6).super_0);
        }
        8 => {
            let mut t_7: *mut spShearYTimeline = timeline as *mut spShearYTimeline;
            _spDebug_printCurveTimeline(&mut (*t_7).super_0);
        }
        9 => {
            let mut t_8: *mut spTranslateXTimeline = timeline as *mut spTranslateXTimeline;
            _spDebug_printCurveTimeline(&mut (*t_8).super_0);
        }
        10 => {
            let mut t_9: *mut spTranslateYTimeline = timeline as *mut spTranslateYTimeline;
            _spDebug_printCurveTimeline(&mut (*t_9).super_0);
        }
        11 => {
            let mut t_10: *mut spScaleTimeline = timeline as *mut spScaleTimeline;
            _spDebug_printCurveTimeline(&mut (*t_10).super_0);
        }
        12 => {
            let mut t_11: *mut spShearTimeline = timeline as *mut spShearTimeline;
            _spDebug_printCurveTimeline(&mut (*t_11).super_0);
        }
        13 => {
            let mut t_12: *mut spTranslateTimeline = timeline as *mut spTranslateTimeline;
            _spDebug_printCurveTimeline(&mut (*t_12).super_0);
        }
        14 => {
            let mut t_13: *mut spDeformTimeline = timeline as *mut spDeformTimeline;
            _spDebug_printCurveTimeline(&mut (*t_13).super_0);
        }
        16 => {
            let mut t_14: *mut spIkConstraintTimeline = timeline as *mut spIkConstraintTimeline;
            _spDebug_printCurveTimeline(&mut (*t_14).super_0);
        }
        17 => {
            let mut t_15: *mut spPathConstraintMixTimeline =
                timeline as *mut spPathConstraintMixTimeline;
            _spDebug_printCurveTimeline(&mut (*t_15).super_0);
        }
        18 => {
            let mut t_16: *mut spRGB2Timeline = timeline as *mut spRGB2Timeline;
            _spDebug_printCurveTimeline(&mut (*t_16).super_0);
        }
        19 => {
            let mut t_17: *mut spRGBA2Timeline = timeline as *mut spRGBA2Timeline;
            _spDebug_printCurveTimeline(&mut (*t_17).super_0);
        }
        20 => {
            let mut t_18: *mut spRGBATimeline = timeline as *mut spRGBATimeline;
            _spDebug_printCurveTimeline(&mut (*t_18).super_0);
        }
        21 => {
            let mut t_19: *mut spRGBTimeline = timeline as *mut spRGBTimeline;
            _spDebug_printCurveTimeline(&mut (*t_19).super_0);
        }
        22 => {
            let mut t_20: *mut spTransformConstraintTimeline =
                timeline as *mut spTransformConstraintTimeline;
            _spDebug_printCurveTimeline(&mut (*t_20).super_0);
        }
        23 => {
            let mut t_21: *mut spDrawOrderTimeline = timeline as *mut spDrawOrderTimeline;
            _spDebug_printTimelineBase(&mut (*t_21).super_0);
        }
        24 => {
            let mut t_22: *mut spEventTimeline = timeline as *mut spEventTimeline;
            _spDebug_printTimelineBase(&mut (*t_22).super_0);
        }
        15 => {
            let mut t_23: *mut spSequenceTimeline = timeline as *mut spSequenceTimeline;
            _spDebug_printTimelineBase(&mut (*t_23).super_0);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn spDebug_printAnimation(mut animation: *mut spAnimation) {
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    spine_printf!(
        b"Animation %s: %i timelines\n\0" as *const u8 as *const c_char,
        (*animation).name,
        (*(*animation).timelines).size,
    );
    i = 0 as c_int;
    n = (*(*animation).timelines).size;
    while i < n {
        spDebug_printTimeline(*((*(*animation).timelines).items).offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spDebug_printBoneDatas(
    mut boneDatas: *mut *mut spBoneData,
    mut numBoneDatas: c_int,
) {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < numBoneDatas {
        spDebug_printBoneData(*boneDatas.offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spDebug_printBoneData(mut boneData: *mut spBoneData) {
    spine_printf!(
        b"Bone data %s: %f, %f, %f, %f, %f, %f %f\n\0" as *const u8 as *const c_char,
        (*boneData).name,
        (*boneData).rotation as c_double,
        (*boneData).scaleX as c_double,
        (*boneData).scaleY as c_double,
        (*boneData).x as c_double,
        (*boneData).y as c_double,
        (*boneData).shearX as c_double,
        (*boneData).shearY as c_double,
    );
}
#[no_mangle]
pub unsafe extern "C" fn spDebug_printSkeleton(mut skeleton: *mut spSkeleton) {
    spDebug_printBones((*skeleton).bones, (*skeleton).bonesCount);
}
#[no_mangle]
pub unsafe extern "C" fn spDebug_printBones(mut bones: *mut *mut spBone, mut numBones: c_int) {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < numBones {
        spDebug_printBone(*bones.offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spDebug_printBone(mut bone: *mut spBone) {
    spine_printf!(
        b"Bone %s: %f, %f, %f, %f, %f, %f\n\0" as *const u8 as *const c_char,
        (*(*bone).data).name,
        (*bone).a as c_double,
        (*bone).b as c_double,
        (*bone).c as c_double,
        (*bone).d as c_double,
        (*bone).worldX as c_double,
        (*bone).worldY as c_double,
    );
}
#[no_mangle]
pub unsafe extern "C" fn spDebug_printFloats(mut values: *mut c_float, mut numFloats: c_int) {
    let mut i: c_int = 0;
    spine_printf!(b"(%i) [\0" as *const u8 as *const c_char, numFloats);
    i = 0 as c_int;
    while i < numFloats {
        spine_printf!(
            b"%f, \0" as *const u8 as *const c_char,
            *values.offset(i as isize) as c_double,
        );
        i += 1;
    }
    spine_printf!(b"]\0" as *const u8 as *const c_char);
}
#[no_mangle]
pub unsafe extern "C" fn spEvent_create(
    mut time: c_float,
    mut data: *mut spEventData,
) -> *mut spEvent {
    let mut self_0: *mut spEvent = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spEvent>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        5639 as c_int,
    ) as *mut spEvent;
    let ref mut fresh61 = *(&(*self_0).data as *const *mut spEventData as *mut *mut spEventData);
    *fresh61 = data;
    *(&(*self_0).time as *const c_float as *mut c_float) = time;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spEvent_dispose(mut self_0: *mut spEvent) {
    _spFree((*self_0).stringValue as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spEventData_create(mut name: *const c_char) -> *mut spEventData {
    let mut self_0: *mut spEventData = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spEventData>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        5682 as c_int,
    ) as *mut spEventData;
    let ref mut fresh62 = *(&(*self_0).name as *const *const c_char as *mut *mut c_char);
    *fresh62 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul((spine_strlen(name)).wrapping_add(1 as c_int as c_ulong)),
        b"spine.c\0" as *const u8 as *const c_char,
        5683 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh62, name);
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spEventData_dispose(mut self_0: *mut spEventData) {
    _spFree((*self_0).audioPath as *mut c_void);
    _spFree((*self_0).stringValue as *mut c_void);
    _spFree((*self_0).name as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spIkConstraint_create(
    mut data: *mut spIkConstraintData,
    mut skeleton: *const spSkeleton,
) -> *mut spIkConstraint {
    let mut i: c_int = 0;
    let mut self_0: *mut spIkConstraint = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spIkConstraint>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        5730 as c_int,
    ) as *mut spIkConstraint;
    let ref mut fresh63 =
        *(&(*self_0).data as *const *mut spIkConstraintData as *mut *mut spIkConstraintData);
    *fresh63 = data;
    (*self_0).bendDirection = (*data).bendDirection;
    (*self_0).compress = (*data).compress;
    (*self_0).stretch = (*data).stretch;
    (*self_0).mix = (*data).mix;
    (*self_0).softness = (*data).softness;
    (*self_0).bonesCount = (*(*self_0).data).bonesCount;
    (*self_0).bones = _spMalloc(
        (::core::mem::size_of::<*mut spBone>() as c_ulong)
            .wrapping_mul((*self_0).bonesCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        5739 as c_int,
    ) as *mut *mut spBone;
    i = 0 as c_int;
    while i < (*self_0).bonesCount {
        let ref mut fresh64 = *((*self_0).bones).offset(i as isize);
        *fresh64 = spSkeleton_findBone(
            skeleton,
            (**((*(*self_0).data).bones).offset(i as isize)).name,
        );
        i += 1;
    }
    (*self_0).target = spSkeleton_findBone(skeleton, (*(*(*self_0).data).target).name);
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spIkConstraint_dispose(mut self_0: *mut spIkConstraint) {
    _spFree((*self_0).bones as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spIkConstraint_update(mut self_0: *mut spIkConstraint) {
    if (*self_0).mix == 0 as c_int as c_float {
        return;
    }
    match (*self_0).bonesCount {
        1 => {
            spIkConstraint_apply1(
                *((*self_0).bones).offset(0 as c_int as isize),
                (*(*self_0).target).worldX,
                (*(*self_0).target).worldY,
                (*self_0).compress,
                (*self_0).stretch,
                (*(*self_0).data).uniform,
                (*self_0).mix,
            );
        }
        2 => {
            spIkConstraint_apply2(
                *((*self_0).bones).offset(0 as c_int as isize),
                *((*self_0).bones).offset(1 as c_int as isize),
                (*(*self_0).target).worldX,
                (*(*self_0).target).worldY,
                (*self_0).bendDirection,
                (*self_0).stretch,
                (*(*self_0).data).uniform,
                (*self_0).softness,
                (*self_0).mix,
            );
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn spIkConstraint_apply1(
    mut bone: *mut spBone,
    mut targetX: c_float,
    mut targetY: c_float,
    mut compress: c_int,
    mut stretch: c_int,
    mut uniform: c_int,
    mut alpha: c_float,
) {
    let mut p: *mut spBone = (*bone).parent;
    let mut pa: c_float = (*p).a;
    let mut pb: c_float = (*p).b;
    let mut pc: c_float = (*p).c;
    let mut pd: c_float = (*p).d;
    let mut rotationIK: c_float = -(*bone).ashearX - (*bone).arotation;
    let mut tx: c_float = 0 as c_int as c_float;
    let mut ty: c_float = 0 as c_int as c_float;
    let mut sx: c_float = 0 as c_int as c_float;
    let mut sy: c_float = 0 as c_int as c_float;
    let mut s: c_float = 0 as c_int as c_float;
    let mut sa: c_float = 0 as c_int as c_float;
    let mut sc: c_float = 0 as c_int as c_float;
    let mut current_block_11: u64;
    match (*(*bone).data).transformMode as c_uint {
        1 => {
            tx = targetX - (*bone).worldX;
            ty = targetY - (*bone).worldY;
            current_block_11 = 1856101646708284338;
        }
        2 => {
            s = (if pa * pd - pb * pc < 0 as c_int as c_float {
                -(pa * pd - pb * pc)
            } else {
                pa * pd - pb * pc
            }) / (pa * pa + pc * pc);
            sa = pa / (*(*bone).skeleton).scaleX;
            sc = pc / (*(*bone).skeleton).scaleY;
            pb = -sc * s * (*(*bone).skeleton).scaleX;
            pd = sa * s * (*(*bone).skeleton).scaleY;
            rotationIK += atan2f(sc, sa) * (180 as c_int as c_float / 3.1415926535897932385f32);
            current_block_11 = 16979364676087028331;
        }
        _ => {
            current_block_11 = 16979364676087028331;
        }
    }
    match current_block_11 {
        16979364676087028331 => {
            let mut x: c_float = targetX - (*p).worldX;
            let mut y: c_float = targetY - (*p).worldY;
            let mut d: c_float = pa * pd - pb * pc;
            tx = (x * pd - y * pb) / d - (*bone).ax;
            ty = (y * pa - x * pc) / d - (*bone).ay;
        }
        _ => {}
    }
    rotationIK += atan2f(ty, tx) * (180 as c_int as c_float / 3.1415926535897932385f32);
    if (*bone).ascaleX < 0 as c_int as c_float {
        rotationIK += 180 as c_int as c_float;
    }
    if rotationIK > 180 as c_int as c_float {
        rotationIK -= 360 as c_int as c_float;
    } else if rotationIK < -(180 as c_int) as c_float {
        rotationIK += 360 as c_int as c_float;
    }
    sx = (*bone).ascaleX;
    sy = (*bone).ascaleY;
    if compress != 0 || stretch != 0 {
        let mut b: c_float = 0.;
        let mut dd: c_float = 0.;
        match (*(*bone).data).transformMode as c_uint {
            3 | 4 => {
                tx = targetX - (*bone).worldX;
                ty = targetY - (*bone).worldY;
            }
            _ => {}
        }
        b = (*(*bone).data).length * sx;
        dd = spine_sqrtf(tx * tx + ty * ty);
        if compress != 0 && dd < b || stretch != 0 && dd > b && b > 0.0001f32 {
            s = (dd / b - 1 as c_int as c_float) * alpha + 1 as c_int as c_float;
            sx *= s;
            if uniform != 0 {
                sy *= s;
            }
        }
    }
    spBone_updateWorldTransformWith(
        bone,
        (*bone).ax,
        (*bone).ay,
        (*bone).arotation + rotationIK * alpha,
        sx,
        sy,
        (*bone).ashearX,
        (*bone).ashearY,
    );
}
#[no_mangle]
pub unsafe extern "C" fn spIkConstraint_apply2(
    mut parent: *mut spBone,
    mut child: *mut spBone,
    mut targetX: c_float,
    mut targetY: c_float,
    mut bendDir: c_int,
    mut stretch: c_int,
    mut uniform: c_int,
    mut softness: c_float,
    mut alpha: c_float,
) {
    let mut current_block: u64;
    let mut a: c_float = 0.;
    let mut b: c_float = 0.;
    let mut c: c_float = 0.;
    let mut d: c_float = 0.;
    let mut px: c_float = 0.;
    let mut py: c_float = 0.;
    let mut psx: c_float = 0.;
    let mut psy: c_float = 0.;
    let mut sx: c_float = 0.;
    let mut sy: c_float = 0.;
    let mut cx: c_float = 0.;
    let mut cy: c_float = 0.;
    let mut csx: c_float = 0.;
    let mut cwx: c_float = 0.;
    let mut cwy: c_float = 0.;
    let mut o1: c_int = 0;
    let mut o2: c_int = 0;
    let mut s2: c_int = 0;
    let mut u: c_int = 0;
    let mut pp: *mut spBone = (*parent).parent;
    let mut tx: c_float = 0.;
    let mut ty: c_float = 0.;
    let mut dd: c_float = 0.;
    let mut dx: c_float = 0.;
    let mut dy: c_float = 0.;
    let mut l1: c_float = 0.;
    let mut l2: c_float = 0.;
    let mut a1: c_float = 0.;
    let mut a2: c_float = 0.;
    let mut r: c_float = 0.;
    let mut td: c_float = 0.;
    let mut sd: c_float = 0.;
    let mut p: c_float = 0.;
    let mut id: c_float = 0.;
    let mut x: c_float = 0.;
    let mut y: c_float = 0.;
    let mut aa: c_float = 0.;
    let mut bb: c_float = 0.;
    let mut ll: c_float = 0.;
    let mut ta: c_float = 0.;
    let mut c0: c_float = 0.;
    let mut c1: c_float = 0.;
    let mut c2: c_float = 0.;
    px = (*parent).ax;
    py = (*parent).ay;
    psx = (*parent).ascaleX;
    psy = (*parent).ascaleY;
    sx = psx;
    sy = psy;
    csx = (*child).ascaleX;
    if psx < 0 as c_int as c_float {
        psx = -psx;
        o1 = 180 as c_int;
        s2 = -(1 as c_int);
    } else {
        o1 = 0 as c_int;
        s2 = 1 as c_int;
    }
    if psy < 0 as c_int as c_float {
        psy = -psy;
        s2 = -s2;
    }
    if csx < 0 as c_int as c_float {
        csx = -csx;
        o2 = 180 as c_int;
    } else {
        o2 = 0 as c_int;
    }
    r = psx - psy;
    cx = (*child).ax;
    u = ((if r < 0 as c_int as c_float { -r } else { r }) <= 0.0001f32) as c_int;
    if u == 0 || stretch != 0 {
        cy = 0 as c_int as c_float;
        cwx = (*parent).a * cx + (*parent).worldX;
        cwy = (*parent).c * cx + (*parent).worldY;
    } else {
        cy = (*child).ay;
        cwx = (*parent).a * cx + (*parent).b * cy + (*parent).worldX;
        cwy = (*parent).c * cx + (*parent).d * cy + (*parent).worldY;
    }
    a = (*pp).a;
    b = (*pp).b;
    c = (*pp).c;
    d = (*pp).d;
    id = 1 as c_int as c_float / (a * d - b * c);
    x = cwx - (*pp).worldX;
    y = cwy - (*pp).worldY;
    dx = (x * d - y * b) * id - px;
    dy = (y * a - x * c) * id - py;
    l1 = spine_sqrtf(dx * dx + dy * dy);
    l2 = (*(*child).data).length * csx;
    if (l1 as c_double) < 0.0001f64 {
        spIkConstraint_apply1(
            parent, targetX, targetY, 0 as c_int, stretch, 0 as c_int, alpha,
        );
        spBone_updateWorldTransformWith(
            child,
            cx,
            cy,
            0 as c_int as c_float,
            (*child).ascaleX,
            (*child).ascaleY,
            (*child).ashearX,
            (*child).ashearY,
        );
        return;
    }
    x = targetX - (*pp).worldX;
    y = targetY - (*pp).worldY;
    tx = (x * d - y * b) * id - px;
    ty = (y * a - x * c) * id - py;
    dd = tx * tx + ty * ty;
    if softness != 0 as c_int as c_float {
        softness *= psx * (csx + 1 as c_int as c_float) * 0.5f32;
        td = spine_sqrtf(dd);
        sd = td - l1 - l2 * psx + softness;
        if sd > 0 as c_int as c_float {
            p = (if (1 as c_int as c_float) < sd / (softness * 2 as c_int as c_float) {
                1 as c_int as c_float
            } else {
                sd / (softness * 2 as c_int as c_float)
            }) - 1 as c_int as c_float;
            p = (sd - softness * (1 as c_int as c_float - p * p)) / td;
            tx -= p * tx;
            ty -= p * ty;
            dd = tx * tx + ty * ty;
        }
    }
    if u != 0 {
        let mut cosine: c_float = 0.;
        l2 *= psx;
        cosine = (dd - l1 * l1 - l2 * l2) / (2 as c_int as c_float * l1 * l2);
        if cosine < -(1 as c_int) as c_float {
            cosine = -(1 as c_int) as c_float;
            a2 = 3.1415926535897932385f32 * bendDir as c_float;
        } else if cosine > 1 as c_int as c_float {
            cosine = 1 as c_int as c_float;
            a2 = 0 as c_int as c_float;
            if stretch != 0 {
                a = (spine_sqrtf(dd) / (l1 + l2) - 1 as c_int as c_float) * alpha
                    + 1 as c_int as c_float;
                sx *= a;
                if uniform != 0 {
                    sy *= a;
                }
            }
        } else {
            a2 = acosf(cosine) * bendDir as c_float;
        }
        a = l1 + l2 * cosine;
        b = l2 * sinf(a2);
        a1 = atan2f(ty * a - tx * b, tx * a + ty * b);
    } else {
        a = psx * l2;
        b = psy * l2;
        aa = a * a;
        bb = b * b;
        ll = l1 * l1;
        ta = atan2f(ty, tx);
        c0 = bb * ll + aa * dd - aa * bb;
        c1 = -(2 as c_int) as c_float * bb * l1;
        c2 = bb - aa;
        d = c1 * c1 - 4 as c_int as c_float * c2 * c0;
        if d >= 0 as c_int as c_float {
            let mut q: c_float = spine_sqrtf(d);
            let mut r0: c_float = 0.;
            let mut r1: c_float = 0.;
            if c1 < 0 as c_int as c_float {
                q = -q;
            }
            q = -(c1 + q) * 0.5f32;
            r0 = q / c2;
            r1 = c0 / q;
            r = if (if r0 < 0 as c_int as c_float { -r0 } else { r0 })
                < (if r1 < 0 as c_int as c_float { -r1 } else { r1 })
            {
                r0
            } else {
                r1
            };
            if r * r <= dd {
                y = spine_sqrtf(dd - r * r) * bendDir as c_float;
                a1 = ta - atan2f(y, r);
                a2 = atan2f(y / psy, (r - l1) / psx);
                current_block = 5723090765117457223;
            } else {
                current_block = 13723035087248630346;
            }
        } else {
            current_block = 13723035087248630346;
        }
        match current_block {
            5723090765117457223 => {}
            _ => {
                let mut minAngle: c_float = 3.1415926535897932385f32;
                let mut minX: c_float = l1 - a;
                let mut minDist: c_float = minX * minX;
                let mut minY: c_float = 0 as c_int as c_float;
                let mut maxAngle: c_float = 0 as c_int as c_float;
                let mut maxX: c_float = l1 + a;
                let mut maxDist: c_float = maxX * maxX;
                let mut maxY: c_float = 0 as c_int as c_float;
                c0 = -a * l1 / (aa - bb);
                if c0 >= -(1 as c_int) as c_float && c0 <= 1 as c_int as c_float {
                    c0 = acosf(c0);
                    x = a * cosf(c0) + l1;
                    y = b * sinf(c0);
                    d = x * x + y * y;
                    if d < minDist {
                        minAngle = c0;
                        minDist = d;
                        minX = x;
                        minY = y;
                    }
                    if d > maxDist {
                        maxAngle = c0;
                        maxDist = d;
                        maxX = x;
                        maxY = y;
                    }
                }
                if dd <= (minDist + maxDist) * 0.5f32 {
                    a1 = ta - atan2f(minY * bendDir as c_float, minX);
                    a2 = minAngle * bendDir as c_float;
                } else {
                    a1 = ta - atan2f(maxY * bendDir as c_float, maxX);
                    a2 = maxAngle * bendDir as c_float;
                }
            }
        }
    }
    let mut os: c_float = atan2f(cy, cx) * s2 as c_float;
    let mut rotation: c_float = (*parent).arotation;
    a1 =
        (a1 - os) * (180 as c_int as c_float / 3.1415926535897932385f32) + o1 as c_float - rotation;
    if a1 > 180 as c_int as c_float {
        a1 -= 360 as c_int as c_float;
    } else if a1 < -(180 as c_int) as c_float {
        a1 += 360 as c_int as c_float;
    }
    spBone_updateWorldTransformWith(
        parent,
        px,
        py,
        rotation + a1 * alpha,
        sx,
        sy,
        0 as c_int as c_float,
        0 as c_int as c_float,
    );
    rotation = (*child).arotation;
    a2 = ((a2 + os) * (180 as c_int as c_float / 3.1415926535897932385f32) - (*child).ashearX)
        * s2 as c_float
        + o2 as c_float
        - rotation;
    if a2 > 180 as c_int as c_float {
        a2 -= 360 as c_int as c_float;
    } else if a2 < -(180 as c_int) as c_float {
        a2 += 360 as c_int as c_float;
    }
    spBone_updateWorldTransformWith(
        child,
        cx,
        cy,
        rotation + a2 * alpha,
        (*child).ascaleX,
        (*child).ascaleY,
        (*child).ashearX,
        (*child).ashearY,
    );
}
#[no_mangle]
pub unsafe extern "C" fn spIkConstraintData_create(
    mut name: *const c_char,
) -> *mut spIkConstraintData {
    let mut self_0: *mut spIkConstraintData = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spIkConstraintData>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        6023 as c_int,
    ) as *mut spIkConstraintData;
    let ref mut fresh65 = *(&(*self_0).name as *const *const c_char as *mut *mut c_char);
    *fresh65 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul((spine_strlen(name)).wrapping_add(1 as c_int as c_ulong)),
        b"spine.c\0" as *const u8 as *const c_char,
        6024 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh65, name);
    (*self_0).bendDirection = 1 as c_int;
    (*self_0).compress = 0 as c_int;
    (*self_0).stretch = 0 as c_int;
    (*self_0).uniform = 0 as c_int;
    (*self_0).mix = 1 as c_int as c_float;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spIkConstraintData_dispose(mut self_0: *mut spIkConstraintData) {
    _spFree((*self_0).name as *mut c_void);
    _spFree((*self_0).bones as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
static mut ep: *const c_char = 0 as *const c_char;
#[no_mangle]
pub unsafe extern "C" fn Json_getError() -> *const c_char {
    return ep;
}
unsafe extern "C" fn Json_strcasecmp(mut s1: *const c_char, mut s2: *const c_char) -> c_int {
    if !s1.is_null() && !s2.is_null() {
        return spine_strcasecmp(s1, s2);
    } else if s1 < s2 {
        return -(1 as c_int);
    } else if s1 == s2 {
        return 0 as c_int;
    } else {
        return 1 as c_int;
    };
}
unsafe extern "C" fn Json_new() -> *mut Json {
    return _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<Json>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        6113 as c_int,
    ) as *mut Json;
}
#[no_mangle]
pub unsafe extern "C" fn Json_dispose(mut c: *mut Json) {
    let mut next: *mut Json = 0 as *mut Json;
    while !c.is_null() {
        next = (*c).next;
        if !((*c).child).is_null() {
            Json_dispose((*c).child);
        }
        if !((*c).valueString).is_null() {
            _spFree((*c).valueString as *mut c_void);
        }
        if !((*c).name).is_null() {
            _spFree((*c).name as *mut c_void);
        }
        _spFree(c as *mut c_void);
        c = next;
    }
}
unsafe extern "C" fn parse_number(mut item: *mut Json, mut num: *const c_char) -> *const c_char {
    let mut result: c_double = 0.0f64;
    let mut negative: c_int = 0 as c_int;
    let mut ptr: *mut c_char = num as *mut c_char;
    if *ptr as c_int == '-' as i32 {
        negative = -(1 as c_int);
        ptr = ptr.offset(1);
    }
    while *ptr as c_int >= '0' as i32 && *ptr as c_int <= '9' as i32 {
        result = result * 10.0f64 + (*ptr as c_int - '0' as i32) as c_double;
        ptr = ptr.offset(1);
    }
    if *ptr as c_int == '.' as i32 {
        let mut fraction: c_double = 0.0f64;
        let mut n: c_int = 0 as c_int;
        ptr = ptr.offset(1);
        while *ptr as c_int >= '0' as i32 && *ptr as c_int <= '9' as i32 {
            fraction = fraction * 10.0f64 + (*ptr as c_int - '0' as i32) as c_double;
            ptr = ptr.offset(1);
            n += 1;
        }
        result += fraction / pow(10.0f64, n as c_double);
    }
    if negative != 0 {
        result = -result;
    }
    if *ptr as c_int == 'e' as i32 || *ptr as c_int == 'E' as i32 {
        let mut exponent: c_double = 0 as c_int as c_double;
        let mut expNegative: c_int = 0 as c_int;
        let mut _n_0: c_int = 0 as c_int;
        ptr = ptr.offset(1);
        if *ptr as c_int == '-' as i32 {
            expNegative = -(1 as c_int);
            ptr = ptr.offset(1);
        } else if *ptr as c_int == '+' as i32 {
            ptr = ptr.offset(1);
        }
        while *ptr as c_int >= '0' as i32 && *ptr as c_int <= '9' as i32 {
            exponent = exponent * 10.0f64 + (*ptr as c_int - '0' as i32) as c_double;
            ptr = ptr.offset(1);
            _n_0 += 1;
        }
        if expNegative != 0 {
            result = result / pow(10 as c_int as c_double, exponent);
        } else {
            result = result * pow(10 as c_int as c_double, exponent);
        }
    }
    if ptr != num as *mut c_char {
        (*item).valueFloat = result as c_float;
        (*item).valueInt = result as c_int;
        (*item).type_0 = 3 as c_int;
        return ptr;
    } else {
        ep = num;
        return 0 as *const c_char;
    };
}
static mut firstByteMark: [c_uchar; 7] = [
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0xc0 as c_int as c_uchar,
    0xe0 as c_int as c_uchar,
    0xf0 as c_int as c_uchar,
    0xf8 as c_int as c_uchar,
    0xfc as c_int as c_uchar,
];
unsafe extern "C" fn parse_string(mut item: *mut Json, mut str: *const c_char) -> *const c_char {
    let mut ptr: *const c_char = str.offset(1 as c_int as isize);
    let mut ptr2: *mut c_char = 0 as *mut c_char;
    let mut out: *mut c_char = 0 as *mut c_char;
    let mut len: c_int = 0 as c_int;
    let mut uc: c_uint = 0;
    let mut uc2: c_uint = 0;
    if *str as c_int != '"' as i32 {
        ep = str;
        return 0 as *const c_char;
    }
    while *ptr as c_int != '"' as i32 && *ptr as c_int != 0 && {
        len += 1;
        len != 0
    } {
        let fresh66 = ptr;
        ptr = ptr.offset(1);
        if *fresh66 as c_int == '\\' as i32 {
            ptr = ptr.offset(1);
        }
    }
    out = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong).wrapping_mul((len + 1 as c_int) as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        6214 as c_int,
    ) as *mut c_char;
    if out.is_null() {
        return 0 as *const c_char;
    }
    ptr = str.offset(1 as c_int as isize);
    ptr2 = out;
    while *ptr as c_int != '"' as i32 && *ptr as c_int != 0 {
        if *ptr as c_int != '\\' as i32 {
            let fresh67 = ptr;
            ptr = ptr.offset(1);
            let fresh68 = ptr2;
            ptr2 = ptr2.offset(1);
            *fresh68 = *fresh67;
        } else {
            ptr = ptr.offset(1);
            let mut current_block_41: u64;
            match *ptr as c_int {
                98 => {
                    let fresh69 = ptr2;
                    ptr2 = ptr2.offset(1);
                    *fresh69 = '\u{8}' as i32 as c_char;
                }
                102 => {
                    let fresh70 = ptr2;
                    ptr2 = ptr2.offset(1);
                    *fresh70 = '\u{c}' as i32 as c_char;
                }
                110 => {
                    let fresh71 = ptr2;
                    ptr2 = ptr2.offset(1);
                    *fresh71 = '\n' as i32 as c_char;
                }
                114 => {
                    let fresh72 = ptr2;
                    ptr2 = ptr2.offset(1);
                    *fresh72 = '\r' as i32 as c_char;
                }
                116 => {
                    let fresh73 = ptr2;
                    ptr2 = ptr2.offset(1);
                    *fresh73 = '\t' as i32 as c_char;
                }
                117 => {
                    spine_sscanf!(
                        ptr.offset(1 as c_int as isize),
                        b"%4x\0" as *const u8 as *const c_char,
                        &mut uc as *mut c_uint,
                    );
                    ptr = ptr.offset(4 as c_int as isize);
                    if !(uc >= 0xdc00 as c_int as c_uint && uc <= 0xdfff as c_int as c_uint
                        || uc == 0 as c_int as c_uint)
                    {
                        if uc >= 0xd800 as c_int as c_uint && uc <= 0xdbff as c_int as c_uint {
                            if *ptr.offset(1 as c_int as isize) as c_int != '\\' as i32
                                || *ptr.offset(2 as c_int as isize) as c_int != 'u' as i32
                            {
                                current_block_41 = 9441801433784995173;
                            } else {
                                spine_sscanf!(
                                    ptr.offset(3 as c_int as isize),
                                    b"%4x\0" as *const u8 as *const c_char,
                                    &mut uc2 as *mut c_uint,
                                );
                                ptr = ptr.offset(6 as c_int as isize);
                                if uc2 < 0xdc00 as c_int as c_uint
                                    || uc2 > 0xdfff as c_int as c_uint
                                {
                                    current_block_41 = 9441801433784995173;
                                } else {
                                    uc = (0x10000 as c_int as c_uint).wrapping_add(
                                        (uc & 0x3ff as c_int as c_uint) << 10 as c_int
                                            | uc2 & 0x3ff as c_int as c_uint,
                                    );
                                    current_block_41 = 1608152415753874203;
                                }
                            }
                        } else {
                            current_block_41 = 1608152415753874203;
                        }
                        match current_block_41 {
                            9441801433784995173 => {}
                            _ => {
                                len = 4 as c_int;
                                if uc < 0x80 as c_int as c_uint {
                                    len = 1 as c_int;
                                } else if uc < 0x800 as c_int as c_uint {
                                    len = 2 as c_int;
                                } else if uc < 0x10000 as c_int as c_uint {
                                    len = 3 as c_int;
                                }
                                ptr2 = ptr2.offset(len as isize);
                                let mut current_block_38: u64;
                                match len {
                                    4 => {
                                        ptr2 = ptr2.offset(-1);
                                        *ptr2 = ((uc | 0x80 as c_int as c_uint)
                                            & 0xbf as c_int as c_uint)
                                            as c_char;
                                        uc >>= 6 as c_int;
                                        current_block_38 = 8430303908314653947;
                                    }
                                    3 => {
                                        current_block_38 = 8430303908314653947;
                                    }
                                    2 => {
                                        current_block_38 = 12361033550402706186;
                                    }
                                    1 => {
                                        current_block_38 = 14272422275774239914;
                                    }
                                    _ => {
                                        current_block_38 = 4567019141635105728;
                                    }
                                }
                                match current_block_38 {
                                    8430303908314653947 => {
                                        ptr2 = ptr2.offset(-1);
                                        *ptr2 = ((uc | 0x80 as c_int as c_uint)
                                            & 0xbf as c_int as c_uint)
                                            as c_char;
                                        uc >>= 6 as c_int;
                                        current_block_38 = 12361033550402706186;
                                    }
                                    _ => {}
                                }
                                match current_block_38 {
                                    12361033550402706186 => {
                                        ptr2 = ptr2.offset(-1);
                                        *ptr2 = ((uc | 0x80 as c_int as c_uint)
                                            & 0xbf as c_int as c_uint)
                                            as c_char;
                                        uc >>= 6 as c_int;
                                        current_block_38 = 14272422275774239914;
                                    }
                                    _ => {}
                                }
                                match current_block_38 {
                                    14272422275774239914 => {
                                        ptr2 = ptr2.offset(-1);
                                        *ptr2 =
                                            (uc | firstByteMark[len as usize] as c_uint) as c_char;
                                    }
                                    _ => {}
                                }
                                ptr2 = ptr2.offset(len as isize);
                            }
                        }
                    }
                }
                _ => {
                    let fresh74 = ptr2;
                    ptr2 = ptr2.offset(1);
                    *fresh74 = *ptr;
                }
            }
            ptr = ptr.offset(1);
        }
    }
    *ptr2 = 0 as c_int as c_char;
    if *ptr as c_int == '"' as i32 {
        ptr = ptr.offset(1);
    }
    (*item).valueString = out;
    (*item).type_0 = 4 as c_int;
    return ptr;
}
unsafe extern "C" fn skip(mut in_0: *const c_char) -> *const c_char {
    if in_0.is_null() {
        return 0 as *const c_char;
    }
    while *in_0 as c_int != 0 && *in_0 as c_uchar as c_int <= 32 as c_int {
        in_0 = in_0.offset(1);
    }
    return in_0;
}
#[no_mangle]
pub unsafe extern "C" fn Json_create(mut value: *const c_char) -> *mut Json {
    let mut c: *mut Json = 0 as *mut Json;
    ep = 0 as *const c_char;
    if value.is_null() {
        return 0 as *mut Json;
    }
    c = Json_new();
    if c.is_null() {
        return 0 as *mut Json;
    }
    value = parse_value(c, skip(value));
    if value.is_null() {
        Json_dispose(c);
        return 0 as *mut Json;
    }
    return c;
}
unsafe extern "C" fn parse_value(mut item: *mut Json, mut value: *const c_char) -> *const c_char {
    match *value as c_int {
        110 => {
            if spine_strncmp(
                value.offset(1 as c_int as isize),
                b"ull\0" as *const u8 as *const c_char,
                3 as c_int as size_t,
            ) == 0
            {
                (*item).type_0 = 2 as c_int;
                return value.offset(4 as c_int as isize);
            }
        }
        102 => {
            if spine_strncmp(
                value.offset(1 as c_int as isize),
                b"alse\0" as *const u8 as *const c_char,
                4 as c_int as size_t,
            ) == 0
            {
                (*item).type_0 = 0 as c_int;
                return value.offset(5 as c_int as isize);
            }
        }
        116 => {
            if spine_strncmp(
                value.offset(1 as c_int as isize),
                b"rue\0" as *const u8 as *const c_char,
                3 as c_int as size_t,
            ) == 0
            {
                (*item).type_0 = 1 as c_int;
                (*item).valueInt = 1 as c_int;
                return value.offset(4 as c_int as isize);
            }
        }
        34 => return parse_string(item, value),
        91 => return parse_array(item, value),
        123 => return parse_object(item, value),
        45 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
            return parse_number(item, value);
        }
        _ => {}
    }
    ep = value;
    return 0 as *const c_char;
}
unsafe extern "C" fn parse_array(mut item: *mut Json, mut value: *const c_char) -> *const c_char {
    let mut child: *mut Json = 0 as *mut Json;
    (*item).type_0 = 5 as c_int;
    value = skip(value.offset(1 as c_int as isize));
    if *value as c_int == ']' as i32 {
        return value.offset(1 as c_int as isize);
    }
    child = Json_new();
    (*item).child = child;
    if ((*item).child).is_null() {
        return 0 as *const c_char;
    }
    value = skip(parse_value(child, skip(value)));
    if value.is_null() {
        return 0 as *const c_char;
    }
    (*item).size = 1 as c_int;
    while *value as c_int == ',' as i32 {
        let mut new_item: *mut Json = Json_new();
        if new_item.is_null() {
            return 0 as *const c_char;
        }
        (*child).next = new_item;
        child = new_item;
        value = skip(parse_value(child, skip(value.offset(1 as c_int as isize))));
        if value.is_null() {
            return 0 as *const c_char;
        }
        (*item).size += 1;
    }
    if *value as c_int == ']' as i32 {
        return value.offset(1 as c_int as isize);
    }
    ep = value;
    return 0 as *const c_char;
}
unsafe extern "C" fn parse_object(mut item: *mut Json, mut value: *const c_char) -> *const c_char {
    let mut child: *mut Json = 0 as *mut Json;
    (*item).type_0 = 6 as c_int;
    value = skip(value.offset(1 as c_int as isize));
    if *value as c_int == '}' as i32 {
        return value.offset(1 as c_int as isize);
    }
    child = Json_new();
    (*item).child = child;
    if ((*item).child).is_null() {
        return 0 as *const c_char;
    }
    value = skip(parse_string(child, skip(value)));
    if value.is_null() {
        return 0 as *const c_char;
    }
    (*child).name = (*child).valueString;
    (*child).valueString = 0 as *const c_char;
    if *value as c_int != ':' as i32 {
        ep = value;
        return 0 as *const c_char;
    }
    value = skip(parse_value(child, skip(value.offset(1 as c_int as isize))));
    if value.is_null() {
        return 0 as *const c_char;
    }
    (*item).size = 1 as c_int;
    while *value as c_int == ',' as i32 {
        let mut new_item: *mut Json = Json_new();
        if new_item.is_null() {
            return 0 as *const c_char;
        }
        (*child).next = new_item;
        child = new_item;
        value = skip(parse_string(child, skip(value.offset(1 as c_int as isize))));
        if value.is_null() {
            return 0 as *const c_char;
        }
        (*child).name = (*child).valueString;
        (*child).valueString = 0 as *const c_char;
        if *value as c_int != ':' as i32 {
            ep = value;
            return 0 as *const c_char;
        }
        value = skip(parse_value(child, skip(value.offset(1 as c_int as isize))));
        if value.is_null() {
            return 0 as *const c_char;
        }
        (*item).size += 1;
    }
    if *value as c_int == '}' as i32 {
        return value.offset(1 as c_int as isize);
    }
    ep = value;
    return 0 as *const c_char;
}
#[no_mangle]
pub unsafe extern "C" fn Json_getItem(
    mut object: *mut Json,
    mut string: *const c_char,
) -> *mut Json {
    let mut c: *mut Json = (*object).child;
    while !c.is_null() && Json_strcasecmp((*c).name, string) != 0 {
        c = (*c).next;
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn Json_getItemAtIndex(
    mut object: *mut Json,
    mut childIndex: c_int,
) -> *mut Json {
    let mut current: *mut Json = (*object).child;
    while !current.is_null() && childIndex > 0 as c_int {
        childIndex -= 1;
        current = (*current).next;
    }
    return current;
}
#[no_mangle]
pub unsafe extern "C" fn Json_getString(
    mut object: *mut Json,
    mut name: *const c_char,
    mut defaultValue: *const c_char,
) -> *const c_char {
    object = Json_getItem(object, name);
    if !object.is_null() {
        return (*object).valueString;
    }
    return defaultValue;
}
#[no_mangle]
pub unsafe extern "C" fn Json_getFloat(
    mut value: *mut Json,
    mut name: *const c_char,
    mut defaultValue: c_float,
) -> c_float {
    value = Json_getItem(value, name);
    return if !value.is_null() {
        (*value).valueFloat
    } else {
        defaultValue
    };
}
#[no_mangle]
pub unsafe extern "C" fn Json_getInt(
    mut value: *mut Json,
    mut name: *const c_char,
    mut defaultValue: c_int,
) -> c_int {
    value = Json_getItem(value, name);
    return if !value.is_null() {
        (*value).valueInt
    } else {
        defaultValue
    };
}
#[no_mangle]
pub unsafe extern "C" fn _spMeshAttachment_dispose(mut attachment: *mut spAttachment) {
    let mut self_0: *mut spMeshAttachment = attachment as *mut spMeshAttachment;
    if !((*self_0).sequence).is_null() {
        spSequence_dispose((*self_0).sequence);
    }
    _spFree((*self_0).path as *mut c_void);
    _spFree((*self_0).uvs as *mut c_void);
    if ((*self_0).parentMesh).is_null() {
        _spVertexAttachment_deinit(&mut (*self_0).super_0);
        _spFree((*self_0).regionUVs as *mut c_void);
        _spFree((*self_0).triangles as *mut c_void);
        _spFree((*self_0).edges as *mut c_void);
    } else {
        _spAttachment_deinit(attachment);
    }
    if !((*self_0).sequence).is_null() {
        _spFree((*self_0).sequence as *mut c_void);
    }
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _spMeshAttachment_copy(
    mut attachment: *mut spAttachment,
) -> *mut spAttachment {
    let mut copy: *mut spMeshAttachment = 0 as *mut spMeshAttachment;
    let mut self_0: *mut spMeshAttachment = attachment as *mut spMeshAttachment;
    if !((*self_0).parentMesh).is_null() {
        return &mut (*(spMeshAttachment_newLinkedMesh
            as unsafe extern "C" fn(*mut spMeshAttachment) -> *mut spMeshAttachment)(
            self_0
        ))
        .super_0
        .super_0;
    }
    copy = spMeshAttachment_create((*attachment).name);
    (*copy).rendererObject = (*self_0).rendererObject;
    (*copy).region = (*self_0).region;
    (*copy).sequence = if !((*self_0).sequence).is_null() {
        spSequence_copy((*self_0).sequence)
    } else {
        0 as *mut spSequence
    };
    let ref mut fresh75 = *(&mut (*copy).path as *mut *const c_char as *mut *mut c_char);
    *fresh75 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul((spine_strlen((*self_0).path)).wrapping_add(1 as c_int as c_ulong)),
        b"spine.c\0" as *const u8 as *const c_char,
        6570 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh75, (*self_0).path);
    spColor_setFromColor(&mut (*copy).color, &mut (*self_0).color);
    spVertexAttachment_copyTo(&mut (*self_0).super_0, &mut (*copy).super_0);
    (*copy).regionUVs = _spMalloc(
        (::core::mem::size_of::<c_float>() as c_ulong)
            .wrapping_mul((*self_0).super_0.worldVerticesLength as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        6574 as c_int,
    ) as *mut c_float;
    spine_memcpy(
        (*copy).regionUVs as *mut c_void,
        (*self_0).regionUVs as *const c_void,
        ((*self_0).super_0.worldVerticesLength as c_ulong)
            .wrapping_mul(::core::mem::size_of::<c_float>() as c_ulong),
    );
    (*copy).uvs = _spMalloc(
        (::core::mem::size_of::<c_float>() as c_ulong)
            .wrapping_mul((*self_0).super_0.worldVerticesLength as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        6576 as c_int,
    ) as *mut c_float;
    spine_memcpy(
        (*copy).uvs as *mut c_void,
        (*self_0).uvs as *const c_void,
        ((*self_0).super_0.worldVerticesLength as c_ulong)
            .wrapping_mul(::core::mem::size_of::<c_float>() as c_ulong),
    );
    (*copy).trianglesCount = (*self_0).trianglesCount;
    (*copy).triangles = _spMalloc(
        (::core::mem::size_of::<c_ushort>() as c_ulong)
            .wrapping_mul((*self_0).trianglesCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        6579 as c_int,
    ) as *mut c_ushort;
    spine_memcpy(
        (*copy).triangles as *mut c_void,
        (*self_0).triangles as *const c_void,
        ((*self_0).trianglesCount as c_ulong)
            .wrapping_mul(::core::mem::size_of::<c_short>() as c_ulong),
    );
    (*copy).hullLength = (*self_0).hullLength;
    if (*self_0).edgesCount > 0 as c_int {
        (*copy).edgesCount = (*self_0).edgesCount;
        (*copy).edges = _spMalloc(
            (::core::mem::size_of::<c_int>() as c_ulong)
                .wrapping_mul((*self_0).edgesCount as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            6584 as c_int,
        ) as *mut c_int;
        spine_memcpy(
            (*copy).edges as *mut c_void,
            (*self_0).edges as *const c_void,
            ((*self_0).edgesCount as c_ulong)
                .wrapping_mul(::core::mem::size_of::<c_int>() as c_ulong),
        );
    }
    (*copy).width = (*self_0).width;
    (*copy).height = (*self_0).height;
    return &mut (*copy).super_0.super_0;
}
#[no_mangle]
pub unsafe extern "C" fn spMeshAttachment_newLinkedMesh(
    mut self_0: *mut spMeshAttachment,
) -> *mut spMeshAttachment {
    let mut copy: *mut spMeshAttachment = spMeshAttachment_create((*self_0).super_0.super_0.name);
    (*copy).rendererObject = (*self_0).rendererObject;
    (*copy).region = (*self_0).region;
    let ref mut fresh76 = *(&mut (*copy).path as *mut *const c_char as *mut *mut c_char);
    *fresh76 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul((spine_strlen((*self_0).path)).wrapping_add(1 as c_int as c_ulong)),
        b"spine.c\0" as *const u8 as *const c_char,
        6598 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh76, (*self_0).path);
    spColor_setFromColor(&mut (*copy).color, &mut (*self_0).color);
    (*copy).super_0.timelineAttachment = (*self_0).super_0.timelineAttachment;
    spMeshAttachment_setParentMesh(
        copy,
        if !((*self_0).parentMesh).is_null() {
            (*self_0).parentMesh
        } else {
            self_0
        },
    );
    if !((*copy).region).is_null() {
        spMeshAttachment_updateRegion(copy);
    }
    return copy;
}
#[no_mangle]
pub unsafe extern "C" fn spMeshAttachment_create(mut name: *const c_char) -> *mut spMeshAttachment {
    let mut self_0: *mut spMeshAttachment = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spMeshAttachment>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        6607 as c_int,
    ) as *mut spMeshAttachment;
    _spVertexAttachment_init(&mut (*self_0).super_0);
    spColor_setFromFloats(
        &mut (*self_0).color,
        1 as c_int as c_float,
        1 as c_int as c_float,
        1 as c_int as c_float,
        1 as c_int as c_float,
    );
    _spAttachment_init(
        &mut (*self_0).super_0.super_0,
        name,
        SP_ATTACHMENT_MESH,
        Some(_spMeshAttachment_dispose as unsafe extern "C" fn(*mut spAttachment) -> ()),
        Some(
            _spMeshAttachment_copy as unsafe extern "C" fn(*mut spAttachment) -> *mut spAttachment,
        ),
    );
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spMeshAttachment_updateRegion(mut self_0: *mut spMeshAttachment) {
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    let mut uvs: *mut c_float = 0 as *mut c_float;
    let mut u: c_float = 0.;
    let mut v: c_float = 0.;
    let mut width: c_float = 0.;
    let mut height: c_float = 0.;
    let mut verticesLength: c_int = (*self_0).super_0.worldVerticesLength;
    _spFree((*self_0).uvs as *mut c_void);
    (*self_0).uvs = _spMalloc(
        (::core::mem::size_of::<c_float>() as c_ulong).wrapping_mul(verticesLength as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        6620 as c_int,
    ) as *mut c_float;
    uvs = (*self_0).uvs;
    n = verticesLength;
    u = (*(*self_0).region).u;
    v = (*(*self_0).region).v;
    match (*(*self_0).region).degrees {
        90 => {
            let mut textureWidth: c_float = (*(*self_0).region).height as c_float
                / ((*(*self_0).region).u2 - (*(*self_0).region).u);
            let mut textureHeight: c_float = (*(*self_0).region).width as c_float
                / ((*(*self_0).region).v2 - (*(*self_0).region).v);
            u -= ((*(*self_0).region).originalHeight as c_float
                - (*(*self_0).region).offsetY
                - (*(*self_0).region).height as c_float)
                / textureWidth;
            v -= ((*(*self_0).region).originalWidth as c_float
                - (*(*self_0).region).offsetX
                - (*(*self_0).region).width as c_float)
                / textureHeight;
            width = (*(*self_0).region).originalHeight as c_float / textureWidth;
            height = (*(*self_0).region).originalWidth as c_float / textureHeight;
            i = 0 as c_int;
            while i < n {
                *uvs.offset(i as isize) =
                    u + *((*self_0).regionUVs).offset((i + 1 as c_int) as isize) * width;
                *uvs.offset((i + 1 as c_int) as isize) = v
                    + (1 as c_int as c_float - *((*self_0).regionUVs).offset(i as isize)) * height;
                i += 2 as c_int;
            }
            return;
        }
        180 => {
            let mut textureWidth_0: c_float = (*(*self_0).region).width as c_float
                / ((*(*self_0).region).u2 - (*(*self_0).region).u);
            let mut textureHeight_0: c_float = (*(*self_0).region).height as c_float
                / ((*(*self_0).region).v2 - (*(*self_0).region).v);
            u -= ((*(*self_0).region).originalWidth as c_float
                - (*(*self_0).region).offsetX
                - (*(*self_0).region).width as c_float)
                / textureWidth_0;
            v -= (*(*self_0).region).offsetY / textureHeight_0;
            width = (*(*self_0).region).originalWidth as c_float / textureWidth_0;
            height = (*(*self_0).region).originalHeight as c_float / textureHeight_0;
            i = 0 as c_int;
            while i < n {
                *uvs.offset(i as isize) =
                    u + (1 as c_int as c_float - *((*self_0).regionUVs).offset(i as isize)) * width;
                *uvs.offset((i + 1 as c_int) as isize) = v
                    + (1 as c_int as c_float
                        - *((*self_0).regionUVs).offset((i + 1 as c_int) as isize))
                        * height;
                i += 2 as c_int;
            }
            return;
        }
        270 => {
            let mut textureHeight_1: c_float = (*(*self_0).region).height as c_float
                / ((*(*self_0).region).v2 - (*(*self_0).region).v);
            let mut textureWidth_1: c_float = (*(*self_0).region).width as c_float
                / ((*(*self_0).region).u2 - (*(*self_0).region).u);
            u -= (*(*self_0).region).offsetY / textureWidth_1;
            v -= (*(*self_0).region).offsetX / textureHeight_1;
            width = (*(*self_0).region).originalHeight as c_float / textureWidth_1;
            height = (*(*self_0).region).originalWidth as c_float / textureHeight_1;
            i = 0 as c_int;
            while i < n {
                *uvs.offset(i as isize) = u
                    + (1 as c_int as c_float
                        - *((*self_0).regionUVs).offset((i + 1 as c_int) as isize))
                        * width;
                *uvs.offset((i + 1 as c_int) as isize) =
                    v + *((*self_0).regionUVs).offset(i as isize) * height;
                i += 2 as c_int;
            }
            return;
        }
        _ => {
            let mut textureWidth_2: c_float = (*(*self_0).region).width as c_float
                / ((*(*self_0).region).u2 - (*(*self_0).region).u);
            let mut textureHeight_2: c_float = (*(*self_0).region).height as c_float
                / ((*(*self_0).region).v2 - (*(*self_0).region).v);
            u -= (*(*self_0).region).offsetX / textureWidth_2;
            v -= ((*(*self_0).region).originalHeight as c_float
                - (*(*self_0).region).offsetY
                - (*(*self_0).region).height as c_float)
                / textureHeight_2;
            width = (*(*self_0).region).originalWidth as c_float / textureWidth_2;
            height = (*(*self_0).region).originalHeight as c_float / textureHeight_2;
            i = 0 as c_int;
            while i < n {
                *uvs.offset(i as isize) = u + *((*self_0).regionUVs).offset(i as isize) * width;
                *uvs.offset((i + 1 as c_int) as isize) =
                    v + *((*self_0).regionUVs).offset((i + 1 as c_int) as isize) * height;
                i += 2 as c_int;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn spMeshAttachment_setParentMesh(
    mut self_0: *mut spMeshAttachment,
    mut parentMesh: *mut spMeshAttachment,
) {
    let ref mut fresh77 =
        *(&(*self_0).parentMesh as *const *mut spMeshAttachment as *mut *mut spMeshAttachment);
    *fresh77 = parentMesh;
    if !parentMesh.is_null() {
        (*self_0).super_0.bones = (*parentMesh).super_0.bones;
        (*self_0).super_0.bonesCount = (*parentMesh).super_0.bonesCount;
        (*self_0).super_0.vertices = (*parentMesh).super_0.vertices;
        (*self_0).super_0.verticesCount = (*parentMesh).super_0.verticesCount;
        (*self_0).regionUVs = (*parentMesh).regionUVs;
        (*self_0).triangles = (*parentMesh).triangles;
        (*self_0).trianglesCount = (*parentMesh).trianglesCount;
        (*self_0).hullLength = (*parentMesh).hullLength;
        (*self_0).super_0.worldVerticesLength = (*parentMesh).super_0.worldVerticesLength;
        (*self_0).edges = (*parentMesh).edges;
        (*self_0).edgesCount = (*parentMesh).edgesCount;
        (*self_0).width = (*parentMesh).width;
        (*self_0).height = (*parentMesh).height;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _spPathAttachment_dispose(mut attachment: *mut spAttachment) {
    let mut self_0: *mut spPathAttachment = attachment as *mut spPathAttachment;
    _spVertexAttachment_deinit(&mut (*self_0).super_0);
    _spFree((*self_0).lengths as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _spPathAttachment_copy(
    mut attachment: *mut spAttachment,
) -> *mut spAttachment {
    let mut copy: *mut spPathAttachment = spPathAttachment_create((*attachment).name);
    let mut self_0: *mut spPathAttachment = attachment as *mut spPathAttachment;
    spVertexAttachment_copyTo(&mut (*self_0).super_0, &mut (*copy).super_0);
    (*copy).lengthsLength = (*self_0).lengthsLength;
    (*copy).lengths = _spMalloc(
        (::core::mem::size_of::<c_float>() as c_ulong)
            .wrapping_mul((*self_0).lengthsLength as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        6751 as c_int,
    ) as *mut c_float;
    spine_memcpy(
        (*copy).lengths as *mut c_void,
        (*self_0).lengths as *const c_void,
        ((*self_0).lengthsLength as c_ulong)
            .wrapping_mul(::core::mem::size_of::<c_float>() as c_ulong),
    );
    (*copy).closed = (*self_0).closed;
    (*copy).constantSpeed = (*self_0).constantSpeed;
    return &mut (*copy).super_0.super_0;
}
#[no_mangle]
pub unsafe extern "C" fn spPathAttachment_create(mut name: *const c_char) -> *mut spPathAttachment {
    let mut self_0: *mut spPathAttachment = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spPathAttachment>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        6759 as c_int,
    ) as *mut spPathAttachment;
    _spVertexAttachment_init(&mut (*self_0).super_0);
    _spAttachment_init(
        &mut (*self_0).super_0.super_0,
        name,
        SP_ATTACHMENT_PATH,
        Some(_spPathAttachment_dispose as unsafe extern "C" fn(*mut spAttachment) -> ()),
        Some(
            _spPathAttachment_copy as unsafe extern "C" fn(*mut spAttachment) -> *mut spAttachment,
        ),
    );
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spPathConstraint_create(
    mut data: *mut spPathConstraintData,
    mut skeleton: *const spSkeleton,
) -> *mut spPathConstraint {
    let mut i: c_int = 0;
    let mut self_0: *mut spPathConstraint = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spPathConstraint>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        6804 as c_int,
    ) as *mut spPathConstraint;
    let ref mut fresh78 =
        *(&(*self_0).data as *const *mut spPathConstraintData as *mut *mut spPathConstraintData);
    *fresh78 = data;
    (*self_0).bonesCount = (*data).bonesCount;
    let ref mut fresh79 = *(&(*self_0).bones as *const *mut *mut spBone as *mut *mut *mut spBone);
    *fresh79 = _spMalloc(
        (::core::mem::size_of::<*mut spBone>() as c_ulong)
            .wrapping_mul((*self_0).bonesCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        6807 as c_int,
    ) as *mut *mut spBone;
    i = 0 as c_int;
    while i < (*self_0).bonesCount {
        let ref mut fresh80 = *((*self_0).bones).offset(i as isize);
        *fresh80 = spSkeleton_findBone(
            skeleton,
            (**((*(*self_0).data).bones).offset(i as isize)).name,
        );
        i += 1;
    }
    (*self_0).target = spSkeleton_findSlot(skeleton, (*(*(*self_0).data).target).name);
    (*self_0).position = (*data).position;
    (*self_0).spacing = (*data).spacing;
    (*self_0).mixRotate = (*data).mixRotate;
    (*self_0).mixX = (*data).mixX;
    (*self_0).mixY = (*data).mixY;
    (*self_0).spacesCount = 0 as c_int;
    (*self_0).spaces = 0 as *mut c_float;
    (*self_0).positionsCount = 0 as c_int;
    (*self_0).positions = 0 as *mut c_float;
    (*self_0).worldCount = 0 as c_int;
    (*self_0).world = 0 as *mut c_float;
    (*self_0).curvesCount = 0 as c_int;
    (*self_0).curves = 0 as *mut c_float;
    (*self_0).lengthsCount = 0 as c_int;
    (*self_0).lengths = 0 as *mut c_float;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spPathConstraint_dispose(mut self_0: *mut spPathConstraint) {
    _spFree((*self_0).bones as *mut c_void);
    _spFree((*self_0).spaces as *mut c_void);
    if !((*self_0).positions).is_null() {
        _spFree((*self_0).positions as *mut c_void);
    }
    if !((*self_0).world).is_null() {
        _spFree((*self_0).world as *mut c_void);
    }
    if !((*self_0).curves).is_null() {
        _spFree((*self_0).curves as *mut c_void);
    }
    if !((*self_0).lengths).is_null() {
        _spFree((*self_0).lengths as *mut c_void);
    }
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spPathConstraint_update(mut self_0: *mut spPathConstraint) {
    let mut i: c_int = 0;
    let mut p: c_int = 0;
    let mut n: c_int = 0;
    let mut length: c_float = 0.;
    let mut setupLength: c_float = 0.;
    let mut x: c_float = 0.;
    let mut y: c_float = 0.;
    let mut dx: c_float = 0.;
    let mut dy: c_float = 0.;
    let mut s: c_float = 0.;
    let mut sum: c_float = 0.;
    let mut spaces: *mut c_float = 0 as *mut c_float;
    let mut lengths: *mut c_float = 0 as *mut c_float;
    let mut positions: *mut c_float = 0 as *mut c_float;
    let mut spacing: c_float = 0.;
    let mut boneX: c_float = 0.;
    let mut boneY: c_float = 0.;
    let mut offsetRotation: c_float = 0.;
    let mut tip: c_int = 0;
    let mut mixRotate: c_float = (*self_0).mixRotate;
    let mut mixX: c_float = (*self_0).mixX;
    let mut mixY: c_float = (*self_0).mixY;
    let mut lengthSpacing: c_int = 0;
    let mut attachment: *mut spPathAttachment =
        (*(*self_0).target).attachment as *mut spPathAttachment;
    let mut data: *mut spPathConstraintData = (*self_0).data;
    let mut tangents: c_int =
        ((*data).rotateMode as c_uint == SP_ROTATE_MODE_TANGENT as c_int as c_uint) as c_int;
    let mut scale: c_int =
        ((*data).rotateMode as c_uint == SP_ROTATE_MODE_CHAIN_SCALE as c_int as c_uint) as c_int;
    let mut boneCount: c_int = (*self_0).bonesCount;
    let mut spacesCount: c_int = if tangents != 0 {
        boneCount
    } else {
        boneCount + 1 as c_int
    };
    let mut bones: *mut *mut spBone = (*self_0).bones;
    let mut pa: *mut spBone = 0 as *mut spBone;
    if mixRotate == 0 as c_int as c_float
        && mixX == 0 as c_int as c_float
        && mixY == 0 as c_int as c_float
    {
        return;
    }
    if attachment.is_null()
        || (*attachment).super_0.super_0.type_0 as c_uint != SP_ATTACHMENT_PATH as c_int as c_uint
    {
        return;
    }
    if (*self_0).spacesCount != spacesCount {
        if !((*self_0).spaces).is_null() {
            _spFree((*self_0).spaces as *mut c_void);
        }
        (*self_0).spaces = _spMalloc(
            (::core::mem::size_of::<c_float>() as c_ulong).wrapping_mul(spacesCount as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            6860 as c_int,
        ) as *mut c_float;
        (*self_0).spacesCount = spacesCount;
    }
    spaces = (*self_0).spaces;
    *spaces.offset(0 as c_int as isize) = 0 as c_int as c_float;
    lengths = 0 as *mut c_float;
    spacing = (*self_0).spacing;
    if scale != 0 {
        if (*self_0).lengthsCount != boneCount {
            if !((*self_0).lengths).is_null() {
                _spFree((*self_0).lengths as *mut c_void);
            }
            (*self_0).lengths = _spMalloc(
                (::core::mem::size_of::<c_float>() as c_ulong).wrapping_mul(boneCount as c_ulong),
                b"spine.c\0" as *const u8 as *const c_char,
                6871 as c_int,
            ) as *mut c_float;
            (*self_0).lengthsCount = boneCount;
        }
        lengths = (*self_0).lengths;
    }
    match (*data).spacingMode as c_uint {
        2 => {
            if scale != 0 {
                i = 0 as c_int;
                n = spacesCount - 1 as c_int;
                while i < n {
                    let mut bone: *mut spBone = *bones.offset(i as isize);
                    setupLength = (*(*bone).data).length;
                    if setupLength < 0.00001f32 {
                        *lengths.offset(i as isize) = 0 as c_int as c_float;
                    } else {
                        x = setupLength * (*bone).a;
                        y = setupLength * (*bone).c;
                        *lengths.offset(i as isize) = spine_sqrtf(x * x + y * y);
                    }
                    i += 1;
                }
            }
            i = 1 as c_int;
            n = spacesCount;
            while i < n {
                *spaces.offset(i as isize) = spacing;
                i += 1;
            }
        }
        3 => {
            sum = 0 as c_int as c_float;
            i = 0 as c_int;
            n = spacesCount - 1 as c_int;
            while i < n {
                let mut bone_0: *mut spBone = *bones.offset(i as isize);
                setupLength = (*(*bone_0).data).length;
                if setupLength < 0.00001f32 {
                    if scale != 0 {
                        *lengths.offset(i as isize) = 0 as c_int as c_float;
                    }
                    i += 1;
                    *spaces.offset(i as isize) = spacing;
                } else {
                    x = setupLength * (*bone_0).a;
                    y = setupLength * (*bone_0).c;
                    length = spine_sqrtf(x * x + y * y);
                    if scale != 0 {
                        *lengths.offset(i as isize) = length;
                    }
                    i += 1;
                    *spaces.offset(i as isize) = length;
                    sum += length;
                }
            }
            if sum > 0 as c_int as c_float {
                sum = spacesCount as c_float / sum * spacing;
                i = 1 as c_int;
                while i < spacesCount {
                    *spaces.offset(i as isize) *= sum;
                    i += 1;
                }
            }
        }
        _ => {
            lengthSpacing = ((*data).spacingMode as c_uint
                == SP_SPACING_MODE_LENGTH as c_int as c_uint) as c_int;
            i = 0 as c_int;
            n = spacesCount - 1 as c_int;
            while i < n {
                let mut bone_1: *mut spBone = *bones.offset(i as isize);
                setupLength = (*(*bone_1).data).length;
                if setupLength < 0.00001f32 {
                    if scale != 0 {
                        *lengths.offset(i as isize) = 0 as c_int as c_float;
                    }
                    i += 1;
                    *spaces.offset(i as isize) = spacing;
                } else {
                    x = setupLength * (*bone_1).a;
                    y = setupLength * (*bone_1).c;
                    length = spine_sqrtf(x * x + y * y);
                    if scale != 0 {
                        *lengths.offset(i as isize) = length;
                    }
                    i += 1;
                    *spaces.offset(i as isize) = (if lengthSpacing != 0 {
                        setupLength + spacing
                    } else {
                        spacing
                    }) * length
                        / setupLength;
                }
            }
        }
    }
    positions = spPathConstraint_computeWorldPositions(self_0, attachment, spacesCount, tangents);
    boneX = *positions.offset(0 as c_int as isize);
    boneY = *positions.offset(1 as c_int as isize);
    offsetRotation = (*(*self_0).data).offsetRotation;
    tip = 0 as c_int;
    if offsetRotation == 0 as c_int as c_float {
        tip = ((*data).rotateMode as c_uint == SP_ROTATE_MODE_CHAIN as c_int as c_uint) as c_int;
    } else {
        tip = 0 as c_int;
        pa = (*(*self_0).target).bone;
        offsetRotation *= if (*pa).a * (*pa).d - (*pa).b * (*pa).c > 0 as c_int as c_float {
            3.1415926535897932385f32 / 180 as c_int as c_float
        } else {
            -(3.1415926535897932385f32 / 180 as c_int as c_float)
        };
    }
    i = 0 as c_int;
    p = 3 as c_int;
    while i < boneCount {
        let mut bone_2: *mut spBone = *bones.offset(i as isize);
        *(&(*bone_2).worldX as *const c_float as *mut c_float) += (boneX - (*bone_2).worldX) * mixX;
        *(&(*bone_2).worldY as *const c_float as *mut c_float) += (boneY - (*bone_2).worldY) * mixY;
        x = *positions.offset(p as isize);
        y = *positions.offset((p + 1 as c_int) as isize);
        dx = x - boneX;
        dy = y - boneY;
        if scale != 0 {
            length = *lengths.offset(i as isize);
            if length != 0 as c_int as c_float {
                s = (spine_sqrtf(dx * dx + dy * dy) / length - 1 as c_int as c_float) * mixRotate
                    + 1 as c_int as c_float;
                *(&(*bone_2).a as *const c_float as *mut c_float) *= s;
                *(&(*bone_2).c as *const c_float as *mut c_float) *= s;
            }
        }
        boneX = x;
        boneY = y;
        if mixRotate > 0 as c_int as c_float {
            let mut a: c_float = (*bone_2).a;
            let mut b: c_float = (*bone_2).b;
            let mut c: c_float = (*bone_2).c;
            let mut d: c_float = (*bone_2).d;
            let mut r: c_float = 0.;
            let mut cosine: c_float = 0.;
            let mut sine: c_float = 0.;
            if tangents != 0 {
                r = *positions.offset((p - 1 as c_int) as isize);
            } else if *spaces.offset((i + 1 as c_int) as isize) == 0 as c_int as c_float {
                r = *positions.offset((p + 2 as c_int) as isize);
            } else {
                r = atan2f(dy, dx);
            }
            r -= atan2f(c, a)
                - offsetRotation * (3.1415926535897932385f32 / 180 as c_int as c_float);
            if tip != 0 {
                cosine = cosf(r);
                sine = sinf(r);
                length = (*(*bone_2).data).length;
                boneX += (length * (cosine * a - sine * c) - dx) * mixRotate;
                boneY += (length * (sine * a + cosine * c) - dy) * mixRotate;
            } else {
                r += offsetRotation;
            }
            if r > 3.1415926535897932385f32 {
                r -= 3.1415926535897932385f32 * 2 as c_int as c_float;
            } else if r < -3.1415926535897932385f32 {
                r += 3.1415926535897932385f32 * 2 as c_int as c_float;
            }
            r *= mixRotate;
            cosine = cosf(r);
            sine = sinf(r);
            *(&(*bone_2).a as *const c_float as *mut c_float) = cosine * a - sine * c;
            *(&(*bone_2).b as *const c_float as *mut c_float) = cosine * b - sine * d;
            *(&(*bone_2).c as *const c_float as *mut c_float) = sine * a + cosine * c;
            *(&(*bone_2).d as *const c_float as *mut c_float) = sine * b + cosine * d;
        }
        spBone_updateAppliedTransform(bone_2);
        i += 1;
        p += 3 as c_int;
    }
}
unsafe extern "C" fn _addBeforePosition(
    mut p: c_float,
    mut temp: *mut c_float,
    mut i: c_int,
    mut out: *mut c_float,
    mut o: c_int,
) {
    let mut x1: c_float = *temp.offset(i as isize);
    let mut y1: c_float = *temp.offset((i + 1 as c_int) as isize);
    let mut dx: c_float = *temp.offset((i + 2 as c_int) as isize) - x1;
    let mut dy: c_float = *temp.offset((i + 3 as c_int) as isize) - y1;
    let mut r: c_float = atan2f(dy, dx);
    *out.offset(o as isize) = x1 + p * cosf(r);
    *out.offset((o + 1 as c_int) as isize) = y1 + p * sinf(r);
    *out.offset((o + 2 as c_int) as isize) = r;
}
unsafe extern "C" fn _addAfterPosition(
    mut p: c_float,
    mut temp: *mut c_float,
    mut i: c_int,
    mut out: *mut c_float,
    mut o: c_int,
) {
    let mut x1: c_float = *temp.offset((i + 2 as c_int) as isize);
    let mut y1: c_float = *temp.offset((i + 3 as c_int) as isize);
    let mut dx: c_float = x1 - *temp.offset(i as isize);
    let mut dy: c_float = y1 - *temp.offset((i + 1 as c_int) as isize);
    let mut r: c_float = atan2f(dy, dx);
    *out.offset(o as isize) = x1 + p * cosf(r);
    *out.offset((o + 1 as c_int) as isize) = y1 + p * sinf(r);
    *out.offset((o + 2 as c_int) as isize) = r;
}
unsafe extern "C" fn _addCurvePosition(
    mut p: c_float,
    mut x1: c_float,
    mut y1: c_float,
    mut cx1: c_float,
    mut cy1: c_float,
    mut cx2: c_float,
    mut cy2: c_float,
    mut x2: c_float,
    mut y2: c_float,
    mut out: *mut c_float,
    mut o: c_int,
    mut tangents: c_int,
) {
    let mut tt: c_float = 0.;
    let mut ttt: c_float = 0.;
    let mut u: c_float = 0.;
    let mut uu: c_float = 0.;
    let mut uuu: c_float = 0.;
    let mut ut: c_float = 0.;
    let mut ut3: c_float = 0.;
    let mut uut3: c_float = 0.;
    let mut utt3: c_float = 0.;
    let mut x: c_float = 0.;
    let mut y: c_float = 0.;
    if p == 0 as c_int as c_float || p.is_nan() as i32 != 0 {
        *out.offset(o as isize) = x1;
        *out.offset((o + 1 as c_int) as isize) = y1;
        *out.offset((o + 2 as c_int) as isize) = atan2f(cy1 - y1, cx1 - x1);
        return;
    }
    tt = p * p;
    ttt = tt * p;
    u = 1 as c_int as c_float - p;
    uu = u * u;
    uuu = uu * u;
    ut = u * p;
    ut3 = ut * 3 as c_int as c_float;
    uut3 = u * ut3;
    utt3 = ut3 * p;
    x = x1 * uuu + cx1 * uut3 + cx2 * utt3 + x2 * ttt;
    y = y1 * uuu + cy1 * uut3 + cy2 * utt3 + y2 * ttt;
    *out.offset(o as isize) = x;
    *out.offset((o + 1 as c_int) as isize) = y;
    if tangents != 0 {
        if (p as c_double) < 0.001f64 {
            *out.offset((o + 2 as c_int) as isize) = atan2f(cy1 - y1, cx1 - x1);
        } else {
            *out.offset((o + 2 as c_int) as isize) = atan2f(
                y - (y1 * uu + cy1 * ut * 2 as c_int as c_float + cy2 * tt),
                x - (x1 * uu + cx1 * ut * 2 as c_int as c_float + cx2 * tt),
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn spPathConstraint_computeWorldPositions(
    mut self_0: *mut spPathConstraint,
    mut path: *mut spPathAttachment,
    mut spacesCount: c_int,
    mut tangents: c_int,
) -> *mut c_float {
    let mut i: c_int = 0;
    let mut o: c_int = 0;
    let mut w: c_int = 0;
    let mut curve: c_int = 0;
    let mut segment: c_int = 0;
    let mut closed: c_int = 0;
    let mut verticesLength: c_int = 0;
    let mut curveCount: c_int = 0;
    let mut prevCurve: c_int = 0;
    let mut out: *mut c_float = 0 as *mut c_float;
    let mut curves: *mut c_float = 0 as *mut c_float;
    let mut segments: *mut c_float = 0 as *mut c_float;
    let mut tmpx: c_float = 0.;
    let mut tmpy: c_float = 0.;
    let mut dddfx: c_float = 0.;
    let mut dddfy: c_float = 0.;
    let mut ddfx: c_float = 0.;
    let mut ddfy: c_float = 0.;
    let mut dfx: c_float = 0.;
    let mut dfy: c_float = 0.;
    let mut pathLength: c_float = 0.;
    let mut curveLength: c_float = 0.;
    let mut p: c_float = 0.;
    let mut x1: c_float = 0.;
    let mut y1: c_float = 0.;
    let mut cx1: c_float = 0.;
    let mut cy1: c_float = 0.;
    let mut cx2: c_float = 0.;
    let mut cy2: c_float = 0.;
    let mut x2: c_float = 0.;
    let mut y2: c_float = 0.;
    let mut multiplier: c_float = 0.;
    let mut target: *mut spSlot = (*self_0).target;
    let mut position: c_float = (*self_0).position;
    let mut spaces: *mut c_float = (*self_0).spaces;
    let mut world: *mut c_float = 0 as *mut c_float;
    if (*self_0).positionsCount != spacesCount * 3 as c_int + 2 as c_int {
        if !((*self_0).positions).is_null() {
            _spFree((*self_0).positions as *mut c_void);
        }
        (*self_0).positions = _spMalloc(
            (::core::mem::size_of::<c_float>() as c_ulong)
                .wrapping_mul((spacesCount * 3 as c_int + 2 as c_int) as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            7041 as c_int,
        ) as *mut c_float;
        (*self_0).positionsCount = spacesCount * 3 as c_int + 2 as c_int;
    }
    out = (*self_0).positions;
    closed = (*path).closed;
    verticesLength = (*path).super_0.worldVerticesLength;
    curveCount = verticesLength / 6 as c_int;
    prevCurve = -(1 as c_int);
    if (*path).constantSpeed == 0 {
        let mut lengths: *mut c_float = (*path).lengths;
        curveCount -= if closed != 0 { 1 as c_int } else { 2 as c_int };
        pathLength = *lengths.offset(curveCount as isize);
        if (*(*self_0).data).positionMode as c_uint == SP_POSITION_MODE_PERCENT as c_int as c_uint {
            position += pathLength;
        }
        match (*(*self_0).data).spacingMode as c_uint {
            2 => {
                multiplier = pathLength;
            }
            3 => {
                multiplier = pathLength / spacesCount as c_float;
            }
            _ => {
                multiplier = 1 as c_int as c_float;
            }
        }
        if (*self_0).worldCount != 8 as c_int {
            if !((*self_0).world).is_null() {
                _spFree((*self_0).world as *mut c_void);
            }
            (*self_0).world = _spMalloc(
                (::core::mem::size_of::<c_float>() as c_ulong).wrapping_mul(8 as c_int as c_ulong),
                b"spine.c\0" as *const u8 as *const c_char,
                7066 as c_int,
            ) as *mut c_float;
            (*self_0).worldCount = 8 as c_int;
        }
        world = (*self_0).world;
        let mut current_block_55: u64;
        i = 0 as c_int;
        o = 0 as c_int;
        curve = 0 as c_int;
        while i < spacesCount {
            let mut space: c_float = *spaces.offset(i as isize) * multiplier;
            position += space;
            p = position;
            if closed != 0 {
                p = fmodf(p, pathLength);
                if p < 0 as c_int as c_float {
                    p += pathLength;
                }
                curve = 0 as c_int;
                current_block_55 = 18435049525520518667;
            } else if p < 0 as c_int as c_float {
                if prevCurve != -(2 as c_int) {
                    prevCurve = -(2 as c_int);
                    spVertexAttachment_computeWorldVertices(
                        &mut (*path).super_0,
                        target,
                        2 as c_int,
                        4 as c_int,
                        world,
                        0 as c_int,
                        2 as c_int,
                    );
                }
                _addBeforePosition(p, world, 0 as c_int, out, o);
                current_block_55 = 10043043949733653460;
            } else if p > pathLength {
                if prevCurve != -(3 as c_int) {
                    prevCurve = -(3 as c_int);
                    spVertexAttachment_computeWorldVertices(
                        &mut (*path).super_0,
                        target,
                        verticesLength - 6 as c_int,
                        4 as c_int,
                        world,
                        0 as c_int,
                        2 as c_int,
                    );
                }
                _addAfterPosition(p - pathLength, world, 0 as c_int, out, o);
                current_block_55 = 10043043949733653460;
            } else {
                current_block_55 = 18435049525520518667;
            }
            match current_block_55 {
                18435049525520518667 => {
                    loop {
                        let mut length: c_float = *lengths.offset(curve as isize);
                        if p > length {
                            curve += 1;
                        } else {
                            if curve == 0 as c_int {
                                p /= length;
                            } else {
                                let mut prev: c_float =
                                    *lengths.offset((curve - 1 as c_int) as isize);
                                p = (p - prev) / (length - prev);
                            }
                            break;
                        }
                    }
                    if curve != prevCurve {
                        prevCurve = curve;
                        if closed != 0 && curve == curveCount {
                            spVertexAttachment_computeWorldVertices(
                                &mut (*path).super_0,
                                target,
                                verticesLength - 4 as c_int,
                                4 as c_int,
                                world,
                                0 as c_int,
                                2 as c_int,
                            );
                            spVertexAttachment_computeWorldVertices(
                                &mut (*path).super_0,
                                target,
                                0 as c_int,
                                4 as c_int,
                                world,
                                4 as c_int,
                                2 as c_int,
                            );
                        } else {
                            spVertexAttachment_computeWorldVertices(
                                &mut (*path).super_0,
                                target,
                                curve * 6 as c_int + 2 as c_int,
                                8 as c_int,
                                world,
                                0 as c_int,
                                2 as c_int,
                            );
                        }
                    }
                    _addCurvePosition(
                        p,
                        *world.offset(0 as c_int as isize),
                        *world.offset(1 as c_int as isize),
                        *world.offset(2 as c_int as isize),
                        *world.offset(3 as c_int as isize),
                        *world.offset(4 as c_int as isize),
                        *world.offset(5 as c_int as isize),
                        *world.offset(6 as c_int as isize),
                        *world.offset(7 as c_int as isize),
                        out,
                        o,
                        (tangents != 0 || i > 0 as c_int && space == 0 as c_int as c_float)
                            as c_int,
                    );
                }
                _ => {}
            }
            i += 1;
            o += 3 as c_int;
        }
        return out;
    }
    if closed != 0 {
        verticesLength += 2 as c_int;
        if (*self_0).worldCount != verticesLength {
            if !((*self_0).world).is_null() {
                _spFree((*self_0).world as *mut c_void);
            }
            (*self_0).world = _spMalloc(
                (::core::mem::size_of::<c_float>() as c_ulong)
                    .wrapping_mul(verticesLength as c_ulong),
                b"spine.c\0" as *const u8 as *const c_char,
                7126 as c_int,
            ) as *mut c_float;
            (*self_0).worldCount = verticesLength;
        }
        world = (*self_0).world;
        spVertexAttachment_computeWorldVertices(
            &mut (*path).super_0,
            target,
            2 as c_int,
            verticesLength - 4 as c_int,
            world,
            0 as c_int,
            2 as c_int,
        );
        spVertexAttachment_computeWorldVertices(
            &mut (*path).super_0,
            target,
            0 as c_int,
            2 as c_int,
            world,
            verticesLength - 4 as c_int,
            2 as c_int,
        );
        *world.offset((verticesLength - 2 as c_int) as isize) = *world.offset(0 as c_int as isize);
        *world.offset((verticesLength - 1 as c_int) as isize) = *world.offset(1 as c_int as isize);
    } else {
        curveCount -= 1;
        verticesLength -= 4 as c_int;
        if (*self_0).worldCount != verticesLength {
            if !((*self_0).world).is_null() {
                _spFree((*self_0).world as *mut c_void);
            }
            (*self_0).world = _spMalloc(
                (::core::mem::size_of::<c_float>() as c_ulong)
                    .wrapping_mul(verticesLength as c_ulong),
                b"spine.c\0" as *const u8 as *const c_char,
                7139 as c_int,
            ) as *mut c_float;
            (*self_0).worldCount = verticesLength;
        }
        world = (*self_0).world;
        spVertexAttachment_computeWorldVertices(
            &mut (*path).super_0,
            target,
            2 as c_int,
            verticesLength,
            world,
            0 as c_int,
            2 as c_int,
        );
    }
    if (*self_0).curvesCount != curveCount {
        if !((*self_0).curves).is_null() {
            _spFree((*self_0).curves as *mut c_void);
        }
        (*self_0).curves = _spMalloc(
            (::core::mem::size_of::<c_float>() as c_ulong).wrapping_mul(curveCount as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            7149 as c_int,
        ) as *mut c_float;
        (*self_0).curvesCount = curveCount;
    }
    curves = (*self_0).curves;
    pathLength = 0 as c_int as c_float;
    x1 = *world.offset(0 as c_int as isize);
    y1 = *world.offset(1 as c_int as isize);
    cx1 = 0 as c_int as c_float;
    cy1 = 0 as c_int as c_float;
    cx2 = 0 as c_int as c_float;
    cy2 = 0 as c_int as c_float;
    x2 = 0 as c_int as c_float;
    y2 = 0 as c_int as c_float;
    i = 0 as c_int;
    w = 2 as c_int;
    while i < curveCount {
        cx1 = *world.offset(w as isize);
        cy1 = *world.offset((w + 1 as c_int) as isize);
        cx2 = *world.offset((w + 2 as c_int) as isize);
        cy2 = *world.offset((w + 3 as c_int) as isize);
        x2 = *world.offset((w + 4 as c_int) as isize);
        y2 = *world.offset((w + 5 as c_int) as isize);
        tmpx = (x1 - cx1 * 2 as c_int as c_float + cx2) * 0.1875f32;
        tmpy = (y1 - cy1 * 2 as c_int as c_float + cy2) * 0.1875f32;
        dddfx = ((cx1 - cx2) * 3 as c_int as c_float - x1 + x2) * 0.09375f32;
        dddfy = ((cy1 - cy2) * 3 as c_int as c_float - y1 + y2) * 0.09375f32;
        ddfx = tmpx * 2 as c_int as c_float + dddfx;
        ddfy = tmpy * 2 as c_int as c_float + dddfy;
        dfx = (cx1 - x1) * 0.75f32 + tmpx + dddfx * 0.16666667f32;
        dfy = (cy1 - y1) * 0.75f32 + tmpy + dddfy * 0.16666667f32;
        pathLength += spine_sqrtf(dfx * dfx + dfy * dfy);
        dfx += ddfx;
        dfy += ddfy;
        ddfx += dddfx;
        ddfy += dddfy;
        pathLength += spine_sqrtf(dfx * dfx + dfy * dfy);
        dfx += ddfx;
        dfy += ddfy;
        pathLength += spine_sqrtf(dfx * dfx + dfy * dfy);
        dfx += ddfx + dddfx;
        dfy += ddfy + dddfy;
        pathLength += spine_sqrtf(dfx * dfx + dfy * dfy);
        *curves.offset(i as isize) = pathLength;
        x1 = x2;
        y1 = y2;
        i += 1;
        w += 6 as c_int;
    }
    if (*(*self_0).data).positionMode as c_uint == SP_POSITION_MODE_PERCENT as c_int as c_uint {
        position *= pathLength;
    }
    match (*(*self_0).data).spacingMode as c_uint {
        2 => {
            multiplier = pathLength;
        }
        3 => {
            multiplier = pathLength / spacesCount as c_float;
        }
        _ => {
            multiplier = 1 as c_int as c_float;
        }
    }
    segments = ((*self_0).segments).as_mut_ptr();
    curveLength = 0 as c_int as c_float;
    let mut current_block_195: u64;
    i = 0 as c_int;
    o = 0 as c_int;
    curve = 0 as c_int;
    segment = 0 as c_int;
    while i < spacesCount {
        let mut space_0: c_float = *spaces.offset(i as isize) * multiplier;
        position += space_0;
        p = position;
        if closed != 0 {
            p = fmodf(p, pathLength);
            if p < 0 as c_int as c_float {
                p += pathLength;
            }
            curve = 0 as c_int;
            current_block_195 = 15456862084301247793;
        } else if p < 0 as c_int as c_float {
            _addBeforePosition(p, world, 0 as c_int, out, o);
            current_block_195 = 5404178929002277151;
        } else if p > pathLength {
            _addAfterPosition(p - pathLength, world, verticesLength - 4 as c_int, out, o);
            current_block_195 = 5404178929002277151;
        } else {
            current_block_195 = 15456862084301247793;
        }
        match current_block_195 {
            15456862084301247793 => {
                loop {
                    let mut length_0: c_float = *curves.offset(curve as isize);
                    if p > length_0 {
                        curve += 1;
                    } else {
                        if curve == 0 as c_int {
                            p /= length_0;
                        } else {
                            let mut prev_0: c_float = *curves.offset((curve - 1 as c_int) as isize);
                            p = (p - prev_0) / (length_0 - prev_0);
                        }
                        break;
                    }
                }
                if curve != prevCurve {
                    let mut ii: c_int = 0;
                    prevCurve = curve;
                    ii = curve * 6 as c_int;
                    x1 = *world.offset(ii as isize);
                    y1 = *world.offset((ii + 1 as c_int) as isize);
                    cx1 = *world.offset((ii + 2 as c_int) as isize);
                    cy1 = *world.offset((ii + 3 as c_int) as isize);
                    cx2 = *world.offset((ii + 4 as c_int) as isize);
                    cy2 = *world.offset((ii + 5 as c_int) as isize);
                    x2 = *world.offset((ii + 6 as c_int) as isize);
                    y2 = *world.offset((ii + 7 as c_int) as isize);
                    tmpx = (x1 - cx1 * 2 as c_int as c_float + cx2) * 0.03f32;
                    tmpy = (y1 - cy1 * 2 as c_int as c_float + cy2) * 0.03f32;
                    dddfx = ((cx1 - cx2) * 3 as c_int as c_float - x1 + x2) * 0.006f32;
                    dddfy = ((cy1 - cy2) * 3 as c_int as c_float - y1 + y2) * 0.006f32;
                    ddfx = tmpx * 2 as c_int as c_float + dddfx;
                    ddfy = tmpy * 2 as c_int as c_float + dddfy;
                    dfx = (cx1 - x1) * 0.3f32 + tmpx + dddfx * 0.16666667f32;
                    dfy = (cy1 - y1) * 0.3f32 + tmpy + dddfy * 0.16666667f32;
                    curveLength = spine_sqrtf(dfx * dfx + dfy * dfy);
                    *segments.offset(0 as c_int as isize) = curveLength;
                    ii = 1 as c_int;
                    while ii < 8 as c_int {
                        dfx += ddfx;
                        dfy += ddfy;
                        ddfx += dddfx;
                        ddfy += dddfy;
                        curveLength += spine_sqrtf(dfx * dfx + dfy * dfy);
                        *segments.offset(ii as isize) = curveLength;
                        ii += 1;
                    }
                    dfx += ddfx;
                    dfy += ddfy;
                    curveLength += spine_sqrtf(dfx * dfx + dfy * dfy);
                    *segments.offset(8 as c_int as isize) = curveLength;
                    dfx += ddfx + dddfx;
                    dfy += ddfy + dddfy;
                    curveLength += spine_sqrtf(dfx * dfx + dfy * dfy);
                    *segments.offset(9 as c_int as isize) = curveLength;
                    segment = 0 as c_int;
                }
                p *= curveLength;
                loop {
                    let mut length_1: c_float = *segments.offset(segment as isize);
                    if p > length_1 {
                        segment += 1;
                    } else {
                        if segment == 0 as c_int {
                            p /= length_1;
                        } else {
                            let mut prev_1: c_float =
                                *segments.offset((segment - 1 as c_int) as isize);
                            p = segment as c_float + (p - prev_1) / (length_1 - prev_1);
                        }
                        break;
                    }
                }
                _addCurvePosition(
                    p * 0.1f32,
                    x1,
                    y1,
                    cx1,
                    cy1,
                    cx2,
                    cy2,
                    x2,
                    y2,
                    out,
                    o,
                    (tangents != 0 || i > 0 as c_int && space_0 == 0 as c_int as c_float) as c_int,
                );
            }
            _ => {}
        }
        i += 1;
        o += 3 as c_int;
    }
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn spPathConstraintData_create(
    mut name: *const c_char,
) -> *mut spPathConstraintData {
    let mut self_0: *mut spPathConstraintData = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spPathConstraintData>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        7324 as c_int,
    ) as *mut spPathConstraintData;
    let ref mut fresh81 = *(&(*self_0).name as *const *const c_char as *mut *mut c_char);
    *fresh81 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul((spine_strlen(name)).wrapping_add(1 as c_int as c_ulong)),
        b"spine.c\0" as *const u8 as *const c_char,
        7325 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh81, name);
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spPathConstraintData_dispose(mut self_0: *mut spPathConstraintData) {
    _spFree((*self_0).name as *mut c_void);
    _spFree((*self_0).bones as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _spPointAttachment_dispose(mut attachment: *mut spAttachment) {
    let mut self_0: *mut spPointAttachment = attachment as *mut spPointAttachment;
    _spAttachment_deinit(attachment);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _spPointAttachment_copy(
    mut attachment: *mut spAttachment,
) -> *mut spAttachment {
    let mut self_0: *mut spPointAttachment = attachment as *mut spPointAttachment;
    let mut copy: *mut spPointAttachment = spPointAttachment_create((*attachment).name);
    (*copy).x = (*self_0).x;
    (*copy).y = (*self_0).y;
    (*copy).rotation = (*self_0).rotation;
    spColor_setFromColor(&mut (*copy).color, &mut (*self_0).color);
    return &mut (*copy).super_0;
}
#[no_mangle]
pub unsafe extern "C" fn spPointAttachment_create(
    mut name: *const c_char,
) -> *mut spPointAttachment {
    let mut self_0: *mut spPointAttachment = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spPointAttachment>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        7383 as c_int,
    ) as *mut spPointAttachment;
    _spAttachment_init(
        &mut (*self_0).super_0,
        name,
        SP_ATTACHMENT_POINT,
        Some(_spPointAttachment_dispose as unsafe extern "C" fn(*mut spAttachment) -> ()),
        Some(
            _spPointAttachment_copy as unsafe extern "C" fn(*mut spAttachment) -> *mut spAttachment,
        ),
    );
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spPointAttachment_computeWorldPosition(
    mut self_0: *mut spPointAttachment,
    mut bone: *mut spBone,
    mut x: *mut c_float,
    mut y: *mut c_float,
) {
    *x = (*self_0).x * (*bone).a + (*self_0).y * (*bone).b + (*bone).worldX;
    *y = (*self_0).x * (*bone).c + (*self_0).y * (*bone).d + (*bone).worldY;
}
#[no_mangle]
pub unsafe extern "C" fn spPointAttachment_computeWorldRotation(
    mut self_0: *mut spPointAttachment,
    mut bone: *mut spBone,
) -> c_float {
    let mut cosine: c_float = 0.;
    let mut sine: c_float = 0.;
    let mut x: c_float = 0.;
    let mut y: c_float = 0.;
    cosine = cosf((*self_0).rotation * (3.1415926535897932385f32 / 180 as c_int as c_float));
    sine = sinf((*self_0).rotation * (3.1415926535897932385f32 / 180 as c_int as c_float));
    x = cosine * (*bone).a + sine * (*bone).b;
    y = cosine * (*bone).c + sine * (*bone).d;
    return atan2f(y, x) * (180 as c_int as c_float / 3.1415926535897932385f32);
}
#[no_mangle]
pub unsafe extern "C" fn _spRegionAttachment_dispose(mut attachment: *mut spAttachment) {
    let mut self_0: *mut spRegionAttachment = attachment as *mut spRegionAttachment;
    if !((*self_0).sequence).is_null() {
        spSequence_dispose((*self_0).sequence);
    }
    _spAttachment_deinit(attachment);
    _spFree((*self_0).path as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _spRegionAttachment_copy(
    mut attachment: *mut spAttachment,
) -> *mut spAttachment {
    let mut self_0: *mut spRegionAttachment = attachment as *mut spRegionAttachment;
    let mut copy: *mut spRegionAttachment = spRegionAttachment_create((*attachment).name);
    (*copy).region = (*self_0).region;
    (*copy).rendererObject = (*self_0).rendererObject;
    let ref mut fresh82 = *(&mut (*copy).path as *mut *const c_char as *mut *mut c_char);
    *fresh82 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul((spine_strlen((*self_0).path)).wrapping_add(1 as c_int as c_ulong)),
        b"spine.c\0" as *const u8 as *const c_char,
        7457 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh82, (*self_0).path);
    (*copy).x = (*self_0).x;
    (*copy).y = (*self_0).y;
    (*copy).scaleX = (*self_0).scaleX;
    (*copy).scaleY = (*self_0).scaleY;
    (*copy).rotation = (*self_0).rotation;
    (*copy).width = (*self_0).width;
    (*copy).height = (*self_0).height;
    spine_memcpy(
        ((*copy).uvs).as_mut_ptr() as *mut c_void,
        ((*self_0).uvs).as_mut_ptr() as *const c_void,
        (::core::mem::size_of::<c_float>() as c_ulong).wrapping_mul(8 as c_int as c_ulong),
    );
    spine_memcpy(
        ((*copy).offset).as_mut_ptr() as *mut c_void,
        ((*self_0).offset).as_mut_ptr() as *const c_void,
        (::core::mem::size_of::<c_float>() as c_ulong).wrapping_mul(8 as c_int as c_ulong),
    );
    spColor_setFromColor(&mut (*copy).color, &mut (*self_0).color);
    (*copy).sequence = if !((*self_0).sequence).is_null() {
        spSequence_copy((*self_0).sequence)
    } else {
        0 as *mut spSequence
    };
    return &mut (*copy).super_0;
}
#[no_mangle]
pub unsafe extern "C" fn spRegionAttachment_create(
    mut name: *const c_char,
) -> *mut spRegionAttachment {
    let mut self_0: *mut spRegionAttachment = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spRegionAttachment>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        7473 as c_int,
    ) as *mut spRegionAttachment;
    (*self_0).scaleX = 1 as c_int as c_float;
    (*self_0).scaleY = 1 as c_int as c_float;
    spColor_setFromFloats(
        &mut (*self_0).color,
        1 as c_int as c_float,
        1 as c_int as c_float,
        1 as c_int as c_float,
        1 as c_int as c_float,
    );
    _spAttachment_init(
        &mut (*self_0).super_0,
        name,
        SP_ATTACHMENT_REGION,
        Some(_spRegionAttachment_dispose as unsafe extern "C" fn(*mut spAttachment) -> ()),
        Some(
            _spRegionAttachment_copy
                as unsafe extern "C" fn(*mut spAttachment) -> *mut spAttachment,
        ),
    );
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spRegionAttachment_updateRegion(mut self_0: *mut spRegionAttachment) {
    let mut regionScaleX: c_float =
        (*self_0).width / (*(*self_0).region).originalWidth as c_float * (*self_0).scaleX;
    let mut regionScaleY: c_float =
        (*self_0).height / (*(*self_0).region).originalHeight as c_float * (*self_0).scaleY;
    let mut localX: c_float = -(*self_0).width / 2 as c_int as c_float * (*self_0).scaleX
        + (*(*self_0).region).offsetX * regionScaleX;
    let mut localY: c_float = -(*self_0).height / 2 as c_int as c_float * (*self_0).scaleY
        + (*(*self_0).region).offsetY * regionScaleY;
    let mut localX2: c_float = localX + (*(*self_0).region).width as c_float * regionScaleX;
    let mut localY2: c_float = localY + (*(*self_0).region).height as c_float * regionScaleY;
    let mut radians: c_float =
        (*self_0).rotation * (3.1415926535897932385f32 / 180 as c_int as c_float);
    let mut cosine: c_float = cosf(radians);
    let mut sine: c_float = sinf(radians);
    let mut localXCos: c_float = localX * cosine + (*self_0).x;
    let mut localXSin: c_float = localX * sine;
    let mut localYCos: c_float = localY * cosine + (*self_0).y;
    let mut localYSin: c_float = localY * sine;
    let mut localX2Cos: c_float = localX2 * cosine + (*self_0).x;
    let mut localX2Sin: c_float = localX2 * sine;
    let mut localY2Cos: c_float = localY2 * cosine + (*self_0).y;
    let mut localY2Sin: c_float = localY2 * sine;
    (*self_0).offset[BLX as c_int as usize] = localXCos - localYSin;
    (*self_0).offset[BLY as c_int as usize] = localYCos + localXSin;
    (*self_0).offset[ULX as c_int as usize] = localXCos - localY2Sin;
    (*self_0).offset[ULY as c_int as usize] = localY2Cos + localXSin;
    (*self_0).offset[URX as c_int as usize] = localX2Cos - localY2Sin;
    (*self_0).offset[URY as c_int as usize] = localY2Cos + localX2Sin;
    (*self_0).offset[BRX as c_int as usize] = localX2Cos - localYSin;
    (*self_0).offset[BRY as c_int as usize] = localYCos + localX2Sin;
    if (*(*self_0).region).degrees == 90 as c_int {
        (*self_0).uvs[URX as c_int as usize] = (*(*self_0).region).u;
        (*self_0).uvs[URY as c_int as usize] = (*(*self_0).region).v2;
        (*self_0).uvs[BRX as c_int as usize] = (*(*self_0).region).u;
        (*self_0).uvs[BRY as c_int as usize] = (*(*self_0).region).v;
        (*self_0).uvs[BLX as c_int as usize] = (*(*self_0).region).u2;
        (*self_0).uvs[BLY as c_int as usize] = (*(*self_0).region).v;
        (*self_0).uvs[ULX as c_int as usize] = (*(*self_0).region).u2;
        (*self_0).uvs[ULY as c_int as usize] = (*(*self_0).region).v2;
    } else {
        (*self_0).uvs[ULX as c_int as usize] = (*(*self_0).region).u;
        (*self_0).uvs[ULY as c_int as usize] = (*(*self_0).region).v2;
        (*self_0).uvs[URX as c_int as usize] = (*(*self_0).region).u;
        (*self_0).uvs[URY as c_int as usize] = (*(*self_0).region).v;
        (*self_0).uvs[BRX as c_int as usize] = (*(*self_0).region).u2;
        (*self_0).uvs[BRY as c_int as usize] = (*(*self_0).region).v;
        (*self_0).uvs[BLX as c_int as usize] = (*(*self_0).region).u2;
        (*self_0).uvs[BLY as c_int as usize] = (*(*self_0).region).v2;
    };
}
#[no_mangle]
pub unsafe extern "C" fn spRegionAttachment_computeWorldVertices(
    mut self_0: *mut spRegionAttachment,
    mut slot: *mut spSlot,
    mut vertices: *mut c_float,
    mut offset: c_int,
    mut stride: c_int,
) {
    let mut offsets: *const c_float = ((*self_0).offset).as_mut_ptr();
    let mut bone: *mut spBone = (*slot).bone;
    let mut x: c_float = (*bone).worldX;
    let mut y: c_float = (*bone).worldY;
    let mut offsetX: c_float = 0.;
    let mut offsetY: c_float = 0.;
    if !((*self_0).sequence).is_null() {
        spSequence_apply((*self_0).sequence, slot, &mut (*self_0).super_0);
    }
    offsetX = *offsets.offset(BRX as c_int as isize);
    offsetY = *offsets.offset(BRY as c_int as isize);
    *vertices.offset(offset as isize) = offsetX * (*bone).a + offsetY * (*bone).b + x;
    *vertices.offset((offset + 1 as c_int) as isize) =
        offsetX * (*bone).c + offsetY * (*bone).d + y;
    offset += stride;
    offsetX = *offsets.offset(BLX as c_int as isize);
    offsetY = *offsets.offset(BLY as c_int as isize);
    *vertices.offset(offset as isize) = offsetX * (*bone).a + offsetY * (*bone).b + x;
    *vertices.offset((offset + 1 as c_int) as isize) =
        offsetX * (*bone).c + offsetY * (*bone).d + y;
    offset += stride;
    offsetX = *offsets.offset(ULX as c_int as isize);
    offsetY = *offsets.offset(ULY as c_int as isize);
    *vertices.offset(offset as isize) = offsetX * (*bone).a + offsetY * (*bone).b + x;
    *vertices.offset((offset + 1 as c_int) as isize) =
        offsetX * (*bone).c + offsetY * (*bone).d + y;
    offset += stride;
    offsetX = *offsets.offset(URX as c_int as isize);
    offsetY = *offsets.offset(URY as c_int as isize);
    *vertices.offset(offset as isize) = offsetX * (*bone).a + offsetY * (*bone).b + x;
    *vertices.offset((offset + 1 as c_int) as isize) =
        offsetX * (*bone).c + offsetY * (*bone).d + y;
}
#[no_mangle]
pub unsafe extern "C" fn spTextureRegionArray_create(
    mut initialCapacity: c_int,
) -> *mut spTextureRegionArray {
    let mut array: *mut spTextureRegionArray = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spTextureRegionArray>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        7593 as c_int,
    ) as *mut spTextureRegionArray;
    (*array).size = 0 as c_int;
    (*array).capacity = initialCapacity;
    (*array).items = _spCalloc(
        initialCapacity as size_t,
        ::core::mem::size_of::<*mut spTextureRegion>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        7593 as c_int,
    ) as *mut *mut spTextureRegion;
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn spTextureRegionArray_dispose(mut self_0: *mut spTextureRegionArray) {
    _spFree((*self_0).items as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spTextureRegionArray_clear(mut self_0: *mut spTextureRegionArray) {
    (*self_0).size = 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spTextureRegionArray_setSize(
    mut self_0: *mut spTextureRegionArray,
    mut newSize: c_int,
) -> *mut spTextureRegionArray {
    (*self_0).size = newSize;
    if (*self_0).capacity < newSize {
        (*self_0).capacity = if 8 as c_int > ((*self_0).size as c_float * 1.75f32) as c_int {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0).items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<*mut spTextureRegion>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut *mut spTextureRegion;
    }
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spTextureRegionArray_ensureCapacity(
    mut self_0: *mut spTextureRegionArray,
    mut newCapacity: c_int,
) {
    if (*self_0).capacity >= newCapacity {
        return;
    }
    (*self_0).capacity = newCapacity;
    (*self_0).items = _spRealloc(
        (*self_0).items as *mut c_void,
        (::core::mem::size_of::<*mut spTextureRegion>() as c_ulong)
            .wrapping_mul((*self_0).capacity as c_ulong),
    ) as *mut *mut spTextureRegion;
}
#[no_mangle]
pub unsafe extern "C" fn spTextureRegionArray_add(
    mut self_0: *mut spTextureRegionArray,
    mut value: *mut spTextureRegion,
) {
    if (*self_0).size == (*self_0).capacity {
        (*self_0).capacity = if 8 as c_int > ((*self_0).size as c_float * 1.75f32) as c_int {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0).items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<*mut spTextureRegion>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut *mut spTextureRegion;
    }
    let fresh83 = (*self_0).size;
    (*self_0).size = (*self_0).size + 1;
    let ref mut fresh84 = *((*self_0).items).offset(fresh83 as isize);
    *fresh84 = value;
}
#[no_mangle]
pub unsafe extern "C" fn spTextureRegionArray_addAll(
    mut self_0: *mut spTextureRegionArray,
    mut other: *mut spTextureRegionArray,
) {
    let mut i: c_int = 0 as c_int;
    while i < (*other).size {
        spTextureRegionArray_add(self_0, *((*other).items).offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spTextureRegionArray_addAllValues(
    mut self_0: *mut spTextureRegionArray,
    mut values: *mut *mut spTextureRegion,
    mut offset: c_int,
    mut count: c_int,
) {
    let mut i: c_int = offset;
    let mut n: c_int = offset + count;
    while i < n {
        spTextureRegionArray_add(self_0, *values.offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spTextureRegionArray_removeAt(
    mut self_0: *mut spTextureRegionArray,
    mut index: c_int,
) {
    (*self_0).size -= 1;
    spine_memmove(
        ((*self_0).items).offset(index as isize) as *mut c_void,
        ((*self_0).items)
            .offset(index as isize)
            .offset(1 as c_int as isize) as *const c_void,
        (::core::mem::size_of::<*mut spTextureRegion>() as c_ulong)
            .wrapping_mul(((*self_0).size - index) as c_ulong),
    );
}
#[no_mangle]
pub unsafe extern "C" fn spTextureRegionArray_contains(
    mut self_0: *mut spTextureRegionArray,
    mut value: *mut spTextureRegion,
) -> c_int {
    let mut items: *mut *mut spTextureRegion = (*self_0).items;
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    i = 0 as c_int;
    n = (*self_0).size;
    while i < n {
        if *items.offset(i as isize) == value {
            return -(1 as c_int);
        }
        i += 1;
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spTextureRegionArray_pop(
    mut self_0: *mut spTextureRegionArray,
) -> *mut spTextureRegion {
    (*self_0).size -= 1;
    let mut item: *mut spTextureRegion = *((*self_0).items).offset((*self_0).size as isize);
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn spTextureRegionArray_peek(
    mut self_0: *mut spTextureRegionArray,
) -> *mut spTextureRegion {
    return *((*self_0).items).offset(((*self_0).size - 1 as c_int) as isize);
}
static mut nextSequenceId: c_int = 0 as c_int;
#[no_mangle]
pub unsafe extern "C" fn spSequence_create(mut numRegions: c_int) -> *mut spSequence {
    let mut self_0: *mut spSequence = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spSequence>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        7598 as c_int,
    ) as *mut spSequence;
    let fresh85 = nextSequenceId;
    nextSequenceId = nextSequenceId + 1;
    (*self_0).id = fresh85;
    (*self_0).regions = spTextureRegionArray_create(numRegions);
    spTextureRegionArray_setSize((*self_0).regions, numRegions);
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spSequence_dispose(mut self_0: *mut spSequence) {
    _spFree((*self_0).regions as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spSequence_copy(mut self_0: *mut spSequence) -> *mut spSequence {
    let mut i: c_int = 0 as c_int;
    let mut copy: *mut spSequence = spSequence_create((*(*self_0).regions).size);
    while i < (*(*self_0).regions).size {
        let ref mut fresh86 = *((*(*copy).regions).items).offset(i as isize);
        *fresh86 = *((*(*self_0).regions).items).offset(i as isize);
        i += 1;
    }
    (*copy).start = (*self_0).start;
    (*copy).digits = (*self_0).digits;
    (*copy).setupIndex = (*self_0).setupIndex;
    return copy;
}
#[no_mangle]
pub unsafe extern "C" fn spSequence_apply(
    mut self_0: *mut spSequence,
    mut slot: *mut spSlot,
    mut attachment: *mut spAttachment,
) {
    let mut index: c_int = (*slot).sequenceIndex;
    let mut region: *mut spTextureRegion = 0 as *mut spTextureRegion;
    if index == -(1 as c_int) {
        index = (*self_0).setupIndex;
    }
    if index >= (*(*self_0).regions).size {
        index = (*(*self_0).regions).size - 1 as c_int;
    }
    region = *((*(*self_0).regions).items).offset(index as isize);
    if (*attachment).type_0 as c_uint == SP_ATTACHMENT_REGION as c_int as c_uint {
        let mut regionAttachment: *mut spRegionAttachment = attachment as *mut spRegionAttachment;
        if (*regionAttachment).region != region {
            (*regionAttachment).rendererObject = region as *mut c_void;
            (*regionAttachment).region = region;
            spRegionAttachment_updateRegion(regionAttachment);
        }
    }
    if (*attachment).type_0 as c_uint == SP_ATTACHMENT_MESH as c_int as c_uint {
        let mut meshAttachment: *mut spMeshAttachment = attachment as *mut spMeshAttachment;
        if (*meshAttachment).region != region {
            (*meshAttachment).rendererObject = region as *mut c_void;
            (*meshAttachment).region = region;
            spMeshAttachment_updateRegion(meshAttachment);
        }
    }
}
unsafe extern "C" fn num_digits(mut value: c_int) -> c_int {
    let mut count: c_int = if value < 0 as c_int {
        1 as c_int
    } else {
        0 as c_int
    };
    loop {
        value /= 10 as c_int;
        count += 1;
        if !(value != 0 as c_int) {
            break;
        }
    }
    return count;
}
unsafe extern "C" fn string_append(mut str: *mut c_char, mut b: *const c_char) -> *mut c_char {
    let mut lenB: c_int = spine_strlen(b) as c_int;
    spine_memcpy(
        str as *mut c_void,
        b as *const c_void,
        (lenB + 1 as c_int) as size_t,
    );
    return str.offset(lenB as isize);
}
unsafe extern "C" fn string_append_int(mut str: *mut c_char, mut value: c_int) -> *mut c_char {
    let mut intStr: [c_char; 20] = [0; 20];
    spine_sprintf!(
        intStr.as_mut_ptr(),
        b"%i\0" as *const u8 as *const c_char,
        value,
    );
    return string_append(str, intStr.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn spSequence_getPath(
    mut self_0: *mut spSequence,
    mut basePath: *const c_char,
    mut index: c_int,
    mut path: *mut c_char,
) {
    let mut i: c_int = 0;
    path = string_append(path, basePath);
    i = (*self_0).digits - num_digits((*self_0).start + index);
    while i > 0 as c_int {
        path = string_append(path, b"0\0" as *const u8 as *const c_char);
        i -= 1;
    }
    path = string_append_int(path, (*self_0).start + index);
}
#[no_mangle]
pub unsafe extern "C" fn spSkeleton_create(mut data: *mut spSkeletonData) -> *mut spSkeleton {
    let mut i: c_int = 0;
    let mut childrenCounts: *mut c_int = 0 as *mut c_int;
    let mut internal: *mut _spSkeleton = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<_spSkeleton>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        7733 as c_int,
    ) as *mut _spSkeleton;
    let mut self_0: *mut spSkeleton = &mut (*internal).super_0;
    let ref mut fresh87 =
        *(&(*self_0).data as *const *mut spSkeletonData as *mut *mut spSkeletonData);
    *fresh87 = data;
    (*self_0).bonesCount = (*(*self_0).data).bonesCount;
    (*self_0).bones = _spMalloc(
        (::core::mem::size_of::<*mut spBone>() as c_ulong)
            .wrapping_mul((*self_0).bonesCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        7738 as c_int,
    ) as *mut *mut spBone;
    childrenCounts = _spCalloc(
        (*self_0).bonesCount as size_t,
        ::core::mem::size_of::<c_int>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        7739 as c_int,
    ) as *mut c_int;
    i = 0 as c_int;
    while i < (*self_0).bonesCount {
        let mut boneData: *mut spBoneData = *((*(*self_0).data).bones).offset(i as isize);
        let mut newBone: *mut spBone = 0 as *mut spBone;
        if ((*boneData).parent).is_null() {
            newBone = spBone_create(boneData, self_0, 0 as *mut spBone);
        } else {
            let mut parent: *mut spBone =
                *((*self_0).bones).offset((*(*boneData).parent).index as isize);
            newBone = spBone_create(boneData, self_0, parent);
            let ref mut fresh88 = *childrenCounts.offset((*(*boneData).parent).index as isize);
            *fresh88 += 1;
        }
        let ref mut fresh89 = *((*self_0).bones).offset(i as isize);
        *fresh89 = newBone;
        i += 1;
    }
    i = 0 as c_int;
    while i < (*self_0).bonesCount {
        let mut boneData_0: *mut spBoneData = *((*(*self_0).data).bones).offset(i as isize);
        let mut bone: *mut spBone = *((*self_0).bones).offset(i as isize);
        let ref mut fresh90 =
            *(&(*bone).children as *const *mut *mut spBone as *mut *mut *mut spBone);
        *fresh90 = _spMalloc(
            (::core::mem::size_of::<*mut spBone>() as c_ulong)
                .wrapping_mul(*childrenCounts.offset((*boneData_0).index as isize) as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            7756 as c_int,
        ) as *mut *mut spBone;
        i += 1;
    }
    i = 0 as c_int;
    while i < (*self_0).bonesCount {
        let mut bone_0: *mut spBone = *((*self_0).bones).offset(i as isize);
        let mut parent_0: *mut spBone = (*bone_0).parent;
        if !parent_0.is_null() {
            let fresh91 = (*parent_0).childrenCount;
            (*parent_0).childrenCount = (*parent_0).childrenCount + 1;
            let ref mut fresh92 = *((*parent_0).children).offset(fresh91 as isize);
            *fresh92 = bone_0;
        }
        i += 1;
    }
    let ref mut fresh93 = *(&(*self_0).root as *const *mut spBone as *mut *mut spBone);
    *fresh93 = if (*self_0).bonesCount > 0 as c_int {
        *((*self_0).bones).offset(0 as c_int as isize)
    } else {
        0 as *mut spBone
    };
    (*self_0).slotsCount = (*data).slotsCount;
    (*self_0).slots = _spMalloc(
        (::core::mem::size_of::<*mut spSlot>() as c_ulong)
            .wrapping_mul((*self_0).slotsCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        7767 as c_int,
    ) as *mut *mut spSlot;
    i = 0 as c_int;
    while i < (*self_0).slotsCount {
        let mut slotData: *mut spSlotData = *((*data).slots).offset(i as isize);
        let mut bone_1: *mut spBone =
            *((*self_0).bones).offset((*(*slotData).boneData).index as isize);
        let ref mut fresh94 = *((*self_0).slots).offset(i as isize);
        *fresh94 = spSlot_create(slotData, bone_1);
        i += 1;
    }
    (*self_0).drawOrder = _spMalloc(
        (::core::mem::size_of::<*mut spSlot>() as c_ulong)
            .wrapping_mul((*self_0).slotsCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        7774 as c_int,
    ) as *mut *mut spSlot;
    spine_memcpy(
        (*self_0).drawOrder as *mut c_void,
        (*self_0).slots as *const c_void,
        (::core::mem::size_of::<*mut spSlot>() as c_ulong)
            .wrapping_mul((*self_0).slotsCount as c_ulong),
    );
    (*self_0).ikConstraintsCount = (*data).ikConstraintsCount;
    (*self_0).ikConstraints = _spMalloc(
        (::core::mem::size_of::<*mut spIkConstraint>() as c_ulong)
            .wrapping_mul((*self_0).ikConstraintsCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        7778 as c_int,
    ) as *mut *mut spIkConstraint;
    i = 0 as c_int;
    while i < (*(*self_0).data).ikConstraintsCount {
        let ref mut fresh95 = *((*self_0).ikConstraints).offset(i as isize);
        *fresh95 = spIkConstraint_create(
            *((*(*self_0).data).ikConstraints).offset(i as isize),
            self_0,
        );
        i += 1;
    }
    (*self_0).transformConstraintsCount = (*data).transformConstraintsCount;
    (*self_0).transformConstraints = _spMalloc(
        (::core::mem::size_of::<*mut spTransformConstraint>() as c_ulong)
            .wrapping_mul((*self_0).transformConstraintsCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        7783 as c_int,
    ) as *mut *mut spTransformConstraint;
    i = 0 as c_int;
    while i < (*(*self_0).data).transformConstraintsCount {
        let ref mut fresh96 = *((*self_0).transformConstraints).offset(i as isize);
        *fresh96 = spTransformConstraint_create(
            *((*(*self_0).data).transformConstraints).offset(i as isize),
            self_0,
        );
        i += 1;
    }
    (*self_0).pathConstraintsCount = (*data).pathConstraintsCount;
    (*self_0).pathConstraints = _spMalloc(
        (::core::mem::size_of::<*mut spPathConstraint>() as c_ulong)
            .wrapping_mul((*self_0).pathConstraintsCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        7788 as c_int,
    ) as *mut *mut spPathConstraint;
    i = 0 as c_int;
    while i < (*(*self_0).data).pathConstraintsCount {
        let ref mut fresh97 = *((*self_0).pathConstraints).offset(i as isize);
        *fresh97 = spPathConstraint_create(
            *((*(*self_0).data).pathConstraints).offset(i as isize),
            self_0,
        );
        i += 1;
    }
    spColor_setFromFloats(
        &mut (*self_0).color,
        1 as c_int as c_float,
        1 as c_int as c_float,
        1 as c_int as c_float,
        1 as c_int as c_float,
    );
    (*self_0).scaleX = 1 as c_int as c_float;
    (*self_0).scaleY = 1 as c_int as c_float;
    spSkeleton_updateCache(self_0);
    _spFree(childrenCounts as *mut c_void);
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeleton_dispose(mut self_0: *mut spSkeleton) {
    let mut i: c_int = 0;
    let mut internal: *mut _spSkeleton = self_0 as *mut _spSkeleton;
    _spFree((*internal).updateCache as *mut c_void);
    i = 0 as c_int;
    while i < (*self_0).bonesCount {
        spBone_dispose(*((*self_0).bones).offset(i as isize));
        i += 1;
    }
    _spFree((*self_0).bones as *mut c_void);
    i = 0 as c_int;
    while i < (*self_0).slotsCount {
        spSlot_dispose(*((*self_0).slots).offset(i as isize));
        i += 1;
    }
    _spFree((*self_0).slots as *mut c_void);
    i = 0 as c_int;
    while i < (*self_0).ikConstraintsCount {
        spIkConstraint_dispose(*((*self_0).ikConstraints).offset(i as isize));
        i += 1;
    }
    _spFree((*self_0).ikConstraints as *mut c_void);
    i = 0 as c_int;
    while i < (*self_0).transformConstraintsCount {
        spTransformConstraint_dispose(*((*self_0).transformConstraints).offset(i as isize));
        i += 1;
    }
    _spFree((*self_0).transformConstraints as *mut c_void);
    i = 0 as c_int;
    while i < (*self_0).pathConstraintsCount {
        spPathConstraint_dispose(*((*self_0).pathConstraints).offset(i as isize));
        i += 1;
    }
    _spFree((*self_0).pathConstraints as *mut c_void);
    _spFree((*self_0).drawOrder as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
unsafe extern "C" fn _addToUpdateCache(
    internal: *mut _spSkeleton,
    mut type_0: _spUpdateType,
    mut object: *mut c_void,
) {
    let mut update: *mut _spUpdate = 0 as *mut _spUpdate;
    if (*internal).updateCacheCount == (*internal).updateCacheCapacity {
        (*internal).updateCacheCapacity *= 2 as c_int;
        (*internal).updateCache = _spRealloc(
            (*internal).updateCache as *mut c_void,
            (::core::mem::size_of::<_spUpdate>() as c_ulong)
                .wrapping_mul((*internal).updateCacheCapacity as c_ulong),
        ) as *mut _spUpdate;
    }
    update = ((*internal).updateCache).offset((*internal).updateCacheCount as isize);
    (*update).type_0 = type_0;
    (*update).object = object;
    (*internal).updateCacheCount += 1;
}
unsafe extern "C" fn _sortBone(internal: *mut _spSkeleton, mut bone: *mut spBone) {
    if (*bone).sorted != 0 {
        return;
    }
    if !((*bone).parent).is_null() {
        _sortBone(internal, (*bone).parent);
    }
    (*bone).sorted = 1 as c_int;
    _addToUpdateCache(internal, SP_UPDATE_BONE, bone as *mut c_void);
}
unsafe extern "C" fn _sortPathConstraintAttachmentBones(
    internal: *mut _spSkeleton,
    mut attachment: *mut spAttachment,
    mut slotBone: *mut spBone,
) {
    let mut pathAttachment: *mut spPathAttachment = attachment as *mut spPathAttachment;
    let mut pathBones: *mut c_int = 0 as *mut c_int;
    let mut pathBonesCount: c_int = 0;
    if (*pathAttachment).super_0.super_0.type_0 as c_uint != SP_ATTACHMENT_PATH as c_int as c_uint {
        return;
    }
    pathBones = (*pathAttachment).super_0.bones;
    pathBonesCount = (*pathAttachment).super_0.bonesCount;
    if pathBones.is_null() {
        _sortBone(internal, slotBone);
    } else {
        let mut bones: *mut *mut spBone = (*internal).super_0.bones;
        let mut i: c_int = 0 as c_int;
        let mut n: c_int = 0;
        i = 0 as c_int;
        n = pathBonesCount;
        while i < n {
            let fresh98 = i;
            i = i + 1;
            let mut nn: c_int = *pathBones.offset(fresh98 as isize);
            nn += i;
            while i < nn {
                let fresh99 = i;
                i = i + 1;
                _sortBone(
                    internal,
                    *bones.offset(*pathBones.offset(fresh99 as isize) as isize),
                );
            }
        }
    };
}
unsafe extern "C" fn _sortPathConstraintAttachment(
    internal: *mut _spSkeleton,
    mut skin: *mut spSkin,
    mut slotIndex: c_int,
    mut slotBone: *mut spBone,
) {
    let mut entry: *mut _Entry = (*(skin as *mut _spSkin)).entries;
    while !entry.is_null() {
        if (*entry).slotIndex == slotIndex {
            _sortPathConstraintAttachmentBones(internal, (*entry).attachment, slotBone);
        }
        entry = (*entry).next;
    }
}
unsafe extern "C" fn _sortReset(mut bones: *mut *mut spBone, mut bonesCount: c_int) {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < bonesCount {
        let mut bone: *mut spBone = *bones.offset(i as isize);
        if !((*bone).active == 0) {
            if (*bone).sorted != 0 {
                _sortReset((*bone).children, (*bone).childrenCount);
            }
            (*bone).sorted = 0 as c_int;
        }
        i += 1;
    }
}
unsafe extern "C" fn _sortIkConstraint(
    internal: *mut _spSkeleton,
    mut constraint: *mut spIkConstraint,
) {
    let mut target: *mut spBone = (*constraint).target;
    let mut constrained: *mut *mut spBone = 0 as *mut *mut spBone;
    let mut parent: *mut spBone = 0 as *mut spBone;
    (*constraint).active = ((*(*constraint).target).active != 0
        && ((*(*constraint).data).skinRequired == 0
            || !((*internal).super_0.skin).is_null()
                && spIkConstraintDataArray_contains(
                    (*(*internal).super_0.skin).ikConstraints,
                    (*constraint).data,
                ) != 0)) as c_int;
    if (*constraint).active == 0 {
        return;
    }
    _sortBone(internal, target);
    constrained = (*constraint).bones;
    parent = *constrained.offset(0 as c_int as isize);
    _sortBone(internal, parent);
    if (*constraint).bonesCount == 1 as c_int {
        _addToUpdateCache(internal, SP_UPDATE_IK_CONSTRAINT, constraint as *mut c_void);
        _sortReset((*parent).children, (*parent).childrenCount);
    } else {
        let mut child: *mut spBone =
            *constrained.offset(((*constraint).bonesCount - 1 as c_int) as isize);
        _sortBone(internal, child);
        _addToUpdateCache(internal, SP_UPDATE_IK_CONSTRAINT, constraint as *mut c_void);
        _sortReset((*parent).children, (*parent).childrenCount);
        (*child).sorted = 1 as c_int;
    };
}
unsafe extern "C" fn _sortPathConstraint(
    internal: *mut _spSkeleton,
    mut constraint: *mut spPathConstraint,
) {
    let mut slot: *mut spSlot = (*constraint).target;
    let mut slotIndex: c_int = (*(*slot).data).index;
    let mut slotBone: *mut spBone = (*slot).bone;
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    let mut boneCount: c_int = 0;
    let mut attachment: *mut spAttachment = 0 as *mut spAttachment;
    let mut constrained: *mut *mut spBone = 0 as *mut *mut spBone;
    let mut skeleton: *mut spSkeleton = internal as *mut spSkeleton;
    (*constraint).active = ((*(*(*constraint).target).bone).active != 0
        && ((*(*constraint).data).skinRequired == 0
            || !((*internal).super_0.skin).is_null()
                && spPathConstraintDataArray_contains(
                    (*(*internal).super_0.skin).pathConstraints,
                    (*constraint).data,
                ) != 0)) as c_int;
    if (*constraint).active == 0 {
        return;
    }
    if !((*skeleton).skin).is_null() {
        _sortPathConstraintAttachment(internal, (*skeleton).skin, slotIndex, slotBone);
    }
    if !((*(*skeleton).data).defaultSkin).is_null()
        && (*(*skeleton).data).defaultSkin != (*skeleton).skin
    {
        _sortPathConstraintAttachment(
            internal,
            (*(*skeleton).data).defaultSkin,
            slotIndex,
            slotBone,
        );
    }
    i = 0 as c_int;
    n = (*(*skeleton).data).skinsCount;
    while i < n {
        _sortPathConstraintAttachment(
            internal,
            *((*(*skeleton).data).skins).offset(i as isize),
            slotIndex,
            slotBone,
        );
        i += 1;
    }
    attachment = (*slot).attachment;
    if !attachment.is_null()
        && (*attachment).type_0 as c_uint == SP_ATTACHMENT_PATH as c_int as c_uint
    {
        _sortPathConstraintAttachmentBones(internal, attachment, slotBone);
    }
    constrained = (*constraint).bones;
    boneCount = (*constraint).bonesCount;
    i = 0 as c_int;
    while i < boneCount {
        _sortBone(internal, *constrained.offset(i as isize));
        i += 1;
    }
    _addToUpdateCache(
        internal,
        SP_UPDATE_PATH_CONSTRAINT,
        constraint as *mut c_void,
    );
    i = 0 as c_int;
    while i < boneCount {
        _sortReset(
            (**constrained.offset(i as isize)).children,
            (**constrained.offset(i as isize)).childrenCount,
        );
        i += 1;
    }
    i = 0 as c_int;
    while i < boneCount {
        (**constrained.offset(i as isize)).sorted = 1 as c_int;
        i += 1;
    }
}
unsafe extern "C" fn _sortTransformConstraint(
    internal: *mut _spSkeleton,
    mut constraint: *mut spTransformConstraint,
) {
    let mut i: c_int = 0;
    let mut boneCount: c_int = 0;
    let mut constrained: *mut *mut spBone = 0 as *mut *mut spBone;
    let mut child: *mut spBone = 0 as *mut spBone;
    (*constraint).active = ((*(*constraint).target).active != 0
        && ((*(*constraint).data).skinRequired == 0
            || !((*internal).super_0.skin).is_null()
                && spTransformConstraintDataArray_contains(
                    (*(*internal).super_0.skin).transformConstraints,
                    (*constraint).data,
                ) != 0)) as c_int;
    if (*constraint).active == 0 {
        return;
    }
    _sortBone(internal, (*constraint).target);
    constrained = (*constraint).bones;
    boneCount = (*constraint).bonesCount;
    if (*(*constraint).data).local != 0 {
        i = 0 as c_int;
        while i < boneCount {
            child = *constrained.offset(i as isize);
            _sortBone(internal, (*child).parent);
            _sortBone(internal, child);
            i += 1;
        }
    } else {
        i = 0 as c_int;
        while i < boneCount {
            _sortBone(internal, *constrained.offset(i as isize));
            i += 1;
        }
    }
    _addToUpdateCache(
        internal,
        SP_UPDATE_TRANSFORM_CONSTRAINT,
        constraint as *mut c_void,
    );
    i = 0 as c_int;
    while i < boneCount {
        _sortReset(
            (**constrained.offset(i as isize)).children,
            (**constrained.offset(i as isize)).childrenCount,
        );
        i += 1;
    }
    i = 0 as c_int;
    while i < boneCount {
        (**constrained.offset(i as isize)).sorted = 1 as c_int;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spSkeleton_updateCache(mut self_0: *mut spSkeleton) {
    let mut i: c_int = 0;
    let mut ii: c_int = 0;
    let mut bones: *mut *mut spBone = 0 as *mut *mut spBone;
    let mut ikConstraints: *mut *mut spIkConstraint = 0 as *mut *mut spIkConstraint;
    let mut pathConstraints: *mut *mut spPathConstraint = 0 as *mut *mut spPathConstraint;
    let mut transformConstraints: *mut *mut spTransformConstraint =
        0 as *mut *mut spTransformConstraint;
    let mut ikCount: c_int = 0;
    let mut transformCount: c_int = 0;
    let mut pathCount: c_int = 0;
    let mut constraintCount: c_int = 0;
    let mut internal: *mut _spSkeleton = self_0 as *mut _spSkeleton;
    (*internal).updateCacheCapacity = (*self_0).bonesCount
        + (*self_0).ikConstraintsCount
        + (*self_0).transformConstraintsCount
        + (*self_0).pathConstraintsCount;
    _spFree((*internal).updateCache as *mut c_void);
    (*internal).updateCache = _spMalloc(
        (::core::mem::size_of::<_spUpdate>() as c_ulong)
            .wrapping_mul((*internal).updateCacheCapacity as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        8010 as c_int,
    ) as *mut _spUpdate;
    (*internal).updateCacheCount = 0 as c_int;
    bones = (*self_0).bones;
    i = 0 as c_int;
    while i < (*self_0).bonesCount {
        let mut bone: *mut spBone = *bones.offset(i as isize);
        (*bone).sorted = (*(*bone).data).skinRequired;
        (*bone).active = ((*bone).sorted == 0) as c_int;
        i += 1;
    }
    if !((*self_0).skin).is_null() {
        let mut skinBones: *mut spBoneDataArray = (*(*self_0).skin).bones;
        i = 0 as c_int;
        while i < (*skinBones).size {
            let mut bone_0: *mut spBone = *((*self_0).bones)
                .offset((**((*skinBones).items).offset(i as isize)).index as isize);
            loop {
                (*bone_0).sorted = 0 as c_int;
                (*bone_0).active = -(1 as c_int);
                bone_0 = (*bone_0).parent;
                if bone_0.is_null() {
                    break;
                }
            }
            i += 1;
        }
    }
    ikConstraints = (*self_0).ikConstraints;
    transformConstraints = (*self_0).transformConstraints;
    pathConstraints = (*self_0).pathConstraints;
    ikCount = (*self_0).ikConstraintsCount;
    transformCount = (*self_0).transformConstraintsCount;
    pathCount = (*self_0).pathConstraintsCount;
    constraintCount = ikCount + transformCount + pathCount;
    i = 0 as c_int;
    's_125: while i < constraintCount {
        ii = 0 as c_int;
        while ii < ikCount {
            let mut ikConstraint: *mut spIkConstraint = *ikConstraints.offset(ii as isize);
            if (*(*ikConstraint).data).order == i {
                _sortIkConstraint(internal, ikConstraint);
                i += 1;
                continue 's_125;
            } else {
                ii += 1;
            }
        }
        ii = 0 as c_int;
        while ii < transformCount {
            let mut transformConstraint: *mut spTransformConstraint =
                *transformConstraints.offset(ii as isize);
            if (*(*transformConstraint).data).order == i {
                _sortTransformConstraint(internal, transformConstraint);
                i += 1;
                continue 's_125;
            } else {
                ii += 1;
            }
        }
        ii = 0 as c_int;
        while ii < pathCount {
            let mut pathConstraint: *mut spPathConstraint = *pathConstraints.offset(ii as isize);
            if (*(*pathConstraint).data).order == i {
                _sortPathConstraint(internal, pathConstraint);
                i += 1;
                continue 's_125;
            } else {
                ii += 1;
            }
        }
        i += 1;
    }
    i = 0 as c_int;
    while i < (*self_0).bonesCount {
        _sortBone(internal, *((*self_0).bones).offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spSkeleton_updateWorldTransform(mut self_0: *const spSkeleton) {
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    let mut internal: *mut _spSkeleton = self_0 as *mut _spSkeleton;
    i = 0 as c_int;
    n = (*self_0).bonesCount;
    while i < n {
        let mut bone: *mut spBone = *((*self_0).bones).offset(i as isize);
        (*bone).ax = (*bone).x;
        (*bone).ay = (*bone).y;
        (*bone).arotation = (*bone).rotation;
        (*bone).ascaleX = (*bone).scaleX;
        (*bone).ascaleY = (*bone).scaleY;
        (*bone).ashearX = (*bone).shearX;
        (*bone).ashearY = (*bone).shearY;
        i += 1;
    }
    i = 0 as c_int;
    while i < (*internal).updateCacheCount {
        let mut update: *mut _spUpdate = ((*internal).updateCache).offset(i as isize);
        match (*update).type_0 as c_uint {
            0 => {
                spBone_update((*update).object as *mut spBone);
            }
            1 => {
                spIkConstraint_update((*update).object as *mut spIkConstraint);
            }
            3 => {
                spTransformConstraint_update((*update).object as *mut spTransformConstraint);
            }
            2 => {
                spPathConstraint_update((*update).object as *mut spPathConstraint);
            }
            _ => {}
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spSkeleton_updateWorldTransformWith(
    mut self_0: *const spSkeleton,
    mut parent: *const spBone,
) {
    let mut i: c_int = 0;
    let mut rotationY: c_float = 0.;
    let mut la: c_float = 0.;
    let mut lb: c_float = 0.;
    let mut lc: c_float = 0.;
    let mut ld: c_float = 0.;
    let mut _updateCache: *mut _spUpdate = 0 as *mut _spUpdate;
    let mut internal: *mut _spSkeleton = self_0 as *mut _spSkeleton;
    let mut rootBone: *mut spBone = (*self_0).root;
    let mut pa: c_float = (*parent).a;
    let mut pb: c_float = (*parent).b;
    let mut pc: c_float = (*parent).c;
    let mut pd: c_float = (*parent).d;
    *(&(*rootBone).worldX as *const c_float as *mut c_float) =
        pa * (*self_0).x + pb * (*self_0).y + (*parent).worldX;
    *(&(*rootBone).worldY as *const c_float as *mut c_float) =
        pc * (*self_0).x + pd * (*self_0).y + (*parent).worldY;
    rotationY = (*rootBone).rotation + 90 as c_int as c_float + (*rootBone).shearY;
    la = cosf(
        ((*rootBone).rotation + (*rootBone).shearX)
            * (3.1415926535897932385f32 / 180 as c_int as c_float),
    ) * (*rootBone).scaleX;
    lb =
        cosf(rotationY * (3.1415926535897932385f32 / 180 as c_int as c_float)) * (*rootBone).scaleY;
    lc = sinf(
        ((*rootBone).rotation + (*rootBone).shearX)
            * (3.1415926535897932385f32 / 180 as c_int as c_float),
    ) * (*rootBone).scaleX;
    ld =
        sinf(rotationY * (3.1415926535897932385f32 / 180 as c_int as c_float)) * (*rootBone).scaleY;
    *(&(*rootBone).a as *const c_float as *mut c_float) = (pa * la + pb * lc) * (*self_0).scaleX;
    *(&(*rootBone).b as *const c_float as *mut c_float) = (pa * lb + pb * ld) * (*self_0).scaleX;
    *(&(*rootBone).c as *const c_float as *mut c_float) = (pc * la + pd * lc) * (*self_0).scaleY;
    *(&(*rootBone).d as *const c_float as *mut c_float) = (pc * lb + pd * ld) * (*self_0).scaleY;
    _updateCache = (*internal).updateCache;
    i = 0 as c_int;
    while i < (*internal).updateCacheCount {
        let mut update: *mut _spUpdate = ((*internal).updateCache).offset(i as isize);
        match (*update).type_0 as c_uint {
            0 => {
                if (*update).object as *mut spBone != rootBone {
                    spBone_updateWorldTransform((*update).object as *mut spBone);
                }
            }
            1 => {
                spIkConstraint_update((*update).object as *mut spIkConstraint);
            }
            3 => {
                spTransformConstraint_update((*update).object as *mut spTransformConstraint);
            }
            2 => {
                spPathConstraint_update((*update).object as *mut spPathConstraint);
            }
            _ => {}
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spSkeleton_setToSetupPose(mut self_0: *const spSkeleton) {
    spSkeleton_setBonesToSetupPose(self_0);
    spSkeleton_setSlotsToSetupPose(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn spSkeleton_setBonesToSetupPose(mut self_0: *const spSkeleton) {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*self_0).bonesCount {
        spBone_setToSetupPose(*((*self_0).bones).offset(i as isize));
        i += 1;
    }
    i = 0 as c_int;
    while i < (*self_0).ikConstraintsCount {
        let mut ikConstraint: *mut spIkConstraint = *((*self_0).ikConstraints).offset(i as isize);
        (*ikConstraint).bendDirection = (*(*ikConstraint).data).bendDirection;
        (*ikConstraint).compress = (*(*ikConstraint).data).compress;
        (*ikConstraint).stretch = (*(*ikConstraint).data).stretch;
        (*ikConstraint).softness = (*(*ikConstraint).data).softness;
        (*ikConstraint).mix = (*(*ikConstraint).data).mix;
        i += 1;
    }
    i = 0 as c_int;
    while i < (*self_0).transformConstraintsCount {
        let mut constraint: *mut spTransformConstraint =
            *((*self_0).transformConstraints).offset(i as isize);
        let mut data: *mut spTransformConstraintData = (*constraint).data;
        (*constraint).mixRotate = (*data).mixRotate;
        (*constraint).mixX = (*data).mixX;
        (*constraint).mixY = (*data).mixY;
        (*constraint).mixScaleX = (*data).mixScaleX;
        (*constraint).mixScaleY = (*data).mixScaleY;
        (*constraint).mixShearY = (*data).mixShearY;
        i += 1;
    }
    i = 0 as c_int;
    while i < (*self_0).pathConstraintsCount {
        let mut constraint_0: *mut spPathConstraint =
            *((*self_0).pathConstraints).offset(i as isize);
        let mut data_0: *mut spPathConstraintData = (*constraint_0).data;
        (*constraint_0).position = (*data_0).position;
        (*constraint_0).spacing = (*data_0).spacing;
        (*constraint_0).mixRotate = (*data_0).mixRotate;
        (*constraint_0).mixX = (*data_0).mixX;
        (*constraint_0).mixY = (*data_0).mixY;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spSkeleton_setSlotsToSetupPose(mut self_0: *const spSkeleton) {
    let mut i: c_int = 0;
    spine_memcpy(
        (*self_0).drawOrder as *mut c_void,
        (*self_0).slots as *const c_void,
        ((*self_0).slotsCount as c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut spSlot>() as c_ulong),
    );
    i = 0 as c_int;
    while i < (*self_0).slotsCount {
        spSlot_setToSetupPose(*((*self_0).slots).offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spSkeleton_findBone(
    mut self_0: *const spSkeleton,
    mut boneName: *const c_char,
) -> *mut spBone {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*self_0).bonesCount {
        if spine_strcmp(
            (**((*(*self_0).data).bones).offset(i as isize)).name,
            boneName,
        ) == 0 as c_int
        {
            return *((*self_0).bones).offset(i as isize);
        }
        i += 1;
    }
    return 0 as *mut spBone;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeleton_findSlot(
    mut self_0: *const spSkeleton,
    mut slotName: *const c_char,
) -> *mut spSlot {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*self_0).slotsCount {
        if spine_strcmp(
            (**((*(*self_0).data).slots).offset(i as isize)).name,
            slotName,
        ) == 0 as c_int
        {
            return *((*self_0).slots).offset(i as isize);
        }
        i += 1;
    }
    return 0 as *mut spSlot;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeleton_setSkinByName(
    mut self_0: *mut spSkeleton,
    mut skinName: *const c_char,
) -> c_int {
    let mut skin: *mut spSkin = 0 as *mut spSkin;
    if skinName.is_null() {
        spSkeleton_setSkin(self_0, 0 as *mut spSkin);
        return 1 as c_int;
    }
    skin = spSkeletonData_findSkin((*self_0).data, skinName);
    if skin.is_null() {
        return 0 as c_int;
    }
    spSkeleton_setSkin(self_0, skin);
    return 1 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeleton_setSkin(mut self_0: *mut spSkeleton, mut newSkin: *mut spSkin) {
    if (*self_0).skin == newSkin {
        return;
    }
    if !newSkin.is_null() {
        if !((*self_0).skin).is_null() {
            spSkin_attachAll(newSkin, self_0, (*self_0).skin);
        } else {
            let mut i: c_int = 0;
            i = 0 as c_int;
            while i < (*self_0).slotsCount {
                let mut slot: *mut spSlot = *((*self_0).slots).offset(i as isize);
                if !((*(*slot).data).attachmentName).is_null() {
                    let mut attachment: *mut spAttachment =
                        spSkin_getAttachment(newSkin, i, (*(*slot).data).attachmentName);
                    if !attachment.is_null() {
                        spSlot_setAttachment(slot, attachment);
                    }
                }
                i += 1;
            }
        }
    }
    let ref mut fresh100 = *(&(*self_0).skin as *const *mut spSkin as *mut *mut spSkin);
    *fresh100 = newSkin;
    spSkeleton_updateCache(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn spSkeleton_getAttachmentForSlotName(
    mut self_0: *const spSkeleton,
    mut slotName: *const c_char,
    mut attachmentName: *const c_char,
) -> *mut spAttachment {
    let mut slotIndex: c_int = (*spSkeletonData_findSlot((*self_0).data, slotName)).index;
    return spSkeleton_getAttachmentForSlotIndex(self_0, slotIndex, attachmentName);
}
#[no_mangle]
pub unsafe extern "C" fn spSkeleton_getAttachmentForSlotIndex(
    mut self_0: *const spSkeleton,
    mut slotIndex: c_int,
    mut attachmentName: *const c_char,
) -> *mut spAttachment {
    if slotIndex == -(1 as c_int) {
        return 0 as *mut spAttachment;
    }
    if !((*self_0).skin).is_null() {
        let mut attachment: *mut spAttachment =
            spSkin_getAttachment((*self_0).skin, slotIndex, attachmentName);
        if !attachment.is_null() {
            return attachment;
        }
    }
    if !((*(*self_0).data).defaultSkin).is_null() {
        let mut attachment_0: *mut spAttachment =
            spSkin_getAttachment((*(*self_0).data).defaultSkin, slotIndex, attachmentName);
        if !attachment_0.is_null() {
            return attachment_0;
        }
    }
    return 0 as *mut spAttachment;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeleton_setAttachment(
    mut self_0: *mut spSkeleton,
    mut slotName: *const c_char,
    mut attachmentName: *const c_char,
) -> c_int {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*self_0).slotsCount {
        let mut slot: *mut spSlot = *((*self_0).slots).offset(i as isize);
        if spine_strcmp((*(*slot).data).name, slotName) == 0 as c_int {
            if attachmentName.is_null() {
                spSlot_setAttachment(slot, 0 as *mut spAttachment);
            } else {
                let mut attachment: *mut spAttachment =
                    spSkeleton_getAttachmentForSlotIndex(self_0, i, attachmentName);
                if attachment.is_null() {
                    return 0 as c_int;
                }
                spSlot_setAttachment(slot, attachment);
            }
            return 1 as c_int;
        }
        i += 1;
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeleton_findIkConstraint(
    mut self_0: *const spSkeleton,
    mut constraintName: *const c_char,
) -> *mut spIkConstraint {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*self_0).ikConstraintsCount {
        if spine_strcmp(
            (*(**((*self_0).ikConstraints).offset(i as isize)).data).name,
            constraintName,
        ) == 0 as c_int
        {
            return *((*self_0).ikConstraints).offset(i as isize);
        }
        i += 1;
    }
    return 0 as *mut spIkConstraint;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeleton_findTransformConstraint(
    mut self_0: *const spSkeleton,
    mut constraintName: *const c_char,
) -> *mut spTransformConstraint {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*self_0).transformConstraintsCount {
        if spine_strcmp(
            (*(**((*self_0).transformConstraints).offset(i as isize)).data).name,
            constraintName,
        ) == 0 as c_int
        {
            return *((*self_0).transformConstraints).offset(i as isize);
        }
        i += 1;
    }
    return 0 as *mut spTransformConstraint;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeleton_findPathConstraint(
    mut self_0: *const spSkeleton,
    mut constraintName: *const c_char,
) -> *mut spPathConstraint {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*self_0).pathConstraintsCount {
        if spine_strcmp(
            (*(**((*self_0).pathConstraints).offset(i as isize)).data).name,
            constraintName,
        ) == 0 as c_int
        {
            return *((*self_0).pathConstraints).offset(i as isize);
        }
        i += 1;
    }
    return 0 as *mut spPathConstraint;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonBinary_createWithLoader(
    mut attachmentLoader: *mut spAttachmentLoader,
) -> *mut spSkeletonBinary {
    let mut self_0: *mut spSkeletonBinary = &mut (*((_spCalloc
        as unsafe extern "C" fn(size_t, size_t, *const c_char, c_int) -> *mut c_void)(
        1 as c_int as size_t,
        ::core::mem::size_of::<_spSkeletonBinary>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        8365 as c_int,
    ) as *mut _spSkeletonBinary))
        .super_0;
    (*self_0).scale = 1 as c_int as c_float;
    (*self_0).attachmentLoader = attachmentLoader;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonBinary_create(mut atlas: *mut spAtlas) -> *mut spSkeletonBinary {
    let mut attachmentLoader: *mut spAtlasAttachmentLoader = spAtlasAttachmentLoader_create(atlas);
    let mut self_0: *mut spSkeletonBinary =
        spSkeletonBinary_createWithLoader(&mut (*attachmentLoader).super_0);
    (*(self_0 as *mut _spSkeletonBinary)).ownsLoader = 1 as c_int;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonBinary_dispose(mut self_0: *mut spSkeletonBinary) {
    let mut internal: *mut _spSkeletonBinary = self_0 as *mut _spSkeletonBinary;
    if (*internal).ownsLoader != 0 {
        spAttachmentLoader_dispose((*self_0).attachmentLoader);
    }
    _spFree((*internal).linkedMeshes as *mut c_void);
    _spFree((*self_0).error as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _spSkeletonBinary_setError(
    mut self_0: *mut spSkeletonBinary,
    mut value1: *const c_char,
    mut value2: *const c_char,
) {
    let mut message: [c_char; 256] = [0; 256];
    let mut length: c_int = 0;
    _spFree((*self_0).error as *mut c_void);
    spine_strcpy(message.as_mut_ptr(), value1);
    length = spine_strlen(value1) as c_int;
    if !value2.is_null() {
        spine_strncat(
            message.as_mut_ptr().offset(length as isize),
            value2,
            (255 as c_int - length) as size_t,
        );
    }
    let ref mut fresh101 = *(&(*self_0).error as *const *const c_char as *mut *mut c_char);
    *fresh101 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul((spine_strlen(message.as_mut_ptr())).wrapping_add(1 as c_int as c_ulong)),
        b"spine.c\0" as *const u8 as *const c_char,
        8393 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh101, message.as_mut_ptr());
}
unsafe extern "C" fn readByte(mut input: *mut _dataInput) -> c_uchar {
    let fresh102 = (*input).cursor;
    (*input).cursor = ((*input).cursor).offset(1);
    return *fresh102;
}
unsafe extern "C" fn readSByte(mut input: *mut _dataInput) -> c_schar {
    return readByte(input) as c_schar;
}
unsafe extern "C" fn readBoolean(mut input: *mut _dataInput) -> c_int {
    return (readByte(input) as c_int != 0 as c_int) as c_int;
}
unsafe extern "C" fn readInt(mut input: *mut _dataInput) -> c_int {
    let mut result: uint32_t = readByte(input) as uint32_t;
    result <<= 8 as c_int;
    result |= readByte(input) as c_uint;
    result <<= 8 as c_int;
    result |= readByte(input) as c_uint;
    result <<= 8 as c_int;
    result |= readByte(input) as c_uint;
    return result as c_int;
}
unsafe extern "C" fn readVarint(mut input: *mut _dataInput, mut optimizePositive: c_int) -> c_int {
    let mut b: c_uchar = readByte(input);
    let mut value: uint32_t = (b as c_int & 0x7f as c_int) as uint32_t;
    if b as c_int & 0x80 as c_int != 0 {
        b = readByte(input);
        value |= ((b as c_int & 0x7f as c_int) << 7 as c_int) as c_uint;
        if b as c_int & 0x80 as c_int != 0 {
            b = readByte(input);
            value |= ((b as c_int & 0x7f as c_int) << 14 as c_int) as c_uint;
            if b as c_int & 0x80 as c_int != 0 {
                b = readByte(input);
                value |= ((b as c_int & 0x7f as c_int) << 21 as c_int) as c_uint;
                if b as c_int & 0x80 as c_int != 0 {
                    value |=
                        ((readByte(input) as c_int & 0x7f as c_int) as uint32_t) << 28 as c_int;
                }
            }
        }
    }
    if optimizePositive == 0 {
        value = value >> 1 as c_int ^ (value & 1 as c_int as c_uint).wrapping_neg();
    }
    return value as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn readFloat(mut input: *mut _dataInput) -> c_float {
    let mut intToFloat: C2RustUnnamed_0 = C2RustUnnamed_0 { intValue: 0 };
    intToFloat.intValue = readInt(input);
    return intToFloat.floatValue;
}
#[no_mangle]
pub unsafe extern "C" fn readString(mut input: *mut _dataInput) -> *mut c_char {
    let mut length: c_int = readVarint(input, 1 as c_int);
    let mut string: *mut c_char = 0 as *mut c_char;
    if length == 0 as c_int {
        return 0 as *mut c_char;
    }
    string = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong).wrapping_mul(length as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        8452 as c_int,
    ) as *mut c_char;
    spine_memcpy(
        string as *mut c_void,
        (*input).cursor as *const c_void,
        (length - 1 as c_int) as size_t,
    );
    (*input).cursor = ((*input).cursor).offset((length - 1 as c_int) as isize);
    *string.offset((length - 1 as c_int) as isize) = '\0' as i32 as c_char;
    return string;
}
unsafe extern "C" fn readStringRef(
    mut input: *mut _dataInput,
    mut skeletonData: *mut spSkeletonData,
) -> *mut c_char {
    let mut index: c_int = readVarint(input, 1 as c_int);
    return if index == 0 as c_int {
        0 as *mut c_char
    } else {
        *((*skeletonData).strings).offset((index - 1 as c_int) as isize)
    };
}
unsafe extern "C" fn readColor(
    mut input: *mut _dataInput,
    mut r: *mut c_float,
    mut g: *mut c_float,
    mut b: *mut c_float,
    mut a: *mut c_float,
) {
    *r = readByte(input) as c_int as c_float / 255.0f32;
    *g = readByte(input) as c_int as c_float / 255.0f32;
    *b = readByte(input) as c_int as c_float / 255.0f32;
    *a = readByte(input) as c_int as c_float / 255.0f32;
}
unsafe extern "C" fn readSequenceBinary(mut input: *mut _dataInput) -> *mut spSequence {
    let mut sequence: *mut spSequence = 0 as *mut spSequence;
    if readBoolean(input) == 0 {
        return 0 as *mut spSequence;
    }
    sequence = spSequence_create(readVarint(input, -(1 as c_int)));
    (*sequence).start = readVarint(input, -(1 as c_int));
    (*sequence).digits = readVarint(input, -(1 as c_int));
    (*sequence).setupIndex = readVarint(input, -(1 as c_int));
    return sequence;
}
unsafe extern "C" fn setBezierBinary(
    mut input: *mut _dataInput,
    mut timeline: *mut spTimeline,
    mut bezier: c_int,
    mut frame: c_int,
    mut value: c_int,
    mut time1: c_float,
    mut time2: c_float,
    mut value1: c_float,
    mut value2: c_float,
    mut scale: c_float,
) {
    let mut cx1: c_float = readFloat(input);
    let mut cy1: c_float = readFloat(input);
    let mut cx2: c_float = readFloat(input);
    let mut cy2: c_float = readFloat(input);
    spTimeline_setBezier(
        timeline,
        bezier,
        frame,
        value as c_float,
        time1,
        value1,
        cx1,
        cy1 * scale,
        cx2,
        cy2 * scale,
        time2,
        value2,
    );
}
unsafe extern "C" fn readTimelineBinary(
    mut input: *mut _dataInput,
    mut timeline: *mut spCurveTimeline1,
    mut scale: c_float,
) -> *mut spTimeline {
    let mut frame: c_int = 0;
    let mut bezier: c_int = 0;
    let mut frameLast: c_int = 0;
    let mut time2: c_float = 0.;
    let mut value2: c_float = 0.;
    let mut time: c_float = readFloat(input);
    let mut value: c_float = readFloat(input) * scale;
    frame = 0 as c_int;
    bezier = 0 as c_int;
    frameLast = (*timeline).super_0.frameCount - 1 as c_int;
    loop {
        spCurveTimeline1_setFrame(timeline, frame, time, value);
        if frame == frameLast {
            break;
        }
        time2 = readFloat(input);
        value2 = readFloat(input) * scale;
        match readSByte(input) as c_int {
            1 => {
                spCurveTimeline_setStepped(timeline, frame);
            }
            2 => {
                let fresh103 = bezier;
                bezier = bezier + 1;
                setBezierBinary(
                    input,
                    &mut (*timeline).super_0,
                    fresh103,
                    frame,
                    0 as c_int,
                    time,
                    time2,
                    value,
                    value2,
                    scale,
                );
            }
            _ => {}
        }
        time = time2;
        value = value2;
        frame += 1;
    }
    return &mut (*timeline).super_0;
}
unsafe extern "C" fn readTimeline2Binary(
    mut input: *mut _dataInput,
    mut timeline: *mut spCurveTimeline2,
    mut scale: c_float,
) -> *mut spTimeline {
    let mut frame: c_int = 0;
    let mut bezier: c_int = 0;
    let mut frameLast: c_int = 0;
    let mut time2: c_float = 0.;
    let mut nvalue1: c_float = 0.;
    let mut nvalue2: c_float = 0.;
    let mut time: c_float = readFloat(input);
    let mut value1: c_float = readFloat(input) * scale;
    let mut value2: c_float = readFloat(input) * scale;
    frame = 0 as c_int;
    bezier = 0 as c_int;
    frameLast = (*timeline).super_0.frameCount - 1 as c_int;
    loop {
        spCurveTimeline2_setFrame(timeline, frame, time, value1, value2);
        if frame == frameLast {
            break;
        }
        time2 = readFloat(input);
        nvalue1 = readFloat(input) * scale;
        nvalue2 = readFloat(input) * scale;
        match readSByte(input) as c_int {
            1 => {
                spCurveTimeline_setStepped(timeline, frame);
            }
            2 => {
                let fresh104 = bezier;
                bezier = bezier + 1;
                setBezierBinary(
                    input,
                    &mut (*timeline).super_0,
                    fresh104,
                    frame,
                    0 as c_int,
                    time,
                    time2,
                    value1,
                    nvalue1,
                    scale,
                );
                let fresh105 = bezier;
                bezier = bezier + 1;
                setBezierBinary(
                    input,
                    &mut (*timeline).super_0,
                    fresh105,
                    frame,
                    1 as c_int,
                    time,
                    time2,
                    value2,
                    nvalue2,
                    scale,
                );
            }
            _ => {}
        }
        time = time2;
        value1 = nvalue1;
        value2 = nvalue2;
        frame += 1;
    }
    return &mut (*timeline).super_0;
}
unsafe extern "C" fn _spSkeletonBinary_addLinkedMesh(
    mut self_0: *mut spSkeletonBinary,
    mut mesh: *mut spMeshAttachment,
    mut skin: *const c_char,
    mut slotIndex: c_int,
    mut parent: *const c_char,
    mut inheritDeform: c_int,
) {
    let mut linkedMesh: *mut _spLinkedMeshBinary = 0 as *mut _spLinkedMeshBinary;
    let mut internal: *mut _spSkeletonBinary = self_0 as *mut _spSkeletonBinary;
    if (*internal).linkedMeshCount == (*internal).linkedMeshCapacity {
        let mut linkedMeshes: *mut _spLinkedMeshBinary = 0 as *mut _spLinkedMeshBinary;
        (*internal).linkedMeshCapacity *= 2 as c_int;
        if (*internal).linkedMeshCapacity < 8 as c_int {
            (*internal).linkedMeshCapacity = 8 as c_int;
        }
        linkedMeshes = _spMalloc(
            (::core::mem::size_of::<_spLinkedMeshBinary>() as c_ulong)
                .wrapping_mul((*internal).linkedMeshCapacity as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            8603 as c_int,
        ) as *mut _spLinkedMeshBinary;
        spine_memcpy(
            linkedMeshes as *mut c_void,
            (*internal).linkedMeshes as *const c_void,
            (::core::mem::size_of::<_spLinkedMeshBinary>() as c_ulong)
                .wrapping_mul((*internal).linkedMeshCount as c_ulong),
        );
        _spFree((*internal).linkedMeshes as *mut c_void);
        (*internal).linkedMeshes = linkedMeshes;
    }
    let fresh106 = (*internal).linkedMeshCount;
    (*internal).linkedMeshCount = (*internal).linkedMeshCount + 1;
    linkedMesh = ((*internal).linkedMeshes).offset(fresh106 as isize);
    (*linkedMesh).mesh = mesh;
    (*linkedMesh).skin = skin;
    (*linkedMesh).slotIndex = slotIndex;
    (*linkedMesh).parent = parent;
    (*linkedMesh).inheritTimeline = inheritDeform;
}
unsafe extern "C" fn _spSkeletonBinary_readAnimation(
    mut self_0: *mut spSkeletonBinary,
    mut name: *const c_char,
    mut input: *mut _dataInput,
    mut skeletonData: *mut spSkeletonData,
) -> *mut spAnimation {
    let mut timelines: *mut spTimelineArray = spTimelineArray_create(18 as c_int);
    let mut duration: c_float = 0 as c_int as c_float;
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    let mut ii: c_int = 0;
    let mut nn: c_int = 0;
    let mut iii: c_int = 0;
    let mut nnn: c_int = 0;
    let mut frame: c_int = 0;
    let mut bezier: c_int = 0;
    let mut drawOrderCount: c_int = 0;
    let mut eventCount: c_int = 0;
    let mut animation: *mut spAnimation = 0 as *mut spAnimation;
    let mut scale: c_float = (*self_0).scale;
    let mut _numTimelines: c_int = readVarint(input, 1 as c_int);
    i = 0 as c_int;
    n = readVarint(input, 1 as c_int);
    while i < n {
        let mut slotIndex: c_int = readVarint(input, 1 as c_int);
        ii = 0 as c_int;
        nn = readVarint(input, 1 as c_int);
        while ii < nn {
            let mut timelineType: c_uchar = readByte(input);
            let mut frameCount: c_int = readVarint(input, 1 as c_int);
            let mut frameLast: c_int = frameCount - 1 as c_int;
            match timelineType as c_int {
                0 => {
                    let mut timeline: *mut spAttachmentTimeline =
                        spAttachmentTimeline_create(frameCount, slotIndex);
                    frame = 0 as c_int;
                    while frame < frameCount {
                        let mut time: c_float = readFloat(input);
                        let mut attachmentName: *const c_char = readStringRef(input, skeletonData);
                        spAttachmentTimeline_setFrame(timeline, frame, time, attachmentName);
                        frame += 1;
                    }
                    spTimelineArray_add(timelines, &mut (*timeline).super_0);
                }
                1 => {
                    let mut bezierCount: c_int = readVarint(input, 1 as c_int);
                    let mut timeline_0: *mut spRGBATimeline =
                        spRGBATimeline_create(frameCount, bezierCount, slotIndex);
                    let mut time_0: c_float = readFloat(input);
                    let mut r: c_float =
                        (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                    let mut g: c_float =
                        (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                    let mut b: c_float =
                        (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                    let mut a: c_float =
                        (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                    frame = 0 as c_int;
                    bezier = 0 as c_int;
                    loop {
                        let mut time2: c_float = 0.;
                        let mut r2: c_float = 0.;
                        let mut g2: c_float = 0.;
                        let mut b2: c_float = 0.;
                        let mut a2: c_float = 0.;
                        spRGBATimeline_setFrame(timeline_0, frame, time_0, r, g, b, a);
                        if frame == frameLast {
                            break;
                        }
                        time2 = readFloat(input);
                        r2 = (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                        g2 = (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                        b2 = (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                        a2 = (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                        match readSByte(input) as c_int {
                            1 => {
                                spCurveTimeline_setStepped(&mut (*timeline_0).super_0, frame);
                            }
                            2 => {
                                let fresh107 = bezier;
                                bezier = bezier + 1;
                                setBezierBinary(
                                    input,
                                    &mut (*timeline_0).super_0.super_0,
                                    fresh107,
                                    frame,
                                    0 as c_int,
                                    time_0,
                                    time2,
                                    r,
                                    r2,
                                    1 as c_int as c_float,
                                );
                                let fresh108 = bezier;
                                bezier = bezier + 1;
                                setBezierBinary(
                                    input,
                                    &mut (*timeline_0).super_0.super_0,
                                    fresh108,
                                    frame,
                                    1 as c_int,
                                    time_0,
                                    time2,
                                    g,
                                    g2,
                                    1 as c_int as c_float,
                                );
                                let fresh109 = bezier;
                                bezier = bezier + 1;
                                setBezierBinary(
                                    input,
                                    &mut (*timeline_0).super_0.super_0,
                                    fresh109,
                                    frame,
                                    2 as c_int,
                                    time_0,
                                    time2,
                                    b,
                                    b2,
                                    1 as c_int as c_float,
                                );
                                let fresh110 = bezier;
                                bezier = bezier + 1;
                                setBezierBinary(
                                    input,
                                    &mut (*timeline_0).super_0.super_0,
                                    fresh110,
                                    frame,
                                    3 as c_int,
                                    time_0,
                                    time2,
                                    a,
                                    a2,
                                    1 as c_int as c_float,
                                );
                            }
                            _ => {}
                        }
                        time_0 = time2;
                        r = r2;
                        g = g2;
                        b = b2;
                        a = a2;
                        frame += 1;
                    }
                    spTimelineArray_add(timelines, &mut (*timeline_0).super_0.super_0);
                }
                2 => {
                    let mut bezierCount_0: c_int = readVarint(input, 1 as c_int);
                    let mut timeline_1: *mut spRGBTimeline =
                        spRGBTimeline_create(frameCount, bezierCount_0, slotIndex);
                    let mut time_1: c_float = readFloat(input);
                    let mut r_0: c_float =
                        (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                    let mut g_0: c_float =
                        (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                    let mut b_0: c_float =
                        (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                    frame = 0 as c_int;
                    bezier = 0 as c_int;
                    loop {
                        let mut time2_0: c_float = 0.;
                        let mut r2_0: c_float = 0.;
                        let mut g2_0: c_float = 0.;
                        let mut b2_0: c_float = 0.;
                        spRGBTimeline_setFrame(timeline_1, frame, time_1, r_0, g_0, b_0);
                        if frame == frameLast {
                            break;
                        }
                        time2_0 = readFloat(input);
                        r2_0 = (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                        g2_0 = (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                        b2_0 = (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                        match readSByte(input) as c_int {
                            1 => {
                                spCurveTimeline_setStepped(&mut (*timeline_1).super_0, frame);
                            }
                            2 => {
                                let fresh111 = bezier;
                                bezier = bezier + 1;
                                setBezierBinary(
                                    input,
                                    &mut (*timeline_1).super_0.super_0,
                                    fresh111,
                                    frame,
                                    0 as c_int,
                                    time_1,
                                    time2_0,
                                    r_0,
                                    r2_0,
                                    1 as c_int as c_float,
                                );
                                let fresh112 = bezier;
                                bezier = bezier + 1;
                                setBezierBinary(
                                    input,
                                    &mut (*timeline_1).super_0.super_0,
                                    fresh112,
                                    frame,
                                    1 as c_int,
                                    time_1,
                                    time2_0,
                                    g_0,
                                    g2_0,
                                    1 as c_int as c_float,
                                );
                                let fresh113 = bezier;
                                bezier = bezier + 1;
                                setBezierBinary(
                                    input,
                                    &mut (*timeline_1).super_0.super_0,
                                    fresh113,
                                    frame,
                                    2 as c_int,
                                    time_1,
                                    time2_0,
                                    b_0,
                                    b2_0,
                                    1 as c_int as c_float,
                                );
                            }
                            _ => {}
                        }
                        time_1 = time2_0;
                        r_0 = r2_0;
                        g_0 = g2_0;
                        b_0 = b2_0;
                        frame += 1;
                    }
                    spTimelineArray_add(timelines, &mut (*timeline_1).super_0.super_0);
                }
                3 => {
                    let mut bezierCount_1: c_int = readVarint(input, 1 as c_int);
                    let mut timeline_2: *mut spRGBA2Timeline =
                        spRGBA2Timeline_create(frameCount, bezierCount_1, slotIndex);
                    let mut time_2: c_float = readFloat(input);
                    let mut r_1: c_float =
                        (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                    let mut g_1: c_float =
                        (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                    let mut b_1: c_float =
                        (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                    let mut a_0: c_float =
                        (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                    let mut r2_1: c_float =
                        (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                    let mut g2_1: c_float =
                        (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                    let mut b2_1: c_float =
                        (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                    frame = 0 as c_int;
                    bezier = 0 as c_int;
                    loop {
                        let mut time2_1: c_float = 0.;
                        let mut nr: c_float = 0.;
                        let mut ng: c_float = 0.;
                        let mut nb: c_float = 0.;
                        let mut na: c_float = 0.;
                        let mut nr2: c_float = 0.;
                        let mut ng2: c_float = 0.;
                        let mut nb2: c_float = 0.;
                        spRGBA2Timeline_setFrame(
                            timeline_2, frame, time_2, r_1, g_1, b_1, a_0, r2_1, g2_1, b2_1,
                        );
                        if frame == frameLast {
                            break;
                        }
                        time2_1 = readFloat(input);
                        nr = (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                        ng = (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                        nb = (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                        na = (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                        nr2 = (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                        ng2 = (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                        nb2 = (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                        match readSByte(input) as c_int {
                            1 => {
                                spCurveTimeline_setStepped(&mut (*timeline_2).super_0, frame);
                            }
                            2 => {
                                let fresh114 = bezier;
                                bezier = bezier + 1;
                                setBezierBinary(
                                    input,
                                    &mut (*timeline_2).super_0.super_0,
                                    fresh114,
                                    frame,
                                    0 as c_int,
                                    time_2,
                                    time2_1,
                                    r_1,
                                    nr,
                                    1 as c_int as c_float,
                                );
                                let fresh115 = bezier;
                                bezier = bezier + 1;
                                setBezierBinary(
                                    input,
                                    &mut (*timeline_2).super_0.super_0,
                                    fresh115,
                                    frame,
                                    1 as c_int,
                                    time_2,
                                    time2_1,
                                    g_1,
                                    ng,
                                    1 as c_int as c_float,
                                );
                                let fresh116 = bezier;
                                bezier = bezier + 1;
                                setBezierBinary(
                                    input,
                                    &mut (*timeline_2).super_0.super_0,
                                    fresh116,
                                    frame,
                                    2 as c_int,
                                    time_2,
                                    time2_1,
                                    b_1,
                                    nb,
                                    1 as c_int as c_float,
                                );
                                let fresh117 = bezier;
                                bezier = bezier + 1;
                                setBezierBinary(
                                    input,
                                    &mut (*timeline_2).super_0.super_0,
                                    fresh117,
                                    frame,
                                    3 as c_int,
                                    time_2,
                                    time2_1,
                                    a_0,
                                    na,
                                    1 as c_int as c_float,
                                );
                                let fresh118 = bezier;
                                bezier = bezier + 1;
                                setBezierBinary(
                                    input,
                                    &mut (*timeline_2).super_0.super_0,
                                    fresh118,
                                    frame,
                                    4 as c_int,
                                    time_2,
                                    time2_1,
                                    r2_1,
                                    nr2,
                                    1 as c_int as c_float,
                                );
                                let fresh119 = bezier;
                                bezier = bezier + 1;
                                setBezierBinary(
                                    input,
                                    &mut (*timeline_2).super_0.super_0,
                                    fresh119,
                                    frame,
                                    5 as c_int,
                                    time_2,
                                    time2_1,
                                    g2_1,
                                    ng2,
                                    1 as c_int as c_float,
                                );
                                let fresh120 = bezier;
                                bezier = bezier + 1;
                                setBezierBinary(
                                    input,
                                    &mut (*timeline_2).super_0.super_0,
                                    fresh120,
                                    frame,
                                    6 as c_int,
                                    time_2,
                                    time2_1,
                                    b2_1,
                                    nb2,
                                    1 as c_int as c_float,
                                );
                            }
                            _ => {}
                        }
                        time_2 = time2_1;
                        r_1 = nr;
                        g_1 = ng;
                        b_1 = nb;
                        a_0 = na;
                        r2_1 = nr2;
                        g2_1 = ng2;
                        b2_1 = nb2;
                        frame += 1;
                    }
                    spTimelineArray_add(timelines, &mut (*timeline_2).super_0.super_0);
                }
                4 => {
                    let mut bezierCount_2: c_int = readVarint(input, 1 as c_int);
                    let mut timeline_3: *mut spRGB2Timeline =
                        spRGB2Timeline_create(frameCount, bezierCount_2, slotIndex);
                    let mut time_3: c_float = readFloat(input);
                    let mut r_2: c_float =
                        (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                    let mut g_2: c_float =
                        (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                    let mut b_2: c_float =
                        (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                    let mut r2_2: c_float =
                        (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                    let mut g2_2: c_float =
                        (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                    let mut b2_2: c_float =
                        (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                    frame = 0 as c_int;
                    bezier = 0 as c_int;
                    loop {
                        let mut time2_2: c_float = 0.;
                        let mut nr_0: c_float = 0.;
                        let mut ng_0: c_float = 0.;
                        let mut nb_0: c_float = 0.;
                        let mut nr2_0: c_float = 0.;
                        let mut ng2_0: c_float = 0.;
                        let mut nb2_0: c_float = 0.;
                        spRGB2Timeline_setFrame(
                            timeline_3, frame, time_3, r_2, g_2, b_2, r2_2, g2_2, b2_2,
                        );
                        if frame == frameLast {
                            break;
                        }
                        time2_2 = readFloat(input);
                        nr_0 = (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                        ng_0 = (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                        nb_0 = (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                        nr2_0 = (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                        ng2_0 = (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                        nb2_0 = (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                        match readSByte(input) as c_int {
                            1 => {
                                spCurveTimeline_setStepped(&mut (*timeline_3).super_0, frame);
                            }
                            2 => {
                                let fresh121 = bezier;
                                bezier = bezier + 1;
                                setBezierBinary(
                                    input,
                                    &mut (*timeline_3).super_0.super_0,
                                    fresh121,
                                    frame,
                                    0 as c_int,
                                    time_3,
                                    time2_2,
                                    r_2,
                                    nr_0,
                                    1 as c_int as c_float,
                                );
                                let fresh122 = bezier;
                                bezier = bezier + 1;
                                setBezierBinary(
                                    input,
                                    &mut (*timeline_3).super_0.super_0,
                                    fresh122,
                                    frame,
                                    1 as c_int,
                                    time_3,
                                    time2_2,
                                    g_2,
                                    ng_0,
                                    1 as c_int as c_float,
                                );
                                let fresh123 = bezier;
                                bezier = bezier + 1;
                                setBezierBinary(
                                    input,
                                    &mut (*timeline_3).super_0.super_0,
                                    fresh123,
                                    frame,
                                    2 as c_int,
                                    time_3,
                                    time2_2,
                                    b_2,
                                    nb_0,
                                    1 as c_int as c_float,
                                );
                                let fresh124 = bezier;
                                bezier = bezier + 1;
                                setBezierBinary(
                                    input,
                                    &mut (*timeline_3).super_0.super_0,
                                    fresh124,
                                    frame,
                                    3 as c_int,
                                    time_3,
                                    time2_2,
                                    r2_2,
                                    nr2_0,
                                    1 as c_int as c_float,
                                );
                                let fresh125 = bezier;
                                bezier = bezier + 1;
                                setBezierBinary(
                                    input,
                                    &mut (*timeline_3).super_0.super_0,
                                    fresh125,
                                    frame,
                                    4 as c_int,
                                    time_3,
                                    time2_2,
                                    g2_2,
                                    ng2_0,
                                    1 as c_int as c_float,
                                );
                                let fresh126 = bezier;
                                bezier = bezier + 1;
                                setBezierBinary(
                                    input,
                                    &mut (*timeline_3).super_0.super_0,
                                    fresh126,
                                    frame,
                                    5 as c_int,
                                    time_3,
                                    time2_2,
                                    b2_2,
                                    nb2_0,
                                    1 as c_int as c_float,
                                );
                            }
                            _ => {}
                        }
                        time_3 = time2_2;
                        r_2 = nr_0;
                        g_2 = ng_0;
                        b_2 = nb_0;
                        r2_2 = nr2_0;
                        g2_2 = ng2_0;
                        b2_2 = nb2_0;
                        frame += 1;
                    }
                    spTimelineArray_add(timelines, &mut (*timeline_3).super_0.super_0);
                }
                5 => {
                    let mut bezierCount_3: c_int = readVarint(input, 1 as c_int);
                    let mut timeline_4: *mut spAlphaTimeline =
                        spAlphaTimeline_create(frameCount, bezierCount_3, slotIndex);
                    let mut time_4: c_float = readFloat(input);
                    let mut a_1: c_float =
                        (readByte(input) as c_int as c_double / 255.0f64) as c_float;
                    frame = 0 as c_int;
                    bezier = 0 as c_int;
                    loop {
                        let mut time2_3: c_float = 0.;
                        let mut a2_0: c_float = 0.;
                        spAlphaTimeline_setFrame(timeline_4, frame, time_4, a_1);
                        if frame == frameLast {
                            break;
                        }
                        time2_3 = readFloat(input);
                        a2_0 = (readByte(input) as c_int / 255 as c_int) as c_float;
                        match readSByte(input) as c_int {
                            1 => {
                                spCurveTimeline_setStepped(&mut (*timeline_4).super_0, frame);
                            }
                            2 => {
                                let fresh127 = bezier;
                                bezier = bezier + 1;
                                setBezierBinary(
                                    input,
                                    &mut (*timeline_4).super_0.super_0,
                                    fresh127,
                                    frame,
                                    0 as c_int,
                                    time_4,
                                    time2_3,
                                    a_1,
                                    a2_0,
                                    1 as c_int as c_float,
                                );
                            }
                            _ => {}
                        }
                        time_4 = time2_3;
                        a_1 = a2_0;
                        frame += 1;
                    }
                    spTimelineArray_add(timelines, &mut (*timeline_4).super_0.super_0);
                }
                _ => return 0 as *mut spAnimation,
            }
            ii += 1;
        }
        i += 1;
    }
    i = 0 as c_int;
    n = readVarint(input, 1 as c_int);
    while i < n {
        let mut boneIndex: c_int = readVarint(input, 1 as c_int);
        ii = 0 as c_int;
        nn = readVarint(input, 1 as c_int);
        while ii < nn {
            let mut timelineType_0: c_uchar = readByte(input);
            let mut frameCount_0: c_int = readVarint(input, 1 as c_int);
            let mut bezierCount_4: c_int = readVarint(input, 1 as c_int);
            let mut timeline_5: *mut spTimeline = 0 as *mut spTimeline;
            match timelineType_0 as c_int {
                0 => {
                    timeline_5 = readTimelineBinary(
                        input,
                        &mut (*(spRotateTimeline_create
                            as unsafe extern "C" fn(c_int, c_int, c_int) -> *mut spRotateTimeline)(
                            frameCount_0,
                            bezierCount_4,
                            boneIndex,
                        ))
                        .super_0,
                        1 as c_int as c_float,
                    );
                }
                1 => {
                    timeline_5 = readTimeline2Binary(
                        input,
                        &mut (*(spTranslateTimeline_create
                            as unsafe extern "C" fn(
                                c_int,
                                c_int,
                                c_int,
                            )
                                -> *mut spTranslateTimeline)(
                            frameCount_0,
                            bezierCount_4,
                            boneIndex,
                        ))
                        .super_0,
                        scale,
                    );
                }
                2 => {
                    timeline_5 = readTimelineBinary(
                        input,
                        &mut (*(spTranslateXTimeline_create
                            as unsafe extern "C" fn(
                                c_int,
                                c_int,
                                c_int,
                            )
                                -> *mut spTranslateXTimeline)(
                            frameCount_0,
                            bezierCount_4,
                            boneIndex,
                        ))
                        .super_0,
                        scale,
                    );
                }
                3 => {
                    timeline_5 = readTimelineBinary(
                        input,
                        &mut (*(spTranslateYTimeline_create
                            as unsafe extern "C" fn(
                                c_int,
                                c_int,
                                c_int,
                            )
                                -> *mut spTranslateYTimeline)(
                            frameCount_0,
                            bezierCount_4,
                            boneIndex,
                        ))
                        .super_0,
                        scale,
                    );
                }
                4 => {
                    timeline_5 = readTimeline2Binary(
                        input,
                        &mut (*(spScaleTimeline_create
                            as unsafe extern "C" fn(c_int, c_int, c_int) -> *mut spScaleTimeline)(
                            frameCount_0,
                            bezierCount_4,
                            boneIndex,
                        ))
                        .super_0,
                        1 as c_int as c_float,
                    );
                }
                5 => {
                    timeline_5 = readTimelineBinary(
                        input,
                        &mut (*(spScaleXTimeline_create
                            as unsafe extern "C" fn(c_int, c_int, c_int) -> *mut spScaleXTimeline)(
                            frameCount_0,
                            bezierCount_4,
                            boneIndex,
                        ))
                        .super_0,
                        1 as c_int as c_float,
                    );
                }
                6 => {
                    timeline_5 = readTimelineBinary(
                        input,
                        &mut (*(spScaleYTimeline_create
                            as unsafe extern "C" fn(c_int, c_int, c_int) -> *mut spScaleYTimeline)(
                            frameCount_0,
                            bezierCount_4,
                            boneIndex,
                        ))
                        .super_0,
                        1 as c_int as c_float,
                    );
                }
                7 => {
                    timeline_5 = readTimeline2Binary(
                        input,
                        &mut (*(spShearTimeline_create
                            as unsafe extern "C" fn(c_int, c_int, c_int) -> *mut spShearTimeline)(
                            frameCount_0,
                            bezierCount_4,
                            boneIndex,
                        ))
                        .super_0,
                        1 as c_int as c_float,
                    );
                }
                8 => {
                    timeline_5 = readTimelineBinary(
                        input,
                        &mut (*(spShearXTimeline_create
                            as unsafe extern "C" fn(c_int, c_int, c_int) -> *mut spShearXTimeline)(
                            frameCount_0,
                            bezierCount_4,
                            boneIndex,
                        ))
                        .super_0,
                        1 as c_int as c_float,
                    );
                }
                9 => {
                    timeline_5 = readTimelineBinary(
                        input,
                        &mut (*(spShearYTimeline_create
                            as unsafe extern "C" fn(c_int, c_int, c_int) -> *mut spShearYTimeline)(
                            frameCount_0,
                            bezierCount_4,
                            boneIndex,
                        ))
                        .super_0,
                        1 as c_int as c_float,
                    );
                }
                _ => {
                    iii = 0 as c_int;
                    while iii < (*timelines).size {
                        spTimeline_dispose(*((*timelines).items).offset(iii as isize));
                        iii += 1;
                    }
                    spTimelineArray_dispose(timelines);
                    _spSkeletonBinary_setError(
                        self_0,
                        b"Invalid timeline type for a bone: \0" as *const u8 as *const c_char,
                        (**((*skeletonData).bones).offset(boneIndex as isize)).name,
                    );
                    return 0 as *mut spAnimation;
                }
            }
            spTimelineArray_add(timelines, timeline_5);
            ii += 1;
        }
        i += 1;
    }
    i = 0 as c_int;
    n = readVarint(input, 1 as c_int);
    while i < n {
        let mut index: c_int = readVarint(input, 1 as c_int);
        let mut frameCount_1: c_int = readVarint(input, 1 as c_int);
        let mut frameLast_0: c_int = frameCount_1 - 1 as c_int;
        let mut bezierCount_5: c_int = readVarint(input, 1 as c_int);
        let mut timeline_6: *mut spIkConstraintTimeline =
            spIkConstraintTimeline_create(frameCount_1, bezierCount_5, index);
        let mut time_5: c_float = readFloat(input);
        let mut mix: c_float = readFloat(input);
        let mut softness: c_float = readFloat(input) * scale;
        frame = 0 as c_int;
        bezier = 0 as c_int;
        loop {
            let mut time2_4: c_float = 0.;
            let mut mix2: c_float = 0.;
            let mut softness2: c_float = 0.;
            let mut bendDirection: c_int = readSByte(input) as c_int;
            let mut compress: c_int = readBoolean(input);
            let mut stretch: c_int = readBoolean(input);
            spIkConstraintTimeline_setFrame(
                timeline_6,
                frame,
                time_5,
                mix,
                softness,
                bendDirection,
                compress,
                stretch,
            );
            if frame == frameLast_0 {
                break;
            }
            time2_4 = readFloat(input);
            mix2 = readFloat(input);
            softness2 = readFloat(input) * scale;
            match readSByte(input) as c_int {
                1 => {
                    spCurveTimeline_setStepped(&mut (*timeline_6).super_0, frame);
                }
                2 => {
                    let fresh128 = bezier;
                    bezier = bezier + 1;
                    setBezierBinary(
                        input,
                        &mut (*timeline_6).super_0.super_0,
                        fresh128,
                        frame,
                        0 as c_int,
                        time_5,
                        time2_4,
                        mix,
                        mix2,
                        1 as c_int as c_float,
                    );
                    let fresh129 = bezier;
                    bezier = bezier + 1;
                    setBezierBinary(
                        input,
                        &mut (*timeline_6).super_0.super_0,
                        fresh129,
                        frame,
                        1 as c_int,
                        time_5,
                        time2_4,
                        softness,
                        softness2,
                        scale,
                    );
                }
                _ => {}
            }
            time_5 = time2_4;
            mix = mix2;
            softness = softness2;
            frame += 1;
        }
        spTimelineArray_add(timelines, &mut (*timeline_6).super_0.super_0);
        i += 1;
    }
    i = 0 as c_int;
    n = readVarint(input, 1 as c_int);
    while i < n {
        let mut index_0: c_int = readVarint(input, 1 as c_int);
        let mut frameCount_2: c_int = readVarint(input, 1 as c_int);
        let mut frameLast_1: c_int = frameCount_2 - 1 as c_int;
        let mut bezierCount_6: c_int = readVarint(input, 1 as c_int);
        let mut timeline_7: *mut spTransformConstraintTimeline =
            spTransformConstraintTimeline_create(frameCount_2, bezierCount_6, index_0);
        let mut time_6: c_float = readFloat(input);
        let mut mixRotate: c_float = readFloat(input);
        let mut mixX: c_float = readFloat(input);
        let mut mixY: c_float = readFloat(input);
        let mut mixScaleX: c_float = readFloat(input);
        let mut mixScaleY: c_float = readFloat(input);
        let mut mixShearY: c_float = readFloat(input);
        frame = 0 as c_int;
        bezier = 0 as c_int;
        loop {
            let mut time2_5: c_float = 0.;
            let mut mixRotate2: c_float = 0.;
            let mut mixX2: c_float = 0.;
            let mut mixY2: c_float = 0.;
            let mut mixScaleX2: c_float = 0.;
            let mut mixScaleY2: c_float = 0.;
            let mut mixShearY2: c_float = 0.;
            spTransformConstraintTimeline_setFrame(
                timeline_7, frame, time_6, mixRotate, mixX, mixY, mixScaleX, mixScaleY, mixShearY,
            );
            if frame == frameLast_1 {
                break;
            }
            time2_5 = readFloat(input);
            mixRotate2 = readFloat(input);
            mixX2 = readFloat(input);
            mixY2 = readFloat(input);
            mixScaleX2 = readFloat(input);
            mixScaleY2 = readFloat(input);
            mixShearY2 = readFloat(input);
            match readSByte(input) as c_int {
                1 => {
                    spCurveTimeline_setStepped(&mut (*timeline_7).super_0, frame);
                }
                2 => {
                    let fresh130 = bezier;
                    bezier = bezier + 1;
                    setBezierBinary(
                        input,
                        &mut (*timeline_7).super_0.super_0,
                        fresh130,
                        frame,
                        0 as c_int,
                        time_6,
                        time2_5,
                        mixRotate,
                        mixRotate2,
                        1 as c_int as c_float,
                    );
                    let fresh131 = bezier;
                    bezier = bezier + 1;
                    setBezierBinary(
                        input,
                        &mut (*timeline_7).super_0.super_0,
                        fresh131,
                        frame,
                        1 as c_int,
                        time_6,
                        time2_5,
                        mixX,
                        mixX2,
                        1 as c_int as c_float,
                    );
                    let fresh132 = bezier;
                    bezier = bezier + 1;
                    setBezierBinary(
                        input,
                        &mut (*timeline_7).super_0.super_0,
                        fresh132,
                        frame,
                        2 as c_int,
                        time_6,
                        time2_5,
                        mixY,
                        mixY2,
                        1 as c_int as c_float,
                    );
                    let fresh133 = bezier;
                    bezier = bezier + 1;
                    setBezierBinary(
                        input,
                        &mut (*timeline_7).super_0.super_0,
                        fresh133,
                        frame,
                        3 as c_int,
                        time_6,
                        time2_5,
                        mixScaleX,
                        mixScaleX2,
                        1 as c_int as c_float,
                    );
                    let fresh134 = bezier;
                    bezier = bezier + 1;
                    setBezierBinary(
                        input,
                        &mut (*timeline_7).super_0.super_0,
                        fresh134,
                        frame,
                        4 as c_int,
                        time_6,
                        time2_5,
                        mixScaleY,
                        mixScaleY2,
                        1 as c_int as c_float,
                    );
                    let fresh135 = bezier;
                    bezier = bezier + 1;
                    setBezierBinary(
                        input,
                        &mut (*timeline_7).super_0.super_0,
                        fresh135,
                        frame,
                        5 as c_int,
                        time_6,
                        time2_5,
                        mixShearY,
                        mixShearY2,
                        1 as c_int as c_float,
                    );
                }
                _ => {}
            }
            time_6 = time2_5;
            mixRotate = mixRotate2;
            mixX = mixX2;
            mixY = mixY2;
            mixScaleX = mixScaleX2;
            mixScaleY = mixScaleY2;
            mixShearY = mixShearY2;
            frame += 1;
        }
        spTimelineArray_add(timelines, &mut (*timeline_7).super_0.super_0);
        i += 1;
    }
    i = 0 as c_int;
    n = readVarint(input, 1 as c_int);
    while i < n {
        let mut index_1: c_int = readVarint(input, 1 as c_int);
        let mut data: *mut spPathConstraintData =
            *((*skeletonData).pathConstraints).offset(index_1 as isize);
        ii = 0 as c_int;
        nn = readVarint(input, 1 as c_int);
        while ii < nn {
            let mut type_0: c_int = readSByte(input) as c_int;
            let mut frameCount_3: c_int = readVarint(input, 1 as c_int);
            let mut bezierCount_7: c_int = readVarint(input, 1 as c_int);
            match type_0 {
                0 => {
                    spTimelineArray_add(
                        timelines,
                        readTimelineBinary(
                            input,
                            &mut (*(spPathConstraintPositionTimeline_create
                                as unsafe extern "C" fn(
                                    c_int,
                                    c_int,
                                    c_int,
                                )
                                    -> *mut spPathConstraintPositionTimeline)(
                                frameCount_3,
                                bezierCount_7,
                                index_1,
                            ))
                            .super_0,
                            if (*data).positionMode as c_uint
                                == SP_POSITION_MODE_FIXED as c_int as c_uint
                            {
                                scale
                            } else {
                                1 as c_int as c_float
                            },
                        ),
                    );
                }
                1 => {
                    spTimelineArray_add(
                        timelines,
                        readTimelineBinary(
                            input,
                            &mut (*(spPathConstraintSpacingTimeline_create
                                as unsafe extern "C" fn(
                                    c_int,
                                    c_int,
                                    c_int,
                                )
                                    -> *mut spPathConstraintSpacingTimeline)(
                                frameCount_3,
                                bezierCount_7,
                                index_1,
                            ))
                            .super_0,
                            if (*data).spacingMode as c_uint
                                == SP_SPACING_MODE_LENGTH as c_int as c_uint
                                || (*data).spacingMode as c_uint
                                    == SP_SPACING_MODE_FIXED as c_int as c_uint
                            {
                                scale
                            } else {
                                1 as c_int as c_float
                            },
                        ),
                    );
                }
                2 => {
                    let mut time_7: c_float = 0.;
                    let mut mixRotate_0: c_float = 0.;
                    let mut mixX_0: c_float = 0.;
                    let mut mixY_0: c_float = 0.;
                    let mut frameLast_2: c_int = 0;
                    let mut timeline_8: *mut spPathConstraintMixTimeline =
                        spPathConstraintMixTimeline_create(frameCount_3, bezierCount_7, index_1);
                    time_7 = readFloat(input);
                    mixRotate_0 = readFloat(input);
                    mixX_0 = readFloat(input);
                    mixY_0 = readFloat(input);
                    frame = 0 as c_int;
                    bezier = 0 as c_int;
                    frameLast_2 = (*timeline_8).super_0.super_0.frameCount - 1 as c_int;
                    loop {
                        let mut time2_6: c_float = 0.;
                        let mut mixRotate2_0: c_float = 0.;
                        let mut mixX2_0: c_float = 0.;
                        let mut mixY2_0: c_float = 0.;
                        spPathConstraintMixTimeline_setFrame(
                            timeline_8,
                            frame,
                            time_7,
                            mixRotate_0,
                            mixX_0,
                            mixY_0,
                        );
                        if frame == frameLast_2 {
                            break;
                        }
                        time2_6 = readFloat(input);
                        mixRotate2_0 = readFloat(input);
                        mixX2_0 = readFloat(input);
                        mixY2_0 = readFloat(input);
                        match readSByte(input) as c_int {
                            1 => {
                                spCurveTimeline_setStepped(&mut (*timeline_8).super_0, frame);
                            }
                            2 => {
                                let fresh136 = bezier;
                                bezier = bezier + 1;
                                setBezierBinary(
                                    input,
                                    &mut (*timeline_8).super_0.super_0,
                                    fresh136,
                                    frame,
                                    0 as c_int,
                                    time_7,
                                    time2_6,
                                    mixRotate_0,
                                    mixRotate2_0,
                                    1 as c_int as c_float,
                                );
                                let fresh137 = bezier;
                                bezier = bezier + 1;
                                setBezierBinary(
                                    input,
                                    &mut (*timeline_8).super_0.super_0,
                                    fresh137,
                                    frame,
                                    1 as c_int,
                                    time_7,
                                    time2_6,
                                    mixX_0,
                                    mixX2_0,
                                    1 as c_int as c_float,
                                );
                                let fresh138 = bezier;
                                bezier = bezier + 1;
                                setBezierBinary(
                                    input,
                                    &mut (*timeline_8).super_0.super_0,
                                    fresh138,
                                    frame,
                                    2 as c_int,
                                    time_7,
                                    time2_6,
                                    mixY_0,
                                    mixY2_0,
                                    1 as c_int as c_float,
                                );
                            }
                            _ => {}
                        }
                        time_7 = time2_6;
                        mixRotate_0 = mixRotate2_0;
                        mixX_0 = mixX2_0;
                        mixY_0 = mixY2_0;
                        frame += 1;
                    }
                    spTimelineArray_add(timelines, &mut (*timeline_8).super_0.super_0);
                }
                _ => {}
            }
            ii += 1;
        }
        i += 1;
    }
    i = 0 as c_int;
    n = readVarint(input, 1 as c_int);
    while i < n {
        let mut skin: *mut spSkin =
            *((*skeletonData).skins).offset(readVarint(input, 1 as c_int) as isize);
        ii = 0 as c_int;
        nn = readVarint(input, 1 as c_int);
        while ii < nn {
            let mut slotIndex_0: c_int = readVarint(input, 1 as c_int);
            iii = 0 as c_int;
            nnn = readVarint(input, 1 as c_int);
            while iii < nnn {
                let mut frameCount_4: c_int = 0;
                let mut frameLast_3: c_int = 0;
                let mut bezierCount_8: c_int = 0;
                let mut time_8: c_float = 0.;
                let mut time2_7: c_float = 0.;
                let mut timelineType_1: c_uint = 0;
                let mut attachmentName_0: *const c_char = readStringRef(input, skeletonData);
                let mut attachment: *mut spVertexAttachment =
                    spSkin_getAttachment(skin, slotIndex_0, attachmentName_0)
                        as *mut spVertexAttachment;
                if attachment.is_null() {
                    i = 0 as c_int;
                    while i < (*timelines).size {
                        spTimeline_dispose(*((*timelines).items).offset(i as isize));
                        i += 1;
                    }
                    spTimelineArray_dispose(timelines);
                    _spSkeletonBinary_setError(
                        self_0,
                        b"Attachment not found: \0" as *const u8 as *const c_char,
                        attachmentName_0,
                    );
                    return 0 as *mut spAnimation;
                }
                timelineType_1 = readByte(input) as c_uint;
                frameCount_4 = readVarint(input, 1 as c_int);
                frameLast_3 = frameCount_4 - 1 as c_int;
                match timelineType_1 {
                    0 => {
                        let mut tempDeform: *mut c_float = 0 as *mut c_float;
                        let mut weighted: c_int = 0;
                        let mut deformLength: c_int = 0;
                        let mut timeline_9: *mut spDeformTimeline = 0 as *mut spDeformTimeline;
                        weighted = ((*attachment).bones != 0 as *mut c_int) as c_int;
                        deformLength = if weighted != 0 {
                            (*attachment).verticesCount / 3 as c_int * 2 as c_int
                        } else {
                            (*attachment).verticesCount
                        };
                        tempDeform = _spMalloc(
                            (::core::mem::size_of::<c_float>() as c_ulong)
                                .wrapping_mul(deformLength as c_ulong),
                            b"spine.c\0" as *const u8 as *const c_char,
                            9101 as c_int,
                        ) as *mut c_float;
                        bezierCount_8 = readVarint(input, 1 as c_int);
                        timeline_9 = spDeformTimeline_create(
                            frameCount_4,
                            deformLength,
                            bezierCount_8,
                            slotIndex_0,
                            attachment,
                        );
                        time_8 = readFloat(input);
                        frame = 0 as c_int;
                        bezier = 0 as c_int;
                        loop {
                            let mut deform: *mut c_float = 0 as *mut c_float;
                            let mut end: c_int = readVarint(input, 1 as c_int);
                            if end == 0 {
                                if weighted != 0 {
                                    deform = tempDeform;
                                    spine_memset(
                                        deform as *mut c_void,
                                        0 as c_int,
                                        (::core::mem::size_of::<c_float>() as c_ulong)
                                            .wrapping_mul(deformLength as c_ulong),
                                    );
                                } else {
                                    deform = (*attachment).vertices;
                                }
                            } else {
                                let mut v: c_int = 0;
                                let mut start: c_int = readVarint(input, 1 as c_int);
                                deform = tempDeform;
                                spine_memset(
                                    deform as *mut c_void,
                                    0 as c_int,
                                    (::core::mem::size_of::<c_float>() as c_ulong)
                                        .wrapping_mul(start as c_ulong),
                                );
                                end += start;
                                if (*self_0).scale == 1 as c_int as c_float {
                                    v = start;
                                    while v < end {
                                        *deform.offset(v as isize) = readFloat(input);
                                        v += 1;
                                    }
                                } else {
                                    v = start;
                                    while v < end {
                                        *deform.offset(v as isize) =
                                            readFloat(input) * (*self_0).scale;
                                        v += 1;
                                    }
                                }
                                spine_memset(
                                    deform.offset(v as isize) as *mut c_void,
                                    0 as c_int,
                                    (::core::mem::size_of::<c_float>() as c_ulong)
                                        .wrapping_mul((deformLength - v) as c_ulong),
                                );
                                if weighted == 0 {
                                    let mut vertices: *mut c_float = (*attachment).vertices;
                                    v = 0 as c_int;
                                    while v < deformLength {
                                        *deform.offset(v as isize) += *vertices.offset(v as isize);
                                        v += 1;
                                    }
                                }
                            }
                            spDeformTimeline_setFrame(timeline_9, frame, time_8, deform);
                            if frame == frameLast_3 {
                                break;
                            }
                            time2_7 = readFloat(input);
                            match readSByte(input) as c_int {
                                1 => {
                                    spCurveTimeline_setStepped(&mut (*timeline_9).super_0, frame);
                                }
                                2 => {
                                    let fresh139 = bezier;
                                    bezier = bezier + 1;
                                    setBezierBinary(
                                        input,
                                        &mut (*timeline_9).super_0.super_0,
                                        fresh139,
                                        frame,
                                        0 as c_int,
                                        time_8,
                                        time2_7,
                                        0 as c_int as c_float,
                                        1 as c_int as c_float,
                                        1 as c_int as c_float,
                                    );
                                }
                                _ => {}
                            }
                            time_8 = time2_7;
                            frame += 1;
                        }
                        _spFree(tempDeform as *mut c_void);
                        spTimelineArray_add(timelines, timeline_9 as *mut spTimeline);
                    }
                    1 => {
                        let mut modeAndIndex: c_int = 0;
                        let mut delay: c_float = 0.;
                        let mut timeline_10: *mut spSequenceTimeline = spSequenceTimeline_create(
                            frameCount_4,
                            slotIndex_0,
                            attachment as *mut spAttachment,
                        );
                        frame = 0 as c_int;
                        while frame < frameCount_4 {
                            time_8 = readFloat(input);
                            modeAndIndex = readInt(input);
                            delay = readFloat(input);
                            spSequenceTimeline_setFrame(
                                timeline_10,
                                frame,
                                time_8,
                                modeAndIndex & 0xf as c_int,
                                modeAndIndex >> 4 as c_int,
                                delay,
                            );
                            frame += 1;
                        }
                        spTimelineArray_add(timelines, timeline_10 as *mut spTimeline);
                    }
                    _ => {}
                }
                iii += 1;
            }
            ii += 1;
        }
        i += 1;
    }
    drawOrderCount = readVarint(input, 1 as c_int);
    if drawOrderCount != 0 {
        let mut timeline_11: *mut spDrawOrderTimeline =
            spDrawOrderTimeline_create(drawOrderCount, (*skeletonData).slotsCount);
        i = 0 as c_int;
        while i < drawOrderCount {
            let mut time_9: c_float = readFloat(input);
            let mut offsetCount: c_int = readVarint(input, 1 as c_int);
            let mut drawOrder: *mut c_int = _spMalloc(
                (::core::mem::size_of::<c_int>() as c_ulong)
                    .wrapping_mul((*skeletonData).slotsCount as c_ulong),
                b"spine.c\0" as *const u8 as *const c_char,
                9178 as c_int,
            ) as *mut c_int;
            let mut unchanged: *mut c_int = _spMalloc(
                (::core::mem::size_of::<c_int>() as c_ulong)
                    .wrapping_mul(((*skeletonData).slotsCount - offsetCount) as c_ulong),
                b"spine.c\0" as *const u8 as *const c_char,
                9179 as c_int,
            ) as *mut c_int;
            let mut originalIndex: c_int = 0 as c_int;
            let mut unchangedIndex: c_int = 0 as c_int;
            spine_memset(
                drawOrder as *mut c_void,
                -(1 as c_int),
                (::core::mem::size_of::<c_int>() as c_ulong)
                    .wrapping_mul((*skeletonData).slotsCount as c_ulong),
            );
            ii = 0 as c_int;
            while ii < offsetCount {
                let mut slotIndex_1: c_int = readVarint(input, 1 as c_int);
                while originalIndex != slotIndex_1 {
                    let fresh140 = originalIndex;
                    originalIndex = originalIndex + 1;
                    let fresh141 = unchangedIndex;
                    unchangedIndex = unchangedIndex + 1;
                    *unchanged.offset(fresh141 as isize) = fresh140;
                }
                *drawOrder.offset((originalIndex + readVarint(input, 1 as c_int)) as isize) =
                    originalIndex;
                originalIndex += 1;
                ii += 1;
            }
            while originalIndex < (*skeletonData).slotsCount {
                let fresh142 = originalIndex;
                originalIndex = originalIndex + 1;
                let fresh143 = unchangedIndex;
                unchangedIndex = unchangedIndex + 1;
                *unchanged.offset(fresh143 as isize) = fresh142;
            }
            ii = (*skeletonData).slotsCount - 1 as c_int;
            while ii >= 0 as c_int {
                if *drawOrder.offset(ii as isize) == -(1 as c_int) {
                    unchangedIndex -= 1;
                    *drawOrder.offset(ii as isize) = *unchanged.offset(unchangedIndex as isize);
                }
                ii -= 1;
            }
            _spFree(unchanged as *mut c_void);
            spDrawOrderTimeline_setFrame(timeline_11, i, time_9, drawOrder);
            _spFree(drawOrder as *mut c_void);
            i += 1;
        }
        spTimelineArray_add(timelines, timeline_11 as *mut spTimeline);
    }
    eventCount = readVarint(input, 1 as c_int);
    if eventCount != 0 {
        let mut timeline_12: *mut spEventTimeline = spEventTimeline_create(eventCount);
        i = 0 as c_int;
        while i < eventCount {
            let mut time_10: c_float = readFloat(input);
            let mut eventData: *mut spEventData =
                *((*skeletonData).events).offset(readVarint(input, 1 as c_int) as isize);
            let mut event: *mut spEvent = spEvent_create(time_10, eventData);
            (*event).intValue = readVarint(input, 0 as c_int);
            (*event).floatValue = readFloat(input);
            if readBoolean(input) != 0 {
                (*event).stringValue = readString(input);
            } else {
                let ref mut fresh144 =
                    *(&mut (*event).stringValue as *mut *const c_char as *mut *mut c_char);
                *fresh144 = _spMalloc(
                    (::core::mem::size_of::<c_char>() as c_ulong).wrapping_mul(
                        (spine_strlen((*eventData).stringValue))
                            .wrapping_add(1 as c_int as c_ulong),
                    ),
                    b"spine.c\0" as *const u8 as *const c_char,
                    9218 as c_int,
                ) as *mut c_char;
                spine_strcpy(*fresh144, (*eventData).stringValue);
            }
            if !((*eventData).audioPath).is_null() {
                (*event).volume = readFloat(input);
                (*event).balance = readFloat(input);
            }
            spEventTimeline_setFrame(timeline_12, i, event);
            i += 1;
        }
        spTimelineArray_add(timelines, timeline_12 as *mut spTimeline);
    }
    duration = 0 as c_int as c_float;
    i = 0 as c_int;
    n = (*timelines).size;
    while i < n {
        duration = if duration > spTimeline_getDuration(*((*timelines).items).offset(i as isize)) {
            duration
        } else {
            spTimeline_getDuration(*((*timelines).items).offset(i as isize))
        };
        i += 1;
    }
    animation = spAnimation_create(name, timelines, duration);
    return animation;
}
unsafe extern "C" fn _readFloatArray(
    mut input: *mut _dataInput,
    mut n: c_int,
    mut scale: c_float,
) -> *mut c_float {
    let mut array: *mut c_float = _spMalloc(
        (::core::mem::size_of::<c_float>() as c_ulong).wrapping_mul(n as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        9237 as c_int,
    ) as *mut c_float;
    let mut i: c_int = 0;
    if scale == 1 as c_int as c_float {
        i = 0 as c_int;
        while i < n {
            *array.offset(i as isize) = readFloat(input);
            i += 1;
        }
    } else {
        i = 0 as c_int;
        while i < n {
            *array.offset(i as isize) = readFloat(input) * scale;
            i += 1;
        }
    }
    return array;
}
unsafe extern "C" fn _readShortArray(
    mut input: *mut _dataInput,
    mut length: *mut c_int,
) -> *mut c_short {
    let mut n: c_int = readVarint(input, 1 as c_int);
    let mut array: *mut c_short = _spMalloc(
        (::core::mem::size_of::<c_short>() as c_ulong).wrapping_mul(n as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        9250 as c_int,
    ) as *mut c_short;
    let mut i: c_int = 0;
    *length = n;
    i = 0 as c_int;
    while i < n {
        *array.offset(i as isize) = ((readByte(input) as c_int) << 8 as c_int) as c_short;
        let ref mut fresh145 = *array.offset(i as isize);
        *fresh145 = (*fresh145 as c_int | readByte(input) as c_int) as c_short;
        i += 1;
    }
    return array;
}
unsafe extern "C" fn _readVerticesBinary(
    mut self_0: *mut spSkeletonBinary,
    mut input: *mut _dataInput,
    mut bonesCount: *mut c_int,
    mut bones2: *mut *mut c_int,
    mut verticesCount: *mut c_int,
    mut vertices: *mut *mut c_float,
    mut worldVerticesLength: *mut c_int,
    mut vertexCount: c_int,
) {
    let mut i: c_int = 0;
    let mut ii: c_int = 0;
    let mut verticesLength: c_int = vertexCount << 1 as c_int;
    let mut weights: *mut spFloatArray = spFloatArray_create(8 as c_int);
    let mut bones: *mut spIntArray = spIntArray_create(8 as c_int);
    *worldVerticesLength = verticesLength;
    if readBoolean(input) == 0 {
        *verticesCount = verticesLength;
        *vertices = _readFloatArray(input, verticesLength, (*self_0).scale);
        *bonesCount = 0 as c_int;
        *bones2 = 0 as *mut c_int;
        spFloatArray_dispose(weights);
        spIntArray_dispose(bones);
        return;
    }
    spFloatArray_ensureCapacity(weights, verticesLength * 3 as c_int * 3 as c_int);
    spIntArray_ensureCapacity(bones, verticesLength * 3 as c_int);
    i = 0 as c_int;
    while i < vertexCount {
        let mut boneCount: c_int = readVarint(input, 1 as c_int);
        spIntArray_add(bones, boneCount);
        ii = 0 as c_int;
        while ii < boneCount {
            spIntArray_add(bones, readVarint(input, 1 as c_int));
            spFloatArray_add(weights, readFloat(input) * (*self_0).scale);
            spFloatArray_add(weights, readFloat(input) * (*self_0).scale);
            spFloatArray_add(weights, readFloat(input));
            ii += 1;
        }
        i += 1;
    }
    *verticesCount = (*weights).size;
    *vertices = (*weights).items;
    _spFree(weights as *mut c_void);
    *bonesCount = (*bones).size;
    *bones2 = (*bones).items;
    _spFree(bones as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonBinary_readAttachment(
    mut self_0: *mut spSkeletonBinary,
    mut input: *mut _dataInput,
    mut skin: *mut spSkin,
    mut slotIndex: c_int,
    mut attachmentName: *const c_char,
    mut skeletonData: *mut spSkeletonData,
    mut nonessential: c_int,
) -> *mut spAttachment {
    let mut i: c_int = 0;
    let mut type_0: spAttachmentType = SP_ATTACHMENT_REGION;
    let mut name: *const c_char = readStringRef(input, skeletonData);
    if name.is_null() {
        name = attachmentName;
    }
    type_0 = readByte(input) as spAttachmentType;
    match type_0 as c_uint {
        0 => {
            let mut path: *const c_char = readStringRef(input, skeletonData);
            let mut rotation: c_float = 0.;
            let mut x: c_float = 0.;
            let mut y: c_float = 0.;
            let mut scaleX: c_float = 0.;
            let mut scaleY: c_float = 0.;
            let mut width: c_float = 0.;
            let mut height: c_float = 0.;
            let mut color: spColor = spColor {
                r: 0.,
                g: 0.,
                b: 0.,
                a: 0.,
            };
            let mut sequence: *mut spSequence = 0 as *mut spSequence;
            if path.is_null() {
                let ref mut fresh146 = *(&mut path as *mut *const c_char as *mut *mut c_char);
                *fresh146 = _spMalloc(
                    (::core::mem::size_of::<c_char>() as c_ulong)
                        .wrapping_mul((spine_strlen(name)).wrapping_add(1 as c_int as c_ulong)),
                    b"spine.c\0" as *const u8 as *const c_char,
                    9318 as c_int,
                ) as *mut c_char;
                spine_strcpy(*fresh146, name);
            } else {
                let mut tmp: *const c_char = 0 as *const c_char;
                let ref mut fresh147 = *(&mut tmp as *mut *const c_char as *mut *mut c_char);
                *fresh147 = _spMalloc(
                    (::core::mem::size_of::<c_char>() as c_ulong)
                        .wrapping_mul((spine_strlen(path)).wrapping_add(1 as c_int as c_ulong)),
                    b"spine.c\0" as *const u8 as *const c_char,
                    9321 as c_int,
                ) as *mut c_char;
                spine_strcpy(*fresh147, path);
                path = tmp;
            }
            rotation = readFloat(input);
            x = readFloat(input) * (*self_0).scale;
            y = readFloat(input) * (*self_0).scale;
            scaleX = readFloat(input);
            scaleY = readFloat(input);
            width = readFloat(input) * (*self_0).scale;
            height = readFloat(input) * (*self_0).scale;
            readColor(
                input,
                &mut color.r,
                &mut color.g,
                &mut color.b,
                &mut color.a,
            );
            sequence = readSequenceBinary(input);
            let mut attachment: *mut spAttachment = spAttachmentLoader_createAttachment(
                (*self_0).attachmentLoader,
                skin,
                type_0,
                name,
                path,
                sequence,
            );
            let mut region: *mut spRegionAttachment = attachment as *mut spRegionAttachment;
            (*region).path = path;
            (*region).rotation = rotation;
            (*region).x = x;
            (*region).y = y;
            (*region).scaleX = scaleX;
            (*region).scaleY = scaleY;
            (*region).width = width;
            (*region).height = height;
            spColor_setFromColor(&mut (*region).color, &mut color);
            (*region).sequence = sequence;
            if sequence.is_null() {
                spRegionAttachment_updateRegion(region);
            }
            spAttachmentLoader_configureAttachment((*self_0).attachmentLoader, attachment);
            return attachment;
        }
        1 => {
            let mut vertexCount: c_int = readVarint(input, 1 as c_int);
            let mut attachment_0: *mut spAttachment = spAttachmentLoader_createAttachment(
                (*self_0).attachmentLoader,
                skin,
                type_0,
                name,
                0 as *const c_char,
                0 as *mut spSequence,
            );
            let mut vertexAttachment: *mut spVertexAttachment =
                attachment_0 as *mut spVertexAttachment;
            _readVerticesBinary(
                self_0,
                input,
                &mut (*vertexAttachment).bonesCount,
                &mut (*vertexAttachment).bones,
                &mut (*vertexAttachment).verticesCount,
                &mut (*vertexAttachment).vertices,
                &mut (*vertexAttachment).worldVerticesLength,
                vertexCount,
            );
            if nonessential != 0 {
                let mut bbox: *mut spBoundingBoxAttachment =
                    attachment_0 as *mut spBoundingBoxAttachment;
                readColor(
                    input,
                    &mut (*bbox).color.r,
                    &mut (*bbox).color.g,
                    &mut (*bbox).color.b,
                    &mut (*bbox).color.a,
                );
            }
            spAttachmentLoader_configureAttachment((*self_0).attachmentLoader, attachment_0);
            return attachment_0;
        }
        2 => {
            let mut vertexCount_0: c_int = 0;
            let mut path_0: *const c_char = readStringRef(input, skeletonData);
            let mut color_0: spColor = spColor {
                r: 0.,
                g: 0.,
                b: 0.,
                a: 0.,
            };
            let mut regionUVs: *mut c_float = 0 as *mut c_float;
            let mut triangles: *mut c_ushort = 0 as *mut c_ushort;
            let mut trianglesCount: c_int = 0;
            let mut bones: *mut c_int = 0 as *mut c_int;
            let mut bonesCount: c_int = 0;
            let mut vertices: *mut c_float = 0 as *mut c_float;
            let mut verticesCount: c_int = 0;
            let mut worldVerticesLength: c_int = 0;
            let mut hullLength: c_int = 0;
            let mut sequence_0: *mut spSequence = 0 as *mut spSequence;
            let mut edges: *mut c_int = 0 as *mut c_int;
            let mut edgesCount: c_int = 0 as c_int;
            let mut width_0: c_float = 0 as c_int as c_float;
            let mut height_0: c_float = 0 as c_int as c_float;
            if path_0.is_null() {
                let ref mut fresh148 = *(&mut path_0 as *mut *const c_char as *mut *mut c_char);
                *fresh148 = _spMalloc(
                    (::core::mem::size_of::<c_char>() as c_ulong)
                        .wrapping_mul((spine_strlen(name)).wrapping_add(1 as c_int as c_ulong)),
                    b"spine.c\0" as *const u8 as *const c_char,
                    9386 as c_int,
                ) as *mut c_char;
                spine_strcpy(*fresh148, name);
            } else {
                let mut tmp_0: *const c_char = 0 as *const c_char;
                let ref mut fresh149 = *(&mut tmp_0 as *mut *const c_char as *mut *mut c_char);
                *fresh149 = _spMalloc(
                    (::core::mem::size_of::<c_char>() as c_ulong)
                        .wrapping_mul((spine_strlen(path_0)).wrapping_add(1 as c_int as c_ulong)),
                    b"spine.c\0" as *const u8 as *const c_char,
                    9389 as c_int,
                ) as *mut c_char;
                spine_strcpy(*fresh149, path_0);
                path_0 = tmp_0;
            }
            readColor(
                input,
                &mut color_0.r,
                &mut color_0.g,
                &mut color_0.b,
                &mut color_0.a,
            );
            vertexCount_0 = readVarint(input, 1 as c_int);
            regionUVs = _readFloatArray(input, vertexCount_0 << 1 as c_int, 1 as c_int as c_float);
            triangles = _readShortArray(input, &mut trianglesCount) as *mut c_ushort;
            _readVerticesBinary(
                self_0,
                input,
                &mut bonesCount,
                &mut bones,
                &mut verticesCount,
                &mut vertices,
                &mut worldVerticesLength,
                vertexCount_0,
            );
            hullLength = readVarint(input, 1 as c_int) << 1 as c_int;
            sequence_0 = readSequenceBinary(input);
            if nonessential != 0 {
                edges = _readShortArray(input, &mut edgesCount) as *mut c_int;
                width_0 = readFloat(input) * (*self_0).scale;
                height_0 = readFloat(input) * (*self_0).scale;
            }
            let mut attachment_1: *mut spAttachment = spAttachmentLoader_createAttachment(
                (*self_0).attachmentLoader,
                skin,
                type_0,
                name,
                path_0,
                sequence_0,
            );
            let mut mesh: *mut spMeshAttachment = attachment_1 as *mut spMeshAttachment;
            (*mesh).path = path_0;
            spColor_setFromColor(&mut (*mesh).color, &mut color_0);
            (*mesh).regionUVs = regionUVs;
            (*mesh).triangles = triangles;
            (*mesh).trianglesCount = trianglesCount;
            (*mesh).super_0.vertices = vertices;
            (*mesh).super_0.verticesCount = verticesCount;
            (*mesh).super_0.bones = bones;
            (*mesh).super_0.bonesCount = bonesCount;
            (*mesh).super_0.worldVerticesLength = worldVerticesLength;
            (*mesh).hullLength = hullLength;
            (*mesh).edges = edges;
            (*mesh).edgesCount = edgesCount;
            (*mesh).width = width_0;
            (*mesh).height = height_0;
            (*mesh).sequence = sequence_0;
            if sequence_0.is_null() {
                spMeshAttachment_updateRegion(mesh);
            }
            spAttachmentLoader_configureAttachment((*self_0).attachmentLoader, attachment_1);
            return attachment_1;
        }
        3 => {
            let mut color_1: spColor = spColor {
                r: 0.,
                g: 0.,
                b: 0.,
                a: 0.,
            };
            let mut width_1: c_float = 0 as c_int as c_float;
            let mut height_1: c_float = 0 as c_int as c_float;
            let mut skinName: *const c_char = 0 as *const c_char;
            let mut parent: *const c_char = 0 as *const c_char;
            let mut inheritTimeline: c_int = 0;
            let mut sequence_1: *mut spSequence = 0 as *mut spSequence;
            let mut path_1: *const c_char = readStringRef(input, skeletonData);
            if path_1.is_null() {
                let ref mut fresh150 = *(&mut path_1 as *mut *const c_char as *mut *mut c_char);
                *fresh150 = _spMalloc(
                    (::core::mem::size_of::<c_char>() as c_ulong)
                        .wrapping_mul((spine_strlen(name)).wrapping_add(1 as c_int as c_ulong)),
                    b"spine.c\0" as *const u8 as *const c_char,
                    9438 as c_int,
                ) as *mut c_char;
                spine_strcpy(*fresh150, name);
            } else {
                let mut tmp_1: *const c_char = 0 as *const c_char;
                let ref mut fresh151 = *(&mut tmp_1 as *mut *const c_char as *mut *mut c_char);
                *fresh151 = _spMalloc(
                    (::core::mem::size_of::<c_char>() as c_ulong)
                        .wrapping_mul((spine_strlen(path_1)).wrapping_add(1 as c_int as c_ulong)),
                    b"spine.c\0" as *const u8 as *const c_char,
                    9441 as c_int,
                ) as *mut c_char;
                spine_strcpy(*fresh151, path_1);
                path_1 = tmp_1;
            }
            readColor(
                input,
                &mut color_1.r,
                &mut color_1.g,
                &mut color_1.b,
                &mut color_1.a,
            );
            skinName = readStringRef(input, skeletonData);
            parent = readStringRef(input, skeletonData);
            inheritTimeline = readBoolean(input);
            sequence_1 = readSequenceBinary(input);
            if nonessential != 0 {
                width_1 = readFloat(input) * (*self_0).scale;
                height_1 = readFloat(input) * (*self_0).scale;
            }
            let mut attachment_2: *mut spAttachment = spAttachmentLoader_createAttachment(
                (*self_0).attachmentLoader,
                skin,
                type_0,
                name,
                path_1,
                sequence_1,
            );
            let mut mesh_0: *mut spMeshAttachment = attachment_2 as *mut spMeshAttachment;
            (*mesh_0).path = path_1;
            spColor_setFromColor(&mut (*mesh_0).color, &mut color_1);
            (*mesh_0).sequence = sequence_1;
            (*mesh_0).width = width_1;
            (*mesh_0).height = height_1;
            _spSkeletonBinary_addLinkedMesh(
                self_0,
                mesh_0,
                skinName,
                slotIndex,
                parent,
                inheritTimeline,
            );
            return attachment_2;
        }
        4 => {
            let mut attachment_3: *mut spAttachment = spAttachmentLoader_createAttachment(
                (*self_0).attachmentLoader,
                skin,
                type_0,
                name,
                0 as *const c_char,
                0 as *mut spSequence,
            );
            let mut path_2: *mut spPathAttachment = attachment_3 as *mut spPathAttachment;
            let mut vertexAttachment_0: *mut spVertexAttachment = &mut (*path_2).super_0;
            let mut vertexCount_1: c_int = 0 as c_int;
            (*path_2).closed = readBoolean(input);
            (*path_2).constantSpeed = readBoolean(input);
            vertexCount_1 = readVarint(input, 1 as c_int);
            _readVerticesBinary(
                self_0,
                input,
                &mut (*vertexAttachment_0).bonesCount,
                &mut (*vertexAttachment_0).bones,
                &mut (*vertexAttachment_0).verticesCount,
                &mut (*vertexAttachment_0).vertices,
                &mut (*vertexAttachment_0).worldVerticesLength,
                vertexCount_1,
            );
            (*path_2).lengthsLength = vertexCount_1 / 3 as c_int;
            (*path_2).lengths = _spMalloc(
                (::core::mem::size_of::<c_float>() as c_ulong)
                    .wrapping_mul((*path_2).lengthsLength as c_ulong),
                b"spine.c\0" as *const u8 as *const c_char,
                9480 as c_int,
            ) as *mut c_float;
            i = 0 as c_int;
            while i < (*path_2).lengthsLength {
                *((*path_2).lengths).offset(i as isize) = readFloat(input) * (*self_0).scale;
                i += 1;
            }
            if nonessential != 0 {
                readColor(
                    input,
                    &mut (*path_2).color.r,
                    &mut (*path_2).color.g,
                    &mut (*path_2).color.b,
                    &mut (*path_2).color.a,
                );
            }
            spAttachmentLoader_configureAttachment((*self_0).attachmentLoader, attachment_3);
            return attachment_3;
        }
        5 => {
            let mut attachment_4: *mut spAttachment = spAttachmentLoader_createAttachment(
                (*self_0).attachmentLoader,
                skin,
                type_0,
                name,
                0 as *const c_char,
                0 as *mut spSequence,
            );
            let mut point: *mut spPointAttachment = attachment_4 as *mut spPointAttachment;
            (*point).rotation = readFloat(input);
            (*point).x = readFloat(input) * (*self_0).scale;
            (*point).y = readFloat(input) * (*self_0).scale;
            if nonessential != 0 {
                readColor(
                    input,
                    &mut (*point).color.r,
                    &mut (*point).color.g,
                    &mut (*point).color.b,
                    &mut (*point).color.a,
                );
            }
            spAttachmentLoader_configureAttachment((*self_0).attachmentLoader, attachment_4);
            return attachment_4;
        }
        6 => {
            let mut endSlotIndex: c_int = readVarint(input, 1 as c_int);
            let mut vertexCount_2: c_int = readVarint(input, 1 as c_int);
            let mut attachment_5: *mut spAttachment = spAttachmentLoader_createAttachment(
                (*self_0).attachmentLoader,
                skin,
                type_0,
                name,
                0 as *const c_char,
                0 as *mut spSequence,
            );
            let mut clip: *mut spClippingAttachment = attachment_5 as *mut spClippingAttachment;
            let mut vertexAttachment_1: *mut spVertexAttachment = &mut (*clip).super_0;
            _readVerticesBinary(
                self_0,
                input,
                &mut (*vertexAttachment_1).bonesCount,
                &mut (*vertexAttachment_1).bones,
                &mut (*vertexAttachment_1).verticesCount,
                &mut (*vertexAttachment_1).vertices,
                &mut (*vertexAttachment_1).worldVerticesLength,
                vertexCount_2,
            );
            if nonessential != 0 {
                readColor(
                    input,
                    &mut (*clip).color.r,
                    &mut (*clip).color.g,
                    &mut (*clip).color.b,
                    &mut (*clip).color.a,
                );
            }
            (*clip).endSlot = *((*skeletonData).slots).offset(endSlotIndex as isize);
            spAttachmentLoader_configureAttachment((*self_0).attachmentLoader, attachment_5);
            return attachment_5;
        }
        _ => {}
    }
    return 0 as *mut spAttachment;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonBinary_readSkin(
    mut self_0: *mut spSkeletonBinary,
    mut input: *mut _dataInput,
    mut defaultSkin: c_int,
    mut skeletonData: *mut spSkeletonData,
    mut nonessential: c_int,
) -> *mut spSkin {
    let mut skin: *mut spSkin = 0 as *mut spSkin;
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    let mut ii: c_int = 0;
    let mut nn: c_int = 0;
    let mut slotCount: c_int = 0;
    if defaultSkin != 0 {
        slotCount = readVarint(input, 1 as c_int);
        if slotCount == 0 as c_int {
            return 0 as *mut spSkin;
        }
        skin = spSkin_create(b"default\0" as *const u8 as *const c_char);
    } else {
        skin = spSkin_create(readStringRef(input, skeletonData));
        i = 0 as c_int;
        n = readVarint(input, 1 as c_int);
        while i < n {
            spBoneDataArray_add(
                (*skin).bones,
                *((*skeletonData).bones).offset(readVarint(input, 1 as c_int) as isize),
            );
            i += 1;
        }
        i = 0 as c_int;
        n = readVarint(input, 1 as c_int);
        while i < n {
            spIkConstraintDataArray_add(
                (*skin).ikConstraints,
                *((*skeletonData).ikConstraints).offset(readVarint(input, 1 as c_int) as isize),
            );
            i += 1;
        }
        i = 0 as c_int;
        n = readVarint(input, 1 as c_int);
        while i < n {
            spTransformConstraintDataArray_add(
                (*skin).transformConstraints,
                *((*skeletonData).transformConstraints)
                    .offset(readVarint(input, 1 as c_int) as isize),
            );
            i += 1;
        }
        i = 0 as c_int;
        n = readVarint(input, 1 as c_int);
        while i < n {
            spPathConstraintDataArray_add(
                (*skin).pathConstraints,
                *((*skeletonData).pathConstraints).offset(readVarint(input, 1 as c_int) as isize),
            );
            i += 1;
        }
        slotCount = readVarint(input, 1 as c_int);
    }
    i = 0 as c_int;
    while i < slotCount {
        let mut slotIndex: c_int = readVarint(input, 1 as c_int);
        ii = 0 as c_int;
        nn = readVarint(input, 1 as c_int);
        while ii < nn {
            let mut name: *const c_char = readStringRef(input, skeletonData);
            let mut attachment: *mut spAttachment = spSkeletonBinary_readAttachment(
                self_0,
                input,
                skin,
                slotIndex,
                name,
                skeletonData,
                nonessential,
            );
            if !attachment.is_null() {
                spSkin_setAttachment(skin, slotIndex, name, attachment);
            }
            ii += 1;
        }
        i += 1;
    }
    return skin;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonBinary_readSkeletonDataFile(
    mut self_0: *mut spSkeletonBinary,
    mut path: *const c_char,
) -> *mut spSkeletonData {
    let mut length: c_int = 0;
    let mut skeletonData: *mut spSkeletonData = 0 as *mut spSkeletonData;
    let mut binary: *const c_char = _spUtil_readFile(path, &mut length);
    if length == 0 as c_int || binary.is_null() {
        _spSkeletonBinary_setError(
            self_0,
            b"Unable to read skeleton file: \0" as *const u8 as *const c_char,
            path,
        );
        return 0 as *mut spSkeletonData;
    }
    skeletonData = spSkeletonBinary_readSkeletonData(self_0, binary as *mut c_uchar, length);
    _spFree(binary as *mut c_void);
    return skeletonData;
}
unsafe extern "C" fn string_starts_with_binary(
    mut str: *const c_char,
    mut needle: *const c_char,
) -> c_int {
    let mut lenStr: c_int = 0;
    let mut lenNeedle: c_int = 0;
    let mut i: c_int = 0;
    if str.is_null() {
        return 0 as c_int;
    }
    lenStr = spine_strlen(str) as c_int;
    lenNeedle = spine_strlen(needle) as c_int;
    if lenStr < lenNeedle {
        return 0 as c_int;
    }
    i = 0 as c_int;
    while i < lenNeedle {
        if *str.offset(i as isize) as c_int != *needle.offset(i as isize) as c_int {
            return 0 as c_int;
        }
        i += 1;
    }
    return -(1 as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonBinary_readSkeletonData(
    mut self_0: *mut spSkeletonBinary,
    mut binary: *const c_uchar,
    length: c_int,
) -> *mut spSkeletonData {
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    let mut ii: c_int = 0;
    let mut nonessential: c_int = 0;
    let mut buffer: [c_char; 32] = [0; 32];
    let mut lowHash: c_int = 0;
    let mut highHash: c_int = 0;
    let mut skeletonData: *mut spSkeletonData = 0 as *mut spSkeletonData;
    let mut internal: *mut _spSkeletonBinary = self_0 as *mut _spSkeletonBinary;
    let mut input: *mut _dataInput = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<_dataInput>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        9598 as c_int,
    ) as *mut _dataInput;
    (*input).cursor = binary;
    (*input).end = binary.offset(length as isize);
    _spFree((*self_0).error as *mut c_void);
    let ref mut fresh152 = *(&(*self_0).error as *const *const c_char as *mut *mut c_char);
    *fresh152 = 0 as *mut c_char;
    (*internal).linkedMeshCount = 0 as c_int;
    skeletonData = spSkeletonData_create();
    lowHash = readInt(input);
    highHash = readInt(input);
    spine_sprintf!(
        buffer.as_mut_ptr(),
        b"%x%x\0" as *const u8 as *const c_char,
        highHash,
        lowHash,
    );
    buffer[31 as c_int as usize] = 0 as c_int as c_char;
    let ref mut fresh153 = *(&mut (*skeletonData).hash as *mut *const c_char as *mut *mut c_char);
    *fresh153 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul((spine_strlen(buffer.as_mut_ptr())).wrapping_add(1 as c_int as c_ulong)),
        b"spine.c\0" as *const u8 as *const c_char,
        9611 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh153, buffer.as_mut_ptr());
    (*skeletonData).version = readString(input);
    if spine_strlen((*skeletonData).version) == 0 {
        _spFree((*skeletonData).version as *mut c_void);
        (*skeletonData).version = 0 as *const c_char;
    } else if string_starts_with_binary(
        (*skeletonData).version,
        b"4.1\0" as *const u8 as *const c_char,
    ) == 0
    {
        let mut errorMsg: [c_char; 255] = [0; 255];
        spine_sprintf!(
            errorMsg.as_mut_ptr(),
            b"Skeleton version %s does not match runtime version %s\0" as *const u8
                as *const c_char,
            (*skeletonData).version,
            b"4.1\0" as *const u8 as *const c_char,
        );
        _spSkeletonBinary_setError(self_0, errorMsg.as_mut_ptr(), 0 as *const c_char);
        return 0 as *mut spSkeletonData;
    }
    (*skeletonData).x = readFloat(input);
    (*skeletonData).y = readFloat(input);
    (*skeletonData).width = readFloat(input);
    (*skeletonData).height = readFloat(input);
    nonessential = readBoolean(input);
    if nonessential != 0 {
        (*skeletonData).fps = readFloat(input);
        (*skeletonData).imagesPath = readString(input);
        if spine_strlen((*skeletonData).imagesPath) == 0 {
            _spFree((*skeletonData).imagesPath as *mut c_void);
            (*skeletonData).imagesPath = 0 as *const c_char;
        }
        (*skeletonData).audioPath = readString(input);
        if spine_strlen((*skeletonData).audioPath) == 0 {
            _spFree((*skeletonData).audioPath as *mut c_void);
            (*skeletonData).audioPath = 0 as *const c_char;
        }
    }
    n = readVarint(input, 1 as c_int);
    (*skeletonData).stringsCount = n;
    (*skeletonData).strings = _spMalloc(
        (::core::mem::size_of::<*mut c_char>() as c_ulong)
            .wrapping_mul((*skeletonData).stringsCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        9648 as c_int,
    ) as *mut *mut c_char;
    i = 0 as c_int;
    while i < n {
        let ref mut fresh154 = *((*skeletonData).strings).offset(i as isize);
        *fresh154 = readString(input);
        i += 1;
    }
    (*skeletonData).bonesCount = readVarint(input, 1 as c_int);
    (*skeletonData).bones = _spMalloc(
        (::core::mem::size_of::<*mut spBoneData>() as c_ulong)
            .wrapping_mul((*skeletonData).bonesCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        9655 as c_int,
    ) as *mut *mut spBoneData;
    i = 0 as c_int;
    while i < (*skeletonData).bonesCount {
        let mut data: *mut spBoneData = 0 as *mut spBoneData;
        let mut mode: c_int = 0;
        let mut name: *const c_char = readString(input);
        let mut parent: *mut spBoneData = if i == 0 as c_int {
            0 as *mut spBoneData
        } else {
            *((*skeletonData).bones).offset(readVarint(input, 1 as c_int) as isize)
        };
        data = spBoneData_create(i, name, parent);
        _spFree(name as *mut c_void);
        (*data).rotation = readFloat(input);
        (*data).x = readFloat(input) * (*self_0).scale;
        (*data).y = readFloat(input) * (*self_0).scale;
        (*data).scaleX = readFloat(input);
        (*data).scaleY = readFloat(input);
        (*data).shearX = readFloat(input);
        (*data).shearY = readFloat(input);
        (*data).length = readFloat(input) * (*self_0).scale;
        mode = readVarint(input, 1 as c_int);
        match mode {
            0 => {
                (*data).transformMode = SP_TRANSFORMMODE_NORMAL;
            }
            1 => {
                (*data).transformMode = SP_TRANSFORMMODE_ONLYTRANSLATION;
            }
            2 => {
                (*data).transformMode = SP_TRANSFORMMODE_NOROTATIONORREFLECTION;
            }
            3 => {
                (*data).transformMode = SP_TRANSFORMMODE_NOSCALE;
            }
            4 => {
                (*data).transformMode = SP_TRANSFORMMODE_NOSCALEORREFLECTION;
            }
            _ => {}
        }
        (*data).skinRequired = readBoolean(input);
        if nonessential != 0 {
            readColor(
                input,
                &mut (*data).color.r,
                &mut (*data).color.g,
                &mut (*data).color.b,
                &mut (*data).color.a,
            );
        }
        let ref mut fresh155 = *((*skeletonData).bones).offset(i as isize);
        *fresh155 = data;
        i += 1;
    }
    (*skeletonData).slotsCount = readVarint(input, 1 as c_int);
    (*skeletonData).slots = _spMalloc(
        (::core::mem::size_of::<*mut spSlotData>() as c_ulong)
            .wrapping_mul((*skeletonData).slotsCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        9699 as c_int,
    ) as *mut *mut spSlotData;
    i = 0 as c_int;
    while i < (*skeletonData).slotsCount {
        let mut r: c_int = 0;
        let mut g: c_int = 0;
        let mut b: c_int = 0;
        let mut a: c_int = 0;
        let mut attachmentName: *const c_char = 0 as *const c_char;
        let mut slotName: *const c_char = readString(input);
        let mut boneData: *mut spBoneData =
            *((*skeletonData).bones).offset(readVarint(input, 1 as c_int) as isize);
        let mut slotData: *mut spSlotData = spSlotData_create(i, slotName, boneData);
        _spFree(slotName as *mut c_void);
        readColor(
            input,
            &mut (*slotData).color.r,
            &mut (*slotData).color.g,
            &mut (*slotData).color.b,
            &mut (*slotData).color.a,
        );
        a = readByte(input) as c_int;
        r = readByte(input) as c_int;
        g = readByte(input) as c_int;
        b = readByte(input) as c_int;
        if !(r == 0xff as c_int && g == 0xff as c_int && b == 0xff as c_int && a == 0xff as c_int) {
            (*slotData).darkColor = spColor_create();
            spColor_setFromFloats(
                (*slotData).darkColor,
                r as c_float / 255.0f32,
                g as c_float / 255.0f32,
                b as c_float / 255.0f32,
                1 as c_int as c_float,
            );
        }
        attachmentName = readStringRef(input, skeletonData);
        if !attachmentName.is_null() {
            let ref mut fresh156 =
                *(&mut (*slotData).attachmentName as *mut *const c_char as *mut *mut c_char);
            *fresh156 = _spMalloc(
                (::core::mem::size_of::<c_char>() as c_ulong).wrapping_mul(
                    (spine_strlen(attachmentName)).wrapping_add(1 as c_int as c_ulong),
                ),
                b"spine.c\0" as *const u8 as *const c_char,
                9718 as c_int,
            ) as *mut c_char;
            spine_strcpy(*fresh156, attachmentName);
        } else {
            (*slotData).attachmentName = 0 as *const c_char;
        }
        (*slotData).blendMode = readVarint(input, 1 as c_int) as spBlendMode;
        let ref mut fresh157 = *((*skeletonData).slots).offset(i as isize);
        *fresh157 = slotData;
        i += 1;
    }
    (*skeletonData).ikConstraintsCount = readVarint(input, 1 as c_int);
    (*skeletonData).ikConstraints = _spMalloc(
        (::core::mem::size_of::<*mut spIkConstraintData>() as c_ulong)
            .wrapping_mul((*skeletonData).ikConstraintsCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        9727 as c_int,
    ) as *mut *mut spIkConstraintData;
    i = 0 as c_int;
    while i < (*skeletonData).ikConstraintsCount {
        let mut name_0: *const c_char = readString(input);
        let mut data_0: *mut spIkConstraintData = spIkConstraintData_create(name_0);
        (*data_0).order = readVarint(input, 1 as c_int);
        (*data_0).skinRequired = readBoolean(input);
        _spFree(name_0 as *mut c_void);
        (*data_0).bonesCount = readVarint(input, 1 as c_int);
        (*data_0).bones = _spMalloc(
            (::core::mem::size_of::<*mut spBoneData>() as c_ulong)
                .wrapping_mul((*data_0).bonesCount as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            9736 as c_int,
        ) as *mut *mut spBoneData;
        ii = 0 as c_int;
        while ii < (*data_0).bonesCount {
            let ref mut fresh158 = *((*data_0).bones).offset(ii as isize);
            *fresh158 = *((*skeletonData).bones).offset(readVarint(input, 1 as c_int) as isize);
            ii += 1;
        }
        (*data_0).target = *((*skeletonData).bones).offset(readVarint(input, 1 as c_int) as isize);
        (*data_0).mix = readFloat(input);
        (*data_0).softness = readFloat(input);
        (*data_0).bendDirection = readSByte(input) as c_int;
        (*data_0).compress = readBoolean(input);
        (*data_0).stretch = readBoolean(input);
        (*data_0).uniform = readBoolean(input);
        let ref mut fresh159 = *((*skeletonData).ikConstraints).offset(i as isize);
        *fresh159 = data_0;
        i += 1;
    }
    (*skeletonData).transformConstraintsCount = readVarint(input, 1 as c_int);
    (*skeletonData).transformConstraints = _spMalloc(
        (::core::mem::size_of::<*mut spTransformConstraintData>() as c_ulong)
            .wrapping_mul((*skeletonData).transformConstraintsCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        9751 as c_int,
    ) as *mut *mut spTransformConstraintData;
    i = 0 as c_int;
    while i < (*skeletonData).transformConstraintsCount {
        let mut name_1: *const c_char = readString(input);
        let mut data_1: *mut spTransformConstraintData = spTransformConstraintData_create(name_1);
        (*data_1).order = readVarint(input, 1 as c_int);
        (*data_1).skinRequired = readBoolean(input);
        _spFree(name_1 as *mut c_void);
        (*data_1).bonesCount = readVarint(input, 1 as c_int);
        let ref mut fresh160 =
            *(&(*data_1).bones as *const *mut *mut spBoneData as *mut *mut *mut spBoneData);
        *fresh160 = _spMalloc(
            (::core::mem::size_of::<*mut spBoneData>() as c_ulong)
                .wrapping_mul((*data_1).bonesCount as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            9761 as c_int,
        ) as *mut *mut spBoneData;
        ii = 0 as c_int;
        while ii < (*data_1).bonesCount {
            let ref mut fresh161 = *((*data_1).bones).offset(ii as isize);
            *fresh161 = *((*skeletonData).bones).offset(readVarint(input, 1 as c_int) as isize);
            ii += 1;
        }
        (*data_1).target = *((*skeletonData).bones).offset(readVarint(input, 1 as c_int) as isize);
        (*data_1).local = readBoolean(input);
        (*data_1).relative = readBoolean(input);
        (*data_1).offsetRotation = readFloat(input);
        (*data_1).offsetX = readFloat(input) * (*self_0).scale;
        (*data_1).offsetY = readFloat(input) * (*self_0).scale;
        (*data_1).offsetScaleX = readFloat(input);
        (*data_1).offsetScaleY = readFloat(input);
        (*data_1).offsetShearY = readFloat(input);
        (*data_1).mixRotate = readFloat(input);
        (*data_1).mixX = readFloat(input);
        (*data_1).mixY = readFloat(input);
        (*data_1).mixScaleX = readFloat(input);
        (*data_1).mixScaleY = readFloat(input);
        (*data_1).mixShearY = readFloat(input);
        let ref mut fresh162 = *((*skeletonData).transformConstraints).offset(i as isize);
        *fresh162 = data_1;
        i += 1;
    }
    (*skeletonData).pathConstraintsCount = readVarint(input, 1 as c_int);
    (*skeletonData).pathConstraints = _spMalloc(
        (::core::mem::size_of::<*mut spPathConstraintData>() as c_ulong)
            .wrapping_mul((*skeletonData).pathConstraintsCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        9784 as c_int,
    ) as *mut *mut spPathConstraintData;
    i = 0 as c_int;
    while i < (*skeletonData).pathConstraintsCount {
        let mut name_2: *const c_char = readString(input);
        let mut data_2: *mut spPathConstraintData = spPathConstraintData_create(name_2);
        (*data_2).order = readVarint(input, 1 as c_int);
        (*data_2).skinRequired = readBoolean(input);
        _spFree(name_2 as *mut c_void);
        (*data_2).bonesCount = readVarint(input, 1 as c_int);
        let ref mut fresh163 =
            *(&(*data_2).bones as *const *mut *mut spBoneData as *mut *mut *mut spBoneData);
        *fresh163 = _spMalloc(
            (::core::mem::size_of::<*mut spBoneData>() as c_ulong)
                .wrapping_mul((*data_2).bonesCount as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            9793 as c_int,
        ) as *mut *mut spBoneData;
        ii = 0 as c_int;
        while ii < (*data_2).bonesCount {
            let ref mut fresh164 = *((*data_2).bones).offset(ii as isize);
            *fresh164 = *((*skeletonData).bones).offset(readVarint(input, 1 as c_int) as isize);
            ii += 1;
        }
        (*data_2).target = *((*skeletonData).slots).offset(readVarint(input, 1 as c_int) as isize);
        (*data_2).positionMode = readVarint(input, 1 as c_int) as spPositionMode;
        (*data_2).spacingMode = readVarint(input, 1 as c_int) as spSpacingMode;
        (*data_2).rotateMode = readVarint(input, 1 as c_int) as spRotateMode;
        (*data_2).offsetRotation = readFloat(input);
        (*data_2).position = readFloat(input);
        if (*data_2).positionMode as c_uint == SP_POSITION_MODE_FIXED as c_int as c_uint {
            (*data_2).position *= (*self_0).scale;
        }
        (*data_2).spacing = readFloat(input);
        if (*data_2).spacingMode as c_uint == SP_SPACING_MODE_LENGTH as c_int as c_uint
            || (*data_2).spacingMode as c_uint == SP_SPACING_MODE_FIXED as c_int as c_uint
        {
            (*data_2).spacing *= (*self_0).scale;
        }
        (*data_2).mixRotate = readFloat(input);
        (*data_2).mixX = readFloat(input);
        (*data_2).mixY = readFloat(input);
        let ref mut fresh165 = *((*skeletonData).pathConstraints).offset(i as isize);
        *fresh165 = data_2;
        i += 1;
    }
    (*skeletonData).defaultSkin =
        spSkeletonBinary_readSkin(self_0, input, -(1 as c_int), skeletonData, nonessential);
    (*skeletonData).skinsCount = readVarint(input, 1 as c_int);
    if !((*skeletonData).defaultSkin).is_null() {
        (*skeletonData).skinsCount += 1;
    }
    (*skeletonData).skins = _spMalloc(
        (::core::mem::size_of::<*mut spSkin>() as c_ulong)
            .wrapping_mul((*skeletonData).skinsCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        9819 as c_int,
    ) as *mut *mut spSkin;
    if !((*skeletonData).defaultSkin).is_null() {
        let ref mut fresh166 = *((*skeletonData).skins).offset(0 as c_int as isize);
        *fresh166 = (*skeletonData).defaultSkin;
    }
    i = if !((*skeletonData).defaultSkin).is_null() {
        1 as c_int
    } else {
        0 as c_int
    };
    while i < (*skeletonData).skinsCount {
        let ref mut fresh167 = *((*skeletonData).skins).offset(i as isize);
        *fresh167 =
            spSkeletonBinary_readSkin(self_0, input, 0 as c_int, skeletonData, nonessential);
        i += 1;
    }
    i = 0 as c_int;
    while i < (*internal).linkedMeshCount {
        let mut linkedMesh: *mut _spLinkedMeshBinary =
            ((*internal).linkedMeshes).offset(i as isize);
        let mut skin: *mut spSkin = if ((*linkedMesh).skin).is_null() {
            (*skeletonData).defaultSkin
        } else {
            spSkeletonData_findSkin(skeletonData, (*linkedMesh).skin)
        };
        let mut parent_0: *mut spAttachment = 0 as *mut spAttachment;
        if skin.is_null() {
            _spFree(input as *mut c_void);
            spSkeletonData_dispose(skeletonData);
            _spSkeletonBinary_setError(
                self_0,
                b"Skin not found: \0" as *const u8 as *const c_char,
                (*linkedMesh).skin,
            );
            return 0 as *mut spSkeletonData;
        }
        parent_0 = spSkin_getAttachment(skin, (*linkedMesh).slotIndex, (*linkedMesh).parent);
        if parent_0.is_null() {
            _spFree(input as *mut c_void);
            spSkeletonData_dispose(skeletonData);
            _spSkeletonBinary_setError(
                self_0,
                b"Parent mesh not found: \0" as *const u8 as *const c_char,
                (*linkedMesh).parent,
            );
            return 0 as *mut spSkeletonData;
        }
        (*(*linkedMesh).mesh).super_0.timelineAttachment = if (*linkedMesh).inheritTimeline != 0 {
            parent_0
        } else {
            &mut (*(*linkedMesh).mesh).super_0.super_0
        };
        spMeshAttachment_setParentMesh((*linkedMesh).mesh, parent_0 as *mut spMeshAttachment);
        if !((*(*linkedMesh).mesh).region).is_null() {
            spMeshAttachment_updateRegion((*linkedMesh).mesh);
        }
        spAttachmentLoader_configureAttachment(
            (*self_0).attachmentLoader,
            &mut (*(*linkedMesh).mesh).super_0.super_0,
        );
        i += 1;
    }
    (*skeletonData).eventsCount = readVarint(input, 1 as c_int);
    (*skeletonData).events = _spMalloc(
        (::core::mem::size_of::<*mut spEventData>() as c_ulong)
            .wrapping_mul((*skeletonData).eventsCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        9856 as c_int,
    ) as *mut *mut spEventData;
    i = 0 as c_int;
    while i < (*skeletonData).eventsCount {
        let mut name_3: *const c_char = readStringRef(input, skeletonData);
        let mut eventData: *mut spEventData = spEventData_create(name_3);
        (*eventData).intValue = readVarint(input, 0 as c_int);
        (*eventData).floatValue = readFloat(input);
        (*eventData).stringValue = readString(input);
        (*eventData).audioPath = readString(input);
        if !((*eventData).audioPath).is_null() {
            (*eventData).volume = readFloat(input);
            (*eventData).balance = readFloat(input);
        }
        let ref mut fresh168 = *((*skeletonData).events).offset(i as isize);
        *fresh168 = eventData;
        i += 1;
    }
    (*skeletonData).animationsCount = readVarint(input, 1 as c_int);
    (*skeletonData).animations = _spMalloc(
        (::core::mem::size_of::<*mut spAnimation>() as c_ulong)
            .wrapping_mul((*skeletonData).animationsCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        9873 as c_int,
    ) as *mut *mut spAnimation;
    i = 0 as c_int;
    while i < (*skeletonData).animationsCount {
        let mut name_4: *const c_char = readString(input);
        let mut animation: *mut spAnimation =
            _spSkeletonBinary_readAnimation(self_0, name_4, input, skeletonData);
        _spFree(name_4 as *mut c_void);
        if animation.is_null() {
            _spFree(input as *mut c_void);
            spSkeletonData_dispose(skeletonData);
            return 0 as *mut spSkeletonData;
        }
        let ref mut fresh169 = *((*skeletonData).animations).offset(i as isize);
        *fresh169 = animation;
        i += 1;
    }
    _spFree(input as *mut c_void);
    return skeletonData;
}
#[no_mangle]
pub unsafe extern "C" fn spPolygon_create(mut capacity: c_int) -> *mut spPolygon {
    let mut self_0: *mut spPolygon = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spPolygon>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        9923 as c_int,
    ) as *mut spPolygon;
    (*self_0).capacity = capacity;
    let ref mut fresh170 = *(&(*self_0).vertices as *const *mut c_float as *mut *mut c_float);
    *fresh170 = _spMalloc(
        (::core::mem::size_of::<c_float>() as c_ulong).wrapping_mul(capacity as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        9925 as c_int,
    ) as *mut c_float;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spPolygon_dispose(mut self_0: *mut spPolygon) {
    _spFree((*self_0).vertices as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spPolygon_containsPoint(
    mut self_0: *mut spPolygon,
    mut x: c_float,
    mut y: c_float,
) -> c_int {
    let mut prevIndex: c_int = (*self_0).count - 2 as c_int;
    let mut inside: c_int = 0 as c_int;
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*self_0).count {
        let mut vertexY: c_float = *((*self_0).vertices).offset((i + 1 as c_int) as isize);
        let mut prevY: c_float = *((*self_0).vertices).offset((prevIndex + 1 as c_int) as isize);
        if vertexY < y && prevY >= y || prevY < y && vertexY >= y {
            let mut vertexX: c_float = *((*self_0).vertices).offset(i as isize);
            if vertexX
                + (y - vertexY) / (prevY - vertexY)
                    * (*((*self_0).vertices).offset(prevIndex as isize) - vertexX)
                < x
            {
                inside = (inside == 0) as c_int;
            }
        }
        prevIndex = i;
        i += 2 as c_int;
    }
    return inside;
}
#[no_mangle]
pub unsafe extern "C" fn spPolygon_intersectsSegment(
    mut self_0: *mut spPolygon,
    mut x1: c_float,
    mut y1: c_float,
    mut x2: c_float,
    mut y2: c_float,
) -> c_int {
    let mut width12: c_float = x1 - x2;
    let mut height12: c_float = y1 - y2;
    let mut det1: c_float = x1 * y2 - y1 * x2;
    let mut x3: c_float = *((*self_0).vertices).offset(((*self_0).count - 2 as c_int) as isize);
    let mut y3: c_float = *((*self_0).vertices).offset(((*self_0).count - 1 as c_int) as isize);
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*self_0).count {
        let mut x4: c_float = *((*self_0).vertices).offset(i as isize);
        let mut y4: c_float = *((*self_0).vertices).offset((i + 1 as c_int) as isize);
        let mut det2: c_float = x3 * y4 - y3 * x4;
        let mut width34: c_float = x3 - x4;
        let mut height34: c_float = y3 - y4;
        let mut det3: c_float = width12 * height34 - height12 * width34;
        let mut x: c_float = (det1 * width34 - width12 * det2) / det3;
        if (x >= x3 && x <= x4 || x >= x4 && x <= x3) && (x >= x1 && x <= x2 || x >= x2 && x <= x1)
        {
            let mut y: c_float = (det1 * height34 - height12 * det2) / det3;
            if (y >= y3 && y <= y4 || y >= y4 && y <= y3)
                && (y >= y1 && y <= y2 || y >= y2 && y <= y1)
            {
                return 1 as c_int;
            }
        }
        x3 = x4;
        y3 = y4;
        i += 2 as c_int;
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonBounds_create() -> *mut spSkeletonBounds {
    return &mut (*((_spCalloc
        as unsafe extern "C" fn(size_t, size_t, *const c_char, c_int) -> *mut c_void)(
        1 as c_int as size_t,
        ::core::mem::size_of::<_spSkeletonBounds>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        9981 as c_int,
    ) as *mut _spSkeletonBounds))
        .super_0;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonBounds_dispose(mut self_0: *mut spSkeletonBounds) {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*(self_0 as *mut _spSkeletonBounds)).capacity {
        if !(*((*self_0).polygons).offset(i as isize)).is_null() {
            spPolygon_dispose(*((*self_0).polygons).offset(i as isize));
        }
        i += 1;
    }
    _spFree((*self_0).polygons as *mut c_void);
    _spFree((*self_0).boundingBoxes as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonBounds_update(
    mut self_0: *mut spSkeletonBounds,
    mut skeleton: *mut spSkeleton,
    mut updateAabb: c_int,
) {
    let mut i: c_int = 0;
    let mut internal: *mut _spSkeletonBounds = self_0 as *mut _spSkeletonBounds;
    if (*internal).capacity < (*skeleton).slotsCount {
        let mut newPolygons: *mut *mut spPolygon = 0 as *mut *mut spPolygon;
        _spFree((*self_0).boundingBoxes as *mut c_void);
        (*self_0).boundingBoxes = _spMalloc(
            (::core::mem::size_of::<*mut spBoundingBoxAttachment>() as c_ulong)
                .wrapping_mul((*skeleton).slotsCount as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            10001 as c_int,
        ) as *mut *mut spBoundingBoxAttachment;
        newPolygons = _spCalloc(
            (*skeleton).slotsCount as size_t,
            ::core::mem::size_of::<*mut spPolygon>() as c_ulong,
            b"spine.c\0" as *const u8 as *const c_char,
            10003 as c_int,
        ) as *mut *mut spPolygon;
        spine_memcpy(
            newPolygons as *mut c_void,
            (*self_0).polygons as *const c_void,
            (::core::mem::size_of::<*mut spPolygon>() as c_ulong)
                .wrapping_mul((*internal).capacity as c_ulong),
        );
        _spFree((*self_0).polygons as *mut c_void);
        (*self_0).polygons = newPolygons;
        (*internal).capacity = (*skeleton).slotsCount;
    }
    (*self_0).minX = 0x7fffffff as c_int as c_float;
    (*self_0).minY = 0x7fffffff as c_int as c_float;
    (*self_0).maxX = (-(0x7fffffff as c_int) - 1 as c_int) as c_float;
    (*self_0).maxY = (-(0x7fffffff as c_int) - 1 as c_int) as c_float;
    (*self_0).count = 0 as c_int;
    i = 0 as c_int;
    while i < (*skeleton).slotsCount {
        let mut polygon: *mut spPolygon = 0 as *mut spPolygon;
        let mut boundingBox: *mut spBoundingBoxAttachment = 0 as *mut spBoundingBoxAttachment;
        let mut attachment: *mut spAttachment = 0 as *mut spAttachment;
        let mut slot: *mut spSlot = *((*skeleton).slots).offset(i as isize);
        if !((*(*slot).bone).active == 0) {
            attachment = (*slot).attachment;
            if !(attachment.is_null()
                || (*attachment).type_0 as c_uint != SP_ATTACHMENT_BOUNDING_BOX as c_int as c_uint)
            {
                boundingBox = attachment as *mut spBoundingBoxAttachment;
                let ref mut fresh171 = *((*self_0).boundingBoxes).offset((*self_0).count as isize);
                *fresh171 = boundingBox;
                polygon = *((*self_0).polygons).offset((*self_0).count as isize);
                if polygon.is_null()
                    || (*polygon).capacity < (*boundingBox).super_0.worldVerticesLength
                {
                    if !polygon.is_null() {
                        spPolygon_dispose(polygon);
                    }
                    polygon = spPolygon_create((*boundingBox).super_0.worldVerticesLength);
                    let ref mut fresh172 = *((*self_0).polygons).offset((*self_0).count as isize);
                    *fresh172 = polygon;
                }
                (*polygon).count = (*boundingBox).super_0.worldVerticesLength;
                spVertexAttachment_computeWorldVertices(
                    &mut (*boundingBox).super_0,
                    slot,
                    0 as c_int,
                    (*polygon).count,
                    (*polygon).vertices,
                    0 as c_int,
                    2 as c_int,
                );
                if updateAabb != 0 {
                    let mut ii: c_int = 0 as c_int;
                    while ii < (*polygon).count {
                        let mut x: c_float = *((*polygon).vertices).offset(ii as isize);
                        let mut y: c_float =
                            *((*polygon).vertices).offset((ii + 1 as c_int) as isize);
                        if x < (*self_0).minX {
                            (*self_0).minX = x;
                        }
                        if y < (*self_0).minY {
                            (*self_0).minY = y;
                        }
                        if x > (*self_0).maxX {
                            (*self_0).maxX = x;
                        }
                        if y > (*self_0).maxY {
                            (*self_0).maxY = y;
                        }
                        ii += 2 as c_int;
                    }
                }
                (*self_0).count += 1;
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonBounds_aabbContainsPoint(
    mut self_0: *mut spSkeletonBounds,
    mut x: c_float,
    mut y: c_float,
) -> c_int {
    return (x >= (*self_0).minX
        && x <= (*self_0).maxX
        && y >= (*self_0).minY
        && y <= (*self_0).maxY) as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonBounds_aabbIntersectsSegment(
    mut self_0: *mut spSkeletonBounds,
    mut x1: c_float,
    mut y1: c_float,
    mut x2: c_float,
    mut y2: c_float,
) -> c_int {
    let mut m: c_float = 0.;
    let mut x: c_float = 0.;
    let mut y: c_float = 0.;
    if x1 <= (*self_0).minX && x2 <= (*self_0).minX
        || y1 <= (*self_0).minY && y2 <= (*self_0).minY
        || x1 >= (*self_0).maxX && x2 >= (*self_0).maxX
        || y1 >= (*self_0).maxY && y2 >= (*self_0).maxY
    {
        return 0 as c_int;
    }
    m = (y2 - y1) / (x2 - x1);
    y = m * ((*self_0).minX - x1) + y1;
    if y > (*self_0).minY && y < (*self_0).maxY {
        return 1 as c_int;
    }
    y = m * ((*self_0).maxX - x1) + y1;
    if y > (*self_0).minY && y < (*self_0).maxY {
        return 1 as c_int;
    }
    x = ((*self_0).minY - y1) / m + x1;
    if x > (*self_0).minX && x < (*self_0).maxX {
        return 1 as c_int;
    }
    x = ((*self_0).maxY - y1) / m + x1;
    if x > (*self_0).minX && x < (*self_0).maxX {
        return 1 as c_int;
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonBounds_aabbIntersectsSkeleton(
    mut self_0: *mut spSkeletonBounds,
    mut bounds: *mut spSkeletonBounds,
) -> c_int {
    return ((*self_0).minX < (*bounds).maxX
        && (*self_0).maxX > (*bounds).minX
        && (*self_0).minY < (*bounds).maxY
        && (*self_0).maxY > (*bounds).minY) as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonBounds_containsPoint(
    mut self_0: *mut spSkeletonBounds,
    mut x: c_float,
    mut y: c_float,
) -> *mut spBoundingBoxAttachment {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*self_0).count {
        if spPolygon_containsPoint(*((*self_0).polygons).offset(i as isize), x, y) != 0 {
            return *((*self_0).boundingBoxes).offset(i as isize);
        }
        i += 1;
    }
    return 0 as *mut spBoundingBoxAttachment;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonBounds_intersectsSegment(
    mut self_0: *mut spSkeletonBounds,
    mut x1: c_float,
    mut y1: c_float,
    mut x2: c_float,
    mut y2: c_float,
) -> *mut spBoundingBoxAttachment {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*self_0).count {
        if spPolygon_intersectsSegment(*((*self_0).polygons).offset(i as isize), x1, y1, x2, y2)
            != 0
        {
            return *((*self_0).boundingBoxes).offset(i as isize);
        }
        i += 1;
    }
    return 0 as *mut spBoundingBoxAttachment;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonBounds_getPolygon(
    mut self_0: *mut spSkeletonBounds,
    mut boundingBox: *mut spBoundingBoxAttachment,
) -> *mut spPolygon {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*self_0).count {
        if *((*self_0).boundingBoxes).offset(i as isize) == boundingBox {
            return *((*self_0).polygons).offset(i as isize);
        }
        i += 1;
    }
    return 0 as *mut spPolygon;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonClipping_create() -> *mut spSkeletonClipping {
    let mut clipping: *mut spSkeletonClipping = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spSkeletonClipping>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        10132 as c_int,
    ) as *mut spSkeletonClipping;
    (*clipping).triangulator = spTriangulator_create();
    (*clipping).clippingPolygon = spFloatArray_create(128 as c_int);
    (*clipping).clipOutput = spFloatArray_create(128 as c_int);
    (*clipping).clippedVertices = spFloatArray_create(128 as c_int);
    (*clipping).clippedUVs = spFloatArray_create(128 as c_int);
    (*clipping).clippedTriangles = spUnsignedShortArray_create(128 as c_int);
    (*clipping).scratch = spFloatArray_create(128 as c_int);
    return clipping;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonClipping_dispose(mut self_0: *mut spSkeletonClipping) {
    spTriangulator_dispose((*self_0).triangulator);
    spFloatArray_dispose((*self_0).clippingPolygon);
    spFloatArray_dispose((*self_0).clipOutput);
    spFloatArray_dispose((*self_0).clippedVertices);
    spFloatArray_dispose((*self_0).clippedUVs);
    spUnsignedShortArray_dispose((*self_0).clippedTriangles);
    spFloatArray_dispose((*self_0).scratch);
    _spFree(self_0 as *mut c_void);
}
unsafe extern "C" fn _makeClockwise(mut polygon: *mut spFloatArray) {
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    let mut lastX: c_int = 0;
    let mut vertices: *mut c_float = (*polygon).items;
    let mut verticeslength: c_int = (*polygon).size;
    let mut area: c_float = *vertices.offset((verticeslength - 2 as c_int) as isize)
        * *vertices.offset(1 as c_int as isize)
        - *vertices.offset(0 as c_int as isize)
            * *vertices.offset((verticeslength - 1 as c_int) as isize);
    let mut p1x: c_float = 0.;
    let mut p1y: c_float = 0.;
    let mut p2x: c_float = 0.;
    let mut p2y: c_float = 0.;
    i = 0 as c_int;
    n = verticeslength - 3 as c_int;
    while i < n {
        p1x = *vertices.offset(i as isize);
        p1y = *vertices.offset((i + 1 as c_int) as isize);
        p2x = *vertices.offset((i + 2 as c_int) as isize);
        p2y = *vertices.offset((i + 3 as c_int) as isize);
        area += p1x * p2y - p2x * p1y;
        i += 2 as c_int;
    }
    if area < 0 as c_int as c_float {
        return;
    }
    i = 0 as c_int;
    lastX = verticeslength - 2 as c_int;
    n = verticeslength >> 1 as c_int;
    while i < n {
        let mut x: c_float = *vertices.offset(i as isize);
        let mut y: c_float = *vertices.offset((i + 1 as c_int) as isize);
        let mut other: c_int = lastX - i;
        *vertices.offset(i as isize) = *vertices.offset(other as isize);
        *vertices.offset((i + 1 as c_int) as isize) =
            *vertices.offset((other + 1 as c_int) as isize);
        *vertices.offset(other as isize) = x;
        *vertices.offset((other + 1 as c_int) as isize) = y;
        i += 2 as c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonClipping_clipStart(
    mut self_0: *mut spSkeletonClipping,
    mut slot: *mut spSlot,
    mut clip: *mut spClippingAttachment,
) -> c_int {
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    let mut vertices: *mut c_float = 0 as *mut c_float;
    if !((*self_0).clipAttachment).is_null() {
        return 0 as c_int;
    }
    (*self_0).clipAttachment = clip;
    n = (*clip).super_0.worldVerticesLength;
    vertices = (*spFloatArray_setSize((*self_0).clippingPolygon, n)).items;
    spVertexAttachment_computeWorldVertices(
        &mut (*clip).super_0,
        slot,
        0 as c_int,
        n,
        vertices,
        0 as c_int,
        2 as c_int,
    );
    _makeClockwise((*self_0).clippingPolygon);
    (*self_0).clippingPolygons = spTriangulator_decompose(
        (*self_0).triangulator,
        (*self_0).clippingPolygon,
        spTriangulator_triangulate((*self_0).triangulator, (*self_0).clippingPolygon),
    );
    i = 0 as c_int;
    n = (*(*self_0).clippingPolygons).size;
    while i < n {
        let mut polygon: *mut spFloatArray =
            *((*(*self_0).clippingPolygons).items).offset(i as isize);
        _makeClockwise(polygon);
        spFloatArray_add(polygon, *((*polygon).items).offset(0 as c_int as isize));
        spFloatArray_add(polygon, *((*polygon).items).offset(1 as c_int as isize));
        i += 1;
    }
    return (*(*self_0).clippingPolygons).size;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonClipping_clipEnd(
    mut self_0: *mut spSkeletonClipping,
    mut slot: *mut spSlot,
) {
    if !((*self_0).clipAttachment).is_null() && (*(*self_0).clipAttachment).endSlot == (*slot).data
    {
        spSkeletonClipping_clipEnd2(self_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonClipping_clipEnd2(mut self_0: *mut spSkeletonClipping) {
    if ((*self_0).clipAttachment).is_null() {
        return;
    }
    (*self_0).clipAttachment = 0 as *mut spClippingAttachment;
    (*self_0).clippingPolygons = 0 as *mut spArrayFloatArray;
    spFloatArray_clear((*self_0).clippedVertices);
    spFloatArray_clear((*self_0).clippedUVs);
    spUnsignedShortArray_clear((*self_0).clippedTriangles);
    spFloatArray_clear((*self_0).clippingPolygon);
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonClipping_isClipping(
    mut self_0: *mut spSkeletonClipping,
) -> c_int {
    return ((*self_0).clipAttachment != 0 as *mut spClippingAttachment) as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _clip(
    mut self_0: *mut spSkeletonClipping,
    mut x1: c_float,
    mut y1: c_float,
    mut x2: c_float,
    mut y2: c_float,
    mut x3: c_float,
    mut y3: c_float,
    mut clippingArea: *mut spFloatArray,
    mut output: *mut spFloatArray,
) -> c_int {
    let mut i: c_int = 0;
    let mut originalOutput: *mut spFloatArray = output;
    let mut clipped: c_int = 0 as c_int;
    let mut clippingVertices: *mut c_float = 0 as *mut c_float;
    let mut clippingVerticesLast: c_int = 0;
    let mut input: *mut spFloatArray = 0 as *mut spFloatArray;
    if (*clippingArea).size % 4 as c_int >= 2 as c_int {
        input = output;
        output = (*self_0).scratch;
    } else {
        input = (*self_0).scratch;
    }
    spFloatArray_clear(input);
    spFloatArray_add(input, x1);
    spFloatArray_add(input, y1);
    spFloatArray_add(input, x2);
    spFloatArray_add(input, y2);
    spFloatArray_add(input, x3);
    spFloatArray_add(input, y3);
    spFloatArray_add(input, x1);
    spFloatArray_add(input, y1);
    spFloatArray_clear(output);
    clippingVertices = (*clippingArea).items;
    clippingVerticesLast = (*clippingArea).size - 4 as c_int;
    i = 0 as c_int;
    loop {
        let mut ii: c_int = 0;
        let mut temp: *mut spFloatArray = 0 as *mut spFloatArray;
        let mut edgeX: c_float = *clippingVertices.offset(i as isize);
        let mut edgeY: c_float = *clippingVertices.offset((i + 1 as c_int) as isize);
        let mut edgeX2: c_float = *clippingVertices.offset((i + 2 as c_int) as isize);
        let mut edgeY2: c_float = *clippingVertices.offset((i + 3 as c_int) as isize);
        let mut deltaX: c_float = edgeX - edgeX2;
        let mut deltaY: c_float = edgeY - edgeY2;
        let mut inputVertices: *mut c_float = (*input).items;
        let mut inputVerticesLength: c_int = (*input).size - 2 as c_int;
        let mut outputStart: c_int = (*output).size;
        let mut current_block_42: u64;
        ii = 0 as c_int;
        while ii < inputVerticesLength {
            let mut inputX: c_float = *inputVertices.offset(ii as isize);
            let mut inputY: c_float = *inputVertices.offset((ii + 1 as c_int) as isize);
            let mut inputX2: c_float = *inputVertices.offset((ii + 2 as c_int) as isize);
            let mut inputY2: c_float = *inputVertices.offset((ii + 3 as c_int) as isize);
            let mut side2: c_int = (deltaX * (inputY2 - edgeY2) - deltaY * (inputX2 - edgeX2)
                > 0 as c_int as c_float) as c_int;
            if deltaX * (inputY - edgeY2) - deltaY * (inputX - edgeX2) > 0 as c_int as c_float {
                let mut c0: c_float = 0.;
                let mut c2: c_float = 0.;
                let mut s: c_float = 0.;
                let mut ua: c_float = 0.;
                if side2 != 0 {
                    spFloatArray_add(output, inputX2);
                    spFloatArray_add(output, inputY2);
                    current_block_42 = 15904375183555213903;
                } else {
                    c0 = inputY2 - inputY;
                    c2 = inputX2 - inputX;
                    s = c0 * (edgeX2 - edgeX) - c2 * (edgeY2 - edgeY);
                    if (if s < 0 as c_int as c_float { -s } else { s }) > 0.000001f32 {
                        ua = (c2 * (edgeY - inputY) - c0 * (edgeX - inputX)) / s;
                        spFloatArray_add(output, edgeX + (edgeX2 - edgeX) * ua);
                        spFloatArray_add(output, edgeY + (edgeY2 - edgeY) * ua);
                    } else {
                        spFloatArray_add(output, edgeX);
                        spFloatArray_add(output, edgeY);
                    }
                    current_block_42 = 5330834795799507926;
                }
            } else {
                if side2 != 0 {
                    let mut c0_0: c_float = inputY2 - inputY;
                    let mut c2_0: c_float = inputX2 - inputX;
                    let mut s_0: c_float = c0_0 * (edgeX2 - edgeX) - c2_0 * (edgeY2 - edgeY);
                    if (if s_0 < 0 as c_int as c_float {
                        -s_0
                    } else {
                        s_0
                    }) > 0.000001f32
                    {
                        let mut ua_0: c_float =
                            (c2_0 * (edgeY - inputY) - c0_0 * (edgeX - inputX)) / s_0;
                        spFloatArray_add(output, edgeX + (edgeX2 - edgeX) * ua_0);
                        spFloatArray_add(output, edgeY + (edgeY2 - edgeY) * ua_0);
                    } else {
                        spFloatArray_add(output, edgeX);
                        spFloatArray_add(output, edgeY);
                    }
                    spFloatArray_add(output, inputX2);
                    spFloatArray_add(output, inputY2);
                }
                current_block_42 = 5330834795799507926;
            }
            match current_block_42 {
                5330834795799507926 => {
                    clipped = 1 as c_int;
                }
                _ => {}
            }
            ii += 2 as c_int;
        }
        if outputStart == (*output).size {
            spFloatArray_clear(originalOutput);
            return 1 as c_int;
        }
        spFloatArray_add(output, *((*output).items).offset(0 as c_int as isize));
        spFloatArray_add(output, *((*output).items).offset(1 as c_int as isize));
        if i == clippingVerticesLast {
            break;
        }
        temp = output;
        output = input;
        spFloatArray_clear(output);
        input = temp;
        i += 2 as c_int;
    }
    if originalOutput != output {
        spFloatArray_clear(originalOutput);
        spFloatArray_addAllValues(
            originalOutput,
            (*output).items,
            0 as c_int,
            (*output).size - 2 as c_int,
        );
    } else {
        spFloatArray_setSize(originalOutput, (*originalOutput).size - 2 as c_int);
    }
    return clipped;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonClipping_clipTriangles(
    mut self_0: *mut spSkeletonClipping,
    mut vertices: *mut c_float,
    mut _verticesLength: c_int,
    mut triangles: *mut c_ushort,
    mut trianglesLength: c_int,
    mut uvs: *mut c_float,
    mut stride: c_int,
) {
    let mut i: c_int = 0;
    let mut clipOutput: *mut spFloatArray = (*self_0).clipOutput;
    let mut clippedVertices: *mut spFloatArray = (*self_0).clippedVertices;
    let mut clippedUVs: *mut spFloatArray = (*self_0).clippedUVs;
    let mut clippedTriangles: *mut spUnsignedShortArray = (*self_0).clippedTriangles;
    let mut polygons: *mut *mut spFloatArray = (*(*self_0).clippingPolygons).items;
    let mut polygonsCount: c_int = (*(*self_0).clippingPolygons).size;
    let mut index: c_short = 0 as c_int as c_short;
    spFloatArray_clear(clippedVertices);
    spFloatArray_clear(clippedUVs);
    spUnsignedShortArray_clear(clippedTriangles);
    i = 0 as c_int;
    's_30: while i < trianglesLength {
        let mut p: c_int = 0;
        let mut vertexOffset: c_int = *triangles.offset(i as isize) as c_int * stride;
        let mut x2: c_float = 0.;
        let mut y2: c_float = 0.;
        let mut u2: c_float = 0.;
        let mut v2: c_float = 0.;
        let mut x3: c_float = 0.;
        let mut y3: c_float = 0.;
        let mut u3: c_float = 0.;
        let mut v3: c_float = 0.;
        let mut x1: c_float = *vertices.offset(vertexOffset as isize);
        let mut y1: c_float = *vertices.offset((vertexOffset + 1 as c_int) as isize);
        let mut u1: c_float = *uvs.offset(vertexOffset as isize);
        let mut v1: c_float = *uvs.offset((vertexOffset + 1 as c_int) as isize);
        vertexOffset = *triangles.offset((i + 1 as c_int) as isize) as c_int * stride;
        x2 = *vertices.offset(vertexOffset as isize);
        y2 = *vertices.offset((vertexOffset + 1 as c_int) as isize);
        u2 = *uvs.offset(vertexOffset as isize);
        v2 = *uvs.offset((vertexOffset + 1 as c_int) as isize);
        vertexOffset = *triangles.offset((i + 2 as c_int) as isize) as c_int * stride;
        x3 = *vertices.offset(vertexOffset as isize);
        y3 = *vertices.offset((vertexOffset + 1 as c_int) as isize);
        u3 = *uvs.offset(vertexOffset as isize);
        v3 = *uvs.offset((vertexOffset + 1 as c_int) as isize);
        p = 0 as c_int;
        while p < polygonsCount {
            let mut s: c_int = (*clippedVertices).size;
            if _clip(
                self_0,
                x1,
                y1,
                x2,
                y2,
                x3,
                y3,
                *polygons.offset(p as isize),
                clipOutput,
            ) != 0
            {
                let mut ii: c_int = 0;
                let mut d0: c_float = 0.;
                let mut d1: c_float = 0.;
                let mut d2: c_float = 0.;
                let mut d4: c_float = 0.;
                let mut d: c_float = 0.;
                let mut clippedTrianglesItems: *mut c_ushort = 0 as *mut c_ushort;
                let mut clipOutputCount: c_int = 0;
                let mut clipOutputItems: *mut c_float = 0 as *mut c_float;
                let mut clippedVerticesItems: *mut c_float = 0 as *mut c_float;
                let mut clippedUVsItems: *mut c_float = 0 as *mut c_float;
                let mut clipOutputLength: c_int = (*clipOutput).size;
                if !(clipOutputLength == 0 as c_int) {
                    d0 = y2 - y3;
                    d1 = x3 - x2;
                    d2 = x1 - x3;
                    d4 = y3 - y1;
                    d = 1 as c_int as c_float / (d0 * d2 + d1 * (y1 - y3));
                    clipOutputCount = clipOutputLength >> 1 as c_int;
                    clipOutputItems = (*clipOutput).items;
                    clippedVerticesItems = (*spFloatArray_setSize(
                        clippedVertices,
                        s + (clipOutputCount << 1 as c_int),
                    ))
                    .items;
                    clippedUVsItems =
                        (*spFloatArray_setSize(clippedUVs, s + (clipOutputCount << 1 as c_int)))
                            .items;
                    ii = 0 as c_int;
                    while ii < clipOutputLength {
                        let mut c0: c_float = 0.;
                        let mut c1: c_float = 0.;
                        let mut a: c_float = 0.;
                        let mut b: c_float = 0.;
                        let mut c: c_float = 0.;
                        let mut x: c_float = *clipOutputItems.offset(ii as isize);
                        let mut y: c_float = *clipOutputItems.offset((ii + 1 as c_int) as isize);
                        *clippedVerticesItems.offset(s as isize) = x;
                        *clippedVerticesItems.offset((s + 1 as c_int) as isize) = y;
                        c0 = x - x3;
                        c1 = y - y3;
                        a = (d0 * c0 + d1 * c1) * d;
                        b = (d4 * c0 + d2 * c1) * d;
                        c = 1 as c_int as c_float - a - b;
                        *clippedUVsItems.offset(s as isize) = u1 * a + u2 * b + u3 * c;
                        *clippedUVsItems.offset((s + 1 as c_int) as isize) =
                            v1 * a + v2 * b + v3 * c;
                        s += 2 as c_int;
                        ii += 2 as c_int;
                    }
                    s = (*clippedTriangles).size;
                    clippedTrianglesItems = (*spUnsignedShortArray_setSize(
                        clippedTriangles,
                        s + 3 as c_int * (clipOutputCount - 2 as c_int),
                    ))
                    .items;
                    clipOutputCount -= 1;
                    ii = 1 as c_int;
                    while ii < clipOutputCount {
                        *clippedTrianglesItems.offset(s as isize) = index as c_ushort;
                        *clippedTrianglesItems.offset((s + 1 as c_int) as isize) =
                            (index as c_int + ii) as c_ushort;
                        *clippedTrianglesItems.offset((s + 2 as c_int) as isize) =
                            (index as c_int + ii + 1 as c_int) as c_ushort;
                        s += 3 as c_int;
                        ii += 1;
                    }
                    index = (index as c_int + (clipOutputCount + 1 as c_int)) as c_short;
                }
                p += 1;
            } else {
                let mut clippedTrianglesItems_0: *mut c_ushort = 0 as *mut c_ushort;
                let mut clippedVerticesItems_0: *mut c_float =
                    (*spFloatArray_setSize(clippedVertices, s + ((3 as c_int) << 1 as c_int)))
                        .items;
                let mut clippedUVsItems_0: *mut c_float =
                    (*spFloatArray_setSize(clippedUVs, s + ((3 as c_int) << 1 as c_int))).items;
                *clippedVerticesItems_0.offset(s as isize) = x1;
                *clippedVerticesItems_0.offset((s + 1 as c_int) as isize) = y1;
                *clippedVerticesItems_0.offset((s + 2 as c_int) as isize) = x2;
                *clippedVerticesItems_0.offset((s + 3 as c_int) as isize) = y2;
                *clippedVerticesItems_0.offset((s + 4 as c_int) as isize) = x3;
                *clippedVerticesItems_0.offset((s + 5 as c_int) as isize) = y3;
                *clippedUVsItems_0.offset(s as isize) = u1;
                *clippedUVsItems_0.offset((s + 1 as c_int) as isize) = v1;
                *clippedUVsItems_0.offset((s + 2 as c_int) as isize) = u2;
                *clippedUVsItems_0.offset((s + 3 as c_int) as isize) = v2;
                *clippedUVsItems_0.offset((s + 4 as c_int) as isize) = u3;
                *clippedUVsItems_0.offset((s + 5 as c_int) as isize) = v3;
                s = (*clippedTriangles).size;
                clippedTrianglesItems_0 =
                    (*spUnsignedShortArray_setSize(clippedTriangles, s + 3 as c_int)).items;
                *clippedTrianglesItems_0.offset(s as isize) = index as c_ushort;
                *clippedTrianglesItems_0.offset((s + 1 as c_int) as isize) =
                    (index as c_int + 1 as c_int) as c_ushort;
                *clippedTrianglesItems_0.offset((s + 2 as c_int) as isize) =
                    (index as c_int + 2 as c_int) as c_ushort;
                index = (index as c_int + 3 as c_int) as c_short;
                i += 3 as c_int;
                continue 's_30;
            }
        }
        i += 3 as c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonData_create() -> *mut spSkeletonData {
    return _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spSkeletonData>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        10475 as c_int,
    ) as *mut spSkeletonData;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonData_dispose(mut self_0: *mut spSkeletonData) {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*self_0).stringsCount {
        _spFree(*((*self_0).strings).offset(i as isize) as *mut c_void);
        i += 1;
    }
    _spFree((*self_0).strings as *mut c_void);
    i = 0 as c_int;
    while i < (*self_0).bonesCount {
        spBoneData_dispose(*((*self_0).bones).offset(i as isize));
        i += 1;
    }
    _spFree((*self_0).bones as *mut c_void);
    i = 0 as c_int;
    while i < (*self_0).slotsCount {
        spSlotData_dispose(*((*self_0).slots).offset(i as isize));
        i += 1;
    }
    _spFree((*self_0).slots as *mut c_void);
    i = 0 as c_int;
    while i < (*self_0).skinsCount {
        spSkin_dispose(*((*self_0).skins).offset(i as isize));
        i += 1;
    }
    _spFree((*self_0).skins as *mut c_void);
    i = 0 as c_int;
    while i < (*self_0).eventsCount {
        spEventData_dispose(*((*self_0).events).offset(i as isize));
        i += 1;
    }
    _spFree((*self_0).events as *mut c_void);
    i = 0 as c_int;
    while i < (*self_0).animationsCount {
        spAnimation_dispose(*((*self_0).animations).offset(i as isize));
        i += 1;
    }
    _spFree((*self_0).animations as *mut c_void);
    i = 0 as c_int;
    while i < (*self_0).ikConstraintsCount {
        spIkConstraintData_dispose(*((*self_0).ikConstraints).offset(i as isize));
        i += 1;
    }
    _spFree((*self_0).ikConstraints as *mut c_void);
    i = 0 as c_int;
    while i < (*self_0).transformConstraintsCount {
        spTransformConstraintData_dispose(*((*self_0).transformConstraints).offset(i as isize));
        i += 1;
    }
    _spFree((*self_0).transformConstraints as *mut c_void);
    i = 0 as c_int;
    while i < (*self_0).pathConstraintsCount {
        spPathConstraintData_dispose(*((*self_0).pathConstraints).offset(i as isize));
        i += 1;
    }
    _spFree((*self_0).pathConstraints as *mut c_void);
    _spFree((*self_0).hash as *mut c_void);
    _spFree((*self_0).version as *mut c_void);
    _spFree((*self_0).imagesPath as *mut c_void);
    _spFree((*self_0).audioPath as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonData_findBone(
    mut self_0: *const spSkeletonData,
    mut boneName: *const c_char,
) -> *mut spBoneData {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*self_0).bonesCount {
        if spine_strcmp((**((*self_0).bones).offset(i as isize)).name, boneName) == 0 as c_int {
            return *((*self_0).bones).offset(i as isize);
        }
        i += 1;
    }
    return 0 as *mut spBoneData;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonData_findSlot(
    mut self_0: *const spSkeletonData,
    mut slotName: *const c_char,
) -> *mut spSlotData {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*self_0).slotsCount {
        if spine_strcmp((**((*self_0).slots).offset(i as isize)).name, slotName) == 0 as c_int {
            return *((*self_0).slots).offset(i as isize);
        }
        i += 1;
    }
    return 0 as *mut spSlotData;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonData_findSkin(
    mut self_0: *const spSkeletonData,
    mut skinName: *const c_char,
) -> *mut spSkin {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*self_0).skinsCount {
        if spine_strcmp((**((*self_0).skins).offset(i as isize)).name, skinName) == 0 as c_int {
            return *((*self_0).skins).offset(i as isize);
        }
        i += 1;
    }
    return 0 as *mut spSkin;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonData_findEvent(
    mut self_0: *const spSkeletonData,
    mut eventName: *const c_char,
) -> *mut spEventData {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*self_0).eventsCount {
        if spine_strcmp((**((*self_0).events).offset(i as isize)).name, eventName) == 0 as c_int {
            return *((*self_0).events).offset(i as isize);
        }
        i += 1;
    }
    return 0 as *mut spEventData;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonData_findAnimation(
    mut self_0: *const spSkeletonData,
    mut animationName: *const c_char,
) -> *mut spAnimation {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*self_0).animationsCount {
        if spine_strcmp(
            (**((*self_0).animations).offset(i as isize)).name,
            animationName,
        ) == 0 as c_int
        {
            return *((*self_0).animations).offset(i as isize);
        }
        i += 1;
    }
    return 0 as *mut spAnimation;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonData_findIkConstraint(
    mut self_0: *const spSkeletonData,
    mut constraintName: *const c_char,
) -> *mut spIkConstraintData {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*self_0).ikConstraintsCount {
        if spine_strcmp(
            (**((*self_0).ikConstraints).offset(i as isize)).name,
            constraintName,
        ) == 0 as c_int
        {
            return *((*self_0).ikConstraints).offset(i as isize);
        }
        i += 1;
    }
    return 0 as *mut spIkConstraintData;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonData_findTransformConstraint(
    mut self_0: *const spSkeletonData,
    mut constraintName: *const c_char,
) -> *mut spTransformConstraintData {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*self_0).transformConstraintsCount {
        if spine_strcmp(
            (**((*self_0).transformConstraints).offset(i as isize)).name,
            constraintName,
        ) == 0 as c_int
        {
            return *((*self_0).transformConstraints).offset(i as isize);
        }
        i += 1;
    }
    return 0 as *mut spTransformConstraintData;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonData_findPathConstraint(
    mut self_0: *const spSkeletonData,
    mut constraintName: *const c_char,
) -> *mut spPathConstraintData {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*self_0).pathConstraintsCount {
        if spine_strcmp(
            (**((*self_0).pathConstraints).offset(i as isize)).name,
            constraintName,
        ) == 0 as c_int
        {
            return *((*self_0).pathConstraints).offset(i as isize);
        }
        i += 1;
    }
    return 0 as *mut spPathConstraintData;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonJson_createWithLoader(
    mut attachmentLoader: *mut spAttachmentLoader,
) -> *mut spSkeletonJson {
    let mut self_0: *mut spSkeletonJson = &mut (*((_spCalloc
        as unsafe extern "C" fn(size_t, size_t, *const c_char, c_int) -> *mut c_void)(
        1 as c_int as size_t,
        ::core::mem::size_of::<_spSkeletonJson>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        10636 as c_int,
    ) as *mut _spSkeletonJson))
        .super_0;
    (*self_0).scale = 1 as c_int as c_float;
    (*self_0).attachmentLoader = attachmentLoader;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonJson_create(mut atlas: *mut spAtlas) -> *mut spSkeletonJson {
    let mut attachmentLoader: *mut spAtlasAttachmentLoader = spAtlasAttachmentLoader_create(atlas);
    let mut self_0: *mut spSkeletonJson =
        spSkeletonJson_createWithLoader(&mut (*attachmentLoader).super_0);
    (*(self_0 as *mut _spSkeletonJson)).ownsLoader = 1 as c_int;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonJson_dispose(mut self_0: *mut spSkeletonJson) {
    let mut internal: *mut _spSkeletonJson = self_0 as *mut _spSkeletonJson;
    if (*internal).ownsLoader != 0 {
        spAttachmentLoader_dispose((*self_0).attachmentLoader);
    }
    _spFree((*internal).linkedMeshes as *mut c_void);
    _spFree((*self_0).error as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _spSkeletonJson_setError(
    mut self_0: *mut spSkeletonJson,
    mut root: *mut Json,
    mut value1: *const c_char,
    mut value2: *const c_char,
) {
    let mut message: [c_char; 256] = [0; 256];
    let mut length: c_int = 0;
    _spFree((*self_0).error as *mut c_void);
    spine_strcpy(message.as_mut_ptr(), value1);
    length = spine_strlen(value1) as c_int;
    if !value2.is_null() {
        spine_strncat(
            message.as_mut_ptr().offset(length as isize),
            value2,
            (255 as c_int - length) as size_t,
        );
    }
    let ref mut fresh173 = *(&(*self_0).error as *const *const c_char as *mut *mut c_char);
    *fresh173 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul((spine_strlen(message.as_mut_ptr())).wrapping_add(1 as c_int as c_ulong)),
        b"spine.c\0" as *const u8 as *const c_char,
        10664 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh173, message.as_mut_ptr());
    if !root.is_null() {
        Json_dispose(root);
    }
}
unsafe extern "C" fn toColor(mut value: *const c_char, mut index: c_int) -> c_float {
    let mut digits: [c_char; 3] = [0; 3];
    let mut error: *mut c_char = 0 as *mut c_char;
    let mut color: c_int = 0;
    if index as size_t >= (spine_strlen(value)).wrapping_div(2 as c_int as c_ulong) {
        return -(1 as c_int) as c_float;
    }
    value = value.offset((index * 2 as c_int) as isize);
    digits[0 as c_int as usize] = *value;
    digits[1 as c_int as usize] = *value.offset(1 as c_int as isize);
    digits[2 as c_int as usize] = '\0' as i32 as c_char;
    color = spine_strtoul(digits.as_mut_ptr(), &mut error, 16 as c_int) as c_int;
    if *error as c_int != 0 as c_int {
        return -(1 as c_int) as c_float;
    }
    return color as c_float / 255 as c_int as c_float;
}
unsafe extern "C" fn toColor2(
    mut color: *mut spColor,
    mut value: *const c_char,
    mut hasAlpha: c_int,
) {
    (*color).r = toColor(value, 0 as c_int);
    (*color).g = toColor(value, 1 as c_int);
    (*color).b = toColor(value, 2 as c_int);
    if hasAlpha != 0 {
        (*color).a = toColor(value, 3 as c_int);
    }
}
unsafe extern "C" fn setBezierJson(
    mut timeline: *mut spCurveTimeline,
    mut frame: c_int,
    mut value: c_int,
    mut bezier: c_int,
    mut time1: c_float,
    mut value1: c_float,
    mut cx1: c_float,
    mut cy1: c_float,
    mut cx2: c_float,
    mut cy2: c_float,
    mut time2: c_float,
    mut value2: c_float,
) {
    spTimeline_setBezier(
        &mut (*timeline).super_0,
        bezier,
        frame,
        value as c_float,
        time1,
        value1,
        cx1,
        cy1,
        cx2,
        cy2,
        time2,
        value2,
    );
}
unsafe extern "C" fn readCurve(
    mut curve: *mut Json,
    mut timeline: *mut spCurveTimeline,
    mut bezier: c_int,
    mut frame: c_int,
    mut value: c_int,
    mut time1: c_float,
    mut time2: c_float,
    mut value1: c_float,
    mut value2: c_float,
    mut scale: c_float,
) -> c_int {
    let mut cx1: c_float = 0.;
    let mut cy1: c_float = 0.;
    let mut cx2: c_float = 0.;
    let mut cy2: c_float = 0.;
    if (*curve).type_0 == 4 as c_int
        && spine_strcmp(
            (*curve).valueString,
            b"stepped\0" as *const u8 as *const c_char,
        ) == 0 as c_int
    {
        if value != 0 as c_int {
            spCurveTimeline_setStepped(timeline, frame);
        }
        return bezier;
    }
    curve = Json_getItemAtIndex(curve, value << 2 as c_int);
    cx1 = (*curve).valueFloat;
    curve = (*curve).next;
    cy1 = (*curve).valueFloat * scale;
    curve = (*curve).next;
    cx2 = (*curve).valueFloat;
    curve = (*curve).next;
    cy2 = (*curve).valueFloat * scale;
    setBezierJson(
        timeline, frame, value, bezier, time1, value1, cx1, cy1, cx2, cy2, time2, value2,
    );
    return bezier + 1 as c_int;
}
unsafe extern "C" fn readTimelineJson(
    mut keyMap: *mut Json,
    mut timeline: *mut spCurveTimeline1,
    mut defaultValue: c_float,
    mut scale: c_float,
) -> *mut spTimeline {
    let mut time: c_float = Json_getFloat(
        keyMap,
        b"time\0" as *const u8 as *const c_char,
        0 as c_int as c_float,
    );
    let mut value: c_float = Json_getFloat(
        keyMap,
        b"value\0" as *const u8 as *const c_char,
        defaultValue,
    ) * scale;
    let mut frame: c_int = 0;
    let mut bezier: c_int = 0 as c_int;
    frame = 0 as c_int;
    loop {
        let mut nextMap: *mut Json = 0 as *mut Json;
        let mut curve: *mut Json = 0 as *mut Json;
        let mut time2: c_float = 0.;
        let mut value2: c_float = 0.;
        spCurveTimeline1_setFrame(timeline, frame, time, value);
        nextMap = (*keyMap).next;
        if nextMap.is_null() {
            break;
        }
        time2 = Json_getFloat(
            nextMap,
            b"time\0" as *const u8 as *const c_char,
            0 as c_int as c_float,
        );
        value2 = Json_getFloat(
            nextMap,
            b"value\0" as *const u8 as *const c_char,
            defaultValue,
        ) * scale;
        curve = Json_getItem(keyMap, b"curve\0" as *const u8 as *const c_char);
        if !curve.is_null() {
            bezier = readCurve(
                curve, timeline, bezier, frame, 0 as c_int, time, time2, value, value2, scale,
            );
        }
        time = time2;
        value = value2;
        keyMap = nextMap;
        frame += 1;
    }
    return &mut (*timeline).super_0;
}
unsafe extern "C" fn readTimeline2Json(
    mut keyMap: *mut Json,
    mut timeline: *mut spCurveTimeline2,
    mut name1: *const c_char,
    mut name2: *const c_char,
    mut defaultValue: c_float,
    mut scale: c_float,
) -> *mut spTimeline {
    let mut time: c_float = Json_getFloat(
        keyMap,
        b"time\0" as *const u8 as *const c_char,
        0 as c_int as c_float,
    );
    let mut value1: c_float = Json_getFloat(keyMap, name1, defaultValue) * scale;
    let mut value2: c_float = Json_getFloat(keyMap, name2, defaultValue) * scale;
    let mut frame: c_int = 0;
    let mut bezier: c_int = 0 as c_int;
    frame = 0 as c_int;
    loop {
        let mut nextMap: *mut Json = 0 as *mut Json;
        let mut curve: *mut Json = 0 as *mut Json;
        let mut time2: c_float = 0.;
        let mut nvalue1: c_float = 0.;
        let mut nvalue2: c_float = 0.;
        spCurveTimeline2_setFrame(timeline, frame, time, value1, value2);
        nextMap = (*keyMap).next;
        if nextMap.is_null() {
            break;
        }
        time2 = Json_getFloat(
            nextMap,
            b"time\0" as *const u8 as *const c_char,
            0 as c_int as c_float,
        );
        nvalue1 = Json_getFloat(nextMap, name1, defaultValue) * scale;
        nvalue2 = Json_getFloat(nextMap, name2, defaultValue) * scale;
        curve = Json_getItem(keyMap, b"curve\0" as *const u8 as *const c_char);
        if !curve.is_null() {
            bezier = readCurve(
                curve, timeline, bezier, frame, 0 as c_int, time, time2, value1, nvalue1, scale,
            );
            bezier = readCurve(
                curve, timeline, bezier, frame, 1 as c_int, time, time2, value2, nvalue2, scale,
            );
        }
        time = time2;
        value1 = nvalue1;
        value2 = nvalue2;
        keyMap = nextMap;
        frame += 1;
    }
    return &mut (*timeline).super_0;
}
unsafe extern "C" fn readSequenceJson(mut item: *mut Json) -> *mut spSequence {
    let mut sequence: *mut spSequence = 0 as *mut spSequence;
    if item.is_null() {
        return 0 as *mut spSequence;
    }
    sequence = spSequence_create(Json_getInt(
        item,
        b"count\0" as *const u8 as *const c_char,
        0 as c_int,
    ));
    (*sequence).start = Json_getInt(item, b"start\0" as *const u8 as *const c_char, 1 as c_int);
    (*sequence).digits = Json_getInt(item, b"digits\0" as *const u8 as *const c_char, 0 as c_int);
    (*sequence).setupIndex = Json_getInt(
        item,
        b"setupIndex\0" as *const u8 as *const c_char,
        0 as c_int,
    );
    return sequence;
}
unsafe extern "C" fn _spSkeletonJson_addLinkedMesh(
    mut self_0: *mut spSkeletonJson,
    mut mesh: *mut spMeshAttachment,
    mut skin: *const c_char,
    mut slotIndex: c_int,
    mut parent: *const c_char,
    mut inheritDeform: c_int,
) {
    let mut linkedMesh: *mut _spLinkedMeshJson = 0 as *mut _spLinkedMeshJson;
    let mut internal: *mut _spSkeletonJson = self_0 as *mut _spSkeletonJson;
    if (*internal).linkedMeshCount == (*internal).linkedMeshCapacity {
        let mut linkedMeshes: *mut _spLinkedMeshJson = 0 as *mut _spLinkedMeshJson;
        (*internal).linkedMeshCapacity *= 2 as c_int;
        if (*internal).linkedMeshCapacity < 8 as c_int {
            (*internal).linkedMeshCapacity = 8 as c_int;
        }
        linkedMeshes = _spMalloc(
            (::core::mem::size_of::<_spLinkedMeshJson>() as c_ulong)
                .wrapping_mul((*internal).linkedMeshCapacity as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            10787 as c_int,
        ) as *mut _spLinkedMeshJson;
        spine_memcpy(
            linkedMeshes as *mut c_void,
            (*internal).linkedMeshes as *const c_void,
            (::core::mem::size_of::<_spLinkedMeshJson>() as c_ulong)
                .wrapping_mul((*internal).linkedMeshCount as c_ulong),
        );
        _spFree((*internal).linkedMeshes as *mut c_void);
        (*internal).linkedMeshes = linkedMeshes;
    }
    let fresh174 = (*internal).linkedMeshCount;
    (*internal).linkedMeshCount = (*internal).linkedMeshCount + 1;
    linkedMesh = ((*internal).linkedMeshes).offset(fresh174 as isize);
    (*linkedMesh).mesh = mesh;
    (*linkedMesh).skin = skin;
    (*linkedMesh).slotIndex = slotIndex;
    (*linkedMesh).parent = parent;
    (*linkedMesh).inheritTimeline = inheritDeform;
}
unsafe extern "C" fn cleanUpTimelines(mut timelines: *mut spTimelineArray) {
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    i = 0 as c_int;
    n = (*timelines).size;
    while i < n {
        spTimeline_dispose(*((*timelines).items).offset(i as isize));
        i += 1;
    }
    spTimelineArray_dispose(timelines);
}
unsafe extern "C" fn findSlotIndex(
    mut json: *mut spSkeletonJson,
    mut skeletonData: *const spSkeletonData,
    mut slotName: *const c_char,
    mut timelines: *mut spTimelineArray,
) -> c_int {
    let mut slot: *mut spSlotData = spSkeletonData_findSlot(skeletonData, slotName);
    if !slot.is_null() {
        return (*slot).index;
    }
    cleanUpTimelines(timelines);
    _spSkeletonJson_setError(
        json,
        0 as *mut Json,
        b"Slot not found: \0" as *const u8 as *const c_char,
        slotName,
    );
    return -(1 as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn findIkConstraintIndex(
    mut json: *mut spSkeletonJson,
    mut skeletonData: *const spSkeletonData,
    mut constraint: *const spIkConstraintData,
    mut timelines: *mut spTimelineArray,
) -> c_int {
    if !constraint.is_null() {
        let mut i: c_int = 0;
        i = 0 as c_int;
        while i < (*skeletonData).ikConstraintsCount {
            if *((*skeletonData).ikConstraints).offset(i as isize)
                == constraint as *mut spIkConstraintData
            {
                return i;
            }
            i += 1;
        }
    }
    cleanUpTimelines(timelines);
    _spSkeletonJson_setError(
        json,
        0 as *mut Json,
        b"IK constraint not found: \0" as *const u8 as *const c_char,
        (*constraint).name,
    );
    return -(1 as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn findTransformConstraintIndex(
    mut json: *mut spSkeletonJson,
    mut skeletonData: *const spSkeletonData,
    mut constraint: *const spTransformConstraintData,
    mut timelines: *mut spTimelineArray,
) -> c_int {
    if !constraint.is_null() {
        let mut i: c_int = 0;
        i = 0 as c_int;
        while i < (*skeletonData).transformConstraintsCount {
            if *((*skeletonData).transformConstraints).offset(i as isize)
                == constraint as *mut spTransformConstraintData
            {
                return i;
            }
            i += 1;
        }
    }
    cleanUpTimelines(timelines);
    _spSkeletonJson_setError(
        json,
        0 as *mut Json,
        b"Transform constraint not found: \0" as *const u8 as *const c_char,
        (*constraint).name,
    );
    return -(1 as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn findPathConstraintIndex(
    mut json: *mut spSkeletonJson,
    mut skeletonData: *const spSkeletonData,
    mut constraint: *const spPathConstraintData,
    mut timelines: *mut spTimelineArray,
) -> c_int {
    if !constraint.is_null() {
        let mut i: c_int = 0;
        i = 0 as c_int;
        while i < (*skeletonData).pathConstraintsCount {
            if *((*skeletonData).pathConstraints).offset(i as isize)
                == constraint as *mut spPathConstraintData
            {
                return i;
            }
            i += 1;
        }
    }
    cleanUpTimelines(timelines);
    _spSkeletonJson_setError(
        json,
        0 as *mut Json,
        b"Path constraint not found: \0" as *const u8 as *const c_char,
        (*constraint).name,
    );
    return -(1 as c_int);
}
unsafe extern "C" fn _spSkeletonJson_readAnimation(
    mut self_0: *mut spSkeletonJson,
    mut root: *mut Json,
    mut skeletonData: *mut spSkeletonData,
) -> *mut spAnimation {
    let mut timelines: *mut spTimelineArray = spTimelineArray_create(8 as c_int);
    let mut scale: c_float = (*self_0).scale;
    let mut duration: c_float = 0.;
    let mut bones: *mut Json = Json_getItem(root, b"bones\0" as *const u8 as *const c_char);
    let mut slots: *mut Json = Json_getItem(root, b"slots\0" as *const u8 as *const c_char);
    let mut ik: *mut Json = Json_getItem(root, b"ik\0" as *const u8 as *const c_char);
    let mut transform: *mut Json = Json_getItem(root, b"transform\0" as *const u8 as *const c_char);
    let mut paths: *mut Json = Json_getItem(root, b"path\0" as *const u8 as *const c_char);
    let mut attachmentsJson: *mut Json =
        Json_getItem(root, b"attachments\0" as *const u8 as *const c_char);
    let mut drawOrderJson: *mut Json =
        Json_getItem(root, b"drawOrder\0" as *const u8 as *const c_char);
    let mut events: *mut Json = Json_getItem(root, b"events\0" as *const u8 as *const c_char);
    let mut boneMap: *mut Json = 0 as *mut Json;
    let mut slotMap: *mut Json = 0 as *mut Json;
    let mut keyMap: *mut Json = 0 as *mut Json;
    let mut nextMap: *mut Json = 0 as *mut Json;
    let mut curve: *mut Json = 0 as *mut Json;
    let mut timelineMap: *mut Json = 0 as *mut Json;
    let mut attachmentsMap: *mut Json = 0 as *mut Json;
    let mut constraintMap: *mut Json = 0 as *mut Json;
    let mut frame: c_int = 0;
    let mut bezier: c_int = 0;
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    let mut color: spColor = spColor {
        r: 0.,
        g: 0.,
        b: 0.,
        a: 0.,
    };
    let mut color2: spColor = spColor {
        r: 0.,
        g: 0.,
        b: 0.,
        a: 0.,
    };
    let mut newColor: spColor = spColor {
        r: 0.,
        g: 0.,
        b: 0.,
        a: 0.,
    };
    let mut newColor2: spColor = spColor {
        r: 0.,
        g: 0.,
        b: 0.,
        a: 0.,
    };
    slotMap = if !slots.is_null() {
        (*slots).child
    } else {
        0 as *mut Json
    };
    while !slotMap.is_null() {
        let mut slotIndex: c_int = findSlotIndex(self_0, skeletonData, (*slotMap).name, timelines);
        if slotIndex == -(1 as c_int) {
            return 0 as *mut spAnimation;
        }
        timelineMap = (*slotMap).child;
        while !timelineMap.is_null() {
            let mut frames: c_int = (*timelineMap).size;
            if spine_strcmp(
                (*timelineMap).name,
                b"attachment\0" as *const u8 as *const c_char,
            ) == 0 as c_int
            {
                let mut timeline: *mut spAttachmentTimeline =
                    spAttachmentTimeline_create(frames, slotIndex);
                keyMap = (*timelineMap).child;
                frame = 0 as c_int;
                while !keyMap.is_null() {
                    spAttachmentTimeline_setFrame(
                        timeline,
                        frame,
                        Json_getFloat(
                            keyMap,
                            b"time\0" as *const u8 as *const c_char,
                            0 as c_int as c_float,
                        ),
                        if !(Json_getItem(keyMap, b"name\0" as *const u8 as *const c_char))
                            .is_null()
                        {
                            (*Json_getItem(keyMap, b"name\0" as *const u8 as *const c_char))
                                .valueString
                        } else {
                            0 as *const c_char
                        },
                    );
                    keyMap = (*keyMap).next;
                    frame += 1;
                }
                spTimelineArray_add(timelines, &mut (*timeline).super_0);
            } else if spine_strcmp((*timelineMap).name, b"rgba\0" as *const u8 as *const c_char)
                == 0 as c_int
            {
                let mut time: c_float = 0.;
                let mut timeline_0: *mut spRGBATimeline =
                    spRGBATimeline_create(frames, frames << 2 as c_int, slotIndex);
                keyMap = (*timelineMap).child;
                time = Json_getFloat(
                    keyMap,
                    b"time\0" as *const u8 as *const c_char,
                    0 as c_int as c_float,
                );
                toColor2(
                    &mut color,
                    Json_getString(
                        keyMap,
                        b"color\0" as *const u8 as *const c_char,
                        0 as *const c_char,
                    ),
                    1 as c_int,
                );
                frame = 0 as c_int;
                bezier = 0 as c_int;
                loop {
                    let mut time2: c_float = 0.;
                    spRGBATimeline_setFrame(
                        timeline_0, frame, time, color.r, color.g, color.b, color.a,
                    );
                    nextMap = (*keyMap).next;
                    if nextMap.is_null() {
                        break;
                    }
                    time2 = Json_getFloat(
                        nextMap,
                        b"time\0" as *const u8 as *const c_char,
                        0 as c_int as c_float,
                    );
                    toColor2(
                        &mut newColor,
                        Json_getString(
                            nextMap,
                            b"color\0" as *const u8 as *const c_char,
                            0 as *const c_char,
                        ),
                        1 as c_int,
                    );
                    curve = Json_getItem(keyMap, b"curve\0" as *const u8 as *const c_char);
                    if !curve.is_null() {
                        bezier = readCurve(
                            curve,
                            &mut (*timeline_0).super_0,
                            bezier,
                            frame,
                            0 as c_int,
                            time,
                            time2,
                            color.r,
                            newColor.r,
                            1 as c_int as c_float,
                        );
                        bezier = readCurve(
                            curve,
                            &mut (*timeline_0).super_0,
                            bezier,
                            frame,
                            1 as c_int,
                            time,
                            time2,
                            color.g,
                            newColor.g,
                            1 as c_int as c_float,
                        );
                        bezier = readCurve(
                            curve,
                            &mut (*timeline_0).super_0,
                            bezier,
                            frame,
                            2 as c_int,
                            time,
                            time2,
                            color.b,
                            newColor.b,
                            1 as c_int as c_float,
                        );
                        bezier = readCurve(
                            curve,
                            &mut (*timeline_0).super_0,
                            bezier,
                            frame,
                            3 as c_int,
                            time,
                            time2,
                            color.a,
                            newColor.a,
                            1 as c_int as c_float,
                        );
                    }
                    time = time2;
                    color = newColor;
                    keyMap = nextMap;
                    frame += 1;
                }
                spTimelineArray_add(timelines, &mut (*timeline_0).super_0.super_0);
            } else if spine_strcmp((*timelineMap).name, b"rgb\0" as *const u8 as *const c_char)
                == 0 as c_int
            {
                let mut time_0: c_float = 0.;
                let mut timeline_1: *mut spRGBTimeline =
                    spRGBTimeline_create(frames, frames * 3 as c_int, slotIndex);
                keyMap = (*timelineMap).child;
                time_0 = Json_getFloat(
                    keyMap,
                    b"time\0" as *const u8 as *const c_char,
                    0 as c_int as c_float,
                );
                toColor2(
                    &mut color,
                    Json_getString(
                        keyMap,
                        b"color\0" as *const u8 as *const c_char,
                        0 as *const c_char,
                    ),
                    1 as c_int,
                );
                frame = 0 as c_int;
                bezier = 0 as c_int;
                loop {
                    let mut time2_0: c_float = 0.;
                    spRGBTimeline_setFrame(timeline_1, frame, time_0, color.r, color.g, color.b);
                    nextMap = (*keyMap).next;
                    if nextMap.is_null() {
                        break;
                    }
                    time2_0 = Json_getFloat(
                        nextMap,
                        b"time\0" as *const u8 as *const c_char,
                        0 as c_int as c_float,
                    );
                    toColor2(
                        &mut newColor,
                        Json_getString(
                            nextMap,
                            b"color\0" as *const u8 as *const c_char,
                            0 as *const c_char,
                        ),
                        1 as c_int,
                    );
                    curve = Json_getItem(keyMap, b"curve\0" as *const u8 as *const c_char);
                    if !curve.is_null() {
                        bezier = readCurve(
                            curve,
                            &mut (*timeline_1).super_0,
                            bezier,
                            frame,
                            0 as c_int,
                            time_0,
                            time2_0,
                            color.r,
                            newColor.r,
                            1 as c_int as c_float,
                        );
                        bezier = readCurve(
                            curve,
                            &mut (*timeline_1).super_0,
                            bezier,
                            frame,
                            1 as c_int,
                            time_0,
                            time2_0,
                            color.g,
                            newColor.g,
                            1 as c_int as c_float,
                        );
                        bezier = readCurve(
                            curve,
                            &mut (*timeline_1).super_0,
                            bezier,
                            frame,
                            2 as c_int,
                            time_0,
                            time2_0,
                            color.b,
                            newColor.b,
                            1 as c_int as c_float,
                        );
                    }
                    time_0 = time2_0;
                    color = newColor;
                    keyMap = nextMap;
                    frame += 1;
                }
                spTimelineArray_add(timelines, &mut (*timeline_1).super_0.super_0);
            } else if spine_strcmp(
                (*timelineMap).name,
                b"alpha\0" as *const u8 as *const c_char,
            ) == 0 as c_int
            {
                spTimelineArray_add(
                    timelines,
                    readTimelineJson(
                        (*timelineMap).child,
                        &mut (*(spAlphaTimeline_create
                            as unsafe extern "C" fn(c_int, c_int, c_int) -> *mut spAlphaTimeline)(
                            frames, frames, slotIndex,
                        ))
                        .super_0,
                        0 as c_int as c_float,
                        1 as c_int as c_float,
                    ),
                );
            } else if spine_strcmp(
                (*timelineMap).name,
                b"rgba2\0" as *const u8 as *const c_char,
            ) == 0 as c_int
            {
                let mut time_1: c_float = 0.;
                let mut timeline_2: *mut spRGBA2Timeline =
                    spRGBA2Timeline_create(frames, frames * 7 as c_int, slotIndex);
                keyMap = (*timelineMap).child;
                time_1 = Json_getFloat(
                    keyMap,
                    b"time\0" as *const u8 as *const c_char,
                    0 as c_int as c_float,
                );
                toColor2(
                    &mut color,
                    Json_getString(
                        keyMap,
                        b"light\0" as *const u8 as *const c_char,
                        0 as *const c_char,
                    ),
                    1 as c_int,
                );
                toColor2(
                    &mut color2,
                    Json_getString(
                        keyMap,
                        b"dark\0" as *const u8 as *const c_char,
                        0 as *const c_char,
                    ),
                    0 as c_int,
                );
                frame = 0 as c_int;
                bezier = 0 as c_int;
                loop {
                    let mut time2_1: c_float = 0.;
                    spRGBA2Timeline_setFrame(
                        timeline_2, frame, time_1, color.r, color.g, color.b, color.a, color2.g,
                        color2.g, color2.b,
                    );
                    nextMap = (*keyMap).next;
                    if nextMap.is_null() {
                        break;
                    }
                    time2_1 = Json_getFloat(
                        nextMap,
                        b"time\0" as *const u8 as *const c_char,
                        0 as c_int as c_float,
                    );
                    toColor2(
                        &mut newColor,
                        Json_getString(
                            nextMap,
                            b"light\0" as *const u8 as *const c_char,
                            0 as *const c_char,
                        ),
                        1 as c_int,
                    );
                    toColor2(
                        &mut newColor2,
                        Json_getString(
                            nextMap,
                            b"dark\0" as *const u8 as *const c_char,
                            0 as *const c_char,
                        ),
                        0 as c_int,
                    );
                    curve = Json_getItem(keyMap, b"curve\0" as *const u8 as *const c_char);
                    if !curve.is_null() {
                        bezier = readCurve(
                            curve,
                            &mut (*timeline_2).super_0,
                            bezier,
                            frame,
                            0 as c_int,
                            time_1,
                            time2_1,
                            color.r,
                            newColor.r,
                            1 as c_int as c_float,
                        );
                        bezier = readCurve(
                            curve,
                            &mut (*timeline_2).super_0,
                            bezier,
                            frame,
                            1 as c_int,
                            time_1,
                            time2_1,
                            color.g,
                            newColor.g,
                            1 as c_int as c_float,
                        );
                        bezier = readCurve(
                            curve,
                            &mut (*timeline_2).super_0,
                            bezier,
                            frame,
                            2 as c_int,
                            time_1,
                            time2_1,
                            color.b,
                            newColor.b,
                            1 as c_int as c_float,
                        );
                        bezier = readCurve(
                            curve,
                            &mut (*timeline_2).super_0,
                            bezier,
                            frame,
                            3 as c_int,
                            time_1,
                            time2_1,
                            color.a,
                            newColor.a,
                            1 as c_int as c_float,
                        );
                        bezier = readCurve(
                            curve,
                            &mut (*timeline_2).super_0,
                            bezier,
                            frame,
                            4 as c_int,
                            time_1,
                            time2_1,
                            color2.r,
                            newColor2.r,
                            1 as c_int as c_float,
                        );
                        bezier = readCurve(
                            curve,
                            &mut (*timeline_2).super_0,
                            bezier,
                            frame,
                            5 as c_int,
                            time_1,
                            time2_1,
                            color2.g,
                            newColor2.g,
                            1 as c_int as c_float,
                        );
                        bezier = readCurve(
                            curve,
                            &mut (*timeline_2).super_0,
                            bezier,
                            frame,
                            6 as c_int,
                            time_1,
                            time2_1,
                            color2.b,
                            newColor2.b,
                            1 as c_int as c_float,
                        );
                    }
                    time_1 = time2_1;
                    color = newColor;
                    color2 = newColor2;
                    keyMap = nextMap;
                    frame += 1;
                }
                spTimelineArray_add(timelines, &mut (*timeline_2).super_0.super_0);
            } else if spine_strcmp((*timelineMap).name, b"rgb2\0" as *const u8 as *const c_char)
                == 0 as c_int
            {
                let mut time_2: c_float = 0.;
                let mut timeline_3: *mut spRGBA2Timeline =
                    spRGBA2Timeline_create(frames, frames * 6 as c_int, slotIndex);
                keyMap = (*timelineMap).child;
                time_2 = Json_getFloat(
                    keyMap,
                    b"time\0" as *const u8 as *const c_char,
                    0 as c_int as c_float,
                );
                toColor2(
                    &mut color,
                    Json_getString(
                        keyMap,
                        b"light\0" as *const u8 as *const c_char,
                        0 as *const c_char,
                    ),
                    0 as c_int,
                );
                toColor2(
                    &mut color2,
                    Json_getString(
                        keyMap,
                        b"dark\0" as *const u8 as *const c_char,
                        0 as *const c_char,
                    ),
                    0 as c_int,
                );
                frame = 0 as c_int;
                bezier = 0 as c_int;
                loop {
                    let mut time2_2: c_float = 0.;
                    spRGBA2Timeline_setFrame(
                        timeline_3, frame, time_2, color.r, color.g, color.b, color.a, color2.r,
                        color2.g, color2.b,
                    );
                    nextMap = (*keyMap).next;
                    if nextMap.is_null() {
                        break;
                    }
                    time2_2 = Json_getFloat(
                        nextMap,
                        b"time\0" as *const u8 as *const c_char,
                        0 as c_int as c_float,
                    );
                    toColor2(
                        &mut newColor,
                        Json_getString(
                            nextMap,
                            b"light\0" as *const u8 as *const c_char,
                            0 as *const c_char,
                        ),
                        0 as c_int,
                    );
                    toColor2(
                        &mut newColor2,
                        Json_getString(
                            nextMap,
                            b"dark\0" as *const u8 as *const c_char,
                            0 as *const c_char,
                        ),
                        0 as c_int,
                    );
                    curve = Json_getItem(keyMap, b"curve\0" as *const u8 as *const c_char);
                    if !curve.is_null() {
                        bezier = readCurve(
                            curve,
                            &mut (*timeline_3).super_0,
                            bezier,
                            frame,
                            0 as c_int,
                            time_2,
                            time2_2,
                            color.r,
                            newColor.r,
                            1 as c_int as c_float,
                        );
                        bezier = readCurve(
                            curve,
                            &mut (*timeline_3).super_0,
                            bezier,
                            frame,
                            1 as c_int,
                            time_2,
                            time2_2,
                            color.g,
                            newColor.g,
                            1 as c_int as c_float,
                        );
                        bezier = readCurve(
                            curve,
                            &mut (*timeline_3).super_0,
                            bezier,
                            frame,
                            2 as c_int,
                            time_2,
                            time2_2,
                            color.b,
                            newColor.b,
                            1 as c_int as c_float,
                        );
                        bezier = readCurve(
                            curve,
                            &mut (*timeline_3).super_0,
                            bezier,
                            frame,
                            3 as c_int,
                            time_2,
                            time2_2,
                            color2.r,
                            newColor2.r,
                            1 as c_int as c_float,
                        );
                        bezier = readCurve(
                            curve,
                            &mut (*timeline_3).super_0,
                            bezier,
                            frame,
                            4 as c_int,
                            time_2,
                            time2_2,
                            color2.g,
                            newColor2.g,
                            1 as c_int as c_float,
                        );
                        bezier = readCurve(
                            curve,
                            &mut (*timeline_3).super_0,
                            bezier,
                            frame,
                            5 as c_int,
                            time_2,
                            time2_2,
                            color2.b,
                            newColor2.b,
                            1 as c_int as c_float,
                        );
                    }
                    time_2 = time2_2;
                    color = newColor;
                    color2 = newColor2;
                    keyMap = nextMap;
                    frame += 1;
                }
                spTimelineArray_add(timelines, &mut (*timeline_3).super_0.super_0);
            } else {
                cleanUpTimelines(timelines);
                _spSkeletonJson_setError(
                    self_0,
                    0 as *mut Json,
                    b"Invalid timeline type for a slot: \0" as *const u8 as *const c_char,
                    (*timelineMap).name,
                );
                return 0 as *mut spAnimation;
            }
            timelineMap = (*timelineMap).next;
        }
        slotMap = (*slotMap).next;
    }
    boneMap = if !bones.is_null() {
        (*bones).child
    } else {
        0 as *mut Json
    };
    while !boneMap.is_null() {
        let mut boneIndex: c_int = -(1 as c_int);
        i = 0 as c_int;
        while i < (*skeletonData).bonesCount {
            if spine_strcmp(
                (**((*skeletonData).bones).offset(i as isize)).name,
                (*boneMap).name,
            ) == 0 as c_int
            {
                boneIndex = i;
                break;
            } else {
                i += 1;
            }
        }
        if boneIndex == -(1 as c_int) {
            cleanUpTimelines(timelines);
            _spSkeletonJson_setError(
                self_0,
                0 as *mut Json,
                b"Bone not found: \0" as *const u8 as *const c_char,
                (*boneMap).name,
            );
            return 0 as *mut spAnimation;
        }
        timelineMap = (*boneMap).child;
        while !timelineMap.is_null() {
            let mut frames_0: c_int = (*timelineMap).size;
            if !(frames_0 == 0 as c_int) {
                if spine_strcmp(
                    (*timelineMap).name,
                    b"rotate\0" as *const u8 as *const c_char,
                ) == 0 as c_int
                {
                    spTimelineArray_add(
                        timelines,
                        readTimelineJson(
                            (*timelineMap).child,
                            &mut (*(spRotateTimeline_create
                                as unsafe extern "C" fn(
                                    c_int,
                                    c_int,
                                    c_int,
                                )
                                    -> *mut spRotateTimeline)(
                                frames_0, frames_0, boneIndex
                            ))
                            .super_0,
                            0 as c_int as c_float,
                            1 as c_int as c_float,
                        ),
                    );
                } else if spine_strcmp(
                    (*timelineMap).name,
                    b"translate\0" as *const u8 as *const c_char,
                ) == 0 as c_int
                {
                    let mut timeline_4: *mut spTranslateTimeline =
                        spTranslateTimeline_create(frames_0, frames_0 << 1 as c_int, boneIndex);
                    spTimelineArray_add(
                        timelines,
                        readTimeline2Json(
                            (*timelineMap).child,
                            &mut (*timeline_4).super_0,
                            b"x\0" as *const u8 as *const c_char,
                            b"y\0" as *const u8 as *const c_char,
                            0 as c_int as c_float,
                            scale,
                        ),
                    );
                } else if spine_strcmp(
                    (*timelineMap).name,
                    b"translatex\0" as *const u8 as *const c_char,
                ) == 0 as c_int
                {
                    let mut timeline_5: *mut spTranslateXTimeline =
                        spTranslateXTimeline_create(frames_0, frames_0, boneIndex);
                    spTimelineArray_add(
                        timelines,
                        readTimelineJson(
                            (*timelineMap).child,
                            &mut (*timeline_5).super_0,
                            0 as c_int as c_float,
                            scale,
                        ),
                    );
                } else if spine_strcmp(
                    (*timelineMap).name,
                    b"translatey\0" as *const u8 as *const c_char,
                ) == 0 as c_int
                {
                    let mut timeline_6: *mut spTranslateYTimeline =
                        spTranslateYTimeline_create(frames_0, frames_0, boneIndex);
                    spTimelineArray_add(
                        timelines,
                        readTimelineJson(
                            (*timelineMap).child,
                            &mut (*timeline_6).super_0,
                            0 as c_int as c_float,
                            scale,
                        ),
                    );
                } else if spine_strcmp(
                    (*timelineMap).name,
                    b"scale\0" as *const u8 as *const c_char,
                ) == 0 as c_int
                {
                    let mut timeline_7: *mut spScaleTimeline =
                        spScaleTimeline_create(frames_0, frames_0 << 1 as c_int, boneIndex);
                    spTimelineArray_add(
                        timelines,
                        readTimeline2Json(
                            (*timelineMap).child,
                            &mut (*timeline_7).super_0,
                            b"x\0" as *const u8 as *const c_char,
                            b"y\0" as *const u8 as *const c_char,
                            1 as c_int as c_float,
                            1 as c_int as c_float,
                        ),
                    );
                } else if spine_strcmp(
                    (*timelineMap).name,
                    b"scalex\0" as *const u8 as *const c_char,
                ) == 0 as c_int
                {
                    let mut timeline_8: *mut spScaleXTimeline =
                        spScaleXTimeline_create(frames_0, frames_0, boneIndex);
                    spTimelineArray_add(
                        timelines,
                        readTimelineJson(
                            (*timelineMap).child,
                            &mut (*timeline_8).super_0,
                            1 as c_int as c_float,
                            1 as c_int as c_float,
                        ),
                    );
                } else if spine_strcmp(
                    (*timelineMap).name,
                    b"scaley\0" as *const u8 as *const c_char,
                ) == 0 as c_int
                {
                    let mut timeline_9: *mut spScaleYTimeline =
                        spScaleYTimeline_create(frames_0, frames_0, boneIndex);
                    spTimelineArray_add(
                        timelines,
                        readTimelineJson(
                            (*timelineMap).child,
                            &mut (*timeline_9).super_0,
                            1 as c_int as c_float,
                            1 as c_int as c_float,
                        ),
                    );
                } else if spine_strcmp(
                    (*timelineMap).name,
                    b"shear\0" as *const u8 as *const c_char,
                ) == 0 as c_int
                {
                    let mut timeline_10: *mut spShearTimeline =
                        spShearTimeline_create(frames_0, frames_0 << 1 as c_int, boneIndex);
                    spTimelineArray_add(
                        timelines,
                        readTimeline2Json(
                            (*timelineMap).child,
                            &mut (*timeline_10).super_0,
                            b"x\0" as *const u8 as *const c_char,
                            b"y\0" as *const u8 as *const c_char,
                            0 as c_int as c_float,
                            1 as c_int as c_float,
                        ),
                    );
                } else if spine_strcmp(
                    (*timelineMap).name,
                    b"shearx\0" as *const u8 as *const c_char,
                ) == 0 as c_int
                {
                    let mut timeline_11: *mut spShearXTimeline =
                        spShearXTimeline_create(frames_0, frames_0, boneIndex);
                    spTimelineArray_add(
                        timelines,
                        readTimelineJson(
                            (*timelineMap).child,
                            &mut (*timeline_11).super_0,
                            0 as c_int as c_float,
                            1 as c_int as c_float,
                        ),
                    );
                } else if spine_strcmp(
                    (*timelineMap).name,
                    b"sheary\0" as *const u8 as *const c_char,
                ) == 0 as c_int
                {
                    let mut timeline_12: *mut spShearYTimeline =
                        spShearYTimeline_create(frames_0, frames_0, boneIndex);
                    spTimelineArray_add(
                        timelines,
                        readTimelineJson(
                            (*timelineMap).child,
                            &mut (*timeline_12).super_0,
                            0 as c_int as c_float,
                            1 as c_int as c_float,
                        ),
                    );
                } else {
                    cleanUpTimelines(timelines);
                    _spSkeletonJson_setError(
                        self_0,
                        0 as *mut Json,
                        b"Invalid timeline type for a bone: \0" as *const u8 as *const c_char,
                        (*timelineMap).name,
                    );
                    return 0 as *mut spAnimation;
                }
            }
            timelineMap = (*timelineMap).next;
        }
        boneMap = (*boneMap).next;
    }
    constraintMap = if !ik.is_null() {
        (*ik).child
    } else {
        0 as *mut Json
    };
    while !constraintMap.is_null() {
        let mut constraint: *mut spIkConstraintData = 0 as *mut spIkConstraintData;
        let mut timeline_13: *mut spIkConstraintTimeline = 0 as *mut spIkConstraintTimeline;
        let mut constraintIndex: c_int = 0;
        let mut time_3: c_float = 0.;
        let mut mix: c_float = 0.;
        let mut softness: c_float = 0.;
        keyMap = (*constraintMap).child;
        if !keyMap.is_null() {
            constraint = spSkeletonData_findIkConstraint(skeletonData, (*constraintMap).name);
            constraintIndex = findIkConstraintIndex(self_0, skeletonData, constraint, timelines);
            if constraintIndex == -(1 as c_int) {
                return 0 as *mut spAnimation;
            }
            timeline_13 = spIkConstraintTimeline_create(
                (*constraintMap).size,
                (*constraintMap).size << 1 as c_int,
                constraintIndex,
            );
            time_3 = Json_getFloat(
                keyMap,
                b"time\0" as *const u8 as *const c_char,
                0 as c_int as c_float,
            );
            mix = Json_getFloat(
                keyMap,
                b"mix\0" as *const u8 as *const c_char,
                1 as c_int as c_float,
            );
            softness = Json_getFloat(
                keyMap,
                b"softness\0" as *const u8 as *const c_char,
                0 as c_int as c_float,
            ) * scale;
            frame = 0 as c_int;
            bezier = 0 as c_int;
            loop {
                let mut time2_3: c_float = 0.;
                let mut mix2: c_float = 0.;
                let mut softness2: c_float = 0.;
                let mut bendDirection: c_int = if Json_getInt(
                    keyMap,
                    b"bendPositive\0" as *const u8 as *const c_char,
                    1 as c_int,
                ) != 0
                {
                    1 as c_int
                } else {
                    -(1 as c_int)
                };
                spIkConstraintTimeline_setFrame(
                    timeline_13,
                    frame,
                    time_3,
                    mix,
                    softness,
                    bendDirection,
                    if Json_getInt(
                        keyMap,
                        b"compress\0" as *const u8 as *const c_char,
                        0 as c_int,
                    ) != 0
                    {
                        1 as c_int
                    } else {
                        0 as c_int
                    },
                    if Json_getInt(
                        keyMap,
                        b"stretch\0" as *const u8 as *const c_char,
                        0 as c_int,
                    ) != 0
                    {
                        1 as c_int
                    } else {
                        0 as c_int
                    },
                );
                nextMap = (*keyMap).next;
                if nextMap.is_null() {
                    break;
                }
                time2_3 = Json_getFloat(
                    nextMap,
                    b"time\0" as *const u8 as *const c_char,
                    0 as c_int as c_float,
                );
                mix2 = Json_getFloat(
                    nextMap,
                    b"mix\0" as *const u8 as *const c_char,
                    1 as c_int as c_float,
                );
                softness2 = Json_getFloat(
                    nextMap,
                    b"softness\0" as *const u8 as *const c_char,
                    0 as c_int as c_float,
                ) * scale;
                curve = Json_getItem(keyMap, b"curve\0" as *const u8 as *const c_char);
                if !curve.is_null() {
                    bezier = readCurve(
                        curve,
                        &mut (*timeline_13).super_0,
                        bezier,
                        frame,
                        0 as c_int,
                        time_3,
                        time2_3,
                        mix,
                        mix2,
                        1 as c_int as c_float,
                    );
                    bezier = readCurve(
                        curve,
                        &mut (*timeline_13).super_0,
                        bezier,
                        frame,
                        1 as c_int,
                        time_3,
                        time2_3,
                        softness,
                        softness2,
                        scale,
                    );
                }
                time_3 = time2_3;
                mix = mix2;
                softness = softness2;
                keyMap = nextMap;
                frame += 1;
            }
            spTimelineArray_add(timelines, &mut (*timeline_13).super_0.super_0);
        }
        constraintMap = (*constraintMap).next;
    }
    constraintMap = if !transform.is_null() {
        (*transform).child
    } else {
        0 as *mut Json
    };
    while !constraintMap.is_null() {
        let mut constraint_0: *mut spTransformConstraintData = 0 as *mut spTransformConstraintData;
        let mut timeline_14: *mut spTransformConstraintTimeline =
            0 as *mut spTransformConstraintTimeline;
        let mut constraintIndex_0: c_int = 0;
        let mut time_4: c_float = 0.;
        let mut mixRotate: c_float = 0.;
        let mut mixShearY: c_float = 0.;
        let mut mixX: c_float = 0.;
        let mut mixY: c_float = 0.;
        let mut mixScaleX: c_float = 0.;
        let mut mixScaleY: c_float = 0.;
        keyMap = (*constraintMap).child;
        if !keyMap.is_null() {
            constraint_0 =
                spSkeletonData_findTransformConstraint(skeletonData, (*constraintMap).name);
            constraintIndex_0 =
                findTransformConstraintIndex(self_0, skeletonData, constraint_0, timelines);
            if constraintIndex_0 == -(1 as c_int) {
                return 0 as *mut spAnimation;
            }
            timeline_14 = spTransformConstraintTimeline_create(
                (*constraintMap).size,
                (*constraintMap).size * 6 as c_int,
                constraintIndex_0,
            );
            time_4 = Json_getFloat(
                keyMap,
                b"time\0" as *const u8 as *const c_char,
                0 as c_int as c_float,
            );
            mixRotate = Json_getFloat(
                keyMap,
                b"mixRotate\0" as *const u8 as *const c_char,
                1 as c_int as c_float,
            );
            mixShearY = Json_getFloat(
                keyMap,
                b"mixShearY\0" as *const u8 as *const c_char,
                1 as c_int as c_float,
            );
            mixX = Json_getFloat(
                keyMap,
                b"mixX\0" as *const u8 as *const c_char,
                1 as c_int as c_float,
            );
            mixY = Json_getFloat(keyMap, b"mixY\0" as *const u8 as *const c_char, mixX);
            mixScaleX = Json_getFloat(
                keyMap,
                b"mixScaleX\0" as *const u8 as *const c_char,
                1 as c_int as c_float,
            );
            mixScaleY = Json_getFloat(
                keyMap,
                b"mixScaleY\0" as *const u8 as *const c_char,
                mixScaleX,
            );
            frame = 0 as c_int;
            bezier = 0 as c_int;
            loop {
                let mut time2_4: c_float = 0.;
                let mut mixRotate2: c_float = 0.;
                let mut mixShearY2: c_float = 0.;
                let mut mixX2: c_float = 0.;
                let mut mixY2: c_float = 0.;
                let mut mixScaleX2: c_float = 0.;
                let mut mixScaleY2: c_float = 0.;
                spTransformConstraintTimeline_setFrame(
                    timeline_14,
                    frame,
                    time_4,
                    mixRotate,
                    mixX,
                    mixY,
                    mixScaleX,
                    mixScaleY,
                    mixShearY,
                );
                nextMap = (*keyMap).next;
                if nextMap.is_null() {
                    break;
                }
                time2_4 = Json_getFloat(
                    nextMap,
                    b"time\0" as *const u8 as *const c_char,
                    0 as c_int as c_float,
                );
                mixRotate2 = Json_getFloat(
                    nextMap,
                    b"mixRotate\0" as *const u8 as *const c_char,
                    1 as c_int as c_float,
                );
                mixShearY2 = Json_getFloat(
                    nextMap,
                    b"mixShearY\0" as *const u8 as *const c_char,
                    1 as c_int as c_float,
                );
                mixX2 = Json_getFloat(
                    nextMap,
                    b"mixX\0" as *const u8 as *const c_char,
                    1 as c_int as c_float,
                );
                mixY2 = Json_getFloat(nextMap, b"mixY\0" as *const u8 as *const c_char, mixX2);
                mixScaleX2 = Json_getFloat(
                    nextMap,
                    b"mixScaleX\0" as *const u8 as *const c_char,
                    1 as c_int as c_float,
                );
                mixScaleY2 = Json_getFloat(
                    nextMap,
                    b"mixScaleY\0" as *const u8 as *const c_char,
                    mixScaleX2,
                );
                curve = Json_getItem(keyMap, b"curve\0" as *const u8 as *const c_char);
                if !curve.is_null() {
                    bezier = readCurve(
                        curve,
                        &mut (*timeline_14).super_0,
                        bezier,
                        frame,
                        0 as c_int,
                        time_4,
                        time2_4,
                        mixRotate,
                        mixRotate2,
                        1 as c_int as c_float,
                    );
                    bezier = readCurve(
                        curve,
                        &mut (*timeline_14).super_0,
                        bezier,
                        frame,
                        1 as c_int,
                        time_4,
                        time2_4,
                        mixX,
                        mixX2,
                        1 as c_int as c_float,
                    );
                    bezier = readCurve(
                        curve,
                        &mut (*timeline_14).super_0,
                        bezier,
                        frame,
                        2 as c_int,
                        time_4,
                        time2_4,
                        mixY,
                        mixY2,
                        1 as c_int as c_float,
                    );
                    bezier = readCurve(
                        curve,
                        &mut (*timeline_14).super_0,
                        bezier,
                        frame,
                        3 as c_int,
                        time_4,
                        time2_4,
                        mixScaleX,
                        mixScaleX2,
                        1 as c_int as c_float,
                    );
                    bezier = readCurve(
                        curve,
                        &mut (*timeline_14).super_0,
                        bezier,
                        frame,
                        4 as c_int,
                        time_4,
                        time2_4,
                        mixScaleY,
                        mixScaleY2,
                        1 as c_int as c_float,
                    );
                    bezier = readCurve(
                        curve,
                        &mut (*timeline_14).super_0,
                        bezier,
                        frame,
                        5 as c_int,
                        time_4,
                        time2_4,
                        mixShearY,
                        mixShearY2,
                        1 as c_int as c_float,
                    );
                }
                time_4 = time2_4;
                mixRotate = mixRotate2;
                mixX = mixX2;
                mixY = mixY2;
                mixScaleX = mixScaleX2;
                mixScaleY = mixScaleY2;
                mixScaleX = mixScaleX2;
                keyMap = nextMap;
                frame += 1;
            }
            spTimelineArray_add(timelines, &mut (*timeline_14).super_0.super_0);
        }
        constraintMap = (*constraintMap).next;
    }
    constraintMap = if !paths.is_null() {
        (*paths).child
    } else {
        0 as *mut Json
    };
    while !constraintMap.is_null() {
        let mut constraint_1: *mut spPathConstraintData =
            spSkeletonData_findPathConstraint(skeletonData, (*constraintMap).name);
        let mut constraintIndex_1: c_int =
            findPathConstraintIndex(self_0, skeletonData, constraint_1, timelines);
        if constraintIndex_1 == -(1 as c_int) {
            return 0 as *mut spAnimation;
        }
        timelineMap = (*constraintMap).child;
        while !timelineMap.is_null() {
            let mut timelineName: *const c_char = 0 as *const c_char;
            let mut frames_1: c_int = 0;
            keyMap = (*timelineMap).child;
            if !keyMap.is_null() {
                frames_1 = (*timelineMap).size;
                timelineName = (*timelineMap).name;
                if spine_strcmp(timelineName, b"position\0" as *const u8 as *const c_char)
                    == 0 as c_int
                {
                    let mut timeline_15: *mut spPathConstraintPositionTimeline =
                        spPathConstraintPositionTimeline_create(
                            frames_1,
                            frames_1,
                            constraintIndex_1,
                        );
                    spTimelineArray_add(
                        timelines,
                        readTimelineJson(
                            keyMap,
                            &mut (*timeline_15).super_0,
                            0 as c_int as c_float,
                            if (*constraint_1).positionMode as c_uint
                                == SP_POSITION_MODE_FIXED as c_int as c_uint
                            {
                                scale
                            } else {
                                1 as c_int as c_float
                            },
                        ),
                    );
                } else if spine_strcmp(timelineName, b"spacing\0" as *const u8 as *const c_char)
                    == 0 as c_int
                {
                    let mut timeline_16: *mut spCurveTimeline1 =
                        &mut (*(spPathConstraintSpacingTimeline_create
                            as unsafe extern "C" fn(
                                c_int,
                                c_int,
                                c_int,
                            )
                                -> *mut spPathConstraintSpacingTimeline)(
                            frames_1,
                            frames_1,
                            constraintIndex_1,
                        ))
                        .super_0;
                    spTimelineArray_add(
                        timelines,
                        readTimelineJson(
                            keyMap,
                            timeline_16,
                            0 as c_int as c_float,
                            if (*constraint_1).spacingMode as c_uint
                                == SP_SPACING_MODE_LENGTH as c_int as c_uint
                                || (*constraint_1).spacingMode as c_uint
                                    == SP_SPACING_MODE_FIXED as c_int as c_uint
                            {
                                scale
                            } else {
                                1 as c_int as c_float
                            },
                        ),
                    );
                } else if spine_strcmp(timelineName, b"mix\0" as *const u8 as *const c_char)
                    == 0 as c_int
                {
                    let mut timeline_17: *mut spPathConstraintMixTimeline =
                        spPathConstraintMixTimeline_create(
                            frames_1,
                            frames_1 * 3 as c_int,
                            constraintIndex_1,
                        );
                    let mut time_5: c_float = Json_getFloat(
                        keyMap,
                        b"time\0" as *const u8 as *const c_char,
                        0 as c_int as c_float,
                    );
                    let mut mixRotate_0: c_float = Json_getFloat(
                        keyMap,
                        b"mixRotate\0" as *const u8 as *const c_char,
                        1 as c_int as c_float,
                    );
                    let mut mixX_0: c_float = Json_getFloat(
                        keyMap,
                        b"mixX\0" as *const u8 as *const c_char,
                        1 as c_int as c_float,
                    );
                    let mut mixY_0: c_float =
                        Json_getFloat(keyMap, b"mixY\0" as *const u8 as *const c_char, mixX_0);
                    frame = 0 as c_int;
                    bezier = 0 as c_int;
                    loop {
                        let mut time2_5: c_float = 0.;
                        let mut mixRotate2_0: c_float = 0.;
                        let mut mixX2_0: c_float = 0.;
                        let mut mixY2_0: c_float = 0.;
                        spPathConstraintMixTimeline_setFrame(
                            timeline_17,
                            frame,
                            time_5,
                            mixRotate_0,
                            mixX_0,
                            mixY_0,
                        );
                        nextMap = (*keyMap).next;
                        if nextMap.is_null() {
                            break;
                        }
                        time2_5 = Json_getFloat(
                            nextMap,
                            b"time\0" as *const u8 as *const c_char,
                            0 as c_int as c_float,
                        );
                        mixRotate2_0 = Json_getFloat(
                            nextMap,
                            b"mixRotate\0" as *const u8 as *const c_char,
                            1 as c_int as c_float,
                        );
                        mixX2_0 = Json_getFloat(
                            nextMap,
                            b"mixX\0" as *const u8 as *const c_char,
                            1 as c_int as c_float,
                        );
                        mixY2_0 = Json_getFloat(
                            nextMap,
                            b"mixY\0" as *const u8 as *const c_char,
                            mixX2_0,
                        );
                        curve = Json_getItem(keyMap, b"curve\0" as *const u8 as *const c_char);
                        if !curve.is_null() {
                            bezier = readCurve(
                                curve,
                                &mut (*timeline_17).super_0,
                                bezier,
                                frame,
                                0 as c_int,
                                time_5,
                                time2_5,
                                mixRotate_0,
                                mixRotate2_0,
                                1 as c_int as c_float,
                            );
                            bezier = readCurve(
                                curve,
                                &mut (*timeline_17).super_0,
                                bezier,
                                frame,
                                1 as c_int,
                                time_5,
                                time2_5,
                                mixX_0,
                                mixX2_0,
                                1 as c_int as c_float,
                            );
                            bezier = readCurve(
                                curve,
                                &mut (*timeline_17).super_0,
                                bezier,
                                frame,
                                2 as c_int,
                                time_5,
                                time2_5,
                                mixY_0,
                                mixY2_0,
                                1 as c_int as c_float,
                            );
                        }
                        time_5 = time2_5;
                        mixRotate_0 = mixRotate2_0;
                        mixX_0 = mixX2_0;
                        mixY_0 = mixY2_0;
                        keyMap = nextMap;
                        frame += 1;
                    }
                    spTimelineArray_add(timelines, &mut (*timeline_17).super_0.super_0);
                }
            }
            timelineMap = (*timelineMap).next;
        }
        constraintMap = (*constraintMap).next;
    }
    attachmentsMap = if !attachmentsJson.is_null() {
        (*attachmentsJson).child
    } else {
        0 as *mut Json
    };
    while !attachmentsMap.is_null() {
        let mut skin: *mut spSkin = spSkeletonData_findSkin(skeletonData, (*attachmentsMap).name);
        slotMap = (*attachmentsMap).child;
        while !slotMap.is_null() {
            let mut attachmentMap: *mut Json = 0 as *mut Json;
            let mut slotIndex_0: c_int =
                findSlotIndex(self_0, skeletonData, (*slotMap).name, timelines);
            if slotIndex_0 == -(1 as c_int) {
                return 0 as *mut spAnimation;
            }
            attachmentMap = (*slotMap).child;
            while !attachmentMap.is_null() {
                let mut baseAttachment: *mut spAttachment =
                    spSkin_getAttachment(skin, slotIndex_0, (*attachmentMap).name);
                if baseAttachment.is_null() {
                    cleanUpTimelines(timelines);
                    _spSkeletonJson_setError(
                        self_0,
                        0 as *mut Json,
                        b"Attachment not found: \0" as *const u8 as *const c_char,
                        (*attachmentMap).name,
                    );
                    return 0 as *mut spAnimation;
                }
                timelineMap = (*attachmentMap).child;
                while !timelineMap.is_null() {
                    let mut frames_2: c_int = 0;
                    let mut timelineName_0: *const c_char = 0 as *const c_char;
                    keyMap = (*timelineMap).child;
                    if !keyMap.is_null() {
                        frames_2 = (*timelineMap).size;
                        timelineName_0 = (*timelineMap).name;
                        if spine_strcmp(b"deform\0" as *const u8 as *const c_char, timelineName_0)
                            == 0
                        {
                            let mut tempDeform: *mut c_float = 0 as *mut c_float;
                            let mut vertexAttachment: *mut spVertexAttachment =
                                0 as *mut spVertexAttachment;
                            let mut weighted: c_int = 0;
                            let mut deformLength: c_int = 0;
                            let mut timeline_18: *mut spDeformTimeline = 0 as *mut spDeformTimeline;
                            let mut time_6: c_float = 0.;
                            vertexAttachment = baseAttachment as *mut spVertexAttachment;
                            weighted = ((*vertexAttachment).bones != 0 as *mut c_int) as c_int;
                            deformLength = if weighted != 0 {
                                (*vertexAttachment).verticesCount / 3 as c_int * 2 as c_int
                            } else {
                                (*vertexAttachment).verticesCount
                            };
                            tempDeform = _spMalloc(
                                (::core::mem::size_of::<c_float>() as c_ulong)
                                    .wrapping_mul(deformLength as c_ulong),
                                b"spine.c\0" as *const u8 as *const c_char,
                                11315 as c_int,
                            ) as *mut c_float;
                            timeline_18 = spDeformTimeline_create(
                                (*timelineMap).size,
                                deformLength,
                                (*timelineMap).size,
                                slotIndex_0,
                                vertexAttachment,
                            );
                            time_6 = Json_getFloat(
                                keyMap,
                                b"time\0" as *const u8 as *const c_char,
                                0 as c_int as c_float,
                            );
                            frame = 0 as c_int;
                            bezier = 0 as c_int;
                            loop {
                                let mut vertices: *mut Json = Json_getItem(
                                    keyMap,
                                    b"vertices\0" as *const u8 as *const c_char,
                                );
                                let mut deform: *mut c_float = 0 as *mut c_float;
                                let mut time2_6: c_float = 0.;
                                if vertices.is_null() {
                                    if weighted != 0 {
                                        deform = tempDeform;
                                        spine_memset(
                                            deform as *mut c_void,
                                            0 as c_int,
                                            (::core::mem::size_of::<c_float>() as c_ulong)
                                                .wrapping_mul(deformLength as c_ulong),
                                        );
                                    } else {
                                        deform = (*vertexAttachment).vertices;
                                    }
                                } else {
                                    let mut v: c_int = 0;
                                    let mut start: c_int = Json_getInt(
                                        keyMap,
                                        b"offset\0" as *const u8 as *const c_char,
                                        0 as c_int,
                                    );
                                    let mut vertex: *mut Json = 0 as *mut Json;
                                    deform = tempDeform;
                                    spine_memset(
                                        deform as *mut c_void,
                                        0 as c_int,
                                        (::core::mem::size_of::<c_float>() as c_ulong)
                                            .wrapping_mul(start as c_ulong),
                                    );
                                    if (*self_0).scale == 1 as c_int as c_float {
                                        vertex = (*vertices).child;
                                        v = start;
                                        while !vertex.is_null() {
                                            *deform.offset(v as isize) = (*vertex).valueFloat;
                                            vertex = (*vertex).next;
                                            v += 1;
                                        }
                                    } else {
                                        vertex = (*vertices).child;
                                        v = start;
                                        while !vertex.is_null() {
                                            *deform.offset(v as isize) =
                                                (*vertex).valueFloat * (*self_0).scale;
                                            vertex = (*vertex).next;
                                            v += 1;
                                        }
                                    }
                                    spine_memset(
                                        deform.offset(v as isize) as *mut c_void,
                                        0 as c_int,
                                        (::core::mem::size_of::<c_float>() as c_ulong)
                                            .wrapping_mul((deformLength - v) as c_ulong),
                                    );
                                    if weighted == 0 {
                                        let mut verticesValues: *mut c_float =
                                            (*vertexAttachment).vertices;
                                        v = 0 as c_int;
                                        while v < deformLength {
                                            *deform.offset(v as isize) +=
                                                *verticesValues.offset(v as isize);
                                            v += 1;
                                        }
                                    }
                                }
                                spDeformTimeline_setFrame(timeline_18, frame, time_6, deform);
                                nextMap = (*keyMap).next;
                                if nextMap.is_null() {
                                    break;
                                }
                                time2_6 = Json_getFloat(
                                    nextMap,
                                    b"time\0" as *const u8 as *const c_char,
                                    0 as c_int as c_float,
                                );
                                curve =
                                    Json_getItem(keyMap, b"curve\0" as *const u8 as *const c_char);
                                if !curve.is_null() {
                                    bezier = readCurve(
                                        curve,
                                        &mut (*timeline_18).super_0,
                                        bezier,
                                        frame,
                                        0 as c_int,
                                        time_6,
                                        time2_6,
                                        0 as c_int as c_float,
                                        1 as c_int as c_float,
                                        1 as c_int as c_float,
                                    );
                                }
                                time_6 = time2_6;
                                keyMap = nextMap;
                                frame += 1;
                            }
                            _spFree(tempDeform as *mut c_void);
                            spTimelineArray_add(timelines, &mut (*timeline_18).super_0.super_0);
                        } else if spine_strcmp(
                            timelineName_0,
                            b"sequence\0" as *const u8 as *const c_char,
                        ) == 0
                        {
                            let mut timeline_19: *mut spSequenceTimeline =
                                spSequenceTimeline_create(frames_2, slotIndex_0, baseAttachment);
                            let mut lastDelay: c_float = 0 as c_int as c_float;
                            frame = 0 as c_int;
                            while !keyMap.is_null() {
                                let mut delay: c_float = Json_getFloat(
                                    keyMap,
                                    b"delay\0" as *const u8 as *const c_char,
                                    lastDelay,
                                );
                                let mut time_7: c_float = Json_getFloat(
                                    keyMap,
                                    b"time\0" as *const u8 as *const c_char,
                                    0 as c_int as c_float,
                                );
                                let mut modeString: *const c_char = Json_getString(
                                    keyMap,
                                    b"mode\0" as *const u8 as *const c_char,
                                    b"hold\0" as *const u8 as *const c_char,
                                );
                                let mut index: c_int = Json_getInt(
                                    keyMap,
                                    b"index\0" as *const u8 as *const c_char,
                                    0 as c_int,
                                );
                                let mut mode: c_int = 0 as c_int;
                                if spine_strcmp(modeString, b"once\0" as *const u8 as *const c_char)
                                    == 0
                                {
                                    mode = 1 as c_int;
                                }
                                if spine_strcmp(modeString, b"loop\0" as *const u8 as *const c_char)
                                    == 0
                                {
                                    mode = 2 as c_int;
                                }
                                if spine_strcmp(
                                    modeString,
                                    b"pingpong\0" as *const u8 as *const c_char,
                                ) == 0
                                {
                                    mode = 3 as c_int;
                                }
                                if spine_strcmp(
                                    modeString,
                                    b"onceReverse\0" as *const u8 as *const c_char,
                                ) == 0
                                {
                                    mode = 4 as c_int;
                                }
                                if spine_strcmp(
                                    modeString,
                                    b"loopReverse\0" as *const u8 as *const c_char,
                                ) == 0
                                {
                                    mode = 5 as c_int;
                                }
                                if spine_strcmp(
                                    modeString,
                                    b"pingpongReverse\0" as *const u8 as *const c_char,
                                ) == 0
                                {
                                    mode = 6 as c_int;
                                }
                                spSequenceTimeline_setFrame(
                                    timeline_19,
                                    frame,
                                    time_7,
                                    mode,
                                    index,
                                    delay,
                                );
                                lastDelay = delay;
                                keyMap = (*keyMap).next;
                                frame += 1;
                            }
                            spTimelineArray_add(timelines, &mut (*timeline_19).super_0);
                        }
                    }
                    timelineMap = (*timelineMap).next;
                }
                attachmentMap = (*attachmentMap).next;
            }
            slotMap = (*slotMap).next;
        }
        attachmentsMap = (*attachmentsMap).next;
    }
    if !drawOrderJson.is_null() {
        let mut timeline_20: *mut spDrawOrderTimeline =
            spDrawOrderTimeline_create((*drawOrderJson).size, (*skeletonData).slotsCount);
        keyMap = (*drawOrderJson).child;
        frame = 0 as c_int;
        while !keyMap.is_null() {
            let mut ii: c_int = 0;
            let mut drawOrder: *mut c_int = 0 as *mut c_int;
            let mut offsets: *mut Json =
                Json_getItem(keyMap, b"offsets\0" as *const u8 as *const c_char);
            if !offsets.is_null() {
                let mut offsetMap: *mut Json = 0 as *mut Json;
                let mut unchanged: *mut c_int = _spMalloc(
                    (::core::mem::size_of::<c_int>() as c_ulong)
                        .wrapping_mul(((*skeletonData).slotsCount - (*offsets).size) as c_ulong),
                    b"spine.c\0" as *const u8 as *const c_char,
                    11403 as c_int,
                ) as *mut c_int;
                let mut originalIndex: c_int = 0 as c_int;
                let mut unchangedIndex: c_int = 0 as c_int;
                drawOrder = _spMalloc(
                    (::core::mem::size_of::<c_int>() as c_ulong)
                        .wrapping_mul((*skeletonData).slotsCount as c_ulong),
                    b"spine.c\0" as *const u8 as *const c_char,
                    11406 as c_int,
                ) as *mut c_int;
                ii = (*skeletonData).slotsCount - 1 as c_int;
                while ii >= 0 as c_int {
                    *drawOrder.offset(ii as isize) = -(1 as c_int);
                    ii -= 1;
                }
                offsetMap = (*offsets).child;
                while !offsetMap.is_null() {
                    let mut slotIndex_1: c_int = findSlotIndex(
                        self_0,
                        skeletonData,
                        Json_getString(
                            offsetMap,
                            b"slot\0" as *const u8 as *const c_char,
                            0 as *const c_char,
                        ),
                        timelines,
                    );
                    if slotIndex_1 == -(1 as c_int) {
                        return 0 as *mut spAnimation;
                    }
                    while originalIndex != slotIndex_1 {
                        let fresh175 = originalIndex;
                        originalIndex = originalIndex + 1;
                        let fresh176 = unchangedIndex;
                        unchangedIndex = unchangedIndex + 1;
                        *unchanged.offset(fresh176 as isize) = fresh175;
                    }
                    *drawOrder.offset(
                        (originalIndex
                            + Json_getInt(
                                offsetMap,
                                b"offset\0" as *const u8 as *const c_char,
                                0 as c_int,
                            )) as isize,
                    ) = originalIndex;
                    originalIndex += 1;
                    offsetMap = (*offsetMap).next;
                }
                while originalIndex < (*skeletonData).slotsCount {
                    let fresh177 = originalIndex;
                    originalIndex = originalIndex + 1;
                    let fresh178 = unchangedIndex;
                    unchangedIndex = unchangedIndex + 1;
                    *unchanged.offset(fresh178 as isize) = fresh177;
                }
                ii = (*skeletonData).slotsCount - 1 as c_int;
                while ii >= 0 as c_int {
                    if *drawOrder.offset(ii as isize) == -(1 as c_int) {
                        unchangedIndex -= 1;
                        *drawOrder.offset(ii as isize) = *unchanged.offset(unchangedIndex as isize);
                    }
                    ii -= 1;
                }
                _spFree(unchanged as *mut c_void);
            }
            spDrawOrderTimeline_setFrame(
                timeline_20,
                frame,
                Json_getFloat(
                    keyMap,
                    b"time\0" as *const u8 as *const c_char,
                    0 as c_int as c_float,
                ),
                drawOrder,
            );
            _spFree(drawOrder as *mut c_void);
            keyMap = (*keyMap).next;
            frame += 1;
        }
        spTimelineArray_add(timelines, &mut (*timeline_20).super_0);
    }
    if !events.is_null() {
        let mut timeline_21: *mut spEventTimeline = spEventTimeline_create((*events).size);
        keyMap = (*events).child;
        frame = 0 as c_int;
        while !keyMap.is_null() {
            let mut event: *mut spEvent = 0 as *mut spEvent;
            let mut stringValue: *const c_char = 0 as *const c_char;
            let mut eventData: *mut spEventData = spSkeletonData_findEvent(
                skeletonData,
                Json_getString(
                    keyMap,
                    b"name\0" as *const u8 as *const c_char,
                    0 as *const c_char,
                ),
            );
            if eventData.is_null() {
                cleanUpTimelines(timelines);
                _spSkeletonJson_setError(
                    self_0,
                    0 as *mut Json,
                    b"Event not found: \0" as *const u8 as *const c_char,
                    Json_getString(
                        keyMap,
                        b"name\0" as *const u8 as *const c_char,
                        0 as *const c_char,
                    ),
                );
                return 0 as *mut spAnimation;
            }
            event = spEvent_create(
                Json_getFloat(
                    keyMap,
                    b"time\0" as *const u8 as *const c_char,
                    0 as c_int as c_float,
                ),
                eventData,
            );
            (*event).intValue = Json_getInt(
                keyMap,
                b"int\0" as *const u8 as *const c_char,
                (*eventData).intValue,
            );
            (*event).floatValue = Json_getFloat(
                keyMap,
                b"float\0" as *const u8 as *const c_char,
                (*eventData).floatValue,
            );
            stringValue = Json_getString(
                keyMap,
                b"string\0" as *const u8 as *const c_char,
                (*eventData).stringValue,
            );
            if !stringValue.is_null() {
                let ref mut fresh179 =
                    *(&mut (*event).stringValue as *mut *const c_char as *mut *mut c_char);
                *fresh179 = _spMalloc(
                    (::core::mem::size_of::<c_char>() as c_ulong).wrapping_mul(
                        (spine_strlen(stringValue)).wrapping_add(1 as c_int as c_ulong),
                    ),
                    b"spine.c\0" as *const u8 as *const c_char,
                    11452 as c_int,
                ) as *mut c_char;
                spine_strcpy(*fresh179, stringValue);
            }
            if !((*eventData).audioPath).is_null() {
                (*event).volume = Json_getFloat(
                    keyMap,
                    b"volume\0" as *const u8 as *const c_char,
                    1 as c_int as c_float,
                );
                (*event).balance = Json_getFloat(
                    keyMap,
                    b"volume\0" as *const u8 as *const c_char,
                    0 as c_int as c_float,
                );
            }
            spEventTimeline_setFrame(timeline_21, frame, event);
            keyMap = (*keyMap).next;
            frame += 1;
        }
        spTimelineArray_add(timelines, &mut (*timeline_21).super_0);
    }
    duration = 0 as c_int as c_float;
    i = 0 as c_int;
    n = (*timelines).size;
    while i < n {
        duration = if duration > spTimeline_getDuration(*((*timelines).items).offset(i as isize)) {
            duration
        } else {
            spTimeline_getDuration(*((*timelines).items).offset(i as isize))
        };
        i += 1;
    }
    return spAnimation_create((*root).name, timelines, duration);
}
unsafe extern "C" fn _readVerticesJson(
    mut self_0: *mut spSkeletonJson,
    mut attachmentMap: *mut Json,
    mut attachment: *mut spVertexAttachment,
    mut verticesLength: c_int,
) {
    let mut entry: *mut Json = 0 as *mut Json;
    let mut vertices: *mut c_float = 0 as *mut c_float;
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    let mut nn: c_int = 0;
    let mut entrySize: c_int = 0;
    let mut weights: *mut spFloatArray = 0 as *mut spFloatArray;
    let mut bones: *mut spIntArray = 0 as *mut spIntArray;
    (*attachment).worldVerticesLength = verticesLength;
    entry = Json_getItem(attachmentMap, b"vertices\0" as *const u8 as *const c_char);
    entrySize = (*entry).size;
    vertices = _spMalloc(
        (::core::mem::size_of::<c_float>() as c_ulong).wrapping_mul(entrySize as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        11480 as c_int,
    ) as *mut c_float;
    entry = (*entry).child;
    i = 0 as c_int;
    while !entry.is_null() {
        *vertices.offset(i as isize) = (*entry).valueFloat;
        entry = (*entry).next;
        i += 1;
    }
    if verticesLength == entrySize {
        if (*self_0).scale != 1 as c_int as c_float {
            i = 0 as c_int;
            while i < entrySize {
                *vertices.offset(i as isize) *= (*self_0).scale;
                i += 1;
            }
        }
        (*attachment).verticesCount = verticesLength;
        (*attachment).vertices = vertices;
        (*attachment).bonesCount = 0 as c_int;
        (*attachment).bones = 0 as *mut c_int;
        return;
    }
    weights = spFloatArray_create(verticesLength * 3 as c_int * 3 as c_int);
    bones = spIntArray_create(verticesLength * 3 as c_int);
    i = 0 as c_int;
    n = entrySize;
    while i < n {
        let fresh180 = i;
        i = i + 1;
        let mut boneCount: c_int = *vertices.offset(fresh180 as isize) as c_int;
        spIntArray_add(bones, boneCount);
        nn = i + boneCount * 4 as c_int;
        while i < nn {
            spIntArray_add(bones, *vertices.offset(i as isize) as c_int);
            spFloatArray_add(
                weights,
                *vertices.offset((i + 1 as c_int) as isize) * (*self_0).scale,
            );
            spFloatArray_add(
                weights,
                *vertices.offset((i + 2 as c_int) as isize) * (*self_0).scale,
            );
            spFloatArray_add(weights, *vertices.offset((i + 3 as c_int) as isize));
            i += 4 as c_int;
        }
    }
    (*attachment).verticesCount = (*weights).size;
    (*attachment).vertices = (*weights).items;
    _spFree(weights as *mut c_void);
    (*attachment).bonesCount = (*bones).size;
    (*attachment).bones = (*bones).items;
    _spFree(bones as *mut c_void);
    _spFree(vertices as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonJson_readSkeletonDataFile(
    mut self_0: *mut spSkeletonJson,
    mut path: *const c_char,
) -> *mut spSkeletonData {
    let mut length: c_int = 0;
    let mut skeletonData: *mut spSkeletonData = 0 as *mut spSkeletonData;
    let mut json: *const c_char = _spUtil_readFile(path, &mut length);
    if length == 0 as c_int || json.is_null() {
        _spSkeletonJson_setError(
            self_0,
            0 as *mut Json,
            b"Unable to read skeleton file: \0" as *const u8 as *const c_char,
            path,
        );
        return 0 as *mut spSkeletonData;
    }
    skeletonData = spSkeletonJson_readSkeletonData(self_0, json);
    _spFree(json as *mut c_void);
    return skeletonData;
}
unsafe extern "C" fn string_starts_with_json(
    mut str: *const c_char,
    mut needle: *const c_char,
) -> c_int {
    let mut lenStr: c_int = 0;
    let mut lenNeedle: c_int = 0;
    let mut i: c_int = 0;
    if str.is_null() {
        return 0 as c_int;
    }
    lenStr = spine_strlen(str) as c_int;
    lenNeedle = spine_strlen(needle) as c_int;
    if lenStr < lenNeedle {
        return 0 as c_int;
    }
    i = 0 as c_int;
    while i < lenNeedle {
        if *str.offset(i as isize) as c_int != *needle.offset(i as isize) as c_int {
            return 0 as c_int;
        }
        i += 1;
    }
    return -(1 as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonJson_readSkeletonData(
    mut self_0: *mut spSkeletonJson,
    mut json: *const c_char,
) -> *mut spSkeletonData {
    let mut i: c_int = 0;
    let mut ii: c_int = 0;
    let mut skeletonData: *mut spSkeletonData = 0 as *mut spSkeletonData;
    let mut root: *mut Json = 0 as *mut Json;
    let mut skeleton: *mut Json = 0 as *mut Json;
    let mut bones: *mut Json = 0 as *mut Json;
    let mut boneMap: *mut Json = 0 as *mut Json;
    let mut ik: *mut Json = 0 as *mut Json;
    let mut transform: *mut Json = 0 as *mut Json;
    let mut pathJson: *mut Json = 0 as *mut Json;
    let mut slots: *mut Json = 0 as *mut Json;
    let mut skins: *mut Json = 0 as *mut Json;
    let mut animations: *mut Json = 0 as *mut Json;
    let mut events: *mut Json = 0 as *mut Json;
    let mut internal: *mut _spSkeletonJson = self_0 as *mut _spSkeletonJson;
    _spFree((*self_0).error as *mut c_void);
    let ref mut fresh181 = *(&(*self_0).error as *const *const c_char as *mut *mut c_char);
    *fresh181 = 0 as *mut c_char;
    (*internal).linkedMeshCount = 0 as c_int;
    root = Json_create(json);
    if root.is_null() {
        _spSkeletonJson_setError(
            self_0,
            0 as *mut Json,
            b"Invalid skeleton JSON: \0" as *const u8 as *const c_char,
            Json_getError(),
        );
        return 0 as *mut spSkeletonData;
    }
    skeletonData = spSkeletonData_create();
    skeleton = Json_getItem(root, b"skeleton\0" as *const u8 as *const c_char);
    if !skeleton.is_null() {
        let ref mut fresh182 =
            *(&mut (*skeletonData).hash as *mut *const c_char as *mut *mut c_char);
        *fresh182 = _spMalloc(
            (::core::mem::size_of::<c_char>() as c_ulong).wrapping_mul(
                (spine_strlen(Json_getString(
                    skeleton,
                    b"hash\0" as *const u8 as *const c_char,
                    0 as *const c_char,
                )))
                .wrapping_add(1 as c_int as c_ulong),
            ),
            b"spine.c\0" as *const u8 as *const c_char,
            11565 as c_int,
        ) as *mut c_char;
        spine_strcpy(
            *fresh182,
            Json_getString(
                skeleton,
                b"hash\0" as *const u8 as *const c_char,
                0 as *const c_char,
            ),
        );
        let ref mut fresh183 =
            *(&mut (*skeletonData).version as *mut *const c_char as *mut *mut c_char);
        *fresh183 = _spMalloc(
            (::core::mem::size_of::<c_char>() as c_ulong).wrapping_mul(
                (spine_strlen(Json_getString(
                    skeleton,
                    b"spine\0" as *const u8 as *const c_char,
                    0 as *const c_char,
                )))
                .wrapping_add(1 as c_int as c_ulong),
            ),
            b"spine.c\0" as *const u8 as *const c_char,
            11566 as c_int,
        ) as *mut c_char;
        spine_strcpy(
            *fresh183,
            Json_getString(
                skeleton,
                b"spine\0" as *const u8 as *const c_char,
                0 as *const c_char,
            ),
        );
        if string_starts_with_json(
            (*skeletonData).version,
            b"4.1\0" as *const u8 as *const c_char,
        ) == 0
        {
            let mut errorMsg: [c_char; 255] = [0; 255];
            spine_sprintf!(
                errorMsg.as_mut_ptr(),
                b"Skeleton version %s does not match runtime version %s\0" as *const u8
                    as *const c_char,
                (*skeletonData).version,
                b"4.1\0" as *const u8 as *const c_char,
            );
            _spSkeletonJson_setError(
                self_0,
                0 as *mut Json,
                errorMsg.as_mut_ptr(),
                0 as *const c_char,
            );
            return 0 as *mut spSkeletonData;
        }
        (*skeletonData).x = Json_getFloat(
            skeleton,
            b"x\0" as *const u8 as *const c_char,
            0 as c_int as c_float,
        );
        (*skeletonData).y = Json_getFloat(
            skeleton,
            b"y\0" as *const u8 as *const c_char,
            0 as c_int as c_float,
        );
        (*skeletonData).width = Json_getFloat(
            skeleton,
            b"width\0" as *const u8 as *const c_char,
            0 as c_int as c_float,
        );
        (*skeletonData).height = Json_getFloat(
            skeleton,
            b"height\0" as *const u8 as *const c_char,
            0 as c_int as c_float,
        );
        (*skeletonData).fps = Json_getFloat(
            skeleton,
            b"fps\0" as *const u8 as *const c_char,
            30 as c_int as c_float,
        );
        (*skeletonData).imagesPath = Json_getString(
            skeleton,
            b"images\0" as *const u8 as *const c_char,
            0 as *const c_char,
        );
        if !((*skeletonData).imagesPath).is_null() {
            let mut tmp: *mut c_char = 0 as *mut c_char;
            let ref mut fresh184 = *(&mut tmp as *mut *mut c_char);
            *fresh184 = _spMalloc(
                (::core::mem::size_of::<c_char>() as c_ulong).wrapping_mul(
                    (spine_strlen((*skeletonData).imagesPath)).wrapping_add(1 as c_int as c_ulong),
                ),
                b"spine.c\0" as *const u8 as *const c_char,
                11581 as c_int,
            ) as *mut c_char;
            spine_strcpy(*fresh184, (*skeletonData).imagesPath);
            (*skeletonData).imagesPath = tmp;
        }
        (*skeletonData).audioPath = Json_getString(
            skeleton,
            b"audio\0" as *const u8 as *const c_char,
            0 as *const c_char,
        );
        if !((*skeletonData).audioPath).is_null() {
            let mut tmp_0: *mut c_char = 0 as *mut c_char;
            let ref mut fresh185 = *(&mut tmp_0 as *mut *mut c_char);
            *fresh185 = _spMalloc(
                (::core::mem::size_of::<c_char>() as c_ulong).wrapping_mul(
                    (spine_strlen((*skeletonData).audioPath)).wrapping_add(1 as c_int as c_ulong),
                ),
                b"spine.c\0" as *const u8 as *const c_char,
                11587 as c_int,
            ) as *mut c_char;
            spine_strcpy(*fresh185, (*skeletonData).audioPath);
            (*skeletonData).audioPath = tmp_0;
        }
    }
    bones = Json_getItem(root, b"bones\0" as *const u8 as *const c_char);
    (*skeletonData).bones = _spMalloc(
        (::core::mem::size_of::<*mut spBoneData>() as c_ulong)
            .wrapping_mul((*bones).size as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        11594 as c_int,
    ) as *mut *mut spBoneData;
    boneMap = (*bones).child;
    i = 0 as c_int;
    while !boneMap.is_null() {
        let mut data: *mut spBoneData = 0 as *mut spBoneData;
        let mut transformMode: *const c_char = 0 as *const c_char;
        let mut color: *const c_char = 0 as *const c_char;
        let mut parent: *mut spBoneData = 0 as *mut spBoneData;
        let mut parentName: *const c_char = Json_getString(
            boneMap,
            b"parent\0" as *const u8 as *const c_char,
            0 as *const c_char,
        );
        if !parentName.is_null() {
            parent = spSkeletonData_findBone(skeletonData, parentName);
            if parent.is_null() {
                spSkeletonData_dispose(skeletonData);
                _spSkeletonJson_setError(
                    self_0,
                    root,
                    b"Parent bone not found: \0" as *const u8 as *const c_char,
                    parentName,
                );
                return 0 as *mut spSkeletonData;
            }
        }
        data = spBoneData_create(
            (*skeletonData).bonesCount,
            Json_getString(
                boneMap,
                b"name\0" as *const u8 as *const c_char,
                0 as *const c_char,
            ),
            parent,
        );
        (*data).length = Json_getFloat(
            boneMap,
            b"length\0" as *const u8 as *const c_char,
            0 as c_int as c_float,
        ) * (*self_0).scale;
        (*data).x = Json_getFloat(
            boneMap,
            b"x\0" as *const u8 as *const c_char,
            0 as c_int as c_float,
        ) * (*self_0).scale;
        (*data).y = Json_getFloat(
            boneMap,
            b"y\0" as *const u8 as *const c_char,
            0 as c_int as c_float,
        ) * (*self_0).scale;
        (*data).rotation = Json_getFloat(
            boneMap,
            b"rotation\0" as *const u8 as *const c_char,
            0 as c_int as c_float,
        );
        (*data).scaleX = Json_getFloat(
            boneMap,
            b"scaleX\0" as *const u8 as *const c_char,
            1 as c_int as c_float,
        );
        (*data).scaleY = Json_getFloat(
            boneMap,
            b"scaleY\0" as *const u8 as *const c_char,
            1 as c_int as c_float,
        );
        (*data).shearX = Json_getFloat(
            boneMap,
            b"shearX\0" as *const u8 as *const c_char,
            0 as c_int as c_float,
        );
        (*data).shearY = Json_getFloat(
            boneMap,
            b"shearY\0" as *const u8 as *const c_char,
            0 as c_int as c_float,
        );
        transformMode = Json_getString(
            boneMap,
            b"transform\0" as *const u8 as *const c_char,
            b"normal\0" as *const u8 as *const c_char,
        );
        (*data).transformMode = SP_TRANSFORMMODE_NORMAL;
        if spine_strcmp(transformMode, b"normal\0" as *const u8 as *const c_char) == 0 as c_int {
            (*data).transformMode = SP_TRANSFORMMODE_NORMAL;
        } else if spine_strcmp(
            transformMode,
            b"onlyTranslation\0" as *const u8 as *const c_char,
        ) == 0 as c_int
        {
            (*data).transformMode = SP_TRANSFORMMODE_ONLYTRANSLATION;
        } else if spine_strcmp(
            transformMode,
            b"noRotationOrReflection\0" as *const u8 as *const c_char,
        ) == 0 as c_int
        {
            (*data).transformMode = SP_TRANSFORMMODE_NOROTATIONORREFLECTION;
        } else if spine_strcmp(transformMode, b"noScale\0" as *const u8 as *const c_char)
            == 0 as c_int
        {
            (*data).transformMode = SP_TRANSFORMMODE_NOSCALE;
        } else if spine_strcmp(
            transformMode,
            b"noScaleOrReflection\0" as *const u8 as *const c_char,
        ) == 0 as c_int
        {
            (*data).transformMode = SP_TRANSFORMMODE_NOSCALEORREFLECTION;
        }
        (*data).skinRequired =
            if Json_getInt(boneMap, b"skin\0" as *const u8 as *const c_char, 0 as c_int) != 0 {
                1 as c_int
            } else {
                0 as c_int
            };
        color = Json_getString(
            boneMap,
            b"color\0" as *const u8 as *const c_char,
            0 as *const c_char,
        );
        if !color.is_null() {
            toColor2(&mut (*data).color, color, -(1 as c_int));
        }
        let ref mut fresh186 = *((*skeletonData).bones).offset(i as isize);
        *fresh186 = data;
        (*skeletonData).bonesCount += 1;
        boneMap = (*boneMap).next;
        i += 1;
    }
    slots = Json_getItem(root, b"slots\0" as *const u8 as *const c_char);
    if !slots.is_null() {
        let mut slotMap: *mut Json = 0 as *mut Json;
        (*skeletonData).slotsCount = (*slots).size;
        (*skeletonData).slots = _spMalloc(
            (::core::mem::size_of::<*mut spSlotData>() as c_ulong)
                .wrapping_mul((*slots).size as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            11645 as c_int,
        ) as *mut *mut spSlotData;
        slotMap = (*slots).child;
        i = 0 as c_int;
        while !slotMap.is_null() {
            let mut data_0: *mut spSlotData = 0 as *mut spSlotData;
            let mut color_0: *const c_char = 0 as *const c_char;
            let mut dark: *const c_char = 0 as *const c_char;
            let mut item: *mut Json = 0 as *mut Json;
            let mut boneName: *const c_char = Json_getString(
                slotMap,
                b"bone\0" as *const u8 as *const c_char,
                0 as *const c_char,
            );
            let mut boneData: *mut spBoneData = spSkeletonData_findBone(skeletonData, boneName);
            if boneData.is_null() {
                spSkeletonData_dispose(skeletonData);
                _spSkeletonJson_setError(
                    self_0,
                    root,
                    b"Slot bone not found: \0" as *const u8 as *const c_char,
                    boneName,
                );
                return 0 as *mut spSkeletonData;
            }
            data_0 = spSlotData_create(
                i,
                Json_getString(
                    slotMap,
                    b"name\0" as *const u8 as *const c_char,
                    0 as *const c_char,
                ),
                boneData,
            );
            color_0 = Json_getString(
                slotMap,
                b"color\0" as *const u8 as *const c_char,
                0 as *const c_char,
            );
            if !color_0.is_null() {
                spColor_setFromFloats(
                    &mut (*data_0).color,
                    toColor(color_0, 0 as c_int),
                    toColor(color_0, 1 as c_int),
                    toColor(color_0, 2 as c_int),
                    toColor(color_0, 3 as c_int),
                );
            }
            dark = Json_getString(
                slotMap,
                b"dark\0" as *const u8 as *const c_char,
                0 as *const c_char,
            );
            if !dark.is_null() {
                (*data_0).darkColor = spColor_create();
                spColor_setFromFloats(
                    (*data_0).darkColor,
                    toColor(dark, 0 as c_int),
                    toColor(dark, 1 as c_int),
                    toColor(dark, 2 as c_int),
                    toColor(dark, 3 as c_int),
                );
            }
            item = Json_getItem(slotMap, b"attachment\0" as *const u8 as *const c_char);
            if !item.is_null() {
                spSlotData_setAttachmentName(data_0, (*item).valueString);
            }
            item = Json_getItem(slotMap, b"blend\0" as *const u8 as *const c_char);
            if !item.is_null() {
                if spine_strcmp(
                    (*item).valueString,
                    b"additive\0" as *const u8 as *const c_char,
                ) == 0 as c_int
                {
                    (*data_0).blendMode = SP_BLEND_MODE_ADDITIVE;
                } else if spine_strcmp(
                    (*item).valueString,
                    b"multiply\0" as *const u8 as *const c_char,
                ) == 0 as c_int
                {
                    (*data_0).blendMode = SP_BLEND_MODE_MULTIPLY;
                } else if spine_strcmp(
                    (*item).valueString,
                    b"screen\0" as *const u8 as *const c_char,
                ) == 0 as c_int
                {
                    (*data_0).blendMode = SP_BLEND_MODE_SCREEN;
                }
            }
            let ref mut fresh187 = *((*skeletonData).slots).offset(i as isize);
            *fresh187 = data_0;
            slotMap = (*slotMap).next;
            i += 1;
        }
    }
    ik = Json_getItem(root, b"ik\0" as *const u8 as *const c_char);
    if !ik.is_null() {
        let mut constraintMap: *mut Json = 0 as *mut Json;
        (*skeletonData).ikConstraintsCount = (*ik).size;
        (*skeletonData).ikConstraints = _spMalloc(
            (::core::mem::size_of::<*mut spIkConstraintData>() as c_ulong)
                .wrapping_mul((*ik).size as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            11703 as c_int,
        ) as *mut *mut spIkConstraintData;
        constraintMap = (*ik).child;
        i = 0 as c_int;
        while !constraintMap.is_null() {
            let mut targetName: *const c_char = 0 as *const c_char;
            let mut data_1: *mut spIkConstraintData = spIkConstraintData_create(Json_getString(
                constraintMap,
                b"name\0" as *const u8 as *const c_char,
                0 as *const c_char,
            ));
            (*data_1).order = Json_getInt(
                constraintMap,
                b"order\0" as *const u8 as *const c_char,
                0 as c_int,
            );
            (*data_1).skinRequired = if Json_getInt(
                constraintMap,
                b"skin\0" as *const u8 as *const c_char,
                0 as c_int,
            ) != 0
            {
                1 as c_int
            } else {
                0 as c_int
            };
            boneMap = Json_getItem(constraintMap, b"bones\0" as *const u8 as *const c_char);
            (*data_1).bonesCount = (*boneMap).size;
            (*data_1).bones = _spMalloc(
                (::core::mem::size_of::<*mut spBoneData>() as c_ulong)
                    .wrapping_mul((*boneMap).size as c_ulong),
                b"spine.c\0" as *const u8 as *const c_char,
                11713 as c_int,
            ) as *mut *mut spBoneData;
            boneMap = (*boneMap).child;
            ii = 0 as c_int;
            while !boneMap.is_null() {
                let ref mut fresh188 = *((*data_1).bones).offset(ii as isize);
                *fresh188 = spSkeletonData_findBone(skeletonData, (*boneMap).valueString);
                if (*((*data_1).bones).offset(ii as isize)).is_null() {
                    spSkeletonData_dispose(skeletonData);
                    _spSkeletonJson_setError(
                        self_0,
                        root,
                        b"IK bone not found: \0" as *const u8 as *const c_char,
                        (*boneMap).valueString,
                    );
                    return 0 as *mut spSkeletonData;
                }
                boneMap = (*boneMap).next;
                ii += 1;
            }
            targetName = Json_getString(
                constraintMap,
                b"target\0" as *const u8 as *const c_char,
                0 as *const c_char,
            );
            (*data_1).target = spSkeletonData_findBone(skeletonData, targetName);
            if ((*data_1).target).is_null() {
                spSkeletonData_dispose(skeletonData);
                _spSkeletonJson_setError(
                    self_0,
                    root,
                    b"Target bone not found: \0" as *const u8 as *const c_char,
                    targetName,
                );
                return 0 as *mut spSkeletonData;
            }
            (*data_1).bendDirection = if Json_getInt(
                constraintMap,
                b"bendPositive\0" as *const u8 as *const c_char,
                1 as c_int,
            ) != 0
            {
                1 as c_int
            } else {
                -(1 as c_int)
            };
            (*data_1).compress = if Json_getInt(
                constraintMap,
                b"compress\0" as *const u8 as *const c_char,
                0 as c_int,
            ) != 0
            {
                1 as c_int
            } else {
                0 as c_int
            };
            (*data_1).stretch = if Json_getInt(
                constraintMap,
                b"stretch\0" as *const u8 as *const c_char,
                0 as c_int,
            ) != 0
            {
                1 as c_int
            } else {
                0 as c_int
            };
            (*data_1).uniform = if Json_getInt(
                constraintMap,
                b"uniform\0" as *const u8 as *const c_char,
                0 as c_int,
            ) != 0
            {
                1 as c_int
            } else {
                0 as c_int
            };
            (*data_1).mix = Json_getFloat(
                constraintMap,
                b"mix\0" as *const u8 as *const c_char,
                1 as c_int as c_float,
            );
            (*data_1).softness = Json_getFloat(
                constraintMap,
                b"softness\0" as *const u8 as *const c_char,
                0 as c_int as c_float,
            ) * (*self_0).scale;
            let ref mut fresh189 = *((*skeletonData).ikConstraints).offset(i as isize);
            *fresh189 = data_1;
            constraintMap = (*constraintMap).next;
            i += 1;
        }
    }
    transform = Json_getItem(root, b"transform\0" as *const u8 as *const c_char);
    if !transform.is_null() {
        let mut constraintMap_0: *mut Json = 0 as *mut Json;
        (*skeletonData).transformConstraintsCount = (*transform).size;
        (*skeletonData).transformConstraints = _spMalloc(
            (::core::mem::size_of::<*mut spTransformConstraintData>() as c_ulong)
                .wrapping_mul((*transform).size as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            11747 as c_int,
        ) as *mut *mut spTransformConstraintData;
        constraintMap_0 = (*transform).child;
        i = 0 as c_int;
        while !constraintMap_0.is_null() {
            let mut name: *const c_char = 0 as *const c_char;
            let mut data_2: *mut spTransformConstraintData =
                spTransformConstraintData_create(Json_getString(
                    constraintMap_0,
                    b"name\0" as *const u8 as *const c_char,
                    0 as *const c_char,
                ));
            (*data_2).order = Json_getInt(
                constraintMap_0,
                b"order\0" as *const u8 as *const c_char,
                0 as c_int,
            );
            (*data_2).skinRequired = if Json_getInt(
                constraintMap_0,
                b"skin\0" as *const u8 as *const c_char,
                0 as c_int,
            ) != 0
            {
                1 as c_int
            } else {
                0 as c_int
            };
            boneMap = Json_getItem(constraintMap_0, b"bones\0" as *const u8 as *const c_char);
            (*data_2).bonesCount = (*boneMap).size;
            let ref mut fresh190 =
                *(&(*data_2).bones as *const *mut *mut spBoneData as *mut *mut *mut spBoneData);
            *fresh190 = _spMalloc(
                (::core::mem::size_of::<*mut spBoneData>() as c_ulong)
                    .wrapping_mul((*boneMap).size as c_ulong),
                b"spine.c\0" as *const u8 as *const c_char,
                11758 as c_int,
            ) as *mut *mut spBoneData;
            boneMap = (*boneMap).child;
            ii = 0 as c_int;
            while !boneMap.is_null() {
                let ref mut fresh191 = *((*data_2).bones).offset(ii as isize);
                *fresh191 = spSkeletonData_findBone(skeletonData, (*boneMap).valueString);
                if (*((*data_2).bones).offset(ii as isize)).is_null() {
                    spSkeletonData_dispose(skeletonData);
                    _spSkeletonJson_setError(
                        self_0,
                        root,
                        b"Transform bone not found: \0" as *const u8 as *const c_char,
                        (*boneMap).valueString,
                    );
                    return 0 as *mut spSkeletonData;
                }
                boneMap = (*boneMap).next;
                ii += 1;
            }
            name = Json_getString(
                constraintMap_0,
                b"target\0" as *const u8 as *const c_char,
                0 as *const c_char,
            );
            (*data_2).target = spSkeletonData_findBone(skeletonData, name);
            if ((*data_2).target).is_null() {
                spSkeletonData_dispose(skeletonData);
                _spSkeletonJson_setError(
                    self_0,
                    root,
                    b"Target bone not found: \0" as *const u8 as *const c_char,
                    name,
                );
                return 0 as *mut spSkeletonData;
            }
            (*data_2).local = Json_getInt(
                constraintMap_0,
                b"local\0" as *const u8 as *const c_char,
                0 as c_int,
            );
            (*data_2).relative = Json_getInt(
                constraintMap_0,
                b"relative\0" as *const u8 as *const c_char,
                0 as c_int,
            );
            (*data_2).offsetRotation = Json_getFloat(
                constraintMap_0,
                b"rotation\0" as *const u8 as *const c_char,
                0 as c_int as c_float,
            );
            (*data_2).offsetX = Json_getFloat(
                constraintMap_0,
                b"x\0" as *const u8 as *const c_char,
                0 as c_int as c_float,
            ) * (*self_0).scale;
            (*data_2).offsetY = Json_getFloat(
                constraintMap_0,
                b"y\0" as *const u8 as *const c_char,
                0 as c_int as c_float,
            ) * (*self_0).scale;
            (*data_2).offsetScaleX = Json_getFloat(
                constraintMap_0,
                b"scaleX\0" as *const u8 as *const c_char,
                0 as c_int as c_float,
            );
            (*data_2).offsetScaleY = Json_getFloat(
                constraintMap_0,
                b"scaleY\0" as *const u8 as *const c_char,
                0 as c_int as c_float,
            );
            (*data_2).offsetShearY = Json_getFloat(
                constraintMap_0,
                b"shearY\0" as *const u8 as *const c_char,
                0 as c_int as c_float,
            );
            (*data_2).mixRotate = Json_getFloat(
                constraintMap_0,
                b"mixRotate\0" as *const u8 as *const c_char,
                1 as c_int as c_float,
            );
            (*data_2).mixX = Json_getFloat(
                constraintMap_0,
                b"mixX\0" as *const u8 as *const c_char,
                1 as c_int as c_float,
            );
            (*data_2).mixY = Json_getFloat(
                constraintMap_0,
                b"mixY\0" as *const u8 as *const c_char,
                (*data_2).mixX,
            );
            (*data_2).mixScaleX = Json_getFloat(
                constraintMap_0,
                b"mixScaleX\0" as *const u8 as *const c_char,
                1 as c_int as c_float,
            );
            (*data_2).mixScaleY = Json_getFloat(
                constraintMap_0,
                b"mixScaleY\0" as *const u8 as *const c_char,
                (*data_2).mixScaleX,
            );
            (*data_2).mixShearY = Json_getFloat(
                constraintMap_0,
                b"mixShearY\0" as *const u8 as *const c_char,
                1 as c_int as c_float,
            );
            let ref mut fresh192 = *((*skeletonData).transformConstraints).offset(i as isize);
            *fresh192 = data_2;
            constraintMap_0 = (*constraintMap_0).next;
            i += 1;
        }
    }
    pathJson = Json_getItem(root, b"path\0" as *const u8 as *const c_char);
    if !pathJson.is_null() {
        let mut constraintMap_1: *mut Json = 0 as *mut Json;
        (*skeletonData).pathConstraintsCount = (*pathJson).size;
        (*skeletonData).pathConstraints = _spMalloc(
            (::core::mem::size_of::<*mut spPathConstraintData>() as c_ulong)
                .wrapping_mul((*pathJson).size as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            11801 as c_int,
        ) as *mut *mut spPathConstraintData;
        constraintMap_1 = (*pathJson).child;
        i = 0 as c_int;
        while !constraintMap_1.is_null() {
            let mut name_0: *const c_char = 0 as *const c_char;
            let mut item_0: *const c_char = 0 as *const c_char;
            let mut data_3: *mut spPathConstraintData =
                spPathConstraintData_create(Json_getString(
                    constraintMap_1,
                    b"name\0" as *const u8 as *const c_char,
                    0 as *const c_char,
                ));
            (*data_3).order = Json_getInt(
                constraintMap_1,
                b"order\0" as *const u8 as *const c_char,
                0 as c_int,
            );
            (*data_3).skinRequired = if Json_getInt(
                constraintMap_1,
                b"skin\0" as *const u8 as *const c_char,
                0 as c_int,
            ) != 0
            {
                1 as c_int
            } else {
                0 as c_int
            };
            boneMap = Json_getItem(constraintMap_1, b"bones\0" as *const u8 as *const c_char);
            (*data_3).bonesCount = (*boneMap).size;
            let ref mut fresh193 =
                *(&(*data_3).bones as *const *mut *mut spBoneData as *mut *mut *mut spBoneData);
            *fresh193 = _spMalloc(
                (::core::mem::size_of::<*mut spBoneData>() as c_ulong)
                    .wrapping_mul((*boneMap).size as c_ulong),
                b"spine.c\0" as *const u8 as *const c_char,
                11812 as c_int,
            ) as *mut *mut spBoneData;
            boneMap = (*boneMap).child;
            ii = 0 as c_int;
            while !boneMap.is_null() {
                let ref mut fresh194 = *((*data_3).bones).offset(ii as isize);
                *fresh194 = spSkeletonData_findBone(skeletonData, (*boneMap).valueString);
                if (*((*data_3).bones).offset(ii as isize)).is_null() {
                    spSkeletonData_dispose(skeletonData);
                    _spSkeletonJson_setError(
                        self_0,
                        root,
                        b"Path bone not found: \0" as *const u8 as *const c_char,
                        (*boneMap).valueString,
                    );
                    return 0 as *mut spSkeletonData;
                }
                boneMap = (*boneMap).next;
                ii += 1;
            }
            name_0 = Json_getString(
                constraintMap_1,
                b"target\0" as *const u8 as *const c_char,
                0 as *const c_char,
            );
            (*data_3).target = spSkeletonData_findSlot(skeletonData, name_0);
            if ((*data_3).target).is_null() {
                spSkeletonData_dispose(skeletonData);
                _spSkeletonJson_setError(
                    self_0,
                    root,
                    b"Target slot not found: \0" as *const u8 as *const c_char,
                    name_0,
                );
                return 0 as *mut spSkeletonData;
            }
            item_0 = Json_getString(
                constraintMap_1,
                b"positionMode\0" as *const u8 as *const c_char,
                b"percent\0" as *const u8 as *const c_char,
            );
            if spine_strcmp(item_0, b"fixed\0" as *const u8 as *const c_char) == 0 as c_int {
                (*data_3).positionMode = SP_POSITION_MODE_FIXED;
            } else if spine_strcmp(item_0, b"percent\0" as *const u8 as *const c_char) == 0 as c_int
            {
                (*data_3).positionMode = SP_POSITION_MODE_PERCENT;
            }
            item_0 = Json_getString(
                constraintMap_1,
                b"spacingMode\0" as *const u8 as *const c_char,
                b"length\0" as *const u8 as *const c_char,
            );
            if spine_strcmp(item_0, b"length\0" as *const u8 as *const c_char) == 0 as c_int {
                (*data_3).spacingMode = SP_SPACING_MODE_LENGTH;
            } else if spine_strcmp(item_0, b"fixed\0" as *const u8 as *const c_char) == 0 as c_int {
                (*data_3).spacingMode = SP_SPACING_MODE_FIXED;
            } else if spine_strcmp(item_0, b"percent\0" as *const u8 as *const c_char) == 0 as c_int
            {
                (*data_3).spacingMode = SP_SPACING_MODE_PERCENT;
            } else {
                (*data_3).spacingMode = SP_SPACING_MODE_PROPORTIONAL;
            }
            item_0 = Json_getString(
                constraintMap_1,
                b"rotateMode\0" as *const u8 as *const c_char,
                b"tangent\0" as *const u8 as *const c_char,
            );
            if spine_strcmp(item_0, b"tangent\0" as *const u8 as *const c_char) == 0 as c_int {
                (*data_3).rotateMode = SP_ROTATE_MODE_TANGENT;
            } else if spine_strcmp(item_0, b"chain\0" as *const u8 as *const c_char) == 0 as c_int {
                (*data_3).rotateMode = SP_ROTATE_MODE_CHAIN;
            } else if spine_strcmp(item_0, b"chainScale\0" as *const u8 as *const c_char)
                == 0 as c_int
            {
                (*data_3).rotateMode = SP_ROTATE_MODE_CHAIN_SCALE;
            }
            (*data_3).offsetRotation = Json_getFloat(
                constraintMap_1,
                b"rotation\0" as *const u8 as *const c_char,
                0 as c_int as c_float,
            );
            (*data_3).position = Json_getFloat(
                constraintMap_1,
                b"position\0" as *const u8 as *const c_char,
                0 as c_int as c_float,
            );
            if (*data_3).positionMode as c_uint == SP_POSITION_MODE_FIXED as c_int as c_uint {
                (*data_3).position *= (*self_0).scale;
            }
            (*data_3).spacing = Json_getFloat(
                constraintMap_1,
                b"spacing\0" as *const u8 as *const c_char,
                0 as c_int as c_float,
            );
            if (*data_3).spacingMode as c_uint == SP_SPACING_MODE_LENGTH as c_int as c_uint
                || (*data_3).spacingMode as c_uint == SP_SPACING_MODE_FIXED as c_int as c_uint
            {
                (*data_3).spacing *= (*self_0).scale;
            }
            (*data_3).mixRotate = Json_getFloat(
                constraintMap_1,
                b"mixRotate\0" as *const u8 as *const c_char,
                1 as c_int as c_float,
            );
            (*data_3).mixX = Json_getFloat(
                constraintMap_1,
                b"mixX\0" as *const u8 as *const c_char,
                1 as c_int as c_float,
            );
            (*data_3).mixY = Json_getFloat(
                constraintMap_1,
                b"mixY\0" as *const u8 as *const c_char,
                (*data_3).mixX,
            );
            let ref mut fresh195 = *((*skeletonData).pathConstraints).offset(i as isize);
            *fresh195 = data_3;
            constraintMap_1 = (*constraintMap_1).next;
            i += 1;
        }
    }
    skins = Json_getItem(root, b"skins\0" as *const u8 as *const c_char);
    if !skins.is_null() {
        let mut skinMap: *mut Json = 0 as *mut Json;
        (*skeletonData).skins = _spMalloc(
            (::core::mem::size_of::<*mut spSkin>() as c_ulong)
                .wrapping_mul((*skins).size as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            11869 as c_int,
        ) as *mut *mut spSkin;
        skinMap = (*skins).child;
        i = 0 as c_int;
        while !skinMap.is_null() {
            let mut attachmentsMap: *mut Json = 0 as *mut Json;
            let mut curves: *mut Json = 0 as *mut Json;
            let mut skinPart: *mut Json = 0 as *mut Json;
            let mut skin: *mut spSkin = spSkin_create(Json_getString(
                skinMap,
                b"name\0" as *const u8 as *const c_char,
                b"\0" as *const u8 as *const c_char,
            ));
            skinPart = Json_getItem(skinMap, b"bones\0" as *const u8 as *const c_char);
            if !skinPart.is_null() {
                skinPart = (*skinPart).child;
                while !skinPart.is_null() {
                    let mut bone: *mut spBoneData =
                        spSkeletonData_findBone(skeletonData, (*skinPart).valueString);
                    if bone.is_null() {
                        spSkeletonData_dispose(skeletonData);
                        _spSkeletonJson_setError(
                            self_0,
                            root,
                            b"Skin bone constraint not found: \0" as *const u8 as *const c_char,
                            (*skinPart).valueString,
                        );
                        return 0 as *mut spSkeletonData;
                    }
                    spBoneDataArray_add((*skin).bones, bone);
                    skinPart = (*skinPart).next;
                }
            }
            skinPart = Json_getItem(skinMap, b"ik\0" as *const u8 as *const c_char);
            if !skinPart.is_null() {
                skinPart = (*skinPart).child;
                while !skinPart.is_null() {
                    let mut constraint: *mut spIkConstraintData =
                        spSkeletonData_findIkConstraint(skeletonData, (*skinPart).valueString);
                    if constraint.is_null() {
                        spSkeletonData_dispose(skeletonData);
                        _spSkeletonJson_setError(
                            self_0,
                            root,
                            b"Skin IK constraint not found: \0" as *const u8 as *const c_char,
                            (*skinPart).valueString,
                        );
                        return 0 as *mut spSkeletonData;
                    }
                    spIkConstraintDataArray_add((*skin).ikConstraints, constraint);
                    skinPart = (*skinPart).next;
                }
            }
            skinPart = Json_getItem(skinMap, b"path\0" as *const u8 as *const c_char);
            if !skinPart.is_null() {
                skinPart = (*skinPart).child;
                while !skinPart.is_null() {
                    let mut constraint_0: *mut spPathConstraintData =
                        spSkeletonData_findPathConstraint(skeletonData, (*skinPart).valueString);
                    if constraint_0.is_null() {
                        spSkeletonData_dispose(skeletonData);
                        _spSkeletonJson_setError(
                            self_0,
                            root,
                            b"Skin path constraint not found: \0" as *const u8 as *const c_char,
                            (*skinPart).valueString,
                        );
                        return 0 as *mut spSkeletonData;
                    }
                    spPathConstraintDataArray_add((*skin).pathConstraints, constraint_0);
                    skinPart = (*skinPart).next;
                }
            }
            skinPart = Json_getItem(skinMap, b"transform\0" as *const u8 as *const c_char);
            if !skinPart.is_null() {
                skinPart = (*skinPart).child;
                while !skinPart.is_null() {
                    let mut constraint_1: *mut spTransformConstraintData =
                        spSkeletonData_findTransformConstraint(
                            skeletonData,
                            (*skinPart).valueString,
                        );
                    if constraint_1.is_null() {
                        spSkeletonData_dispose(skeletonData);
                        _spSkeletonJson_setError(
                            self_0,
                            root,
                            b"Skin transform constraint not found: \0" as *const u8
                                as *const c_char,
                            (*skinPart).valueString,
                        );
                        return 0 as *mut spSkeletonData;
                    }
                    spTransformConstraintDataArray_add((*skin).transformConstraints, constraint_1);
                    skinPart = (*skinPart).next;
                }
            }
            let fresh196 = (*skeletonData).skinsCount;
            (*skeletonData).skinsCount = (*skeletonData).skinsCount + 1;
            let ref mut fresh197 = *((*skeletonData).skins).offset(fresh196 as isize);
            *fresh197 = skin;
            if spine_strcmp((*skin).name, b"default\0" as *const u8 as *const c_char) == 0 as c_int
            {
                (*skeletonData).defaultSkin = skin;
            }
            attachmentsMap =
                (*Json_getItem(skinMap, b"attachments\0" as *const u8 as *const c_char)).child;
            while !attachmentsMap.is_null() {
                let mut slot: *mut spSlotData =
                    spSkeletonData_findSlot(skeletonData, (*attachmentsMap).name);
                let mut attachmentMap: *mut Json = 0 as *mut Json;
                attachmentMap = (*attachmentsMap).child;
                while !attachmentMap.is_null() {
                    let mut attachment: *mut spAttachment = 0 as *mut spAttachment;
                    let mut skinAttachmentName: *const c_char = (*attachmentMap).name;
                    let mut attachmentName: *const c_char = Json_getString(
                        attachmentMap,
                        b"name\0" as *const u8 as *const c_char,
                        skinAttachmentName,
                    );
                    let mut path: *const c_char = Json_getString(
                        attachmentMap,
                        b"path\0" as *const u8 as *const c_char,
                        attachmentName,
                    );
                    let mut color_1: *const c_char = 0 as *const c_char;
                    let mut entry: *mut Json = 0 as *mut Json;
                    let mut sequence: *mut spSequence = 0 as *mut spSequence;
                    let mut typeString: *const c_char = Json_getString(
                        attachmentMap,
                        b"type\0" as *const u8 as *const c_char,
                        b"region\0" as *const u8 as *const c_char,
                    );
                    let mut type_0: spAttachmentType = SP_ATTACHMENT_REGION;
                    if spine_strcmp(typeString, b"region\0" as *const u8 as *const c_char)
                        == 0 as c_int
                    {
                        type_0 = SP_ATTACHMENT_REGION;
                    } else if spine_strcmp(typeString, b"mesh\0" as *const u8 as *const c_char)
                        == 0 as c_int
                    {
                        type_0 = SP_ATTACHMENT_MESH;
                    } else if spine_strcmp(
                        typeString,
                        b"linkedmesh\0" as *const u8 as *const c_char,
                    ) == 0 as c_int
                    {
                        type_0 = SP_ATTACHMENT_LINKED_MESH;
                    } else if spine_strcmp(
                        typeString,
                        b"boundingbox\0" as *const u8 as *const c_char,
                    ) == 0 as c_int
                    {
                        type_0 = SP_ATTACHMENT_BOUNDING_BOX;
                    } else if spine_strcmp(typeString, b"path\0" as *const u8 as *const c_char)
                        == 0 as c_int
                    {
                        type_0 = SP_ATTACHMENT_PATH;
                    } else if spine_strcmp(typeString, b"clipping\0" as *const u8 as *const c_char)
                        == 0 as c_int
                    {
                        type_0 = SP_ATTACHMENT_CLIPPING;
                    } else if spine_strcmp(typeString, b"point\0" as *const u8 as *const c_char)
                        == 0 as c_int
                    {
                        type_0 = SP_ATTACHMENT_POINT;
                    } else {
                        spSkeletonData_dispose(skeletonData);
                        _spSkeletonJson_setError(
                            self_0,
                            root,
                            b"Unknown attachment type: \0" as *const u8 as *const c_char,
                            typeString,
                        );
                        return 0 as *mut spSkeletonData;
                    }
                    sequence = readSequenceJson(Json_getItem(
                        attachmentMap,
                        b"sequence\0" as *const u8 as *const c_char,
                    ));
                    attachment = spAttachmentLoader_createAttachment(
                        (*self_0).attachmentLoader,
                        skin,
                        type_0,
                        attachmentName,
                        path,
                        sequence,
                    );
                    if attachment.is_null() {
                        if !((*(*self_0).attachmentLoader).error1).is_null() {
                            spSkeletonData_dispose(skeletonData);
                            _spSkeletonJson_setError(
                                self_0,
                                root,
                                (*(*self_0).attachmentLoader).error1,
                                (*(*self_0).attachmentLoader).error2,
                            );
                            return 0 as *mut spSkeletonData;
                        }
                    } else {
                        match (*attachment).type_0 as c_uint {
                            0 => {
                                let mut region: *mut spRegionAttachment =
                                    attachment as *mut spRegionAttachment;
                                if !path.is_null() {
                                    let ref mut fresh198 = *(&mut (*region).path
                                        as *mut *const c_char
                                        as *mut *mut c_char);
                                    *fresh198 = _spMalloc(
                                        (::core::mem::size_of::<c_char>() as c_ulong).wrapping_mul(
                                            (spine_strlen(path))
                                                .wrapping_add(1 as c_int as c_ulong),
                                        ),
                                        b"spine.c\0" as *const u8 as *const c_char,
                                        11988 as c_int,
                                    )
                                        as *mut c_char;
                                    spine_strcpy(*fresh198, path);
                                }
                                (*region).x = Json_getFloat(
                                    attachmentMap,
                                    b"x\0" as *const u8 as *const c_char,
                                    0 as c_int as c_float,
                                ) * (*self_0).scale;
                                (*region).y = Json_getFloat(
                                    attachmentMap,
                                    b"y\0" as *const u8 as *const c_char,
                                    0 as c_int as c_float,
                                ) * (*self_0).scale;
                                (*region).scaleX = Json_getFloat(
                                    attachmentMap,
                                    b"scaleX\0" as *const u8 as *const c_char,
                                    1 as c_int as c_float,
                                );
                                (*region).scaleY = Json_getFloat(
                                    attachmentMap,
                                    b"scaleY\0" as *const u8 as *const c_char,
                                    1 as c_int as c_float,
                                );
                                (*region).rotation = Json_getFloat(
                                    attachmentMap,
                                    b"rotation\0" as *const u8 as *const c_char,
                                    0 as c_int as c_float,
                                );
                                (*region).width = Json_getFloat(
                                    attachmentMap,
                                    b"width\0" as *const u8 as *const c_char,
                                    32 as c_int as c_float,
                                ) * (*self_0).scale;
                                (*region).height = Json_getFloat(
                                    attachmentMap,
                                    b"height\0" as *const u8 as *const c_char,
                                    32 as c_int as c_float,
                                ) * (*self_0).scale;
                                (*region).sequence = sequence;
                                color_1 = Json_getString(
                                    attachmentMap,
                                    b"color\0" as *const u8 as *const c_char,
                                    0 as *const c_char,
                                );
                                if !color_1.is_null() {
                                    spColor_setFromFloats(
                                        &mut (*region).color,
                                        toColor(color_1, 0 as c_int),
                                        toColor(color_1, 1 as c_int),
                                        toColor(color_1, 2 as c_int),
                                        toColor(color_1, 3 as c_int),
                                    );
                                }
                                if !((*region).region).is_null() {
                                    spRegionAttachment_updateRegion(region);
                                }
                                spAttachmentLoader_configureAttachment(
                                    (*self_0).attachmentLoader,
                                    attachment,
                                );
                            }
                            2 | 3 => {
                                let mut mesh: *mut spMeshAttachment =
                                    attachment as *mut spMeshAttachment;
                                let ref mut fresh199 =
                                    *(&mut (*mesh).path as *mut *const c_char as *mut *mut c_char);
                                *fresh199 = _spMalloc(
                                    (::core::mem::size_of::<c_char>() as c_ulong).wrapping_mul(
                                        (spine_strlen(path)).wrapping_add(1 as c_int as c_ulong),
                                    ),
                                    b"spine.c\0" as *const u8 as *const c_char,
                                    12016 as c_int,
                                ) as *mut c_char;
                                spine_strcpy(*fresh199, path);
                                color_1 = Json_getString(
                                    attachmentMap,
                                    b"color\0" as *const u8 as *const c_char,
                                    0 as *const c_char,
                                );
                                if !color_1.is_null() {
                                    spColor_setFromFloats(
                                        &mut (*mesh).color,
                                        toColor(color_1, 0 as c_int),
                                        toColor(color_1, 1 as c_int),
                                        toColor(color_1, 2 as c_int),
                                        toColor(color_1, 3 as c_int),
                                    );
                                }
                                (*mesh).width = Json_getFloat(
                                    attachmentMap,
                                    b"width\0" as *const u8 as *const c_char,
                                    32 as c_int as c_float,
                                ) * (*self_0).scale;
                                (*mesh).height = Json_getFloat(
                                    attachmentMap,
                                    b"height\0" as *const u8 as *const c_char,
                                    32 as c_int as c_float,
                                ) * (*self_0).scale;
                                (*mesh).sequence = sequence;
                                entry = Json_getItem(
                                    attachmentMap,
                                    b"parent\0" as *const u8 as *const c_char,
                                );
                                if entry.is_null() {
                                    let mut verticesLength: c_int = 0;
                                    entry = Json_getItem(
                                        attachmentMap,
                                        b"triangles\0" as *const u8 as *const c_char,
                                    );
                                    (*mesh).trianglesCount = (*entry).size;
                                    (*mesh).triangles = _spMalloc(
                                        (::core::mem::size_of::<c_ushort>() as c_ulong)
                                            .wrapping_mul((*entry).size as c_ulong),
                                        b"spine.c\0" as *const u8 as *const c_char,
                                        12036 as c_int,
                                    )
                                        as *mut c_ushort;
                                    entry = (*entry).child;
                                    ii = 0 as c_int;
                                    while !entry.is_null() {
                                        *((*mesh).triangles).offset(ii as isize) =
                                            (*entry).valueInt as c_ushort;
                                        entry = (*entry).next;
                                        ii += 1;
                                    }
                                    entry = Json_getItem(
                                        attachmentMap,
                                        b"uvs\0" as *const u8 as *const c_char,
                                    );
                                    verticesLength = (*entry).size;
                                    (*mesh).regionUVs = _spMalloc(
                                        (::core::mem::size_of::<c_float>() as c_ulong)
                                            .wrapping_mul(verticesLength as c_ulong),
                                        b"spine.c\0" as *const u8 as *const c_char,
                                        12042 as c_int,
                                    )
                                        as *mut c_float;
                                    entry = (*entry).child;
                                    ii = 0 as c_int;
                                    while !entry.is_null() {
                                        *((*mesh).regionUVs).offset(ii as isize) =
                                            (*entry).valueFloat;
                                        entry = (*entry).next;
                                        ii += 1;
                                    }
                                    _readVerticesJson(
                                        self_0,
                                        attachmentMap,
                                        &mut (*mesh).super_0,
                                        verticesLength,
                                    );
                                    if !((*mesh).region).is_null() {
                                        spMeshAttachment_updateRegion(mesh);
                                    }
                                    (*mesh).hullLength = Json_getInt(
                                        attachmentMap,
                                        b"hull\0" as *const u8 as *const c_char,
                                        0 as c_int,
                                    );
                                    entry = Json_getItem(
                                        attachmentMap,
                                        b"edges\0" as *const u8 as *const c_char,
                                    );
                                    if !entry.is_null() {
                                        (*mesh).edgesCount = (*entry).size;
                                        (*mesh).edges = _spMalloc(
                                            (::core::mem::size_of::<c_int>() as c_ulong)
                                                .wrapping_mul((*entry).size as c_ulong),
                                            b"spine.c\0" as *const u8 as *const c_char,
                                            12055 as c_int,
                                        )
                                            as *mut c_int;
                                        entry = (*entry).child;
                                        ii = 0 as c_int;
                                        while !entry.is_null() {
                                            *((*mesh).edges).offset(ii as isize) =
                                                (*entry).valueInt;
                                            entry = (*entry).next;
                                            ii += 1;
                                        }
                                    }
                                    spAttachmentLoader_configureAttachment(
                                        (*self_0).attachmentLoader,
                                        attachment,
                                    );
                                } else {
                                    let mut inheritTimelines: c_int = Json_getInt(
                                        attachmentMap,
                                        b"timelines\0" as *const u8 as *const c_char,
                                        1 as c_int,
                                    );
                                    _spSkeletonJson_addLinkedMesh(
                                        self_0,
                                        attachment as *mut spMeshAttachment,
                                        Json_getString(
                                            attachmentMap,
                                            b"skin\0" as *const u8 as *const c_char,
                                            0 as *const c_char,
                                        ),
                                        (*slot).index,
                                        (*entry).valueString,
                                        inheritTimelines,
                                    );
                                }
                            }
                            1 => {
                                let mut box_0: *mut spBoundingBoxAttachment =
                                    attachment as *mut spBoundingBoxAttachment;
                                let mut vertexCount: c_int = Json_getInt(
                                    attachmentMap,
                                    b"vertexCount\0" as *const u8 as *const c_char,
                                    0 as c_int,
                                ) << 1 as c_int;
                                _readVerticesJson(
                                    self_0,
                                    attachmentMap,
                                    &mut (*box_0).super_0,
                                    vertexCount,
                                );
                                (*box_0).super_0.verticesCount = vertexCount;
                                color_1 = Json_getString(
                                    attachmentMap,
                                    b"color\0" as *const u8 as *const c_char,
                                    0 as *const c_char,
                                );
                                if !color_1.is_null() {
                                    spColor_setFromFloats(
                                        &mut (*box_0).color,
                                        toColor(color_1, 0 as c_int),
                                        toColor(color_1, 1 as c_int),
                                        toColor(color_1, 2 as c_int),
                                        toColor(color_1, 3 as c_int),
                                    );
                                }
                                spAttachmentLoader_configureAttachment(
                                    (*self_0).attachmentLoader,
                                    attachment,
                                );
                            }
                            4 => {
                                let mut pathAttachment: *mut spPathAttachment =
                                    attachment as *mut spPathAttachment;
                                let mut vertexCount_0: c_int = 0 as c_int;
                                (*pathAttachment).closed = Json_getInt(
                                    attachmentMap,
                                    b"closed\0" as *const u8 as *const c_char,
                                    0 as c_int,
                                );
                                (*pathAttachment).constantSpeed = Json_getInt(
                                    attachmentMap,
                                    b"constantSpeed\0" as *const u8 as *const c_char,
                                    1 as c_int,
                                );
                                vertexCount_0 = Json_getInt(
                                    attachmentMap,
                                    b"vertexCount\0" as *const u8 as *const c_char,
                                    0 as c_int,
                                );
                                _readVerticesJson(
                                    self_0,
                                    attachmentMap,
                                    &mut (*pathAttachment).super_0,
                                    vertexCount_0 << 1 as c_int,
                                );
                                (*pathAttachment).lengthsLength = vertexCount_0 / 3 as c_int;
                                (*pathAttachment).lengths = _spMalloc(
                                    (::core::mem::size_of::<c_float>() as c_ulong)
                                        .wrapping_mul((*pathAttachment).lengthsLength as c_ulong),
                                    b"spine.c\0" as *const u8 as *const c_char,
                                    12094 as c_int,
                                )
                                    as *mut c_float;
                                curves = Json_getItem(
                                    attachmentMap,
                                    b"lengths\0" as *const u8 as *const c_char,
                                );
                                curves = (*curves).child;
                                ii = 0 as c_int;
                                while !curves.is_null() {
                                    *((*pathAttachment).lengths).offset(ii as isize) =
                                        (*curves).valueFloat * (*self_0).scale;
                                    curves = (*curves).next;
                                    ii += 1;
                                }
                                color_1 = Json_getString(
                                    attachmentMap,
                                    b"color\0" as *const u8 as *const c_char,
                                    0 as *const c_char,
                                );
                                if !color_1.is_null() {
                                    spColor_setFromFloats(
                                        &mut (*pathAttachment).color,
                                        toColor(color_1, 0 as c_int),
                                        toColor(color_1, 1 as c_int),
                                        toColor(color_1, 2 as c_int),
                                        toColor(color_1, 3 as c_int),
                                    );
                                }
                            }
                            5 => {
                                let mut point: *mut spPointAttachment =
                                    attachment as *mut spPointAttachment;
                                (*point).x = Json_getFloat(
                                    attachmentMap,
                                    b"x\0" as *const u8 as *const c_char,
                                    0 as c_int as c_float,
                                ) * (*self_0).scale;
                                (*point).y = Json_getFloat(
                                    attachmentMap,
                                    b"y\0" as *const u8 as *const c_char,
                                    0 as c_int as c_float,
                                ) * (*self_0).scale;
                                (*point).rotation = Json_getFloat(
                                    attachmentMap,
                                    b"rotation\0" as *const u8 as *const c_char,
                                    0 as c_int as c_float,
                                );
                                color_1 = Json_getString(
                                    attachmentMap,
                                    b"color\0" as *const u8 as *const c_char,
                                    0 as *const c_char,
                                );
                                if !color_1.is_null() {
                                    spColor_setFromFloats(
                                        &mut (*point).color,
                                        toColor(color_1, 0 as c_int),
                                        toColor(color_1, 1 as c_int),
                                        toColor(color_1, 2 as c_int),
                                        toColor(color_1, 3 as c_int),
                                    );
                                }
                            }
                            6 => {
                                let mut clip: *mut spClippingAttachment =
                                    attachment as *mut spClippingAttachment;
                                let mut vertexCount_1: c_int = 0 as c_int;
                                let mut end: *const c_char = Json_getString(
                                    attachmentMap,
                                    b"end\0" as *const u8 as *const c_char,
                                    0 as *const c_char,
                                );
                                if !end.is_null() {
                                    let mut endSlot: *mut spSlotData =
                                        spSkeletonData_findSlot(skeletonData, end);
                                    (*clip).endSlot = endSlot;
                                }
                                vertexCount_1 = Json_getInt(
                                    attachmentMap,
                                    b"vertexCount\0" as *const u8 as *const c_char,
                                    0 as c_int,
                                ) << 1 as c_int;
                                _readVerticesJson(
                                    self_0,
                                    attachmentMap,
                                    &mut (*clip).super_0,
                                    vertexCount_1,
                                );
                                color_1 = Json_getString(
                                    attachmentMap,
                                    b"color\0" as *const u8 as *const c_char,
                                    0 as *const c_char,
                                );
                                if !color_1.is_null() {
                                    spColor_setFromFloats(
                                        &mut (*clip).color,
                                        toColor(color_1, 0 as c_int),
                                        toColor(color_1, 1 as c_int),
                                        toColor(color_1, 2 as c_int),
                                        toColor(color_1, 3 as c_int),
                                    );
                                }
                                spAttachmentLoader_configureAttachment(
                                    (*self_0).attachmentLoader,
                                    attachment,
                                );
                            }
                            _ => {}
                        }
                        spSkin_setAttachment(skin, (*slot).index, skinAttachmentName, attachment);
                    }
                    attachmentMap = (*attachmentMap).next;
                }
                attachmentsMap = (*attachmentsMap).next;
            }
            skinMap = (*skinMap).next;
            i += 1;
        }
    }
    i = 0 as c_int;
    while i < (*internal).linkedMeshCount {
        let mut parent_0: *mut spAttachment = 0 as *mut spAttachment;
        let mut linkedMesh: *mut _spLinkedMeshJson = ((*internal).linkedMeshes).offset(i as isize);
        let mut skin_0: *mut spSkin = if ((*linkedMesh).skin).is_null() {
            (*skeletonData).defaultSkin
        } else {
            spSkeletonData_findSkin(skeletonData, (*linkedMesh).skin)
        };
        if skin_0.is_null() {
            spSkeletonData_dispose(skeletonData);
            _spSkeletonJson_setError(
                self_0,
                0 as *mut Json,
                b"Skin not found: \0" as *const u8 as *const c_char,
                (*linkedMesh).skin,
            );
            return 0 as *mut spSkeletonData;
        }
        parent_0 = spSkin_getAttachment(skin_0, (*linkedMesh).slotIndex, (*linkedMesh).parent);
        if parent_0.is_null() {
            spSkeletonData_dispose(skeletonData);
            _spSkeletonJson_setError(
                self_0,
                0 as *mut Json,
                b"Parent mesh not found: \0" as *const u8 as *const c_char,
                (*linkedMesh).parent,
            );
            return 0 as *mut spSkeletonData;
        }
        (*(*linkedMesh).mesh).super_0.timelineAttachment = if (*linkedMesh).inheritTimeline != 0 {
            parent_0
        } else {
            &mut (*(*linkedMesh).mesh).super_0.super_0
        };
        spMeshAttachment_setParentMesh((*linkedMesh).mesh, parent_0 as *mut spMeshAttachment);
        if !((*(*linkedMesh).mesh).region).is_null() {
            spMeshAttachment_updateRegion((*linkedMesh).mesh);
        }
        spAttachmentLoader_configureAttachment(
            (*self_0).attachmentLoader,
            &mut (*(*linkedMesh).mesh).super_0.super_0,
        );
        i += 1;
    }
    events = Json_getItem(root, b"events\0" as *const u8 as *const c_char);
    if !events.is_null() {
        let mut eventMap: *mut Json = 0 as *mut Json;
        let mut stringValue: *const c_char = 0 as *const c_char;
        let mut audioPath: *const c_char = 0 as *const c_char;
        (*skeletonData).eventsCount = (*events).size;
        (*skeletonData).events = _spMalloc(
            (::core::mem::size_of::<*mut spEventData>() as c_ulong)
                .wrapping_mul((*events).size as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            12184 as c_int,
        ) as *mut *mut spEventData;
        eventMap = (*events).child;
        i = 0 as c_int;
        while !eventMap.is_null() {
            let mut eventData: *mut spEventData = spEventData_create((*eventMap).name);
            (*eventData).intValue =
                Json_getInt(eventMap, b"int\0" as *const u8 as *const c_char, 0 as c_int);
            (*eventData).floatValue = Json_getFloat(
                eventMap,
                b"float\0" as *const u8 as *const c_char,
                0 as c_int as c_float,
            );
            stringValue = Json_getString(
                eventMap,
                b"string\0" as *const u8 as *const c_char,
                0 as *const c_char,
            );
            if !stringValue.is_null() {
                let ref mut fresh200 =
                    *(&mut (*eventData).stringValue as *mut *const c_char as *mut *mut c_char);
                *fresh200 = _spMalloc(
                    (::core::mem::size_of::<c_char>() as c_ulong).wrapping_mul(
                        (spine_strlen(stringValue)).wrapping_add(1 as c_int as c_ulong),
                    ),
                    b"spine.c\0" as *const u8 as *const c_char,
                    12190 as c_int,
                ) as *mut c_char;
                spine_strcpy(*fresh200, stringValue);
            }
            audioPath = Json_getString(
                eventMap,
                b"audio\0" as *const u8 as *const c_char,
                0 as *const c_char,
            );
            if !audioPath.is_null() {
                let ref mut fresh201 =
                    *(&mut (*eventData).audioPath as *mut *const c_char as *mut *mut c_char);
                *fresh201 = _spMalloc(
                    (::core::mem::size_of::<c_char>() as c_ulong).wrapping_mul(
                        (spine_strlen(audioPath)).wrapping_add(1 as c_int as c_ulong),
                    ),
                    b"spine.c\0" as *const u8 as *const c_char,
                    12193 as c_int,
                ) as *mut c_char;
                spine_strcpy(*fresh201, audioPath);
                (*eventData).volume = Json_getFloat(
                    eventMap,
                    b"volume\0" as *const u8 as *const c_char,
                    1 as c_int as c_float,
                );
                (*eventData).balance = Json_getFloat(
                    eventMap,
                    b"balance\0" as *const u8 as *const c_char,
                    0 as c_int as c_float,
                );
            }
            let ref mut fresh202 = *((*skeletonData).events).offset(i as isize);
            *fresh202 = eventData;
            eventMap = (*eventMap).next;
            i += 1;
        }
    }
    animations = Json_getItem(root, b"animations\0" as *const u8 as *const c_char);
    if !animations.is_null() {
        let mut animationMap: *mut Json = 0 as *mut Json;
        (*skeletonData).animations = _spMalloc(
            (::core::mem::size_of::<*mut spAnimation>() as c_ulong)
                .wrapping_mul((*animations).size as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            12205 as c_int,
        ) as *mut *mut spAnimation;
        animationMap = (*animations).child;
        while !animationMap.is_null() {
            let mut animation: *mut spAnimation =
                _spSkeletonJson_readAnimation(self_0, animationMap, skeletonData);
            if animation.is_null() {
                spSkeletonData_dispose(skeletonData);
                return 0 as *mut spSkeletonData;
            }
            let fresh203 = (*skeletonData).animationsCount;
            (*skeletonData).animationsCount = (*skeletonData).animationsCount + 1;
            let ref mut fresh204 = *((*skeletonData).animations).offset(fresh203 as isize);
            *fresh204 = animation;
            animationMap = (*animationMap).next;
        }
    }
    Json_dispose(root);
    return skeletonData;
}
#[no_mangle]
pub unsafe extern "C" fn spBoneDataArray_create(
    mut initialCapacity: c_int,
) -> *mut spBoneDataArray {
    let mut array: *mut spBoneDataArray = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spBoneDataArray>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        12252 as c_int,
    ) as *mut spBoneDataArray;
    (*array).size = 0 as c_int;
    (*array).capacity = initialCapacity;
    (*array).items = _spCalloc(
        initialCapacity as size_t,
        ::core::mem::size_of::<*mut spBoneData>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        12252 as c_int,
    ) as *mut *mut spBoneData;
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn spBoneDataArray_dispose(mut self_0: *mut spBoneDataArray) {
    _spFree((*self_0).items as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spBoneDataArray_clear(mut self_0: *mut spBoneDataArray) {
    (*self_0).size = 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spBoneDataArray_setSize(
    mut self_0: *mut spBoneDataArray,
    mut newSize: c_int,
) -> *mut spBoneDataArray {
    (*self_0).size = newSize;
    if (*self_0).capacity < newSize {
        (*self_0).capacity = if 8 as c_int > ((*self_0).size as c_float * 1.75f32) as c_int {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0).items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<*mut spBoneData>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut *mut spBoneData;
    }
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spBoneDataArray_ensureCapacity(
    mut self_0: *mut spBoneDataArray,
    mut newCapacity: c_int,
) {
    if (*self_0).capacity >= newCapacity {
        return;
    }
    (*self_0).capacity = newCapacity;
    (*self_0).items = _spRealloc(
        (*self_0).items as *mut c_void,
        (::core::mem::size_of::<*mut spBoneData>() as c_ulong)
            .wrapping_mul((*self_0).capacity as c_ulong),
    ) as *mut *mut spBoneData;
}
#[no_mangle]
pub unsafe extern "C" fn spBoneDataArray_add(
    mut self_0: *mut spBoneDataArray,
    mut value: *mut spBoneData,
) {
    if (*self_0).size == (*self_0).capacity {
        (*self_0).capacity = if 8 as c_int > ((*self_0).size as c_float * 1.75f32) as c_int {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0).items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<*mut spBoneData>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut *mut spBoneData;
    }
    let fresh205 = (*self_0).size;
    (*self_0).size = (*self_0).size + 1;
    let ref mut fresh206 = *((*self_0).items).offset(fresh205 as isize);
    *fresh206 = value;
}
#[no_mangle]
pub unsafe extern "C" fn spBoneDataArray_addAll(
    mut self_0: *mut spBoneDataArray,
    mut other: *mut spBoneDataArray,
) {
    let mut i: c_int = 0 as c_int;
    while i < (*other).size {
        spBoneDataArray_add(self_0, *((*other).items).offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spBoneDataArray_addAllValues(
    mut self_0: *mut spBoneDataArray,
    mut values: *mut *mut spBoneData,
    mut offset: c_int,
    mut count: c_int,
) {
    let mut i: c_int = offset;
    let mut n: c_int = offset + count;
    while i < n {
        spBoneDataArray_add(self_0, *values.offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spBoneDataArray_removeAt(
    mut self_0: *mut spBoneDataArray,
    mut index: c_int,
) {
    (*self_0).size -= 1;
    spine_memmove(
        ((*self_0).items).offset(index as isize) as *mut c_void,
        ((*self_0).items)
            .offset(index as isize)
            .offset(1 as c_int as isize) as *const c_void,
        (::core::mem::size_of::<*mut spBoneData>() as c_ulong)
            .wrapping_mul(((*self_0).size - index) as c_ulong),
    );
}
#[no_mangle]
pub unsafe extern "C" fn spBoneDataArray_contains(
    mut self_0: *mut spBoneDataArray,
    mut value: *mut spBoneData,
) -> c_int {
    let mut items: *mut *mut spBoneData = (*self_0).items;
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    i = 0 as c_int;
    n = (*self_0).size;
    while i < n {
        if *items.offset(i as isize) == value {
            return -(1 as c_int);
        }
        i += 1;
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spBoneDataArray_pop(mut self_0: *mut spBoneDataArray) -> *mut spBoneData {
    (*self_0).size -= 1;
    let mut item: *mut spBoneData = *((*self_0).items).offset((*self_0).size as isize);
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn spBoneDataArray_peek(mut self_0: *mut spBoneDataArray) -> *mut spBoneData {
    return *((*self_0).items).offset(((*self_0).size - 1 as c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn spIkConstraintDataArray_create(
    mut initialCapacity: c_int,
) -> *mut spIkConstraintDataArray {
    let mut array: *mut spIkConstraintDataArray = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spIkConstraintDataArray>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        12254 as c_int,
    ) as *mut spIkConstraintDataArray;
    (*array).size = 0 as c_int;
    (*array).capacity = initialCapacity;
    (*array).items = _spCalloc(
        initialCapacity as size_t,
        ::core::mem::size_of::<*mut spIkConstraintData>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        12254 as c_int,
    ) as *mut *mut spIkConstraintData;
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn spIkConstraintDataArray_dispose(mut self_0: *mut spIkConstraintDataArray) {
    _spFree((*self_0).items as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spIkConstraintDataArray_clear(mut self_0: *mut spIkConstraintDataArray) {
    (*self_0).size = 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spIkConstraintDataArray_setSize(
    mut self_0: *mut spIkConstraintDataArray,
    mut newSize: c_int,
) -> *mut spIkConstraintDataArray {
    (*self_0).size = newSize;
    if (*self_0).capacity < newSize {
        (*self_0).capacity = if 8 as c_int > ((*self_0).size as c_float * 1.75f32) as c_int {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0).items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<*mut spIkConstraintData>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut *mut spIkConstraintData;
    }
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spIkConstraintDataArray_ensureCapacity(
    mut self_0: *mut spIkConstraintDataArray,
    mut newCapacity: c_int,
) {
    if (*self_0).capacity >= newCapacity {
        return;
    }
    (*self_0).capacity = newCapacity;
    (*self_0).items = _spRealloc(
        (*self_0).items as *mut c_void,
        (::core::mem::size_of::<*mut spIkConstraintData>() as c_ulong)
            .wrapping_mul((*self_0).capacity as c_ulong),
    ) as *mut *mut spIkConstraintData;
}
#[no_mangle]
pub unsafe extern "C" fn spIkConstraintDataArray_add(
    mut self_0: *mut spIkConstraintDataArray,
    mut value: *mut spIkConstraintData,
) {
    if (*self_0).size == (*self_0).capacity {
        (*self_0).capacity = if 8 as c_int > ((*self_0).size as c_float * 1.75f32) as c_int {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0).items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<*mut spIkConstraintData>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut *mut spIkConstraintData;
    }
    let fresh207 = (*self_0).size;
    (*self_0).size = (*self_0).size + 1;
    let ref mut fresh208 = *((*self_0).items).offset(fresh207 as isize);
    *fresh208 = value;
}
#[no_mangle]
pub unsafe extern "C" fn spIkConstraintDataArray_addAll(
    mut self_0: *mut spIkConstraintDataArray,
    mut other: *mut spIkConstraintDataArray,
) {
    let mut i: c_int = 0 as c_int;
    while i < (*other).size {
        spIkConstraintDataArray_add(self_0, *((*other).items).offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spIkConstraintDataArray_addAllValues(
    mut self_0: *mut spIkConstraintDataArray,
    mut values: *mut *mut spIkConstraintData,
    mut offset: c_int,
    mut count: c_int,
) {
    let mut i: c_int = offset;
    let mut n: c_int = offset + count;
    while i < n {
        spIkConstraintDataArray_add(self_0, *values.offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spIkConstraintDataArray_removeAt(
    mut self_0: *mut spIkConstraintDataArray,
    mut index: c_int,
) {
    (*self_0).size -= 1;
    spine_memmove(
        ((*self_0).items).offset(index as isize) as *mut c_void,
        ((*self_0).items)
            .offset(index as isize)
            .offset(1 as c_int as isize) as *const c_void,
        (::core::mem::size_of::<*mut spIkConstraintData>() as c_ulong)
            .wrapping_mul(((*self_0).size - index) as c_ulong),
    );
}
#[no_mangle]
pub unsafe extern "C" fn spIkConstraintDataArray_contains(
    mut self_0: *mut spIkConstraintDataArray,
    mut value: *mut spIkConstraintData,
) -> c_int {
    let mut items: *mut *mut spIkConstraintData = (*self_0).items;
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    i = 0 as c_int;
    n = (*self_0).size;
    while i < n {
        if *items.offset(i as isize) == value {
            return -(1 as c_int);
        }
        i += 1;
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spIkConstraintDataArray_pop(
    mut self_0: *mut spIkConstraintDataArray,
) -> *mut spIkConstraintData {
    (*self_0).size -= 1;
    let mut item: *mut spIkConstraintData = *((*self_0).items).offset((*self_0).size as isize);
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn spIkConstraintDataArray_peek(
    mut self_0: *mut spIkConstraintDataArray,
) -> *mut spIkConstraintData {
    return *((*self_0).items).offset(((*self_0).size - 1 as c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn spTransformConstraintDataArray_create(
    mut initialCapacity: c_int,
) -> *mut spTransformConstraintDataArray {
    let mut array: *mut spTransformConstraintDataArray = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spTransformConstraintDataArray>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        12256 as c_int,
    )
        as *mut spTransformConstraintDataArray;
    (*array).size = 0 as c_int;
    (*array).capacity = initialCapacity;
    (*array).items = _spCalloc(
        initialCapacity as size_t,
        ::core::mem::size_of::<*mut spTransformConstraintData>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        12256 as c_int,
    ) as *mut *mut spTransformConstraintData;
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn spTransformConstraintDataArray_dispose(
    mut self_0: *mut spTransformConstraintDataArray,
) {
    _spFree((*self_0).items as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spTransformConstraintDataArray_clear(
    mut self_0: *mut spTransformConstraintDataArray,
) {
    (*self_0).size = 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spTransformConstraintDataArray_setSize(
    mut self_0: *mut spTransformConstraintDataArray,
    mut newSize: c_int,
) -> *mut spTransformConstraintDataArray {
    (*self_0).size = newSize;
    if (*self_0).capacity < newSize {
        (*self_0).capacity = if 8 as c_int > ((*self_0).size as c_float * 1.75f32) as c_int {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0).items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<*mut spTransformConstraintData>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut *mut spTransformConstraintData;
    }
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spTransformConstraintDataArray_ensureCapacity(
    mut self_0: *mut spTransformConstraintDataArray,
    mut newCapacity: c_int,
) {
    if (*self_0).capacity >= newCapacity {
        return;
    }
    (*self_0).capacity = newCapacity;
    (*self_0).items = _spRealloc(
        (*self_0).items as *mut c_void,
        (::core::mem::size_of::<*mut spTransformConstraintData>() as c_ulong)
            .wrapping_mul((*self_0).capacity as c_ulong),
    ) as *mut *mut spTransformConstraintData;
}
#[no_mangle]
pub unsafe extern "C" fn spTransformConstraintDataArray_add(
    mut self_0: *mut spTransformConstraintDataArray,
    mut value: *mut spTransformConstraintData,
) {
    if (*self_0).size == (*self_0).capacity {
        (*self_0).capacity = if 8 as c_int > ((*self_0).size as c_float * 1.75f32) as c_int {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0).items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<*mut spTransformConstraintData>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut *mut spTransformConstraintData;
    }
    let fresh209 = (*self_0).size;
    (*self_0).size = (*self_0).size + 1;
    let ref mut fresh210 = *((*self_0).items).offset(fresh209 as isize);
    *fresh210 = value;
}
#[no_mangle]
pub unsafe extern "C" fn spTransformConstraintDataArray_addAll(
    mut self_0: *mut spTransformConstraintDataArray,
    mut other: *mut spTransformConstraintDataArray,
) {
    let mut i: c_int = 0 as c_int;
    while i < (*other).size {
        spTransformConstraintDataArray_add(self_0, *((*other).items).offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spTransformConstraintDataArray_addAllValues(
    mut self_0: *mut spTransformConstraintDataArray,
    mut values: *mut *mut spTransformConstraintData,
    mut offset: c_int,
    mut count: c_int,
) {
    let mut i: c_int = offset;
    let mut n: c_int = offset + count;
    while i < n {
        spTransformConstraintDataArray_add(self_0, *values.offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spTransformConstraintDataArray_removeAt(
    mut self_0: *mut spTransformConstraintDataArray,
    mut index: c_int,
) {
    (*self_0).size -= 1;
    spine_memmove(
        ((*self_0).items).offset(index as isize) as *mut c_void,
        ((*self_0).items)
            .offset(index as isize)
            .offset(1 as c_int as isize) as *const c_void,
        (::core::mem::size_of::<*mut spTransformConstraintData>() as c_ulong)
            .wrapping_mul(((*self_0).size - index) as c_ulong),
    );
}
#[no_mangle]
pub unsafe extern "C" fn spTransformConstraintDataArray_contains(
    mut self_0: *mut spTransformConstraintDataArray,
    mut value: *mut spTransformConstraintData,
) -> c_int {
    let mut items: *mut *mut spTransformConstraintData = (*self_0).items;
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    i = 0 as c_int;
    n = (*self_0).size;
    while i < n {
        if *items.offset(i as isize) == value {
            return -(1 as c_int);
        }
        i += 1;
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spTransformConstraintDataArray_pop(
    mut self_0: *mut spTransformConstraintDataArray,
) -> *mut spTransformConstraintData {
    (*self_0).size -= 1;
    let mut item: *mut spTransformConstraintData =
        *((*self_0).items).offset((*self_0).size as isize);
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn spTransformConstraintDataArray_peek(
    mut self_0: *mut spTransformConstraintDataArray,
) -> *mut spTransformConstraintData {
    return *((*self_0).items).offset(((*self_0).size - 1 as c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn spPathConstraintDataArray_create(
    mut initialCapacity: c_int,
) -> *mut spPathConstraintDataArray {
    let mut array: *mut spPathConstraintDataArray = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spPathConstraintDataArray>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        12258 as c_int,
    ) as *mut spPathConstraintDataArray;
    (*array).size = 0 as c_int;
    (*array).capacity = initialCapacity;
    (*array).items = _spCalloc(
        initialCapacity as size_t,
        ::core::mem::size_of::<*mut spPathConstraintData>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        12258 as c_int,
    ) as *mut *mut spPathConstraintData;
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn spPathConstraintDataArray_dispose(
    mut self_0: *mut spPathConstraintDataArray,
) {
    _spFree((*self_0).items as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spPathConstraintDataArray_clear(
    mut self_0: *mut spPathConstraintDataArray,
) {
    (*self_0).size = 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spPathConstraintDataArray_setSize(
    mut self_0: *mut spPathConstraintDataArray,
    mut newSize: c_int,
) -> *mut spPathConstraintDataArray {
    (*self_0).size = newSize;
    if (*self_0).capacity < newSize {
        (*self_0).capacity = if 8 as c_int > ((*self_0).size as c_float * 1.75f32) as c_int {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0).items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<*mut spPathConstraintData>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut *mut spPathConstraintData;
    }
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spPathConstraintDataArray_ensureCapacity(
    mut self_0: *mut spPathConstraintDataArray,
    mut newCapacity: c_int,
) {
    if (*self_0).capacity >= newCapacity {
        return;
    }
    (*self_0).capacity = newCapacity;
    (*self_0).items = _spRealloc(
        (*self_0).items as *mut c_void,
        (::core::mem::size_of::<*mut spPathConstraintData>() as c_ulong)
            .wrapping_mul((*self_0).capacity as c_ulong),
    ) as *mut *mut spPathConstraintData;
}
#[no_mangle]
pub unsafe extern "C" fn spPathConstraintDataArray_add(
    mut self_0: *mut spPathConstraintDataArray,
    mut value: *mut spPathConstraintData,
) {
    if (*self_0).size == (*self_0).capacity {
        (*self_0).capacity = if 8 as c_int > ((*self_0).size as c_float * 1.75f32) as c_int {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0).items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<*mut spPathConstraintData>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut *mut spPathConstraintData;
    }
    let fresh211 = (*self_0).size;
    (*self_0).size = (*self_0).size + 1;
    let ref mut fresh212 = *((*self_0).items).offset(fresh211 as isize);
    *fresh212 = value;
}
#[no_mangle]
pub unsafe extern "C" fn spPathConstraintDataArray_addAll(
    mut self_0: *mut spPathConstraintDataArray,
    mut other: *mut spPathConstraintDataArray,
) {
    let mut i: c_int = 0 as c_int;
    while i < (*other).size {
        spPathConstraintDataArray_add(self_0, *((*other).items).offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spPathConstraintDataArray_addAllValues(
    mut self_0: *mut spPathConstraintDataArray,
    mut values: *mut *mut spPathConstraintData,
    mut offset: c_int,
    mut count: c_int,
) {
    let mut i: c_int = offset;
    let mut n: c_int = offset + count;
    while i < n {
        spPathConstraintDataArray_add(self_0, *values.offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spPathConstraintDataArray_removeAt(
    mut self_0: *mut spPathConstraintDataArray,
    mut index: c_int,
) {
    (*self_0).size -= 1;
    spine_memmove(
        ((*self_0).items).offset(index as isize) as *mut c_void,
        ((*self_0).items)
            .offset(index as isize)
            .offset(1 as c_int as isize) as *const c_void,
        (::core::mem::size_of::<*mut spPathConstraintData>() as c_ulong)
            .wrapping_mul(((*self_0).size - index) as c_ulong),
    );
}
#[no_mangle]
pub unsafe extern "C" fn spPathConstraintDataArray_contains(
    mut self_0: *mut spPathConstraintDataArray,
    mut value: *mut spPathConstraintData,
) -> c_int {
    let mut items: *mut *mut spPathConstraintData = (*self_0).items;
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    i = 0 as c_int;
    n = (*self_0).size;
    while i < n {
        if *items.offset(i as isize) == value {
            return -(1 as c_int);
        }
        i += 1;
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spPathConstraintDataArray_pop(
    mut self_0: *mut spPathConstraintDataArray,
) -> *mut spPathConstraintData {
    (*self_0).size -= 1;
    let mut item: *mut spPathConstraintData = *((*self_0).items).offset((*self_0).size as isize);
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn spPathConstraintDataArray_peek(
    mut self_0: *mut spPathConstraintDataArray,
) -> *mut spPathConstraintData {
    return *((*self_0).items).offset(((*self_0).size - 1 as c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn _Entry_create(
    mut slotIndex: c_int,
    mut name: *const c_char,
    mut attachment: *mut spAttachment,
) -> *mut _Entry {
    let mut self_0: *mut _Entry = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<_Entry>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        12261 as c_int,
    ) as *mut _Entry;
    (*self_0).slotIndex = slotIndex;
    let ref mut fresh213 = *(&mut (*self_0).name as *mut *const c_char as *mut *mut c_char);
    *fresh213 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul((spine_strlen(name)).wrapping_add(1 as c_int as c_ulong)),
        b"spine.c\0" as *const u8 as *const c_char,
        12263 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh213, name);
    (*self_0).attachment = attachment;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn _Entry_dispose(mut self_0: *mut _Entry) {
    spAttachment_dispose((*self_0).attachment);
    _spFree((*self_0).name as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
unsafe extern "C" fn _SkinHashTableEntry_create(
    mut entry: *mut _Entry,
) -> *mut _SkinHashTableEntry {
    let mut self_0: *mut _SkinHashTableEntry = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<_SkinHashTableEntry>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        12275 as c_int,
    ) as *mut _SkinHashTableEntry;
    (*self_0).entry = entry;
    return self_0;
}
unsafe extern "C" fn _SkinHashTableEntry_dispose(mut self_0: *mut _SkinHashTableEntry) {
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spSkin_create(mut name: *const c_char) -> *mut spSkin {
    let mut self_0: *mut spSkin = &mut (*((_spCalloc
        as unsafe extern "C" fn(size_t, size_t, *const c_char, c_int) -> *mut c_void)(
        1 as c_int as size_t,
        ::core::mem::size_of::<_spSkin>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        12287 as c_int,
    ) as *mut _spSkin))
        .super_0;
    let ref mut fresh214 = *(&(*self_0).name as *const *const c_char as *mut *mut c_char);
    *fresh214 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul((spine_strlen(name)).wrapping_add(1 as c_int as c_ulong)),
        b"spine.c\0" as *const u8 as *const c_char,
        12288 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh214, name);
    (*self_0).bones = spBoneDataArray_create(4 as c_int);
    (*self_0).ikConstraints = spIkConstraintDataArray_create(4 as c_int);
    (*self_0).transformConstraints = spTransformConstraintDataArray_create(4 as c_int);
    (*self_0).pathConstraints = spPathConstraintDataArray_create(4 as c_int);
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spSkin_dispose(mut self_0: *mut spSkin) {
    let mut entry: *mut _Entry = (*(self_0 as *mut _spSkin)).entries;
    while !entry.is_null() {
        let mut nextEntry: *mut _Entry = (*entry).next;
        _Entry_dispose(entry);
        entry = nextEntry;
    }
    let mut currentHashtableEntry: *mut *mut _SkinHashTableEntry =
        ((*(self_0 as *mut _spSkin)).entriesHashTable).as_mut_ptr();
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < 100 as c_int {
        let mut hashtableEntry: *mut _SkinHashTableEntry = *currentHashtableEntry;
        while !hashtableEntry.is_null() {
            let mut nextEntry_0: *mut _SkinHashTableEntry = (*hashtableEntry).next;
            _SkinHashTableEntry_dispose(hashtableEntry);
            hashtableEntry = nextEntry_0;
        }
        i += 1;
        currentHashtableEntry = currentHashtableEntry.offset(1);
    }
    spBoneDataArray_dispose((*self_0).bones);
    spIkConstraintDataArray_dispose((*self_0).ikConstraints);
    spTransformConstraintDataArray_dispose((*self_0).transformConstraints);
    spPathConstraintDataArray_dispose((*self_0).pathConstraints);
    _spFree((*self_0).name as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spSkin_setAttachment(
    mut self_0: *mut spSkin,
    mut slotIndex: c_int,
    mut name: *const c_char,
    mut attachment: *mut spAttachment,
) {
    let mut existingEntry: *mut _SkinHashTableEntry = 0 as *mut _SkinHashTableEntry;
    let mut hashEntry: *mut _SkinHashTableEntry = (*(self_0 as *mut _spSkin)).entriesHashTable
        [(slotIndex as c_uint).wrapping_rem(100 as c_int as c_uint) as usize];
    while !hashEntry.is_null() {
        if (*(*hashEntry).entry).slotIndex == slotIndex
            && spine_strcmp((*(*hashEntry).entry).name, name) == 0 as c_int
        {
            existingEntry = hashEntry;
            break;
        } else {
            hashEntry = (*hashEntry).next;
        }
    }
    if !attachment.is_null() {
        (*attachment).refCount += 1;
    }
    if !existingEntry.is_null() {
        if !((*(*hashEntry).entry).attachment).is_null() {
            spAttachment_dispose((*(*hashEntry).entry).attachment);
        }
        (*(*hashEntry).entry).attachment = attachment;
    } else {
        let mut newEntry: *mut _Entry = _Entry_create(slotIndex, name, attachment);
        (*newEntry).next = (*(self_0 as *mut _spSkin)).entries;
        let ref mut fresh215 = (*(self_0 as *mut _spSkin)).entries;
        *fresh215 = newEntry;
        let mut hashTableIndex: c_uint = (slotIndex as c_uint).wrapping_rem(100 as c_int as c_uint);
        let mut hashTable: *mut *mut _SkinHashTableEntry =
            ((*(self_0 as *mut _spSkin)).entriesHashTable).as_mut_ptr();
        let mut newHashEntry: *mut _SkinHashTableEntry = _SkinHashTableEntry_create(newEntry);
        (*newHashEntry).next = *hashTable.offset(hashTableIndex as isize);
        let ref mut fresh216 =
            (*(self_0 as *mut _spSkin)).entriesHashTable[hashTableIndex as usize];
        *fresh216 = newHashEntry;
    };
}
#[no_mangle]
pub unsafe extern "C" fn spSkin_getAttachment(
    mut self_0: *const spSkin,
    mut slotIndex: c_int,
    mut name: *const c_char,
) -> *mut spAttachment {
    let mut hashEntry: *const _SkinHashTableEntry = (*(self_0 as *mut _spSkin)).entriesHashTable
        [(slotIndex as c_uint).wrapping_rem(100 as c_int as c_uint) as usize];
    while !hashEntry.is_null() {
        if (*(*hashEntry).entry).slotIndex == slotIndex
            && spine_strcmp((*(*hashEntry).entry).name, name) == 0 as c_int
        {
            return (*(*hashEntry).entry).attachment;
        }
        hashEntry = (*hashEntry).next;
    }
    return 0 as *mut spAttachment;
}
#[no_mangle]
pub unsafe extern "C" fn spSkin_getAttachmentName(
    mut self_0: *const spSkin,
    mut slotIndex: c_int,
    mut attachmentIndex: c_int,
) -> *const c_char {
    let mut entry: *const _Entry = (*(self_0 as *mut _spSkin)).entries;
    let mut i: c_int = 0 as c_int;
    while !entry.is_null() {
        if (*entry).slotIndex == slotIndex {
            if i == attachmentIndex {
                return (*entry).name;
            }
            i += 1;
        }
        entry = (*entry).next;
    }
    return 0 as *const c_char;
}
#[no_mangle]
pub unsafe extern "C" fn spSkin_attachAll(
    mut self_0: *const spSkin,
    mut skeleton: *mut spSkeleton,
    mut oldSkin: *const spSkin,
) {
    let mut entry: *const _Entry = (*(oldSkin as *mut _spSkin)).entries;
    while !entry.is_null() {
        let mut slot: *mut spSlot = *((*skeleton).slots).offset((*entry).slotIndex as isize);
        if (*slot).attachment == (*entry).attachment {
            let mut attachment: *mut spAttachment =
                spSkin_getAttachment(self_0, (*entry).slotIndex, (*entry).name);
            if !attachment.is_null() {
                spSlot_setAttachment(slot, attachment);
            }
        }
        entry = (*entry).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spSkin_addSkin(mut self_0: *mut spSkin, mut other: *const spSkin) {
    let mut i: c_int = 0 as c_int;
    let mut entry: *mut spSkinEntry = 0 as *mut spSkinEntry;
    i = 0 as c_int;
    while i < (*(*other).bones).size {
        if spBoneDataArray_contains(
            (*self_0).bones,
            *((*(*other).bones).items).offset(i as isize),
        ) == 0
        {
            spBoneDataArray_add(
                (*self_0).bones,
                *((*(*other).bones).items).offset(i as isize),
            );
        }
        i += 1;
    }
    i = 0 as c_int;
    while i < (*(*other).ikConstraints).size {
        if spIkConstraintDataArray_contains(
            (*self_0).ikConstraints,
            *((*(*other).ikConstraints).items).offset(i as isize),
        ) == 0
        {
            spIkConstraintDataArray_add(
                (*self_0).ikConstraints,
                *((*(*other).ikConstraints).items).offset(i as isize),
            );
        }
        i += 1;
    }
    i = 0 as c_int;
    while i < (*(*other).transformConstraints).size {
        if spTransformConstraintDataArray_contains(
            (*self_0).transformConstraints,
            *((*(*other).transformConstraints).items).offset(i as isize),
        ) == 0
        {
            spTransformConstraintDataArray_add(
                (*self_0).transformConstraints,
                *((*(*other).transformConstraints).items).offset(i as isize),
            );
        }
        i += 1;
    }
    i = 0 as c_int;
    while i < (*(*other).pathConstraints).size {
        if spPathConstraintDataArray_contains(
            (*self_0).pathConstraints,
            *((*(*other).pathConstraints).items).offset(i as isize),
        ) == 0
        {
            spPathConstraintDataArray_add(
                (*self_0).pathConstraints,
                *((*(*other).pathConstraints).items).offset(i as isize),
            );
        }
        i += 1;
    }
    entry = spSkin_getAttachments(other);
    while !entry.is_null() {
        spSkin_setAttachment(
            self_0,
            (*entry).slotIndex,
            (*entry).name,
            (*entry).attachment,
        );
        entry = (*entry).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spSkin_copySkin(mut self_0: *mut spSkin, mut other: *const spSkin) {
    let mut i: c_int = 0 as c_int;
    let mut entry: *mut spSkinEntry = 0 as *mut spSkinEntry;
    i = 0 as c_int;
    while i < (*(*other).bones).size {
        if spBoneDataArray_contains(
            (*self_0).bones,
            *((*(*other).bones).items).offset(i as isize),
        ) == 0
        {
            spBoneDataArray_add(
                (*self_0).bones,
                *((*(*other).bones).items).offset(i as isize),
            );
        }
        i += 1;
    }
    i = 0 as c_int;
    while i < (*(*other).ikConstraints).size {
        if spIkConstraintDataArray_contains(
            (*self_0).ikConstraints,
            *((*(*other).ikConstraints).items).offset(i as isize),
        ) == 0
        {
            spIkConstraintDataArray_add(
                (*self_0).ikConstraints,
                *((*(*other).ikConstraints).items).offset(i as isize),
            );
        }
        i += 1;
    }
    i = 0 as c_int;
    while i < (*(*other).transformConstraints).size {
        if spTransformConstraintDataArray_contains(
            (*self_0).transformConstraints,
            *((*(*other).transformConstraints).items).offset(i as isize),
        ) == 0
        {
            spTransformConstraintDataArray_add(
                (*self_0).transformConstraints,
                *((*(*other).transformConstraints).items).offset(i as isize),
            );
        }
        i += 1;
    }
    i = 0 as c_int;
    while i < (*(*other).pathConstraints).size {
        if spPathConstraintDataArray_contains(
            (*self_0).pathConstraints,
            *((*(*other).pathConstraints).items).offset(i as isize),
        ) == 0
        {
            spPathConstraintDataArray_add(
                (*self_0).pathConstraints,
                *((*(*other).pathConstraints).items).offset(i as isize),
            );
        }
        i += 1;
    }
    entry = spSkin_getAttachments(other);
    while !entry.is_null() {
        if (*(*entry).attachment).type_0 as c_uint == SP_ATTACHMENT_MESH as c_int as c_uint {
            let mut attachment: *mut spMeshAttachment =
                spMeshAttachment_newLinkedMesh((*entry).attachment as *mut spMeshAttachment);
            spSkin_setAttachment(
                self_0,
                (*entry).slotIndex,
                (*entry).name,
                &mut (*attachment).super_0.super_0,
            );
        } else {
            let mut attachment_0: *mut spAttachment = if !((*entry).attachment).is_null() {
                spAttachment_copy((*entry).attachment)
            } else {
                0 as *mut spAttachment
            };
            spSkin_setAttachment(self_0, (*entry).slotIndex, (*entry).name, attachment_0);
        }
        entry = (*entry).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spSkin_getAttachments(mut self_0: *const spSkin) -> *mut spSkinEntry {
    return (*(self_0 as *mut _spSkin)).entries;
}
#[no_mangle]
pub unsafe extern "C" fn spSkin_clear(mut self_0: *mut spSkin) {
    let mut entry: *mut _Entry = (*(self_0 as *mut _spSkin)).entries;
    while !entry.is_null() {
        let mut nextEntry: *mut _Entry = (*entry).next;
        _Entry_dispose(entry);
        entry = nextEntry;
    }
    let ref mut fresh217 = (*(self_0 as *mut _spSkin)).entries;
    *fresh217 = 0 as *mut _Entry;
    let mut currentHashtableEntry: *mut *mut _SkinHashTableEntry =
        ((*(self_0 as *mut _spSkin)).entriesHashTable).as_mut_ptr();
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < 100 as c_int {
        let mut hashtableEntry: *mut _SkinHashTableEntry = *currentHashtableEntry;
        while !hashtableEntry.is_null() {
            let mut nextEntry_0: *mut _SkinHashTableEntry = (*hashtableEntry).next;
            _SkinHashTableEntry_dispose(hashtableEntry);
            hashtableEntry = nextEntry_0;
        }
        let ref mut fresh218 = (*(self_0 as *mut _spSkin)).entriesHashTable[i as usize];
        *fresh218 = 0 as *mut _SkinHashTableEntry;
        i += 1;
        currentHashtableEntry = currentHashtableEntry.offset(1);
    }
    spBoneDataArray_clear((*self_0).bones);
    spIkConstraintDataArray_clear((*self_0).ikConstraints);
    spTransformConstraintDataArray_clear((*self_0).transformConstraints);
    spPathConstraintDataArray_clear((*self_0).pathConstraints);
}
#[no_mangle]
pub unsafe extern "C" fn spSlot_create(
    mut data: *mut spSlotData,
    mut bone: *mut spBone,
) -> *mut spSlot {
    let mut self_0: *mut spSlot = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spSlot>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        12533 as c_int,
    ) as *mut spSlot;
    let ref mut fresh219 = *(&(*self_0).data as *const *mut spSlotData as *mut *mut spSlotData);
    *fresh219 = data;
    let ref mut fresh220 = *(&(*self_0).bone as *const *mut spBone as *mut *mut spBone);
    *fresh220 = bone;
    spColor_setFromFloats(
        &mut (*self_0).color,
        1 as c_int as c_float,
        1 as c_int as c_float,
        1 as c_int as c_float,
        1 as c_int as c_float,
    );
    (*self_0).darkColor = if ((*data).darkColor).is_null() {
        0 as *mut spColor
    } else {
        spColor_create()
    };
    spSlot_setToSetupPose(self_0);
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spSlot_dispose(mut self_0: *mut spSlot) {
    _spFree((*self_0).deform as *mut c_void);
    _spFree((*self_0).darkColor as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
unsafe extern "C" fn isVertexAttachment(mut attachment: *mut spAttachment) -> c_int {
    if attachment.is_null() {
        return 0 as c_int;
    }
    match (*attachment).type_0 as c_uint {
        1 | 6 | 2 | 4 => return -(1 as c_int),
        _ => return 0 as c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn spSlot_setAttachment(
    mut self_0: *mut spSlot,
    mut attachment: *mut spAttachment,
) {
    if attachment == (*self_0).attachment {
        return;
    }
    if isVertexAttachment(attachment) == 0
        || isVertexAttachment((*self_0).attachment) == 0
        || (*(attachment as *mut spVertexAttachment)).timelineAttachment
            != (*((*self_0).attachment as *mut spVertexAttachment)).timelineAttachment
    {
        (*self_0).deformCount = 0 as c_int;
    }
    let ref mut fresh221 = *(&mut (*self_0).attachment as *mut *mut spAttachment);
    *fresh221 = attachment;
    (*self_0).sequenceIndex = -(1 as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn spSlot_setToSetupPose(mut self_0: *mut spSlot) {
    spColor_setFromColor(&mut (*self_0).color, &mut (*(*self_0).data).color);
    if !((*self_0).darkColor).is_null() {
        spColor_setFromColor((*self_0).darkColor, (*(*self_0).data).darkColor);
    }
    if ((*(*self_0).data).attachmentName).is_null() {
        spSlot_setAttachment(self_0, 0 as *mut spAttachment);
    } else {
        let mut attachment: *mut spAttachment = spSkeleton_getAttachmentForSlotIndex(
            (*(*self_0).bone).skeleton,
            (*(*self_0).data).index,
            (*(*self_0).data).attachmentName,
        );
        let ref mut fresh222 = *(&mut (*self_0).attachment as *mut *mut spAttachment);
        *fresh222 = 0 as *mut spAttachment;
        spSlot_setAttachment(self_0, attachment);
    };
}
#[no_mangle]
pub unsafe extern "C" fn spSlotData_create(
    index: c_int,
    mut name: *const c_char,
    mut boneData: *mut spBoneData,
) -> *mut spSlotData {
    let mut self_0: *mut spSlotData = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spSlotData>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        12619 as c_int,
    ) as *mut spSlotData;
    *(&(*self_0).index as *const c_int as *mut c_int) = index;
    let ref mut fresh223 = *(&(*self_0).name as *const *const c_char as *mut *mut c_char);
    *fresh223 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul((spine_strlen(name)).wrapping_add(1 as c_int as c_ulong)),
        b"spine.c\0" as *const u8 as *const c_char,
        12621 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh223, name);
    let ref mut fresh224 =
        *(&(*self_0).boneData as *const *const spBoneData as *mut *mut spBoneData);
    *fresh224 = boneData;
    spColor_setFromFloats(
        &mut (*self_0).color,
        1 as c_int as c_float,
        1 as c_int as c_float,
        1 as c_int as c_float,
        1 as c_int as c_float,
    );
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spSlotData_dispose(mut self_0: *mut spSlotData) {
    _spFree((*self_0).name as *mut c_void);
    _spFree((*self_0).attachmentName as *mut c_void);
    _spFree((*self_0).darkColor as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spSlotData_setAttachmentName(
    mut self_0: *mut spSlotData,
    mut attachmentName: *const c_char,
) {
    _spFree((*self_0).attachmentName as *mut c_void);
    if !attachmentName.is_null() {
        let ref mut fresh225 =
            *(&mut (*self_0).attachmentName as *mut *const c_char as *mut *mut c_char);
        *fresh225 = _spMalloc(
            (::core::mem::size_of::<c_char>() as c_ulong)
                .wrapping_mul((spine_strlen(attachmentName)).wrapping_add(1 as c_int as c_ulong)),
            b"spine.c\0" as *const u8 as *const c_char,
            12637 as c_int,
        ) as *mut c_char;
        spine_strcpy(*fresh225, attachmentName);
    } else {
        let ref mut fresh226 =
            *(&mut (*self_0).attachmentName as *mut *const c_char as *mut *mut c_char);
        *fresh226 = 0 as *mut c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn spTransformConstraint_create(
    mut data: *mut spTransformConstraintData,
    mut skeleton: *const spSkeleton,
) -> *mut spTransformConstraint {
    let mut i: c_int = 0;
    let mut self_0: *mut spTransformConstraint = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spTransformConstraint>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        12676 as c_int,
    ) as *mut spTransformConstraint;
    let ref mut fresh227 = *(&(*self_0).data as *const *mut spTransformConstraintData
        as *mut *mut spTransformConstraintData);
    *fresh227 = data;
    (*self_0).mixRotate = (*data).mixRotate;
    (*self_0).mixX = (*data).mixX;
    (*self_0).mixY = (*data).mixY;
    (*self_0).mixScaleX = (*data).mixScaleX;
    (*self_0).mixScaleY = (*data).mixScaleY;
    (*self_0).mixShearY = (*data).mixShearY;
    (*self_0).bonesCount = (*data).bonesCount;
    let ref mut fresh228 = *(&(*self_0).bones as *const *mut *mut spBone as *mut *mut *mut spBone);
    *fresh228 = _spMalloc(
        (::core::mem::size_of::<*mut spBone>() as c_ulong)
            .wrapping_mul((*self_0).bonesCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        12685 as c_int,
    ) as *mut *mut spBone;
    i = 0 as c_int;
    while i < (*self_0).bonesCount {
        let ref mut fresh229 = *((*self_0).bones).offset(i as isize);
        *fresh229 = spSkeleton_findBone(
            skeleton,
            (**((*(*self_0).data).bones).offset(i as isize)).name,
        );
        i += 1;
    }
    (*self_0).target = spSkeleton_findBone(skeleton, (*(*(*self_0).data).target).name);
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spTransformConstraint_dispose(mut self_0: *mut spTransformConstraint) {
    _spFree((*self_0).bones as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _spTransformConstraint_applyAbsoluteWorld(
    mut self_0: *mut spTransformConstraint,
) {
    let mut mixRotate: c_float = (*self_0).mixRotate;
    let mut mixX: c_float = (*self_0).mixX;
    let mut mixY: c_float = (*self_0).mixY;
    let mut mixScaleX: c_float = (*self_0).mixScaleX;
    let mut mixScaleY: c_float = (*self_0).mixScaleY;
    let mut mixShearY: c_float = (*self_0).mixShearY;
    let mut translate: c_int =
        (mixX != 0 as c_int as c_float || mixY != 0 as c_int as c_float) as c_int;
    let mut target: *mut spBone = (*self_0).target;
    let mut ta: c_float = (*target).a;
    let mut tb: c_float = (*target).b;
    let mut tc: c_float = (*target).c;
    let mut td: c_float = (*target).d;
    let mut degRadReflect: c_float = if ta * td - tb * tc > 0 as c_int as c_float {
        3.1415926535897932385f32 / 180 as c_int as c_float
    } else {
        -(3.1415926535897932385f32 / 180 as c_int as c_float)
    };
    let mut offsetRotation: c_float = (*(*self_0).data).offsetRotation * degRadReflect;
    let mut offsetShearY: c_float = (*(*self_0).data).offsetShearY * degRadReflect;
    let mut i: c_int = 0;
    let mut a: c_float = 0.;
    let mut b: c_float = 0.;
    let mut c: c_float = 0.;
    let mut d: c_float = 0.;
    let mut r: c_float = 0.;
    let mut cosine: c_float = 0.;
    let mut sine: c_float = 0.;
    let mut x: c_float = 0.;
    let mut y: c_float = 0.;
    let mut s: c_float = 0.;
    let mut by: c_float = 0.;
    i = 0 as c_int;
    while i < (*self_0).bonesCount {
        let mut bone: *mut spBone = *((*self_0).bones).offset(i as isize);
        if mixRotate != 0 as c_int as c_float {
            a = (*bone).a;
            b = (*bone).b;
            c = (*bone).c;
            d = (*bone).d;
            r = atan2f(tc, ta) - atan2f(c, a) + offsetRotation;
            if r > 3.1415926535897932385f32 {
                r -= 3.1415926535897932385f32 * 2 as c_int as c_float;
            } else if r < -3.1415926535897932385f32 {
                r += 3.1415926535897932385f32 * 2 as c_int as c_float;
            }
            r *= mixRotate;
            cosine = cosf(r);
            sine = sinf(r);
            *(&(*bone).a as *const c_float as *mut c_float) = cosine * a - sine * c;
            *(&(*bone).b as *const c_float as *mut c_float) = cosine * b - sine * d;
            *(&(*bone).c as *const c_float as *mut c_float) = sine * a + cosine * c;
            *(&(*bone).d as *const c_float as *mut c_float) = sine * b + cosine * d;
        }
        if translate != 0 {
            spBone_localToWorld(
                target,
                (*(*self_0).data).offsetX,
                (*(*self_0).data).offsetY,
                &mut x,
                &mut y,
            );
            *(&(*bone).worldX as *const c_float as *mut c_float) += (x - (*bone).worldX) * mixX;
            *(&(*bone).worldY as *const c_float as *mut c_float) += (y - (*bone).worldY) * mixY;
        }
        if mixScaleX > 0 as c_int as c_float {
            s = spine_sqrtf((*bone).a * (*bone).a + (*bone).c * (*bone).c);
            if s != 0 as c_int as c_float {
                s = (s
                    + (spine_sqrtf(ta * ta + tc * tc) - s + (*(*self_0).data).offsetScaleX)
                        * mixScaleX)
                    / s;
            }
            *(&(*bone).a as *const c_float as *mut c_float) *= s;
            *(&(*bone).c as *const c_float as *mut c_float) *= s;
        }
        if mixScaleY != 0 as c_int as c_float {
            s = spine_sqrtf((*bone).b * (*bone).b + (*bone).d * (*bone).d);
            if s != 0 as c_int as c_float {
                s = (s
                    + (spine_sqrtf(tb * tb + td * td) - s + (*(*self_0).data).offsetScaleY)
                        * mixScaleY)
                    / s;
            }
            *(&(*bone).b as *const c_float as *mut c_float) *= s;
            *(&(*bone).d as *const c_float as *mut c_float) *= s;
        }
        if mixShearY > 0 as c_int as c_float {
            b = (*bone).b;
            d = (*bone).d;
            by = atan2f(d, b);
            r = atan2f(td, tb) - atan2f(tc, ta) - (by - atan2f((*bone).c, (*bone).a));
            s = spine_sqrtf(b * b + d * d);
            if r > 3.1415926535897932385f32 {
                r -= 3.1415926535897932385f32 * 2 as c_int as c_float;
            } else if r < -3.1415926535897932385f32 {
                r += 3.1415926535897932385f32 * 2 as c_int as c_float;
            }
            r = by + (r + offsetShearY) * mixShearY;
            *(&(*bone).b as *const c_float as *mut c_float) = cosf(r) * s;
            *(&(*bone).d as *const c_float as *mut c_float) = sinf(r) * s;
        }
        spBone_updateAppliedTransform(bone);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _spTransformConstraint_applyRelativeWorld(
    mut self_0: *mut spTransformConstraint,
) {
    let mut mixRotate: c_float = (*self_0).mixRotate;
    let mut mixX: c_float = (*self_0).mixX;
    let mut mixY: c_float = (*self_0).mixY;
    let mut mixScaleX: c_float = (*self_0).mixScaleX;
    let mut mixScaleY: c_float = (*self_0).mixScaleY;
    let mut mixShearY: c_float = (*self_0).mixShearY;
    let mut translate: c_int =
        (mixX != 0 as c_int as c_float || mixY != 0 as c_int as c_float) as c_int;
    let mut target: *mut spBone = (*self_0).target;
    let mut ta: c_float = (*target).a;
    let mut tb: c_float = (*target).b;
    let mut tc: c_float = (*target).c;
    let mut td: c_float = (*target).d;
    let mut degRadReflect: c_float = if ta * td - tb * tc > 0 as c_int as c_float {
        3.1415926535897932385f32 / 180 as c_int as c_float
    } else {
        -(3.1415926535897932385f32 / 180 as c_int as c_float)
    };
    let mut offsetRotation: c_float = (*(*self_0).data).offsetRotation * degRadReflect;
    let mut offsetShearY: c_float = (*(*self_0).data).offsetShearY * degRadReflect;
    let mut i: c_int = 0;
    let mut a: c_float = 0.;
    let mut b: c_float = 0.;
    let mut c: c_float = 0.;
    let mut d: c_float = 0.;
    let mut r: c_float = 0.;
    let mut cosine: c_float = 0.;
    let mut sine: c_float = 0.;
    let mut x: c_float = 0.;
    let mut y: c_float = 0.;
    let mut s: c_float = 0.;
    i = 0 as c_int;
    while i < (*self_0).bonesCount {
        let mut bone: *mut spBone = *((*self_0).bones).offset(i as isize);
        if mixRotate != 0 as c_int as c_float {
            a = (*bone).a;
            b = (*bone).b;
            c = (*bone).c;
            d = (*bone).d;
            r = atan2f(tc, ta) + offsetRotation;
            if r > 3.1415926535897932385f32 {
                r -= 3.1415926535897932385f32 * 2 as c_int as c_float;
            } else if r < -3.1415926535897932385f32 {
                r += 3.1415926535897932385f32 * 2 as c_int as c_float;
            }
            r *= mixRotate;
            cosine = cosf(r);
            sine = sinf(r);
            *(&(*bone).a as *const c_float as *mut c_float) = cosine * a - sine * c;
            *(&(*bone).b as *const c_float as *mut c_float) = cosine * b - sine * d;
            *(&(*bone).c as *const c_float as *mut c_float) = sine * a + cosine * c;
            *(&(*bone).d as *const c_float as *mut c_float) = sine * b + cosine * d;
        }
        if translate != 0 as c_int {
            spBone_localToWorld(
                target,
                (*(*self_0).data).offsetX,
                (*(*self_0).data).offsetY,
                &mut x,
                &mut y,
            );
            *(&(*bone).worldX as *const c_float as *mut c_float) += x * mixX;
            *(&(*bone).worldY as *const c_float as *mut c_float) += y * mixY;
        }
        if mixScaleX != 0 as c_int as c_float {
            s = (spine_sqrtf(ta * ta + tc * tc) - 1 as c_int as c_float
                + (*(*self_0).data).offsetScaleX)
                * mixScaleX
                + 1 as c_int as c_float;
            *(&(*bone).a as *const c_float as *mut c_float) *= s;
            *(&(*bone).c as *const c_float as *mut c_float) *= s;
        }
        if mixScaleY > 0 as c_int as c_float {
            s = (spine_sqrtf(tb * tb + td * td) - 1 as c_int as c_float
                + (*(*self_0).data).offsetScaleY)
                * mixScaleY
                + 1 as c_int as c_float;
            *(&(*bone).b as *const c_float as *mut c_float) *= s;
            *(&(*bone).d as *const c_float as *mut c_float) *= s;
        }
        if mixShearY > 0 as c_int as c_float {
            r = atan2f(td, tb) - atan2f(tc, ta);
            if r > 3.1415926535897932385f32 {
                r -= 3.1415926535897932385f32 * 2 as c_int as c_float;
            } else if r < -3.1415926535897932385f32 {
                r += 3.1415926535897932385f32 * 2 as c_int as c_float;
            }
            b = (*bone).b;
            d = (*bone).d;
            r = atan2f(d, b)
                + (r - 3.1415926535897932385f32 / 2 as c_int as c_float + offsetShearY) * mixShearY;
            s = spine_sqrtf(b * b + d * d);
            *(&(*bone).b as *const c_float as *mut c_float) = cosf(r) * s;
            *(&(*bone).d as *const c_float as *mut c_float) = sinf(r) * s;
        }
        spBone_updateAppliedTransform(bone);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _spTransformConstraint_applyAbsoluteLocal(
    mut self_0: *mut spTransformConstraint,
) {
    let mut mixRotate: c_float = (*self_0).mixRotate;
    let mut mixX: c_float = (*self_0).mixX;
    let mut mixY: c_float = (*self_0).mixY;
    let mut mixScaleX: c_float = (*self_0).mixScaleX;
    let mut mixScaleY: c_float = (*self_0).mixScaleY;
    let mut mixShearY: c_float = (*self_0).mixShearY;
    let mut target: *mut spBone = (*self_0).target;
    let mut i: c_int = 0;
    let mut rotation: c_float = 0.;
    let mut r: c_float = 0.;
    let mut x: c_float = 0.;
    let mut y: c_float = 0.;
    let mut scaleX: c_float = 0.;
    let mut scaleY: c_float = 0.;
    let mut shearY: c_float = 0.;
    i = 0 as c_int;
    while i < (*self_0).bonesCount {
        let mut bone: *mut spBone = *((*self_0).bones).offset(i as isize);
        rotation = (*bone).arotation;
        if mixRotate != 0 as c_int as c_float {
            r = (*target).arotation - rotation + (*(*self_0).data).offsetRotation;
            r -= ((16384 as c_int
                - (16384.499999999996f64 - (r / 360 as c_int as c_float) as c_double) as c_int)
                * 360 as c_int) as c_float;
            rotation += r * mixRotate;
        }
        x = (*bone).ax;
        y = (*bone).ay;
        x += ((*target).ax - x + (*(*self_0).data).offsetX) * mixX;
        y += ((*target).ay - y + (*(*self_0).data).offsetY) * mixY;
        scaleX = (*bone).ascaleX;
        scaleY = (*bone).ascaleY;
        if mixScaleX != 0 as c_int as c_float && scaleX != 0 as c_int as c_float {
            scaleX = (scaleX
                + ((*target).ascaleX - scaleX + (*(*self_0).data).offsetScaleX) * mixScaleX)
                / scaleX;
        }
        if mixScaleY != 0 as c_int as c_float && scaleY != 0 as c_int as c_float {
            scaleY = (scaleY
                + ((*target).ascaleY - scaleY + (*(*self_0).data).offsetScaleY) * mixScaleY)
                / scaleY;
        }
        shearY = (*bone).ashearY;
        if mixShearY != 0 as c_int as c_float {
            r = (*target).ashearY - shearY + (*(*self_0).data).offsetShearY;
            r -= ((16384 as c_int
                - (16384.499999999996f64 - (r / 360 as c_int as c_float) as c_double) as c_int)
                * 360 as c_int) as c_float;
            shearY += r * mixShearY;
        }
        spBone_updateWorldTransformWith(
            bone,
            x,
            y,
            rotation,
            scaleX,
            scaleY,
            (*bone).ashearX,
            shearY,
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _spTransformConstraint_applyRelativeLocal(
    mut self_0: *mut spTransformConstraint,
) {
    let mut mixRotate: c_float = (*self_0).mixRotate;
    let mut mixX: c_float = (*self_0).mixX;
    let mut mixY: c_float = (*self_0).mixY;
    let mut mixScaleX: c_float = (*self_0).mixScaleX;
    let mut mixScaleY: c_float = (*self_0).mixScaleY;
    let mut mixShearY: c_float = (*self_0).mixShearY;
    let mut target: *mut spBone = (*self_0).target;
    let mut i: c_int = 0;
    let mut rotation: c_float = 0.;
    let mut x: c_float = 0.;
    let mut y: c_float = 0.;
    let mut scaleX: c_float = 0.;
    let mut scaleY: c_float = 0.;
    let mut shearY: c_float = 0.;
    i = 0 as c_int;
    while i < (*self_0).bonesCount {
        let mut bone: *mut spBone = *((*self_0).bones).offset(i as isize);
        rotation = (*bone).arotation
            + ((*target).arotation + (*(*self_0).data).offsetRotation) * mixRotate;
        x = (*bone).ax + ((*target).ax + (*(*self_0).data).offsetX) * mixX;
        y = (*bone).ay + ((*target).ay + (*(*self_0).data).offsetY) * mixY;
        scaleX = (*bone).ascaleX
            * (((*target).ascaleX - 1 as c_int as c_float + (*(*self_0).data).offsetScaleX)
                * mixScaleX
                + 1 as c_int as c_float);
        scaleY = (*bone).ascaleY
            * (((*target).ascaleY - 1 as c_int as c_float + (*(*self_0).data).offsetScaleY)
                * mixScaleY
                + 1 as c_int as c_float);
        shearY = (*bone).ashearY + ((*target).ashearY + (*(*self_0).data).offsetShearY) * mixShearY;
        spBone_updateWorldTransformWith(
            bone,
            x,
            y,
            rotation,
            scaleX,
            scaleY,
            (*bone).ashearX,
            shearY,
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spTransformConstraint_update(mut self_0: *mut spTransformConstraint) {
    if (*self_0).mixRotate == 0 as c_int as c_float
        && (*self_0).mixX == 0 as c_int as c_float
        && (*self_0).mixY == 0 as c_int as c_float
        && (*self_0).mixScaleX == 0 as c_int as c_float
        && (*self_0).mixScaleX == 0 as c_int as c_float
        && (*self_0).mixShearY == 0 as c_int as c_float
    {
        return;
    }
    if (*(*self_0).data).local != 0 {
        if (*(*self_0).data).relative != 0 {
            _spTransformConstraint_applyRelativeLocal(self_0);
        } else {
            _spTransformConstraint_applyAbsoluteLocal(self_0);
        }
    } else if (*(*self_0).data).relative != 0 {
        _spTransformConstraint_applyRelativeWorld(self_0);
    } else {
        _spTransformConstraint_applyAbsoluteWorld(self_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn spTransformConstraintData_create(
    mut name: *const c_char,
) -> *mut spTransformConstraintData {
    let mut self_0: *mut spTransformConstraintData = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spTransformConstraintData>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        12933 as c_int,
    ) as *mut spTransformConstraintData;
    let ref mut fresh230 = *(&(*self_0).name as *const *const c_char as *mut *mut c_char);
    *fresh230 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul((spine_strlen(name)).wrapping_add(1 as c_int as c_ulong)),
        b"spine.c\0" as *const u8 as *const c_char,
        12934 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh230, name);
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spTransformConstraintData_dispose(
    mut self_0: *mut spTransformConstraintData,
) {
    _spFree((*self_0).name as *mut c_void);
    _spFree((*self_0).bones as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spTriangulator_create() -> *mut spTriangulator {
    let mut triangulator: *mut spTriangulator = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spTriangulator>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        12977 as c_int,
    ) as *mut spTriangulator;
    (*triangulator).convexPolygons = spArrayFloatArray_create(16 as c_int);
    (*triangulator).convexPolygonsIndices = spArrayShortArray_create(16 as c_int);
    (*triangulator).indicesArray = spShortArray_create(128 as c_int);
    (*triangulator).isConcaveArray = spIntArray_create(128 as c_int);
    (*triangulator).triangles = spShortArray_create(128 as c_int);
    (*triangulator).polygonPool = spArrayFloatArray_create(16 as c_int);
    (*triangulator).polygonIndicesPool = spArrayShortArray_create(128 as c_int);
    return triangulator;
}
#[no_mangle]
pub unsafe extern "C" fn spTriangulator_dispose(mut self_0: *mut spTriangulator) {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*(*self_0).convexPolygons).size {
        spFloatArray_dispose(*((*(*self_0).convexPolygons).items).offset(i as isize));
        i += 1;
    }
    spArrayFloatArray_dispose((*self_0).convexPolygons);
    i = 0 as c_int;
    while i < (*(*self_0).convexPolygonsIndices).size {
        spShortArray_dispose(*((*(*self_0).convexPolygonsIndices).items).offset(i as isize));
        i += 1;
    }
    spArrayShortArray_dispose((*self_0).convexPolygonsIndices);
    spShortArray_dispose((*self_0).indicesArray);
    spIntArray_dispose((*self_0).isConcaveArray);
    spShortArray_dispose((*self_0).triangles);
    i = 0 as c_int;
    while i < (*(*self_0).polygonPool).size {
        spFloatArray_dispose(*((*(*self_0).polygonPool).items).offset(i as isize));
        i += 1;
    }
    spArrayFloatArray_dispose((*self_0).polygonPool);
    i = 0 as c_int;
    while i < (*(*self_0).polygonIndicesPool).size {
        spShortArray_dispose(*((*(*self_0).polygonIndicesPool).items).offset(i as isize));
        i += 1;
    }
    spArrayShortArray_dispose((*self_0).polygonIndicesPool);
    _spFree(self_0 as *mut c_void);
}
unsafe extern "C" fn _obtainPolygon(mut self_0: *mut spTriangulator) -> *mut spFloatArray {
    if (*(*self_0).polygonPool).size == 0 as c_int {
        return spFloatArray_create(16 as c_int);
    } else {
        return spArrayFloatArray_pop((*self_0).polygonPool);
    };
}
unsafe extern "C" fn _freePolygon(mut self_0: *mut spTriangulator, mut polygon: *mut spFloatArray) {
    spArrayFloatArray_add((*self_0).polygonPool, polygon);
}
unsafe extern "C" fn _freeAllPolygons(
    mut self_0: *mut spTriangulator,
    mut polygons: *mut spArrayFloatArray,
) {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*polygons).size {
        _freePolygon(self_0, *((*polygons).items).offset(i as isize));
        i += 1;
    }
}
unsafe extern "C" fn _obtainPolygonIndices(mut self_0: *mut spTriangulator) -> *mut spShortArray {
    if (*(*self_0).polygonIndicesPool).size == 0 as c_int {
        return spShortArray_create(16 as c_int);
    } else {
        return spArrayShortArray_pop((*self_0).polygonIndicesPool);
    };
}
unsafe extern "C" fn _freePolygonIndices(
    mut self_0: *mut spTriangulator,
    mut indices: *mut spShortArray,
) {
    spArrayShortArray_add((*self_0).polygonIndicesPool, indices);
}
unsafe extern "C" fn _freeAllPolygonIndices(
    mut self_0: *mut spTriangulator,
    mut polygonIndices: *mut spArrayShortArray,
) {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*polygonIndices).size {
        _freePolygonIndices(self_0, *((*polygonIndices).items).offset(i as isize));
        i += 1;
    }
}
unsafe extern "C" fn _positiveArea(
    mut p1x: c_float,
    mut p1y: c_float,
    mut p2x: c_float,
    mut p2y: c_float,
    mut p3x: c_float,
    mut p3y: c_float,
) -> c_int {
    return (p1x * (p3y - p2y) + p2x * (p1y - p3y) + p3x * (p2y - p1y) >= 0 as c_int as c_float)
        as c_int;
}
unsafe extern "C" fn _isConcave(
    mut index: c_int,
    mut vertexCount: c_int,
    mut vertices: *mut c_float,
    mut indices: *mut c_short,
) -> c_int {
    let mut previous: c_int =
        (*indices.offset(((vertexCount + index - 1 as c_int) % vertexCount) as isize) as c_int)
            << 1 as c_int;
    let mut current: c_int = (*indices.offset(index as isize) as c_int) << 1 as c_int;
    let mut next: c_int =
        (*indices.offset(((index + 1 as c_int) % vertexCount) as isize) as c_int) << 1 as c_int;
    return (_positiveArea(
        *vertices.offset(previous as isize),
        *vertices.offset((previous + 1 as c_int) as isize),
        *vertices.offset(current as isize),
        *vertices.offset((current + 1 as c_int) as isize),
        *vertices.offset(next as isize),
        *vertices.offset((next + 1 as c_int) as isize),
    ) == 0) as c_int;
}
unsafe extern "C" fn _winding(
    mut p1x: c_float,
    mut p1y: c_float,
    mut p2x: c_float,
    mut p2y: c_float,
    mut p3x: c_float,
    mut p3y: c_float,
) -> c_int {
    let mut px: c_float = p2x - p1x;
    let mut py: c_float = p2y - p1y;
    return if p3x * py - p3y * px + px * p1y - p1x * py >= 0 as c_int as c_float {
        1 as c_int
    } else {
        -(1 as c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn spTriangulator_triangulate(
    mut self_0: *mut spTriangulator,
    mut verticesArray: *mut spFloatArray,
) -> *mut spShortArray {
    let mut vertices: *mut c_float = (*verticesArray).items;
    let mut vertexCount: c_int = (*verticesArray).size >> 1 as c_int;
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    let mut ii: c_int = 0;
    let mut indicesArray: *mut spShortArray = (*self_0).indicesArray;
    let mut indices: *mut c_short = 0 as *mut c_short;
    let mut isConcaveArray: *mut spIntArray = 0 as *mut spIntArray;
    let mut isConcave: *mut c_int = 0 as *mut c_int;
    let mut triangles: *mut spShortArray = 0 as *mut spShortArray;
    spShortArray_clear(indicesArray);
    indices = (*spShortArray_setSize(indicesArray, vertexCount)).items;
    i = 0 as c_int;
    while i < vertexCount {
        *indices.offset(i as isize) = i as c_short;
        i += 1;
    }
    isConcaveArray = (*self_0).isConcaveArray;
    isConcave = (*spIntArray_setSize(isConcaveArray, vertexCount)).items;
    i = 0 as c_int;
    n = vertexCount;
    while i < n {
        *isConcave.offset(i as isize) = _isConcave(i, vertexCount, vertices, indices);
        i += 1;
    }
    triangles = (*self_0).triangles;
    spShortArray_clear(triangles);
    spShortArray_ensureCapacity(
        triangles,
        (if 0 as c_int > vertexCount - 2 as c_int {
            0 as c_int
        } else {
            vertexCount - 2 as c_int
        }) << 2 as c_int,
    );
    while vertexCount > 3 as c_int {
        let mut previous: c_int = vertexCount - 1 as c_int;
        let mut next: c_int = 1 as c_int;
        let mut previousIndex: c_int = 0;
        let mut nextIndex: c_int = 0;
        i = 0 as c_int;
        's_80: loop {
            if *isConcave.offset(i as isize) == 0 {
                let mut p1: c_int = (*indices.offset(previous as isize) as c_int) << 1 as c_int;
                let mut p2: c_int = (*indices.offset(i as isize) as c_int) << 1 as c_int;
                let mut p3: c_int = (*indices.offset(next as isize) as c_int) << 1 as c_int;
                let mut p1x: c_float = *vertices.offset(p1 as isize);
                let mut p1y: c_float = *vertices.offset((p1 + 1 as c_int) as isize);
                let mut p2x: c_float = *vertices.offset(p2 as isize);
                let mut p2y: c_float = *vertices.offset((p2 + 1 as c_int) as isize);
                let mut p3x: c_float = *vertices.offset(p3 as isize);
                let mut p3y: c_float = *vertices.offset((p3 + 1 as c_int) as isize);
                ii = (next + 1 as c_int) % vertexCount;
                loop {
                    if !(ii != previous) {
                        break 's_80;
                    }
                    let mut v: c_int = 0;
                    let mut vx: c_float = 0.;
                    let mut vy: c_float = 0.;
                    if !(*isConcave.offset(ii as isize) == 0) {
                        v = (*indices.offset(ii as isize) as c_int) << 1 as c_int;
                        vx = *vertices.offset(v as isize);
                        vy = *vertices.offset((v + 1 as c_int) as isize);
                        if _positiveArea(p3x, p3y, p1x, p1y, vx, vy) != 0 {
                            if _positiveArea(p1x, p1y, p2x, p2y, vx, vy) != 0 {
                                if _positiveArea(p2x, p2y, p3x, p3y, vx, vy) != 0 {
                                    break;
                                }
                            }
                        }
                    }
                    ii = (ii + 1 as c_int) % vertexCount;
                }
            }
            if next == 0 as c_int {
                while !(*isConcave.offset(i as isize) == 0) {
                    i -= 1;
                    if !(i > 0 as c_int) {
                        break;
                    }
                }
                break;
            } else {
                previous = i;
                i = next;
                next = (next + 1 as c_int) % vertexCount;
            }
        }
        spShortArray_add(
            triangles,
            *indices.offset(((vertexCount + i - 1 as c_int) % vertexCount) as isize),
        );
        spShortArray_add(triangles, *indices.offset(i as isize));
        spShortArray_add(
            triangles,
            *indices.offset(((i + 1 as c_int) % vertexCount) as isize),
        );
        spShortArray_removeAt(indicesArray, i);
        spIntArray_removeAt(isConcaveArray, i);
        vertexCount -= 1;
        previousIndex = (vertexCount + i - 1 as c_int) % vertexCount;
        nextIndex = if i == vertexCount { 0 as c_int } else { i };
        *isConcave.offset(previousIndex as isize) =
            _isConcave(previousIndex, vertexCount, vertices, indices);
        *isConcave.offset(nextIndex as isize) =
            _isConcave(nextIndex, vertexCount, vertices, indices);
    }
    if vertexCount == 3 as c_int {
        spShortArray_add(triangles, *indices.offset(2 as c_int as isize));
        spShortArray_add(triangles, *indices.offset(0 as c_int as isize));
        spShortArray_add(triangles, *indices.offset(1 as c_int as isize));
    }
    return triangles;
}
#[no_mangle]
pub unsafe extern "C" fn spTriangulator_decompose(
    mut self_0: *mut spTriangulator,
    mut verticesArray: *mut spFloatArray,
    mut triangles: *mut spShortArray,
) -> *mut spArrayFloatArray {
    let mut vertices: *mut c_float = (*verticesArray).items;
    let mut convexPolygons: *mut spArrayFloatArray = (*self_0).convexPolygons;
    let mut convexPolygonsIndices: *mut spArrayShortArray = 0 as *mut spArrayShortArray;
    let mut polygonIndices: *mut spShortArray = 0 as *mut spShortArray;
    let mut polygon: *mut spFloatArray = 0 as *mut spFloatArray;
    let mut fanBaseIndex: c_int = 0;
    let mut lastWinding: c_int = 0;
    let mut trianglesItems: *mut c_short = 0 as *mut c_short;
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    _freeAllPolygons(self_0, convexPolygons);
    spArrayFloatArray_clear(convexPolygons);
    convexPolygonsIndices = (*self_0).convexPolygonsIndices;
    _freeAllPolygonIndices(self_0, convexPolygonsIndices);
    spArrayShortArray_clear(convexPolygonsIndices);
    polygonIndices = _obtainPolygonIndices(self_0);
    spShortArray_clear(polygonIndices);
    polygon = _obtainPolygon(self_0);
    spFloatArray_clear(polygon);
    fanBaseIndex = -(1 as c_int);
    lastWinding = 0 as c_int;
    trianglesItems = (*triangles).items;
    i = 0 as c_int;
    n = (*triangles).size;
    while i < n {
        let mut t1: c_int = (*trianglesItems.offset(i as isize) as c_int) << 1 as c_int;
        let mut t2: c_int =
            (*trianglesItems.offset((i + 1 as c_int) as isize) as c_int) << 1 as c_int;
        let mut t3: c_int =
            (*trianglesItems.offset((i + 2 as c_int) as isize) as c_int) << 1 as c_int;
        let mut x1: c_float = *vertices.offset(t1 as isize);
        let mut y1: c_float = *vertices.offset((t1 + 1 as c_int) as isize);
        let mut x2: c_float = *vertices.offset(t2 as isize);
        let mut y2: c_float = *vertices.offset((t2 + 1 as c_int) as isize);
        let mut x3: c_float = *vertices.offset(t3 as isize);
        let mut y3: c_float = *vertices.offset((t3 + 1 as c_int) as isize);
        let mut merged: c_int = 0 as c_int;
        if fanBaseIndex == t1 {
            let mut o: c_int = (*polygon).size - 4 as c_int;
            let mut p: *mut c_float = (*polygon).items;
            let mut winding1: c_int = _winding(
                *p.offset(o as isize),
                *p.offset((o + 1 as c_int) as isize),
                *p.offset((o + 2 as c_int) as isize),
                *p.offset((o + 3 as c_int) as isize),
                x3,
                y3,
            );
            let mut winding2: c_int = _winding(
                x3,
                y3,
                *p.offset(0 as c_int as isize),
                *p.offset(1 as c_int as isize),
                *p.offset(2 as c_int as isize),
                *p.offset(3 as c_int as isize),
            );
            if winding1 == lastWinding && winding2 == lastWinding {
                spFloatArray_add(polygon, x3);
                spFloatArray_add(polygon, y3);
                spShortArray_add(polygonIndices, t3 as c_short);
                merged = 1 as c_int;
            }
        }
        if merged == 0 {
            if (*polygon).size > 0 as c_int {
                spArrayFloatArray_add(convexPolygons, polygon);
                spArrayShortArray_add(convexPolygonsIndices, polygonIndices);
            } else {
                _freePolygon(self_0, polygon);
                _freePolygonIndices(self_0, polygonIndices);
            }
            polygon = _obtainPolygon(self_0);
            spFloatArray_clear(polygon);
            spFloatArray_add(polygon, x1);
            spFloatArray_add(polygon, y1);
            spFloatArray_add(polygon, x2);
            spFloatArray_add(polygon, y2);
            spFloatArray_add(polygon, x3);
            spFloatArray_add(polygon, y3);
            polygonIndices = _obtainPolygonIndices(self_0);
            spShortArray_clear(polygonIndices);
            spShortArray_add(polygonIndices, t1 as c_short);
            spShortArray_add(polygonIndices, t2 as c_short);
            spShortArray_add(polygonIndices, t3 as c_short);
            lastWinding = _winding(x1, y1, x2, y2, x3, y3);
            fanBaseIndex = t1;
        }
        i += 3 as c_int;
    }
    if (*polygon).size > 0 as c_int {
        spArrayFloatArray_add(convexPolygons, polygon);
        spArrayShortArray_add(convexPolygonsIndices, polygonIndices);
    }
    i = 0 as c_int;
    n = (*convexPolygons).size;
    while i < n {
        let mut firstIndex: c_int = 0;
        let mut lastIndex: c_int = 0;
        let mut o_0: c_int = 0;
        let mut p_0: *mut c_float = 0 as *mut c_float;
        let mut prevPrevX: c_float = 0.;
        let mut prevPrevY: c_float = 0.;
        let mut prevX: c_float = 0.;
        let mut prevY: c_float = 0.;
        let mut firstX: c_float = 0.;
        let mut firstY: c_float = 0.;
        let mut secondX: c_float = 0.;
        let mut secondY: c_float = 0.;
        let mut winding: c_int = 0;
        let mut ii: c_int = 0;
        polygonIndices = *((*convexPolygonsIndices).items).offset(i as isize);
        if !((*polygonIndices).size == 0 as c_int) {
            firstIndex = *((*polygonIndices).items).offset(0 as c_int as isize) as c_int;
            lastIndex = *((*polygonIndices).items)
                .offset(((*polygonIndices).size - 1 as c_int) as isize)
                as c_int;
            polygon = *((*convexPolygons).items).offset(i as isize);
            o_0 = (*polygon).size - 4 as c_int;
            p_0 = (*polygon).items;
            prevPrevX = *p_0.offset(o_0 as isize);
            prevPrevY = *p_0.offset((o_0 + 1 as c_int) as isize);
            prevX = *p_0.offset((o_0 + 2 as c_int) as isize);
            prevY = *p_0.offset((o_0 + 3 as c_int) as isize);
            firstX = *p_0.offset(0 as c_int as isize);
            firstY = *p_0.offset(1 as c_int as isize);
            secondX = *p_0.offset(2 as c_int as isize);
            secondY = *p_0.offset(3 as c_int as isize);
            winding = _winding(prevPrevX, prevPrevY, prevX, prevY, firstX, firstY);
            ii = 0 as c_int;
            while ii < n {
                let mut otherIndices: *mut spShortArray = 0 as *mut spShortArray;
                let mut otherFirstIndex: c_int = 0;
                let mut otherSecondIndex: c_int = 0;
                let mut otherLastIndex: c_int = 0;
                let mut otherPoly: *mut spFloatArray = 0 as *mut spFloatArray;
                let mut x3_0: c_float = 0.;
                let mut y3_0: c_float = 0.;
                let mut winding1_0: c_int = 0;
                let mut winding2_0: c_int = 0;
                if !(ii == i) {
                    otherIndices = *((*convexPolygonsIndices).items).offset(ii as isize);
                    if !((*otherIndices).size != 3 as c_int) {
                        otherFirstIndex =
                            *((*otherIndices).items).offset(0 as c_int as isize) as c_int;
                        otherSecondIndex =
                            *((*otherIndices).items).offset(1 as c_int as isize) as c_int;
                        otherLastIndex =
                            *((*otherIndices).items).offset(2 as c_int as isize) as c_int;
                        otherPoly = *((*convexPolygons).items).offset(ii as isize);
                        x3_0 =
                            *((*otherPoly).items).offset(((*otherPoly).size - 2 as c_int) as isize);
                        y3_0 =
                            *((*otherPoly).items).offset(((*otherPoly).size - 1 as c_int) as isize);
                        if !(otherFirstIndex != firstIndex || otherSecondIndex != lastIndex) {
                            winding1_0 = _winding(prevPrevX, prevPrevY, prevX, prevY, x3_0, y3_0);
                            winding2_0 = _winding(x3_0, y3_0, firstX, firstY, secondX, secondY);
                            if winding1_0 == winding && winding2_0 == winding {
                                spFloatArray_clear(otherPoly);
                                spShortArray_clear(otherIndices);
                                spFloatArray_add(polygon, x3_0);
                                spFloatArray_add(polygon, y3_0);
                                spShortArray_add(polygonIndices, otherLastIndex as c_short);
                                prevPrevX = prevX;
                                prevPrevY = prevY;
                                prevX = x3_0;
                                prevY = y3_0;
                                ii = 0 as c_int;
                            }
                        }
                    }
                }
                ii += 1;
            }
        }
        i += 1;
    }
    i = (*convexPolygons).size - 1 as c_int;
    while i >= 0 as c_int {
        polygon = *((*convexPolygons).items).offset(i as isize);
        if (*polygon).size == 0 as c_int {
            spArrayFloatArray_removeAt(convexPolygons, i);
            _freePolygon(self_0, polygon);
            polygonIndices = *((*convexPolygonsIndices).items).offset(i as isize);
            spArrayShortArray_removeAt(convexPolygonsIndices, i);
            _freePolygonIndices(self_0, polygonIndices);
        }
        i -= 1;
    }
    return convexPolygons;
}
static mut nextID: c_int = 0 as c_int;
#[no_mangle]
pub unsafe extern "C" fn _spVertexAttachment_init(mut attachment: *mut spVertexAttachment) {
    let fresh231 = nextID;
    nextID = nextID + 1;
    (*attachment).id = fresh231;
    (*attachment).timelineAttachment = &mut (*attachment).super_0;
}
#[no_mangle]
pub unsafe extern "C" fn _spVertexAttachment_deinit(mut attachment: *mut spVertexAttachment) {
    _spAttachment_deinit(&mut (*attachment).super_0);
    _spFree((*attachment).bones as *mut c_void);
    _spFree((*attachment).vertices as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spVertexAttachment_computeWorldVertices(
    mut self_0: *mut spVertexAttachment,
    mut slot: *mut spSlot,
    mut start: c_int,
    mut count: c_int,
    mut worldVertices: *mut c_float,
    mut offset: c_int,
    mut stride: c_int,
) {
    let mut skeleton: *mut spSkeleton = 0 as *mut spSkeleton;
    let mut deformLength: c_int = 0;
    let mut deformArray: *mut c_float = 0 as *mut c_float;
    let mut vertices: *mut c_float = 0 as *mut c_float;
    let mut bones: *mut c_int = 0 as *mut c_int;
    if (*self_0).super_0.type_0 as c_uint == SP_ATTACHMENT_MESH as c_int as c_uint
        || (*self_0).super_0.type_0 as c_uint == SP_ATTACHMENT_LINKED_MESH as c_int as c_uint
    {
        let mut mesh: *mut spMeshAttachment = self_0 as *mut spMeshAttachment;
        if !((*mesh).sequence).is_null() {
            spSequence_apply((*mesh).sequence, slot, &mut (*self_0).super_0);
        }
    }
    count = offset + (count >> 1 as c_int) * stride;
    skeleton = (*(*slot).bone).skeleton;
    deformLength = (*slot).deformCount;
    deformArray = (*slot).deform;
    vertices = (*self_0).vertices;
    bones = (*self_0).bones;
    if bones.is_null() {
        let mut bone: *mut spBone = 0 as *mut spBone;
        let mut v: c_int = 0;
        let mut w: c_int = 0;
        let mut x: c_float = 0.;
        let mut y: c_float = 0.;
        if deformLength > 0 as c_int {
            vertices = deformArray;
        }
        bone = (*slot).bone;
        x = (*bone).worldX;
        y = (*bone).worldY;
        v = start;
        w = offset;
        while w < count {
            let mut vx: c_float = *vertices.offset(v as isize);
            let mut vy: c_float = *vertices.offset((v + 1 as c_int) as isize);
            *worldVertices.offset(w as isize) = vx * (*bone).a + vy * (*bone).b + x;
            *worldVertices.offset((w + 1 as c_int) as isize) = vx * (*bone).c + vy * (*bone).d + y;
            v += 2 as c_int;
            w += stride;
        }
    } else {
        let mut v_0: c_int = 0 as c_int;
        let mut skip_0: c_int = 0 as c_int;
        let mut i: c_int = 0;
        let mut skeletonBones: *mut *mut spBone = 0 as *mut *mut spBone;
        i = 0 as c_int;
        while i < start {
            let mut n: c_int = *bones.offset(v_0 as isize);
            v_0 += n + 1 as c_int;
            skip_0 += n;
            i += 2 as c_int;
        }
        skeletonBones = (*skeleton).bones;
        if deformLength == 0 as c_int {
            let mut w_0: c_int = 0;
            let mut b: c_int = 0;
            w_0 = offset;
            b = skip_0 * 3 as c_int;
            while w_0 < count {
                let mut wx: c_float = 0 as c_int as c_float;
                let mut wy: c_float = 0 as c_int as c_float;
                let fresh232 = v_0;
                v_0 = v_0 + 1;
                let mut n_0: c_int = *bones.offset(fresh232 as isize);
                n_0 += v_0;
                while v_0 < n_0 {
                    let mut bone_0: *mut spBone =
                        *skeletonBones.offset(*bones.offset(v_0 as isize) as isize);
                    let mut vx_0: c_float = *vertices.offset(b as isize);
                    let mut vy_0: c_float = *vertices.offset((b + 1 as c_int) as isize);
                    let mut weight: c_float = *vertices.offset((b + 2 as c_int) as isize);
                    wx += (vx_0 * (*bone_0).a + vy_0 * (*bone_0).b + (*bone_0).worldX) * weight;
                    wy += (vx_0 * (*bone_0).c + vy_0 * (*bone_0).d + (*bone_0).worldY) * weight;
                    v_0 += 1;
                    b += 3 as c_int;
                }
                *worldVertices.offset(w_0 as isize) = wx;
                *worldVertices.offset((w_0 + 1 as c_int) as isize) = wy;
                w_0 += stride;
            }
        } else {
            let mut w_1: c_int = 0;
            let mut b_0: c_int = 0;
            let mut f: c_int = 0;
            w_1 = offset;
            b_0 = skip_0 * 3 as c_int;
            f = skip_0 << 1 as c_int;
            while w_1 < count {
                let mut wx_0: c_float = 0 as c_int as c_float;
                let mut wy_0: c_float = 0 as c_int as c_float;
                let fresh233 = v_0;
                v_0 = v_0 + 1;
                let mut n_1: c_int = *bones.offset(fresh233 as isize);
                n_1 += v_0;
                while v_0 < n_1 {
                    let mut bone_1: *mut spBone =
                        *skeletonBones.offset(*bones.offset(v_0 as isize) as isize);
                    let mut vx_1: c_float =
                        *vertices.offset(b_0 as isize) + *deformArray.offset(f as isize);
                    let mut vy_1: c_float = *vertices.offset((b_0 + 1 as c_int) as isize)
                        + *deformArray.offset((f + 1 as c_int) as isize);
                    let mut weight_0: c_float = *vertices.offset((b_0 + 2 as c_int) as isize);
                    wx_0 += (vx_1 * (*bone_1).a + vy_1 * (*bone_1).b + (*bone_1).worldX) * weight_0;
                    wy_0 += (vx_1 * (*bone_1).c + vy_1 * (*bone_1).d + (*bone_1).worldY) * weight_0;
                    v_0 += 1;
                    b_0 += 3 as c_int;
                    f += 2 as c_int;
                }
                *worldVertices.offset(w_1 as isize) = wx_0;
                *worldVertices.offset((w_1 + 1 as c_int) as isize) = wy_0;
                w_1 += stride;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn spVertexAttachment_copyTo(
    mut from: *mut spVertexAttachment,
    mut to: *mut spVertexAttachment,
) {
    if (*from).bonesCount != 0 {
        (*to).bonesCount = (*from).bonesCount;
        (*to).bones = _spMalloc(
            (::core::mem::size_of::<c_int>() as c_ulong)
                .wrapping_mul((*from).bonesCount as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            13438 as c_int,
        ) as *mut c_int;
        spine_memcpy(
            (*to).bones as *mut c_void,
            (*from).bones as *const c_void,
            ((*from).bonesCount as c_ulong)
                .wrapping_mul(::core::mem::size_of::<c_int>() as c_ulong),
        );
    } else {
        (*to).bonesCount = 0 as c_int;
        if !((*to).bones).is_null() {
            _spFree((*to).bones as *mut c_void);
            (*to).bones = 0 as *mut c_int;
        }
    }
    if (*from).verticesCount != 0 {
        (*to).verticesCount = (*from).verticesCount;
        (*to).vertices = _spMalloc(
            (::core::mem::size_of::<c_float>() as c_ulong)
                .wrapping_mul((*from).verticesCount as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            13450 as c_int,
        ) as *mut c_float;
        spine_memcpy(
            (*to).vertices as *mut c_void,
            (*from).vertices as *const c_void,
            ((*from).verticesCount as c_ulong)
                .wrapping_mul(::core::mem::size_of::<c_float>() as c_ulong),
        );
    } else {
        (*to).verticesCount = 0 as c_int;
        if !((*to).vertices).is_null() {
            _spFree((*to).vertices as *mut c_void);
            (*to).vertices = 0 as *mut c_float;
        }
    }
    (*to).worldVerticesLength = (*from).worldVerticesLength;
}
#[no_mangle]
pub unsafe extern "C" fn _spInternalRandom() -> c_float {
    return spine_rand() as c_float / 2147483647 as c_int as c_float;
}
static mut mallocFunc: Option<unsafe extern "C" fn(size_t) -> *mut c_void> =
    Some(spine_malloc as unsafe extern "C" fn(size_t) -> *mut c_void);
static mut reallocFunc: Option<unsafe extern "C" fn(*mut c_void, size_t) -> *mut c_void> =
    Some(spine_realloc as unsafe extern "C" fn(*mut c_void, size_t) -> *mut c_void);
static mut debugMallocFunc: Option<
    unsafe extern "C" fn(size_t, *const c_char, c_int) -> *mut c_void,
> = None;
static mut freeFunc: Option<unsafe extern "C" fn(*mut c_void) -> ()> =
    Some(spine_free as unsafe extern "C" fn(*mut c_void) -> ());
static mut randomFunc: Option<unsafe extern "C" fn() -> c_float> = unsafe {
    Some(core::mem::transmute::<
        unsafe extern "C" fn() -> c_float,
        unsafe extern "C" fn() -> c_float,
    >(_spInternalRandom))
};
#[no_mangle]
pub unsafe extern "C" fn _spMalloc(
    mut size: size_t,
    mut file: *const c_char,
    mut line: c_int,
) -> *mut c_void {
    if debugMallocFunc.is_some() {
        return debugMallocFunc.expect("non-null function pointer")(size, file, line);
    }
    return mallocFunc.expect("non-null function pointer")(size);
}
#[no_mangle]
pub unsafe extern "C" fn _spCalloc(
    mut num: size_t,
    mut size: size_t,
    mut file: *const c_char,
    mut line: c_int,
) -> *mut c_void {
    let mut ptr: *mut c_void = _spMalloc(num.wrapping_mul(size), file, line);
    if !ptr.is_null() {
        spine_memset(ptr, 0 as c_int, num.wrapping_mul(size));
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn _spRealloc(mut ptr: *mut c_void, mut size: size_t) -> *mut c_void {
    return reallocFunc.expect("non-null function pointer")(ptr, size);
}
#[no_mangle]
pub unsafe extern "C" fn _spFree(mut ptr: *mut c_void) {
    freeFunc.expect("non-null function pointer")(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn _spRandom() -> c_float {
    return ::core::mem::transmute::<_, fn() -> c_float>(
        randomFunc.expect("non-null function pointer"),
    )();
}
#[no_mangle]
pub unsafe extern "C" fn _spSetDebugMalloc(
    mut spine_malloc_0: Option<unsafe extern "C" fn(size_t, *const c_char, c_int) -> *mut c_void>,
) {
    debugMallocFunc = spine_malloc_0;
}
#[no_mangle]
pub unsafe extern "C" fn _spSetMalloc(
    mut spine_malloc_0: Option<unsafe extern "C" fn(size_t) -> *mut c_void>,
) {
    mallocFunc = spine_malloc_0;
}
#[no_mangle]
pub unsafe extern "C" fn _spSetRealloc(
    mut spine_realloc_0: Option<unsafe extern "C" fn(*mut c_void, size_t) -> *mut c_void>,
) {
    reallocFunc = spine_realloc_0;
}
#[no_mangle]
pub unsafe extern "C" fn _spSetFree(
    mut spine_free_0: Option<unsafe extern "C" fn(*mut c_void) -> ()>,
) {
    freeFunc = spine_free_0;
}
#[no_mangle]
pub unsafe extern "C" fn _spSetRandom(mut random: Option<unsafe extern "C" fn() -> c_float>) {
    randomFunc = random;
}
#[no_mangle]
pub unsafe extern "C" fn _spReadFile(
    mut path: *const c_char,
    mut length: *mut c_int,
) -> *mut c_char {
    let mut data: *mut c_char = 0 as *mut c_char;
    let mut _result: size_t = 0;
    let mut file: *mut FILE = spine_fopen(path, b"rb\0" as *const u8 as *const c_char);
    if file.is_null() {
        return 0 as *mut c_char;
    }
    spine_fseek(file, 0 as c_int as c_long, 2 as c_int);
    *length = spine_ftell(file) as c_int;
    spine_fseek(file, 0 as c_int as c_long, 0 as c_int);
    data = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong).wrapping_mul(*length as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        13562 as c_int,
    ) as *mut c_char;
    _result = spine_fread(
        data as *mut c_void,
        1 as c_int as size_t,
        *length as size_t,
        file,
    );
    spine_fclose(file);
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn _spMath_random(mut min: c_float, mut max: c_float) -> c_float {
    return min + (max - min) * _spRandom();
}
#[no_mangle]
pub unsafe extern "C" fn _spMath_randomTriangular(mut min: c_float, mut max: c_float) -> c_float {
    return _spMath_randomTriangularWith(min, max, (min + max) * 0.5f32);
}
#[no_mangle]
pub unsafe extern "C" fn _spMath_randomTriangularWith(
    mut min: c_float,
    mut max: c_float,
    mut mode: c_float,
) -> c_float {
    let mut u: c_float = _spRandom();
    let mut d: c_float = max - min;
    if u <= (mode - min) / d {
        return min + spine_sqrtf(u * d * (mode - min));
    }
    return max - spine_sqrtf((1 as c_int as c_float - u) * d * (max - mode));
}
#[no_mangle]
pub unsafe extern "C" fn _spMath_interpolate(
    mut apply: Option<unsafe extern "C" fn(c_float) -> c_float>,
    mut start: c_float,
    mut end: c_float,
    mut a: c_float,
) -> c_float {
    return start + (end - start) * apply.expect("non-null function pointer")(a);
}
#[no_mangle]
pub unsafe extern "C" fn _spMath_pow2_apply(mut a: c_float) -> c_float {
    if a as c_double <= 0.5f64 {
        return (pow(
            (a * 2 as c_int as c_float) as c_double,
            2 as c_int as c_double,
        ) / 2 as c_int as c_double) as c_float;
    }
    return (pow(
        ((a - 1 as c_int as c_float) * 2 as c_int as c_float) as c_double,
        2 as c_int as c_double,
    ) / -(2 as c_int) as c_double
        + 1 as c_int as c_double) as c_float;
}
#[no_mangle]
pub unsafe extern "C" fn _spMath_pow2out_apply(mut a: c_float) -> c_float {
    return (pow(
        (a - 1 as c_int as c_float) as c_double,
        2 as c_int as c_double,
    ) * -(1 as c_int) as c_double
        + 1 as c_int as c_double) as c_float;
}

type _IO_wide_data = u8;
type _IO_codecvt = u8;
type _IO_marker = u8;
pub use crate::c::environment::types::*;
