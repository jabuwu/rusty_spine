#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    
    
    
    fn spine_memcpy(
        __dest: *mut c_void,
        __src: *const c_void,
        __n: size_t,
    ) -> *mut c_void;
    fn spine_memmove(
        __dest: *mut c_void,
        __src: *const c_void,
        __n: size_t,
    ) -> *mut c_void;
    fn acosf(_: c_float) -> c_float;
    fn atan2f(_: c_float, _: c_float) -> c_float;
    fn spine_memset(
        __s: *mut c_void,
        __c: c_int,
        __n: size_t,
    ) -> *mut c_void;
    fn cosf(_: c_float) -> c_float;
    fn sinf(_: c_float) -> c_float;
    fn spine_strcasecmp(
        __s1: *const c_char,
        __s2: *const c_char,
    ) -> c_int;
    fn spine_strcpy(
        __dest: *mut c_char,
        __src: *const c_char,
    ) -> *mut c_char;
    fn spine_strncat(
        __dest: *mut c_char,
        __src: *const c_char,
        __n: size_t,
    ) -> *mut c_char;
    fn spine_strcmp(__s1: *const c_char, __s2: *const c_char) -> c_int;
    fn spine_strncmp(
        __s1: *const c_char,
        __s2: *const c_char,
        __n: size_t,
    ) -> c_int;
    fn pow(_: c_double, _: c_double) -> c_double;
    fn spine_sqrtf(__x: c_float) -> c_float;
    fn _spAtlasPage_createTexture(self_0: *mut spAtlasPage, path: *const c_char);
    fn _spAtlasPage_disposeTexture(self_0: *mut spAtlasPage);
    fn _spUtil_readFile(
        path: *const c_char,
        length: *mut c_int,
    ) -> *mut c_char;
    fn fmodf(_: c_float, _: c_float) -> c_float;
    fn spine_strtol(
        __nptr: *const c_char,
        __endptr: *mut *mut c_char,
        __base: c_int,
    ) -> c_long;
    fn spine_strtoul(
        __nptr: *const c_char,
        __endptr: *mut *mut c_char,
        __base: c_int,
    ) -> c_ulong;
    fn spine_fclose(__stream: *mut FILE) -> c_int;
    fn spine_fopen(
        __filename: *const c_char,
        __modes: *const c_char,
    ) -> *mut FILE;
    fn spine_strrchr(__s: *const c_char, __c: c_int) -> *mut c_char;
    
    fn spine_strlen(__s: *const c_char) -> size_t;
    
    fn spine_rand() -> c_int;
    fn spine_malloc(__size: size_t) -> *mut c_void;
    fn spine_realloc(__ptr: *mut c_void, __size: size_t) -> *mut c_void;
    fn spine_free(__ptr: *mut c_void);
    fn spine_fread(
        __ptr: *mut c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn spine_fseek(
        __stream: *mut FILE,
        __off: c_long,
        __whence: c_int,
    ) -> c_int;
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
    pub dispose: Option::<unsafe extern "C" fn(*mut spAttachment) -> ()>,
    pub copy: Option::<unsafe extern "C" fn(*mut spAttachment) -> *mut spAttachment>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spTimeline {
    pub type_0: spTimelineType,
    pub vtable: *const c_void,
}
pub type spTimelineType = c_uint;
pub const SP_TIMELINE_TWOCOLOR: spTimelineType = 14;
pub const SP_TIMELINE_PATHCONSTRAINTMIX: spTimelineType = 13;
pub const SP_TIMELINE_PATHCONSTRAINTSPACING: spTimelineType = 12;
pub const SP_TIMELINE_PATHCONSTRAINTPOSITION: spTimelineType = 11;
pub const SP_TIMELINE_TRANSFORMCONSTRAINT: spTimelineType = 10;
pub const SP_TIMELINE_IKCONSTRAINT: spTimelineType = 9;
pub const SP_TIMELINE_DRAWORDER: spTimelineType = 8;
pub const SP_TIMELINE_EVENT: spTimelineType = 7;
pub const SP_TIMELINE_DEFORM: spTimelineType = 6;
pub const SP_TIMELINE_COLOR: spTimelineType = 5;
pub const SP_TIMELINE_ATTACHMENT: spTimelineType = 4;
pub const SP_TIMELINE_SHEAR: spTimelineType = 3;
pub const SP_TIMELINE_SCALE: spTimelineType = 2;
pub const SP_TIMELINE_TRANSLATE: spTimelineType = 1;
pub const SP_TIMELINE_ROTATE: spTimelineType = 0;
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
    pub time: c_float,
    pub scaleX: c_float,
    pub scaleY: c_float,
    pub x: c_float,
    pub y: c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spColor {
    pub r: c_float,
    pub g: c_float,
    pub b: c_float,
    pub a: c_float,
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
    pub rotateMix: c_float,
    pub translateMix: c_float,
}
pub type spRotateMode = c_uint;
pub const SP_ROTATE_MODE_CHAIN_SCALE: spRotateMode = 2;
pub const SP_ROTATE_MODE_CHAIN: spRotateMode = 1;
pub const SP_ROTATE_MODE_TANGENT: spRotateMode = 0;
pub type spSpacingMode = c_uint;
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
}
pub type spTransformMode = c_uint;
pub const SP_TRANSFORMMODE_NOSCALEORREFLECTION: spTransformMode = 4;
pub const SP_TRANSFORMMODE_NOSCALE: spTransformMode = 3;
pub const SP_TRANSFORMMODE_NOROTATIONORREFLECTION: spTransformMode = 2;
pub const SP_TRANSFORMMODE_ONLYTRANSLATION: spTransformMode = 1;
pub const SP_TRANSFORMMODE_NORMAL: spTransformMode = 0;
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
    pub rotateMix: c_float,
    pub translateMix: c_float,
    pub scaleMix: c_float,
    pub shearMix: c_float,
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
    pub rotateMix: c_float,
    pub translateMix: c_float,
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
    pub appliedValid: c_int,
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
    pub rotateMix: c_float,
    pub translateMix: c_float,
    pub scaleMix: c_float,
    pub shearMix: c_float,
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
    pub timelinesCount: c_int,
    pub timelines: *mut *mut spTimeline,
}
pub type spMixBlend = c_uint;
pub const SP_MIX_BLEND_ADD: spMixBlend = 3;
pub const SP_MIX_BLEND_REPLACE: spMixBlend = 2;
pub const SP_MIX_BLEND_FIRST: spMixBlend = 1;
pub const SP_MIX_BLEND_SETUP: spMixBlend = 0;
pub type spMixDirection = c_uint;
pub const SP_MIX_DIRECTION_OUT: spMixDirection = 1;
pub const SP_MIX_DIRECTION_IN: spMixDirection = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _spTimelineVtable {
    pub apply: Option::<
        unsafe extern "C" fn(
            *const spTimeline,
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
    pub getPropertyId: Option::<unsafe extern "C" fn(*const spTimeline) -> c_int>,
    pub dispose: Option::<unsafe extern "C" fn(*mut spTimeline) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spCurveTimeline {
    pub super_0: spTimeline,
    pub curves: *mut c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spBaseTimeline {
    pub super_0: spCurveTimeline,
    pub framesCount: c_int,
    pub frames: *mut c_float,
    pub boneIndex: c_int,
}
pub type spRotateTimeline = spBaseTimeline;
pub type spTranslateTimeline = spBaseTimeline;
pub type spScaleTimeline = spBaseTimeline;
pub type spShearTimeline = spBaseTimeline;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spColorTimeline {
    pub super_0: spCurveTimeline,
    pub framesCount: c_int,
    pub frames: *mut c_float,
    pub slotIndex: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spTwoColorTimeline {
    pub super_0: spCurveTimeline,
    pub framesCount: c_int,
    pub frames: *mut c_float,
    pub slotIndex: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spAttachmentTimeline {
    pub super_0: spTimeline,
    pub framesCount: c_int,
    pub frames: *mut c_float,
    pub slotIndex: c_int,
    pub attachmentNames: *mut *const c_char,
}
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
pub struct _spSlot {
    pub super_0: spSlot,
    pub attachmentTime: c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spEventTimeline {
    pub super_0: spTimeline,
    pub framesCount: c_int,
    pub frames: *mut c_float,
    pub events: *mut *mut spEvent,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spDrawOrderTimeline {
    pub super_0: spTimeline,
    pub framesCount: c_int,
    pub frames: *mut c_float,
    pub drawOrders: *mut *const c_int,
    pub slotsCount: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spDeformTimeline {
    pub super_0: spCurveTimeline,
    pub framesCount: c_int,
    pub frames: *mut c_float,
    pub frameVerticesCount: c_int,
    pub frameVertices: *mut *const c_float,
    pub slotIndex: c_int,
    pub attachment: *mut spAttachment,
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
    pub deformAttachment: *mut spVertexAttachment,
    pub id: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spIkConstraintTimeline {
    pub super_0: spCurveTimeline,
    pub framesCount: c_int,
    pub frames: *mut c_float,
    pub ikConstraintIndex: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spTransformConstraintTimeline {
    pub super_0: spCurveTimeline,
    pub framesCount: c_int,
    pub frames: *mut c_float,
    pub transformConstraintIndex: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spPathConstraintPositionTimeline {
    pub super_0: spCurveTimeline,
    pub framesCount: c_int,
    pub frames: *mut c_float,
    pub pathConstraintIndex: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spPathConstraintSpacingTimeline {
    pub super_0: spCurveTimeline,
    pub framesCount: c_int,
    pub frames: *mut c_float,
    pub pathConstraintIndex: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spPathConstraintMixTimeline {
    pub super_0: spCurveTimeline,
    pub framesCount: c_int,
    pub frames: *mut c_float,
    pub pathConstraintIndex: c_int,
}
pub type __off_t = c_long;
pub type __off64_t = c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spFloatArray {
    pub size: c_int,
    pub capacity: c_int,
    pub items: *mut c_float,
}
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
pub type spSkinEntry = _Entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spMeshAttachment {
    pub super_0: spVertexAttachment,
    pub rendererObject: *mut c_void,
    pub regionOffsetX: c_int,
    pub regionOffsetY: c_int,
    pub regionWidth: c_int,
    pub regionHeight: c_int,
    pub regionOriginalWidth: c_int,
    pub regionOriginalHeight: c_int,
    pub regionU: c_float,
    pub regionV: c_float,
    pub regionU2: c_float,
    pub regionV2: c_float,
    pub regionRotate: c_int,
    pub regionDegrees: c_int,
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
pub struct _spAttachmentLoaderVtable {
    pub createAttachment: Option::<
        unsafe extern "C" fn(
            *mut spAttachmentLoader,
            *mut spSkin,
            spAttachmentType,
            *const c_char,
            *const c_char,
        ) -> *mut spAttachment,
    >,
    pub configureAttachment: Option::<
        unsafe extern "C" fn(*mut spAttachmentLoader, *mut spAttachment) -> (),
    >,
    pub disposeAttachment: Option::<
        unsafe extern "C" fn(*mut spAttachmentLoader, *mut spAttachment) -> (),
    >,
    pub dispose: Option::<unsafe extern "C" fn(*mut spAttachmentLoader) -> ()>,
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
    pub name: *const c_char,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub u: c_float,
    pub v: c_float,
    pub u2: c_float,
    pub v2: c_float,
    pub offsetX: c_int,
    pub offsetY: c_int,
    pub originalWidth: c_int,
    pub originalHeight: c_int,
    pub index: c_int,
    pub rotate: c_int,
    pub degrees: c_int,
    pub flip: c_int,
    pub splits: *mut c_int,
    pub pads: *mut c_int,
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
pub struct Str {
    pub begin: *const c_char,
    pub end: *const c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spPathAttachment {
    pub super_0: spVertexAttachment,
    pub lengthsLength: c_int,
    pub lengths: *mut c_float,
    pub closed: c_int,
    pub constantSpeed: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _spSkeleton {
    pub super_0: spSkeleton,
    pub updateCacheCount: c_int,
    pub updateCacheCapacity: c_int,
    pub updateCache: *mut _spUpdate,
    pub updateCacheResetCount: c_int,
    pub updateCacheResetCapacity: c_int,
    pub updateCacheReset: *mut *mut spBone,
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
    pub regionOffsetX: c_int,
    pub regionOffsetY: c_int,
    pub regionWidth: c_int,
    pub regionHeight: c_int,
    pub regionOriginalWidth: c_int,
    pub regionOriginalHeight: c_int,
    pub offset: [c_float; 8],
    pub uvs: [c_float; 8],
}
pub const BLY: C2RustUnnamed = 1;
pub const BLX: C2RustUnnamed = 0;
pub const BRY: C2RustUnnamed = 7;
pub const BRX: C2RustUnnamed = 6;
pub const URY: C2RustUnnamed = 5;
pub const URX: C2RustUnnamed = 4;
pub const ULY: C2RustUnnamed = 3;
pub const ULX: C2RustUnnamed = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spBoundingBoxAttachment {
    pub super_0: spVertexAttachment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spClippingAttachment {
    pub super_0: spVertexAttachment,
    pub endSlot: *mut spSlotData,
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
pub type spAnimationStateListener = Option::<
    unsafe extern "C" fn(
        *mut spAnimationState,
        spEventType,
        *mut spTrackEntry,
        *mut spEvent,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spTrackEntry {
    pub animation: *mut spAnimation,
    pub next: *mut spTrackEntry,
    pub mixingFrom: *mut spTrackEntry,
    pub mixingTo: *mut spTrackEntry,
    pub listener: spAnimationStateListener,
    pub trackIndex: c_int,
    pub loop_0: c_int,
    pub holdPrevious: c_int,
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
    pub propertyIDs: *mut c_int,
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
pub type C2RustUnnamed = c_uint;
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
    pub inheritDeform: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dataInput {
    pub cursor: *const c_uchar,
    pub end: *const c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spTimelineArray {
    pub size: c_int,
    pub capacity: c_int,
    pub items: *mut *mut spTimeline,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub intValue: c_int,
    pub floatValue: c_float,
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
    pub inheritDeform: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spVertexEffect {
    pub begin: spVertexEffectBegin,
    pub transform: spVertexEffectTransform,
    pub end: spVertexEffectEnd,
}
pub type spVertexEffectEnd = Option::<unsafe extern "C" fn(*mut spVertexEffect) -> ()>;
pub type spVertexEffectTransform = Option::<
    unsafe extern "C" fn(
        *mut spVertexEffect,
        *mut c_float,
        *mut c_float,
        *mut c_float,
        *mut c_float,
        *mut spColor,
        *mut spColor,
    ) -> (),
>;
pub type spVertexEffectBegin = Option::<
    unsafe extern "C" fn(*mut spVertexEffect, *mut spSkeleton) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spJitterVertexEffect {
    pub super_0: spVertexEffect,
    pub jitterX: c_float,
    pub jitterY: c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spSwirlVertexEffect {
    pub super_0: spVertexEffect,
    pub centerX: c_float,
    pub centerY: c_float,
    pub radius: c_float,
    pub angle: c_float,
    pub worldX: c_float,
    pub worldY: c_float,
}
#[no_mangle]
pub unsafe extern "C" fn isspace_(mut x: c_int) -> c_int {
    return (x <= 32 as c_int) as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spAnimation_create(
    mut name: *const c_char,
    mut timelinesCount: c_int,
) -> *mut spAnimation {
    let mut self_0: *mut spAnimation = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spAnimation>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        38 as c_int,
    ) as *mut spAnimation;
    let ref mut fresh0 = *(&(*self_0).name as *const *const c_char
        as *mut *mut c_char);
    *fresh0 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul(
                (spine_strlen(name)).wrapping_add(1 as c_int as c_ulong),
            ),
        b"spine.c\0" as *const u8 as *const c_char,
        39 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh0, name);
    (*self_0).timelinesCount = timelinesCount;
    (*self_0)
        .timelines = _spMalloc(
        (::core::mem::size_of::<*mut spTimeline>() as c_ulong)
            .wrapping_mul(timelinesCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        41 as c_int,
    ) as *mut *mut spTimeline;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spAnimation_dispose(mut self_0: *mut spAnimation) {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*self_0).timelinesCount {
        spTimeline_dispose(*((*self_0).timelines).offset(i as isize));
        i += 1;
    }
    _spFree((*self_0).timelines as *mut c_void);
    _spFree((*self_0).name as *mut c_void);
    _spFree(self_0 as *mut c_void);
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
    let mut n: c_int = (*self_0).timelinesCount;
    if loop_0 != 0 && (*self_0).duration != 0. {
        time = fmodf(time, (*self_0).duration);
        if lastTime > 0 as c_int as c_float {
            lastTime = fmodf(lastTime, (*self_0).duration);
        }
    }
    i = 0 as c_int;
    while i < n {
        spTimeline_apply(
            *((*self_0).timelines).offset(i as isize),
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
#[no_mangle]
pub unsafe extern "C" fn _spTimeline_init(
    mut self_0: *mut spTimeline,
    mut type_0: spTimelineType,
    mut dispose: Option::<unsafe extern "C" fn(*mut spTimeline) -> ()>,
    mut apply: Option::<
        unsafe extern "C" fn(
            *const spTimeline,
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
    mut getPropertyId: Option::<unsafe extern "C" fn(*const spTimeline) -> c_int>,
) {
    *(&(*self_0).type_0 as *const spTimelineType as *mut spTimelineType) = type_0;
    let ref mut fresh1 = *(&(*self_0).vtable as *const *const c_void
        as *mut *mut _spTimelineVtable);
    *fresh1 = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<_spTimelineVtable>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        84 as c_int,
    ) as *mut _spTimelineVtable;
    let ref mut fresh2 = (*((*self_0).vtable as *mut _spTimelineVtable)).dispose;
    *fresh2 = dispose;
    let ref mut fresh3 = (*((*self_0).vtable as *mut _spTimelineVtable)).apply;
    *fresh3 = apply;
    let ref mut fresh4 = (*((*self_0).vtable as *mut _spTimelineVtable)).getPropertyId;
    *fresh4 = getPropertyId;
}
#[no_mangle]
pub unsafe extern "C" fn _spTimeline_deinit(mut self_0: *mut spTimeline) {
    _spFree((*self_0).vtable as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spTimeline_dispose(mut self_0: *mut spTimeline) {
    ((*((*self_0).vtable as *mut _spTimelineVtable)).dispose)
        .expect("non-null function pointer")(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn spTimeline_apply(
    mut self_0: *const spTimeline,
    mut skeleton: *mut spSkeleton,
    mut lastTime: c_float,
    mut time: c_float,
    mut firedEvents: *mut *mut spEvent,
    mut eventsCount: *mut c_int,
    mut alpha: c_float,
    mut blend: spMixBlend,
    mut direction: spMixDirection,
) {
    ((*((*(self_0 as *mut spTimeline)).vtable as *mut _spTimelineVtable)).apply)
        .expect(
            "non-null function pointer",
        )(
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
pub unsafe extern "C" fn spTimeline_getPropertyId(
    mut self_0: *const spTimeline,
) -> c_int {
    return ((*((*(self_0 as *mut spTimeline)).vtable as *mut _spTimelineVtable))
        .getPropertyId)
        .expect("non-null function pointer")(self_0);
}
static mut CURVE_LINEAR: c_float = 0 as c_int as c_float;
static mut CURVE_STEPPED: c_float = 1 as c_int as c_float;
static mut CURVE_BEZIER: c_float = 2 as c_int as c_float;
static mut BEZIER_SIZE: c_int = 10 as c_int * 2 as c_int
    - 1 as c_int;
#[no_mangle]
pub unsafe extern "C" fn _spCurveTimeline_init(
    mut self_0: *mut spCurveTimeline,
    mut type_0: spTimelineType,
    mut framesCount: c_int,
    mut dispose: Option::<unsafe extern "C" fn(*mut spTimeline) -> ()>,
    mut apply: Option::<
        unsafe extern "C" fn(
            *const spTimeline,
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
    mut getPropertyId: Option::<unsafe extern "C" fn(*const spTimeline) -> c_int>,
) {
    _spTimeline_init(&mut (*self_0).super_0, type_0, dispose, apply, getPropertyId);
    (*self_0)
        .curves = _spCalloc(
        ((framesCount - 1 as c_int) * BEZIER_SIZE) as size_t,
        ::core::mem::size_of::<c_float>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        119 as c_int,
    ) as *mut c_float;
}
#[no_mangle]
pub unsafe extern "C" fn _spCurveTimeline_deinit(mut self_0: *mut spCurveTimeline) {
    _spTimeline_deinit(&mut (*self_0).super_0);
    _spFree((*self_0).curves as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spCurveTimeline_setLinear(
    mut self_0: *mut spCurveTimeline,
    mut frameIndex: c_int,
) {
    *((*self_0).curves).offset((frameIndex * BEZIER_SIZE) as isize) = CURVE_LINEAR;
}
#[no_mangle]
pub unsafe extern "C" fn spCurveTimeline_setStepped(
    mut self_0: *mut spCurveTimeline,
    mut frameIndex: c_int,
) {
    *((*self_0).curves).offset((frameIndex * BEZIER_SIZE) as isize) = CURVE_STEPPED;
}
#[no_mangle]
pub unsafe extern "C" fn spCurveTimeline_setCurve(
    mut self_0: *mut spCurveTimeline,
    mut frameIndex: c_int,
    mut cx1: c_float,
    mut cy1: c_float,
    mut cx2: c_float,
    mut cy2: c_float,
) {
    let mut tmpx: c_float = (-cx1 * 2 as c_int as c_float + cx2)
        * 0.03f32;
    let mut tmpy: c_float = (-cy1 * 2 as c_int as c_float + cy2)
        * 0.03f32;
    let mut dddfx: c_float = ((cx1 - cx2) * 3 as c_int as c_float
        + 1 as c_int as c_float) * 0.006f32;
    let mut dddfy: c_float = ((cy1 - cy2) * 3 as c_int as c_float
        + 1 as c_int as c_float) * 0.006f32;
    let mut ddfx: c_float = tmpx * 2 as c_int as c_float + dddfx;
    let mut ddfy: c_float = tmpy * 2 as c_int as c_float + dddfy;
    let mut dfx: c_float = cx1 * 0.3f32 + tmpx + dddfx * 0.16666667f32;
    let mut dfy: c_float = cy1 * 0.3f32 + tmpy + dddfy * 0.16666667f32;
    let mut x: c_float = dfx;
    let mut y: c_float = dfy;
    let mut i: c_int = frameIndex * BEZIER_SIZE;
    let mut n: c_int = i + BEZIER_SIZE - 1 as c_int;
    let fresh5 = i;
    i = i + 1;
    *((*self_0).curves).offset(fresh5 as isize) = CURVE_BEZIER;
    while i < n {
        *((*self_0).curves).offset(i as isize) = x;
        *((*self_0).curves).offset((i + 1 as c_int) as isize) = y;
        dfx += ddfx;
        dfy += ddfy;
        ddfx += dddfx;
        ddfy += dddfy;
        x += dfx;
        y += dfy;
        i += 2 as c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spCurveTimeline_getCurvePercent(
    mut self_0: *const spCurveTimeline,
    mut frameIndex: c_int,
    mut percent: c_float,
) -> c_float {
    let mut x: c_float = 0.;
    let mut y: c_float = 0.;
    let mut i: c_int = frameIndex * BEZIER_SIZE;
    let mut start: c_int = 0;
    let mut n: c_int = 0;
    let mut type_0: c_float = *((*self_0).curves).offset(i as isize);
    percent = if percent < 0 as c_int as c_float {
        0 as c_int as c_float
    } else if percent > 1 as c_int as c_float {
        1 as c_int as c_float
    } else {
        percent
    };
    if type_0 == CURVE_LINEAR {
        return percent;
    }
    if type_0 == CURVE_STEPPED {
        return 0 as c_int as c_float;
    }
    i += 1;
    x = 0 as c_int as c_float;
    start = i;
    n = i + BEZIER_SIZE - 1 as c_int;
    while i < n {
        x = *((*self_0).curves).offset(i as isize);
        if x >= percent {
            let mut prevX: c_float = 0.;
            let mut prevY: c_float = 0.;
            if i == start {
                prevX = 0 as c_int as c_float;
                prevY = 0 as c_int as c_float;
            } else {
                prevX = *((*self_0).curves).offset((i - 2 as c_int) as isize);
                prevY = *((*self_0).curves).offset((i - 1 as c_int) as isize);
            }
            return prevY
                + (*((*self_0).curves).offset((i + 1 as c_int) as isize) - prevY)
                    * (percent - prevX) / (x - prevX);
        }
        i += 2 as c_int;
    }
    y = *((*self_0).curves).offset((i - 1 as c_int) as isize);
    return y
        + (1 as c_int as c_float - y) * (percent - x)
            / (1 as c_int as c_float - x);
}
unsafe extern "C" fn binarySearch(
    mut values: *mut c_float,
    mut valuesLength: c_int,
    mut target: c_float,
    mut step: c_int,
) -> c_int {
    let mut low: c_int = 0 as c_int;
    let mut current: c_int = 0;
    let mut high: c_int = valuesLength / step - 2 as c_int;
    if high == 0 as c_int {
        return step;
    }
    current = high >> 1 as c_int;
    loop {
        if *values.offset(((current + 1 as c_int) * step) as isize) <= target {
            low = current + 1 as c_int;
        } else {
            high = current;
        }
        if low == high {
            return (low + 1 as c_int) * step;
        }
        current = low + high >> 1 as c_int;
    };
}
static mut ROTATE_PREV_TIME: c_int = -(2 as c_int);
static mut ROTATE_PREV_ROTATION: c_int = -(1 as c_int);
static mut ROTATE_ROTATION: c_int = 1 as c_int;
static mut ROTATE_ENTRIES: c_int = 2 as c_int;
#[no_mangle]
pub unsafe extern "C" fn _spCurveTimeline_binarySearch(
    mut values: *mut c_float,
    mut valuesLength: c_int,
    mut target: c_float,
    mut step: c_int,
) -> c_int {
    return binarySearch(values, valuesLength, target, step);
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
    };
}
static mut TRANSLATE_ENTRIES: c_int = 3 as c_int;
#[no_mangle]
pub unsafe extern "C" fn _spBaseTimeline_dispose(mut timeline: *mut spTimeline) {
    let mut self_0: *mut spBaseTimeline = timeline as *mut spBaseTimeline;
    _spCurveTimeline_deinit(&mut (*self_0).super_0);
    _spFree((*self_0).frames as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _spBaseTimeline_create(
    mut framesCount: c_int,
    mut type_0: spTimelineType,
    mut frameSize: c_int,
    mut apply: Option::<
        unsafe extern "C" fn(
            *const spTimeline,
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
    mut getPropertyId: Option::<unsafe extern "C" fn(*const spTimeline) -> c_int>,
) -> *mut spBaseTimeline {
    let mut self_0: *mut spBaseTimeline = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spBaseTimeline>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        237 as c_int,
    ) as *mut spBaseTimeline;
    _spCurveTimeline_init(
        &mut (*self_0).super_0,
        type_0,
        framesCount,
        Some(_spBaseTimeline_dispose as unsafe extern "C" fn(*mut spTimeline) -> ()),
        apply,
        getPropertyId,
    );
    *(&(*self_0).framesCount as *const c_int
        as *mut c_int) = framesCount * frameSize;
    let ref mut fresh6 = *(&(*self_0).frames as *const *mut c_float
        as *mut *mut c_float);
    *fresh6 = _spCalloc(
        (*self_0).framesCount as size_t,
        ::core::mem::size_of::<c_float>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        241 as c_int,
    ) as *mut c_float;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn _spRotateTimeline_apply(
    mut timeline: *const spTimeline,
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
    let mut frame: c_int = 0;
    let mut prevRotation: c_float = 0.;
    let mut frameTime: c_float = 0.;
    let mut percent: c_float = 0.;
    let mut r: c_float = 0.;
    let mut self_0: *mut spRotateTimeline = timeline as *mut spRotateTimeline;
    bone = *((*skeleton).bones).offset((*self_0).boneIndex as isize);
    if (*bone).active == 0 {
        return;
    }
    if time < *((*self_0).frames).offset(0 as c_int as isize) {
        match blend as c_uint {
            0 => {
                (*bone).rotation = (*(*bone).data).rotation;
                return;
            }
            1 => {
                r = (*(*bone).data).rotation - (*bone).rotation;
                r
                    -= ((16384 as c_int
                        - (16384.499999999996f64
                            - (r / 360 as c_int as c_float)
                                as c_double) as c_int) * 360 as c_int)
                        as c_float;
                (*bone).rotation += r * alpha;
            }
            2 | 3 | _ => {}
        }
        return;
    }
    if time
        >= *((*self_0).frames).offset(((*self_0).framesCount - ROTATE_ENTRIES) as isize)
    {
        r = *((*self_0).frames)
            .offset(((*self_0).framesCount + ROTATE_PREV_ROTATION) as isize);
        let mut current_block_17: u64;
        match blend as c_uint {
            0 => {
                (*bone).rotation = (*(*bone).data).rotation + r * alpha;
                current_block_17 = 2370887241019905314;
            }
            1 | 2 => {
                r += (*(*bone).data).rotation - (*bone).rotation;
                r
                    -= ((16384 as c_int
                        - (16384.499999999996f64
                            - (r / 360 as c_int as c_float)
                                as c_double) as c_int) * 360 as c_int)
                        as c_float;
                current_block_17 = 16089486732670942766;
            }
            3 => {
                current_block_17 = 16089486732670942766;
            }
            _ => {
                current_block_17 = 2370887241019905314;
            }
        }
        match current_block_17 {
            16089486732670942766 => {
                (*bone).rotation += r * alpha;
            }
            _ => {}
        }
        return;
    }
    frame = binarySearch((*self_0).frames, (*self_0).framesCount, time, ROTATE_ENTRIES);
    prevRotation = *((*self_0).frames).offset((frame + ROTATE_PREV_ROTATION) as isize);
    frameTime = *((*self_0).frames).offset(frame as isize);
    percent = spCurveTimeline_getCurvePercent(
        &mut (*self_0).super_0,
        (frame >> 1 as c_int) - 1 as c_int,
        1 as c_int as c_float
            - (time - frameTime)
                / (*((*self_0).frames).offset((frame + ROTATE_PREV_TIME) as isize)
                    - frameTime),
    );
    r = *((*self_0).frames).offset((frame + ROTATE_ROTATION) as isize) - prevRotation;
    r = prevRotation
        + (r
            - ((16384 as c_int
                - (16384.499999999996f64
                    - (r / 360 as c_int as c_float) as c_double)
                    as c_int) * 360 as c_int) as c_float) * percent;
    let mut current_block_30: u64;
    match blend as c_uint {
        0 => {
            (*bone)
                .rotation = (*(*bone).data).rotation
                + (r
                    - ((16384 as c_int
                        - (16384.499999999996f64
                            - (r / 360 as c_int as c_float)
                                as c_double) as c_int) * 360 as c_int)
                        as c_float) * alpha;
            current_block_30 = 4761528863920922185;
        }
        1 | 2 => {
            r += (*(*bone).data).rotation - (*bone).rotation;
            current_block_30 = 776449361283660306;
        }
        3 => {
            current_block_30 = 776449361283660306;
        }
        _ => {
            current_block_30 = 4761528863920922185;
        }
    }
    match current_block_30 {
        776449361283660306 => {
            (*bone).rotation
                += (r
                    - ((16384 as c_int
                        - (16384.499999999996f64
                            - (r / 360 as c_int as c_float)
                                as c_double) as c_int) * 360 as c_int)
                        as c_float) * alpha;
        }
        _ => {}
    };
}
static mut COLOR_ENTRIES: c_int = 5 as c_int;
static mut TWOCOLOR_ENTRIES: c_int = 8 as c_int;
#[no_mangle]
pub unsafe extern "C" fn _spRotateTimeline_getPropertyId(
    mut timeline: *const spTimeline,
) -> c_int {
    return ((SP_TIMELINE_ROTATE as c_int) << 25 as c_int)
        + (*(timeline as *mut spRotateTimeline)).boneIndex;
}
#[no_mangle]
pub unsafe extern "C" fn spRotateTimeline_create(
    mut framesCount: c_int,
) -> *mut spRotateTimeline {
    return _spBaseTimeline_create(
        framesCount,
        SP_TIMELINE_ROTATE,
        ROTATE_ENTRIES,
        Some(
            _spRotateTimeline_apply
                as unsafe extern "C" fn(
                    *const spTimeline,
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
            _spRotateTimeline_getPropertyId
                as unsafe extern "C" fn(*const spTimeline) -> c_int,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn spRotateTimeline_setFrame(
    mut self_0: *mut spRotateTimeline,
    mut frameIndex: c_int,
    mut time: c_float,
    mut degrees: c_float,
) {
    frameIndex <<= 1 as c_int;
    *((*self_0).frames).offset(frameIndex as isize) = time;
    *((*self_0).frames).offset((frameIndex + ROTATE_ROTATION) as isize) = degrees;
}
static mut TRANSLATE_PREV_TIME: c_int = -(3 as c_int);
static mut TRANSLATE_PREV_X: c_int = -(2 as c_int);
static mut TRANSLATE_PREV_Y: c_int = -(1 as c_int);
static mut TRANSLATE_X: c_int = 1 as c_int;
static mut TRANSLATE_Y: c_int = 2 as c_int;
#[no_mangle]
pub unsafe extern "C" fn _spTranslateTimeline_apply(
    mut timeline: *const spTimeline,
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
    let mut frame: c_int = 0;
    let mut frameTime: c_float = 0.;
    let mut percent: c_float = 0.;
    let mut x: c_float = 0.;
    let mut y: c_float = 0.;
    let mut frames: *mut c_float = 0 as *mut c_float;
    let mut framesCount: c_int = 0;
    let mut self_0: *mut spTranslateTimeline = timeline as *mut spTranslateTimeline;
    bone = *((*skeleton).bones).offset((*self_0).boneIndex as isize);
    if (*bone).active == 0 {
        return;
    }
    if time < *((*self_0).frames).offset(0 as c_int as isize) {
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
            2 | 3 | _ => {}
        }
        return;
    }
    frames = (*self_0).frames;
    framesCount = (*self_0).framesCount;
    if time >= *frames.offset((framesCount - TRANSLATE_ENTRIES) as isize) {
        x = *frames.offset((framesCount + TRANSLATE_PREV_X) as isize);
        y = *frames.offset((framesCount + TRANSLATE_PREV_Y) as isize);
    } else {
        frame = binarySearch(frames, framesCount, time, TRANSLATE_ENTRIES);
        x = *frames.offset((frame + TRANSLATE_PREV_X) as isize);
        y = *frames.offset((frame + TRANSLATE_PREV_Y) as isize);
        frameTime = *frames.offset(frame as isize);
        percent = spCurveTimeline_getCurvePercent(
            &mut (*self_0).super_0,
            frame / TRANSLATE_ENTRIES - 1 as c_int,
            1 as c_int as c_float
                - (time - frameTime)
                    / (*frames.offset((frame + TRANSLATE_PREV_TIME) as isize)
                        - frameTime),
        );
        x += (*frames.offset((frame + TRANSLATE_X) as isize) - x) * percent;
        y += (*frames.offset((frame + TRANSLATE_Y) as isize) - y) * percent;
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
pub unsafe extern "C" fn _spTranslateTimeline_getPropertyId(
    mut self_0: *const spTimeline,
) -> c_int {
    return ((SP_TIMELINE_TRANSLATE as c_int) << 24 as c_int)
        + (*(self_0 as *mut spTranslateTimeline)).boneIndex;
}
#[no_mangle]
pub unsafe extern "C" fn spTranslateTimeline_create(
    mut framesCount: c_int,
) -> *mut spTranslateTimeline {
    return _spBaseTimeline_create(
        framesCount,
        SP_TIMELINE_TRANSLATE,
        TRANSLATE_ENTRIES,
        Some(
            _spTranslateTimeline_apply
                as unsafe extern "C" fn(
                    *const spTimeline,
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
            _spTranslateTimeline_getPropertyId
                as unsafe extern "C" fn(*const spTimeline) -> c_int,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn spTranslateTimeline_setFrame(
    mut self_0: *mut spTranslateTimeline,
    mut frameIndex: c_int,
    mut time: c_float,
    mut x: c_float,
    mut y: c_float,
) {
    frameIndex *= TRANSLATE_ENTRIES;
    *((*self_0).frames).offset(frameIndex as isize) = time;
    *((*self_0).frames).offset((frameIndex + TRANSLATE_X) as isize) = x;
    *((*self_0).frames).offset((frameIndex + TRANSLATE_Y) as isize) = y;
}
#[no_mangle]
pub unsafe extern "C" fn _spScaleTimeline_apply(
    mut timeline: *const spTimeline,
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
    let mut frame: c_int = 0;
    let mut frameTime: c_float = 0.;
    let mut percent: c_float = 0.;
    let mut x: c_float = 0.;
    let mut y: c_float = 0.;
    let mut frames: *mut c_float = 0 as *mut c_float;
    let mut framesCount: c_int = 0;
    let mut self_0: *mut spScaleTimeline = timeline as *mut spScaleTimeline;
    bone = *((*skeleton).bones).offset((*self_0).boneIndex as isize);
    if (*bone).active == 0 {
        return;
    }
    if time < *((*self_0).frames).offset(0 as c_int as isize) {
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
            2 | 3 | _ => {}
        }
        return;
    }
    frames = (*self_0).frames;
    framesCount = (*self_0).framesCount;
    if time >= *frames.offset((framesCount - TRANSLATE_ENTRIES) as isize) {
        x = *frames.offset((framesCount + TRANSLATE_PREV_X) as isize)
            * (*(*bone).data).scaleX;
        y = *frames.offset((framesCount + TRANSLATE_PREV_Y) as isize)
            * (*(*bone).data).scaleY;
    } else {
        frame = binarySearch(frames, framesCount, time, TRANSLATE_ENTRIES);
        x = *frames.offset((frame + TRANSLATE_PREV_X) as isize);
        y = *frames.offset((frame + TRANSLATE_PREV_Y) as isize);
        frameTime = *frames.offset(frame as isize);
        percent = spCurveTimeline_getCurvePercent(
            &mut (*self_0).super_0,
            frame / TRANSLATE_ENTRIES - 1 as c_int,
            1 as c_int as c_float
                - (time - frameTime)
                    / (*frames.offset((frame + TRANSLATE_PREV_TIME) as isize)
                        - frameTime),
        );
        x = (x + (*frames.offset((frame + TRANSLATE_X) as isize) - x) * percent)
            * (*(*bone).data).scaleX;
        y = (y + (*frames.offset((frame + TRANSLATE_Y) as isize) - y) * percent)
            * (*(*bone).data).scaleY;
    }
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
        if direction as c_uint
            == SP_MIX_DIRECTION_OUT as c_int as c_uint
        {
            match blend as c_uint {
                0 => {
                    bx = (*(*bone).data).scaleX;
                    by = (*(*bone).data).scaleY;
                    (*bone)
                        .scaleX = bx
                        + ((if x < 0 as c_int as c_float { -x } else { x })
                            * (if bx < 0 as c_int as c_float {
                                -1.0f32
                            } else {
                                if bx > 0 as c_int as c_float {
                                    1.0f32
                                } else {
                                    0.0f32
                                }
                            }) - bx) * alpha;
                    (*bone)
                        .scaleY = by
                        + ((if y < 0 as c_int as c_float { -y } else { y })
                            * (if by < 0 as c_int as c_float {
                                -1.0f32
                            } else {
                                if by > 0 as c_int as c_float {
                                    1.0f32
                                } else {
                                    0.0f32
                                }
                            }) - by) * alpha;
                }
                1 | 2 => {
                    bx = (*bone).scaleX;
                    by = (*bone).scaleY;
                    (*bone)
                        .scaleX = bx
                        + ((if x < 0 as c_int as c_float { -x } else { x })
                            * (if bx < 0 as c_int as c_float {
                                -1.0f32
                            } else {
                                if bx > 0 as c_int as c_float {
                                    1.0f32
                                } else {
                                    0.0f32
                                }
                            }) - bx) * alpha;
                    (*bone)
                        .scaleY = by
                        + ((if y < 0 as c_int as c_float { -y } else { y })
                            * (if by < 0 as c_int as c_float {
                                -1.0f32
                            } else {
                                if by > 0 as c_int as c_float {
                                    1.0f32
                                } else {
                                    0.0f32
                                }
                            }) - by) * alpha;
                }
                3 => {
                    bx = (*bone).scaleX;
                    by = (*bone).scaleY;
                    (*bone)
                        .scaleX = bx
                        + ((if x < 0 as c_int as c_float { -x } else { x })
                            * (if bx < 0 as c_int as c_float {
                                -1.0f32
                            } else {
                                if bx > 0 as c_int as c_float {
                                    1.0f32
                                } else {
                                    0.0f32
                                }
                            }) - (*(*bone).data).scaleX) * alpha;
                    (*bone)
                        .scaleY = by
                        + ((if y < 0 as c_int as c_float { -y } else { y })
                            * (if by < 0 as c_int as c_float {
                                -1.0f32
                            } else {
                                if by > 0 as c_int as c_float {
                                    1.0f32
                                } else {
                                    0.0f32
                                }
                            }) - (*(*bone).data).scaleY) * alpha;
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
                    })
                        * (if x < 0 as c_int as c_float {
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
                    })
                        * (if y < 0 as c_int as c_float {
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
                    })
                        * (if x < 0 as c_int as c_float {
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
                    })
                        * (if y < 0 as c_int as c_float {
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
                    bx = if x < 0 as c_int as c_float {
                        -1.0f32
                    } else if x > 0 as c_int as c_float {
                        1.0f32
                    } else {
                        0.0f32
                    };
                    by = if y < 0 as c_int as c_float {
                        -1.0f32
                    } else if y > 0 as c_int as c_float {
                        1.0f32
                    } else {
                        0.0f32
                    };
                    (*bone)
                        .scaleX = (if (*bone).scaleX < 0 as c_int as c_float
                    {
                        -(*bone).scaleX
                    } else {
                        (*bone).scaleX
                    }) * bx
                        + (x
                            - (if (*(*bone).data).scaleX
                                < 0 as c_int as c_float
                            {
                                -(*(*bone).data).scaleX
                            } else {
                                (*(*bone).data).scaleX
                            }) * bx) * alpha;
                    (*bone)
                        .scaleY = (if (*bone).scaleY < 0 as c_int as c_float
                    {
                        -(*bone).scaleY
                    } else {
                        (*bone).scaleY
                    }) * by
                        + (y
                            - (if (*(*bone).data).scaleY
                                < 0 as c_int as c_float
                            {
                                -(*(*bone).data).scaleY
                            } else {
                                (*(*bone).data).scaleY
                            }) * by) * alpha;
                }
                _ => {}
            }
        }
    };
}
static mut IKCONSTRAINT_ENTRIES: c_int = 6 as c_int;
static mut TRANSFORMCONSTRAINT_ENTRIES: c_int = 5 as c_int;
static mut PATHCONSTRAINTPOSITION_ENTRIES: c_int = 2 as c_int;
static mut PATHCONSTRAINTSPACING_ENTRIES: c_int = 2 as c_int;
#[no_mangle]
pub unsafe extern "C" fn _spScaleTimeline_getPropertyId(
    mut timeline: *const spTimeline,
) -> c_int {
    return ((SP_TIMELINE_SCALE as c_int) << 24 as c_int)
        + (*(timeline as *mut spScaleTimeline)).boneIndex;
}
#[no_mangle]
pub unsafe extern "C" fn spScaleTimeline_create(
    mut framesCount: c_int,
) -> *mut spScaleTimeline {
    return _spBaseTimeline_create(
        framesCount,
        SP_TIMELINE_SCALE,
        TRANSLATE_ENTRIES,
        Some(
            _spScaleTimeline_apply
                as unsafe extern "C" fn(
                    *const spTimeline,
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
            _spScaleTimeline_getPropertyId
                as unsafe extern "C" fn(*const spTimeline) -> c_int,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn spScaleTimeline_setFrame(
    mut self_0: *mut spScaleTimeline,
    mut frameIndex: c_int,
    mut time: c_float,
    mut x: c_float,
    mut y: c_float,
) {
    spTranslateTimeline_setFrame(self_0, frameIndex, time, x, y);
}
#[no_mangle]
pub unsafe extern "C" fn _spShearTimeline_apply(
    mut timeline: *const spTimeline,
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
    let mut frame: c_int = 0;
    let mut frameTime: c_float = 0.;
    let mut percent: c_float = 0.;
    let mut x: c_float = 0.;
    let mut y: c_float = 0.;
    let mut frames: *mut c_float = 0 as *mut c_float;
    let mut framesCount: c_int = 0;
    let mut self_0: *mut spShearTimeline = timeline as *mut spShearTimeline;
    bone = *((*skeleton).bones).offset((*self_0).boneIndex as isize);
    if (*bone).active == 0 {
        return;
    }
    frames = (*self_0).frames;
    framesCount = (*self_0).framesCount;
    if time < *((*self_0).frames).offset(0 as c_int as isize) {
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
            2 | 3 | _ => {}
        }
        return;
    }
    if time >= *frames.offset((framesCount - TRANSLATE_ENTRIES) as isize) {
        x = *frames.offset((framesCount + TRANSLATE_PREV_X) as isize);
        y = *frames.offset((framesCount + TRANSLATE_PREV_Y) as isize);
    } else {
        frame = binarySearch(frames, framesCount, time, TRANSLATE_ENTRIES);
        x = *frames.offset((frame + TRANSLATE_PREV_X) as isize);
        y = *frames.offset((frame + TRANSLATE_PREV_Y) as isize);
        frameTime = *frames.offset(frame as isize);
        percent = spCurveTimeline_getCurvePercent(
            &mut (*self_0).super_0,
            frame / TRANSLATE_ENTRIES - 1 as c_int,
            1 as c_int as c_float
                - (time - frameTime)
                    / (*frames.offset((frame + TRANSLATE_PREV_TIME) as isize)
                        - frameTime),
        );
        x = x + (*frames.offset((frame + TRANSLATE_X) as isize) - x) * percent;
        y = y + (*frames.offset((frame + TRANSLATE_Y) as isize) - y) * percent;
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
static mut PATHCONSTRAINTMIX_ENTRIES: c_int = 3 as c_int;
#[no_mangle]
pub unsafe extern "C" fn _spShearTimeline_getPropertyId(
    mut timeline: *const spTimeline,
) -> c_int {
    return ((SP_TIMELINE_SHEAR as c_int) << 24 as c_int)
        + (*(timeline as *mut spShearTimeline)).boneIndex;
}
#[no_mangle]
pub unsafe extern "C" fn spShearTimeline_create(
    mut framesCount: c_int,
) -> *mut spShearTimeline {
    return _spBaseTimeline_create(
        framesCount,
        SP_TIMELINE_SHEAR,
        3 as c_int,
        Some(
            _spShearTimeline_apply
                as unsafe extern "C" fn(
                    *const spTimeline,
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
            _spShearTimeline_getPropertyId
                as unsafe extern "C" fn(*const spTimeline) -> c_int,
        ),
    ) as *mut spShearTimeline;
}
#[no_mangle]
pub unsafe extern "C" fn spShearTimeline_setFrame(
    mut self_0: *mut spShearTimeline,
    mut frameIndex: c_int,
    mut time: c_float,
    mut x: c_float,
    mut y: c_float,
) {
    spTranslateTimeline_setFrame(self_0, frameIndex, time, x, y);
}
static mut COLOR_PREV_TIME: c_int = -(5 as c_int);
static mut COLOR_PREV_R: c_int = -(4 as c_int);
static mut COLOR_PREV_G: c_int = -(3 as c_int);
static mut COLOR_PREV_B: c_int = -(2 as c_int);
static mut COLOR_PREV_A: c_int = -(1 as c_int);
static mut COLOR_R: c_int = 1 as c_int;
static mut COLOR_G: c_int = 2 as c_int;
static mut COLOR_B: c_int = 3 as c_int;
static mut COLOR_A: c_int = 4 as c_int;
#[no_mangle]
pub unsafe extern "C" fn _spColorTimeline_apply(
    mut timeline: *const spTimeline,
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
    let mut frame: c_int = 0;
    let mut percent: c_float = 0.;
    let mut frameTime: c_float = 0.;
    let mut r: c_float = 0.;
    let mut g: c_float = 0.;
    let mut b: c_float = 0.;
    let mut a: c_float = 0.;
    let mut color: *mut spColor = 0 as *mut spColor;
    let mut setup: *mut spColor = 0 as *mut spColor;
    let mut self_0: *mut spColorTimeline = timeline as *mut spColorTimeline;
    slot = *((*skeleton).slots).offset((*self_0).slotIndex as isize);
    if (*(*slot).bone).active == 0 {
        return;
    }
    if time < *((*self_0).frames).offset(0 as c_int as isize) {
        match blend as c_uint {
            0 => {
                spColor_setFromColor(&mut (*slot).color, &mut (*(*slot).data).color);
                return;
            }
            1 => {
                color = &mut (*slot).color;
                setup = &mut (*(*slot).data).color;
                spColor_addFloats(
                    color,
                    ((*setup).r - (*color).r) * alpha,
                    ((*setup).g - (*color).g) * alpha,
                    ((*setup).b - (*color).b) * alpha,
                    ((*setup).a - (*color).a) * alpha,
                );
            }
            2 | 3 | _ => {}
        }
        return;
    }
    if time
        >= *((*self_0).frames)
            .offset(((*self_0).framesCount - 5 as c_int) as isize)
    {
        let mut i: c_int = (*self_0).framesCount;
        r = *((*self_0).frames).offset((i + COLOR_PREV_R) as isize);
        g = *((*self_0).frames).offset((i + COLOR_PREV_G) as isize);
        b = *((*self_0).frames).offset((i + COLOR_PREV_B) as isize);
        a = *((*self_0).frames).offset((i + COLOR_PREV_A) as isize);
    } else {
        frame = binarySearch(
            (*self_0).frames,
            (*self_0).framesCount,
            time,
            COLOR_ENTRIES,
        );
        r = *((*self_0).frames).offset((frame + COLOR_PREV_R) as isize);
        g = *((*self_0).frames).offset((frame + COLOR_PREV_G) as isize);
        b = *((*self_0).frames).offset((frame + COLOR_PREV_B) as isize);
        a = *((*self_0).frames).offset((frame + COLOR_PREV_A) as isize);
        frameTime = *((*self_0).frames).offset(frame as isize);
        percent = spCurveTimeline_getCurvePercent(
            &mut (*self_0).super_0,
            frame / COLOR_ENTRIES - 1 as c_int,
            1 as c_int as c_float
                - (time - frameTime)
                    / (*((*self_0).frames).offset((frame + COLOR_PREV_TIME) as isize)
                        - frameTime),
        );
        r += (*((*self_0).frames).offset((frame + COLOR_R) as isize) - r) * percent;
        g += (*((*self_0).frames).offset((frame + COLOR_G) as isize) - g) * percent;
        b += (*((*self_0).frames).offset((frame + COLOR_B) as isize) - b) * percent;
        a += (*((*self_0).frames).offset((frame + COLOR_A) as isize) - a) * percent;
    }
    if alpha == 1 as c_int as c_float {
        spColor_setFromFloats(&mut (*slot).color, r, g, b, a);
    } else {
        if blend as c_uint == SP_MIX_BLEND_SETUP as c_int as c_uint {
            spColor_setFromColor(&mut (*slot).color, &mut (*(*slot).data).color);
        }
        spColor_addFloats(
            &mut (*slot).color,
            (r - (*slot).color.r) * alpha,
            (g - (*slot).color.g) * alpha,
            (b - (*slot).color.b) * alpha,
            (a - (*slot).color.a) * alpha,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn _spColorTimeline_getPropertyId(
    mut timeline: *const spTimeline,
) -> c_int {
    return ((SP_TIMELINE_COLOR as c_int) << 24 as c_int)
        + (*(timeline as *mut spColorTimeline)).slotIndex;
}
#[no_mangle]
pub unsafe extern "C" fn spColorTimeline_create(
    mut framesCount: c_int,
) -> *mut spColorTimeline {
    return _spBaseTimeline_create(
        framesCount,
        SP_TIMELINE_COLOR,
        5 as c_int,
        Some(
            _spColorTimeline_apply
                as unsafe extern "C" fn(
                    *const spTimeline,
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
            _spColorTimeline_getPropertyId
                as unsafe extern "C" fn(*const spTimeline) -> c_int,
        ),
    ) as *mut spColorTimeline;
}
#[no_mangle]
pub unsafe extern "C" fn spColorTimeline_setFrame(
    mut self_0: *mut spColorTimeline,
    mut frameIndex: c_int,
    mut time: c_float,
    mut r: c_float,
    mut g: c_float,
    mut b: c_float,
    mut a: c_float,
) {
    frameIndex *= COLOR_ENTRIES;
    *((*self_0).frames).offset(frameIndex as isize) = time;
    *((*self_0).frames).offset((frameIndex + COLOR_R) as isize) = r;
    *((*self_0).frames).offset((frameIndex + COLOR_G) as isize) = g;
    *((*self_0).frames).offset((frameIndex + COLOR_B) as isize) = b;
    *((*self_0).frames).offset((frameIndex + COLOR_A) as isize) = a;
}
static mut TWOCOLOR_PREV_TIME: c_int = -(8 as c_int);
static mut TWOCOLOR_PREV_R: c_int = -(7 as c_int);
static mut TWOCOLOR_PREV_G: c_int = -(6 as c_int);
static mut TWOCOLOR_PREV_B: c_int = -(5 as c_int);
static mut TWOCOLOR_PREV_A: c_int = -(4 as c_int);
static mut TWOCOLOR_PREV_R2: c_int = -(3 as c_int);
static mut TWOCOLOR_PREV_G2: c_int = -(2 as c_int);
static mut TWOCOLOR_PREV_B2: c_int = -(1 as c_int);
static mut TWOCOLOR_R: c_int = 1 as c_int;
static mut TWOCOLOR_G: c_int = 2 as c_int;
static mut TWOCOLOR_B: c_int = 3 as c_int;
static mut TWOCOLOR_A: c_int = 4 as c_int;
static mut TWOCOLOR_R2: c_int = 5 as c_int;
static mut TWOCOLOR_G2: c_int = 6 as c_int;
static mut TWOCOLOR_B2: c_int = 7 as c_int;
#[no_mangle]
pub unsafe extern "C" fn _spTwoColorTimeline_apply(
    mut timeline: *const spTimeline,
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
    let mut frame: c_int = 0;
    let mut percent: c_float = 0.;
    let mut frameTime: c_float = 0.;
    let mut r: c_float = 0.;
    let mut g: c_float = 0.;
    let mut b: c_float = 0.;
    let mut a: c_float = 0.;
    let mut r2: c_float = 0.;
    let mut g2: c_float = 0.;
    let mut b2: c_float = 0.;
    let mut light: *mut spColor = 0 as *mut spColor;
    let mut dark: *mut spColor = 0 as *mut spColor;
    let mut setupLight: *mut spColor = 0 as *mut spColor;
    let mut setupDark: *mut spColor = 0 as *mut spColor;
    let mut self_0: *mut spColorTimeline = timeline as *mut spColorTimeline;
    slot = *((*skeleton).slots).offset((*self_0).slotIndex as isize);
    if (*(*slot).bone).active == 0 {
        return;
    }
    if time < *((*self_0).frames).offset(0 as c_int as isize) {
        match blend as c_uint {
            0 => {
                spColor_setFromColor(&mut (*slot).color, &mut (*(*slot).data).color);
                spColor_setFromColor((*slot).darkColor, (*(*slot).data).darkColor);
                return;
            }
            1 => {
                light = &mut (*slot).color;
                dark = (*slot).darkColor;
                setupLight = &mut (*(*slot).data).color;
                setupDark = (*(*slot).data).darkColor;
                spColor_addFloats(
                    light,
                    ((*setupLight).r - (*light).r) * alpha,
                    ((*setupLight).g - (*light).g) * alpha,
                    ((*setupLight).b - (*light).b) * alpha,
                    ((*setupLight).a - (*light).a) * alpha,
                );
                spColor_addFloats(
                    dark,
                    ((*setupDark).r - (*dark).r) * alpha,
                    ((*setupDark).g - (*dark).g) * alpha,
                    ((*setupDark).b - (*dark).b) * alpha,
                    0 as c_int as c_float,
                );
            }
            2 | 3 | _ => {}
        }
        return;
    }
    if time
        >= *((*self_0).frames)
            .offset(((*self_0).framesCount - TWOCOLOR_ENTRIES) as isize)
    {
        let mut i: c_int = (*self_0).framesCount;
        r = *((*self_0).frames).offset((i + TWOCOLOR_PREV_R) as isize);
        g = *((*self_0).frames).offset((i + TWOCOLOR_PREV_G) as isize);
        b = *((*self_0).frames).offset((i + TWOCOLOR_PREV_B) as isize);
        a = *((*self_0).frames).offset((i + TWOCOLOR_PREV_A) as isize);
        r2 = *((*self_0).frames).offset((i + TWOCOLOR_PREV_R2) as isize);
        g2 = *((*self_0).frames).offset((i + TWOCOLOR_PREV_G2) as isize);
        b2 = *((*self_0).frames).offset((i + TWOCOLOR_PREV_B2) as isize);
    } else {
        frame = binarySearch(
            (*self_0).frames,
            (*self_0).framesCount,
            time,
            TWOCOLOR_ENTRIES,
        );
        r = *((*self_0).frames).offset((frame + TWOCOLOR_PREV_R) as isize);
        g = *((*self_0).frames).offset((frame + TWOCOLOR_PREV_G) as isize);
        b = *((*self_0).frames).offset((frame + TWOCOLOR_PREV_B) as isize);
        a = *((*self_0).frames).offset((frame + TWOCOLOR_PREV_A) as isize);
        r2 = *((*self_0).frames).offset((frame + TWOCOLOR_PREV_R2) as isize);
        g2 = *((*self_0).frames).offset((frame + TWOCOLOR_PREV_G2) as isize);
        b2 = *((*self_0).frames).offset((frame + TWOCOLOR_PREV_B2) as isize);
        frameTime = *((*self_0).frames).offset(frame as isize);
        percent = spCurveTimeline_getCurvePercent(
            &mut (*self_0).super_0,
            frame / TWOCOLOR_ENTRIES - 1 as c_int,
            1 as c_int as c_float
                - (time - frameTime)
                    / (*((*self_0).frames).offset((frame + TWOCOLOR_PREV_TIME) as isize)
                        - frameTime),
        );
        r += (*((*self_0).frames).offset((frame + TWOCOLOR_R) as isize) - r) * percent;
        g += (*((*self_0).frames).offset((frame + TWOCOLOR_G) as isize) - g) * percent;
        b += (*((*self_0).frames).offset((frame + TWOCOLOR_B) as isize) - b) * percent;
        a += (*((*self_0).frames).offset((frame + TWOCOLOR_A) as isize) - a) * percent;
        r2
            += (*((*self_0).frames).offset((frame + TWOCOLOR_R2) as isize) - r2)
                * percent;
        g2
            += (*((*self_0).frames).offset((frame + TWOCOLOR_G2) as isize) - g2)
                * percent;
        b2
            += (*((*self_0).frames).offset((frame + TWOCOLOR_B2) as isize) - b2)
                * percent;
    }
    if alpha == 1 as c_int as c_float {
        spColor_setFromFloats(&mut (*slot).color, r, g, b, a);
        spColor_setFromFloats(
            (*slot).darkColor,
            r2,
            g2,
            b2,
            1 as c_int as c_float,
        );
    } else {
        light = &mut (*slot).color;
        dark = (*slot).darkColor;
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
        spColor_addFloats(
            dark,
            (r2 - (*dark).r) * alpha,
            (g2 - (*dark).g) * alpha,
            (b2 - (*dark).b) * alpha,
            0 as c_int as c_float,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn _spTwoColorTimeline_getPropertyId(
    mut timeline: *const spTimeline,
) -> c_int {
    return ((SP_TIMELINE_TWOCOLOR as c_int) << 24 as c_int)
        + (*(timeline as *mut spTwoColorTimeline)).slotIndex;
}
#[no_mangle]
pub unsafe extern "C" fn spTwoColorTimeline_create(
    mut framesCount: c_int,
) -> *mut spTwoColorTimeline {
    return _spBaseTimeline_create(
        framesCount,
        SP_TIMELINE_TWOCOLOR,
        TWOCOLOR_ENTRIES,
        Some(
            _spTwoColorTimeline_apply
                as unsafe extern "C" fn(
                    *const spTimeline,
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
            _spTwoColorTimeline_getPropertyId
                as unsafe extern "C" fn(*const spTimeline) -> c_int,
        ),
    ) as *mut spTwoColorTimeline;
}
#[no_mangle]
pub unsafe extern "C" fn spTwoColorTimeline_setFrame(
    mut self_0: *mut spTwoColorTimeline,
    mut frameIndex: c_int,
    mut time: c_float,
    mut r: c_float,
    mut g: c_float,
    mut b: c_float,
    mut a: c_float,
    mut r2: c_float,
    mut g2: c_float,
    mut b2: c_float,
) {
    frameIndex *= TWOCOLOR_ENTRIES;
    *((*self_0).frames).offset(frameIndex as isize) = time;
    *((*self_0).frames).offset((frameIndex + TWOCOLOR_R) as isize) = r;
    *((*self_0).frames).offset((frameIndex + TWOCOLOR_G) as isize) = g;
    *((*self_0).frames).offset((frameIndex + TWOCOLOR_B) as isize) = b;
    *((*self_0).frames).offset((frameIndex + TWOCOLOR_A) as isize) = a;
    *((*self_0).frames).offset((frameIndex + TWOCOLOR_R2) as isize) = r2;
    *((*self_0).frames).offset((frameIndex + TWOCOLOR_G2) as isize) = g2;
    *((*self_0).frames).offset((frameIndex + TWOCOLOR_B2) as isize) = b2;
}
unsafe extern "C" fn _spSetAttachment(
    mut timeline: *mut spAttachmentTimeline,
    mut skeleton: *mut spSkeleton,
    mut slot: *mut spSlot,
    mut attachmentName: *const c_char,
) {
    (*slot)
        .attachment = if attachmentName.is_null() {
        0 as *mut spAttachment
    } else {
        spSkeleton_getAttachmentForSlotIndex(
            skeleton,
            (*timeline).slotIndex,
            attachmentName,
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn _spAttachmentTimeline_apply(
    mut timeline: *const spTimeline,
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
    let mut frameIndex: c_int = 0;
    let mut slot: *mut spSlot = *((*skeleton).slots)
        .offset((*self_0).slotIndex as isize);
    if (*(*slot).bone).active == 0 {
        return;
    }
    if direction as c_uint == SP_MIX_DIRECTION_OUT as c_int as c_uint {
        if blend as c_uint == SP_MIX_BLEND_SETUP as c_int as c_uint {
            _spSetAttachment(self_0, skeleton, slot, (*(*slot).data).attachmentName);
        }
        return;
    }
    if time < *((*self_0).frames).offset(0 as c_int as isize) {
        if blend as c_uint == SP_MIX_BLEND_SETUP as c_int as c_uint
            || blend as c_uint == SP_MIX_BLEND_FIRST as c_int as c_uint
        {
            _spSetAttachment(self_0, skeleton, slot, (*(*slot).data).attachmentName);
        }
        return;
    }
    if time
        >= *((*self_0).frames)
            .offset(((*self_0).framesCount - 1 as c_int) as isize)
    {
        frameIndex = (*self_0).framesCount - 1 as c_int;
    } else {
        frameIndex = binarySearch1((*self_0).frames, (*self_0).framesCount, time)
            - 1 as c_int;
    }
    attachmentName = *((*self_0).attachmentNames).offset(frameIndex as isize);
    spSlot_setAttachment(
        *((*skeleton).slots).offset((*self_0).slotIndex as isize),
        if !attachmentName.is_null() {
            spSkeleton_getAttachmentForSlotIndex(
                skeleton,
                (*self_0).slotIndex,
                attachmentName,
            )
        } else {
            0 as *mut spAttachment
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn _spAttachmentTimeline_getPropertyId(
    mut timeline: *const spTimeline,
) -> c_int {
    return ((SP_TIMELINE_ATTACHMENT as c_int) << 24 as c_int)
        + (*(timeline as *mut spAttachmentTimeline)).slotIndex;
}
#[no_mangle]
pub unsafe extern "C" fn _spAttachmentTimeline_dispose(mut timeline: *mut spTimeline) {
    let mut self_0: *mut spAttachmentTimeline = timeline as *mut spAttachmentTimeline;
    let mut i: c_int = 0;
    _spTimeline_deinit(timeline);
    i = 0 as c_int;
    while i < (*self_0).framesCount {
        _spFree(*((*self_0).attachmentNames).offset(i as isize) as *mut c_void);
        i += 1;
    }
    _spFree((*self_0).attachmentNames as *mut c_void);
    _spFree((*self_0).frames as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spAttachmentTimeline_create(
    mut framesCount: c_int,
) -> *mut spAttachmentTimeline {
    let mut self_0: *mut spAttachmentTimeline = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spAttachmentTimeline>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        882 as c_int,
    ) as *mut spAttachmentTimeline;
    _spTimeline_init(
        &mut (*self_0).super_0,
        SP_TIMELINE_ATTACHMENT,
        Some(
            _spAttachmentTimeline_dispose as unsafe extern "C" fn(*mut spTimeline) -> (),
        ),
        Some(
            _spAttachmentTimeline_apply
                as unsafe extern "C" fn(
                    *const spTimeline,
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
            _spAttachmentTimeline_getPropertyId
                as unsafe extern "C" fn(*const spTimeline) -> c_int,
        ),
    );
    *(&(*self_0).framesCount as *const c_int as *mut c_int) = framesCount;
    let ref mut fresh7 = *(&(*self_0).frames as *const *mut c_float
        as *mut *mut c_float);
    *fresh7 = _spCalloc(
        framesCount as size_t,
        ::core::mem::size_of::<c_float>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        886 as c_int,
    ) as *mut c_float;
    let ref mut fresh8 = *(&(*self_0).attachmentNames as *const *mut *const c_char
        as *mut *mut *mut c_char);
    *fresh8 = _spCalloc(
        framesCount as size_t,
        ::core::mem::size_of::<*mut c_char>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        887 as c_int,
    ) as *mut *mut c_char;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spAttachmentTimeline_setFrame(
    mut self_0: *mut spAttachmentTimeline,
    mut frameIndex: c_int,
    mut time: c_float,
    mut attachmentName: *const c_char,
) {
    *((*self_0).frames).offset(frameIndex as isize) = time;
    _spFree(
        *((*self_0).attachmentNames).offset(frameIndex as isize) as *mut c_void,
    );
    if !attachmentName.is_null() {
        let ref mut fresh9 = *(&mut *((*self_0).attachmentNames)
            .offset(frameIndex as isize) as *mut *const c_char
            as *mut *mut c_char);
        *fresh9 = _spMalloc(
            (::core::mem::size_of::<c_char>() as c_ulong)
                .wrapping_mul(
                    (spine_strlen(attachmentName))
                        .wrapping_add(1 as c_int as c_ulong),
                ),
            b"spine.c\0" as *const u8 as *const c_char,
            897 as c_int,
        ) as *mut c_char;
        spine_strcpy(*fresh9, attachmentName);
    } else {
        let ref mut fresh10 = *((*self_0).attachmentNames).offset(frameIndex as isize);
        *fresh10 = 0 as *const c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn _spDeformTimeline_apply(
    mut timeline: *const spTimeline,
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
    let mut frameTime: c_float = 0.;
    let mut prevVertices: *const c_float = 0 as *const c_float;
    let mut nextVertices: *const c_float = 0 as *const c_float;
    let mut frames: *mut c_float = 0 as *mut c_float;
    let mut framesCount: c_int = 0;
    let mut frameVertices: *mut *const c_float = 0 as *mut *const c_float;
    let mut deformArray: *mut c_float = 0 as *mut c_float;
    let mut self_0: *mut spDeformTimeline = timeline as *mut spDeformTimeline;
    let mut slot: *mut spSlot = *((*skeleton).slots)
        .offset((*self_0).slotIndex as isize);
    if (*(*slot).bone).active == 0 {
        return;
    }
    if ((*slot).attachment).is_null() {
        return;
    }
    match (*(*slot).attachment).type_0 as c_uint {
        1 | 6 | 2 | 4 => {
            let mut vertexAttachment: *mut spVertexAttachment = (*slot).attachment
                as *mut spVertexAttachment;
            if (*vertexAttachment).deformAttachment
                != (*self_0).attachment as *mut spVertexAttachment
            {
                return;
            }
        }
        _ => return,
    }
    frames = (*self_0).frames;
    framesCount = (*self_0).framesCount;
    vertexCount = (*self_0).frameVerticesCount;
    if (*slot).deformCount < vertexCount {
        if (*slot).deformCapacity < vertexCount {
            _spFree((*slot).deform as *mut c_void);
            (*slot)
                .deform = _spMalloc(
                (::core::mem::size_of::<c_float>() as c_ulong)
                    .wrapping_mul(vertexCount as c_ulong),
                b"spine.c\0" as *const u8 as *const c_char,
                940 as c_int,
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
        let mut vertexAttachment_0: *mut spVertexAttachment = (*slot).attachment
            as *mut spVertexAttachment;
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
                    let mut setupVertices: *mut c_float = (*vertexAttachment_0)
                        .vertices;
                    i = 0 as c_int;
                    while i < vertexCount {
                        *deformArray.offset(i as isize)
                            += (*setupVertices.offset(i as isize)
                                - *deformArray.offset(i as isize)) * alpha;
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
        let mut lastVertices: *const c_float = *((*self_0).frameVertices)
            .offset((framesCount - 1 as c_int) as isize);
        if alpha == 1 as c_int as c_float {
            if blend as c_uint == SP_MIX_BLEND_ADD as c_int as c_uint {
                let mut vertexAttachment_1: *mut spVertexAttachment = (*slot).attachment
                    as *mut spVertexAttachment;
                if ((*vertexAttachment_1).bones).is_null() {
                    let mut setupVertices_0: *mut c_float = (*vertexAttachment_1)
                        .vertices;
                    i = 0 as c_int;
                    while i < vertexCount {
                        *deformArray.offset(i as isize)
                            += *lastVertices.offset(i as isize)
                                - *setupVertices_0.offset(i as isize);
                        i += 1;
                    }
                } else {
                    i = 0 as c_int;
                    while i < vertexCount {
                        *deformArray.offset(i as isize)
                            += *lastVertices.offset(i as isize);
                        i += 1;
                    }
                }
            } else {
                spine_memcpy(
                    deformArray as *mut c_void,
                    lastVertices as *const c_void,
                    (vertexCount as c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<c_float>() as c_ulong,
                        ),
                );
            }
        } else {
            let mut vertexAttachment_2: *mut spVertexAttachment = 0
                as *mut spVertexAttachment;
            let mut current_block_86: u64;
            match blend as c_uint {
                0 => {
                    vertexAttachment_2 = (*slot).attachment as *mut spVertexAttachment;
                    if ((*vertexAttachment_2).bones).is_null() {
                        let mut setupVertices_1: *mut c_float = (*vertexAttachment_2)
                            .vertices;
                        i = 0 as c_int;
                        while i < vertexCount {
                            let mut setup: c_float = *setupVertices_1
                                .offset(i as isize);
                            *deformArray
                                .offset(
                                    i as isize,
                                ) = setup
                                + (*lastVertices.offset(i as isize) - setup) * alpha;
                            i += 1;
                        }
                    } else {
                        i = 0 as c_int;
                        while i < vertexCount {
                            *deformArray
                                .offset(
                                    i as isize,
                                ) = *lastVertices.offset(i as isize) * alpha;
                            i += 1;
                        }
                    }
                    current_block_86 = 15864857819291709765;
                }
                1 | 2 => {
                    i = 0 as c_int;
                    while i < vertexCount {
                        *deformArray.offset(i as isize)
                            += (*lastVertices.offset(i as isize)
                                - *deformArray.offset(i as isize)) * alpha;
                        i += 1;
                    }
                    current_block_86 = 3299723876590891669;
                }
                3 => {
                    current_block_86 = 3299723876590891669;
                }
                _ => {
                    current_block_86 = 15864857819291709765;
                }
            }
            match current_block_86 {
                3299723876590891669 => {
                    vertexAttachment_2 = (*slot).attachment as *mut spVertexAttachment;
                    if ((*vertexAttachment_2).bones).is_null() {
                        let mut setupVertices_2: *mut c_float = (*vertexAttachment_2)
                            .vertices;
                        i = 0 as c_int;
                        while i < vertexCount {
                            *deformArray.offset(i as isize)
                                += (*lastVertices.offset(i as isize)
                                    - *setupVertices_2.offset(i as isize)) * alpha;
                            i += 1;
                        }
                    } else {
                        i = 0 as c_int;
                        while i < vertexCount {
                            *deformArray.offset(i as isize)
                                += *lastVertices.offset(i as isize) * alpha;
                            i += 1;
                        }
                    }
                }
                _ => {}
            }
        }
        return;
    }
    frame = binarySearch(frames, framesCount, time, 1 as c_int);
    prevVertices = *frameVertices.offset((frame - 1 as c_int) as isize);
    nextVertices = *frameVertices.offset(frame as isize);
    frameTime = *frames.offset(frame as isize);
    percent = spCurveTimeline_getCurvePercent(
        &mut (*self_0).super_0,
        frame - 1 as c_int,
        1 as c_int as c_float
            - (time - frameTime)
                / (*frames.offset((frame - 1 as c_int) as isize) - frameTime),
    );
    if alpha == 1 as c_int as c_float {
        if blend as c_uint == SP_MIX_BLEND_ADD as c_int as c_uint {
            let mut vertexAttachment_3: *mut spVertexAttachment = (*slot).attachment
                as *mut spVertexAttachment;
            if ((*vertexAttachment_3).bones).is_null() {
                let mut setupVertices_3: *mut c_float = (*vertexAttachment_3)
                    .vertices;
                i = 0 as c_int;
                while i < vertexCount {
                    let mut prev: c_float = *prevVertices.offset(i as isize);
                    *deformArray.offset(i as isize)
                        += prev + (*nextVertices.offset(i as isize) - prev) * percent
                            - *setupVertices_3.offset(i as isize);
                    i += 1;
                }
            } else {
                i = 0 as c_int;
                while i < vertexCount {
                    let mut prev_0: c_float = *prevVertices.offset(i as isize);
                    *deformArray.offset(i as isize)
                        += prev_0
                            + (*nextVertices.offset(i as isize) - prev_0) * percent;
                    i += 1;
                }
            }
        } else {
            i = 0 as c_int;
            while i < vertexCount {
                let mut prev_1: c_float = *prevVertices.offset(i as isize);
                *deformArray
                    .offset(
                        i as isize,
                    ) = prev_1 + (*nextVertices.offset(i as isize) - prev_1) * percent;
                i += 1;
            }
        }
    } else {
        let mut vertexAttachment_4: *mut spVertexAttachment = 0
            as *mut spVertexAttachment;
        match blend as c_uint {
            0 => {
                vertexAttachment_4 = (*slot).attachment as *mut spVertexAttachment;
                if ((*vertexAttachment_4).bones).is_null() {
                    let mut setupVertices_4: *mut c_float = (*vertexAttachment_4)
                        .vertices;
                    i = 0 as c_int;
                    while i < vertexCount {
                        let mut prev_2: c_float = *prevVertices.offset(i as isize);
                        let mut setup_0: c_float = *setupVertices_4
                            .offset(i as isize);
                        *deformArray
                            .offset(
                                i as isize,
                            ) = setup_0
                            + (prev_2
                                + (*nextVertices.offset(i as isize) - prev_2) * percent
                                - setup_0) * alpha;
                        i += 1;
                    }
                } else {
                    i = 0 as c_int;
                    while i < vertexCount {
                        let mut prev_3: c_float = *prevVertices.offset(i as isize);
                        *deformArray
                            .offset(
                                i as isize,
                            ) = (prev_3
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
                    *deformArray.offset(i as isize)
                        += (prev_4
                            + (*nextVertices.offset(i as isize) - prev_4) * percent
                            - *deformArray.offset(i as isize)) * alpha;
                    i += 1;
                }
            }
            3 => {
                vertexAttachment_4 = (*slot).attachment as *mut spVertexAttachment;
                if ((*vertexAttachment_4).bones).is_null() {
                    let mut setupVertices_5: *mut c_float = (*vertexAttachment_4)
                        .vertices;
                    i = 0 as c_int;
                    while i < vertexCount {
                        let mut prev_5: c_float = *prevVertices.offset(i as isize);
                        *deformArray.offset(i as isize)
                            += (prev_5
                                + (*nextVertices.offset(i as isize) - prev_5) * percent
                                - *setupVertices_5.offset(i as isize)) * alpha;
                        i += 1;
                    }
                } else {
                    i = 0 as c_int;
                    while i < vertexCount {
                        let mut prev_6: c_float = *prevVertices.offset(i as isize);
                        *deformArray.offset(i as isize)
                            += (prev_6
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
pub unsafe extern "C" fn _spDeformTimeline_getPropertyId(
    mut timeline: *const spTimeline,
) -> c_int {
    return ((SP_TIMELINE_DEFORM as c_int) << 27 as c_int)
        + (*((*(timeline as *mut spDeformTimeline)).attachment
            as *mut spVertexAttachment))
            .id + (*(timeline as *mut spDeformTimeline)).slotIndex;
}
#[no_mangle]
pub unsafe extern "C" fn _spDeformTimeline_dispose(mut timeline: *mut spTimeline) {
    let mut self_0: *mut spDeformTimeline = timeline as *mut spDeformTimeline;
    let mut i: c_int = 0;
    _spCurveTimeline_deinit(&mut (*self_0).super_0);
    i = 0 as c_int;
    while i < (*self_0).framesCount {
        _spFree(*((*self_0).frameVertices).offset(i as isize) as *mut c_void);
        i += 1;
    }
    _spFree((*self_0).frameVertices as *mut c_void);
    _spFree((*self_0).frames as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spDeformTimeline_create(
    mut framesCount: c_int,
    mut frameVerticesCount: c_int,
) -> *mut spDeformTimeline {
    let mut self_0: *mut spDeformTimeline = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spDeformTimeline>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        1134 as c_int,
    ) as *mut spDeformTimeline;
    _spCurveTimeline_init(
        &mut (*self_0).super_0,
        SP_TIMELINE_DEFORM,
        framesCount,
        Some(_spDeformTimeline_dispose as unsafe extern "C" fn(*mut spTimeline) -> ()),
        Some(
            _spDeformTimeline_apply
                as unsafe extern "C" fn(
                    *const spTimeline,
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
            _spDeformTimeline_getPropertyId
                as unsafe extern "C" fn(*const spTimeline) -> c_int,
        ),
    );
    *(&(*self_0).framesCount as *const c_int as *mut c_int) = framesCount;
    let ref mut fresh11 = *(&(*self_0).frames as *const *mut c_float
        as *mut *mut c_float);
    *fresh11 = _spCalloc(
        (*self_0).framesCount as size_t,
        ::core::mem::size_of::<c_float>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        1137 as c_int,
    ) as *mut c_float;
    let ref mut fresh12 = *(&(*self_0).frameVertices as *const *mut *const c_float
        as *mut *mut *mut c_float);
    *fresh12 = _spCalloc(
        framesCount as size_t,
        ::core::mem::size_of::<*mut c_float>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        1138 as c_int,
    ) as *mut *mut c_float;
    *(&(*self_0).frameVerticesCount as *const c_int
        as *mut c_int) = frameVerticesCount;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spDeformTimeline_setFrame(
    mut self_0: *mut spDeformTimeline,
    mut frameIndex: c_int,
    mut time: c_float,
    mut vertices: *mut c_float,
) {
    *((*self_0).frames).offset(frameIndex as isize) = time;
    _spFree(*((*self_0).frameVertices).offset(frameIndex as isize) as *mut c_void);
    if vertices.is_null() {
        let ref mut fresh13 = *((*self_0).frameVertices).offset(frameIndex as isize);
        *fresh13 = 0 as *const c_float;
    } else {
        let ref mut fresh14 = *((*self_0).frameVertices).offset(frameIndex as isize);
        *fresh14 = _spMalloc(
            (::core::mem::size_of::<c_float>() as c_ulong)
                .wrapping_mul((*self_0).frameVerticesCount as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            1150 as c_int,
        ) as *mut c_float;
        spine_memcpy(
            *(&mut *((*self_0).frameVertices).offset(frameIndex as isize)
                as *mut *const c_float as *mut *mut c_float)
                as *mut c_void,
            vertices as *const c_void,
            ((*self_0).frameVerticesCount as c_ulong)
                .wrapping_mul(::core::mem::size_of::<c_float>() as c_ulong),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn _spEventTimeline_apply(
    mut timeline: *const spTimeline,
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
    let mut frame: c_int = 0;
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
    } else if lastTime
            >= *((*self_0).frames)
                .offset(((*self_0).framesCount - 1 as c_int) as isize)
        {
        return
    }
    if time < *((*self_0).frames).offset(0 as c_int as isize) {
        return;
    }
    if lastTime < *((*self_0).frames).offset(0 as c_int as isize) {
        frame = 0 as c_int;
    } else {
        let mut frameTime: c_float = 0.;
        frame = binarySearch1((*self_0).frames, (*self_0).framesCount, lastTime);
        frameTime = *((*self_0).frames).offset(frame as isize);
        while frame > 0 as c_int {
            if *((*self_0).frames).offset((frame - 1 as c_int) as isize)
                != frameTime
            {
                break;
            }
            frame -= 1;
        }
    }
    while frame < (*self_0).framesCount
        && time >= *((*self_0).frames).offset(frame as isize)
    {
        let ref mut fresh15 = *firedEvents.offset(*eventsCount as isize);
        *fresh15 = *((*self_0).events).offset(frame as isize);
        *eventsCount += 1;
        frame += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _spEventTimeline_getPropertyId(
    mut _timeline: *const spTimeline,
) -> c_int {
    return (SP_TIMELINE_EVENT as c_int) << 24 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _spEventTimeline_dispose(mut timeline: *mut spTimeline) {
    let mut self_0: *mut spEventTimeline = timeline as *mut spEventTimeline;
    let mut i: c_int = 0;
    _spTimeline_deinit(timeline);
    i = 0 as c_int;
    while i < (*self_0).framesCount {
        spEvent_dispose(*((*self_0).events).offset(i as isize));
        i += 1;
    }
    _spFree((*self_0).events as *mut c_void);
    _spFree((*self_0).frames as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spEventTimeline_create(
    mut framesCount: c_int,
) -> *mut spEventTimeline {
    let mut self_0: *mut spEventTimeline = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spEventTimeline>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        1210 as c_int,
    ) as *mut spEventTimeline;
    _spTimeline_init(
        &mut (*self_0).super_0,
        SP_TIMELINE_EVENT,
        Some(_spEventTimeline_dispose as unsafe extern "C" fn(*mut spTimeline) -> ()),
        Some(
            _spEventTimeline_apply
                as unsafe extern "C" fn(
                    *const spTimeline,
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
            _spEventTimeline_getPropertyId
                as unsafe extern "C" fn(*const spTimeline) -> c_int,
        ),
    );
    *(&(*self_0).framesCount as *const c_int as *mut c_int) = framesCount;
    let ref mut fresh16 = *(&(*self_0).frames as *const *mut c_float
        as *mut *mut c_float);
    *fresh16 = _spCalloc(
        framesCount as size_t,
        ::core::mem::size_of::<c_float>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        1214 as c_int,
    ) as *mut c_float;
    let ref mut fresh17 = *(&(*self_0).events as *const *mut *mut spEvent
        as *mut *mut *mut spEvent);
    *fresh17 = _spCalloc(
        framesCount as size_t,
        ::core::mem::size_of::<*mut spEvent>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        1215 as c_int,
    ) as *mut *mut spEvent;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spEventTimeline_setFrame(
    mut self_0: *mut spEventTimeline,
    mut frameIndex: c_int,
    mut event: *mut spEvent,
) {
    *((*self_0).frames).offset(frameIndex as isize) = (*event).time;
    _spFree(*((*self_0).events).offset(frameIndex as isize) as *mut c_void);
    let ref mut fresh18 = *((*self_0).events).offset(frameIndex as isize);
    *fresh18 = event;
}
#[no_mangle]
pub unsafe extern "C" fn _spDrawOrderTimeline_apply(
    mut timeline: *const spTimeline,
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
    let mut frame: c_int = 0;
    let mut drawOrderToSetupIndex: *const c_int = 0 as *const c_int;
    let mut self_0: *mut spDrawOrderTimeline = timeline as *mut spDrawOrderTimeline;
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
    if time < *((*self_0).frames).offset(0 as c_int as isize) {
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
    if time
        >= *((*self_0).frames)
            .offset(((*self_0).framesCount - 1 as c_int) as isize)
    {
        frame = (*self_0).framesCount - 1 as c_int;
    } else {
        frame = binarySearch1((*self_0).frames, (*self_0).framesCount, time)
            - 1 as c_int;
    }
    drawOrderToSetupIndex = *((*self_0).drawOrders).offset(frame as isize);
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
            let ref mut fresh19 = *((*skeleton).drawOrder).offset(i as isize);
            *fresh19 = *((*skeleton).slots)
                .offset(*drawOrderToSetupIndex.offset(i as isize) as isize);
            i += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _spDrawOrderTimeline_getPropertyId(
    mut _timeline: *const spTimeline,
) -> c_int {
    return (SP_TIMELINE_DRAWORDER as c_int) << 24 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _spDrawOrderTimeline_dispose(mut timeline: *mut spTimeline) {
    let mut self_0: *mut spDrawOrderTimeline = timeline as *mut spDrawOrderTimeline;
    let mut i: c_int = 0;
    _spTimeline_deinit(timeline);
    i = 0 as c_int;
    while i < (*self_0).framesCount {
        _spFree(*((*self_0).drawOrders).offset(i as isize) as *mut c_void);
        i += 1;
    }
    _spFree((*self_0).drawOrders as *mut c_void);
    _spFree((*self_0).frames as *mut c_void);
    _spFree(self_0 as *mut c_void);
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
        1285 as c_int,
    ) as *mut spDrawOrderTimeline;
    _spTimeline_init(
        &mut (*self_0).super_0,
        SP_TIMELINE_DRAWORDER,
        Some(
            _spDrawOrderTimeline_dispose as unsafe extern "C" fn(*mut spTimeline) -> (),
        ),
        Some(
            _spDrawOrderTimeline_apply
                as unsafe extern "C" fn(
                    *const spTimeline,
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
            _spDrawOrderTimeline_getPropertyId
                as unsafe extern "C" fn(*const spTimeline) -> c_int,
        ),
    );
    *(&(*self_0).framesCount as *const c_int as *mut c_int) = framesCount;
    let ref mut fresh20 = *(&(*self_0).frames as *const *mut c_float
        as *mut *mut c_float);
    *fresh20 = _spCalloc(
        framesCount as size_t,
        ::core::mem::size_of::<c_float>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        1289 as c_int,
    ) as *mut c_float;
    let ref mut fresh21 = *(&(*self_0).drawOrders as *const *mut *const c_int
        as *mut *mut *mut c_int);
    *fresh21 = _spCalloc(
        framesCount as size_t,
        ::core::mem::size_of::<*mut c_int>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        1290 as c_int,
    ) as *mut *mut c_int;
    *(&(*self_0).slotsCount as *const c_int as *mut c_int) = slotsCount;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spDrawOrderTimeline_setFrame(
    mut self_0: *mut spDrawOrderTimeline,
    mut frameIndex: c_int,
    mut time: c_float,
    mut drawOrder: *const c_int,
) {
    *((*self_0).frames).offset(frameIndex as isize) = time;
    _spFree(*((*self_0).drawOrders).offset(frameIndex as isize) as *mut c_void);
    if drawOrder.is_null() {
        let ref mut fresh22 = *((*self_0).drawOrders).offset(frameIndex as isize);
        *fresh22 = 0 as *const c_int;
    } else {
        let ref mut fresh23 = *((*self_0).drawOrders).offset(frameIndex as isize);
        *fresh23 = _spMalloc(
            (::core::mem::size_of::<c_int>() as c_ulong)
                .wrapping_mul((*self_0).slotsCount as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            1303 as c_int,
        ) as *mut c_int;
        spine_memcpy(
            *(&mut *((*self_0).drawOrders).offset(frameIndex as isize)
                as *mut *const c_int as *mut *mut c_int)
                as *mut c_void,
            drawOrder as *const c_void,
            ((*self_0).slotsCount as c_ulong)
                .wrapping_mul(::core::mem::size_of::<c_int>() as c_ulong),
        );
    };
}
static mut IKCONSTRAINT_PREV_TIME: c_int = -(6 as c_int);
static mut IKCONSTRAINT_PREV_MIX: c_int = -(5 as c_int);
static mut IKCONSTRAINT_PREV_SOFTNESS: c_int = -(4 as c_int);
static mut IKCONSTRAINT_PREV_BEND_DIRECTION: c_int = -(3 as c_int);
static mut IKCONSTRAINT_PREV_COMPRESS: c_int = -(2 as c_int);
static mut IKCONSTRAINT_PREV_STRETCH: c_int = -(1 as c_int);
static mut IKCONSTRAINT_MIX: c_int = 1 as c_int;
static mut IKCONSTRAINT_SOFTNESS: c_int = 2 as c_int;
static mut IKCONSTRAINT_BEND_DIRECTION: c_int = 3 as c_int;
static mut IKCONSTRAINT_COMPRESS: c_int = 4 as c_int;
static mut IKCONSTRAINT_STRETCH: c_int = 5 as c_int;
#[no_mangle]
pub unsafe extern "C" fn _spIkConstraintTimeline_apply(
    mut timeline: *const spTimeline,
    mut skeleton: *mut spSkeleton,
    mut _lastTime: c_float,
    mut time: c_float,
    mut _firedEvents: *mut *mut spEvent,
    mut _eventsCount: *mut c_int,
    mut alpha: c_float,
    mut blend: spMixBlend,
    mut direction: spMixDirection,
) {
    let mut frame: c_int = 0;
    let mut frameTime: c_float = 0.;
    let mut percent: c_float = 0.;
    let mut mix: c_float = 0.;
    let mut softness: c_float = 0.;
    let mut frames: *mut c_float = 0 as *mut c_float;
    let mut framesCount: c_int = 0;
    let mut constraint: *mut spIkConstraint = 0 as *mut spIkConstraint;
    let mut self_0: *mut spIkConstraintTimeline = timeline
        as *mut spIkConstraintTimeline;
    constraint = *((*skeleton).ikConstraints)
        .offset((*self_0).ikConstraintIndex as isize);
    if (*constraint).active == 0 {
        return;
    }
    if time < *((*self_0).frames).offset(0 as c_int as isize) {
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
                (*constraint).mix
                    += ((*(*constraint).data).mix - (*constraint).mix) * alpha;
                (*constraint).softness
                    += ((*(*constraint).data).softness - (*constraint).softness) * alpha;
                (*constraint).bendDirection = (*(*constraint).data).bendDirection;
                (*constraint).compress = (*(*constraint).data).compress;
                (*constraint).stretch = (*(*constraint).data).stretch;
            }
            2 | 3 | _ => {}
        }
        return;
    }
    frames = (*self_0).frames;
    framesCount = (*self_0).framesCount;
    if time >= *frames.offset((framesCount - IKCONSTRAINT_ENTRIES) as isize) {
        if blend as c_uint == SP_MIX_BLEND_SETUP as c_int as c_uint {
            (*constraint)
                .mix = (*(*constraint).data).mix
                + (*frames.offset((framesCount + IKCONSTRAINT_PREV_MIX) as isize)
                    - (*(*constraint).data).mix) * alpha;
            (*constraint)
                .softness = (*(*constraint).data).softness
                + (*frames.offset((framesCount + IKCONSTRAINT_PREV_SOFTNESS) as isize)
                    - (*(*constraint).data).softness) * alpha;
            if direction as c_uint
                == SP_MIX_DIRECTION_OUT as c_int as c_uint
            {
                (*constraint).bendDirection = (*(*constraint).data).bendDirection;
                (*constraint).compress = (*(*constraint).data).compress;
                (*constraint).stretch = (*(*constraint).data).stretch;
            } else {
                (*constraint)
                    .bendDirection = *frames
                    .offset((framesCount + IKCONSTRAINT_PREV_BEND_DIRECTION) as isize)
                    as c_int;
                (*constraint)
                    .compress = if *frames
                    .offset((framesCount + IKCONSTRAINT_PREV_COMPRESS) as isize) != 0.
                {
                    1 as c_int
                } else {
                    0 as c_int
                };
                (*constraint)
                    .stretch = if *frames
                    .offset((framesCount + IKCONSTRAINT_PREV_STRETCH) as isize) != 0.
                {
                    1 as c_int
                } else {
                    0 as c_int
                };
            }
        } else {
            (*constraint).mix
                += (*frames.offset((framesCount + IKCONSTRAINT_PREV_MIX) as isize)
                    - (*constraint).mix) * alpha;
            (*constraint).softness
                += (*frames.offset((framesCount + IKCONSTRAINT_PREV_SOFTNESS) as isize)
                    - (*constraint).softness) * alpha;
            if direction as c_uint
                == SP_MIX_DIRECTION_IN as c_int as c_uint
            {
                (*constraint)
                    .bendDirection = *frames
                    .offset((framesCount + IKCONSTRAINT_PREV_BEND_DIRECTION) as isize)
                    as c_int;
                (*constraint)
                    .compress = if *frames
                    .offset((framesCount + IKCONSTRAINT_PREV_COMPRESS) as isize) != 0.
                {
                    1 as c_int
                } else {
                    0 as c_int
                };
                (*constraint)
                    .stretch = if *frames
                    .offset((framesCount + IKCONSTRAINT_PREV_STRETCH) as isize) != 0.
                {
                    1 as c_int
                } else {
                    0 as c_int
                };
            }
        }
        return;
    }
    frame = binarySearch(
        (*self_0).frames,
        (*self_0).framesCount,
        time,
        IKCONSTRAINT_ENTRIES,
    );
    mix = *((*self_0).frames).offset((frame + IKCONSTRAINT_PREV_MIX) as isize);
    softness = *frames.offset((frame + IKCONSTRAINT_PREV_SOFTNESS) as isize);
    frameTime = *((*self_0).frames).offset(frame as isize);
    percent = spCurveTimeline_getCurvePercent(
        &mut (*self_0).super_0,
        frame / IKCONSTRAINT_ENTRIES - 1 as c_int,
        1 as c_int as c_float
            - (time - frameTime)
                / (*((*self_0).frames).offset((frame + IKCONSTRAINT_PREV_TIME) as isize)
                    - frameTime),
    );
    if blend as c_uint == SP_MIX_BLEND_SETUP as c_int as c_uint {
        (*constraint)
            .mix = (*(*constraint).data).mix
            + (mix
                + (*frames.offset((frame + IKCONSTRAINT_MIX) as isize) - mix) * percent
                - (*(*constraint).data).mix) * alpha;
        (*constraint)
            .softness = (*(*constraint).data).softness
            + (softness
                + (*frames.offset((frame + IKCONSTRAINT_SOFTNESS) as isize) - softness)
                    * percent - (*(*constraint).data).softness) * alpha;
        if direction as c_uint
            == SP_MIX_DIRECTION_OUT as c_int as c_uint
        {
            (*constraint).bendDirection = (*(*constraint).data).bendDirection;
            (*constraint).compress = (*(*constraint).data).compress;
            (*constraint).stretch = (*(*constraint).data).stretch;
        } else {
            (*constraint)
                .bendDirection = *frames
                .offset((frame + IKCONSTRAINT_PREV_BEND_DIRECTION) as isize)
                as c_int;
            (*constraint)
                .compress = if *frames
                .offset((frame + IKCONSTRAINT_PREV_COMPRESS) as isize) != 0.
            {
                1 as c_int
            } else {
                0 as c_int
            };
            (*constraint)
                .stretch = if *frames
                .offset((frame + IKCONSTRAINT_PREV_STRETCH) as isize) != 0.
            {
                1 as c_int
            } else {
                0 as c_int
            };
        }
    } else {
        (*constraint).mix
            += (mix
                + (*frames.offset((frame + IKCONSTRAINT_MIX) as isize) - mix) * percent
                - (*constraint).mix) * alpha;
        (*constraint).softness
            += (softness
                + (*frames.offset((frame + IKCONSTRAINT_SOFTNESS) as isize) - softness)
                    * percent - (*constraint).softness) * alpha;
        if direction as c_uint
            == SP_MIX_DIRECTION_IN as c_int as c_uint
        {
            (*constraint)
                .bendDirection = *frames
                .offset((frame + IKCONSTRAINT_PREV_BEND_DIRECTION) as isize)
                as c_int;
            (*constraint)
                .compress = if *frames
                .offset((frame + IKCONSTRAINT_PREV_COMPRESS) as isize) != 0.
            {
                1 as c_int
            } else {
                0 as c_int
            };
            (*constraint)
                .stretch = if *frames
                .offset((frame + IKCONSTRAINT_PREV_STRETCH) as isize) != 0.
            {
                1 as c_int
            } else {
                0 as c_int
            };
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _spIkConstraintTimeline_getPropertyId(
    mut timeline: *const spTimeline,
) -> c_int {
    return ((SP_TIMELINE_IKCONSTRAINT as c_int) << 24 as c_int)
        + (*(timeline as *mut spIkConstraintTimeline)).ikConstraintIndex;
}
#[no_mangle]
pub unsafe extern "C" fn spIkConstraintTimeline_create(
    mut framesCount: c_int,
) -> *mut spIkConstraintTimeline {
    return _spBaseTimeline_create(
        framesCount,
        SP_TIMELINE_IKCONSTRAINT,
        IKCONSTRAINT_ENTRIES,
        Some(
            _spIkConstraintTimeline_apply
                as unsafe extern "C" fn(
                    *const spTimeline,
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
            _spIkConstraintTimeline_getPropertyId
                as unsafe extern "C" fn(*const spTimeline) -> c_int,
        ),
    ) as *mut spIkConstraintTimeline;
}
#[no_mangle]
pub unsafe extern "C" fn spIkConstraintTimeline_setFrame(
    mut self_0: *mut spIkConstraintTimeline,
    mut frameIndex: c_int,
    mut time: c_float,
    mut mix: c_float,
    mut softness: c_float,
    mut bendDirection: c_int,
    mut compress: c_int,
    mut stretch: c_int,
) {
    frameIndex *= IKCONSTRAINT_ENTRIES;
    *((*self_0).frames).offset(frameIndex as isize) = time;
    *((*self_0).frames).offset((frameIndex + IKCONSTRAINT_MIX) as isize) = mix;
    *((*self_0).frames).offset((frameIndex + IKCONSTRAINT_SOFTNESS) as isize) = softness;
    *((*self_0).frames)
        .offset(
            (frameIndex + IKCONSTRAINT_BEND_DIRECTION) as isize,
        ) = bendDirection as c_float;
    *((*self_0).frames)
        .offset(
            (frameIndex + IKCONSTRAINT_COMPRESS) as isize,
        ) = (if compress != 0 { 1 as c_int } else { 0 as c_int })
        as c_float;
    *((*self_0).frames)
        .offset(
            (frameIndex + IKCONSTRAINT_STRETCH) as isize,
        ) = (if stretch != 0 { 1 as c_int } else { 0 as c_int })
        as c_float;
}
static mut TRANSFORMCONSTRAINT_PREV_TIME: c_int = -(5 as c_int);
static mut TRANSFORMCONSTRAINT_PREV_ROTATE: c_int = -(4 as c_int);
static mut TRANSFORMCONSTRAINT_PREV_TRANSLATE: c_int = -(3 as c_int);
static mut TRANSFORMCONSTRAINT_PREV_SCALE: c_int = -(2 as c_int);
static mut TRANSFORMCONSTRAINT_PREV_SHEAR: c_int = -(1 as c_int);
static mut TRANSFORMCONSTRAINT_ROTATE: c_int = 1 as c_int;
static mut TRANSFORMCONSTRAINT_TRANSLATE: c_int = 2 as c_int;
static mut TRANSFORMCONSTRAINT_SCALE: c_int = 3 as c_int;
static mut TRANSFORMCONSTRAINT_SHEAR: c_int = 4 as c_int;
#[no_mangle]
pub unsafe extern "C" fn _spTransformConstraintTimeline_apply(
    mut timeline: *const spTimeline,
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
    let mut frameTime: c_float = 0.;
    let mut percent: c_float = 0.;
    let mut rotate: c_float = 0.;
    let mut translate: c_float = 0.;
    let mut scale: c_float = 0.;
    let mut shear: c_float = 0.;
    let mut constraint: *mut spTransformConstraint = 0 as *mut spTransformConstraint;
    let mut self_0: *mut spTransformConstraintTimeline = timeline
        as *mut spTransformConstraintTimeline;
    let mut frames: *mut c_float = 0 as *mut c_float;
    let mut framesCount: c_int = 0;
    constraint = *((*skeleton).transformConstraints)
        .offset((*self_0).transformConstraintIndex as isize);
    if (*constraint).active == 0 {
        return;
    }
    if time < *((*self_0).frames).offset(0 as c_int as isize) {
        let mut data: *mut spTransformConstraintData = (*constraint).data;
        match blend as c_uint {
            0 => {
                (*constraint).rotateMix = (*data).rotateMix;
                (*constraint).translateMix = (*data).translateMix;
                (*constraint).scaleMix = (*data).scaleMix;
                (*constraint).shearMix = (*data).shearMix;
                return;
            }
            1 => {
                (*constraint).rotateMix
                    += ((*data).rotateMix - (*constraint).rotateMix) * alpha;
                (*constraint).translateMix
                    += ((*data).translateMix - (*constraint).translateMix) * alpha;
                (*constraint).scaleMix
                    += ((*data).scaleMix - (*constraint).scaleMix) * alpha;
                (*constraint).shearMix
                    += ((*data).shearMix - (*constraint).shearMix) * alpha;
            }
            2 | 3 | _ => {}
        }
        return;
    }
    frames = (*self_0).frames;
    framesCount = (*self_0).framesCount;
    if time >= *frames.offset((framesCount - TRANSFORMCONSTRAINT_ENTRIES) as isize) {
        let mut i: c_int = framesCount;
        rotate = *frames.offset((i + TRANSFORMCONSTRAINT_PREV_ROTATE) as isize);
        translate = *frames.offset((i + TRANSFORMCONSTRAINT_PREV_TRANSLATE) as isize);
        scale = *frames.offset((i + TRANSFORMCONSTRAINT_PREV_SCALE) as isize);
        shear = *frames.offset((i + TRANSFORMCONSTRAINT_PREV_SHEAR) as isize);
    } else {
        frame = binarySearch(frames, framesCount, time, TRANSFORMCONSTRAINT_ENTRIES);
        rotate = *frames.offset((frame + TRANSFORMCONSTRAINT_PREV_ROTATE) as isize);
        translate = *frames
            .offset((frame + TRANSFORMCONSTRAINT_PREV_TRANSLATE) as isize);
        scale = *frames.offset((frame + TRANSFORMCONSTRAINT_PREV_SCALE) as isize);
        shear = *frames.offset((frame + TRANSFORMCONSTRAINT_PREV_SHEAR) as isize);
        frameTime = *frames.offset(frame as isize);
        percent = spCurveTimeline_getCurvePercent(
            &mut (*self_0).super_0,
            frame / TRANSFORMCONSTRAINT_ENTRIES - 1 as c_int,
            1 as c_int as c_float
                - (time - frameTime)
                    / (*frames.offset((frame + TRANSFORMCONSTRAINT_PREV_TIME) as isize)
                        - frameTime),
        );
        rotate
            += (*frames.offset((frame + TRANSFORMCONSTRAINT_ROTATE) as isize) - rotate)
                * percent;
        translate
            += (*frames.offset((frame + TRANSFORMCONSTRAINT_TRANSLATE) as isize)
                - translate) * percent;
        scale
            += (*frames.offset((frame + TRANSFORMCONSTRAINT_SCALE) as isize) - scale)
                * percent;
        shear
            += (*frames.offset((frame + TRANSFORMCONSTRAINT_SHEAR) as isize) - shear)
                * percent;
    }
    if blend as c_uint == SP_MIX_BLEND_SETUP as c_int as c_uint {
        let mut data_0: *mut spTransformConstraintData = (*constraint).data;
        (*constraint)
            .rotateMix = (*data_0).rotateMix + (rotate - (*data_0).rotateMix) * alpha;
        (*constraint)
            .translateMix = (*data_0).translateMix
            + (translate - (*data_0).translateMix) * alpha;
        (*constraint)
            .scaleMix = (*data_0).scaleMix + (scale - (*data_0).scaleMix) * alpha;
        (*constraint)
            .shearMix = (*data_0).shearMix + (shear - (*data_0).shearMix) * alpha;
    } else {
        (*constraint).rotateMix += (rotate - (*constraint).rotateMix) * alpha;
        (*constraint).translateMix += (translate - (*constraint).translateMix) * alpha;
        (*constraint).scaleMix += (scale - (*constraint).scaleMix) * alpha;
        (*constraint).shearMix += (shear - (*constraint).shearMix) * alpha;
    };
}
#[no_mangle]
pub unsafe extern "C" fn _spTransformConstraintTimeline_getPropertyId(
    mut timeline: *const spTimeline,
) -> c_int {
    return ((SP_TIMELINE_TRANSFORMCONSTRAINT as c_int) << 24 as c_int)
        + (*(timeline as *mut spTransformConstraintTimeline)).transformConstraintIndex;
}
#[no_mangle]
pub unsafe extern "C" fn spTransformConstraintTimeline_create(
    mut framesCount: c_int,
) -> *mut spTransformConstraintTimeline {
    return _spBaseTimeline_create(
        framesCount,
        SP_TIMELINE_TRANSFORMCONSTRAINT,
        TRANSFORMCONSTRAINT_ENTRIES,
        Some(
            _spTransformConstraintTimeline_apply
                as unsafe extern "C" fn(
                    *const spTimeline,
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
            _spTransformConstraintTimeline_getPropertyId
                as unsafe extern "C" fn(*const spTimeline) -> c_int,
        ),
    ) as *mut spTransformConstraintTimeline;
}
#[no_mangle]
pub unsafe extern "C" fn spTransformConstraintTimeline_setFrame(
    mut self_0: *mut spTransformConstraintTimeline,
    mut frameIndex: c_int,
    mut time: c_float,
    mut rotateMix: c_float,
    mut translateMix: c_float,
    mut scaleMix: c_float,
    mut shearMix: c_float,
) {
    frameIndex *= TRANSFORMCONSTRAINT_ENTRIES;
    *((*self_0).frames).offset(frameIndex as isize) = time;
    *((*self_0).frames)
        .offset((frameIndex + TRANSFORMCONSTRAINT_ROTATE) as isize) = rotateMix;
    *((*self_0).frames)
        .offset((frameIndex + TRANSFORMCONSTRAINT_TRANSLATE) as isize) = translateMix;
    *((*self_0).frames)
        .offset((frameIndex + TRANSFORMCONSTRAINT_SCALE) as isize) = scaleMix;
    *((*self_0).frames)
        .offset((frameIndex + TRANSFORMCONSTRAINT_SHEAR) as isize) = shearMix;
}
static mut PATHCONSTRAINTPOSITION_PREV_TIME: c_int = -(2 as c_int);
static mut PATHCONSTRAINTPOSITION_PREV_VALUE: c_int = -(1 as c_int);
static mut PATHCONSTRAINTPOSITION_VALUE: c_int = 1 as c_int;
#[no_mangle]
pub unsafe extern "C" fn _spPathConstraintPositionTimeline_apply(
    mut timeline: *const spTimeline,
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
    let mut frameTime: c_float = 0.;
    let mut percent: c_float = 0.;
    let mut position: c_float = 0.;
    let mut constraint: *mut spPathConstraint = 0 as *mut spPathConstraint;
    let mut self_0: *mut spPathConstraintPositionTimeline = timeline
        as *mut spPathConstraintPositionTimeline;
    let mut frames: *mut c_float = 0 as *mut c_float;
    let mut framesCount: c_int = 0;
    constraint = *((*skeleton).pathConstraints)
        .offset((*self_0).pathConstraintIndex as isize);
    if (*constraint).active == 0 {
        return;
    }
    if time < *((*self_0).frames).offset(0 as c_int as isize) {
        match blend as c_uint {
            0 => {
                (*constraint).position = (*(*constraint).data).position;
                return;
            }
            1 => {
                (*constraint).position
                    += ((*(*constraint).data).position - (*constraint).position) * alpha;
            }
            2 | 3 | _ => {}
        }
        return;
    }
    frames = (*self_0).frames;
    framesCount = (*self_0).framesCount;
    if time >= *frames.offset((framesCount - PATHCONSTRAINTPOSITION_ENTRIES) as isize) {
        position = *frames
            .offset((framesCount + PATHCONSTRAINTPOSITION_PREV_VALUE) as isize);
    } else {
        frame = binarySearch(frames, framesCount, time, PATHCONSTRAINTPOSITION_ENTRIES);
        position = *frames.offset((frame + PATHCONSTRAINTPOSITION_PREV_VALUE) as isize);
        frameTime = *frames.offset(frame as isize);
        percent = spCurveTimeline_getCurvePercent(
            &mut (*self_0).super_0,
            frame / PATHCONSTRAINTPOSITION_ENTRIES - 1 as c_int,
            1 as c_int as c_float
                - (time - frameTime)
                    / (*frames
                        .offset((frame + PATHCONSTRAINTPOSITION_PREV_TIME) as isize)
                        - frameTime),
        );
        position
            += (*frames.offset((frame + PATHCONSTRAINTPOSITION_VALUE) as isize)
                - position) * percent;
    }
    if blend as c_uint == SP_MIX_BLEND_SETUP as c_int as c_uint {
        (*constraint)
            .position = (*(*constraint).data).position
            + (position - (*(*constraint).data).position) * alpha;
    } else {
        (*constraint).position += (position - (*constraint).position) * alpha;
    };
}
#[no_mangle]
pub unsafe extern "C" fn _spPathConstraintPositionTimeline_getPropertyId(
    mut timeline: *const spTimeline,
) -> c_int {
    return ((SP_TIMELINE_PATHCONSTRAINTPOSITION as c_int) << 24 as c_int)
        + (*(timeline as *mut spPathConstraintPositionTimeline)).pathConstraintIndex;
}
#[no_mangle]
pub unsafe extern "C" fn spPathConstraintPositionTimeline_create(
    mut framesCount: c_int,
) -> *mut spPathConstraintPositionTimeline {
    return _spBaseTimeline_create(
        framesCount,
        SP_TIMELINE_PATHCONSTRAINTPOSITION,
        PATHCONSTRAINTPOSITION_ENTRIES,
        Some(
            _spPathConstraintPositionTimeline_apply
                as unsafe extern "C" fn(
                    *const spTimeline,
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
            _spPathConstraintPositionTimeline_getPropertyId
                as unsafe extern "C" fn(*const spTimeline) -> c_int,
        ),
    ) as *mut spPathConstraintPositionTimeline;
}
#[no_mangle]
pub unsafe extern "C" fn spPathConstraintPositionTimeline_setFrame(
    mut self_0: *mut spPathConstraintPositionTimeline,
    mut frameIndex: c_int,
    mut time: c_float,
    mut value: c_float,
) {
    frameIndex *= PATHCONSTRAINTPOSITION_ENTRIES;
    *((*self_0).frames).offset(frameIndex as isize) = time;
    *((*self_0).frames)
        .offset((frameIndex + PATHCONSTRAINTPOSITION_VALUE) as isize) = value;
}
static mut PATHCONSTRAINTSPACING_PREV_TIME: c_int = -(2 as c_int);
static mut PATHCONSTRAINTSPACING_PREV_VALUE: c_int = -(1 as c_int);
static mut PATHCONSTRAINTSPACING_VALUE: c_int = 1 as c_int;
#[no_mangle]
pub unsafe extern "C" fn _spPathConstraintSpacingTimeline_apply(
    mut timeline: *const spTimeline,
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
    let mut frameTime: c_float = 0.;
    let mut percent: c_float = 0.;
    let mut spacing: c_float = 0.;
    let mut constraint: *mut spPathConstraint = 0 as *mut spPathConstraint;
    let mut self_0: *mut spPathConstraintSpacingTimeline = timeline
        as *mut spPathConstraintSpacingTimeline;
    let mut frames: *mut c_float = 0 as *mut c_float;
    let mut framesCount: c_int = 0;
    constraint = *((*skeleton).pathConstraints)
        .offset((*self_0).pathConstraintIndex as isize);
    if (*constraint).active == 0 {
        return;
    }
    if time < *((*self_0).frames).offset(0 as c_int as isize) {
        match blend as c_uint {
            0 => {
                (*constraint).spacing = (*(*constraint).data).spacing;
                return;
            }
            1 => {
                (*constraint).spacing
                    += ((*(*constraint).data).spacing - (*constraint).spacing) * alpha;
            }
            2 | 3 | _ => {}
        }
        return;
    }
    frames = (*self_0).frames;
    framesCount = (*self_0).framesCount;
    if time >= *frames.offset((framesCount - PATHCONSTRAINTSPACING_ENTRIES) as isize) {
        spacing = *frames
            .offset((framesCount + PATHCONSTRAINTSPACING_PREV_VALUE) as isize);
    } else {
        frame = binarySearch(frames, framesCount, time, PATHCONSTRAINTSPACING_ENTRIES);
        spacing = *frames.offset((frame + PATHCONSTRAINTSPACING_PREV_VALUE) as isize);
        frameTime = *frames.offset(frame as isize);
        percent = spCurveTimeline_getCurvePercent(
            &mut (*self_0).super_0,
            frame / PATHCONSTRAINTSPACING_ENTRIES - 1 as c_int,
            1 as c_int as c_float
                - (time - frameTime)
                    / (*frames.offset((frame + PATHCONSTRAINTSPACING_PREV_TIME) as isize)
                        - frameTime),
        );
        spacing
            += (*frames.offset((frame + PATHCONSTRAINTSPACING_VALUE) as isize) - spacing)
                * percent;
    }
    if blend as c_uint == SP_MIX_BLEND_SETUP as c_int as c_uint {
        (*constraint)
            .spacing = (*(*constraint).data).spacing
            + (spacing - (*(*constraint).data).spacing) * alpha;
    } else {
        (*constraint).spacing += (spacing - (*constraint).spacing) * alpha;
    };
}
#[no_mangle]
pub unsafe extern "C" fn _spPathConstraintSpacingTimeline_getPropertyId(
    mut timeline: *const spTimeline,
) -> c_int {
    return ((SP_TIMELINE_PATHCONSTRAINTSPACING as c_int) << 24 as c_int)
        + (*(timeline as *mut spPathConstraintSpacingTimeline)).pathConstraintIndex;
}
#[no_mangle]
pub unsafe extern "C" fn spPathConstraintSpacingTimeline_create(
    mut framesCount: c_int,
) -> *mut spPathConstraintSpacingTimeline {
    return _spBaseTimeline_create(
        framesCount,
        SP_TIMELINE_PATHCONSTRAINTSPACING,
        PATHCONSTRAINTSPACING_ENTRIES,
        Some(
            _spPathConstraintSpacingTimeline_apply
                as unsafe extern "C" fn(
                    *const spTimeline,
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
            _spPathConstraintSpacingTimeline_getPropertyId
                as unsafe extern "C" fn(*const spTimeline) -> c_int,
        ),
    ) as *mut spPathConstraintSpacingTimeline;
}
#[no_mangle]
pub unsafe extern "C" fn spPathConstraintSpacingTimeline_setFrame(
    mut self_0: *mut spPathConstraintSpacingTimeline,
    mut frameIndex: c_int,
    mut time: c_float,
    mut value: c_float,
) {
    frameIndex *= PATHCONSTRAINTSPACING_ENTRIES;
    *((*self_0).frames).offset(frameIndex as isize) = time;
    *((*self_0).frames)
        .offset((frameIndex + PATHCONSTRAINTSPACING_VALUE) as isize) = value;
}
static mut PATHCONSTRAINTMIX_PREV_TIME: c_int = -(3 as c_int);
static mut PATHCONSTRAINTMIX_PREV_ROTATE: c_int = -(2 as c_int);
static mut PATHCONSTRAINTMIX_PREV_TRANSLATE: c_int = -(1 as c_int);
static mut PATHCONSTRAINTMIX_ROTATE: c_int = 1 as c_int;
static mut PATHCONSTRAINTMIX_TRANSLATE: c_int = 2 as c_int;
#[no_mangle]
pub unsafe extern "C" fn _spPathConstraintMixTimeline_apply(
    mut timeline: *const spTimeline,
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
    let mut frameTime: c_float = 0.;
    let mut percent: c_float = 0.;
    let mut rotate: c_float = 0.;
    let mut translate: c_float = 0.;
    let mut constraint: *mut spPathConstraint = 0 as *mut spPathConstraint;
    let mut self_0: *mut spPathConstraintMixTimeline = timeline
        as *mut spPathConstraintMixTimeline;
    let mut frames: *mut c_float = 0 as *mut c_float;
    let mut framesCount: c_int = 0;
    constraint = *((*skeleton).pathConstraints)
        .offset((*self_0).pathConstraintIndex as isize);
    if (*constraint).active == 0 {
        return;
    }
    if time < *((*self_0).frames).offset(0 as c_int as isize) {
        match blend as c_uint {
            0 => {
                (*constraint).rotateMix = (*(*constraint).data).rotateMix;
                (*constraint).translateMix = (*(*constraint).data).translateMix;
                return;
            }
            1 => {
                (*constraint).rotateMix
                    += ((*(*constraint).data).rotateMix - (*constraint).rotateMix)
                        * alpha;
                (*constraint).translateMix
                    += ((*(*constraint).data).translateMix - (*constraint).translateMix)
                        * alpha;
            }
            2 | 3 | _ => {}
        }
        return;
    }
    frames = (*self_0).frames;
    framesCount = (*self_0).framesCount;
    if time >= *frames.offset((framesCount - PATHCONSTRAINTMIX_ENTRIES) as isize) {
        rotate = *frames.offset((framesCount + PATHCONSTRAINTMIX_PREV_ROTATE) as isize);
        translate = *frames
            .offset((framesCount + PATHCONSTRAINTMIX_PREV_TRANSLATE) as isize);
    } else {
        frame = binarySearch(frames, framesCount, time, PATHCONSTRAINTMIX_ENTRIES);
        rotate = *frames.offset((frame + PATHCONSTRAINTMIX_PREV_ROTATE) as isize);
        translate = *frames.offset((frame + PATHCONSTRAINTMIX_PREV_TRANSLATE) as isize);
        frameTime = *frames.offset(frame as isize);
        percent = spCurveTimeline_getCurvePercent(
            &mut (*self_0).super_0,
            frame / PATHCONSTRAINTMIX_ENTRIES - 1 as c_int,
            1 as c_int as c_float
                - (time - frameTime)
                    / (*frames.offset((frame + PATHCONSTRAINTMIX_PREV_TIME) as isize)
                        - frameTime),
        );
        rotate
            += (*frames.offset((frame + PATHCONSTRAINTMIX_ROTATE) as isize) - rotate)
                * percent;
        translate
            += (*frames.offset((frame + PATHCONSTRAINTMIX_TRANSLATE) as isize)
                - translate) * percent;
    }
    if blend as c_uint == SP_MIX_BLEND_SETUP as c_int as c_uint {
        (*constraint)
            .rotateMix = (*(*constraint).data).rotateMix
            + (rotate - (*(*constraint).data).rotateMix) * alpha;
        (*constraint)
            .translateMix = (*(*constraint).data).translateMix
            + (translate - (*(*constraint).data).translateMix) * alpha;
    } else {
        (*constraint).rotateMix += (rotate - (*constraint).rotateMix) * alpha;
        (*constraint).translateMix += (translate - (*constraint).translateMix) * alpha;
    };
}
#[no_mangle]
pub unsafe extern "C" fn _spPathConstraintMixTimeline_getPropertyId(
    mut timeline: *const spTimeline,
) -> c_int {
    return ((SP_TIMELINE_PATHCONSTRAINTMIX as c_int) << 24 as c_int)
        + (*(timeline as *mut spPathConstraintMixTimeline)).pathConstraintIndex;
}
#[no_mangle]
pub unsafe extern "C" fn spPathConstraintMixTimeline_create(
    mut framesCount: c_int,
) -> *mut spPathConstraintMixTimeline {
    return _spBaseTimeline_create(
        framesCount,
        SP_TIMELINE_PATHCONSTRAINTMIX,
        PATHCONSTRAINTMIX_ENTRIES,
        Some(
            _spPathConstraintMixTimeline_apply
                as unsafe extern "C" fn(
                    *const spTimeline,
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
            _spPathConstraintMixTimeline_getPropertyId
                as unsafe extern "C" fn(*const spTimeline) -> c_int,
        ),
    ) as *mut spPathConstraintMixTimeline;
}
#[no_mangle]
pub unsafe extern "C" fn spPathConstraintMixTimeline_setFrame(
    mut self_0: *mut spPathConstraintMixTimeline,
    mut frameIndex: c_int,
    mut time: c_float,
    mut rotateMix: c_float,
    mut translateMix: c_float,
) {
    frameIndex *= PATHCONSTRAINTMIX_ENTRIES;
    *((*self_0).frames).offset(frameIndex as isize) = time;
    *((*self_0).frames)
        .offset((frameIndex + PATHCONSTRAINTMIX_ROTATE) as isize) = rotateMix;
    *((*self_0).frames)
        .offset((frameIndex + PATHCONSTRAINTMIX_TRANSLATE) as isize) = translateMix;
}
#[no_mangle]
pub unsafe extern "C" fn spTrackEntryArray_create(
    mut initialCapacity: c_int,
) -> *mut spTrackEntryArray {
    let mut array: *mut spTrackEntryArray = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spTrackEntryArray>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        1812 as c_int,
    ) as *mut spTrackEntryArray;
    (*array).size = 0 as c_int;
    (*array).capacity = initialCapacity;
    (*array)
        .items = _spCalloc(
        initialCapacity as size_t,
        ::core::mem::size_of::<*mut spTrackEntry>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        1812 as c_int,
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
        (*self_0)
            .capacity = if 8 as c_int
            > ((*self_0).size as c_float * 1.75f32) as c_int
        {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0)
            .items = _spRealloc(
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
    (*self_0)
        .items = _spRealloc(
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
        (*self_0)
            .capacity = if 8 as c_int
            > ((*self_0).size as c_float * 1.75f32) as c_int
        {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0)
            .items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<*mut spTrackEntry>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut *mut spTrackEntry;
    }
    let fresh24 = (*self_0).size;
    (*self_0).size = (*self_0).size + 1;
    let ref mut fresh25 = *((*self_0).items).offset(fresh24 as isize);
    *fresh25 = value;
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
        ((*self_0).items).offset(index as isize).offset(1 as c_int as isize)
            as *const c_void,
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
static mut SP_EMPTY_ANIMATION: *mut spAnimation = 0 as *const spAnimation
    as *mut spAnimation;
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
        1841 as c_int,
    ) as *mut _spEventQueue;
    (*self_0).state = state;
    (*self_0).objectsCount = 0 as c_int;
    (*self_0).objectsCapacity = 16 as c_int;
    (*self_0)
        .objects = _spCalloc(
        (*self_0).objectsCapacity as size_t,
        ::core::mem::size_of::<_spEventQueueItem>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        1845 as c_int,
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
            1859 as c_int,
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
    let fresh26 = (*self_0).objectsCount;
    (*self_0).objectsCount = (*self_0).objectsCount + 1;
    (*((*self_0).objects).offset(fresh26 as isize)).type_0 = type_0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _spEventQueue_addEntry(
    mut self_0: *mut _spEventQueue,
    mut entry: *mut spTrackEntry,
) {
    _spEventQueue_ensureCapacity(self_0, 1 as c_int);
    let fresh27 = (*self_0).objectsCount;
    (*self_0).objectsCount = (*self_0).objectsCount + 1;
    let ref mut fresh28 = (*((*self_0).objects).offset(fresh27 as isize)).entry;
    *fresh28 = entry;
}
#[no_mangle]
pub unsafe extern "C" fn _spEventQueue_addEvent(
    mut self_0: *mut _spEventQueue,
    mut event: *mut spEvent,
) {
    _spEventQueue_ensureCapacity(self_0, 1 as c_int);
    let fresh29 = (*self_0).objectsCount;
    (*self_0).objectsCount = (*self_0).objectsCount + 1;
    let ref mut fresh30 = (*((*self_0).objects).offset(fresh29 as isize)).event;
    *fresh30 = event;
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
        let mut type_0: spEventType = (*((*self_0).objects).offset(i as isize)).type_0
            as spEventType;
        let mut entry: *mut spTrackEntry = (*((*self_0).objects)
            .offset((i + 1 as c_int) as isize))
            .entry;
        let mut event: *mut spEvent = 0 as *mut spEvent;
        let mut current_block_22: u64;
        match type_0 as c_uint {
            0 | 1 | 3 => {
                if ((*entry).listener).is_some() {
                    ((*entry).listener)
                        .expect(
                            "non-null function pointer",
                        )(
                        &mut (*(*self_0).state).super_0,
                        type_0,
                        entry,
                        0 as *mut spEvent,
                    );
                }
                if ((*(*self_0).state).super_0.listener).is_some() {
                    ((*(*self_0).state).super_0.listener)
                        .expect(
                            "non-null function pointer",
                        )(
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
                    ((*entry).listener)
                        .expect(
                            "non-null function pointer",
                        )(
                        &mut (*(*self_0).state).super_0,
                        type_0,
                        entry,
                        0 as *mut spEvent,
                    );
                }
                if ((*(*self_0).state).super_0.listener).is_some() {
                    ((*(*self_0).state).super_0.listener)
                        .expect(
                            "non-null function pointer",
                        )(
                        &mut (*(*self_0).state).super_0,
                        type_0,
                        entry,
                        0 as *mut spEvent,
                    );
                }
                current_block_22 = 8159799021639475222;
            }
            4 => {
                current_block_22 = 8159799021639475222;
            }
            5 => {
                event = (*((*self_0).objects).offset((i + 2 as c_int) as isize))
                    .event;
                if ((*entry).listener).is_some() {
                    ((*entry).listener)
                        .expect(
                            "non-null function pointer",
                        )(&mut (*(*self_0).state).super_0, type_0, entry, event);
                }
                if ((*(*self_0).state).super_0.listener).is_some() {
                    ((*(*self_0).state).super_0.listener)
                        .expect(
                            "non-null function pointer",
                        )(&mut (*(*self_0).state).super_0, type_0, entry, event);
                }
                i += 1;
                current_block_22 = 10043043949733653460;
            }
            _ => {
                current_block_22 = 10043043949733653460;
            }
        }
        match current_block_22 {
            8159799021639475222 => {
                if ((*entry).listener).is_some() {
                    ((*entry).listener)
                        .expect(
                            "non-null function pointer",
                        )(
                        &mut (*(*self_0).state).super_0,
                        SP_ANIMATION_DISPOSE,
                        entry,
                        0 as *mut spEvent,
                    );
                }
                if ((*(*self_0).state).super_0.listener).is_some() {
                    ((*(*self_0).state).super_0.listener)
                        .expect(
                            "non-null function pointer",
                        )(
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
pub unsafe extern "C" fn _spAnimationState_enableQueue(
    mut self_0: *mut spAnimationState,
) {
    let mut internal: *mut _spAnimationState = self_0 as *mut _spAnimationState;
    (*(*internal).queue).drainDisabled = 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _spAnimationState_disableQueue(
    mut self_0: *mut spAnimationState,
) {
    let mut internal: *mut _spAnimationState = self_0 as *mut _spAnimationState;
    (*(*internal).queue).drainDisabled = 1 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _spAnimationState_disposeTrackEntry(
    mut entry: *mut spTrackEntry,
) {
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
                ((*entry).listener)
                    .expect(
                        "non-null function pointer",
                    )(state, SP_ANIMATION_DISPOSE, from, 0 as *mut spEvent);
            }
            if ((*state).listener).is_some() {
                ((*state).listener)
                    .expect(
                        "non-null function pointer",
                    )(state, SP_ANIMATION_DISPOSE, from, 0 as *mut spEvent);
            }
            _spAnimationState_disposeTrackEntry(from);
            from = nextFrom;
        }
        if ((*entry).listener).is_some() {
            ((*entry).listener)
                .expect(
                    "non-null function pointer",
                )(state, SP_ANIMATION_DISPOSE, entry, 0 as *mut spEvent);
        }
        if ((*state).listener).is_some() {
            ((*state).listener)
                .expect(
                    "non-null function pointer",
                )(state, SP_ANIMATION_DISPOSE, entry, 0 as *mut spEvent);
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
            0 as c_int,
        );
    }
    internal = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<_spAnimationState>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        2000 as c_int,
    ) as *mut _spAnimationState;
    self_0 = &mut (*internal).super_0;
    let ref mut fresh31 = *(&(*self_0).data as *const *mut spAnimationStateData
        as *mut *mut spAnimationStateData);
    *fresh31 = data;
    (*self_0).timeScale = 1 as c_int as c_float;
    (*internal).queue = _spEventQueue_create(internal);
    (*internal)
        .events = _spCalloc(
        128 as c_int as size_t,
        ::core::mem::size_of::<*mut spEvent>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        2007 as c_int,
    ) as *mut *mut spEvent;
    (*internal)
        .propertyIDs = _spCalloc(
        128 as c_int as size_t,
        ::core::mem::size_of::<c_int>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        2009 as c_int,
    ) as *mut c_int;
    (*internal).propertyIDsCapacity = 128 as c_int;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spAnimationState_dispose(mut self_0: *mut spAnimationState) {
    let mut i: c_int = 0;
    let mut internal: *mut _spAnimationState = self_0 as *mut _spAnimationState;
    i = 0 as c_int;
    while i < (*self_0).tracksCount {
        _spAnimationState_disposeTrackEntries(
            self_0,
            *((*self_0).tracks).offset(i as isize),
        );
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
                        let mut nextTime: c_float = (*current).trackLast
                            - (*next).delay;
                        if nextTime >= 0 as c_int as c_float {
                            (*next).delay = 0 as c_int as c_float;
                            (*next).trackTime
                                += if (*current).timeScale
                                    == 0 as c_int as c_float
                                {
                                    0 as c_int as c_float
                                } else {
                                    (nextTime / (*current).timeScale + delta)
                                        * (*next).timeScale
                                };
                            (*current).trackTime += currentDelta;
                            _spAnimationState_setCurrent(
                                self_0,
                                i,
                                next,
                                1 as c_int,
                            );
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
                        let ref mut fresh32 = *((*self_0).tracks).offset(i as isize);
                        *fresh32 = 0 as *mut spTrackEntry;
                        _spEventQueue_end((*internal).queue, current);
                        _spAnimationState_disposeNext(self_0, current);
                        current_block_29 = 16559507199688588974;
                    } else {
                        current_block_29 = 17478428563724192186;
                    }
                    match current_block_29 {
                        16559507199688588974 => {}
                        _ => {
                            if !((*current).mixingFrom).is_null()
                                && _spAnimationState_updateMixingFrom(
                                    self_0,
                                    current,
                                    delta,
                                ) != 0
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
    if (*to).mixTime > 0 as c_int as c_float
        && (*to).mixTime >= (*to).mixDuration
    {
        if (*from).totalAlpha == 0 as c_int as c_float
            || (*to).mixDuration == 0 as c_int as c_float
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
    let mut timelinesRotation: *mut c_float = 0 as *mut c_float;
    let mut timeline: *mut spTimeline = 0 as *mut spTimeline;
    let mut applied: c_int = 0 as c_int;
    let mut blend: spMixBlend = SP_MIX_BLEND_SETUP;
    let mut timelineBlend: spMixBlend = SP_MIX_BLEND_SETUP;
    let mut setupState: c_int = 0 as c_int;
    let mut slots: *mut *mut spSlot = 0 as *mut *mut spSlot;
    let mut slot: *mut spSlot = 0 as *mut spSlot;
    let mut attachmentName: *const c_char = 0 as *const c_char;
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
                mix
                    *= _spAnimationState_applyMixingFrom(
                        self_0,
                        current,
                        skeleton,
                        blend,
                    );
            } else if (*current).trackTime >= (*current).trackEnd
                    && ((*current).next).is_null()
                {
                mix = 0 as c_int as c_float;
            }
            animationLast = (*current).animationLast;
            animationTime = spTrackEntry_getAnimationTime(current);
            timelineCount = (*(*current).animation).timelinesCount;
            timelines = (*(*current).animation).timelines;
            if i == 0 as c_int && mix == 1 as c_int as c_float
                || blend as c_uint
                    == SP_MIX_BLEND_ADD as c_int as c_uint
            {
                ii = 0 as c_int;
                while ii < timelineCount {
                    timeline = *timelines.offset(ii as isize);
                    if (*timeline).type_0 as c_uint
                        == SP_TIMELINE_ATTACHMENT as c_int as c_uint
                    {
                        _spAnimationState_applyAttachmentTimeline(
                            self_0,
                            timeline,
                            skeleton,
                            animationTime,
                            blend,
                            -(1 as c_int),
                        );
                    } else {
                        spTimeline_apply(
                            *timelines.offset(ii as isize),
                            skeleton,
                            animationLast,
                            animationTime,
                            (*internal).events,
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
                firstFrame = ((*current).timelinesRotationCount == 0 as c_int)
                    as c_int;
                if firstFrame != 0 {
                    _spAnimationState_resizeTimelinesRotation(
                        current,
                        timelineCount << 1 as c_int,
                    );
                }
                timelinesRotation = (*current).timelinesRotation;
                ii = 0 as c_int;
                while ii < timelineCount {
                    timeline = *timelines.offset(ii as isize);
                    timelineBlend = (if *((*timelineMode).items).offset(ii as isize)
                        == 0 as c_int
                    {
                        blend as c_uint
                    } else {
                        SP_MIX_BLEND_SETUP as c_int as c_uint
                    }) as spMixBlend;
                    if (*timeline).type_0 as c_uint
                        == SP_TIMELINE_ROTATE as c_int as c_uint
                    {
                        _spAnimationState_applyRotateTimeline(
                            self_0,
                            timeline,
                            skeleton,
                            animationTime,
                            mix,
                            timelineBlend,
                            timelinesRotation,
                            ii << 1 as c_int,
                            firstFrame,
                        );
                    } else if (*timeline).type_0 as c_uint
                            == SP_TIMELINE_ATTACHMENT as c_int as c_uint
                        {
                        _spAnimationState_applyAttachmentTimeline(
                            self_0,
                            timeline,
                            skeleton,
                            animationTime,
                            timelineBlend,
                            -(1 as c_int),
                        );
                    } else {
                        spTimeline_apply(
                            timeline,
                            skeleton,
                            animationLast,
                            animationTime,
                            (*internal).events,
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
            (*slot)
                .attachment = if attachmentName.is_null() {
                0 as *mut spAttachment
            } else {
                spSkeleton_getAttachmentForSlotIndex(
                    skeleton,
                    (*(*slot).data).index,
                    attachmentName,
                )
            };
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
    let mut timelinesRotation: *mut c_float = 0 as *mut c_float;
    let mut i: c_int = 0;
    let mut holdMix: *mut spTrackEntry = 0 as *mut spTrackEntry;
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
    events = if mix < (*from).eventThreshold {
        (*internal).events
    } else {
        0 as *mut *mut spEvent
    };
    attachments = (mix < (*from).attachmentThreshold) as c_int;
    drawOrder = (mix < (*from).drawOrderThreshold) as c_int;
    animationLast = (*from).animationLast;
    animationTime = spTrackEntry_getAnimationTime(from);
    timelineCount = (*(*from).animation).timelinesCount;
    timelines = (*(*from).animation).timelines;
    alphaHold = (*from).alpha * (*to).interruptAlpha;
    alphaMix = alphaHold * (1 as c_int as c_float - mix);
    if blend as c_uint == SP_MIX_BLEND_ADD as c_int as c_uint {
        i = 0 as c_int;
        while i < timelineCount {
            let mut timeline: *mut spTimeline = *timelines.offset(i as isize);
            spTimeline_apply(
                timeline,
                skeleton,
                animationLast,
                animationTime,
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
        firstFrame = ((*from).timelinesRotationCount == 0 as c_int) as c_int;
        if firstFrame != 0 {
            _spAnimationState_resizeTimelinesRotation(
                from,
                timelineCount << 1 as c_int,
            );
        }
        timelinesRotation = (*from).timelinesRotation;
        (*from).totalAlpha = 0 as c_int as c_float;
        let mut current_block_54: u64;
        i = 0 as c_int;
        while i < timelineCount {
            let mut direction: spMixDirection = SP_MIX_DIRECTION_OUT;
            let mut timeline_0: *mut spTimeline = *timelines.offset(i as isize);
            match *((*timelineMode).items).offset(i as isize) {
                0 => {
                    if drawOrder == 0
                        && (*timeline_0).type_0 as c_uint
                            == SP_TIMELINE_DRAWORDER as c_int as c_uint
                    {
                        current_block_54 = 11385396242402735691;
                    } else {
                        timelineBlend = blend;
                        alpha = alphaMix;
                        current_block_54 = 16738040538446813684;
                    }
                }
                1 => {
                    timelineBlend = SP_MIX_BLEND_SETUP;
                    alpha = alphaMix;
                    current_block_54 = 16738040538446813684;
                }
                2 => {
                    timelineBlend = blend;
                    alpha = alphaHold;
                    current_block_54 = 16738040538446813684;
                }
                3 => {
                    timelineBlend = SP_MIX_BLEND_SETUP;
                    alpha = alphaHold;
                    current_block_54 = 16738040538446813684;
                }
                _ => {
                    timelineBlend = SP_MIX_BLEND_SETUP;
                    holdMix = *((*timelineHoldMix).items).offset(i as isize);
                    alpha = alphaHold
                        * (if 0 as c_int as c_float
                            > 1 as c_int as c_float
                                - (*holdMix).mixTime / (*holdMix).mixDuration
                        {
                            0 as c_int as c_float
                        } else {
                            1 as c_int as c_float
                                - (*holdMix).mixTime / (*holdMix).mixDuration
                        });
                    current_block_54 = 16738040538446813684;
                }
            }
            match current_block_54 {
                16738040538446813684 => {
                    (*from).totalAlpha += alpha;
                    if (*timeline_0).type_0 as c_uint
                        == SP_TIMELINE_ROTATE as c_int as c_uint
                    {
                        _spAnimationState_applyRotateTimeline(
                            self_0,
                            timeline_0,
                            skeleton,
                            animationTime,
                            alpha,
                            timelineBlend,
                            timelinesRotation,
                            i << 1 as c_int,
                            firstFrame,
                        );
                    } else if (*timeline_0).type_0 as c_uint
                            == SP_TIMELINE_ATTACHMENT as c_int as c_uint
                        {
                        _spAnimationState_applyAttachmentTimeline(
                            self_0,
                            timeline_0,
                            skeleton,
                            animationTime,
                            timelineBlend,
                            attachments,
                        );
                    } else {
                        if drawOrder != 0
                            && (*timeline_0).type_0 as c_uint
                                == SP_TIMELINE_DRAWORDER as c_int as c_uint
                            && timelineBlend as c_uint
                                == SP_MIX_BLEND_SETUP as c_int as c_uint
                        {
                            direction = SP_MIX_DIRECTION_IN;
                        }
                        spTimeline_apply(
                            timeline_0,
                            skeleton,
                            animationLast,
                            animationTime,
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
    (*slot)
        .attachment = if attachmentName.is_null() {
        0 as *mut spAttachment
    } else {
        spSkeleton_getAttachmentForSlotIndex(
            skeleton,
            (*(*slot).data).index,
            attachmentName,
        )
    };
    if attachments != 0 {
        (*slot).attachmentState = (*self_0).unkeyedState + 2 as c_int;
    }
}
unsafe extern "C" fn binarySearch1_state(
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
    };
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
    let mut attachmentTimeline: *mut spAttachmentTimeline = 0
        as *mut spAttachmentTimeline;
    let mut slot: *mut spSlot = 0 as *mut spSlot;
    let mut frameIndex: c_int = 0;
    let mut frames: *mut c_float = 0 as *mut c_float;
    attachmentTimeline = timeline as *mut spAttachmentTimeline;
    slot = *((*skeleton).slots).offset((*attachmentTimeline).slotIndex as isize);
    if (*(*slot).bone).active == 0 {
        return;
    }
    frames = (*attachmentTimeline).frames;
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
        if time
            >= *frames
                .offset(((*attachmentTimeline).framesCount - 1 as c_int) as isize)
        {
            frameIndex = (*attachmentTimeline).framesCount - 1 as c_int;
        } else {
            frameIndex = binarySearch1_state(
                frames,
                (*attachmentTimeline).framesCount,
                time,
            ) - 1 as c_int;
        }
        _spAnimationState_setAttachment(
            self_0,
            skeleton,
            slot,
            *((*attachmentTimeline).attachmentNames).offset(frameIndex as isize),
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
    let mut frame: c_int = 0;
    let mut prevRotation: c_float = 0.;
    let mut frameTime: c_float = 0.;
    let mut percent: c_float = 0.;
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
    frames = (*rotateTimeline).frames;
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
                    return
                }
            }
        }
    } else {
        r1 = if blend as c_uint
            == SP_MIX_BLEND_SETUP as c_int as c_uint
        {
            (*(*bone).data).rotation
        } else {
            (*bone).rotation
        };
        if time
            >= *frames.offset(((*rotateTimeline).framesCount - ROTATE_ENTRIES) as isize)
        {
            r2 = (*(*bone).data).rotation
                + *frames
                    .offset(
                        ((*rotateTimeline).framesCount + ROTATE_PREV_ROTATION) as isize,
                    );
        } else {
            frame = _spCurveTimeline_binarySearch(
                frames,
                (*rotateTimeline).framesCount,
                time,
                ROTATE_ENTRIES,
            );
            prevRotation = *frames.offset((frame + ROTATE_PREV_ROTATION) as isize);
            frameTime = *frames.offset(frame as isize);
            percent = spCurveTimeline_getCurvePercent(
                &mut (*rotateTimeline).super_0,
                (frame >> 1 as c_int) - 1 as c_int,
                1 as c_int as c_float
                    - (time - frameTime)
                        / (*frames.offset((frame + ROTATE_PREV_TIME) as isize)
                            - frameTime),
            );
            r2 = *frames.offset((frame + ROTATE_ROTATION) as isize) - prevRotation;
            r2
                -= ((16384 as c_int
                    - (16384.499999999996f64
                        - (r2 / 360 as c_int as c_float) as c_double)
                        as c_int) * 360 as c_int) as c_float;
            r2 = prevRotation + r2 * percent + (*(*bone).data).rotation;
            r2
                -= ((16384 as c_int
                    - (16384.499999999996f64
                        - (r2 / 360 as c_int as c_float) as c_double)
                        as c_int) * 360 as c_int) as c_float;
        }
    }
    diff = r2 - r1;
    diff
        -= ((16384 as c_int
            - (16384.499999999996f64
                - (diff / 360 as c_int as c_float) as c_double)
                as c_int) * 360 as c_int) as c_float;
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
            if lastDiff > 0 as c_int as c_float { 1.0f32 } else { 0.0f32 }
        })
            != (if diff < 0 as c_int as c_float {
                -1.0f32
            } else {
                if diff > 0 as c_int as c_float { 1.0f32 } else { 0.0f32 }
            })
            && (if lastDiff < 0 as c_int as c_float {
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
                lastTotal
                    += 360 as c_int as c_float
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
            total
                += 360 as c_int as c_float
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
    r1 += total * alpha;
    (*bone)
        .rotation = r1
        - ((16384 as c_int
            - (16384.499999999996f64
                - (r1 / 360 as c_int as c_float) as c_double)
                as c_int) * 360 as c_int) as c_float;
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
        complete = (animationTime >= animationEnd
            && (*entry).animationLast < animationEnd) as c_int;
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
pub unsafe extern "C" fn spAnimationState_clearTracks(
    mut self_0: *mut spAnimationState,
) {
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
    _spAnimationState_disposeNext(self_0, current);
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
    let ref mut fresh33 = *((*self_0).tracks).offset((*current).trackIndex as isize);
    *fresh33 = 0 as *mut spTrackEntry;
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
    let ref mut fresh34 = *((*self_0).tracks).offset(index as isize);
    *fresh34 = current;
    if !from.is_null() {
        if interrupt != 0 {
            _spEventQueue_interrupt((*internal).queue, from);
        }
        (*current).mixingFrom = from;
        (*from).mixingTo = current;
        (*current).mixTime = 0 as c_int as c_float;
        if !((*from).mixingFrom).is_null()
            && (*from).mixDuration > 0 as c_int as c_float
        {
            (*current).interruptAlpha
                *= if (1 as c_int as c_float)
                    < (*from).mixTime / (*from).mixDuration
                {
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
    let mut animation: *mut spAnimation = spSkeletonData_findAnimation(
        (*(*self_0).data).skeletonData,
        animationName,
    );
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
    let mut current: *mut spTrackEntry = _spAnimationState_expandToIndex(
        self_0,
        trackIndex,
    );
    if !current.is_null() {
        if (*current).nextTrackLast == -(1 as c_int) as c_float {
            let ref mut fresh35 = *((*self_0).tracks).offset(trackIndex as isize);
            *fresh35 = (*current).mixingFrom;
            _spEventQueue_interrupt((*internal).queue, current);
            _spEventQueue_end((*internal).queue, current);
            _spAnimationState_disposeNext(self_0, current);
            current = (*current).mixingFrom;
            interrupt = 0 as c_int;
        } else {
            _spAnimationState_disposeNext(self_0, current);
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
    let mut animation: *mut spAnimation = spSkeletonData_findAnimation(
        (*(*self_0).data).skeletonData,
        animationName,
    );
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
    let mut last: *mut spTrackEntry = _spAnimationState_expandToIndex(
        self_0,
        trackIndex,
    );
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
        if delay <= 0 as c_int as c_float {
            let mut duration: c_float = (*last).animationEnd
                - (*last).animationStart;
            if duration != 0 as c_int as c_float {
                if (*last).loop_0 != 0 {
                    delay
                        += duration
                            * (1 as c_int
                                + ((*last).trackTime / duration) as c_int)
                                as c_float;
                } else {
                    delay
                        += if duration > (*last).trackTime {
                            duration
                        } else {
                            (*last).trackTime
                        };
                }
                delay
                    -= spAnimationStateData_getMix(
                        (*self_0).data,
                        (*last).animation,
                        animation,
                    );
            } else {
                delay = (*last).trackTime;
            }
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
    let mut entry: *mut spTrackEntry = spAnimationState_setAnimation(
        self_0,
        trackIndex,
        SP_EMPTY_ANIMATION,
        0 as c_int,
    );
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
    let mut entry: *mut spTrackEntry = 0 as *mut spTrackEntry;
    if delay <= 0 as c_int as c_float {
        delay -= mixDuration;
    }
    entry = spAnimationState_addAnimation(
        self_0,
        trackIndex,
        SP_EMPTY_ANIMATION,
        0 as c_int,
        delay,
    );
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
            spAnimationState_setEmptyAnimation(
                self_0,
                (*current).trackIndex,
                mixDuration,
            );
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
        2646 as c_int,
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
        2655 as c_int,
    ) as *mut spTrackEntry;
    (*entry).trackIndex = trackIndex;
    (*entry).animation = animation;
    (*entry).loop_0 = loop_0;
    (*entry).holdPrevious = 0 as c_int;
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
    (*entry).interruptAlpha = 1 as c_int as c_float;
    (*entry).mixTime = 0 as c_int as c_float;
    (*entry)
        .mixDuration = if last.is_null() {
        0 as c_int as c_float
    } else {
        spAnimationStateData_getMix((*self_0).data, (*last).animation, animation)
    };
    (*entry).mixBlend = SP_MIX_BLEND_REPLACE;
    (*entry).timelineMode = spIntArray_create(16 as c_int);
    (*entry).timelineHoldMix = spTrackEntryArray_create(16 as c_int);
    return entry;
}
#[no_mangle]
pub unsafe extern "C" fn _spAnimationState_disposeNext(
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
pub unsafe extern "C" fn _spAnimationState_animationsChanged(
    mut self_0: *mut spAnimationState,
) {
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
                    || (*entry).mixBlend as c_uint
                        != SP_MIX_BLEND_ADD as c_int as c_uint
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
            2722 as c_int,
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
        let mut newPropertyIDs: *mut c_int = _spCalloc(
            (capacity << 1 as c_int) as size_t,
            ::core::mem::size_of::<c_int>() as c_ulong,
            b"spine.c\0" as *const u8 as *const c_char,
            2733 as c_int,
        ) as *mut c_int;
        spine_memcpy(
            newPropertyIDs as *mut c_void,
            (*internal).propertyIDs as *const c_void,
            (::core::mem::size_of::<c_int>() as c_ulong)
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
    mut id: c_int,
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
    _spAnimationState_ensureCapacityPropertyIDs(
        self_0,
        (*internal).propertyIDsCount + 1 as c_int,
    );
    *((*internal).propertyIDs).offset((*internal).propertyIDsCount as isize) = id;
    (*internal).propertyIDsCount += 1;
    return 1 as c_int;
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
pub unsafe extern "C" fn spTrackEntry_getAnimationTime(
    mut entry: *mut spTrackEntry,
) -> c_float {
    if (*entry).loop_0 != 0 {
        let mut duration: c_float = (*entry).animationEnd
            - (*entry).animationStart;
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
pub unsafe extern "C" fn _spTrackEntry_hasTimeline(
    mut self_0: *mut spTrackEntry,
    mut id: c_int,
) -> c_int {
    let mut timelines: *mut *mut spTimeline = (*(*self_0).animation).timelines;
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    i = 0 as c_int;
    n = (*(*self_0).animation).timelinesCount;
    while i < n {
        if spTimeline_getPropertyId(*timelines.offset(i as isize)) == id {
            return 1 as c_int;
        }
        i += 1;
    }
    return 0 as c_int;
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
    timelines = (*(*entry).animation).timelines;
    timelinesCount = (*(*entry).animation).timelinesCount;
    timelineMode = (*spIntArray_setSize((*entry).timelineMode, timelinesCount)).items;
    spTrackEntryArray_clear((*entry).timelineHoldMix);
    timelineHoldMix = (*spTrackEntryArray_setSize(
        (*entry).timelineHoldMix,
        timelinesCount,
    ))
        .items;
    if !to.is_null() && (*to).holdPrevious != 0 {
        i = 0 as c_int;
        while i < timelinesCount {
            let mut id: c_int = spTimeline_getPropertyId(
                *timelines.offset(i as isize),
            );
            *timelineMode
                .offset(
                    i as isize,
                ) = if _spAnimationState_addPropertyID(state, id) != 0 {
                3 as c_int
            } else {
                2 as c_int
            };
            i += 1;
        }
        return;
    }
    i = 0 as c_int;
    's_67: while i < timelinesCount {
        let mut timeline: *mut spTimeline = *timelines.offset(i as isize);
        let mut id_0: c_int = spTimeline_getPropertyId(timeline);
        if _spAnimationState_addPropertyID(state, id_0) == 0 {
            *timelineMode.offset(i as isize) = 0 as c_int;
        } else if to.is_null()
                || (*timeline).type_0 as c_uint
                    == SP_TIMELINE_ATTACHMENT as c_int as c_uint
                || (*timeline).type_0 as c_uint
                    == SP_TIMELINE_DRAWORDER as c_int as c_uint
                || (*timeline).type_0 as c_uint
                    == SP_TIMELINE_EVENT as c_int as c_uint
                || _spTrackEntry_hasTimeline(to, id_0) == 0
            {
            *timelineMode.offset(i as isize) = 1 as c_int;
        } else {
            next = (*to).mixingTo;
            while !next.is_null() {
                if _spTrackEntry_hasTimeline(next, id_0) != 0 {
                    next = (*next).mixingTo;
                } else {
                    if !((*next).mixDuration > 0 as c_int as c_float) {
                        break;
                    }
                    *timelineMode.offset(i as isize) = 4 as c_int;
                    let ref mut fresh36 = *timelineHoldMix.offset(i as isize);
                    *fresh36 = next;
                    i += 1;
                    continue 's_67;
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
        2871 as c_int,
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
pub unsafe extern "C" fn _FromEntry_create(
    mut from: *mut spAnimation,
) -> *mut _FromEntry {
    let mut self_0: *mut _FromEntry = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<_FromEntry>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        2891 as c_int,
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
        2903 as c_int,
    ) as *mut spAnimationStateData;
    let ref mut fresh37 = *(&(*self_0).skeletonData as *const *mut spSkeletonData
        as *mut *mut spSkeletonData);
    *fresh37 = skeletonData;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spAnimationStateData_dispose(
    mut self_0: *mut spAnimationStateData,
) {
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
    let mut from: *mut spAnimation = spSkeletonData_findAnimation(
        (*self_0).skeletonData,
        fromName,
    );
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
        let ref mut fresh38 = *(&(*self_0).entries as *const *const c_void
            as *mut *mut _FromEntry);
        *fresh38 = fromEntry;
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
pub unsafe extern "C" fn spFloatArray_create(
    mut initialCapacity: c_int,
) -> *mut spFloatArray {
    let mut array: *mut spFloatArray = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spFloatArray>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        3013 as c_int,
    ) as *mut spFloatArray;
    (*array).size = 0 as c_int;
    (*array).capacity = initialCapacity;
    (*array)
        .items = _spCalloc(
        initialCapacity as size_t,
        ::core::mem::size_of::<c_float>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        3013 as c_int,
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
        (*self_0)
            .capacity = if 8 as c_int
            > ((*self_0).size as c_float * 1.75f32) as c_int
        {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0)
            .items = _spRealloc(
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
    (*self_0)
        .items = _spRealloc(
        (*self_0).items as *mut c_void,
        (::core::mem::size_of::<c_float>() as c_ulong)
            .wrapping_mul((*self_0).capacity as c_ulong),
    ) as *mut c_float;
}
#[no_mangle]
pub unsafe extern "C" fn spFloatArray_add(
    mut self_0: *mut spFloatArray,
    mut value: c_float,
) {
    if (*self_0).size == (*self_0).capacity {
        (*self_0)
            .capacity = if 8 as c_int
            > ((*self_0).size as c_float * 1.75f32) as c_int
        {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0)
            .items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<c_float>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut c_float;
    }
    let fresh39 = (*self_0).size;
    (*self_0).size = (*self_0).size + 1;
    *((*self_0).items).offset(fresh39 as isize) = value;
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
pub unsafe extern "C" fn spFloatArray_removeAt(
    mut self_0: *mut spFloatArray,
    mut index: c_int,
) {
    (*self_0).size -= 1;
    spine_memmove(
        ((*self_0).items).offset(index as isize) as *mut c_void,
        ((*self_0).items).offset(index as isize).offset(1 as c_int as isize)
            as *const c_void,
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
pub unsafe extern "C" fn spFloatArray_pop(
    mut self_0: *mut spFloatArray,
) -> c_float {
    (*self_0).size -= 1;
    let mut item: c_float = *((*self_0).items).offset((*self_0).size as isize);
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn spFloatArray_peek(
    mut self_0: *mut spFloatArray,
) -> c_float {
    return *((*self_0).items).offset(((*self_0).size - 1 as c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn spIntArray_create(
    mut initialCapacity: c_int,
) -> *mut spIntArray {
    let mut array: *mut spIntArray = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spIntArray>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        3014 as c_int,
    ) as *mut spIntArray;
    (*array).size = 0 as c_int;
    (*array).capacity = initialCapacity;
    (*array)
        .items = _spCalloc(
        initialCapacity as size_t,
        ::core::mem::size_of::<c_int>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        3014 as c_int,
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
        (*self_0)
            .capacity = if 8 as c_int
            > ((*self_0).size as c_float * 1.75f32) as c_int
        {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0)
            .items = _spRealloc(
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
    (*self_0)
        .items = _spRealloc(
        (*self_0).items as *mut c_void,
        (::core::mem::size_of::<c_int>() as c_ulong)
            .wrapping_mul((*self_0).capacity as c_ulong),
    ) as *mut c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spIntArray_add(
    mut self_0: *mut spIntArray,
    mut value: c_int,
) {
    if (*self_0).size == (*self_0).capacity {
        (*self_0)
            .capacity = if 8 as c_int
            > ((*self_0).size as c_float * 1.75f32) as c_int
        {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0)
            .items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<c_int>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut c_int;
    }
    let fresh40 = (*self_0).size;
    (*self_0).size = (*self_0).size + 1;
    *((*self_0).items).offset(fresh40 as isize) = value;
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
pub unsafe extern "C" fn spIntArray_removeAt(
    mut self_0: *mut spIntArray,
    mut index: c_int,
) {
    (*self_0).size -= 1;
    spine_memmove(
        ((*self_0).items).offset(index as isize) as *mut c_void,
        ((*self_0).items).offset(index as isize).offset(1 as c_int as isize)
            as *const c_void,
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
pub unsafe extern "C" fn spShortArray_create(
    mut initialCapacity: c_int,
) -> *mut spShortArray {
    let mut array: *mut spShortArray = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spShortArray>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        3015 as c_int,
    ) as *mut spShortArray;
    (*array).size = 0 as c_int;
    (*array).capacity = initialCapacity;
    (*array)
        .items = _spCalloc(
        initialCapacity as size_t,
        ::core::mem::size_of::<c_short>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        3015 as c_int,
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
        (*self_0)
            .capacity = if 8 as c_int
            > ((*self_0).size as c_float * 1.75f32) as c_int
        {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0)
            .items = _spRealloc(
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
    (*self_0)
        .items = _spRealloc(
        (*self_0).items as *mut c_void,
        (::core::mem::size_of::<c_short>() as c_ulong)
            .wrapping_mul((*self_0).capacity as c_ulong),
    ) as *mut c_short;
}
#[no_mangle]
pub unsafe extern "C" fn spShortArray_add(
    mut self_0: *mut spShortArray,
    mut value: c_short,
) {
    if (*self_0).size == (*self_0).capacity {
        (*self_0)
            .capacity = if 8 as c_int
            > ((*self_0).size as c_float * 1.75f32) as c_int
        {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0)
            .items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<c_short>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut c_short;
    }
    let fresh41 = (*self_0).size;
    (*self_0).size = (*self_0).size + 1;
    *((*self_0).items).offset(fresh41 as isize) = value;
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
pub unsafe extern "C" fn spShortArray_removeAt(
    mut self_0: *mut spShortArray,
    mut index: c_int,
) {
    (*self_0).size -= 1;
    spine_memmove(
        ((*self_0).items).offset(index as isize) as *mut c_void,
        ((*self_0).items).offset(index as isize).offset(1 as c_int as isize)
            as *const c_void,
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
pub unsafe extern "C" fn spShortArray_pop(
    mut self_0: *mut spShortArray,
) -> c_short {
    (*self_0).size -= 1;
    let mut item: c_short = *((*self_0).items).offset((*self_0).size as isize);
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn spShortArray_peek(
    mut self_0: *mut spShortArray,
) -> c_short {
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
        3016 as c_int,
    ) as *mut spUnsignedShortArray;
    (*array).size = 0 as c_int;
    (*array).capacity = initialCapacity;
    (*array)
        .items = _spCalloc(
        initialCapacity as size_t,
        ::core::mem::size_of::<c_ushort>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        3016 as c_int,
    ) as *mut c_ushort;
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn spUnsignedShortArray_dispose(
    mut self_0: *mut spUnsignedShortArray,
) {
    _spFree((*self_0).items as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spUnsignedShortArray_clear(
    mut self_0: *mut spUnsignedShortArray,
) {
    (*self_0).size = 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spUnsignedShortArray_setSize(
    mut self_0: *mut spUnsignedShortArray,
    mut newSize: c_int,
) -> *mut spUnsignedShortArray {
    (*self_0).size = newSize;
    if (*self_0).capacity < newSize {
        (*self_0)
            .capacity = if 8 as c_int
            > ((*self_0).size as c_float * 1.75f32) as c_int
        {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0)
            .items = _spRealloc(
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
    (*self_0)
        .items = _spRealloc(
        (*self_0).items as *mut c_void,
        (::core::mem::size_of::<c_ushort>() as c_ulong)
            .wrapping_mul((*self_0).capacity as c_ulong),
    ) as *mut c_ushort;
}
#[no_mangle]
pub unsafe extern "C" fn spUnsignedShortArray_add(
    mut self_0: *mut spUnsignedShortArray,
    mut value: c_ushort,
) {
    if (*self_0).size == (*self_0).capacity {
        (*self_0)
            .capacity = if 8 as c_int
            > ((*self_0).size as c_float * 1.75f32) as c_int
        {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0)
            .items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<c_ushort>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut c_ushort;
    }
    let fresh42 = (*self_0).size;
    (*self_0).size = (*self_0).size + 1;
    *((*self_0).items).offset(fresh42 as isize) = value;
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
        ((*self_0).items).offset(index as isize).offset(1 as c_int as isize)
            as *const c_void,
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
        3017 as c_int,
    ) as *mut spArrayFloatArray;
    (*array).size = 0 as c_int;
    (*array).capacity = initialCapacity;
    (*array)
        .items = _spCalloc(
        initialCapacity as size_t,
        ::core::mem::size_of::<*mut spFloatArray>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        3017 as c_int,
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
        (*self_0)
            .capacity = if 8 as c_int
            > ((*self_0).size as c_float * 1.75f32) as c_int
        {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0)
            .items = _spRealloc(
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
    (*self_0)
        .items = _spRealloc(
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
        (*self_0)
            .capacity = if 8 as c_int
            > ((*self_0).size as c_float * 1.75f32) as c_int
        {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0)
            .items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<*mut spFloatArray>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut *mut spFloatArray;
    }
    let fresh43 = (*self_0).size;
    (*self_0).size = (*self_0).size + 1;
    let ref mut fresh44 = *((*self_0).items).offset(fresh43 as isize);
    *fresh44 = value;
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
        ((*self_0).items).offset(index as isize).offset(1 as c_int as isize)
            as *const c_void,
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
        3018 as c_int,
    ) as *mut spArrayShortArray;
    (*array).size = 0 as c_int;
    (*array).capacity = initialCapacity;
    (*array)
        .items = _spCalloc(
        initialCapacity as size_t,
        ::core::mem::size_of::<*mut spShortArray>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        3018 as c_int,
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
        (*self_0)
            .capacity = if 8 as c_int
            > ((*self_0).size as c_float * 1.75f32) as c_int
        {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0)
            .items = _spRealloc(
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
    (*self_0)
        .items = _spRealloc(
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
        (*self_0)
            .capacity = if 8 as c_int
            > ((*self_0).size as c_float * 1.75f32) as c_int
        {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0)
            .items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<*mut spShortArray>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut *mut spShortArray;
    }
    let fresh45 = (*self_0).size;
    (*self_0).size = (*self_0).size + 1;
    let ref mut fresh46 = *((*self_0).items).offset(fresh45 as isize);
    *fresh46 = value;
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
        ((*self_0).items).offset(index as isize).offset(1 as c_int as isize)
            as *const c_void,
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
pub unsafe extern "C" fn spAtlasPage_create(
    mut atlas: *mut spAtlas,
    mut name: *const c_char,
) -> *mut spAtlasPage {
    let mut self_0: *mut spAtlasPage = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spAtlasPage>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        3053 as c_int,
    ) as *mut spAtlasPage;
    let ref mut fresh47 = *(&mut (*self_0).atlas as *mut *const spAtlas
        as *mut *mut spAtlas);
    *fresh47 = atlas;
    let ref mut fresh48 = *(&mut (*self_0).name as *mut *const c_char
        as *mut *mut c_char);
    *fresh48 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul(
                (spine_strlen(name)).wrapping_add(1 as c_int as c_ulong),
            ),
        b"spine.c\0" as *const u8 as *const c_char,
        3055 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh48, name);
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
    return _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spAtlasRegion>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        3068 as c_int,
    ) as *mut spAtlasRegion;
}
#[no_mangle]
pub unsafe extern "C" fn spAtlasRegion_dispose(mut self_0: *mut spAtlasRegion) {
    _spFree((*self_0).name as *mut c_void);
    _spFree((*self_0).splits as *mut c_void);
    _spFree((*self_0).pads as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
unsafe extern "C" fn trim(mut str: *mut Str) {
    while isspace_(*(*str).begin as c_uchar as c_int) != 0
        && (*str).begin < (*str).end
    {
        (*str).begin = ((*str).begin).offset(1);
    }
    if (*str).begin == (*str).end {
        return;
    }
    (*str).end = ((*str).end).offset(-1);
    while *(*str).end as c_uchar as c_int == '\r' as i32
        && (*str).end >= (*str).begin
    {
        (*str).end = ((*str).end).offset(-1);
    }
    (*str).end = ((*str).end).offset(1);
}
unsafe extern "C" fn readLine(
    mut begin: *mut *const c_char,
    mut end: *const c_char,
    mut str: *mut Str,
) -> c_int {
    if *begin == end {
        return 0 as c_int;
    }
    (*str).begin = *begin;
    while *begin != end && **begin as c_int != '\n' as i32 {
        *begin = (*begin).offset(1);
    }
    (*str).end = *begin;
    trim(str);
    if *begin != end {
        *begin = (*begin).offset(1);
    }
    return 1 as c_int;
}
unsafe extern "C" fn beginPast(mut str: *mut Str, mut c: c_char) -> c_int {
    let mut begin: *const c_char = (*str).begin;
    loop {
        let mut lastSkippedChar: c_char = *begin;
        if begin == (*str).end {
            return 0 as c_int;
        }
        begin = begin.offset(1);
        if lastSkippedChar as c_int == c as c_int {
            break;
        }
    }
    (*str).begin = begin;
    return 1 as c_int;
}
unsafe extern "C" fn readValue(
    mut begin: *mut *const c_char,
    mut end: *const c_char,
    mut str: *mut Str,
) -> c_int {
    readLine(begin, end, str);
    if beginPast(str, ':' as i32 as c_char) == 0 {
        return 0 as c_int;
    }
    trim(str);
    return 1 as c_int;
}
unsafe extern "C" fn readTuple(
    mut begin: *mut *const c_char,
    mut end: *const c_char,
    mut tuple: *mut Str,
) -> c_int {
    let mut i: c_int = 0;
    let mut str: Str = {
        let mut init = Str {
            begin: 0 as *const c_char,
            end: 0 as *const c_char,
        };
        init
    };
    readLine(begin, end, &mut str);
    if beginPast(&mut str, ':' as i32 as c_char) == 0 {
        return 0 as c_int;
    }
    i = 0 as c_int;
    while i < 3 as c_int {
        let ref mut fresh49 = (*tuple.offset(i as isize)).begin;
        *fresh49 = str.begin;
        if beginPast(&mut str, ',' as i32 as c_char) == 0 {
            break;
        }
        let ref mut fresh50 = (*tuple.offset(i as isize)).end;
        *fresh50 = (str.begin).offset(-(2 as c_int as isize));
        trim(&mut *tuple.offset(i as isize));
        i += 1;
    }
    let ref mut fresh51 = (*tuple.offset(i as isize)).begin;
    *fresh51 = str.begin;
    let ref mut fresh52 = (*tuple.offset(i as isize)).end;
    *fresh52 = str.end;
    trim(&mut *tuple.offset(i as isize));
    return i + 1 as c_int;
}
unsafe extern "C" fn mallocString(mut str: *mut Str) -> *mut c_char {
    let mut length: c_int = ((*str).end).offset_from((*str).begin) as c_long
        as c_int;
    let mut string: *mut c_char = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul((length + 1 as c_int) as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        3153 as c_int,
    ) as *mut c_char;
    spine_memcpy(
        string as *mut c_void,
        (*str).begin as *const c_void,
        length as size_t,
    );
    *string.offset(length as isize) = '\0' as i32 as c_char;
    return string;
}
unsafe extern "C" fn indexOf(
    mut array: *mut *const c_char,
    mut count: c_int,
    mut str: *mut Str,
) -> c_int {
    let mut length: c_int = ((*str).end).offset_from((*str).begin) as c_long
        as c_int;
    let mut i: c_int = 0;
    i = count - 1 as c_int;
    while i >= 0 as c_int {
        if spine_strncmp(*array.offset(i as isize), (*str).begin, length as size_t)
            == 0 as c_int
        {
            return i;
        }
        i -= 1;
    }
    return 0 as c_int;
}
unsafe extern "C" fn equals(
    mut str: *mut Str,
    mut other: *const c_char,
) -> c_int {
    return (spine_strncmp(
        other,
        (*str).begin,
        ((*str).end).offset_from((*str).begin) as c_long as size_t,
    ) == 0 as c_int) as c_int;
}
unsafe extern "C" fn toInt(mut str: *mut Str) -> c_int {
    return spine_strtol(
        (*str).begin,
        &mut (*str).end as *mut *const c_char as *mut *mut c_char,
        10 as c_int,
    ) as c_int;
}
unsafe extern "C" fn abortAtlas(mut self_0: *mut spAtlas) -> *mut spAtlas {
    spAtlas_dispose(self_0);
    return 0 as *mut spAtlas;
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
pub unsafe extern "C" fn spAtlas_create(
    mut begin: *const c_char,
    mut length: c_int,
    mut dir: *const c_char,
    mut rendererObject: *mut c_void,
) -> *mut spAtlas {
    let mut self_0: *mut spAtlas = 0 as *mut spAtlas;
    let mut count: c_int = 0;
    let mut end: *const c_char = begin.offset(length as isize);
    let mut dirLength: c_int = spine_strlen(dir) as c_int;
    let mut needsSlash: c_int = (dirLength > 0 as c_int
        && *dir.offset((dirLength - 1 as c_int) as isize) as c_int
            != '/' as i32
        && *dir.offset((dirLength - 1 as c_int) as isize) as c_int
            != '\\' as i32) as c_int;
    let mut page: *mut spAtlasPage = 0 as *mut spAtlasPage;
    let mut lastPage: *mut spAtlasPage = 0 as *mut spAtlasPage;
    let mut lastRegion: *mut spAtlasRegion = 0 as *mut spAtlasRegion;
    let mut str: Str = Str {
        begin: 0 as *const c_char,
        end: 0 as *const c_char,
    };
    let mut tuple: [Str; 4] = [Str {
        begin: 0 as *const c_char,
        end: 0 as *const c_char,
    }; 4];
    self_0 = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spAtlas>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        3198 as c_int,
    ) as *mut spAtlas;
    (*self_0).rendererObject = rendererObject;
    while readLine(&mut begin, end, &mut str) != 0 {
        if (str.end).offset_from(str.begin) as c_long
            == 0 as c_int as c_long
        {
            page = 0 as *mut spAtlasPage;
        } else if page.is_null() {
            let mut name: *mut c_char = mallocString(&mut str);
            let mut path: *mut c_char = _spMalloc(
                (::core::mem::size_of::<c_char>() as c_ulong)
                    .wrapping_mul(
                        ((dirLength + needsSlash) as c_ulong)
                            .wrapping_add(spine_strlen(name))
                            .wrapping_add(1 as c_int as c_ulong),
                    ),
                b"spine.c\0" as *const u8 as *const c_char,
                3206 as c_int,
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
            match readTuple(&mut begin, end, tuple.as_mut_ptr()) {
                0 => return abortAtlas(self_0),
                2 => {
                    (*page).width = toInt(tuple.as_mut_ptr());
                    (*page)
                        .height = toInt(
                        tuple.as_mut_ptr().offset(1 as c_int as isize),
                    );
                    if readTuple(&mut begin, end, tuple.as_mut_ptr()) == 0 {
                        return abortAtlas(self_0);
                    }
                }
                _ => {}
            }
            (*page)
                .format = indexOf(
                formatNames.as_mut_ptr(),
                8 as c_int,
                tuple.as_mut_ptr(),
            ) as spAtlasFormat;
            if readTuple(&mut begin, end, tuple.as_mut_ptr()) == 0 {
                return abortAtlas(self_0);
            }
            (*page)
                .minFilter = indexOf(
                textureFilterNames.as_mut_ptr(),
                8 as c_int,
                tuple.as_mut_ptr(),
            ) as spAtlasFilter;
            (*page)
                .magFilter = indexOf(
                textureFilterNames.as_mut_ptr(),
                8 as c_int,
                tuple.as_mut_ptr().offset(1 as c_int as isize),
            ) as spAtlasFilter;
            if readValue(&mut begin, end, &mut str) == 0 {
                return abortAtlas(self_0);
            }
            (*page).uWrap = SP_ATLAS_CLAMPTOEDGE;
            (*page).vWrap = SP_ATLAS_CLAMPTOEDGE;
            if equals(&mut str, b"none\0" as *const u8 as *const c_char) == 0 {
                if (str.end).offset_from(str.begin) as c_long
                    == 1 as c_int as c_long
                {
                    if *str.begin as c_int == 'x' as i32 {
                        (*page).uWrap = SP_ATLAS_REPEAT;
                    } else if *str.begin as c_int == 'y' as i32 {
                        (*page).vWrap = SP_ATLAS_REPEAT;
                    }
                } else if equals(&mut str, b"xy\0" as *const u8 as *const c_char)
                        != 0
                    {
                    (*page).uWrap = SP_ATLAS_REPEAT;
                    (*page).vWrap = SP_ATLAS_REPEAT;
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
            (*region).name = mallocString(&mut str);
            if readValue(&mut begin, end, &mut str) == 0 {
                return abortAtlas(self_0);
            }
            if equals(&mut str, b"true\0" as *const u8 as *const c_char) != 0 {
                (*region).degrees = 90 as c_int;
            } else if equals(&mut str, b"false\0" as *const u8 as *const c_char)
                    != 0
                {
                (*region).degrees = 0 as c_int;
            } else {
                (*region).degrees = toInt(&mut str);
            }
            (*region).rotate = ((*region).degrees == 90 as c_int) as c_int;
            if readTuple(&mut begin, end, tuple.as_mut_ptr()) != 2 as c_int {
                return abortAtlas(self_0);
            }
            (*region).x = toInt(tuple.as_mut_ptr());
            (*region).y = toInt(tuple.as_mut_ptr().offset(1 as c_int as isize));
            if readTuple(&mut begin, end, tuple.as_mut_ptr()) != 2 as c_int {
                return abortAtlas(self_0);
            }
            (*region).width = toInt(tuple.as_mut_ptr());
            (*region)
                .height = toInt(tuple.as_mut_ptr().offset(1 as c_int as isize));
            (*region).u = (*region).x as c_float / (*page).width as c_float;
            (*region).v = (*region).y as c_float / (*page).height as c_float;
            if (*region).rotate != 0 {
                (*region)
                    .u2 = ((*region).x + (*region).height) as c_float
                    / (*page).width as c_float;
                (*region)
                    .v2 = ((*region).y + (*region).width) as c_float
                    / (*page).height as c_float;
            } else {
                (*region)
                    .u2 = ((*region).x + (*region).width) as c_float
                    / (*page).width as c_float;
                (*region)
                    .v2 = ((*region).y + (*region).height) as c_float
                    / (*page).height as c_float;
            }
            count = readTuple(&mut begin, end, tuple.as_mut_ptr());
            if count == 0 {
                return abortAtlas(self_0);
            }
            if count == 4 as c_int {
                (*region)
                    .splits = _spMalloc(
                    (::core::mem::size_of::<c_int>() as c_ulong)
                        .wrapping_mul(4 as c_int as c_ulong),
                    b"spine.c\0" as *const u8 as *const c_char,
                    3293 as c_int,
                ) as *mut c_int;
                *((*region).splits)
                    .offset(0 as c_int as isize) = toInt(tuple.as_mut_ptr());
                *((*region).splits)
                    .offset(
                        1 as c_int as isize,
                    ) = toInt(tuple.as_mut_ptr().offset(1 as c_int as isize));
                *((*region).splits)
                    .offset(
                        2 as c_int as isize,
                    ) = toInt(tuple.as_mut_ptr().offset(2 as c_int as isize));
                *((*region).splits)
                    .offset(
                        3 as c_int as isize,
                    ) = toInt(tuple.as_mut_ptr().offset(3 as c_int as isize));
                count = readTuple(&mut begin, end, tuple.as_mut_ptr());
                if count == 0 {
                    return abortAtlas(self_0);
                }
                if count == 4 as c_int {
                    (*region)
                        .pads = _spMalloc(
                        (::core::mem::size_of::<c_int>() as c_ulong)
                            .wrapping_mul(4 as c_int as c_ulong),
                        b"spine.c\0" as *const u8 as *const c_char,
                        3302 as c_int,
                    ) as *mut c_int;
                    *((*region).pads)
                        .offset(0 as c_int as isize) = toInt(tuple.as_mut_ptr());
                    *((*region).pads)
                        .offset(
                            1 as c_int as isize,
                        ) = toInt(tuple.as_mut_ptr().offset(1 as c_int as isize));
                    *((*region).pads)
                        .offset(
                            2 as c_int as isize,
                        ) = toInt(tuple.as_mut_ptr().offset(2 as c_int as isize));
                    *((*region).pads)
                        .offset(
                            3 as c_int as isize,
                        ) = toInt(tuple.as_mut_ptr().offset(3 as c_int as isize));
                    if readTuple(&mut begin, end, tuple.as_mut_ptr()) == 0 {
                        return abortAtlas(self_0);
                    }
                }
            }
            (*region).originalWidth = toInt(tuple.as_mut_ptr());
            (*region)
                .originalHeight = toInt(
                tuple.as_mut_ptr().offset(1 as c_int as isize),
            );
            readTuple(&mut begin, end, tuple.as_mut_ptr());
            (*region).offsetX = toInt(tuple.as_mut_ptr());
            (*region)
                .offsetY = toInt(tuple.as_mut_ptr().offset(1 as c_int as isize));
            if readValue(&mut begin, end, &mut str) == 0 {
                return abortAtlas(self_0);
            }
            (*region).index = toInt(&mut str);
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
        3341 as c_int,
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
#[no_mangle]
pub unsafe extern "C" fn _spAtlasAttachmentLoader_createAttachment(
    mut loader: *mut spAttachmentLoader,
    mut _skin: *mut spSkin,
    mut type_0: spAttachmentType,
    mut name: *const c_char,
    mut path: *const c_char,
) -> *mut spAttachment {
    let mut self_0: *mut spAtlasAttachmentLoader = loader
        as *mut spAtlasAttachmentLoader;
    match type_0 as c_uint {
        0 => {
            let mut attachment: *mut spRegionAttachment = 0 as *mut spRegionAttachment;
            let mut region: *mut spAtlasRegion = spAtlas_findRegion(
                (*self_0).atlas,
                path,
            );
            if region.is_null() {
                _spAttachmentLoader_setError(
                    loader,
                    b"Region not found: \0" as *const u8 as *const c_char,
                    path,
                );
                return 0 as *mut spAttachment;
            }
            attachment = spRegionAttachment_create(name);
            (*attachment).rendererObject = region as *mut c_void;
            spRegionAttachment_setUVs(
                attachment,
                (*region).u,
                (*region).v,
                (*region).u2,
                (*region).v2,
                (*region).rotate,
            );
            (*attachment).regionOffsetX = (*region).offsetX;
            (*attachment).regionOffsetY = (*region).offsetY;
            (*attachment).regionWidth = (*region).width;
            (*attachment).regionHeight = (*region).height;
            (*attachment).regionOriginalWidth = (*region).originalWidth;
            (*attachment).regionOriginalHeight = (*region).originalHeight;
            return &mut (*attachment).super_0;
        }
        2 | 3 => {
            let mut attachment_0: *mut spMeshAttachment = 0 as *mut spMeshAttachment;
            let mut region_0: *mut spAtlasRegion = spAtlas_findRegion(
                (*self_0).atlas,
                path,
            );
            if region_0.is_null() {
                _spAttachmentLoader_setError(
                    loader,
                    b"Region not found: \0" as *const u8 as *const c_char,
                    path,
                );
                return 0 as *mut spAttachment;
            }
            attachment_0 = spMeshAttachment_create(name);
            (*attachment_0).rendererObject = region_0 as *mut c_void;
            (*attachment_0).regionU = (*region_0).u;
            (*attachment_0).regionV = (*region_0).v;
            (*attachment_0).regionU2 = (*region_0).u2;
            (*attachment_0).regionV2 = (*region_0).v2;
            (*attachment_0).regionRotate = (*region_0).rotate;
            (*attachment_0).regionDegrees = (*region_0).degrees;
            (*attachment_0).regionOffsetX = (*region_0).offsetX;
            (*attachment_0).regionOffsetY = (*region_0).offsetY;
            (*attachment_0).regionWidth = (*region_0).width;
            (*attachment_0).regionHeight = (*region_0).height;
            (*attachment_0).regionOriginalWidth = (*region_0).originalWidth;
            (*attachment_0).regionOriginalHeight = (*region_0).originalHeight;
            return &mut (*attachment_0).super_0.super_0;
        }
        1 => {
            return &mut (*(spBoundingBoxAttachment_create
                as unsafe extern "C" fn(
                    *const c_char,
                ) -> *mut spBoundingBoxAttachment)(name))
                .super_0
                .super_0;
        }
        4 => {
            return &mut (*(spPathAttachment_create
                as unsafe extern "C" fn(
                    *const c_char,
                ) -> *mut spPathAttachment)(name))
                .super_0
                .super_0;
        }
        5 => {
            return &mut (*(spPointAttachment_create
                as unsafe extern "C" fn(
                    *const c_char,
                ) -> *mut spPointAttachment)(name))
                .super_0;
        }
        6 => {
            return &mut (*(spClippingAttachment_create
                as unsafe extern "C" fn(
                    *const c_char,
                ) -> *mut spClippingAttachment)(name))
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
        3475 as c_int,
    ) as *mut spAtlasAttachmentLoader;
    _spAttachmentLoader_init(
        &mut (*self_0).super_0,
        Some(
            _spAttachmentLoader_deinit
                as unsafe extern "C" fn(*mut spAttachmentLoader) -> (),
        ),
        Some(
            _spAtlasAttachmentLoader_createAttachment
                as unsafe extern "C" fn(
                    *mut spAttachmentLoader,
                    *mut spSkin,
                    spAttachmentType,
                    *const c_char,
                    *const c_char,
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
    mut dispose: Option::<unsafe extern "C" fn(*mut spAttachment) -> ()>,
    mut copy: Option::<unsafe extern "C" fn(*mut spAttachment) -> *mut spAttachment>,
) {
    let ref mut fresh53 = *(&(*self_0).vtable as *const *const c_void
        as *mut *mut _spAttachmentVtable);
    *fresh53 = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<_spAttachmentVtable>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        3521 as c_int,
    ) as *mut _spAttachmentVtable;
    let ref mut fresh54 = (*((*self_0).vtable as *mut _spAttachmentVtable)).dispose;
    *fresh54 = dispose;
    let ref mut fresh55 = (*((*self_0).vtable as *mut _spAttachmentVtable)).copy;
    *fresh55 = copy;
    let ref mut fresh56 = *(&(*self_0).name as *const *const c_char
        as *mut *mut c_char);
    *fresh56 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul(
                (spine_strlen(name)).wrapping_add(1 as c_int as c_ulong),
            ),
        b"spine.c\0" as *const u8 as *const c_char,
        3525 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh56, name);
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
pub unsafe extern "C" fn spAttachment_copy(
    mut self_0: *mut spAttachment,
) -> *mut spAttachment {
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
    mut dispose: Option::<unsafe extern "C" fn(*mut spAttachmentLoader) -> ()>,
    mut createAttachment: Option::<
        unsafe extern "C" fn(
            *mut spAttachmentLoader,
            *mut spSkin,
            spAttachmentType,
            *const c_char,
            *const c_char,
        ) -> *mut spAttachment,
    >,
    mut configureAttachment: Option::<
        unsafe extern "C" fn(*mut spAttachmentLoader, *mut spAttachment) -> (),
    >,
    mut disposeAttachment: Option::<
        unsafe extern "C" fn(*mut spAttachmentLoader, *mut spAttachment) -> (),
    >,
) {
    let ref mut fresh57 = *(&(*self_0).vtable as *const *const c_void
        as *mut *mut _spAttachmentLoaderVtable);
    *fresh57 = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<_spAttachmentLoaderVtable>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        3591 as c_int,
    ) as *mut _spAttachmentLoaderVtable;
    let ref mut fresh58 = (*((*self_0).vtable as *mut _spAttachmentLoaderVtable))
        .dispose;
    *fresh58 = dispose;
    let ref mut fresh59 = (*((*self_0).vtable as *mut _spAttachmentLoaderVtable))
        .createAttachment;
    *fresh59 = createAttachment;
    let ref mut fresh60 = (*((*self_0).vtable as *mut _spAttachmentLoaderVtable))
        .configureAttachment;
    *fresh60 = configureAttachment;
    let ref mut fresh61 = (*((*self_0).vtable as *mut _spAttachmentLoaderVtable))
        .disposeAttachment;
    *fresh61 = disposeAttachment;
}
#[no_mangle]
pub unsafe extern "C" fn _spAttachmentLoader_deinit(
    mut self_0: *mut spAttachmentLoader,
) {
    _spFree((*self_0).vtable as *mut c_void);
    _spFree((*self_0).error1 as *mut c_void);
    _spFree((*self_0).error2 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spAttachmentLoader_dispose(
    mut self_0: *mut spAttachmentLoader,
) {
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
) -> *mut spAttachment {
    _spFree((*self_0).error1 as *mut c_void);
    _spFree((*self_0).error2 as *mut c_void);
    (*self_0).error1 = 0 as *const c_char;
    (*self_0).error2 = 0 as *const c_char;
    return ((*((*self_0).vtable as *mut _spAttachmentLoaderVtable)).createAttachment)
        .expect("non-null function pointer")(self_0, skin, type_0, name, path);
}
#[no_mangle]
pub unsafe extern "C" fn spAttachmentLoader_configureAttachment(
    mut self_0: *mut spAttachmentLoader,
    mut attachment: *mut spAttachment,
) {
    if ((*((*self_0).vtable as *mut _spAttachmentLoaderVtable)).configureAttachment)
        .is_none()
    {
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
    if ((*((*self_0).vtable as *mut _spAttachmentLoaderVtable)).disposeAttachment)
        .is_none()
    {
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
    let ref mut fresh62 = *(&mut (*self_0).error1 as *mut *const c_char
        as *mut *mut c_char);
    *fresh62 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul(
                (spine_strlen(error1)).wrapping_add(1 as c_int as c_ulong),
            ),
        b"spine.c\0" as *const u8 as *const c_char,
        3631 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh62, error1);
    let ref mut fresh63 = *(&mut (*self_0).error2 as *mut *const c_char
        as *mut *mut c_char);
    *fresh63 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul(
                (spine_strlen(error2)).wrapping_add(1 as c_int as c_ulong),
            ),
        b"spine.c\0" as *const u8 as *const c_char,
        3632 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh63, error2);
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
        3683 as c_int,
    ) as *mut spBone;
    let ref mut fresh64 = *(&(*self_0).data as *const *mut spBoneData
        as *mut *mut spBoneData);
    *fresh64 = data;
    let ref mut fresh65 = *(&(*self_0).skeleton as *const *mut spSkeleton
        as *mut *mut spSkeleton);
    *fresh65 = skeleton;
    let ref mut fresh66 = *(&(*self_0).parent as *const *mut spBone as *mut *mut spBone);
    *fresh66 = parent;
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
        * (if spBone_isYDown() != 0 { -(1 as c_int) } else { 1 as c_int })
            as c_float;
    (*self_0).ax = x;
    (*self_0).ay = y;
    (*self_0).arotation = rotation;
    (*self_0).ascaleX = scaleX;
    (*self_0).ascaleY = scaleY;
    (*self_0).ashearX = shearX;
    (*self_0).ashearY = shearY;
    (*self_0).appliedValid = 1 as c_int;
    if parent.is_null() {
        let mut rotationY: c_float = rotation + 90 as c_int as c_float
            + shearY;
        *(&(*self_0).a as *const c_float
            as *mut c_float) = cosf(
            (rotation + shearX)
                * (3.1415926535897932385f32 / 180 as c_int as c_float),
        ) * scaleX * sx;
        *(&(*self_0).b as *const c_float
            as *mut c_float) = cosf(
            rotationY * (3.1415926535897932385f32 / 180 as c_int as c_float),
        ) * scaleY * sx;
        *(&(*self_0).c as *const c_float
            as *mut c_float) = sinf(
            (rotation + shearX)
                * (3.1415926535897932385f32 / 180 as c_int as c_float),
        ) * scaleX * sy;
        *(&(*self_0).d as *const c_float
            as *mut c_float) = sinf(
            rotationY * (3.1415926535897932385f32 / 180 as c_int as c_float),
        ) * scaleY * sy;
        *(&(*self_0).worldX as *const c_float
            as *mut c_float) = x * sx + (*(*self_0).skeleton).x;
        *(&(*self_0).worldY as *const c_float
            as *mut c_float) = y * sy + (*(*self_0).skeleton).y;
        return;
    }
    pa = (*parent).a;
    pb = (*parent).b;
    pc = (*parent).c;
    pd = (*parent).d;
    *(&(*self_0).worldX as *const c_float
        as *mut c_float) = pa * x + pb * y + (*parent).worldX;
    *(&(*self_0).worldY as *const c_float
        as *mut c_float) = pc * x + pd * y + (*parent).worldY;
    match (*(*self_0).data).transformMode as c_uint {
        0 => {
            let mut rotationY_0: c_float = rotation
                + 90 as c_int as c_float + shearY;
            let mut la: c_float = cosf(
                (rotation + shearX)
                    * (3.1415926535897932385f32 / 180 as c_int as c_float),
            ) * scaleX;
            let mut lb: c_float = cosf(
                rotationY_0
                    * (3.1415926535897932385f32 / 180 as c_int as c_float),
            ) * scaleY;
            let mut lc: c_float = sinf(
                (rotation + shearX)
                    * (3.1415926535897932385f32 / 180 as c_int as c_float),
            ) * scaleX;
            let mut ld: c_float = sinf(
                rotationY_0
                    * (3.1415926535897932385f32 / 180 as c_int as c_float),
            ) * scaleY;
            *(&(*self_0).a as *const c_float
                as *mut c_float) = pa * la + pb * lc;
            *(&(*self_0).b as *const c_float
                as *mut c_float) = pa * lb + pb * ld;
            *(&(*self_0).c as *const c_float
                as *mut c_float) = pc * la + pd * lc;
            *(&(*self_0).d as *const c_float
                as *mut c_float) = pc * lb + pd * ld;
            return;
        }
        1 => {
            let mut rotationY_1: c_float = rotation
                + 90 as c_int as c_float + shearY;
            *(&(*self_0).a as *const c_float
                as *mut c_float) = cosf(
                (rotation + shearX)
                    * (3.1415926535897932385f32 / 180 as c_int as c_float),
            ) * scaleX;
            *(&(*self_0).b as *const c_float
                as *mut c_float) = cosf(
                rotationY_1
                    * (3.1415926535897932385f32 / 180 as c_int as c_float),
            ) * scaleY;
            *(&(*self_0).c as *const c_float
                as *mut c_float) = sinf(
                (rotation + shearX)
                    * (3.1415926535897932385f32 / 180 as c_int as c_float),
            ) * scaleX;
            *(&(*self_0).d as *const c_float
                as *mut c_float) = sinf(
                rotationY_1
                    * (3.1415926535897932385f32 / 180 as c_int as c_float),
            ) * scaleY;
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
                prx = atan2f(pc, pa)
                    * (180 as c_int as c_float / 3.1415926535897932385f32);
            } else {
                pa = 0 as c_int as c_float;
                pc = 0 as c_int as c_float;
                prx = 90 as c_int as c_float
                    - atan2f(pd, pb)
                        * (180 as c_int as c_float
                            / 3.1415926535897932385f32);
            }
            rx = rotation + shearX - prx;
            ry = rotation + shearY - prx + 90 as c_int as c_float;
            la_0 = cosf(
                rx * (3.1415926535897932385f32 / 180 as c_int as c_float),
            ) * scaleX;
            lb_0 = cosf(
                ry * (3.1415926535897932385f32 / 180 as c_int as c_float),
            ) * scaleY;
            lc_0 = sinf(
                rx * (3.1415926535897932385f32 / 180 as c_int as c_float),
            ) * scaleX;
            ld_0 = sinf(
                ry * (3.1415926535897932385f32 / 180 as c_int as c_float),
            ) * scaleY;
            *(&(*self_0).a as *const c_float
                as *mut c_float) = pa * la_0 - pb * lc_0;
            *(&(*self_0).b as *const c_float
                as *mut c_float) = pa * lb_0 - pb * ld_0;
            *(&(*self_0).c as *const c_float
                as *mut c_float) = pc * la_0 + pd * lc_0;
            *(&(*self_0).d as *const c_float
                as *mut c_float) = pc * lb_0 + pd * ld_0;
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
            cosine = cosf(
                rotation
                    * (3.1415926535897932385f32 / 180 as c_int as c_float),
            );
            sine = sinf(
                rotation
                    * (3.1415926535897932385f32 / 180 as c_int as c_float),
            );
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
                        != (sy < 0 as c_int as c_float) as c_int)
                        as c_int
            {
                s_0 = -s_0;
            }
            r = 3.1415926535897932385f32 / 2 as c_int as c_float
                + atan2f(zc, za);
            zb = cosf(r) * s_0;
            zd = sinf(r) * s_0;
            la_1 = cosf(
                shearX * (3.1415926535897932385f32 / 180 as c_int as c_float),
            ) * scaleX;
            lb_1 = cosf(
                (90 as c_int as c_float + shearY)
                    * (3.1415926535897932385f32 / 180 as c_int as c_float),
            ) * scaleY;
            lc_1 = sinf(
                shearX * (3.1415926535897932385f32 / 180 as c_int as c_float),
            ) * scaleX;
            ld_1 = sinf(
                (90 as c_int as c_float + shearY)
                    * (3.1415926535897932385f32 / 180 as c_int as c_float),
            ) * scaleY;
            *(&(*self_0).a as *const c_float
                as *mut c_float) = za * la_1 + zb * lc_1;
            *(&(*self_0).b as *const c_float
                as *mut c_float) = za * lb_1 + zb * ld_1;
            *(&(*self_0).c as *const c_float
                as *mut c_float) = zc * la_1 + zd * lc_1;
            *(&(*self_0).d as *const c_float
                as *mut c_float) = zc * lb_1 + zd * ld_1;
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
pub unsafe extern "C" fn spBone_getWorldRotationX(
    mut self_0: *mut spBone,
) -> c_float {
    return atan2f((*self_0).c, (*self_0).a)
        * (180 as c_int as c_float / 3.1415926535897932385f32);
}
#[no_mangle]
pub unsafe extern "C" fn spBone_getWorldRotationY(
    mut self_0: *mut spBone,
) -> c_float {
    return atan2f((*self_0).d, (*self_0).b)
        * (180 as c_int as c_float / 3.1415926535897932385f32);
}
#[no_mangle]
pub unsafe extern "C" fn spBone_getWorldScaleX(
    mut self_0: *mut spBone,
) -> c_float {
    return spine_sqrtf((*self_0).a * (*self_0).a + (*self_0).c * (*self_0).c);
}
#[no_mangle]
pub unsafe extern "C" fn spBone_getWorldScaleY(
    mut self_0: *mut spBone,
) -> c_float {
    return spine_sqrtf((*self_0).b * (*self_0).b + (*self_0).d * (*self_0).d);
}
#[no_mangle]
pub unsafe extern "C" fn spBone_updateAppliedTransform(mut self_0: *mut spBone) {
    let mut parent: *mut spBone = (*self_0).parent;
    (*self_0).appliedValid = 1 as c_int;
    if parent.is_null() {
        (*self_0).ax = (*self_0).worldX;
        (*self_0).ay = (*self_0).worldY;
        (*self_0)
            .arotation = atan2f((*self_0).c, (*self_0).a)
            * (180 as c_int as c_float / 3.1415926535897932385f32);
        (*self_0)
            .ascaleX = spine_sqrtf(
            (*self_0).a * (*self_0).a + (*self_0).c * (*self_0).c,
        );
        (*self_0)
            .ascaleY = spine_sqrtf(
            (*self_0).b * (*self_0).b + (*self_0).d * (*self_0).d,
        );
        (*self_0).ashearX = 0 as c_int as c_float;
        (*self_0)
            .ashearY = atan2f(
            (*self_0).a * (*self_0).b + (*self_0).c * (*self_0).d,
            (*self_0).a * (*self_0).d - (*self_0).b * (*self_0).c,
        ) * (180 as c_int as c_float / 3.1415926535897932385f32);
    } else {
        let mut pa: c_float = (*parent).a;
        let mut pb: c_float = (*parent).b;
        let mut pc: c_float = (*parent).c;
        let mut pd: c_float = (*parent).d;
        let mut pid: c_float = 1 as c_int as c_float
            / (pa * pd - pb * pc);
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
            (*self_0)
                .ashearY = atan2f(ra * rb + rc * rd, det)
                * (180 as c_int as c_float / 3.1415926535897932385f32);
            (*self_0)
                .arotation = atan2f(rc, ra)
                * (180 as c_int as c_float / 3.1415926535897932385f32);
        } else {
            (*self_0).ascaleX = 0 as c_int as c_float;
            (*self_0).ascaleY = spine_sqrtf(rb * rb + rd * rd);
            (*self_0).ashearY = 0 as c_int as c_float;
            (*self_0)
                .arotation = 90 as c_int as c_float
                - atan2f(rd, rb)
                    * (180 as c_int as c_float / 3.1415926535897932385f32);
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
    let mut a: c_float = (*self_0).a;
    let mut b: c_float = (*self_0).b;
    let mut c: c_float = (*self_0).c;
    let mut d: c_float = (*self_0).d;
    let mut invDet: c_float = 1 as c_int as c_float / (a * d - b * c);
    let mut x: c_float = worldX - (*self_0).worldX;
    let mut y: c_float = worldY - (*self_0).worldY;
    *localX = x * d * invDet - y * b * invDet;
    *localY = y * a * invDet - x * c * invDet;
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
    sine = sinf(
        worldRotation * (3.1415926535897932385f32 / 180 as c_int as c_float),
    );
    cosine = cosf(
        worldRotation * (3.1415926535897932385f32 / 180 as c_int as c_float),
    );
    return atan2f(
        (*self_0).a * sine - (*self_0).c * cosine,
        (*self_0).d * cosine - (*self_0).b * sine,
    ) * (180 as c_int as c_float / 3.1415926535897932385f32)
        + (*self_0).rotation - (*self_0).shearX;
}
#[no_mangle]
pub unsafe extern "C" fn spBone_localToWorldRotation(
    mut self_0: *mut spBone,
    mut localRotation: c_float,
) -> c_float {
    let mut sine: c_float = 0.;
    let mut cosine: c_float = 0.;
    localRotation -= (*self_0).rotation - (*self_0).shearX;
    sine = sinf(
        localRotation * (3.1415926535897932385f32 / 180 as c_int as c_float),
    );
    cosine = cosf(
        localRotation * (3.1415926535897932385f32 / 180 as c_int as c_float),
    );
    return atan2f(
        cosine * (*self_0).c + sine * (*self_0).d,
        cosine * (*self_0).a + sine * (*self_0).b,
    ) * (180 as c_int as c_float / 3.1415926535897932385f32);
}
#[no_mangle]
pub unsafe extern "C" fn spBone_rotateWorld(
    mut self_0: *mut spBone,
    mut degrees: c_float,
) {
    let mut a: c_float = (*self_0).a;
    let mut b: c_float = (*self_0).b;
    let mut c: c_float = (*self_0).c;
    let mut d: c_float = (*self_0).d;
    let mut cosine: c_float = cosf(
        degrees * (3.1415926535897932385f32 / 180 as c_int as c_float),
    );
    let mut sine: c_float = sinf(
        degrees * (3.1415926535897932385f32 / 180 as c_int as c_float),
    );
    *(&(*self_0).a as *const c_float
        as *mut c_float) = cosine * a - sine * c;
    *(&(*self_0).b as *const c_float
        as *mut c_float) = cosine * b - sine * d;
    *(&(*self_0).c as *const c_float
        as *mut c_float) = sine * a + cosine * c;
    *(&(*self_0).d as *const c_float
        as *mut c_float) = sine * b + cosine * d;
    *(&mut (*self_0).appliedValid as *mut c_int) = 0 as c_int;
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
        3962 as c_int,
    ) as *mut spBoneData;
    *(&(*self_0).index as *const c_int as *mut c_int) = index;
    let ref mut fresh67 = *(&(*self_0).name as *const *const c_char
        as *mut *mut c_char);
    *fresh67 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul(
                (spine_strlen(name)).wrapping_add(1 as c_int as c_ulong),
            ),
        b"spine.c\0" as *const u8 as *const c_char,
        3964 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh67, name);
    let ref mut fresh68 = *(&(*self_0).parent as *const *mut spBoneData
        as *mut *mut spBoneData);
    *fresh68 = parent;
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
pub unsafe extern "C" fn _spBoundingBoxAttachment_dispose(
    mut attachment: *mut spAttachment,
) {
    let mut self_0: *mut spBoundingBoxAttachment = attachment
        as *mut spBoundingBoxAttachment;
    _spVertexAttachment_deinit(&mut (*self_0).super_0);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _spBoundingBoxAttachment_copy(
    mut attachment: *mut spAttachment,
) -> *mut spAttachment {
    let mut copy: *mut spBoundingBoxAttachment = spBoundingBoxAttachment_create(
        (*attachment).name,
    );
    let mut self_0: *mut spBoundingBoxAttachment = attachment
        as *mut spBoundingBoxAttachment;
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
        4024 as c_int,
    ) as *mut spBoundingBoxAttachment;
    _spVertexAttachment_init(&mut (*self_0).super_0);
    _spAttachment_init(
        &mut (*self_0).super_0.super_0,
        name,
        SP_ATTACHMENT_BOUNDING_BOX,
        Some(
            _spBoundingBoxAttachment_dispose
                as unsafe extern "C" fn(*mut spAttachment) -> (),
        ),
        Some(
            _spBoundingBoxAttachment_copy
                as unsafe extern "C" fn(*mut spAttachment) -> *mut spAttachment,
        ),
    );
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn _spClippingAttachment_dispose(
    mut attachment: *mut spAttachment,
) {
    let mut self_0: *mut spClippingAttachment = attachment as *mut spClippingAttachment;
    _spVertexAttachment_deinit(&mut (*self_0).super_0);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _spClippingAttachment_copy(
    mut attachment: *mut spAttachment,
) -> *mut spAttachment {
    let mut copy: *mut spClippingAttachment = spClippingAttachment_create(
        (*attachment).name,
    );
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
        4078 as c_int,
    ) as *mut spClippingAttachment;
    _spVertexAttachment_init(&mut (*self_0).super_0);
    _spAttachment_init(
        &mut (*self_0).super_0.super_0,
        name,
        SP_ATTACHMENT_CLIPPING,
        Some(
            _spClippingAttachment_dispose
                as unsafe extern "C" fn(*mut spAttachment) -> (),
        ),
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
        (::core::mem::size_of::<spColor>() as c_ulong)
            .wrapping_mul(1 as c_int as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        4117 as c_int,
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
pub unsafe extern "C" fn spColor_addColor(
    mut self_0: *mut spColor,
    mut otherColor: *mut spColor,
) {
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
#[no_mangle]
pub unsafe extern "C" fn spEvent_create(
    mut time: c_float,
    mut data: *mut spEventData,
) -> *mut spEvent {
    let mut self_0: *mut spEvent = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spEvent>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        4201 as c_int,
    ) as *mut spEvent;
    let ref mut fresh69 = *(&(*self_0).data as *const *mut spEventData
        as *mut *mut spEventData);
    *fresh69 = data;
    *(&(*self_0).time as *const c_float as *mut c_float) = time;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spEvent_dispose(mut self_0: *mut spEvent) {
    _spFree((*self_0).stringValue as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spEventData_create(
    mut name: *const c_char,
) -> *mut spEventData {
    let mut self_0: *mut spEventData = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spEventData>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        4244 as c_int,
    ) as *mut spEventData;
    let ref mut fresh70 = *(&(*self_0).name as *const *const c_char
        as *mut *mut c_char);
    *fresh70 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul(
                (spine_strlen(name)).wrapping_add(1 as c_int as c_ulong),
            ),
        b"spine.c\0" as *const u8 as *const c_char,
        4245 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh70, name);
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
        4292 as c_int,
    ) as *mut spIkConstraint;
    let ref mut fresh71 = *(&(*self_0).data as *const *mut spIkConstraintData
        as *mut *mut spIkConstraintData);
    *fresh71 = data;
    (*self_0).bendDirection = (*data).bendDirection;
    (*self_0).compress = (*data).compress;
    (*self_0).stretch = (*data).stretch;
    (*self_0).mix = (*data).mix;
    (*self_0).softness = (*data).softness;
    (*self_0).bonesCount = (*(*self_0).data).bonesCount;
    (*self_0)
        .bones = _spMalloc(
        (::core::mem::size_of::<*mut spBone>() as c_ulong)
            .wrapping_mul((*self_0).bonesCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        4301 as c_int,
    ) as *mut *mut spBone;
    i = 0 as c_int;
    while i < (*self_0).bonesCount {
        let ref mut fresh72 = *((*self_0).bones).offset(i as isize);
        *fresh72 = spSkeleton_findBone(
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
pub unsafe extern "C" fn spIkConstraint_apply(mut self_0: *mut spIkConstraint) {
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
    if (*bone).appliedValid == 0 {
        spBone_updateAppliedTransform(bone);
    }
    let mut current_block_13: u64;
    match (*(*bone).data).transformMode as c_uint {
        1 => {
            tx = targetX - (*bone).worldX;
            ty = targetY - (*bone).worldY;
            current_block_13 = 12599329904712511516;
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
            rotationIK
                += atan2f(sc, sa)
                    * (180 as c_int as c_float / 3.1415926535897932385f32);
            current_block_13 = 12618485878637048149;
        }
        _ => {
            current_block_13 = 12618485878637048149;
        }
    }
    match current_block_13 {
        12618485878637048149 => {
            let mut x: c_float = targetX - (*p).worldX;
            let mut y: c_float = targetY - (*p).worldY;
            let mut d: c_float = pa * pd - pb * pc;
            tx = (x * pd - y * pb) / d - (*bone).ax;
            ty = (y * pa - x * pc) / d - (*bone).ay;
        }
        _ => {}
    }
    rotationIK
        += atan2f(ty, tx)
            * (180 as c_int as c_float / 3.1415926535897932385f32);
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
            s = (dd / b - 1 as c_int as c_float) * alpha
                + 1 as c_int as c_float;
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
    let mut sx: c_float = 0.;
    let mut psy: c_float = 0.;
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
    if alpha == 0 as c_int as c_float {
        spBone_updateWorldTransform(child);
        return;
    }
    if (*parent).appliedValid == 0 {
        spBone_updateAppliedTransform(parent);
    }
    if (*child).appliedValid == 0 {
        spBone_updateAppliedTransform(child);
    }
    px = (*parent).ax;
    py = (*parent).ay;
    psx = (*parent).ascaleX;
    sx = psx;
    psy = (*parent).ascaleY;
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
    u = ((if r < 0 as c_int as c_float { -r } else { r }) <= 0.0001f32)
        as c_int;
    if u == 0 {
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
            parent,
            targetX,
            targetY,
            0 as c_int,
            stretch,
            0 as c_int,
            alpha,
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
        softness
            *= psx * (csx + 1 as c_int as c_float)
                / 2 as c_int as c_float;
        td = spine_sqrtf(dd);
        sd = td - l1 - l2 * psx + softness;
        if sd > 0 as c_int as c_float {
            p = (if (1 as c_int as c_float)
                < sd / (softness * 2 as c_int as c_float)
            {
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
        cosine = (dd - l1 * l1 - l2 * l2)
            / (2 as c_int as c_float * l1 * l2);
        if cosine < -(1 as c_int) as c_float {
            cosine = -(1 as c_int) as c_float;
        } else if cosine > 1 as c_int as c_float {
            cosine = 1 as c_int as c_float;
            if stretch != 0 {
                sx
                    *= (spine_sqrtf(dd) / (l1 + l2) - 1 as c_int as c_float)
                        * alpha + 1 as c_int as c_float;
            }
        }
        a2 = acosf(cosine) * bendDir as c_float;
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
            q = -(c1 + q) / 2 as c_int as c_float;
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
                current_block = 4207557037192841971;
            } else {
                current_block = 13723035087248630346;
            }
        } else {
            current_block = 13723035087248630346;
        }
        match current_block {
            4207557037192841971 => {}
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
                if c0 >= -(1 as c_int) as c_float
                    && c0 <= 1 as c_int as c_float
                {
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
                if dd <= (minDist + maxDist) / 2 as c_int as c_float {
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
    a1 = (a1 - os) * (180 as c_int as c_float / 3.1415926535897932385f32)
        + o1 as c_float - (*parent).arotation;
    if a1 > 180 as c_int as c_float {
        a1 -= 360 as c_int as c_float;
    } else if a1 < -(180 as c_int) as c_float {
        a1 += 360 as c_int as c_float;
    }
    spBone_updateWorldTransformWith(
        parent,
        px,
        py,
        (*parent).arotation + a1 * alpha,
        sx,
        (*parent).ascaleY,
        0 as c_int as c_float,
        0 as c_int as c_float,
    );
    a2 = ((a2 + os) * (180 as c_int as c_float / 3.1415926535897932385f32)
        - (*child).ashearX) * s2 as c_float + o2 as c_float
        - (*child).arotation;
    if a2 > 180 as c_int as c_float {
        a2 -= 360 as c_int as c_float;
    } else if a2 < -(180 as c_int) as c_float {
        a2 += 360 as c_int as c_float;
    }
    spBone_updateWorldTransformWith(
        child,
        cx,
        cy,
        (*child).arotation + a2 * alpha,
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
        4564 as c_int,
    ) as *mut spIkConstraintData;
    let ref mut fresh73 = *(&(*self_0).name as *const *const c_char
        as *mut *mut c_char);
    *fresh73 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul(
                (spine_strlen(name)).wrapping_add(1 as c_int as c_ulong),
            ),
        b"spine.c\0" as *const u8 as *const c_char,
        4565 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh73, name);
    (*self_0).bendDirection = 1 as c_int;
    (*self_0).compress = 0 as c_int;
    (*self_0).stretch = 0 as c_int;
    (*self_0).uniform = 0 as c_int;
    (*self_0).mix = 1 as c_int as c_float;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spIkConstraintData_dispose(
    mut self_0: *mut spIkConstraintData,
) {
    _spFree((*self_0).name as *mut c_void);
    _spFree((*self_0).bones as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
static mut ep: *const c_char = 0 as *const c_char;
#[no_mangle]
pub unsafe extern "C" fn Json_getError() -> *const c_char {
    return ep;
}
unsafe extern "C" fn Json_strcasecmp(
    mut s1: *const c_char,
    mut s2: *const c_char,
) -> c_int {
    if !s1.is_null() && !s2.is_null() {
        return spine_strcasecmp(s1, s2)
    } else if s1 < s2 {
        return -(1 as c_int)
    } else if s1 == s2 {
        return 0 as c_int
    } else {
        return 1 as c_int
    };
}
unsafe extern "C" fn Json_new() -> *mut Json {
    return _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<Json>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        4654 as c_int,
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
unsafe extern "C" fn parse_number(
    mut item: *mut Json,
    mut num: *const c_char,
) -> *const c_char {
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
            fraction = fraction * 10.0f64
                + (*ptr as c_int - '0' as i32) as c_double;
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
        // rustc complains about this whether it's initialized or not
        #[allow(unused_variables)]
        let mut n_0: c_int = 0 as c_int;
        ptr = ptr.offset(1);
        if *ptr as c_int == '-' as i32 {
            expNegative = -(1 as c_int);
            ptr = ptr.offset(1);
        } else if *ptr as c_int == '+' as i32 {
            ptr = ptr.offset(1);
        }
        while *ptr as c_int >= '0' as i32 && *ptr as c_int <= '9' as i32 {
            exponent = exponent * 10.0f64
                + (*ptr as c_int - '0' as i32) as c_double;
            ptr = ptr.offset(1);
            n_0 += 1;
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
unsafe extern "C" fn parse_string(
    mut item: *mut Json,
    mut str: *const c_char,
) -> *const c_char {
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
    while *ptr as c_int != '"' as i32 && *ptr as c_int != 0
        && {
            len += 1;
            len != 0
        }
    {
        let fresh74 = ptr;
        ptr = ptr.offset(1);
        if *fresh74 as c_int == '\\' as i32 {
            ptr = ptr.offset(1);
        }
    }
    out = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul((len + 1 as c_int) as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        4754 as c_int,
    ) as *mut c_char;
    if out.is_null() {
        return 0 as *const c_char;
    }
    ptr = str.offset(1 as c_int as isize);
    ptr2 = out;
    while *ptr as c_int != '"' as i32 && *ptr as c_int != 0 {
        if *ptr as c_int != '\\' as i32 {
            let fresh75 = ptr;
            ptr = ptr.offset(1);
            let fresh76 = ptr2;
            ptr2 = ptr2.offset(1);
            *fresh76 = *fresh75;
        } else {
            ptr = ptr.offset(1);
            let mut current_block_41: u64;
            match *ptr as c_int {
                98 => {
                    let fresh77 = ptr2;
                    ptr2 = ptr2.offset(1);
                    *fresh77 = '\u{8}' as i32 as c_char;
                }
                102 => {
                    let fresh78 = ptr2;
                    ptr2 = ptr2.offset(1);
                    *fresh78 = '\u{c}' as i32 as c_char;
                }
                110 => {
                    let fresh79 = ptr2;
                    ptr2 = ptr2.offset(1);
                    *fresh79 = '\n' as i32 as c_char;
                }
                114 => {
                    let fresh80 = ptr2;
                    ptr2 = ptr2.offset(1);
                    *fresh80 = '\r' as i32 as c_char;
                }
                116 => {
                    let fresh81 = ptr2;
                    ptr2 = ptr2.offset(1);
                    *fresh81 = '\t' as i32 as c_char;
                }
                117 => {
                    spine_sscanf!(
                        ptr.offset(1 as c_int as isize),
                        b"%4x\0" as *const u8 as *const c_char,
                        &mut uc as *mut c_uint,
                    );
                    ptr = ptr.offset(4 as c_int as isize);
                    if !(uc >= 0xdc00 as c_int as c_uint
                        && uc <= 0xdfff as c_int as c_uint
                        || uc == 0 as c_int as c_uint)
                    {
                        if uc >= 0xd800 as c_int as c_uint
                            && uc <= 0xdbff as c_int as c_uint
                        {
                            if *ptr.offset(1 as c_int as isize) as c_int
                                != '\\' as i32
                                || *ptr.offset(2 as c_int as isize) as c_int
                                    != 'u' as i32
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
                                    uc = (0x10000 as c_int as c_uint)
                                        .wrapping_add(
                                            (uc & 0x3ff as c_int as c_uint)
                                                << 10 as c_int
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
                                            & 0xbf as c_int as c_uint) as c_char;
                                        uc >>= 6 as c_int;
                                        current_block_38 = 11263121762281864161;
                                    }
                                    3 => {
                                        current_block_38 = 11263121762281864161;
                                    }
                                    2 => {
                                        current_block_38 = 6495237199510159486;
                                    }
                                    1 => {
                                        current_block_38 = 14701241855920314715;
                                    }
                                    _ => {
                                        current_block_38 = 4567019141635105728;
                                    }
                                }
                                match current_block_38 {
                                    11263121762281864161 => {
                                        ptr2 = ptr2.offset(-1);
                                        *ptr2 = ((uc | 0x80 as c_int as c_uint)
                                            & 0xbf as c_int as c_uint) as c_char;
                                        uc >>= 6 as c_int;
                                        current_block_38 = 6495237199510159486;
                                    }
                                    _ => {}
                                }
                                match current_block_38 {
                                    6495237199510159486 => {
                                        ptr2 = ptr2.offset(-1);
                                        *ptr2 = ((uc | 0x80 as c_int as c_uint)
                                            & 0xbf as c_int as c_uint) as c_char;
                                        uc >>= 6 as c_int;
                                        current_block_38 = 14701241855920314715;
                                    }
                                    _ => {}
                                }
                                match current_block_38 {
                                    14701241855920314715 => {
                                        ptr2 = ptr2.offset(-1);
                                        *ptr2 = (uc | firstByteMark[len as usize] as c_uint)
                                            as c_char;
                                    }
                                    _ => {}
                                }
                                ptr2 = ptr2.offset(len as isize);
                            }
                        }
                    }
                }
                _ => {
                    let fresh82 = ptr2;
                    ptr2 = ptr2.offset(1);
                    *fresh82 = *ptr;
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
    while *in_0 as c_int != 0
        && *in_0 as c_uchar as c_int <= 32 as c_int
    {
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
unsafe extern "C" fn parse_value(
    mut item: *mut Json,
    mut value: *const c_char,
) -> *const c_char {
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
unsafe extern "C" fn parse_array(
    mut item: *mut Json,
    mut value: *const c_char,
) -> *const c_char {
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
unsafe extern "C" fn parse_object(
    mut item: *mut Json,
    mut value: *const c_char,
) -> *const c_char {
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
    return if !value.is_null() { (*value).valueFloat } else { defaultValue };
}
#[no_mangle]
pub unsafe extern "C" fn Json_getInt(
    mut value: *mut Json,
    mut name: *const c_char,
    mut defaultValue: c_int,
) -> c_int {
    value = Json_getItem(value, name);
    return if !value.is_null() { (*value).valueInt } else { defaultValue };
}
#[no_mangle]
pub unsafe extern "C" fn _spMeshAttachment_dispose(mut attachment: *mut spAttachment) {
    let mut self_0: *mut spMeshAttachment = attachment as *mut spMeshAttachment;
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
            as unsafe extern "C" fn(
                *mut spMeshAttachment,
            ) -> *mut spMeshAttachment)(self_0))
            .super_0
            .super_0;
    }
    copy = spMeshAttachment_create((*attachment).name);
    (*copy).rendererObject = (*self_0).rendererObject;
    (*copy).regionU = (*self_0).regionU;
    (*copy).regionV = (*self_0).regionV;
    (*copy).regionU2 = (*self_0).regionU2;
    (*copy).regionV2 = (*self_0).regionV2;
    (*copy).regionRotate = (*self_0).regionRotate;
    (*copy).regionDegrees = (*self_0).regionDegrees;
    (*copy).regionOffsetX = (*self_0).regionOffsetX;
    (*copy).regionOffsetY = (*self_0).regionOffsetY;
    (*copy).regionWidth = (*self_0).regionWidth;
    (*copy).regionHeight = (*self_0).regionHeight;
    (*copy).regionOriginalWidth = (*self_0).regionOriginalWidth;
    (*copy).regionOriginalHeight = (*self_0).regionOriginalHeight;
    let ref mut fresh83 = *(&mut (*copy).path as *mut *const c_char
        as *mut *mut c_char);
    *fresh83 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul(
                (spine_strlen((*self_0).path))
                    .wrapping_add(1 as c_int as c_ulong),
            ),
        b"spine.c\0" as *const u8 as *const c_char,
        5106 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh83, (*self_0).path);
    spColor_setFromColor(&mut (*copy).color, &mut (*self_0).color);
    spVertexAttachment_copyTo(&mut (*self_0).super_0, &mut (*copy).super_0);
    (*copy)
        .regionUVs = _spMalloc(
        (::core::mem::size_of::<c_float>() as c_ulong)
            .wrapping_mul((*self_0).super_0.worldVerticesLength as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        5110 as c_int,
    ) as *mut c_float;
    spine_memcpy(
        (*copy).regionUVs as *mut c_void,
        (*self_0).regionUVs as *const c_void,
        ((*self_0).super_0.worldVerticesLength as c_ulong)
            .wrapping_mul(::core::mem::size_of::<c_float>() as c_ulong),
    );
    (*copy)
        .uvs = _spMalloc(
        (::core::mem::size_of::<c_float>() as c_ulong)
            .wrapping_mul((*self_0).super_0.worldVerticesLength as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        5112 as c_int,
    ) as *mut c_float;
    spine_memcpy(
        (*copy).uvs as *mut c_void,
        (*self_0).uvs as *const c_void,
        ((*self_0).super_0.worldVerticesLength as c_ulong)
            .wrapping_mul(::core::mem::size_of::<c_float>() as c_ulong),
    );
    (*copy).trianglesCount = (*self_0).trianglesCount;
    (*copy)
        .triangles = _spMalloc(
        (::core::mem::size_of::<c_ushort>() as c_ulong)
            .wrapping_mul((*self_0).trianglesCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        5115 as c_int,
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
        (*copy)
            .edges = _spMalloc(
            (::core::mem::size_of::<c_int>() as c_ulong)
                .wrapping_mul((*self_0).edgesCount as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            5120 as c_int,
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
    let mut copy: *mut spMeshAttachment = spMeshAttachment_create(
        (*self_0).super_0.super_0.name,
    );
    (*copy).rendererObject = (*self_0).rendererObject;
    (*copy).regionU = (*self_0).regionU;
    (*copy).regionV = (*self_0).regionV;
    (*copy).regionU2 = (*self_0).regionU2;
    (*copy).regionV2 = (*self_0).regionV2;
    (*copy).regionRotate = (*self_0).regionRotate;
    (*copy).regionDegrees = (*self_0).regionDegrees;
    (*copy).regionOffsetX = (*self_0).regionOffsetX;
    (*copy).regionOffsetY = (*self_0).regionOffsetY;
    (*copy).regionWidth = (*self_0).regionWidth;
    (*copy).regionHeight = (*self_0).regionHeight;
    (*copy).regionOriginalWidth = (*self_0).regionOriginalWidth;
    (*copy).regionOriginalHeight = (*self_0).regionOriginalHeight;
    let ref mut fresh84 = *(&mut (*copy).path as *mut *const c_char
        as *mut *mut c_char);
    *fresh84 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul(
                (spine_strlen((*self_0).path))
                    .wrapping_add(1 as c_int as c_ulong),
            ),
        b"spine.c\0" as *const u8 as *const c_char,
        5145 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh84, (*self_0).path);
    spColor_setFromColor(&mut (*copy).color, &mut (*self_0).color);
    (*copy).super_0.deformAttachment = (*self_0).super_0.deformAttachment;
    spMeshAttachment_setParentMesh(
        copy,
        if !((*self_0).parentMesh).is_null() { (*self_0).parentMesh } else { self_0 },
    );
    spMeshAttachment_updateUVs(copy);
    return copy;
}
#[no_mangle]
pub unsafe extern "C" fn spMeshAttachment_create(
    mut name: *const c_char,
) -> *mut spMeshAttachment {
    let mut self_0: *mut spMeshAttachment = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spMeshAttachment>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        5154 as c_int,
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
            _spMeshAttachment_copy
                as unsafe extern "C" fn(*mut spAttachment) -> *mut spAttachment,
        ),
    );
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spMeshAttachment_updateUVs(mut self_0: *mut spMeshAttachment) {
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    let mut uvs: *mut c_float = 0 as *mut c_float;
    let mut u: c_float = 0.;
    let mut v: c_float = 0.;
    let mut width: c_float = 0.;
    let mut height: c_float = 0.;
    let mut verticesLength: c_int = (*self_0).super_0.worldVerticesLength;
    _spFree((*self_0).uvs as *mut c_void);
    (*self_0)
        .uvs = _spMalloc(
        (::core::mem::size_of::<c_float>() as c_ulong)
            .wrapping_mul(verticesLength as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        5167 as c_int,
    ) as *mut c_float;
    uvs = (*self_0).uvs;
    n = verticesLength;
    u = (*self_0).regionU;
    v = (*self_0).regionV;
    match (*self_0).regionDegrees {
        90 => {
            let mut textureWidth: c_float = (*self_0).regionHeight as c_float
                / ((*self_0).regionU2 - (*self_0).regionU);
            let mut textureHeight: c_float = (*self_0).regionWidth as c_float
                / ((*self_0).regionV2 - (*self_0).regionV);
            u
                -= ((*self_0).regionOriginalHeight - (*self_0).regionOffsetY
                    - (*self_0).regionHeight) as c_float / textureWidth;
            v
                -= ((*self_0).regionOriginalWidth - (*self_0).regionOffsetX
                    - (*self_0).regionWidth) as c_float / textureHeight;
            width = (*self_0).regionOriginalHeight as c_float / textureWidth;
            height = (*self_0).regionOriginalWidth as c_float / textureHeight;
            i = 0 as c_int;
            while i < n {
                *uvs
                    .offset(
                        i as isize,
                    ) = u
                    + *((*self_0).regionUVs).offset((i + 1 as c_int) as isize)
                        * width;
                *uvs
                    .offset(
                        (i + 1 as c_int) as isize,
                    ) = v
                    + (1 as c_int as c_float
                        - *((*self_0).regionUVs).offset(i as isize)) * height;
                i += 2 as c_int;
            }
            return;
        }
        180 => {
            let mut textureWidth_0: c_float = (*self_0).regionWidth
                as c_float / ((*self_0).regionU2 - (*self_0).regionU);
            let mut textureHeight_0: c_float = (*self_0).regionHeight
                as c_float / ((*self_0).regionV2 - (*self_0).regionV);
            u
                -= ((*self_0).regionOriginalWidth - (*self_0).regionOffsetX
                    - (*self_0).regionWidth) as c_float / textureWidth_0;
            v -= (*self_0).regionOffsetY as c_float / textureHeight_0;
            width = (*self_0).regionOriginalWidth as c_float / textureWidth_0;
            height = (*self_0).regionOriginalHeight as c_float / textureHeight_0;
            i = 0 as c_int;
            while i < n {
                *uvs
                    .offset(
                        i as isize,
                    ) = u
                    + (1 as c_int as c_float
                        - *((*self_0).regionUVs).offset(i as isize)) * width;
                *uvs
                    .offset(
                        (i + 1 as c_int) as isize,
                    ) = v
                    + (1 as c_int as c_float
                        - *((*self_0).regionUVs).offset((i + 1 as c_int) as isize))
                        * height;
                i += 2 as c_int;
            }
            return;
        }
        270 => {
            let mut textureHeight_1: c_float = (*self_0).regionHeight
                as c_float / ((*self_0).regionV2 - (*self_0).regionV);
            let mut textureWidth_1: c_float = (*self_0).regionWidth
                as c_float / ((*self_0).regionU2 - (*self_0).regionU);
            u -= (*self_0).regionOffsetY as c_float / textureWidth_1;
            v -= (*self_0).regionOffsetX as c_float / textureHeight_1;
            width = (*self_0).regionOriginalHeight as c_float / textureWidth_1;
            height = (*self_0).regionOriginalWidth as c_float / textureHeight_1;
            i = 0 as c_int;
            while i < n {
                *uvs
                    .offset(
                        i as isize,
                    ) = u
                    + (1 as c_int as c_float
                        - *((*self_0).regionUVs).offset((i + 1 as c_int) as isize))
                        * width;
                *uvs
                    .offset(
                        (i + 1 as c_int) as isize,
                    ) = v + *((*self_0).regionUVs).offset(i as isize) * height;
                i += 2 as c_int;
            }
            return;
        }
        _ => {
            let mut textureWidth_2: c_float = (*self_0).regionWidth
                as c_float / ((*self_0).regionU2 - (*self_0).regionU);
            let mut textureHeight_2: c_float = (*self_0).regionHeight
                as c_float / ((*self_0).regionV2 - (*self_0).regionV);
            u -= (*self_0).regionOffsetX as c_float / textureWidth_2;
            v
                -= ((*self_0).regionOriginalHeight - (*self_0).regionOffsetY
                    - (*self_0).regionHeight) as c_float / textureHeight_2;
            width = (*self_0).regionOriginalWidth as c_float / textureWidth_2;
            height = (*self_0).regionOriginalHeight as c_float / textureHeight_2;
            i = 0 as c_int;
            while i < n {
                *uvs
                    .offset(
                        i as isize,
                    ) = u + *((*self_0).regionUVs).offset(i as isize) * width;
                *uvs
                    .offset(
                        (i + 1 as c_int) as isize,
                    ) = v
                    + *((*self_0).regionUVs).offset((i + 1 as c_int) as isize)
                        * height;
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
    let ref mut fresh85 = *(&(*self_0).parentMesh as *const *mut spMeshAttachment
        as *mut *mut spMeshAttachment);
    *fresh85 = parentMesh;
    if !parentMesh.is_null() {
        (*self_0).super_0.bones = (*parentMesh).super_0.bones;
        (*self_0).super_0.bonesCount = (*parentMesh).super_0.bonesCount;
        (*self_0).super_0.vertices = (*parentMesh).super_0.vertices;
        (*self_0).super_0.verticesCount = (*parentMesh).super_0.verticesCount;
        (*self_0).regionUVs = (*parentMesh).regionUVs;
        (*self_0).triangles = (*parentMesh).triangles;
        (*self_0).trianglesCount = (*parentMesh).trianglesCount;
        (*self_0).hullLength = (*parentMesh).hullLength;
        (*self_0)
            .super_0
            .worldVerticesLength = (*parentMesh).super_0.worldVerticesLength;
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
    (*copy)
        .lengths = _spMalloc(
        (::core::mem::size_of::<c_float>() as c_ulong)
            .wrapping_mul((*self_0).lengthsLength as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        5297 as c_int,
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
pub unsafe extern "C" fn spPathAttachment_create(
    mut name: *const c_char,
) -> *mut spPathAttachment {
    let mut self_0: *mut spPathAttachment = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spPathAttachment>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        5305 as c_int,
    ) as *mut spPathAttachment;
    _spVertexAttachment_init(&mut (*self_0).super_0);
    _spAttachment_init(
        &mut (*self_0).super_0.super_0,
        name,
        SP_ATTACHMENT_PATH,
        Some(_spPathAttachment_dispose as unsafe extern "C" fn(*mut spAttachment) -> ()),
        Some(
            _spPathAttachment_copy
                as unsafe extern "C" fn(*mut spAttachment) -> *mut spAttachment,
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
        5350 as c_int,
    ) as *mut spPathConstraint;
    let ref mut fresh86 = *(&(*self_0).data as *const *mut spPathConstraintData
        as *mut *mut spPathConstraintData);
    *fresh86 = data;
    (*self_0).bonesCount = (*data).bonesCount;
    let ref mut fresh87 = *(&(*self_0).bones as *const *mut *mut spBone
        as *mut *mut *mut spBone);
    *fresh87 = _spMalloc(
        (::core::mem::size_of::<*mut spBone>() as c_ulong)
            .wrapping_mul((*self_0).bonesCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        5353 as c_int,
    ) as *mut *mut spBone;
    i = 0 as c_int;
    while i < (*self_0).bonesCount {
        let ref mut fresh88 = *((*self_0).bones).offset(i as isize);
        *fresh88 = spSkeleton_findBone(
            skeleton,
            (**((*(*self_0).data).bones).offset(i as isize)).name,
        );
        i += 1;
    }
    (*self_0).target = spSkeleton_findSlot(skeleton, (*(*(*self_0).data).target).name);
    (*self_0).position = (*data).position;
    (*self_0).spacing = (*data).spacing;
    (*self_0).rotateMix = (*data).rotateMix;
    (*self_0).translateMix = (*data).translateMix;
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
pub unsafe extern "C" fn spPathConstraint_apply(mut self_0: *mut spPathConstraint) {
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
    let mut spaces: *mut c_float = 0 as *mut c_float;
    let mut lengths: *mut c_float = 0 as *mut c_float;
    let mut positions: *mut c_float = 0 as *mut c_float;
    let mut spacing: c_float = 0.;
    let mut boneX: c_float = 0.;
    let mut boneY: c_float = 0.;
    let mut offsetRotation: c_float = 0.;
    let mut tip: c_int = 0;
    let mut rotateMix: c_float = (*self_0).rotateMix;
    let mut translateMix: c_float = (*self_0).translateMix;
    let mut translate: c_int = (translateMix > 0 as c_int as c_float)
        as c_int;
    let mut rotate: c_int = (rotateMix > 0 as c_int as c_float)
        as c_int;
    let mut lengthSpacing: c_int = 0;
    let mut attachment: *mut spPathAttachment = (*(*self_0).target).attachment
        as *mut spPathAttachment;
    let mut data: *mut spPathConstraintData = (*self_0).data;
    let mut percentSpacing: c_int = ((*data).spacingMode as c_uint
        == SP_SPACING_MODE_PERCENT as c_int as c_uint) as c_int;
    let mut rotateMode: spRotateMode = (*data).rotateMode;
    let mut tangents: c_int = (rotateMode as c_uint
        == SP_ROTATE_MODE_TANGENT as c_int as c_uint) as c_int;
    let mut scale: c_int = (rotateMode as c_uint
        == SP_ROTATE_MODE_CHAIN_SCALE as c_int as c_uint) as c_int;
    let mut boneCount: c_int = (*self_0).bonesCount;
    let mut spacesCount: c_int = if tangents != 0 {
        boneCount
    } else {
        boneCount + 1 as c_int
    };
    let mut bones: *mut *mut spBone = (*self_0).bones;
    let mut pa: *mut spBone = 0 as *mut spBone;
    if translate == 0 && rotate == 0 {
        return;
    }
    if attachment.is_null()
        || (*attachment).super_0.super_0.type_0 as c_uint
            != SP_ATTACHMENT_PATH as c_int as c_uint
    {
        return;
    }
    if (*self_0).spacesCount != spacesCount {
        if !((*self_0).spaces).is_null() {
            _spFree((*self_0).spaces as *mut c_void);
        }
        (*self_0)
            .spaces = _spMalloc(
            (::core::mem::size_of::<c_float>() as c_ulong)
                .wrapping_mul(spacesCount as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            5408 as c_int,
        ) as *mut c_float;
        (*self_0).spacesCount = spacesCount;
    }
    spaces = (*self_0).spaces;
    *spaces.offset(0 as c_int as isize) = 0 as c_int as c_float;
    lengths = 0 as *mut c_float;
    spacing = (*self_0).spacing;
    if scale != 0 || percentSpacing == 0 {
        if scale != 0 {
            if (*self_0).lengthsCount != boneCount {
                if !((*self_0).lengths).is_null() {
                    _spFree((*self_0).lengths as *mut c_void);
                }
                (*self_0)
                    .lengths = _spMalloc(
                    (::core::mem::size_of::<c_float>() as c_ulong)
                        .wrapping_mul(boneCount as c_ulong),
                    b"spine.c\0" as *const u8 as *const c_char,
                    5419 as c_int,
                ) as *mut c_float;
                (*self_0).lengthsCount = boneCount;
            }
            lengths = (*self_0).lengths;
        }
        lengthSpacing = ((*data).spacingMode as c_uint
            == SP_SPACING_MODE_LENGTH as c_int as c_uint) as c_int;
        i = 0 as c_int;
        n = spacesCount - 1 as c_int;
        while i < n {
            let mut bone: *mut spBone = *bones.offset(i as isize);
            setupLength = (*(*bone).data).length;
            if setupLength < 0.00001f32 {
                if scale != 0 {
                    *lengths.offset(i as isize) = 0 as c_int as c_float;
                }
                i += 1;
                *spaces.offset(i as isize) = 0 as c_int as c_float;
            } else if percentSpacing != 0 {
                if scale != 0 {
                    x = setupLength * (*bone).a;
                    y = setupLength * (*bone).c;
                    length = spine_sqrtf(x * x + y * y);
                    *lengths.offset(i as isize) = length;
                }
                i += 1;
                *spaces.offset(i as isize) = spacing;
            } else {
                x = setupLength * (*bone).a;
                y = setupLength * (*bone).c;
                length = spine_sqrtf(x * x + y * y);
                if scale != 0 {
                    *lengths.offset(i as isize) = length;
                }
                i += 1;
                *spaces
                    .offset(
                        i as isize,
                    ) = (if lengthSpacing != 0 {
                    setupLength + spacing
                } else {
                    spacing
                }) * length / setupLength;
            }
        }
    } else {
        i = 1 as c_int;
        while i < spacesCount {
            *spaces.offset(i as isize) = spacing;
            i += 1;
        }
    }
    positions = spPathConstraint_computeWorldPositions(
        self_0,
        attachment,
        spacesCount,
        tangents,
        ((*data).positionMode as c_uint
            == SP_POSITION_MODE_PERCENT as c_int as c_uint) as c_int,
        percentSpacing,
    );
    boneX = *positions.offset(0 as c_int as isize);
    boneY = *positions.offset(1 as c_int as isize);
    offsetRotation = (*(*self_0).data).offsetRotation;
    tip = 0 as c_int;
    if offsetRotation == 0 as c_int as c_float {
        tip = (rotateMode as c_uint
            == SP_ROTATE_MODE_CHAIN as c_int as c_uint) as c_int;
    } else {
        tip = 0 as c_int;
        pa = (*(*self_0).target).bone;
        offsetRotation
            *= if (*pa).a * (*pa).d - (*pa).b * (*pa).c
                > 0 as c_int as c_float
            {
                3.1415926535897932385f32 / 180 as c_int as c_float
            } else {
                -(3.1415926535897932385f32 / 180 as c_int as c_float)
            };
    }
    i = 0 as c_int;
    p = 3 as c_int;
    while i < boneCount {
        let mut bone_0: *mut spBone = *bones.offset(i as isize);
        *(&(*bone_0).worldX as *const c_float as *mut c_float)
            += (boneX - (*bone_0).worldX) * translateMix;
        *(&(*bone_0).worldY as *const c_float as *mut c_float)
            += (boneY - (*bone_0).worldY) * translateMix;
        x = *positions.offset(p as isize);
        y = *positions.offset((p + 1 as c_int) as isize);
        dx = x - boneX;
        dy = y - boneY;
        if scale != 0 {
            length = *lengths.offset(i as isize);
            if length != 0 as c_int as c_float {
                s = (spine_sqrtf(dx * dx + dy * dy) / length
                    - 1 as c_int as c_float) * rotateMix
                    + 1 as c_int as c_float;
                *(&(*bone_0).a as *const c_float as *mut c_float) *= s;
                *(&(*bone_0).c as *const c_float as *mut c_float) *= s;
            }
        }
        boneX = x;
        boneY = y;
        if rotate != 0 {
            let mut a: c_float = (*bone_0).a;
            let mut b: c_float = (*bone_0).b;
            let mut c: c_float = (*bone_0).c;
            let mut d: c_float = (*bone_0).d;
            let mut r: c_float = 0.;
            let mut cosine: c_float = 0.;
            let mut sine: c_float = 0.;
            if tangents != 0 {
                r = *positions.offset((p - 1 as c_int) as isize);
            } else if *spaces.offset((i + 1 as c_int) as isize)
                    == 0 as c_int as c_float
                {
                r = *positions.offset((p + 2 as c_int) as isize);
            } else {
                r = atan2f(dy, dx);
            }
            r
                -= atan2f(c, a)
                    - offsetRotation
                        * (3.1415926535897932385f32
                            / 180 as c_int as c_float);
            if tip != 0 {
                cosine = cosf(r);
                sine = sinf(r);
                length = (*(*bone_0).data).length;
                boneX += (length * (cosine * a - sine * c) - dx) * rotateMix;
                boneY += (length * (sine * a + cosine * c) - dy) * rotateMix;
            } else {
                r += offsetRotation;
            }
            if r > 3.1415926535897932385f32 {
                r -= 3.1415926535897932385f32 * 2 as c_int as c_float;
            } else if r < -3.1415926535897932385f32 {
                r += 3.1415926535897932385f32 * 2 as c_int as c_float;
            }
            r *= rotateMix;
            cosine = cosf(r);
            sine = sinf(r);
            *(&(*bone_0).a as *const c_float
                as *mut c_float) = cosine * a - sine * c;
            *(&(*bone_0).b as *const c_float
                as *mut c_float) = cosine * b - sine * d;
            *(&(*bone_0).c as *const c_float
                as *mut c_float) = sine * a + cosine * c;
            *(&(*bone_0).d as *const c_float
                as *mut c_float) = sine * b + cosine * d;
        }
        *(&mut (*bone_0).appliedValid as *mut c_int) = -(1 as c_int);
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
            *out
                .offset(
                    (o + 2 as c_int) as isize,
                ) = atan2f(
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
    mut percentPosition: c_int,
    mut percentSpacing: c_int,
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
    let mut target: *mut spSlot = (*self_0).target;
    let mut position: c_float = (*self_0).position;
    let mut spaces: *mut c_float = (*self_0).spaces;
    let mut world: *mut c_float = 0 as *mut c_float;
    if (*self_0).positionsCount != spacesCount * 3 as c_int + 2 as c_int {
        if !((*self_0).positions).is_null() {
            _spFree((*self_0).positions as *mut c_void);
        }
        (*self_0)
            .positions = _spMalloc(
            (::core::mem::size_of::<c_float>() as c_ulong)
                .wrapping_mul(
                    (spacesCount * 3 as c_int + 2 as c_int) as c_ulong,
                ),
            b"spine.c\0" as *const u8 as *const c_char,
            5558 as c_int,
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
        if percentPosition != 0 {
            position *= pathLength;
        }
        if percentSpacing != 0 {
            i = 1 as c_int;
            while i < spacesCount {
                *spaces.offset(i as isize) *= pathLength;
                i += 1;
            }
        }
        if (*self_0).worldCount != 8 as c_int {
            if !((*self_0).world).is_null() {
                _spFree((*self_0).world as *mut c_void);
            }
            (*self_0)
                .world = _spMalloc(
                (::core::mem::size_of::<c_float>() as c_ulong)
                    .wrapping_mul(8 as c_int as c_ulong),
                b"spine.c\0" as *const u8 as *const c_char,
                5576 as c_int,
            ) as *mut c_float;
            (*self_0).worldCount = 8 as c_int;
        }
        world = (*self_0).world;
        let mut current_block_56: u64;
        i = 0 as c_int;
        o = 0 as c_int;
        curve = 0 as c_int;
        while i < spacesCount {
            let mut space: c_float = *spaces.offset(i as isize);
            position += space;
            p = position;
            if closed != 0 {
                p = fmodf(p, pathLength);
                if p < 0 as c_int as c_float {
                    p += pathLength;
                }
                curve = 0 as c_int;
                current_block_56 = 7427571413727699167;
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
                current_block_56 = 11307063007268554308;
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
                current_block_56 = 11307063007268554308;
            } else {
                current_block_56 = 7427571413727699167;
            }
            match current_block_56 {
                7427571413727699167 => {
                    loop {
                        let mut length: c_float = *lengths.offset(curve as isize);
                        if p > length {
                            curve += 1;
                        } else {
                            if curve == 0 as c_int {
                                p /= length;
                            } else {
                                let mut prev: c_float = *lengths
                                    .offset((curve - 1 as c_int) as isize);
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
                        (tangents != 0
                            || i > 0 as c_int
                                && space == 0 as c_int as c_float)
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
            (*self_0)
                .world = _spMalloc(
                (::core::mem::size_of::<c_float>() as c_ulong)
                    .wrapping_mul(verticesLength as c_ulong),
                b"spine.c\0" as *const u8 as *const c_char,
                5636 as c_int,
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
        *world
            .offset(
                (verticesLength - 2 as c_int) as isize,
            ) = *world.offset(0 as c_int as isize);
        *world
            .offset(
                (verticesLength - 1 as c_int) as isize,
            ) = *world.offset(1 as c_int as isize);
    } else {
        curveCount -= 1;
        verticesLength -= 4 as c_int;
        if (*self_0).worldCount != verticesLength {
            if !((*self_0).world).is_null() {
                _spFree((*self_0).world as *mut c_void);
            }
            (*self_0)
                .world = _spMalloc(
                (::core::mem::size_of::<c_float>() as c_ulong)
                    .wrapping_mul(verticesLength as c_ulong),
                b"spine.c\0" as *const u8 as *const c_char,
                5649 as c_int,
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
        (*self_0)
            .curves = _spMalloc(
            (::core::mem::size_of::<c_float>() as c_ulong)
                .wrapping_mul(curveCount as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            5659 as c_int,
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
    if percentPosition != 0 {
        position *= pathLength;
    } else {
        position
            *= pathLength
                / *((*path).lengths).offset((curveCount - 1 as c_int) as isize);
    }
    if percentSpacing != 0 {
        i = 1 as c_int;
        while i < spacesCount {
            *spaces.offset(i as isize) *= pathLength;
            i += 1;
        }
    }
    segments = ((*self_0).segments).as_mut_ptr();
    curveLength = 0 as c_int as c_float;
    let mut current_block_198: u64;
    i = 0 as c_int;
    o = 0 as c_int;
    curve = 0 as c_int;
    segment = 0 as c_int;
    while i < spacesCount {
        let mut space_0: c_float = *spaces.offset(i as isize);
        position += space_0;
        p = position;
        if closed != 0 {
            p = fmodf(p, pathLength);
            if p < 0 as c_int as c_float {
                p += pathLength;
            }
            curve = 0 as c_int;
            current_block_198 = 1069630499025798221;
        } else if p < 0 as c_int as c_float {
            _addBeforePosition(p, world, 0 as c_int, out, o);
            current_block_198 = 6733407218104445560;
        } else if p > pathLength {
            _addAfterPosition(
                p - pathLength,
                world,
                verticesLength - 4 as c_int,
                out,
                o,
            );
            current_block_198 = 6733407218104445560;
        } else {
            current_block_198 = 1069630499025798221;
        }
        match current_block_198 {
            1069630499025798221 => {
                loop {
                    let mut length_0: c_float = *curves.offset(curve as isize);
                    if p > length_0 {
                        curve += 1;
                    } else {
                        if curve == 0 as c_int {
                            p /= length_0;
                        } else {
                            let mut prev_0: c_float = *curves
                                .offset((curve - 1 as c_int) as isize);
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
                    tmpx = (x1 - cx1 * 2 as c_int as c_float + cx2)
                        * 0.03f32;
                    tmpy = (y1 - cy1 * 2 as c_int as c_float + cy2)
                        * 0.03f32;
                    dddfx = ((cx1 - cx2) * 3 as c_int as c_float - x1 + x2)
                        * 0.006f32;
                    dddfy = ((cy1 - cy2) * 3 as c_int as c_float - y1 + y2)
                        * 0.006f32;
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
                            let mut prev_1: c_float = *segments
                                .offset((segment - 1 as c_int) as isize);
                            p = segment as c_float
                                + (p - prev_1) / (length_1 - prev_1);
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
                    (tangents != 0
                        || i > 0 as c_int
                            && space_0 == 0 as c_int as c_float)
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
#[no_mangle]
pub unsafe extern "C" fn spPathConstraintData_create(
    mut name: *const c_char,
) -> *mut spPathConstraintData {
    let mut self_0: *mut spPathConstraintData = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spPathConstraintData>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        5829 as c_int,
    ) as *mut spPathConstraintData;
    let ref mut fresh89 = *(&(*self_0).name as *const *const c_char
        as *mut *mut c_char);
    *fresh89 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul(
                (spine_strlen(name)).wrapping_add(1 as c_int as c_ulong),
            ),
        b"spine.c\0" as *const u8 as *const c_char,
        5830 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh89, name);
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spPathConstraintData_dispose(
    mut self_0: *mut spPathConstraintData,
) {
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
        5888 as c_int,
    ) as *mut spPointAttachment;
    _spAttachment_init(
        &mut (*self_0).super_0,
        name,
        SP_ATTACHMENT_POINT,
        Some(
            _spPointAttachment_dispose as unsafe extern "C" fn(*mut spAttachment) -> (),
        ),
        Some(
            _spPointAttachment_copy
                as unsafe extern "C" fn(*mut spAttachment) -> *mut spAttachment,
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
    cosine = cosf(
        (*self_0).rotation
            * (3.1415926535897932385f32 / 180 as c_int as c_float),
    );
    sine = sinf(
        (*self_0).rotation
            * (3.1415926535897932385f32 / 180 as c_int as c_float),
    );
    x = cosine * (*bone).a + sine * (*bone).b;
    y = cosine * (*bone).c + sine * (*bone).d;
    return atan2f(y, x)
        * (180 as c_int as c_float / 3.1415926535897932385f32);
}
#[no_mangle]
pub unsafe extern "C" fn _spRegionAttachment_dispose(mut attachment: *mut spAttachment) {
    let mut self_0: *mut spRegionAttachment = attachment as *mut spRegionAttachment;
    _spAttachment_deinit(attachment);
    _spFree((*self_0).path as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _spRegionAttachment_copy(
    mut attachment: *mut spAttachment,
) -> *mut spAttachment {
    let mut self_0: *mut spRegionAttachment = attachment as *mut spRegionAttachment;
    let mut copy: *mut spRegionAttachment = spRegionAttachment_create(
        (*attachment).name,
    );
    (*copy).regionWidth = (*self_0).regionWidth;
    (*copy).regionHeight = (*self_0).regionHeight;
    (*copy).regionOffsetX = (*self_0).regionOffsetX;
    (*copy).regionOffsetY = (*self_0).regionOffsetY;
    (*copy).regionOriginalWidth = (*self_0).regionOriginalWidth;
    (*copy).regionOriginalHeight = (*self_0).regionOriginalHeight;
    (*copy).rendererObject = (*self_0).rendererObject;
    let ref mut fresh90 = *(&mut (*copy).path as *mut *const c_char
        as *mut *mut c_char);
    *fresh90 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul(
                (spine_strlen((*self_0).path))
                    .wrapping_add(1 as c_int as c_ulong),
            ),
        b"spine.c\0" as *const u8 as *const c_char,
        5959 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh90, (*self_0).path);
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
        (::core::mem::size_of::<c_float>() as c_ulong)
            .wrapping_mul(8 as c_int as c_ulong),
    );
    spine_memcpy(
        ((*copy).offset).as_mut_ptr() as *mut c_void,
        ((*self_0).offset).as_mut_ptr() as *const c_void,
        (::core::mem::size_of::<c_float>() as c_ulong)
            .wrapping_mul(8 as c_int as c_ulong),
    );
    spColor_setFromColor(&mut (*copy).color, &mut (*self_0).color);
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
        5974 as c_int,
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
        Some(
            _spRegionAttachment_dispose as unsafe extern "C" fn(*mut spAttachment) -> (),
        ),
        Some(
            _spRegionAttachment_copy
                as unsafe extern "C" fn(*mut spAttachment) -> *mut spAttachment,
        ),
    );
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spRegionAttachment_setUVs(
    mut self_0: *mut spRegionAttachment,
    mut u: c_float,
    mut v: c_float,
    mut u2: c_float,
    mut v2: c_float,
    mut rotate: c_int,
) {
    if rotate != 0 {
        (*self_0).uvs[URX as c_int as usize] = u;
        (*self_0).uvs[URY as c_int as usize] = v2;
        (*self_0).uvs[BRX as c_int as usize] = u;
        (*self_0).uvs[BRY as c_int as usize] = v;
        (*self_0).uvs[BLX as c_int as usize] = u2;
        (*self_0).uvs[BLY as c_int as usize] = v;
        (*self_0).uvs[ULX as c_int as usize] = u2;
        (*self_0).uvs[ULY as c_int as usize] = v2;
    } else {
        (*self_0).uvs[ULX as c_int as usize] = u;
        (*self_0).uvs[ULY as c_int as usize] = v2;
        (*self_0).uvs[URX as c_int as usize] = u;
        (*self_0).uvs[URY as c_int as usize] = v;
        (*self_0).uvs[BRX as c_int as usize] = u2;
        (*self_0).uvs[BRY as c_int as usize] = v;
        (*self_0).uvs[BLX as c_int as usize] = u2;
        (*self_0).uvs[BLY as c_int as usize] = v2;
    };
}
#[no_mangle]
pub unsafe extern "C" fn spRegionAttachment_updateOffset(
    mut self_0: *mut spRegionAttachment,
) {
    let mut regionScaleX: c_float = (*self_0).width
        / (*self_0).regionOriginalWidth as c_float * (*self_0).scaleX;
    let mut regionScaleY: c_float = (*self_0).height
        / (*self_0).regionOriginalHeight as c_float * (*self_0).scaleY;
    let mut localX: c_float = -(*self_0).width / 2 as c_int as c_float
        * (*self_0).scaleX + (*self_0).regionOffsetX as c_float * regionScaleX;
    let mut localY: c_float = -(*self_0).height / 2 as c_int as c_float
        * (*self_0).scaleY + (*self_0).regionOffsetY as c_float * regionScaleY;
    let mut localX2: c_float = localX
        + (*self_0).regionWidth as c_float * regionScaleX;
    let mut localY2: c_float = localY
        + (*self_0).regionHeight as c_float * regionScaleY;
    let mut radians: c_float = (*self_0).rotation
        * (3.1415926535897932385f32 / 180 as c_int as c_float);
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
}
#[no_mangle]
pub unsafe extern "C" fn spRegionAttachment_computeWorldVertices(
    mut self_0: *mut spRegionAttachment,
    mut bone: *mut spBone,
    mut vertices: *mut c_float,
    mut offset: c_int,
    mut stride: c_int,
) {
    let mut offsets: *const c_float = ((*self_0).offset).as_mut_ptr();
    let mut x: c_float = (*bone).worldX;
    let mut y: c_float = (*bone).worldY;
    let mut offsetX: c_float = 0.;
    let mut offsetY: c_float = 0.;
    offsetX = *offsets.offset(BRX as c_int as isize);
    offsetY = *offsets.offset(BRY as c_int as isize);
    *vertices.offset(offset as isize) = offsetX * (*bone).a + offsetY * (*bone).b + x;
    *vertices
        .offset(
            (offset + 1 as c_int) as isize,
        ) = offsetX * (*bone).c + offsetY * (*bone).d + y;
    offset += stride;
    offsetX = *offsets.offset(BLX as c_int as isize);
    offsetY = *offsets.offset(BLY as c_int as isize);
    *vertices.offset(offset as isize) = offsetX * (*bone).a + offsetY * (*bone).b + x;
    *vertices
        .offset(
            (offset + 1 as c_int) as isize,
        ) = offsetX * (*bone).c + offsetY * (*bone).d + y;
    offset += stride;
    offsetX = *offsets.offset(ULX as c_int as isize);
    offsetY = *offsets.offset(ULY as c_int as isize);
    *vertices.offset(offset as isize) = offsetX * (*bone).a + offsetY * (*bone).b + x;
    *vertices
        .offset(
            (offset + 1 as c_int) as isize,
        ) = offsetX * (*bone).c + offsetY * (*bone).d + y;
    offset += stride;
    offsetX = *offsets.offset(URX as c_int as isize);
    offsetY = *offsets.offset(URY as c_int as isize);
    *vertices.offset(offset as isize) = offsetX * (*bone).a + offsetY * (*bone).b + x;
    *vertices
        .offset(
            (offset + 1 as c_int) as isize,
        ) = offsetX * (*bone).c + offsetY * (*bone).d + y;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeleton_create(
    mut data: *mut spSkeletonData,
) -> *mut spSkeleton {
    let mut i: c_int = 0;
    let mut childrenCounts: *mut c_int = 0 as *mut c_int;
    let mut internal: *mut _spSkeleton = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<_spSkeleton>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        6118 as c_int,
    ) as *mut _spSkeleton;
    let mut self_0: *mut spSkeleton = &mut (*internal).super_0;
    let ref mut fresh91 = *(&(*self_0).data as *const *mut spSkeletonData
        as *mut *mut spSkeletonData);
    *fresh91 = data;
    (*self_0).bonesCount = (*(*self_0).data).bonesCount;
    (*self_0)
        .bones = _spMalloc(
        (::core::mem::size_of::<*mut spBone>() as c_ulong)
            .wrapping_mul((*self_0).bonesCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        6123 as c_int,
    ) as *mut *mut spBone;
    childrenCounts = _spCalloc(
        (*self_0).bonesCount as size_t,
        ::core::mem::size_of::<c_int>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        6124 as c_int,
    ) as *mut c_int;
    i = 0 as c_int;
    while i < (*self_0).bonesCount {
        let mut boneData: *mut spBoneData = *((*(*self_0).data).bones)
            .offset(i as isize);
        let mut newBone: *mut spBone = 0 as *mut spBone;
        if ((*boneData).parent).is_null() {
            newBone = spBone_create(boneData, self_0, 0 as *mut spBone);
        } else {
            let mut parent: *mut spBone = *((*self_0).bones)
                .offset((*(*boneData).parent).index as isize);
            newBone = spBone_create(boneData, self_0, parent);
            let ref mut fresh92 = *childrenCounts
                .offset((*(*boneData).parent).index as isize);
            *fresh92 += 1;
        }
        let ref mut fresh93 = *((*self_0).bones).offset(i as isize);
        *fresh93 = newBone;
        i += 1;
    }
    i = 0 as c_int;
    while i < (*self_0).bonesCount {
        let mut boneData_0: *mut spBoneData = *((*(*self_0).data).bones)
            .offset(i as isize);
        let mut bone: *mut spBone = *((*self_0).bones).offset(i as isize);
        let ref mut fresh94 = *(&(*bone).children as *const *mut *mut spBone
            as *mut *mut *mut spBone);
        *fresh94 = _spMalloc(
            (::core::mem::size_of::<*mut spBone>() as c_ulong)
                .wrapping_mul(
                    *childrenCounts.offset((*boneData_0).index as isize) as c_ulong,
                ),
            b"spine.c\0" as *const u8 as *const c_char,
            6141 as c_int,
        ) as *mut *mut spBone;
        i += 1;
    }
    i = 0 as c_int;
    while i < (*self_0).bonesCount {
        let mut bone_0: *mut spBone = *((*self_0).bones).offset(i as isize);
        let mut parent_0: *mut spBone = (*bone_0).parent;
        if !parent_0.is_null() {
            let fresh95 = (*parent_0).childrenCount;
            (*parent_0).childrenCount = (*parent_0).childrenCount + 1;
            let ref mut fresh96 = *((*parent_0).children).offset(fresh95 as isize);
            *fresh96 = bone_0;
        }
        i += 1;
    }
    let ref mut fresh97 = *(&(*self_0).root as *const *mut spBone as *mut *mut spBone);
    *fresh97 = if (*self_0).bonesCount > 0 as c_int {
        *((*self_0).bones).offset(0 as c_int as isize)
    } else {
        0 as *mut spBone
    };
    (*self_0).slotsCount = (*data).slotsCount;
    (*self_0)
        .slots = _spMalloc(
        (::core::mem::size_of::<*mut spSlot>() as c_ulong)
            .wrapping_mul((*self_0).slotsCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        6152 as c_int,
    ) as *mut *mut spSlot;
    i = 0 as c_int;
    while i < (*self_0).slotsCount {
        let mut slotData: *mut spSlotData = *((*data).slots).offset(i as isize);
        let mut bone_1: *mut spBone = *((*self_0).bones)
            .offset((*(*slotData).boneData).index as isize);
        let ref mut fresh98 = *((*self_0).slots).offset(i as isize);
        *fresh98 = spSlot_create(slotData, bone_1);
        i += 1;
    }
    (*self_0)
        .drawOrder = _spMalloc(
        (::core::mem::size_of::<*mut spSlot>() as c_ulong)
            .wrapping_mul((*self_0).slotsCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        6159 as c_int,
    ) as *mut *mut spSlot;
    spine_memcpy(
        (*self_0).drawOrder as *mut c_void,
        (*self_0).slots as *const c_void,
        (::core::mem::size_of::<*mut spSlot>() as c_ulong)
            .wrapping_mul((*self_0).slotsCount as c_ulong),
    );
    (*self_0).ikConstraintsCount = (*data).ikConstraintsCount;
    (*self_0)
        .ikConstraints = _spMalloc(
        (::core::mem::size_of::<*mut spIkConstraint>() as c_ulong)
            .wrapping_mul((*self_0).ikConstraintsCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        6163 as c_int,
    ) as *mut *mut spIkConstraint;
    i = 0 as c_int;
    while i < (*(*self_0).data).ikConstraintsCount {
        let ref mut fresh99 = *((*self_0).ikConstraints).offset(i as isize);
        *fresh99 = spIkConstraint_create(
            *((*(*self_0).data).ikConstraints).offset(i as isize),
            self_0,
        );
        i += 1;
    }
    (*self_0).transformConstraintsCount = (*data).transformConstraintsCount;
    (*self_0)
        .transformConstraints = _spMalloc(
        (::core::mem::size_of::<*mut spTransformConstraint>() as c_ulong)
            .wrapping_mul((*self_0).transformConstraintsCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        6168 as c_int,
    ) as *mut *mut spTransformConstraint;
    i = 0 as c_int;
    while i < (*(*self_0).data).transformConstraintsCount {
        let ref mut fresh100 = *((*self_0).transformConstraints).offset(i as isize);
        *fresh100 = spTransformConstraint_create(
            *((*(*self_0).data).transformConstraints).offset(i as isize),
            self_0,
        );
        i += 1;
    }
    (*self_0).pathConstraintsCount = (*data).pathConstraintsCount;
    (*self_0)
        .pathConstraints = _spMalloc(
        (::core::mem::size_of::<*mut spPathConstraint>() as c_ulong)
            .wrapping_mul((*self_0).pathConstraintsCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        6173 as c_int,
    ) as *mut *mut spPathConstraint;
    i = 0 as c_int;
    while i < (*(*self_0).data).pathConstraintsCount {
        let ref mut fresh101 = *((*self_0).pathConstraints).offset(i as isize);
        *fresh101 = spPathConstraint_create(
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
    _spFree((*internal).updateCacheReset as *mut c_void);
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
        spTransformConstraint_dispose(
            *((*self_0).transformConstraints).offset(i as isize),
        );
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
        (*internal)
            .updateCache = spine_realloc(
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
unsafe extern "C" fn _addToUpdateCacheReset(
    internal: *mut _spSkeleton,
    mut bone: *mut spBone,
) {
    if (*internal).updateCacheResetCount == (*internal).updateCacheResetCapacity {
        (*internal).updateCacheResetCapacity *= 2 as c_int;
        (*internal)
            .updateCacheReset = spine_realloc(
            (*internal).updateCacheReset as *mut c_void,
            (::core::mem::size_of::<*mut spBone>() as c_ulong)
                .wrapping_mul((*internal).updateCacheResetCapacity as c_ulong),
        ) as *mut *mut spBone;
    }
    let ref mut fresh102 = *((*internal).updateCacheReset)
        .offset((*internal).updateCacheResetCount as isize);
    *fresh102 = bone;
    (*internal).updateCacheResetCount += 1;
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
    if (*pathAttachment).super_0.super_0.type_0 as c_uint
        != SP_ATTACHMENT_PATH as c_int as c_uint
    {
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
        while i < pathBonesCount {
            let fresh103 = i;
            i = i + 1;
            let mut boneCount: c_int = *pathBones.offset(fresh103 as isize);
            n = i + boneCount;
            while i < n {
                _sortBone(
                    internal,
                    *bones.offset(*pathBones.offset(i as isize) as isize),
                );
                i += 1;
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
unsafe extern "C" fn _sortReset(
    mut bones: *mut *mut spBone,
    mut bonesCount: c_int,
) {
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
    let mut contains: c_int = 0 as c_int;
    let mut i: c_int = 0;
    let mut target: *mut spBone = (*constraint).target;
    let mut constrained: *mut *mut spBone = 0 as *mut *mut spBone;
    let mut parent: *mut spBone = 0 as *mut spBone;
    (*constraint)
        .active = ((*(*constraint).target).active != 0
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
    if (*constraint).bonesCount > 1 as c_int {
        let mut child: *mut spBone = *constrained
            .offset(((*constraint).bonesCount - 1 as c_int) as isize);
        contains = 0 as c_int;
        i = 0 as c_int;
        while i < (*internal).updateCacheCount {
            let mut update: _spUpdate = *((*internal).updateCache).offset(i as isize);
            if update.object == child as *mut c_void {
                contains = -(1 as c_int);
                break;
            } else {
                i += 1;
            }
        }
        if contains == 0 {
            _addToUpdateCacheReset(internal, child);
        }
    }
    _addToUpdateCache(
        internal,
        SP_UPDATE_IK_CONSTRAINT,
        constraint as *mut c_void,
    );
    _sortReset((*parent).children, (*parent).childrenCount);
    (**constrained.offset(((*constraint).bonesCount - 1 as c_int) as isize))
        .sorted = 1 as c_int;
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
    (*constraint)
        .active = ((*(*(*constraint).target).bone).active != 0
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
        && (*attachment).type_0 as c_uint
            == SP_ATTACHMENT_PATH as c_int as c_uint
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
    let mut contains: c_int = 0 as c_int;
    (*constraint)
        .active = ((*(*constraint).target).active != 0
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
            _sortBone(internal, child);
            contains = 0 as c_int;
            i = 0 as c_int;
            while i < (*internal).updateCacheCount {
                let mut update: _spUpdate = *((*internal).updateCache)
                    .offset(i as isize);
                if update.object == child as *mut c_void {
                    contains = -(1 as c_int);
                    break;
                } else {
                    i += 1;
                }
            }
            if contains == 0 {
                _addToUpdateCacheReset(internal, child);
            }
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
    let mut pathConstraints: *mut *mut spPathConstraint = 0
        as *mut *mut spPathConstraint;
    let mut transformConstraints: *mut *mut spTransformConstraint = 0
        as *mut *mut spTransformConstraint;
    let mut ikCount: c_int = 0;
    let mut transformCount: c_int = 0;
    let mut pathCount: c_int = 0;
    let mut constraintCount: c_int = 0;
    let mut internal: *mut _spSkeleton = self_0 as *mut _spSkeleton;
    (*internal)
        .updateCacheCapacity = (*self_0).bonesCount + (*self_0).ikConstraintsCount
        + (*self_0).transformConstraintsCount + (*self_0).pathConstraintsCount;
    _spFree((*internal).updateCache as *mut c_void);
    (*internal)
        .updateCache = _spMalloc(
        (::core::mem::size_of::<_spUpdate>() as c_ulong)
            .wrapping_mul((*internal).updateCacheCapacity as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        6406 as c_int,
    ) as *mut _spUpdate;
    (*internal).updateCacheCount = 0 as c_int;
    (*internal).updateCacheResetCapacity = (*self_0).bonesCount;
    _spFree((*internal).updateCacheReset as *mut c_void);
    (*internal)
        .updateCacheReset = _spMalloc(
        (::core::mem::size_of::<*mut spBone>() as c_ulong)
            .wrapping_mul((*internal).updateCacheResetCapacity as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        6411 as c_int,
    ) as *mut *mut spBone;
    (*internal).updateCacheResetCount = 0 as c_int;
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
    's_137: while i < constraintCount {
        ii = 0 as c_int;
        while ii < ikCount {
            let mut ikConstraint: *mut spIkConstraint = *ikConstraints
                .offset(ii as isize);
            if (*(*ikConstraint).data).order == i {
                _sortIkConstraint(internal, ikConstraint);
                i += 1;
                continue 's_137;
            } else {
                ii += 1;
            }
        }
        ii = 0 as c_int;
        while ii < transformCount {
            let mut transformConstraint: *mut spTransformConstraint = *transformConstraints
                .offset(ii as isize);
            if (*(*transformConstraint).data).order == i {
                _sortTransformConstraint(internal, transformConstraint);
                i += 1;
                continue 's_137;
            } else {
                ii += 1;
            }
        }
        ii = 0 as c_int;
        while ii < pathCount {
            let mut pathConstraint: *mut spPathConstraint = *pathConstraints
                .offset(ii as isize);
            if (*(*pathConstraint).data).order == i {
                _sortPathConstraint(internal, pathConstraint);
                i += 1;
                continue 's_137;
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
    let mut internal: *mut _spSkeleton = self_0 as *mut _spSkeleton;
    let mut updateCacheReset: *mut *mut spBone = (*internal).updateCacheReset;
    i = 0 as c_int;
    while i < (*internal).updateCacheResetCount {
        let mut bone: *mut spBone = *updateCacheReset.offset(i as isize);
        *(&mut (*bone).ax as *mut c_float) = (*bone).x;
        *(&mut (*bone).ay as *mut c_float) = (*bone).y;
        *(&mut (*bone).arotation as *mut c_float) = (*bone).rotation;
        *(&mut (*bone).ascaleX as *mut c_float) = (*bone).scaleX;
        *(&mut (*bone).ascaleY as *mut c_float) = (*bone).scaleY;
        *(&mut (*bone).ashearX as *mut c_float) = (*bone).shearX;
        *(&mut (*bone).ashearY as *mut c_float) = (*bone).shearY;
        *(&mut (*bone).appliedValid as *mut c_int) = 1 as c_int;
        i += 1;
    }
    i = 0 as c_int;
    while i < (*internal).updateCacheCount {
        let mut update: *mut _spUpdate = ((*internal).updateCache).offset(i as isize);
        match (*update).type_0 as c_uint {
            0 => {
                spBone_updateWorldTransform((*update).object as *mut spBone);
            }
            1 => {
                spIkConstraint_apply((*update).object as *mut spIkConstraint);
            }
            3 => {
                spTransformConstraint_apply(
                    (*update).object as *mut spTransformConstraint,
                );
            }
            2 => {
                spPathConstraint_apply((*update).object as *mut spPathConstraint);
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
        let mut ikConstraint: *mut spIkConstraint = *((*self_0).ikConstraints)
            .offset(i as isize);
        (*ikConstraint).bendDirection = (*(*ikConstraint).data).bendDirection;
        (*ikConstraint).compress = (*(*ikConstraint).data).compress;
        (*ikConstraint).stretch = (*(*ikConstraint).data).stretch;
        (*ikConstraint).softness = (*(*ikConstraint).data).softness;
        (*ikConstraint).mix = (*(*ikConstraint).data).mix;
        i += 1;
    }
    i = 0 as c_int;
    while i < (*self_0).transformConstraintsCount {
        let mut constraint: *mut spTransformConstraint = *((*self_0)
            .transformConstraints)
            .offset(i as isize);
        let mut data: *mut spTransformConstraintData = (*constraint).data;
        (*constraint).rotateMix = (*data).rotateMix;
        (*constraint).translateMix = (*data).translateMix;
        (*constraint).scaleMix = (*data).scaleMix;
        (*constraint).shearMix = (*data).shearMix;
        i += 1;
    }
    i = 0 as c_int;
    while i < (*self_0).pathConstraintsCount {
        let mut constraint_0: *mut spPathConstraint = *((*self_0).pathConstraints)
            .offset(i as isize);
        let mut data_0: *mut spPathConstraintData = (*constraint_0).data;
        (*constraint_0).position = (*data_0).position;
        (*constraint_0).spacing = (*data_0).spacing;
        (*constraint_0).rotateMix = (*data_0).rotateMix;
        (*constraint_0).translateMix = (*data_0).translateMix;
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
        if spine_strcmp((**((*(*self_0).data).bones).offset(i as isize)).name, boneName)
            == 0 as c_int
        {
            return *((*self_0).bones).offset(i as isize);
        }
        i += 1;
    }
    return 0 as *mut spBone;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeleton_findBoneIndex(
    mut self_0: *const spSkeleton,
    mut boneName: *const c_char,
) -> c_int {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*self_0).bonesCount {
        if spine_strcmp((**((*(*self_0).data).bones).offset(i as isize)).name, boneName)
            == 0 as c_int
        {
            return i;
        }
        i += 1;
    }
    return -(1 as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn spSkeleton_findSlot(
    mut self_0: *const spSkeleton,
    mut slotName: *const c_char,
) -> *mut spSlot {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*self_0).slotsCount {
        if spine_strcmp((**((*(*self_0).data).slots).offset(i as isize)).name, slotName)
            == 0 as c_int
        {
            return *((*self_0).slots).offset(i as isize);
        }
        i += 1;
    }
    return 0 as *mut spSlot;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeleton_findSlotIndex(
    mut self_0: *const spSkeleton,
    mut slotName: *const c_char,
) -> c_int {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*self_0).slotsCount {
        if spine_strcmp((**((*(*self_0).data).slots).offset(i as isize)).name, slotName)
            == 0 as c_int
        {
            return i;
        }
        i += 1;
    }
    return -(1 as c_int);
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
pub unsafe extern "C" fn spSkeleton_setSkin(
    mut self_0: *mut spSkeleton,
    mut newSkin: *mut spSkin,
) {
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
                    let mut attachment: *mut spAttachment = spSkin_getAttachment(
                        newSkin,
                        i,
                        (*(*slot).data).attachmentName,
                    );
                    if !attachment.is_null() {
                        spSlot_setAttachment(slot, attachment);
                    }
                }
                i += 1;
            }
        }
    }
    let ref mut fresh104 = *(&(*self_0).skin as *const *mut spSkin as *mut *mut spSkin);
    *fresh104 = newSkin;
    spSkeleton_updateCache(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn spSkeleton_getAttachmentForSlotName(
    mut self_0: *const spSkeleton,
    mut slotName: *const c_char,
    mut attachmentName: *const c_char,
) -> *mut spAttachment {
    let mut slotIndex: c_int = spSkeletonData_findSlotIndex(
        (*self_0).data,
        slotName,
    );
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
        let mut attachment: *mut spAttachment = spSkin_getAttachment(
            (*self_0).skin,
            slotIndex,
            attachmentName,
        );
        if !attachment.is_null() {
            return attachment;
        }
    }
    if !((*(*self_0).data).defaultSkin).is_null() {
        let mut attachment_0: *mut spAttachment = spSkin_getAttachment(
            (*(*self_0).data).defaultSkin,
            slotIndex,
            attachmentName,
        );
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
                let mut attachment: *mut spAttachment = spSkeleton_getAttachmentForSlotIndex(
                    self_0,
                    i,
                    attachmentName,
                );
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
pub unsafe extern "C" fn spSkeleton_update(
    mut self_0: *mut spSkeleton,
    mut deltaTime: c_float,
) {
    (*self_0).time += deltaTime;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonBinary_createWithLoader(
    mut attachmentLoader: *mut spAttachmentLoader,
) -> *mut spSkeletonBinary {
    let mut self_0: *mut spSkeletonBinary = &mut (*((_spCalloc
        as unsafe extern "C" fn(
            size_t,
            size_t,
            *const c_char,
            c_int,
        ) -> *mut c_void)(
        1 as c_int as size_t,
        ::core::mem::size_of::<_spSkeletonBinary>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        6735 as c_int,
    ) as *mut _spSkeletonBinary))
        .super_0;
    (*self_0).scale = 1 as c_int as c_float;
    (*self_0).attachmentLoader = attachmentLoader;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonBinary_create(
    mut atlas: *mut spAtlas,
) -> *mut spSkeletonBinary {
    let mut attachmentLoader: *mut spAtlasAttachmentLoader = spAtlasAttachmentLoader_create(
        atlas,
    );
    let mut self_0: *mut spSkeletonBinary = spSkeletonBinary_createWithLoader(
        &mut (*attachmentLoader).super_0,
    );
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
    let ref mut fresh105 = *(&(*self_0).error as *const *const c_char
        as *mut *mut c_char);
    *fresh105 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul(
                (spine_strlen(message.as_mut_ptr()))
                    .wrapping_add(1 as c_int as c_ulong),
            ),
        b"spine.c\0" as *const u8 as *const c_char,
        6763 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh105, message.as_mut_ptr());
}
unsafe extern "C" fn readByte(mut input: *mut _dataInput) -> c_uchar {
    let fresh106 = (*input).cursor;
    (*input).cursor = ((*input).cursor).offset(1);
    return *fresh106;
}
unsafe extern "C" fn readSByte(mut input: *mut _dataInput) -> c_schar {
    return readByte(input) as c_schar;
}
unsafe extern "C" fn readBoolean(mut input: *mut _dataInput) -> c_int {
    return (readByte(input) as c_int != 0 as c_int) as c_int;
}
unsafe extern "C" fn readInt(mut input: *mut _dataInput) -> c_int {
    let mut result: c_int = readByte(input) as c_int;
    result <<= 8 as c_int;
    result |= readByte(input) as c_int;
    result <<= 8 as c_int;
    result |= readByte(input) as c_int;
    result <<= 8 as c_int;
    result |= readByte(input) as c_int;
    return result;
}
unsafe extern "C" fn readVarint(
    mut input: *mut _dataInput,
    mut optimizePositive: c_int,
) -> c_int {
    let mut b: c_uchar = readByte(input);
    let mut value: c_int = b as c_int & 0x7f as c_int;
    if b as c_int & 0x80 as c_int != 0 {
        b = readByte(input);
        value |= (b as c_int & 0x7f as c_int) << 7 as c_int;
        if b as c_int & 0x80 as c_int != 0 {
            b = readByte(input);
            value |= (b as c_int & 0x7f as c_int) << 14 as c_int;
            if b as c_int & 0x80 as c_int != 0 {
                b = readByte(input);
                value |= (b as c_int & 0x7f as c_int) << 21 as c_int;
                if b as c_int & 0x80 as c_int != 0 {
                    value
                        |= (readByte(input) as c_int & 0x7f as c_int)
                            << 28 as c_int;
                }
            }
        }
    }
    if optimizePositive == 0 {
        value = (value as c_uint >> 1 as c_int
            ^ -(value & 1 as c_int) as c_uint) as c_int;
    }
    return value;
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
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul(length as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        6824 as c_int,
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
unsafe extern "C" fn readCurveBinary(
    mut input: *mut _dataInput,
    mut timeline: *mut spCurveTimeline,
    mut frameIndex: c_int,
) {
    match readByte(input) as c_int {
        1 => {
            spCurveTimeline_setStepped(timeline, frameIndex);
        }
        2 => {
            let mut cx1: c_float = readFloat(input);
            let mut cy1: c_float = readFloat(input);
            let mut cx2: c_float = readFloat(input);
            let mut cy2: c_float = readFloat(input);
            spCurveTimeline_setCurve(timeline, frameIndex, cx1, cy1, cx2, cy2);
        }
        _ => {}
    };
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
            6909 as c_int,
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
    let fresh107 = (*internal).linkedMeshCount;
    (*internal).linkedMeshCount = (*internal).linkedMeshCount + 1;
    linkedMesh = ((*internal).linkedMeshes).offset(fresh107 as isize);
    (*linkedMesh).mesh = mesh;
    (*linkedMesh).skin = skin;
    (*linkedMesh).slotIndex = slotIndex;
    (*linkedMesh).parent = parent;
    (*linkedMesh).inheritDeform = inheritDeform;
}
#[no_mangle]
pub unsafe extern "C" fn spTimelineArray_create(
    mut initialCapacity: c_int,
) -> *mut spTimelineArray {
    let mut array: *mut spTimelineArray = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spTimelineArray>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        6924 as c_int,
    ) as *mut spTimelineArray;
    (*array).size = 0 as c_int;
    (*array).capacity = initialCapacity;
    (*array)
        .items = _spCalloc(
        initialCapacity as size_t,
        ::core::mem::size_of::<*mut spTimeline>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        6924 as c_int,
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
        (*self_0)
            .capacity = if 8 as c_int
            > ((*self_0).size as c_float * 1.75f32) as c_int
        {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0)
            .items = _spRealloc(
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
    (*self_0)
        .items = _spRealloc(
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
        (*self_0)
            .capacity = if 8 as c_int
            > ((*self_0).size as c_float * 1.75f32) as c_int
        {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0)
            .items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<*mut spTimeline>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut *mut spTimeline;
    }
    let fresh108 = (*self_0).size;
    (*self_0).size = (*self_0).size + 1;
    let ref mut fresh109 = *((*self_0).items).offset(fresh108 as isize);
    *fresh109 = value;
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
        ((*self_0).items).offset(index as isize).offset(1 as c_int as isize)
            as *const c_void,
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
pub unsafe extern "C" fn spTimelineArray_pop(
    mut self_0: *mut spTimelineArray,
) -> *mut spTimeline {
    (*self_0).size -= 1;
    let mut item: *mut spTimeline = *((*self_0).items).offset((*self_0).size as isize);
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn spTimelineArray_peek(
    mut self_0: *mut spTimelineArray,
) -> *mut spTimeline {
    return *((*self_0).items).offset(((*self_0).size - 1 as c_int) as isize);
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
    let mut frameIndex: c_int = 0;
    let mut drawOrderCount: c_int = 0;
    let mut eventCount: c_int = 0;
    let mut animation: *mut spAnimation = 0 as *mut spAnimation;
    i = 0 as c_int;
    n = readVarint(input, 1 as c_int);
    while i < n {
        let mut slotIndex: c_int = readVarint(input, 1 as c_int);
        ii = 0 as c_int;
        nn = readVarint(input, 1 as c_int);
        while ii < nn {
            let mut timelineType: c_uchar = readByte(input);
            let mut frameCount: c_int = readVarint(input, 1 as c_int);
            match timelineType as c_int {
                0 => {
                    let mut timeline: *mut spAttachmentTimeline = spAttachmentTimeline_create(
                        frameCount,
                    );
                    (*timeline).slotIndex = slotIndex;
                    frameIndex = 0 as c_int;
                    while frameIndex < frameCount {
                        let mut time: c_float = readFloat(input);
                        let mut attachmentName: *const c_char = readStringRef(
                            input,
                            skeletonData,
                        );
                        spAttachmentTimeline_setFrame(
                            timeline,
                            frameIndex,
                            time,
                            attachmentName,
                        );
                        frameIndex += 1;
                    }
                    spTimelineArray_add(timelines, timeline as *mut spTimeline);
                    duration = if duration
                        > *((*timeline).frames)
                            .offset((frameCount - 1 as c_int) as isize)
                    {
                        duration
                    } else {
                        *((*timeline).frames)
                            .offset((frameCount - 1 as c_int) as isize)
                    };
                }
                1 => {
                    let mut timeline_0: *mut spColorTimeline = spColorTimeline_create(
                        frameCount,
                    );
                    (*timeline_0).slotIndex = slotIndex;
                    frameIndex = 0 as c_int;
                    while frameIndex < frameCount {
                        let mut time_0: c_float = readFloat(input);
                        let mut r: c_float = 0.;
                        let mut g: c_float = 0.;
                        let mut b: c_float = 0.;
                        let mut a: c_float = 0.;
                        readColor(input, &mut r, &mut g, &mut b, &mut a);
                        spColorTimeline_setFrame(
                            timeline_0,
                            frameIndex,
                            time_0,
                            r,
                            g,
                            b,
                            a,
                        );
                        if frameIndex < frameCount - 1 as c_int {
                            readCurveBinary(
                                input,
                                &mut (*timeline_0).super_0,
                                frameIndex,
                            );
                        }
                        frameIndex += 1;
                    }
                    spTimelineArray_add(timelines, timeline_0 as *mut spTimeline);
                    duration = if duration
                        > *((*timeline_0).frames)
                            .offset(
                                ((frameCount - 1 as c_int) * COLOR_ENTRIES) as isize,
                            )
                    {
                        duration
                    } else {
                        *((*timeline_0).frames)
                            .offset(
                                ((frameCount - 1 as c_int) * COLOR_ENTRIES) as isize,
                            )
                    };
                }
                2 => {
                    let mut timeline_1: *mut spTwoColorTimeline = spTwoColorTimeline_create(
                        frameCount,
                    );
                    (*timeline_1).slotIndex = slotIndex;
                    frameIndex = 0 as c_int;
                    while frameIndex < frameCount {
                        let mut time_1: c_float = readFloat(input);
                        let mut r_0: c_float = 0.;
                        let mut g_0: c_float = 0.;
                        let mut b_0: c_float = 0.;
                        let mut a_0: c_float = 0.;
                        let mut r2: c_float = 0.;
                        let mut g2: c_float = 0.;
                        let mut b2: c_float = 0.;
                        let mut a2: c_float = 0.;
                        readColor(input, &mut r_0, &mut g_0, &mut b_0, &mut a_0);
                        readColor(input, &mut a2, &mut r2, &mut g2, &mut b2);
                        spTwoColorTimeline_setFrame(
                            timeline_1,
                            frameIndex,
                            time_1,
                            r_0,
                            g_0,
                            b_0,
                            a_0,
                            r2,
                            g2,
                            b2,
                        );
                        if frameIndex < frameCount - 1 as c_int {
                            readCurveBinary(
                                input,
                                &mut (*timeline_1).super_0,
                                frameIndex,
                            );
                        }
                        frameIndex += 1;
                    }
                    spTimelineArray_add(timelines, timeline_1 as *mut spTimeline);
                    duration = if duration
                        > *((*timeline_1).frames)
                            .offset(
                                ((frameCount - 1 as c_int) * TWOCOLOR_ENTRIES)
                                    as isize,
                            )
                    {
                        duration
                    } else {
                        *((*timeline_1).frames)
                            .offset(
                                ((frameCount - 1 as c_int) * TWOCOLOR_ENTRIES)
                                    as isize,
                            )
                    };
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
                        b"Invalid timeline type for a slot: \0" as *const u8
                            as *const c_char,
                        (**((*skeletonData).slots).offset(slotIndex as isize)).name,
                    );
                    return 0 as *mut spAnimation;
                }
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
            match timelineType_0 as c_int {
                0 => {
                    let mut timeline_2: *mut spRotateTimeline = spRotateTimeline_create(
                        frameCount_0,
                    );
                    (*timeline_2).boneIndex = boneIndex;
                    frameIndex = 0 as c_int;
                    while frameIndex < frameCount_0 {
                        let mut time_2: c_float = readFloat(input);
                        let mut degrees: c_float = readFloat(input);
                        spRotateTimeline_setFrame(
                            timeline_2,
                            frameIndex,
                            time_2,
                            degrees,
                        );
                        if frameIndex < frameCount_0 - 1 as c_int {
                            readCurveBinary(
                                input,
                                &mut (*timeline_2).super_0,
                                frameIndex,
                            );
                        }
                        frameIndex += 1;
                    }
                    spTimelineArray_add(timelines, timeline_2 as *mut spTimeline);
                    duration = if duration
                        > *((*timeline_2).frames)
                            .offset(
                                ((frameCount_0 - 1 as c_int) * ROTATE_ENTRIES)
                                    as isize,
                            )
                    {
                        duration
                    } else {
                        *((*timeline_2).frames)
                            .offset(
                                ((frameCount_0 - 1 as c_int) * ROTATE_ENTRIES)
                                    as isize,
                            )
                    };
                }
                1 | 2 | 3 => {
                    let mut timelineScale: c_float = 1 as c_int
                        as c_float;
                    let mut timeline_3: *mut spTranslateTimeline = 0
                        as *mut spTranslateTimeline;
                    match timelineType_0 as c_int {
                        2 => {
                            timeline_3 = spScaleTimeline_create(frameCount_0);
                        }
                        3 => {
                            timeline_3 = spShearTimeline_create(frameCount_0);
                        }
                        1 => {
                            timeline_3 = spTranslateTimeline_create(frameCount_0);
                            timelineScale = (*self_0).scale;
                        }
                        _ => {}
                    }
                    (*timeline_3).boneIndex = boneIndex;
                    frameIndex = 0 as c_int;
                    while frameIndex < frameCount_0 {
                        let mut time_3: c_float = readFloat(input);
                        let mut x: c_float = readFloat(input) * timelineScale;
                        let mut y: c_float = readFloat(input) * timelineScale;
                        spTranslateTimeline_setFrame(
                            timeline_3,
                            frameIndex,
                            time_3,
                            x,
                            y,
                        );
                        if frameIndex < frameCount_0 - 1 as c_int {
                            readCurveBinary(
                                input,
                                &mut (*timeline_3).super_0,
                                frameIndex,
                            );
                        }
                        frameIndex += 1;
                    }
                    spTimelineArray_add(timelines, timeline_3 as *mut spTimeline);
                    duration = if duration
                        > *((*timeline_3).frames)
                            .offset(
                                ((frameCount_0 - 1 as c_int) * TRANSLATE_ENTRIES)
                                    as isize,
                            )
                    {
                        duration
                    } else {
                        *((*timeline_3).frames)
                            .offset(
                                ((frameCount_0 - 1 as c_int) * TRANSLATE_ENTRIES)
                                    as isize,
                            )
                    };
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
                        b"Invalid timeline type for a bone: \0" as *const u8
                            as *const c_char,
                        (**((*skeletonData).bones).offset(boneIndex as isize)).name,
                    );
                    return 0 as *mut spAnimation;
                }
            }
            ii += 1;
        }
        i += 1;
    }
    i = 0 as c_int;
    n = readVarint(input, 1 as c_int);
    while i < n {
        let mut index: c_int = readVarint(input, 1 as c_int);
        let mut frameCount_1: c_int = readVarint(input, 1 as c_int);
        let mut timeline_4: *mut spIkConstraintTimeline = spIkConstraintTimeline_create(
            frameCount_1,
        );
        (*timeline_4).ikConstraintIndex = index;
        frameIndex = 0 as c_int;
        while frameIndex < frameCount_1 {
            let mut time_4: c_float = readFloat(input);
            let mut mix: c_float = readFloat(input);
            let mut softness: c_float = readFloat(input);
            let mut bendDirection: c_schar = readSByte(input);
            let mut compress: c_int = readBoolean(input);
            let mut stretch: c_int = readBoolean(input);
            spIkConstraintTimeline_setFrame(
                timeline_4,
                frameIndex,
                time_4,
                mix,
                softness,
                bendDirection as c_int,
                compress,
                stretch,
            );
            if frameIndex < frameCount_1 - 1 as c_int {
                readCurveBinary(input, &mut (*timeline_4).super_0, frameIndex);
            }
            frameIndex += 1;
        }
        spTimelineArray_add(timelines, timeline_4 as *mut spTimeline);
        duration = if duration
            > *((*timeline_4).frames)
                .offset(
                    ((frameCount_1 - 1 as c_int) * IKCONSTRAINT_ENTRIES) as isize,
                )
        {
            duration
        } else {
            *((*timeline_4).frames)
                .offset(
                    ((frameCount_1 - 1 as c_int) * IKCONSTRAINT_ENTRIES) as isize,
                )
        };
        i += 1;
    }
    i = 0 as c_int;
    n = readVarint(input, 1 as c_int);
    while i < n {
        let mut index_0: c_int = readVarint(input, 1 as c_int);
        let mut frameCount_2: c_int = readVarint(input, 1 as c_int);
        let mut timeline_5: *mut spTransformConstraintTimeline = spTransformConstraintTimeline_create(
            frameCount_2,
        );
        (*timeline_5).transformConstraintIndex = index_0;
        frameIndex = 0 as c_int;
        while frameIndex < frameCount_2 {
            let mut time_5: c_float = readFloat(input);
            let mut rotateMix: c_float = readFloat(input);
            let mut translateMix: c_float = readFloat(input);
            let mut scaleMix: c_float = readFloat(input);
            let mut shearMix: c_float = readFloat(input);
            spTransformConstraintTimeline_setFrame(
                timeline_5,
                frameIndex,
                time_5,
                rotateMix,
                translateMix,
                scaleMix,
                shearMix,
            );
            if frameIndex < frameCount_2 - 1 as c_int {
                readCurveBinary(input, &mut (*timeline_5).super_0, frameIndex);
            }
            frameIndex += 1;
        }
        spTimelineArray_add(timelines, timeline_5 as *mut spTimeline);
        duration = if duration
            > *((*timeline_5).frames)
                .offset(
                    ((frameCount_2 - 1 as c_int) * TRANSFORMCONSTRAINT_ENTRIES)
                        as isize,
                )
        {
            duration
        } else {
            *((*timeline_5).frames)
                .offset(
                    ((frameCount_2 - 1 as c_int) * TRANSFORMCONSTRAINT_ENTRIES)
                        as isize,
                )
        };
        i += 1;
    }
    i = 0 as c_int;
    n = readVarint(input, 1 as c_int);
    while i < n {
        let mut index_1: c_int = readVarint(input, 1 as c_int);
        let mut data: *mut spPathConstraintData = *((*skeletonData).pathConstraints)
            .offset(index_1 as isize);
        ii = 0 as c_int;
        nn = readVarint(input, 1 as c_int);
        while ii < nn {
            let mut timelineType_1: c_uchar = readByte(input);
            let mut frameCount_3: c_int = readVarint(input, 1 as c_int);
            match timelineType_1 as c_int {
                0 | 1 => {
                    let mut timeline_6: *mut spPathConstraintPositionTimeline = 0
                        as *mut spPathConstraintPositionTimeline;
                    let mut timelineScale_0: c_float = 1 as c_int
                        as c_float;
                    if timelineType_1 as c_int == 1 as c_int {
                        timeline_6 = spPathConstraintSpacingTimeline_create(frameCount_3)
                            as *mut spPathConstraintPositionTimeline;
                        if (*data).spacingMode as c_uint
                            == SP_SPACING_MODE_LENGTH as c_int as c_uint
                            || (*data).spacingMode as c_uint
                                == SP_SPACING_MODE_FIXED as c_int as c_uint
                        {
                            timelineScale_0 = (*self_0).scale;
                        }
                    } else {
                        timeline_6 = spPathConstraintPositionTimeline_create(
                            frameCount_3,
                        );
                        if (*data).positionMode as c_uint
                            == SP_POSITION_MODE_FIXED as c_int as c_uint
                        {
                            timelineScale_0 = (*self_0).scale;
                        }
                    }
                    (*timeline_6).pathConstraintIndex = index_1;
                    frameIndex = 0 as c_int;
                    while frameIndex < frameCount_3 {
                        let mut time_6: c_float = readFloat(input);
                        let mut value: c_float = readFloat(input)
                            * timelineScale_0;
                        spPathConstraintPositionTimeline_setFrame(
                            timeline_6,
                            frameIndex,
                            time_6,
                            value,
                        );
                        if frameIndex < frameCount_3 - 1 as c_int {
                            readCurveBinary(
                                input,
                                &mut (*timeline_6).super_0,
                                frameIndex,
                            );
                        }
                        frameIndex += 1;
                    }
                    spTimelineArray_add(timelines, timeline_6 as *mut spTimeline);
                    duration = if duration
                        > *((*timeline_6).frames)
                            .offset(
                                ((frameCount_3 - 1 as c_int)
                                    * PATHCONSTRAINTPOSITION_ENTRIES) as isize,
                            )
                    {
                        duration
                    } else {
                        *((*timeline_6).frames)
                            .offset(
                                ((frameCount_3 - 1 as c_int)
                                    * PATHCONSTRAINTPOSITION_ENTRIES) as isize,
                            )
                    };
                }
                2 => {
                    let mut timeline_7: *mut spPathConstraintMixTimeline = spPathConstraintMixTimeline_create(
                        frameCount_3,
                    );
                    (*timeline_7).pathConstraintIndex = index_1;
                    frameIndex = 0 as c_int;
                    while frameIndex < frameCount_3 {
                        let mut time_7: c_float = readFloat(input);
                        let mut rotateMix_0: c_float = readFloat(input);
                        let mut translateMix_0: c_float = readFloat(input);
                        spPathConstraintMixTimeline_setFrame(
                            timeline_7,
                            frameIndex,
                            time_7,
                            rotateMix_0,
                            translateMix_0,
                        );
                        if frameIndex < frameCount_3 - 1 as c_int {
                            readCurveBinary(
                                input,
                                &mut (*timeline_7).super_0,
                                frameIndex,
                            );
                        }
                        frameIndex += 1;
                    }
                    spTimelineArray_add(timelines, timeline_7 as *mut spTimeline);
                    duration = if duration
                        > *((*timeline_7).frames)
                            .offset(
                                ((frameCount_3 - 1 as c_int)
                                    * PATHCONSTRAINTMIX_ENTRIES) as isize,
                            )
                    {
                        duration
                    } else {
                        *((*timeline_7).frames)
                            .offset(
                                ((frameCount_3 - 1 as c_int)
                                    * PATHCONSTRAINTMIX_ENTRIES) as isize,
                            )
                    };
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
        let mut skin: *mut spSkin = *((*skeletonData).skins)
            .offset(readVarint(input, 1 as c_int) as isize);
        ii = 0 as c_int;
        nn = readVarint(input, 1 as c_int);
        while ii < nn {
            let mut slotIndex_0: c_int = readVarint(input, 1 as c_int);
            iii = 0 as c_int;
            nnn = readVarint(input, 1 as c_int);
            while iii < nnn {
                let mut tempDeform: *mut c_float = 0 as *mut c_float;
                let mut timeline_8: *mut spDeformTimeline = 0 as *mut spDeformTimeline;
                let mut weighted: c_int = 0;
                let mut deformLength: c_int = 0;
                let mut attachmentName_0: *const c_char = readStringRef(
                    input,
                    skeletonData,
                );
                let mut frameCount_4: c_int = 0;
                let mut attachment: *mut spVertexAttachment = spSkin_getAttachment(
                    skin,
                    slotIndex_0,
                    attachmentName_0,
                ) as *mut spVertexAttachment;
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
                    7171 as c_int,
                ) as *mut c_float;
                frameCount_4 = readVarint(input, 1 as c_int);
                timeline_8 = spDeformTimeline_create(frameCount_4, deformLength);
                (*timeline_8).slotIndex = slotIndex_0;
                (*timeline_8).attachment = &mut (*attachment).super_0;
                frameIndex = 0 as c_int;
                while frameIndex < frameCount_4 {
                    let mut time_8: c_float = readFloat(input);
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
                                *deform
                                    .offset(v as isize) = readFloat(input) * (*self_0).scale;
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
                            let mut vertices: *mut c_float = (*attachment)
                                .vertices;
                            v = 0 as c_int;
                            while v < deformLength {
                                *deform.offset(v as isize) += *vertices.offset(v as isize);
                                v += 1;
                            }
                        }
                    }
                    spDeformTimeline_setFrame(timeline_8, frameIndex, time_8, deform);
                    if frameIndex < frameCount_4 - 1 as c_int {
                        readCurveBinary(input, &mut (*timeline_8).super_0, frameIndex);
                    }
                    frameIndex += 1;
                }
                _spFree(tempDeform as *mut c_void);
                spTimelineArray_add(timelines, timeline_8 as *mut spTimeline);
                duration = if duration
                    > *((*timeline_8).frames)
                        .offset((frameCount_4 - 1 as c_int) as isize)
                {
                    duration
                } else {
                    *((*timeline_8).frames)
                        .offset((frameCount_4 - 1 as c_int) as isize)
                };
                iii += 1;
            }
            ii += 1;
        }
        i += 1;
    }
    drawOrderCount = readVarint(input, 1 as c_int);
    if drawOrderCount != 0 {
        let mut timeline_9: *mut spDrawOrderTimeline = spDrawOrderTimeline_create(
            drawOrderCount,
            (*skeletonData).slotsCount,
        );
        i = 0 as c_int;
        while i < drawOrderCount {
            let mut time_9: c_float = readFloat(input);
            let mut offsetCount: c_int = readVarint(input, 1 as c_int);
            let mut drawOrder: *mut c_int = _spMalloc(
                (::core::mem::size_of::<c_int>() as c_ulong)
                    .wrapping_mul((*skeletonData).slotsCount as c_ulong),
                b"spine.c\0" as *const u8 as *const c_char,
                7225 as c_int,
            ) as *mut c_int;
            let mut unchanged: *mut c_int = _spMalloc(
                (::core::mem::size_of::<c_int>() as c_ulong)
                    .wrapping_mul(
                        ((*skeletonData).slotsCount - offsetCount) as c_ulong,
                    ),
                b"spine.c\0" as *const u8 as *const c_char,
                7226 as c_int,
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
                    let fresh110 = originalIndex;
                    originalIndex = originalIndex + 1;
                    let fresh111 = unchangedIndex;
                    unchangedIndex = unchangedIndex + 1;
                    *unchanged.offset(fresh111 as isize) = fresh110;
                }
                *drawOrder
                    .offset(
                        (originalIndex + readVarint(input, 1 as c_int)) as isize,
                    ) = originalIndex;
                originalIndex += 1;
                ii += 1;
            }
            while originalIndex < (*skeletonData).slotsCount {
                let fresh112 = originalIndex;
                originalIndex = originalIndex + 1;
                let fresh113 = unchangedIndex;
                unchangedIndex = unchangedIndex + 1;
                *unchanged.offset(fresh113 as isize) = fresh112;
            }
            ii = (*skeletonData).slotsCount - 1 as c_int;
            while ii >= 0 as c_int {
                if *drawOrder.offset(ii as isize) == -(1 as c_int) {
                    unchangedIndex -= 1;
                    *drawOrder
                        .offset(
                            ii as isize,
                        ) = *unchanged.offset(unchangedIndex as isize);
                }
                ii -= 1;
            }
            _spFree(unchanged as *mut c_void);
            spDrawOrderTimeline_setFrame(timeline_9, i, time_9, drawOrder);
            _spFree(drawOrder as *mut c_void);
            i += 1;
        }
        spTimelineArray_add(timelines, timeline_9 as *mut spTimeline);
        duration = if duration
            > *((*timeline_9).frames)
                .offset((drawOrderCount - 1 as c_int) as isize)
        {
            duration
        } else {
            *((*timeline_9).frames).offset((drawOrderCount - 1 as c_int) as isize)
        };
    }
    eventCount = readVarint(input, 1 as c_int);
    if eventCount != 0 {
        let mut timeline_10: *mut spEventTimeline = spEventTimeline_create(eventCount);
        i = 0 as c_int;
        while i < eventCount {
            let mut time_10: c_float = readFloat(input);
            let mut eventData: *mut spEventData = *((*skeletonData).events)
                .offset(readVarint(input, 1 as c_int) as isize);
            let mut event: *mut spEvent = spEvent_create(time_10, eventData);
            (*event).intValue = readVarint(input, 0 as c_int);
            (*event).floatValue = readFloat(input);
            if readBoolean(input) != 0 {
                (*event).stringValue = readString(input);
            } else {
                let ref mut fresh114 = *(&mut (*event).stringValue
                    as *mut *const c_char as *mut *mut c_char);
                *fresh114 = _spMalloc(
                    (::core::mem::size_of::<c_char>() as c_ulong)
                        .wrapping_mul(
                            (spine_strlen((*eventData).stringValue))
                                .wrapping_add(1 as c_int as c_ulong),
                        ),
                    b"spine.c\0" as *const u8 as *const c_char,
                    7266 as c_int,
                ) as *mut c_char;
                spine_strcpy(*fresh114, (*eventData).stringValue);
            }
            if !((*eventData).audioPath).is_null() {
                (*event).volume = readFloat(input);
                (*event).balance = readFloat(input);
            }
            spEventTimeline_setFrame(timeline_10, i, event);
            i += 1;
        }
        spTimelineArray_add(timelines, timeline_10 as *mut spTimeline);
        duration = if duration
            > *((*timeline_10).frames).offset((eventCount - 1 as c_int) as isize)
        {
            duration
        } else {
            *((*timeline_10).frames).offset((eventCount - 1 as c_int) as isize)
        };
    }
    animation = spAnimation_create(name, 0 as c_int);
    _spFree((*animation).timelines as *mut c_void);
    (*animation).duration = duration;
    (*animation).timelinesCount = (*timelines).size;
    (*animation).timelines = (*timelines).items;
    _spFree(timelines as *mut c_void);
    return animation;
}
unsafe extern "C" fn _readFloatArray(
    mut input: *mut _dataInput,
    mut n: c_int,
    mut scale: c_float,
) -> *mut c_float {
    let mut array: *mut c_float = _spMalloc(
        (::core::mem::size_of::<c_float>() as c_ulong)
            .wrapping_mul(n as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        7287 as c_int,
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
        (::core::mem::size_of::<c_short>() as c_ulong)
            .wrapping_mul(n as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        7300 as c_int,
    ) as *mut c_short;
    let mut i: c_int = 0;
    *length = n;
    i = 0 as c_int;
    while i < n {
        *array
            .offset(
                i as isize,
            ) = ((readByte(input) as c_int) << 8 as c_int) as c_short;
        let ref mut fresh115 = *array.offset(i as isize);
        *fresh115 = (*fresh115 as c_int | readByte(input) as c_int)
            as c_short;
        i += 1;
    }
    return array;
}
unsafe extern "C" fn _readVerticesBinary(
    mut self_0: *mut spSkeletonBinary,
    mut input: *mut _dataInput,
    mut attachment: *mut spVertexAttachment,
    mut vertexCount: c_int,
) {
    let mut i: c_int = 0;
    let mut ii: c_int = 0;
    let mut verticesLength: c_int = vertexCount << 1 as c_int;
    let mut weights: *mut spFloatArray = spFloatArray_create(8 as c_int);
    let mut bones: *mut spIntArray = spIntArray_create(8 as c_int);
    (*attachment).worldVerticesLength = verticesLength;
    if readBoolean(input) == 0 {
        (*attachment).verticesCount = verticesLength;
        (*attachment).vertices = _readFloatArray(input, verticesLength, (*self_0).scale);
        (*attachment).bonesCount = 0 as c_int;
        (*attachment).bones = 0 as *mut c_int;
        spFloatArray_dispose(weights);
        spIntArray_dispose(bones);
        return;
    }
    spFloatArray_ensureCapacity(
        weights,
        verticesLength * 3 as c_int * 3 as c_int,
    );
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
    (*attachment).verticesCount = (*weights).size;
    (*attachment).vertices = (*weights).items;
    _spFree(weights as *mut c_void);
    (*attachment).bonesCount = (*bones).size;
    (*attachment).bones = (*bones).items;
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
            let mut attachment: *mut spAttachment = 0 as *mut spAttachment;
            let mut region: *mut spRegionAttachment = 0 as *mut spRegionAttachment;
            if path.is_null() {
                let ref mut fresh116 = *(&mut path as *mut *const c_char
                    as *mut *mut c_char);
                *fresh116 = _spMalloc(
                    (::core::mem::size_of::<c_char>() as c_ulong)
                        .wrapping_mul(
                            (spine_strlen(name))
                                .wrapping_add(1 as c_int as c_ulong),
                        ),
                    b"spine.c\0" as *const u8 as *const c_char,
                    7366 as c_int,
                ) as *mut c_char;
                spine_strcpy(*fresh116, name);
            } else {
                let mut tmp: *const c_char = 0 as *const c_char;
                let ref mut fresh117 = *(&mut tmp as *mut *const c_char
                    as *mut *mut c_char);
                *fresh117 = _spMalloc(
                    (::core::mem::size_of::<c_char>() as c_ulong)
                        .wrapping_mul(
                            (spine_strlen(path))
                                .wrapping_add(1 as c_int as c_ulong),
                        ),
                    b"spine.c\0" as *const u8 as *const c_char,
                    7369 as c_int,
                ) as *mut c_char;
                spine_strcpy(*fresh117, path);
                path = tmp;
            }
            attachment = spAttachmentLoader_createAttachment(
                (*self_0).attachmentLoader,
                skin,
                type_0,
                name,
                path,
            );
            region = attachment as *mut spRegionAttachment;
            (*region).path = path;
            (*region).rotation = readFloat(input);
            (*region).x = readFloat(input) * (*self_0).scale;
            (*region).y = readFloat(input) * (*self_0).scale;
            (*region).scaleX = readFloat(input);
            (*region).scaleY = readFloat(input);
            (*region).width = readFloat(input) * (*self_0).scale;
            (*region).height = readFloat(input) * (*self_0).scale;
            readColor(
                input,
                &mut (*region).color.r,
                &mut (*region).color.g,
                &mut (*region).color.b,
                &mut (*region).color.a,
            );
            spRegionAttachment_updateOffset(region);
            spAttachmentLoader_configureAttachment(
                (*self_0).attachmentLoader,
                attachment,
            );
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
            );
            _readVerticesBinary(
                self_0,
                input,
                attachment_0 as *mut spVertexAttachment,
                vertexCount,
            );
            if nonessential != 0 {
                readInt(input);
            }
            spAttachmentLoader_configureAttachment(
                (*self_0).attachmentLoader,
                attachment_0,
            );
            return attachment_0;
        }
        2 => {
            let mut vertexCount_0: c_int = 0;
            let mut attachment_1: *mut spAttachment = 0 as *mut spAttachment;
            let mut mesh: *mut spMeshAttachment = 0 as *mut spMeshAttachment;
            let mut path_0: *const c_char = readStringRef(input, skeletonData);
            if path_0.is_null() {
                let ref mut fresh118 = *(&mut path_0 as *mut *const c_char
                    as *mut *mut c_char);
                *fresh118 = _spMalloc(
                    (::core::mem::size_of::<c_char>() as c_ulong)
                        .wrapping_mul(
                            (spine_strlen(name))
                                .wrapping_add(1 as c_int as c_ulong),
                        ),
                    b"spine.c\0" as *const u8 as *const c_char,
                    7400 as c_int,
                ) as *mut c_char;
                spine_strcpy(*fresh118, name);
            } else {
                let mut tmp_0: *const c_char = 0 as *const c_char;
                let ref mut fresh119 = *(&mut tmp_0 as *mut *const c_char
                    as *mut *mut c_char);
                *fresh119 = _spMalloc(
                    (::core::mem::size_of::<c_char>() as c_ulong)
                        .wrapping_mul(
                            (spine_strlen(path_0))
                                .wrapping_add(1 as c_int as c_ulong),
                        ),
                    b"spine.c\0" as *const u8 as *const c_char,
                    7403 as c_int,
                ) as *mut c_char;
                spine_strcpy(*fresh119, path_0);
                path_0 = tmp_0;
            }
            attachment_1 = spAttachmentLoader_createAttachment(
                (*self_0).attachmentLoader,
                skin,
                type_0,
                name,
                path_0,
            );
            mesh = attachment_1 as *mut spMeshAttachment;
            (*mesh).path = path_0;
            readColor(
                input,
                &mut (*mesh).color.r,
                &mut (*mesh).color.g,
                &mut (*mesh).color.b,
                &mut (*mesh).color.a,
            );
            vertexCount_0 = readVarint(input, 1 as c_int);
            (*mesh)
                .regionUVs = _readFloatArray(
                input,
                vertexCount_0 << 1 as c_int,
                1 as c_int as c_float,
            );
            (*mesh)
                .triangles = _readShortArray(input, &mut (*mesh).trianglesCount)
                as *mut c_ushort;
            _readVerticesBinary(self_0, input, &mut (*mesh).super_0, vertexCount_0);
            spMeshAttachment_updateUVs(mesh);
            (*mesh).hullLength = readVarint(input, 1 as c_int) << 1 as c_int;
            if nonessential != 0 {
                (*mesh)
                    .edges = _readShortArray(input, &mut (*mesh).edgesCount)
                    as *mut c_int;
                (*mesh).width = readFloat(input) * (*self_0).scale;
                (*mesh).height = readFloat(input) * (*self_0).scale;
            } else {
                (*mesh).edges = 0 as *mut c_int;
                (*mesh).width = 0 as c_int as c_float;
                (*mesh).height = 0 as c_int as c_float;
            }
            spAttachmentLoader_configureAttachment(
                (*self_0).attachmentLoader,
                attachment_1,
            );
            return attachment_1;
        }
        3 => {
            let mut skinName: *const c_char = 0 as *const c_char;
            let mut parent: *const c_char = 0 as *const c_char;
            let mut attachment_2: *mut spAttachment = 0 as *mut spAttachment;
            let mut mesh_0: *mut spMeshAttachment = 0 as *mut spMeshAttachment;
            let mut inheritDeform: c_int = 0;
            let mut path_1: *const c_char = readStringRef(input, skeletonData);
            if path_1.is_null() {
                let ref mut fresh120 = *(&mut path_1 as *mut *const c_char
                    as *mut *mut c_char);
                *fresh120 = _spMalloc(
                    (::core::mem::size_of::<c_char>() as c_ulong)
                        .wrapping_mul(
                            (spine_strlen(name))
                                .wrapping_add(1 as c_int as c_ulong),
                        ),
                    b"spine.c\0" as *const u8 as *const c_char,
                    7435 as c_int,
                ) as *mut c_char;
                spine_strcpy(*fresh120, name);
            } else {
                let mut tmp_1: *const c_char = 0 as *const c_char;
                let ref mut fresh121 = *(&mut tmp_1 as *mut *const c_char
                    as *mut *mut c_char);
                *fresh121 = _spMalloc(
                    (::core::mem::size_of::<c_char>() as c_ulong)
                        .wrapping_mul(
                            (spine_strlen(path_1))
                                .wrapping_add(1 as c_int as c_ulong),
                        ),
                    b"spine.c\0" as *const u8 as *const c_char,
                    7438 as c_int,
                ) as *mut c_char;
                spine_strcpy(*fresh121, path_1);
                path_1 = tmp_1;
            }
            attachment_2 = spAttachmentLoader_createAttachment(
                (*self_0).attachmentLoader,
                skin,
                type_0,
                name,
                path_1,
            );
            mesh_0 = attachment_2 as *mut spMeshAttachment;
            (*mesh_0).path = path_1;
            readColor(
                input,
                &mut (*mesh_0).color.r,
                &mut (*mesh_0).color.g,
                &mut (*mesh_0).color.b,
                &mut (*mesh_0).color.a,
            );
            skinName = readStringRef(input, skeletonData);
            parent = readStringRef(input, skeletonData);
            inheritDeform = readBoolean(input);
            if nonessential != 0 {
                (*mesh_0).width = readFloat(input) * (*self_0).scale;
                (*mesh_0).height = readFloat(input) * (*self_0).scale;
            }
            _spSkeletonBinary_addLinkedMesh(
                self_0,
                mesh_0,
                skinName,
                slotIndex,
                parent,
                inheritDeform,
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
            );
            let mut path_2: *mut spPathAttachment = attachment_3
                as *mut spPathAttachment;
            let mut vertexCount_1: c_int = 0 as c_int;
            (*path_2).closed = readBoolean(input);
            (*path_2).constantSpeed = readBoolean(input);
            vertexCount_1 = readVarint(input, 1 as c_int);
            _readVerticesBinary(self_0, input, &mut (*path_2).super_0, vertexCount_1);
            (*path_2).lengthsLength = vertexCount_1 / 3 as c_int;
            (*path_2)
                .lengths = _spMalloc(
                (::core::mem::size_of::<c_float>() as c_ulong)
                    .wrapping_mul((*path_2).lengthsLength as c_ulong),
                b"spine.c\0" as *const u8 as *const c_char,
                7464 as c_int,
            ) as *mut c_float;
            i = 0 as c_int;
            while i < (*path_2).lengthsLength {
                *((*path_2).lengths)
                    .offset(i as isize) = readFloat(input) * (*self_0).scale;
                i += 1;
            }
            if nonessential != 0 {
                readInt(input);
            }
            spAttachmentLoader_configureAttachment(
                (*self_0).attachmentLoader,
                attachment_3,
            );
            return attachment_3;
        }
        5 => {
            let mut attachment_4: *mut spAttachment = spAttachmentLoader_createAttachment(
                (*self_0).attachmentLoader,
                skin,
                type_0,
                name,
                0 as *const c_char,
            );
            let mut point: *mut spPointAttachment = attachment_4
                as *mut spPointAttachment;
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
            spAttachmentLoader_configureAttachment(
                (*self_0).attachmentLoader,
                attachment_4,
            );
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
            );
            let mut clip: *mut spClippingAttachment = attachment_5
                as *mut spClippingAttachment;
            _readVerticesBinary(
                self_0,
                input,
                attachment_5 as *mut spVertexAttachment,
                vertexCount_2,
            );
            if nonessential != 0 {
                readInt(input);
            }
            (*clip).endSlot = *((*skeletonData).slots).offset(endSlotIndex as isize);
            spAttachmentLoader_configureAttachment(
                (*self_0).attachmentLoader,
                attachment_5,
            );
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
                *((*skeletonData).bones)
                    .offset(readVarint(input, 1 as c_int) as isize),
            );
            i += 1;
        }
        i = 0 as c_int;
        n = readVarint(input, 1 as c_int);
        while i < n {
            spIkConstraintDataArray_add(
                (*skin).ikConstraints,
                *((*skeletonData).ikConstraints)
                    .offset(readVarint(input, 1 as c_int) as isize),
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
                *((*skeletonData).pathConstraints)
                    .offset(readVarint(input, 1 as c_int) as isize),
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
    skeletonData = spSkeletonBinary_readSkeletonData(
        self_0,
        binary as *mut c_uchar,
        length,
    );
    _spFree(binary as *mut c_void);
    return skeletonData;
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
    let mut skeletonData: *mut spSkeletonData = 0 as *mut spSkeletonData;
    let mut internal: *mut _spSkeletonBinary = self_0 as *mut _spSkeletonBinary;
    let mut input: *mut _dataInput = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<_dataInput>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        7557 as c_int,
    ) as *mut _dataInput;
    (*input).cursor = binary;
    (*input).end = binary.offset(length as isize);
    _spFree((*self_0).error as *mut c_void);
    let ref mut fresh122 = *(&(*self_0).error as *const *const c_char
        as *mut *mut c_char);
    *fresh122 = 0 as *mut c_char;
    (*internal).linkedMeshCount = 0 as c_int;
    skeletonData = spSkeletonData_create();
    (*skeletonData).hash = readString(input);
    if spine_strlen((*skeletonData).hash) == 0 {
        _spFree((*skeletonData).hash as *mut c_void);
        (*skeletonData).hash = 0 as *const c_char;
    }
    (*skeletonData).version = readString(input);
    if spine_strlen((*skeletonData).version) == 0 {
        _spFree((*skeletonData).version as *mut c_void);
        (*skeletonData).version = 0 as *const c_char;
    }
    if spine_strcmp(
        (*skeletonData).version,
        b"3.8.75\0" as *const u8 as *const c_char,
    ) == 0 as c_int
    {
        _spFree(input as *mut c_void);
        spSkeletonData_dispose(skeletonData);
        _spSkeletonBinary_setError(
            self_0,
            b"Unsupported skeleton data, please export with a newer version of Spine.\0"
                as *const u8 as *const c_char,
            b"\0" as *const u8 as *const c_char,
        );
        return 0 as *mut spSkeletonData;
    }
    (*skeletonData).x = readFloat(input);
    (*skeletonData).y = readFloat(input);
    (*skeletonData).width = readFloat(input);
    (*skeletonData).height = readFloat(input);
    nonessential = readBoolean(input);
    if nonessential != 0 {
        readFloat(input);
        _spFree(readString(input) as *mut c_void);
        _spFree(readString(input) as *mut c_void);
    }
    n = readVarint(input, 1 as c_int);
    (*skeletonData).stringsCount = n;
    (*skeletonData)
        .strings = _spMalloc(
        (::core::mem::size_of::<*mut c_char>() as c_ulong)
            .wrapping_mul((*skeletonData).stringsCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        7600 as c_int,
    ) as *mut *mut c_char;
    i = 0 as c_int;
    while i < n {
        let ref mut fresh123 = *((*skeletonData).strings).offset(i as isize);
        *fresh123 = readString(input);
        i += 1;
    }
    (*skeletonData).bonesCount = readVarint(input, 1 as c_int);
    (*skeletonData)
        .bones = _spMalloc(
        (::core::mem::size_of::<*mut spBoneData>() as c_ulong)
            .wrapping_mul((*skeletonData).bonesCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        7607 as c_int,
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
            readInt(input);
        }
        let ref mut fresh124 = *((*skeletonData).bones).offset(i as isize);
        *fresh124 = data;
        i += 1;
    }
    (*skeletonData).slotsCount = readVarint(input, 1 as c_int);
    (*skeletonData)
        .slots = _spMalloc(
        (::core::mem::size_of::<*mut spSlotData>() as c_ulong)
            .wrapping_mul((*skeletonData).slotsCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        7639 as c_int,
    ) as *mut *mut spSlotData;
    i = 0 as c_int;
    while i < (*skeletonData).slotsCount {
        let mut r: c_int = 0;
        let mut g: c_int = 0;
        let mut b: c_int = 0;
        let mut a: c_int = 0;
        let mut attachmentName: *const c_char = 0 as *const c_char;
        let mut slotName: *const c_char = readString(input);
        let mut boneData: *mut spBoneData = *((*skeletonData).bones)
            .offset(readVarint(input, 1 as c_int) as isize);
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
        if !(r == 0xff as c_int && g == 0xff as c_int
            && b == 0xff as c_int && a == 0xff as c_int)
        {
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
            let ref mut fresh125 = *(&mut (*slotData).attachmentName
                as *mut *const c_char as *mut *mut c_char);
            *fresh125 = _spMalloc(
                (::core::mem::size_of::<c_char>() as c_ulong)
                    .wrapping_mul(
                        (spine_strlen(attachmentName))
                            .wrapping_add(1 as c_int as c_ulong),
                    ),
                b"spine.c\0" as *const u8 as *const c_char,
                7658 as c_int,
            ) as *mut c_char;
            spine_strcpy(*fresh125, attachmentName);
        } else {
            (*slotData).attachmentName = 0 as *const c_char;
        }
        (*slotData).blendMode = readVarint(input, 1 as c_int) as spBlendMode;
        let ref mut fresh126 = *((*skeletonData).slots).offset(i as isize);
        *fresh126 = slotData;
        i += 1;
    }
    (*skeletonData).ikConstraintsCount = readVarint(input, 1 as c_int);
    (*skeletonData)
        .ikConstraints = _spMalloc(
        (::core::mem::size_of::<*mut spIkConstraintData>() as c_ulong)
            .wrapping_mul((*skeletonData).ikConstraintsCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        7666 as c_int,
    ) as *mut *mut spIkConstraintData;
    i = 0 as c_int;
    while i < (*skeletonData).ikConstraintsCount {
        let mut name_0: *const c_char = readString(input);
        let mut data_0: *mut spIkConstraintData = spIkConstraintData_create(name_0);
        (*data_0).order = readVarint(input, 1 as c_int);
        (*data_0).skinRequired = readBoolean(input);
        _spFree(name_0 as *mut c_void);
        (*data_0).bonesCount = readVarint(input, 1 as c_int);
        (*data_0)
            .bones = _spMalloc(
            (::core::mem::size_of::<*mut spBoneData>() as c_ulong)
                .wrapping_mul((*data_0).bonesCount as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            7675 as c_int,
        ) as *mut *mut spBoneData;
        ii = 0 as c_int;
        while ii < (*data_0).bonesCount {
            let ref mut fresh127 = *((*data_0).bones).offset(ii as isize);
            *fresh127 = *((*skeletonData).bones)
                .offset(readVarint(input, 1 as c_int) as isize);
            ii += 1;
        }
        (*data_0)
            .target = *((*skeletonData).bones)
            .offset(readVarint(input, 1 as c_int) as isize);
        (*data_0).mix = readFloat(input);
        (*data_0).softness = readFloat(input);
        (*data_0).bendDirection = readSByte(input) as c_int;
        (*data_0).compress = readBoolean(input);
        (*data_0).stretch = readBoolean(input);
        (*data_0).uniform = readBoolean(input);
        let ref mut fresh128 = *((*skeletonData).ikConstraints).offset(i as isize);
        *fresh128 = data_0;
        i += 1;
    }
    (*skeletonData).transformConstraintsCount = readVarint(input, 1 as c_int);
    (*skeletonData)
        .transformConstraints = _spMalloc(
        (::core::mem::size_of::<*mut spTransformConstraintData>() as c_ulong)
            .wrapping_mul((*skeletonData).transformConstraintsCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        7690 as c_int,
    ) as *mut *mut spTransformConstraintData;
    i = 0 as c_int;
    while i < (*skeletonData).transformConstraintsCount {
        let mut name_1: *const c_char = readString(input);
        let mut data_1: *mut spTransformConstraintData = spTransformConstraintData_create(
            name_1,
        );
        (*data_1).order = readVarint(input, 1 as c_int);
        (*data_1).skinRequired = readBoolean(input);
        _spFree(name_1 as *mut c_void);
        (*data_1).bonesCount = readVarint(input, 1 as c_int);
        let ref mut fresh129 = *(&(*data_1).bones as *const *mut *mut spBoneData
            as *mut *mut *mut spBoneData);
        *fresh129 = _spMalloc(
            (::core::mem::size_of::<*mut spBoneData>() as c_ulong)
                .wrapping_mul((*data_1).bonesCount as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            7700 as c_int,
        ) as *mut *mut spBoneData;
        ii = 0 as c_int;
        while ii < (*data_1).bonesCount {
            let ref mut fresh130 = *((*data_1).bones).offset(ii as isize);
            *fresh130 = *((*skeletonData).bones)
                .offset(readVarint(input, 1 as c_int) as isize);
            ii += 1;
        }
        (*data_1)
            .target = *((*skeletonData).bones)
            .offset(readVarint(input, 1 as c_int) as isize);
        (*data_1).local = readBoolean(input);
        (*data_1).relative = readBoolean(input);
        (*data_1).offsetRotation = readFloat(input);
        (*data_1).offsetX = readFloat(input) * (*self_0).scale;
        (*data_1).offsetY = readFloat(input) * (*self_0).scale;
        (*data_1).offsetScaleX = readFloat(input);
        (*data_1).offsetScaleY = readFloat(input);
        (*data_1).offsetShearY = readFloat(input);
        (*data_1).rotateMix = readFloat(input);
        (*data_1).translateMix = readFloat(input);
        (*data_1).scaleMix = readFloat(input);
        (*data_1).shearMix = readFloat(input);
        let ref mut fresh131 = *((*skeletonData).transformConstraints)
            .offset(i as isize);
        *fresh131 = data_1;
        i += 1;
    }
    (*skeletonData).pathConstraintsCount = readVarint(input, 1 as c_int);
    (*skeletonData)
        .pathConstraints = _spMalloc(
        (::core::mem::size_of::<*mut spPathConstraintData>() as c_ulong)
            .wrapping_mul((*skeletonData).pathConstraintsCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        7721 as c_int,
    ) as *mut *mut spPathConstraintData;
    i = 0 as c_int;
    while i < (*skeletonData).pathConstraintsCount {
        let mut name_2: *const c_char = readString(input);
        let mut data_2: *mut spPathConstraintData = spPathConstraintData_create(name_2);
        (*data_2).order = readVarint(input, 1 as c_int);
        (*data_2).skinRequired = readBoolean(input);
        _spFree(name_2 as *mut c_void);
        (*data_2).bonesCount = readVarint(input, 1 as c_int);
        let ref mut fresh132 = *(&(*data_2).bones as *const *mut *mut spBoneData
            as *mut *mut *mut spBoneData);
        *fresh132 = _spMalloc(
            (::core::mem::size_of::<*mut spBoneData>() as c_ulong)
                .wrapping_mul((*data_2).bonesCount as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            7730 as c_int,
        ) as *mut *mut spBoneData;
        ii = 0 as c_int;
        while ii < (*data_2).bonesCount {
            let ref mut fresh133 = *((*data_2).bones).offset(ii as isize);
            *fresh133 = *((*skeletonData).bones)
                .offset(readVarint(input, 1 as c_int) as isize);
            ii += 1;
        }
        (*data_2)
            .target = *((*skeletonData).slots)
            .offset(readVarint(input, 1 as c_int) as isize);
        (*data_2).positionMode = readVarint(input, 1 as c_int) as spPositionMode;
        (*data_2).spacingMode = readVarint(input, 1 as c_int) as spSpacingMode;
        (*data_2).rotateMode = readVarint(input, 1 as c_int) as spRotateMode;
        (*data_2).offsetRotation = readFloat(input);
        (*data_2).position = readFloat(input);
        if (*data_2).positionMode as c_uint
            == SP_POSITION_MODE_FIXED as c_int as c_uint
        {
            (*data_2).position *= (*self_0).scale;
        }
        (*data_2).spacing = readFloat(input);
        if (*data_2).spacingMode as c_uint
            == SP_SPACING_MODE_LENGTH as c_int as c_uint
            || (*data_2).spacingMode as c_uint
                == SP_SPACING_MODE_FIXED as c_int as c_uint
        {
            (*data_2).spacing *= (*self_0).scale;
        }
        (*data_2).rotateMix = readFloat(input);
        (*data_2).translateMix = readFloat(input);
        let ref mut fresh134 = *((*skeletonData).pathConstraints).offset(i as isize);
        *fresh134 = data_2;
        i += 1;
    }
    (*skeletonData)
        .defaultSkin = spSkeletonBinary_readSkin(
        self_0,
        input,
        -(1 as c_int),
        skeletonData,
        nonessential,
    );
    (*skeletonData).skinsCount = readVarint(input, 1 as c_int);
    if !((*skeletonData).defaultSkin).is_null() {
        (*skeletonData).skinsCount += 1;
    }
    (*skeletonData)
        .skins = _spMalloc(
        (::core::mem::size_of::<*mut spSkin>() as c_ulong)
            .wrapping_mul((*skeletonData).skinsCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        7754 as c_int,
    ) as *mut *mut spSkin;
    if !((*skeletonData).defaultSkin).is_null() {
        let ref mut fresh135 = *((*skeletonData).skins)
            .offset(0 as c_int as isize);
        *fresh135 = (*skeletonData).defaultSkin;
    }
    i = if !((*skeletonData).defaultSkin).is_null() {
        1 as c_int
    } else {
        0 as c_int
    };
    while i < (*skeletonData).skinsCount {
        let ref mut fresh136 = *((*skeletonData).skins).offset(i as isize);
        *fresh136 = spSkeletonBinary_readSkin(
            self_0,
            input,
            0 as c_int,
            skeletonData,
            nonessential,
        );
        i += 1;
    }
    i = 0 as c_int;
    while i < (*internal).linkedMeshCount {
        let mut linkedMesh: *mut _spLinkedMeshBinary = ((*internal).linkedMeshes)
            .offset(i as isize);
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
        parent_0 = spSkin_getAttachment(
            skin,
            (*linkedMesh).slotIndex,
            (*linkedMesh).parent,
        );
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
        (*(*linkedMesh).mesh)
            .super_0
            .deformAttachment = if (*linkedMesh).inheritDeform != 0 {
            parent_0 as *mut spVertexAttachment
        } else {
            (*linkedMesh).mesh as *mut spVertexAttachment
        };
        spMeshAttachment_setParentMesh(
            (*linkedMesh).mesh,
            parent_0 as *mut spMeshAttachment,
        );
        spMeshAttachment_updateUVs((*linkedMesh).mesh);
        spAttachmentLoader_configureAttachment(
            (*self_0).attachmentLoader,
            &mut (*(*linkedMesh).mesh).super_0.super_0,
        );
        i += 1;
    }
    (*skeletonData).eventsCount = readVarint(input, 1 as c_int);
    (*skeletonData)
        .events = _spMalloc(
        (::core::mem::size_of::<*mut spEventData>() as c_ulong)
            .wrapping_mul((*skeletonData).eventsCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        7790 as c_int,
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
        let ref mut fresh137 = *((*skeletonData).events).offset(i as isize);
        *fresh137 = eventData;
        i += 1;
    }
    (*skeletonData).animationsCount = readVarint(input, 1 as c_int);
    (*skeletonData)
        .animations = _spMalloc(
        (::core::mem::size_of::<*mut spAnimation>() as c_ulong)
            .wrapping_mul((*skeletonData).animationsCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        7807 as c_int,
    ) as *mut *mut spAnimation;
    i = 0 as c_int;
    while i < (*skeletonData).animationsCount {
        let mut name_4: *const c_char = readString(input);
        let mut animation: *mut spAnimation = _spSkeletonBinary_readAnimation(
            self_0,
            name_4,
            input,
            skeletonData,
        );
        _spFree(name_4 as *mut c_void);
        if animation.is_null() {
            _spFree(input as *mut c_void);
            spSkeletonData_dispose(skeletonData);
            return 0 as *mut spSkeletonData;
        }
        let ref mut fresh138 = *((*skeletonData).animations).offset(i as isize);
        *fresh138 = animation;
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
        7857 as c_int,
    ) as *mut spPolygon;
    (*self_0).capacity = capacity;
    let ref mut fresh139 = *(&(*self_0).vertices as *const *mut c_float
        as *mut *mut c_float);
    *fresh139 = _spMalloc(
        (::core::mem::size_of::<c_float>() as c_ulong)
            .wrapping_mul(capacity as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        7859 as c_int,
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
        let mut vertexY: c_float = *((*self_0).vertices)
            .offset((i + 1 as c_int) as isize);
        let mut prevY: c_float = *((*self_0).vertices)
            .offset((prevIndex + 1 as c_int) as isize);
        if vertexY < y && prevY >= y || prevY < y && vertexY >= y {
            let mut vertexX: c_float = *((*self_0).vertices).offset(i as isize);
            if vertexX
                + (y - vertexY) / (prevY - vertexY)
                    * (*((*self_0).vertices).offset(prevIndex as isize) - vertexX) < x
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
    let mut x3: c_float = *((*self_0).vertices)
        .offset(((*self_0).count - 2 as c_int) as isize);
    let mut y3: c_float = *((*self_0).vertices)
        .offset(((*self_0).count - 1 as c_int) as isize);
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*self_0).count {
        let mut x4: c_float = *((*self_0).vertices).offset(i as isize);
        let mut y4: c_float = *((*self_0).vertices)
            .offset((i + 1 as c_int) as isize);
        let mut det2: c_float = x3 * y4 - y3 * x4;
        let mut width34: c_float = x3 - x4;
        let mut height34: c_float = y3 - y4;
        let mut det3: c_float = width12 * height34 - height12 * width34;
        let mut x: c_float = (det1 * width34 - width12 * det2) / det3;
        if (x >= x3 && x <= x4 || x >= x4 && x <= x3)
            && (x >= x1 && x <= x2 || x >= x2 && x <= x1)
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
        as unsafe extern "C" fn(
            size_t,
            size_t,
            *const c_char,
            c_int,
        ) -> *mut c_void)(
        1 as c_int as size_t,
        ::core::mem::size_of::<_spSkeletonBounds>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        7913 as c_int,
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
        (*self_0)
            .boundingBoxes = _spMalloc(
            (::core::mem::size_of::<*mut spBoundingBoxAttachment>() as c_ulong)
                .wrapping_mul((*skeleton).slotsCount as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            7933 as c_int,
        ) as *mut *mut spBoundingBoxAttachment;
        newPolygons = _spCalloc(
            (*skeleton).slotsCount as size_t,
            ::core::mem::size_of::<*mut spPolygon>() as c_ulong,
            b"spine.c\0" as *const u8 as *const c_char,
            7935 as c_int,
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
        let mut boundingBox: *mut spBoundingBoxAttachment = 0
            as *mut spBoundingBoxAttachment;
        let mut attachment: *mut spAttachment = 0 as *mut spAttachment;
        let mut slot: *mut spSlot = *((*skeleton).slots).offset(i as isize);
        if !((*(*slot).bone).active == 0) {
            attachment = (*slot).attachment;
            if !(attachment.is_null()
                || (*attachment).type_0 as c_uint
                    != SP_ATTACHMENT_BOUNDING_BOX as c_int as c_uint)
            {
                boundingBox = attachment as *mut spBoundingBoxAttachment;
                let ref mut fresh140 = *((*self_0).boundingBoxes)
                    .offset((*self_0).count as isize);
                *fresh140 = boundingBox;
                polygon = *((*self_0).polygons).offset((*self_0).count as isize);
                if polygon.is_null()
                    || (*polygon).capacity < (*boundingBox).super_0.worldVerticesLength
                {
                    if !polygon.is_null() {
                        spPolygon_dispose(polygon);
                    }
                    polygon = spPolygon_create(
                        (*boundingBox).super_0.worldVerticesLength,
                    );
                    let ref mut fresh141 = *((*self_0).polygons)
                        .offset((*self_0).count as isize);
                    *fresh141 = polygon;
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
                        let mut x: c_float = *((*polygon).vertices)
                            .offset(ii as isize);
                        let mut y: c_float = *((*polygon).vertices)
                            .offset((ii + 1 as c_int) as isize);
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
    return (x >= (*self_0).minX && x <= (*self_0).maxX && y >= (*self_0).minY
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
    return ((*self_0).minX < (*bounds).maxX && (*self_0).maxX > (*bounds).minX
        && (*self_0).minY < (*bounds).maxY && (*self_0).maxY > (*bounds).minY)
        as c_int;
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
        if spPolygon_intersectsSegment(
            *((*self_0).polygons).offset(i as isize),
            x1,
            y1,
            x2,
            y2,
        ) != 0
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
        8065 as c_int,
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
pub unsafe extern "C" fn spSkeletonClipping_dispose(
    mut self_0: *mut spSkeletonClipping,
) {
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
    let mut area: c_float = *vertices
        .offset((verticeslength - 2 as c_int) as isize)
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
        *vertices
            .offset(
                (i + 1 as c_int) as isize,
            ) = *vertices.offset((other + 1 as c_int) as isize);
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
    (*self_0)
        .clippingPolygons = spTriangulator_decompose(
        (*self_0).triangulator,
        (*self_0).clippingPolygon,
        spTriangulator_triangulate((*self_0).triangulator, (*self_0).clippingPolygon),
    );
    i = 0 as c_int;
    n = (*(*self_0).clippingPolygons).size;
    while i < n {
        let mut polygon: *mut spFloatArray = *((*(*self_0).clippingPolygons).items)
            .offset(i as isize);
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
    if !((*self_0).clipAttachment).is_null()
        && (*(*self_0).clipAttachment).endSlot == (*slot).data
    {
        spSkeletonClipping_clipEnd2(self_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonClipping_clipEnd2(
    mut self_0: *mut spSkeletonClipping,
) {
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
        let mut edgeY: c_float = *clippingVertices
            .offset((i + 1 as c_int) as isize);
        let mut edgeX2: c_float = *clippingVertices
            .offset((i + 2 as c_int) as isize);
        let mut edgeY2: c_float = *clippingVertices
            .offset((i + 3 as c_int) as isize);
        let mut deltaX: c_float = edgeX - edgeX2;
        let mut deltaY: c_float = edgeY - edgeY2;
        let mut inputVertices: *mut c_float = (*input).items;
        let mut inputVerticesLength: c_int = (*input).size - 2 as c_int;
        let mut outputStart: c_int = (*output).size;
        let mut current_block_42: u64;
        ii = 0 as c_int;
        while ii < inputVerticesLength {
            let mut inputX: c_float = *inputVertices.offset(ii as isize);
            let mut inputY: c_float = *inputVertices
                .offset((ii + 1 as c_int) as isize);
            let mut inputX2: c_float = *inputVertices
                .offset((ii + 2 as c_int) as isize);
            let mut inputY2: c_float = *inputVertices
                .offset((ii + 3 as c_int) as isize);
            let mut side2: c_int = (deltaX * (inputY2 - edgeY2)
                - deltaY * (inputX2 - edgeX2) > 0 as c_int as c_float)
                as c_int;
            if deltaX * (inputY - edgeY2) - deltaY * (inputX - edgeX2)
                > 0 as c_int as c_float
            {
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
                    if (if s < 0 as c_int as c_float { -s } else { s })
                        > 0.000001f32
                    {
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
                    let mut s_0: c_float = c0_0 * (edgeX2 - edgeX)
                        - c2_0 * (edgeY2 - edgeY);
                    if (if s_0 < 0 as c_int as c_float { -s_0 } else { s_0 })
                        > 0.000001f32
                    {
                        let mut ua_0: c_float = (c2_0 * (edgeY - inputY)
                            - c0_0 * (edgeX - inputX)) / s_0;
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
        let mut vertexOffset: c_int = *triangles.offset(i as isize) as c_int
            * stride;
        let mut x2: c_float = 0.;
        let mut y2: c_float = 0.;
        let mut u2: c_float = 0.;
        let mut v2: c_float = 0.;
        let mut x3: c_float = 0.;
        let mut y3: c_float = 0.;
        let mut u3: c_float = 0.;
        let mut v3: c_float = 0.;
        let mut x1: c_float = *vertices.offset(vertexOffset as isize);
        let mut y1: c_float = *vertices
            .offset((vertexOffset + 1 as c_int) as isize);
        let mut u1: c_float = *uvs.offset(vertexOffset as isize);
        let mut v1: c_float = *uvs
            .offset((vertexOffset + 1 as c_int) as isize);
        vertexOffset = *triangles.offset((i + 1 as c_int) as isize) as c_int
            * stride;
        x2 = *vertices.offset(vertexOffset as isize);
        y2 = *vertices.offset((vertexOffset + 1 as c_int) as isize);
        u2 = *uvs.offset(vertexOffset as isize);
        v2 = *uvs.offset((vertexOffset + 1 as c_int) as isize);
        vertexOffset = *triangles.offset((i + 2 as c_int) as isize) as c_int
            * stride;
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
                let mut clippedTrianglesItems: *mut c_ushort = 0
                    as *mut c_ushort;
                let mut clipOutputCount: c_int = 0;
                let mut clipOutputItems: *mut c_float = 0 as *mut c_float;
                let mut clippedVerticesItems: *mut c_float = 0
                    as *mut c_float;
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
                    clippedUVsItems = (*spFloatArray_setSize(
                        clippedUVs,
                        s + (clipOutputCount << 1 as c_int),
                    ))
                        .items;
                    ii = 0 as c_int;
                    while ii < clipOutputLength {
                        let mut c0: c_float = 0.;
                        let mut c1: c_float = 0.;
                        let mut a: c_float = 0.;
                        let mut b: c_float = 0.;
                        let mut c: c_float = 0.;
                        let mut x: c_float = *clipOutputItems.offset(ii as isize);
                        let mut y: c_float = *clipOutputItems
                            .offset((ii + 1 as c_int) as isize);
                        *clippedVerticesItems.offset(s as isize) = x;
                        *clippedVerticesItems
                            .offset((s + 1 as c_int) as isize) = y;
                        c0 = x - x3;
                        c1 = y - y3;
                        a = (d0 * c0 + d1 * c1) * d;
                        b = (d4 * c0 + d2 * c1) * d;
                        c = 1 as c_int as c_float - a - b;
                        *clippedUVsItems.offset(s as isize) = u1 * a + u2 * b + u3 * c;
                        *clippedUVsItems
                            .offset(
                                (s + 1 as c_int) as isize,
                            ) = v1 * a + v2 * b + v3 * c;
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
                        *clippedTrianglesItems
                            .offset(s as isize) = index as c_ushort;
                        *clippedTrianglesItems
                            .offset(
                                (s + 1 as c_int) as isize,
                            ) = (index as c_int + ii) as c_ushort;
                        *clippedTrianglesItems
                            .offset(
                                (s + 2 as c_int) as isize,
                            ) = (index as c_int + ii + 1 as c_int)
                            as c_ushort;
                        s += 3 as c_int;
                        ii += 1;
                    }
                    index = (index as c_int + (clipOutputCount + 1 as c_int))
                        as c_short;
                }
                p += 1;
            } else {
                let mut clippedTrianglesItems_0: *mut c_ushort = 0
                    as *mut c_ushort;
                let mut clippedVerticesItems_0: *mut c_float = (*spFloatArray_setSize(
                    clippedVertices,
                    s + ((3 as c_int) << 1 as c_int),
                ))
                    .items;
                let mut clippedUVsItems_0: *mut c_float = (*spFloatArray_setSize(
                    clippedUVs,
                    s + ((3 as c_int) << 1 as c_int),
                ))
                    .items;
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
                clippedTrianglesItems_0 = (*spUnsignedShortArray_setSize(
                    clippedTriangles,
                    s + 3 as c_int,
                ))
                    .items;
                *clippedTrianglesItems_0.offset(s as isize) = index as c_ushort;
                *clippedTrianglesItems_0
                    .offset(
                        (s + 1 as c_int) as isize,
                    ) = (index as c_int + 1 as c_int) as c_ushort;
                *clippedTrianglesItems_0
                    .offset(
                        (s + 2 as c_int) as isize,
                    ) = (index as c_int + 2 as c_int) as c_ushort;
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
        8391 as c_int,
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
        spTransformConstraintData_dispose(
            *((*self_0).transformConstraints).offset(i as isize),
        );
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
        if spine_strcmp((**((*self_0).bones).offset(i as isize)).name, boneName)
            == 0 as c_int
        {
            return *((*self_0).bones).offset(i as isize);
        }
        i += 1;
    }
    return 0 as *mut spBoneData;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonData_findBoneIndex(
    mut self_0: *const spSkeletonData,
    mut boneName: *const c_char,
) -> c_int {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*self_0).bonesCount {
        if spine_strcmp((**((*self_0).bones).offset(i as isize)).name, boneName)
            == 0 as c_int
        {
            return i;
        }
        i += 1;
    }
    return -(1 as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonData_findSlot(
    mut self_0: *const spSkeletonData,
    mut slotName: *const c_char,
) -> *mut spSlotData {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*self_0).slotsCount {
        if spine_strcmp((**((*self_0).slots).offset(i as isize)).name, slotName)
            == 0 as c_int
        {
            return *((*self_0).slots).offset(i as isize);
        }
        i += 1;
    }
    return 0 as *mut spSlotData;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonData_findSlotIndex(
    mut self_0: *const spSkeletonData,
    mut slotName: *const c_char,
) -> c_int {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*self_0).slotsCount {
        if spine_strcmp((**((*self_0).slots).offset(i as isize)).name, slotName)
            == 0 as c_int
        {
            return i;
        }
        i += 1;
    }
    return -(1 as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonData_findSkin(
    mut self_0: *const spSkeletonData,
    mut skinName: *const c_char,
) -> *mut spSkin {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*self_0).skinsCount {
        if spine_strcmp((**((*self_0).skins).offset(i as isize)).name, skinName)
            == 0 as c_int
        {
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
        if spine_strcmp((**((*self_0).events).offset(i as isize)).name, eventName)
            == 0 as c_int
        {
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
        as unsafe extern "C" fn(
            size_t,
            size_t,
            *const c_char,
            c_int,
        ) -> *mut c_void)(
        1 as c_int as size_t,
        ::core::mem::size_of::<_spSkeletonJson>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        8566 as c_int,
    ) as *mut _spSkeletonJson))
        .super_0;
    (*self_0).scale = 1 as c_int as c_float;
    (*self_0).attachmentLoader = attachmentLoader;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spSkeletonJson_create(
    mut atlas: *mut spAtlas,
) -> *mut spSkeletonJson {
    let mut attachmentLoader: *mut spAtlasAttachmentLoader = spAtlasAttachmentLoader_create(
        atlas,
    );
    let mut self_0: *mut spSkeletonJson = spSkeletonJson_createWithLoader(
        &mut (*attachmentLoader).super_0,
    );
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
    let ref mut fresh142 = *(&(*self_0).error as *const *const c_char
        as *mut *mut c_char);
    *fresh142 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul(
                (spine_strlen(message.as_mut_ptr()))
                    .wrapping_add(1 as c_int as c_ulong),
            ),
        b"spine.c\0" as *const u8 as *const c_char,
        8594 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh142, message.as_mut_ptr());
    if !root.is_null() {
        Json_dispose(root);
    }
}
unsafe extern "C" fn toColor(
    mut value: *const c_char,
    mut index: c_int,
) -> c_float {
    let mut digits: [c_char; 3] = [0; 3];
    let mut error: *mut c_char = 0 as *mut c_char;
    let mut color: c_int = 0;
    if index as size_t
        >= (spine_strlen(value)).wrapping_div(2 as c_int as c_ulong)
    {
        return -(1 as c_int) as c_float;
    }
    value = value.offset((index * 2 as c_int) as isize);
    digits[0 as c_int as usize] = *value;
    digits[1 as c_int as usize] = *value.offset(1 as c_int as isize);
    digits[2 as c_int as usize] = '\0' as i32 as c_char;
    color = spine_strtoul(digits.as_mut_ptr(), &mut error, 16 as c_int)
        as c_int;
    if *error as c_int != 0 as c_int {
        return -(1 as c_int) as c_float;
    }
    return color as c_float / 255 as c_int as c_float;
}
unsafe extern "C" fn readCurveJson(
    mut frame: *mut Json,
    mut timeline: *mut spCurveTimeline,
    mut frameIndex: c_int,
) {
    let mut curve: *mut Json = Json_getItem(
        frame,
        b"curve\0" as *const u8 as *const c_char,
    );
    if curve.is_null() {
        return;
    }
    if (*curve).type_0 == 4 as c_int
        && spine_strcmp(
            (*curve).valueString,
            b"stepped\0" as *const u8 as *const c_char,
        ) == 0 as c_int
    {
        spCurveTimeline_setStepped(timeline, frameIndex);
    } else {
        let mut c1: c_float = Json_getFloat(
            frame,
            b"curve\0" as *const u8 as *const c_char,
            0 as c_int as c_float,
        );
        let mut c2: c_float = Json_getFloat(
            frame,
            b"c2\0" as *const u8 as *const c_char,
            0 as c_int as c_float,
        );
        let mut c3: c_float = Json_getFloat(
            frame,
            b"c3\0" as *const u8 as *const c_char,
            1 as c_int as c_float,
        );
        let mut c4: c_float = Json_getFloat(
            frame,
            b"c4\0" as *const u8 as *const c_char,
            1 as c_int as c_float,
        );
        spCurveTimeline_setCurve(timeline, frameIndex, c1, c2, c3, c4);
    };
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
            8638 as c_int,
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
    let fresh143 = (*internal).linkedMeshCount;
    (*internal).linkedMeshCount = (*internal).linkedMeshCount + 1;
    linkedMesh = ((*internal).linkedMeshes).offset(fresh143 as isize);
    (*linkedMesh).mesh = mesh;
    (*linkedMesh).skin = skin;
    (*linkedMesh).slotIndex = slotIndex;
    (*linkedMesh).parent = parent;
    (*linkedMesh).inheritDeform = inheritDeform;
}
unsafe extern "C" fn _spSkeletonJson_readAnimation(
    mut self_0: *mut spSkeletonJson,
    mut root: *mut Json,
    mut skeletonData: *mut spSkeletonData,
) -> *mut spAnimation {
    let mut frameIndex: c_int = 0;
    let mut animation: *mut spAnimation = 0 as *mut spAnimation;
    let mut valueMap: *mut Json = 0 as *mut Json;
    let mut timelinesCount: c_int = 0 as c_int;
    let mut bones: *mut Json = Json_getItem(
        root,
        b"bones\0" as *const u8 as *const c_char,
    );
    let mut slots: *mut Json = Json_getItem(
        root,
        b"slots\0" as *const u8 as *const c_char,
    );
    let mut ik: *mut Json = Json_getItem(
        root,
        b"ik\0" as *const u8 as *const c_char,
    );
    let mut transform: *mut Json = Json_getItem(
        root,
        b"transform\0" as *const u8 as *const c_char,
    );
    let mut path: *mut Json = Json_getItem(
        root,
        b"path\0" as *const u8 as *const c_char,
    );
    let mut deformJson: *mut Json = Json_getItem(
        root,
        b"deform\0" as *const u8 as *const c_char,
    );
    let mut drawOrderJson: *mut Json = Json_getItem(
        root,
        b"drawOrder\0" as *const u8 as *const c_char,
    );
    let mut events: *mut Json = Json_getItem(
        root,
        b"events\0" as *const u8 as *const c_char,
    );
    let mut boneMap: *mut Json = 0 as *mut Json;
    let mut slotMap: *mut Json = 0 as *mut Json;
    let mut constraintMap: *mut Json = 0 as *mut Json;
    if drawOrderJson.is_null() {
        drawOrderJson = Json_getItem(
            root,
            b"draworder\0" as *const u8 as *const c_char,
        );
    }
    boneMap = if !bones.is_null() { (*bones).child } else { 0 as *mut Json };
    while !boneMap.is_null() {
        timelinesCount += (*boneMap).size;
        boneMap = (*boneMap).next;
    }
    slotMap = if !slots.is_null() { (*slots).child } else { 0 as *mut Json };
    while !slotMap.is_null() {
        timelinesCount += (*slotMap).size;
        slotMap = (*slotMap).next;
    }
    timelinesCount += if !ik.is_null() { (*ik).size } else { 0 as c_int };
    timelinesCount
        += if !transform.is_null() { (*transform).size } else { 0 as c_int };
    constraintMap = if !path.is_null() { (*path).child } else { 0 as *mut Json };
    while !constraintMap.is_null() {
        timelinesCount += (*constraintMap).size;
        constraintMap = (*constraintMap).next;
    }
    constraintMap = if !deformJson.is_null() {
        (*deformJson).child
    } else {
        0 as *mut Json
    };
    while !constraintMap.is_null() {
        slotMap = (*constraintMap).child;
        while !slotMap.is_null() {
            timelinesCount += (*slotMap).size;
            slotMap = (*slotMap).next;
        }
        constraintMap = (*constraintMap).next;
    }
    if !drawOrderJson.is_null() {
        timelinesCount += 1;
    }
    if !events.is_null() {
        timelinesCount += 1;
    }
    animation = spAnimation_create((*root).name, timelinesCount);
    (*animation).timelinesCount = 0 as c_int;
    slotMap = if !slots.is_null() { (*slots).child } else { 0 as *mut Json };
    while !slotMap.is_null() {
        let mut timelineMap: *mut Json = 0 as *mut Json;
        let mut slotIndex: c_int = spSkeletonData_findSlotIndex(
            skeletonData,
            (*slotMap).name,
        );
        if slotIndex == -(1 as c_int) {
            spAnimation_dispose(animation);
            _spSkeletonJson_setError(
                self_0,
                root,
                b"Slot not found: \0" as *const u8 as *const c_char,
                (*slotMap).name,
            );
            return 0 as *mut spAnimation;
        }
        timelineMap = (*slotMap).child;
        while !timelineMap.is_null() {
            if spine_strcmp(
                (*timelineMap).name,
                b"attachment\0" as *const u8 as *const c_char,
            ) == 0 as c_int
            {
                let mut timeline: *mut spAttachmentTimeline = spAttachmentTimeline_create(
                    (*timelineMap).size,
                );
                (*timeline).slotIndex = slotIndex;
                valueMap = (*timelineMap).child;
                frameIndex = 0 as c_int;
                while !valueMap.is_null() {
                    let mut name: *mut Json = Json_getItem(
                        valueMap,
                        b"name\0" as *const u8 as *const c_char,
                    );
                    spAttachmentTimeline_setFrame(
                        timeline,
                        frameIndex,
                        Json_getFloat(
                            valueMap,
                            b"time\0" as *const u8 as *const c_char,
                            0 as c_int as c_float,
                        ),
                        if (*name).type_0 == 2 as c_int {
                            0 as *const c_char
                        } else {
                            (*name).valueString
                        },
                    );
                    valueMap = (*valueMap).next;
                    frameIndex += 1;
                }
                let fresh144 = (*animation).timelinesCount;
                (*animation).timelinesCount = (*animation).timelinesCount + 1;
                let ref mut fresh145 = *((*animation).timelines)
                    .offset(fresh144 as isize);
                *fresh145 = timeline as *mut spTimeline;
                (*animation)
                    .duration = if (*animation).duration
                    > *((*timeline).frames)
                        .offset(((*timelineMap).size - 1 as c_int) as isize)
                {
                    (*animation).duration
                } else {
                    *((*timeline).frames)
                        .offset(((*timelineMap).size - 1 as c_int) as isize)
                };
            } else if spine_strcmp(
                    (*timelineMap).name,
                    b"color\0" as *const u8 as *const c_char,
                ) == 0 as c_int
                {
                let mut timeline_0: *mut spColorTimeline = spColorTimeline_create(
                    (*timelineMap).size,
                );
                (*timeline_0).slotIndex = slotIndex;
                valueMap = (*timelineMap).child;
                frameIndex = 0 as c_int;
                while !valueMap.is_null() {
                    let mut s: *const c_char = Json_getString(
                        valueMap,
                        b"color\0" as *const u8 as *const c_char,
                        0 as *const c_char,
                    );
                    spColorTimeline_setFrame(
                        timeline_0,
                        frameIndex,
                        Json_getFloat(
                            valueMap,
                            b"time\0" as *const u8 as *const c_char,
                            0 as c_int as c_float,
                        ),
                        toColor(s, 0 as c_int),
                        toColor(s, 1 as c_int),
                        toColor(s, 2 as c_int),
                        toColor(s, 3 as c_int),
                    );
                    readCurveJson(valueMap, &mut (*timeline_0).super_0, frameIndex);
                    valueMap = (*valueMap).next;
                    frameIndex += 1;
                }
                let fresh146 = (*animation).timelinesCount;
                (*animation).timelinesCount = (*animation).timelinesCount + 1;
                let ref mut fresh147 = *((*animation).timelines)
                    .offset(fresh146 as isize);
                *fresh147 = timeline_0 as *mut spTimeline;
                (*animation)
                    .duration = if (*animation).duration
                    > *((*timeline_0).frames)
                        .offset(
                            (((*timelineMap).size - 1 as c_int) * COLOR_ENTRIES)
                                as isize,
                        )
                {
                    (*animation).duration
                } else {
                    *((*timeline_0).frames)
                        .offset(
                            (((*timelineMap).size - 1 as c_int) * COLOR_ENTRIES)
                                as isize,
                        )
                };
            } else if spine_strcmp(
                    (*timelineMap).name,
                    b"twoColor\0" as *const u8 as *const c_char,
                ) == 0 as c_int
                {
                let mut timeline_1: *mut spTwoColorTimeline = spTwoColorTimeline_create(
                    (*timelineMap).size,
                );
                (*timeline_1).slotIndex = slotIndex;
                valueMap = (*timelineMap).child;
                frameIndex = 0 as c_int;
                while !valueMap.is_null() {
                    let mut s_0: *const c_char = Json_getString(
                        valueMap,
                        b"light\0" as *const u8 as *const c_char,
                        0 as *const c_char,
                    );
                    let mut ds: *const c_char = Json_getString(
                        valueMap,
                        b"dark\0" as *const u8 as *const c_char,
                        0 as *const c_char,
                    );
                    spTwoColorTimeline_setFrame(
                        timeline_1,
                        frameIndex,
                        Json_getFloat(
                            valueMap,
                            b"time\0" as *const u8 as *const c_char,
                            0 as c_int as c_float,
                        ),
                        toColor(s_0, 0 as c_int),
                        toColor(s_0, 1 as c_int),
                        toColor(s_0, 2 as c_int),
                        toColor(s_0, 3 as c_int),
                        toColor(ds, 0 as c_int),
                        toColor(ds, 1 as c_int),
                        toColor(ds, 2 as c_int),
                    );
                    readCurveJson(valueMap, &mut (*timeline_1).super_0, frameIndex);
                    valueMap = (*valueMap).next;
                    frameIndex += 1;
                }
                let fresh148 = (*animation).timelinesCount;
                (*animation).timelinesCount = (*animation).timelinesCount + 1;
                let ref mut fresh149 = *((*animation).timelines)
                    .offset(fresh148 as isize);
                *fresh149 = timeline_1 as *mut spTimeline;
                (*animation)
                    .duration = if (*animation).duration
                    > *((*timeline_1).frames)
                        .offset(
                            (((*timelineMap).size - 1 as c_int) * TWOCOLOR_ENTRIES)
                                as isize,
                        )
                {
                    (*animation).duration
                } else {
                    *((*timeline_1).frames)
                        .offset(
                            (((*timelineMap).size - 1 as c_int) * TWOCOLOR_ENTRIES)
                                as isize,
                        )
                };
            } else {
                spAnimation_dispose(animation);
                _spSkeletonJson_setError(
                    self_0,
                    0 as *mut Json,
                    b"Invalid timeline type for a slot: \0" as *const u8
                        as *const c_char,
                    (*timelineMap).name,
                );
                return 0 as *mut spAnimation;
            }
            timelineMap = (*timelineMap).next;
        }
        slotMap = (*slotMap).next;
    }
    boneMap = if !bones.is_null() { (*bones).child } else { 0 as *mut Json };
    while !boneMap.is_null() {
        let mut timelineMap_0: *mut Json = 0 as *mut Json;
        let mut boneIndex: c_int = spSkeletonData_findBoneIndex(
            skeletonData,
            (*boneMap).name,
        );
        if boneIndex == -(1 as c_int) {
            spAnimation_dispose(animation);
            _spSkeletonJson_setError(
                self_0,
                root,
                b"Bone not found: \0" as *const u8 as *const c_char,
                (*boneMap).name,
            );
            return 0 as *mut spAnimation;
        }
        timelineMap_0 = (*boneMap).child;
        while !timelineMap_0.is_null() {
            if spine_strcmp(
                (*timelineMap_0).name,
                b"rotate\0" as *const u8 as *const c_char,
            ) == 0 as c_int
            {
                let mut timeline_2: *mut spRotateTimeline = spRotateTimeline_create(
                    (*timelineMap_0).size,
                );
                (*timeline_2).boneIndex = boneIndex;
                valueMap = (*timelineMap_0).child;
                frameIndex = 0 as c_int;
                while !valueMap.is_null() {
                    spRotateTimeline_setFrame(
                        timeline_2,
                        frameIndex,
                        Json_getFloat(
                            valueMap,
                            b"time\0" as *const u8 as *const c_char,
                            0 as c_int as c_float,
                        ),
                        Json_getFloat(
                            valueMap,
                            b"angle\0" as *const u8 as *const c_char,
                            0 as c_int as c_float,
                        ),
                    );
                    readCurveJson(valueMap, &mut (*timeline_2).super_0, frameIndex);
                    valueMap = (*valueMap).next;
                    frameIndex += 1;
                }
                let fresh150 = (*animation).timelinesCount;
                (*animation).timelinesCount = (*animation).timelinesCount + 1;
                let ref mut fresh151 = *((*animation).timelines)
                    .offset(fresh150 as isize);
                *fresh151 = timeline_2 as *mut spTimeline;
                (*animation)
                    .duration = if (*animation).duration
                    > *((*timeline_2).frames)
                        .offset(
                            (((*timelineMap_0).size - 1 as c_int) * ROTATE_ENTRIES)
                                as isize,
                        )
                {
                    (*animation).duration
                } else {
                    *((*timeline_2).frames)
                        .offset(
                            (((*timelineMap_0).size - 1 as c_int) * ROTATE_ENTRIES)
                                as isize,
                        )
                };
            } else {
                let mut isScale: c_int = (spine_strcmp(
                    (*timelineMap_0).name,
                    b"scale\0" as *const u8 as *const c_char,
                ) == 0 as c_int) as c_int;
                let mut isTranslate: c_int = (spine_strcmp(
                    (*timelineMap_0).name,
                    b"translate\0" as *const u8 as *const c_char,
                ) == 0 as c_int) as c_int;
                let mut isShear: c_int = (spine_strcmp(
                    (*timelineMap_0).name,
                    b"shear\0" as *const u8 as *const c_char,
                ) == 0 as c_int) as c_int;
                if isScale != 0 || isTranslate != 0 || isShear != 0 {
                    let mut defaultValue: c_float = 0 as c_int
                        as c_float;
                    let mut timelineScale: c_float = if isTranslate != 0 {
                        (*self_0).scale
                    } else {
                        1 as c_int as c_float
                    };
                    let mut timeline_3: *mut spTranslateTimeline = 0
                        as *mut spTranslateTimeline;
                    if isScale != 0 {
                        timeline_3 = spScaleTimeline_create((*timelineMap_0).size);
                        defaultValue = 1 as c_int as c_float;
                    } else if isTranslate != 0 {
                        timeline_3 = spTranslateTimeline_create((*timelineMap_0).size);
                    } else if isShear != 0 {
                        timeline_3 = spShearTimeline_create((*timelineMap_0).size);
                    }
                    (*timeline_3).boneIndex = boneIndex;
                    valueMap = (*timelineMap_0).child;
                    frameIndex = 0 as c_int;
                    while !valueMap.is_null() {
                        spTranslateTimeline_setFrame(
                            timeline_3,
                            frameIndex,
                            Json_getFloat(
                                valueMap,
                                b"time\0" as *const u8 as *const c_char,
                                0 as c_int as c_float,
                            ),
                            Json_getFloat(
                                valueMap,
                                b"x\0" as *const u8 as *const c_char,
                                defaultValue,
                            ) * timelineScale,
                            Json_getFloat(
                                valueMap,
                                b"y\0" as *const u8 as *const c_char,
                                defaultValue,
                            ) * timelineScale,
                        );
                        readCurveJson(valueMap, &mut (*timeline_3).super_0, frameIndex);
                        valueMap = (*valueMap).next;
                        frameIndex += 1;
                    }
                    let fresh152 = (*animation).timelinesCount;
                    (*animation).timelinesCount = (*animation).timelinesCount + 1;
                    let ref mut fresh153 = *((*animation).timelines)
                        .offset(fresh152 as isize);
                    *fresh153 = timeline_3 as *mut spTimeline;
                    (*animation)
                        .duration = if (*animation).duration
                        > *((*timeline_3).frames)
                            .offset(
                                (((*timelineMap_0).size - 1 as c_int)
                                    * TRANSLATE_ENTRIES) as isize,
                            )
                    {
                        (*animation).duration
                    } else {
                        *((*timeline_3).frames)
                            .offset(
                                (((*timelineMap_0).size - 1 as c_int)
                                    * TRANSLATE_ENTRIES) as isize,
                            )
                    };
                } else {
                    spAnimation_dispose(animation);
                    _spSkeletonJson_setError(
                        self_0,
                        0 as *mut Json,
                        b"Invalid timeline type for a bone: \0" as *const u8
                            as *const c_char,
                        (*timelineMap_0).name,
                    );
                    return 0 as *mut spAnimation;
                }
            }
            timelineMap_0 = (*timelineMap_0).next;
        }
        boneMap = (*boneMap).next;
    }
    constraintMap = if !ik.is_null() { (*ik).child } else { 0 as *mut Json };
    while !constraintMap.is_null() {
        let mut constraint: *mut spIkConstraintData = spSkeletonData_findIkConstraint(
            skeletonData,
            (*constraintMap).name,
        );
        let mut timeline_4: *mut spIkConstraintTimeline = spIkConstraintTimeline_create(
            (*constraintMap).size,
        );
        frameIndex = 0 as c_int;
        while frameIndex < (*skeletonData).ikConstraintsCount {
            if constraint == *((*skeletonData).ikConstraints).offset(frameIndex as isize)
            {
                (*timeline_4).ikConstraintIndex = frameIndex;
                break;
            } else {
                frameIndex += 1;
            }
        }
        valueMap = (*constraintMap).child;
        frameIndex = 0 as c_int;
        while !valueMap.is_null() {
            spIkConstraintTimeline_setFrame(
                timeline_4,
                frameIndex,
                Json_getFloat(
                    valueMap,
                    b"time\0" as *const u8 as *const c_char,
                    0 as c_int as c_float,
                ),
                Json_getFloat(
                    valueMap,
                    b"mix\0" as *const u8 as *const c_char,
                    1 as c_int as c_float,
                ),
                Json_getFloat(
                    valueMap,
                    b"softness\0" as *const u8 as *const c_char,
                    0 as c_int as c_float,
                ) * (*self_0).scale,
                if Json_getInt(
                    valueMap,
                    b"bendPositive\0" as *const u8 as *const c_char,
                    1 as c_int,
                ) != 0
                {
                    1 as c_int
                } else {
                    -(1 as c_int)
                },
                if Json_getInt(
                    valueMap,
                    b"compress\0" as *const u8 as *const c_char,
                    0 as c_int,
                ) != 0
                {
                    1 as c_int
                } else {
                    0 as c_int
                },
                if Json_getInt(
                    valueMap,
                    b"stretch\0" as *const u8 as *const c_char,
                    0 as c_int,
                ) != 0
                {
                    1 as c_int
                } else {
                    0 as c_int
                },
            );
            readCurveJson(valueMap, &mut (*timeline_4).super_0, frameIndex);
            valueMap = (*valueMap).next;
            frameIndex += 1;
        }
        let fresh154 = (*animation).timelinesCount;
        (*animation).timelinesCount = (*animation).timelinesCount + 1;
        let ref mut fresh155 = *((*animation).timelines).offset(fresh154 as isize);
        *fresh155 = timeline_4 as *mut spTimeline;
        (*animation)
            .duration = if (*animation).duration
            > *((*timeline_4).frames)
                .offset(
                    (((*constraintMap).size - 1 as c_int) * IKCONSTRAINT_ENTRIES)
                        as isize,
                )
        {
            (*animation).duration
        } else {
            *((*timeline_4).frames)
                .offset(
                    (((*constraintMap).size - 1 as c_int) * IKCONSTRAINT_ENTRIES)
                        as isize,
                )
        };
        constraintMap = (*constraintMap).next;
    }
    constraintMap = if !transform.is_null() {
        (*transform).child
    } else {
        0 as *mut Json
    };
    while !constraintMap.is_null() {
        let mut constraint_0: *mut spTransformConstraintData = spSkeletonData_findTransformConstraint(
            skeletonData,
            (*constraintMap).name,
        );
        let mut timeline_5: *mut spTransformConstraintTimeline = spTransformConstraintTimeline_create(
            (*constraintMap).size,
        );
        frameIndex = 0 as c_int;
        while frameIndex < (*skeletonData).transformConstraintsCount {
            if constraint_0
                == *((*skeletonData).transformConstraints).offset(frameIndex as isize)
            {
                (*timeline_5).transformConstraintIndex = frameIndex;
                break;
            } else {
                frameIndex += 1;
            }
        }
        valueMap = (*constraintMap).child;
        frameIndex = 0 as c_int;
        while !valueMap.is_null() {
            spTransformConstraintTimeline_setFrame(
                timeline_5,
                frameIndex,
                Json_getFloat(
                    valueMap,
                    b"time\0" as *const u8 as *const c_char,
                    0 as c_int as c_float,
                ),
                Json_getFloat(
                    valueMap,
                    b"rotateMix\0" as *const u8 as *const c_char,
                    1 as c_int as c_float,
                ),
                Json_getFloat(
                    valueMap,
                    b"translateMix\0" as *const u8 as *const c_char,
                    1 as c_int as c_float,
                ),
                Json_getFloat(
                    valueMap,
                    b"scaleMix\0" as *const u8 as *const c_char,
                    1 as c_int as c_float,
                ),
                Json_getFloat(
                    valueMap,
                    b"shearMix\0" as *const u8 as *const c_char,
                    1 as c_int as c_float,
                ),
            );
            readCurveJson(valueMap, &mut (*timeline_5).super_0, frameIndex);
            valueMap = (*valueMap).next;
            frameIndex += 1;
        }
        let fresh156 = (*animation).timelinesCount;
        (*animation).timelinesCount = (*animation).timelinesCount + 1;
        let ref mut fresh157 = *((*animation).timelines).offset(fresh156 as isize);
        *fresh157 = timeline_5 as *mut spTimeline;
        (*animation)
            .duration = if (*animation).duration
            > *((*timeline_5).frames)
                .offset(
                    (((*constraintMap).size - 1 as c_int)
                        * TRANSFORMCONSTRAINT_ENTRIES) as isize,
                )
        {
            (*animation).duration
        } else {
            *((*timeline_5).frames)
                .offset(
                    (((*constraintMap).size - 1 as c_int)
                        * TRANSFORMCONSTRAINT_ENTRIES) as isize,
                )
        };
        constraintMap = (*constraintMap).next;
    }
    constraintMap = if !path.is_null() { (*path).child } else { 0 as *mut Json };
    while !constraintMap.is_null() {
        let mut constraintIndex: c_int = 0;
        let mut i: c_int = 0;
        let mut timelineMap_1: *mut Json = 0 as *mut Json;
        let mut data: *mut spPathConstraintData = spSkeletonData_findPathConstraint(
            skeletonData,
            (*constraintMap).name,
        );
        if data.is_null() {
            spAnimation_dispose(animation);
            _spSkeletonJson_setError(
                self_0,
                root,
                b"Path constraint not found: \0" as *const u8 as *const c_char,
                (*constraintMap).name,
            );
            return 0 as *mut spAnimation;
        }
        i = 0 as c_int;
        while i < (*skeletonData).pathConstraintsCount {
            if *((*skeletonData).pathConstraints).offset(i as isize) == data {
                constraintIndex = i;
                break;
            } else {
                i += 1;
            }
        }
        timelineMap_1 = (*constraintMap).child;
        while !timelineMap_1.is_null() {
            let mut timelineName: *const c_char = (*timelineMap_1).name;
            if spine_strcmp(
                timelineName,
                b"position\0" as *const u8 as *const c_char,
            ) == 0 as c_int
                || spine_strcmp(
                    timelineName,
                    b"spacing\0" as *const u8 as *const c_char,
                ) == 0 as c_int
            {
                let mut timeline_6: *mut spPathConstraintPositionTimeline = 0
                    as *mut spPathConstraintPositionTimeline;
                let mut timelineScale_0: c_float = 1 as c_int
                    as c_float;
                if spine_strcmp(
                    timelineName,
                    b"spacing\0" as *const u8 as *const c_char,
                ) == 0 as c_int
                {
                    timeline_6 = spPathConstraintSpacingTimeline_create(
                        (*timelineMap_1).size,
                    ) as *mut spPathConstraintPositionTimeline;
                    if (*data).spacingMode as c_uint
                        == SP_SPACING_MODE_LENGTH as c_int as c_uint
                        || (*data).spacingMode as c_uint
                            == SP_SPACING_MODE_FIXED as c_int as c_uint
                    {
                        timelineScale_0 = (*self_0).scale;
                    }
                } else {
                    timeline_6 = spPathConstraintPositionTimeline_create(
                        (*timelineMap_1).size,
                    );
                    if (*data).positionMode as c_uint
                        == SP_POSITION_MODE_FIXED as c_int as c_uint
                    {
                        timelineScale_0 = (*self_0).scale;
                    }
                }
                (*timeline_6).pathConstraintIndex = constraintIndex;
                valueMap = (*timelineMap_1).child;
                frameIndex = 0 as c_int;
                while !valueMap.is_null() {
                    spPathConstraintPositionTimeline_setFrame(
                        timeline_6,
                        frameIndex,
                        Json_getFloat(
                            valueMap,
                            b"time\0" as *const u8 as *const c_char,
                            0 as c_int as c_float,
                        ),
                        Json_getFloat(
                            valueMap,
                            timelineName,
                            0 as c_int as c_float,
                        ) * timelineScale_0,
                    );
                    readCurveJson(valueMap, &mut (*timeline_6).super_0, frameIndex);
                    valueMap = (*valueMap).next;
                    frameIndex += 1;
                }
                let fresh158 = (*animation).timelinesCount;
                (*animation).timelinesCount = (*animation).timelinesCount + 1;
                let ref mut fresh159 = *((*animation).timelines)
                    .offset(fresh158 as isize);
                *fresh159 = timeline_6 as *mut spTimeline;
                (*animation)
                    .duration = if (*animation).duration
                    > *((*timeline_6).frames)
                        .offset(
                            (((*timelineMap_1).size - 1 as c_int)
                                * PATHCONSTRAINTPOSITION_ENTRIES) as isize,
                        )
                {
                    (*animation).duration
                } else {
                    *((*timeline_6).frames)
                        .offset(
                            (((*timelineMap_1).size - 1 as c_int)
                                * PATHCONSTRAINTPOSITION_ENTRIES) as isize,
                        )
                };
            } else if spine_strcmp(
                    timelineName,
                    b"mix\0" as *const u8 as *const c_char,
                ) == 0 as c_int
                {
                let mut timeline_7: *mut spPathConstraintMixTimeline = spPathConstraintMixTimeline_create(
                    (*timelineMap_1).size,
                );
                (*timeline_7).pathConstraintIndex = constraintIndex;
                valueMap = (*timelineMap_1).child;
                frameIndex = 0 as c_int;
                while !valueMap.is_null() {
                    spPathConstraintMixTimeline_setFrame(
                        timeline_7,
                        frameIndex,
                        Json_getFloat(
                            valueMap,
                            b"time\0" as *const u8 as *const c_char,
                            0 as c_int as c_float,
                        ),
                        Json_getFloat(
                            valueMap,
                            b"rotateMix\0" as *const u8 as *const c_char,
                            1 as c_int as c_float,
                        ),
                        Json_getFloat(
                            valueMap,
                            b"translateMix\0" as *const u8 as *const c_char,
                            1 as c_int as c_float,
                        ),
                    );
                    readCurveJson(valueMap, &mut (*timeline_7).super_0, frameIndex);
                    valueMap = (*valueMap).next;
                    frameIndex += 1;
                }
                let fresh160 = (*animation).timelinesCount;
                (*animation).timelinesCount = (*animation).timelinesCount + 1;
                let ref mut fresh161 = *((*animation).timelines)
                    .offset(fresh160 as isize);
                *fresh161 = timeline_7 as *mut spTimeline;
                (*animation)
                    .duration = if (*animation).duration
                    > *((*timeline_7).frames)
                        .offset(
                            (((*timelineMap_1).size - 1 as c_int)
                                * PATHCONSTRAINTMIX_ENTRIES) as isize,
                        )
                {
                    (*animation).duration
                } else {
                    *((*timeline_7).frames)
                        .offset(
                            (((*timelineMap_1).size - 1 as c_int)
                                * PATHCONSTRAINTMIX_ENTRIES) as isize,
                        )
                };
            }
            timelineMap_1 = (*timelineMap_1).next;
        }
        constraintMap = (*constraintMap).next;
    }
    constraintMap = if !deformJson.is_null() {
        (*deformJson).child
    } else {
        0 as *mut Json
    };
    while !constraintMap.is_null() {
        let mut skin: *mut spSkin = spSkeletonData_findSkin(
            skeletonData,
            (*constraintMap).name,
        );
        slotMap = (*constraintMap).child;
        while !slotMap.is_null() {
            let mut slotIndex_0: c_int = spSkeletonData_findSlotIndex(
                skeletonData,
                (*slotMap).name,
            );
            let mut timelineMap_2: *mut Json = 0 as *mut Json;
            timelineMap_2 = (*slotMap).child;
            while !timelineMap_2.is_null() {
                let mut tempDeform: *mut c_float = 0 as *mut c_float;
                let mut timeline_8: *mut spDeformTimeline = 0 as *mut spDeformTimeline;
                let mut weighted: c_int = 0;
                let mut deformLength: c_int = 0;
                let mut attachment: *mut spVertexAttachment = spSkin_getAttachment(
                    skin,
                    slotIndex_0,
                    (*timelineMap_2).name,
                ) as *mut spVertexAttachment;
                if attachment.is_null() {
                    spAnimation_dispose(animation);
                    _spSkeletonJson_setError(
                        self_0,
                        0 as *mut Json,
                        b"Attachment not found: \0" as *const u8 as *const c_char,
                        (*timelineMap_2).name,
                    );
                    return 0 as *mut spAnimation;
                }
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
                    8910 as c_int,
                ) as *mut c_float;
                timeline_8 = spDeformTimeline_create(
                    (*timelineMap_2).size,
                    deformLength,
                );
                (*timeline_8).slotIndex = slotIndex_0;
                (*timeline_8).attachment = &mut (*attachment).super_0;
                valueMap = (*timelineMap_2).child;
                frameIndex = 0 as c_int;
                while !valueMap.is_null() {
                    let mut deform: *mut c_float = 0 as *mut c_float;
                    let mut vertices: *mut Json = Json_getItem(
                        valueMap,
                        b"vertices\0" as *const u8 as *const c_char,
                    );
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
                            deform = (*attachment).vertices;
                        }
                    } else {
                        let mut v: c_int = 0;
                        let mut start: c_int = Json_getInt(
                            valueMap,
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
                                *deform
                                    .offset(
                                        v as isize,
                                    ) = (*vertex).valueFloat * (*self_0).scale;
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
                            let mut verticesValues: *mut c_float = (*attachment)
                                .vertices;
                            v = 0 as c_int;
                            while v < deformLength {
                                *deform.offset(v as isize)
                                    += *verticesValues.offset(v as isize);
                                v += 1;
                            }
                        }
                    }
                    spDeformTimeline_setFrame(
                        timeline_8,
                        frameIndex,
                        Json_getFloat(
                            valueMap,
                            b"time\0" as *const u8 as *const c_char,
                            0 as c_int as c_float,
                        ),
                        deform,
                    );
                    readCurveJson(valueMap, &mut (*timeline_8).super_0, frameIndex);
                    valueMap = (*valueMap).next;
                    frameIndex += 1;
                }
                _spFree(tempDeform as *mut c_void);
                let fresh162 = (*animation).timelinesCount;
                (*animation).timelinesCount = (*animation).timelinesCount + 1;
                let ref mut fresh163 = *((*animation).timelines)
                    .offset(fresh162 as isize);
                *fresh163 = timeline_8 as *mut spTimeline;
                (*animation)
                    .duration = if (*animation).duration
                    > *((*timeline_8).frames)
                        .offset(((*timelineMap_2).size - 1 as c_int) as isize)
                {
                    (*animation).duration
                } else {
                    *((*timeline_8).frames)
                        .offset(((*timelineMap_2).size - 1 as c_int) as isize)
                };
                timelineMap_2 = (*timelineMap_2).next;
            }
            slotMap = (*slotMap).next;
        }
        constraintMap = (*constraintMap).next;
    }
    if !drawOrderJson.is_null() {
        let mut timeline_9: *mut spDrawOrderTimeline = spDrawOrderTimeline_create(
            (*drawOrderJson).size,
            (*skeletonData).slotsCount,
        );
        valueMap = (*drawOrderJson).child;
        frameIndex = 0 as c_int;
        while !valueMap.is_null() {
            let mut ii: c_int = 0;
            let mut drawOrder: *mut c_int = 0 as *mut c_int;
            let mut offsets: *mut Json = Json_getItem(
                valueMap,
                b"offsets\0" as *const u8 as *const c_char,
            );
            if !offsets.is_null() {
                let mut offsetMap: *mut Json = 0 as *mut Json;
                let mut unchanged: *mut c_int = _spMalloc(
                    (::core::mem::size_of::<c_int>() as c_ulong)
                        .wrapping_mul(
                            ((*skeletonData).slotsCount - (*offsets).size)
                                as c_ulong,
                        ),
                    b"spine.c\0" as *const u8 as *const c_char,
                    8964 as c_int,
                ) as *mut c_int;
                let mut originalIndex: c_int = 0 as c_int;
                let mut unchangedIndex: c_int = 0 as c_int;
                drawOrder = _spMalloc(
                    (::core::mem::size_of::<c_int>() as c_ulong)
                        .wrapping_mul((*skeletonData).slotsCount as c_ulong),
                    b"spine.c\0" as *const u8 as *const c_char,
                    8967 as c_int,
                ) as *mut c_int;
                ii = (*skeletonData).slotsCount - 1 as c_int;
                while ii >= 0 as c_int {
                    *drawOrder.offset(ii as isize) = -(1 as c_int);
                    ii -= 1;
                }
                offsetMap = (*offsets).child;
                while !offsetMap.is_null() {
                    let mut slotIndex_1: c_int = spSkeletonData_findSlotIndex(
                        skeletonData,
                        Json_getString(
                            offsetMap,
                            b"slot\0" as *const u8 as *const c_char,
                            0 as *const c_char,
                        ),
                    );
                    if slotIndex_1 == -(1 as c_int) {
                        spAnimation_dispose(animation);
                        _spSkeletonJson_setError(
                            self_0,
                            0 as *mut Json,
                            b"Slot not found: \0" as *const u8 as *const c_char,
                            Json_getString(
                                offsetMap,
                                b"slot\0" as *const u8 as *const c_char,
                                0 as *const c_char,
                            ),
                        );
                        return 0 as *mut spAnimation;
                    }
                    while originalIndex != slotIndex_1 {
                        let fresh164 = originalIndex;
                        originalIndex = originalIndex + 1;
                        let fresh165 = unchangedIndex;
                        unchangedIndex = unchangedIndex + 1;
                        *unchanged.offset(fresh165 as isize) = fresh164;
                    }
                    *drawOrder
                        .offset(
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
                    let fresh166 = originalIndex;
                    originalIndex = originalIndex + 1;
                    let fresh167 = unchangedIndex;
                    unchangedIndex = unchangedIndex + 1;
                    *unchanged.offset(fresh167 as isize) = fresh166;
                }
                ii = (*skeletonData).slotsCount - 1 as c_int;
                while ii >= 0 as c_int {
                    if *drawOrder.offset(ii as isize) == -(1 as c_int) {
                        unchangedIndex -= 1;
                        *drawOrder
                            .offset(
                                ii as isize,
                            ) = *unchanged.offset(unchangedIndex as isize);
                    }
                    ii -= 1;
                }
                _spFree(unchanged as *mut c_void);
            }
            spDrawOrderTimeline_setFrame(
                timeline_9,
                frameIndex,
                Json_getFloat(
                    valueMap,
                    b"time\0" as *const u8 as *const c_char,
                    0 as c_int as c_float,
                ),
                drawOrder,
            );
            _spFree(drawOrder as *mut c_void);
            valueMap = (*valueMap).next;
            frameIndex += 1;
        }
        let fresh168 = (*animation).timelinesCount;
        (*animation).timelinesCount = (*animation).timelinesCount + 1;
        let ref mut fresh169 = *((*animation).timelines).offset(fresh168 as isize);
        *fresh169 = timeline_9 as *mut spTimeline;
        (*animation)
            .duration = if (*animation).duration
            > *((*timeline_9).frames)
                .offset(((*drawOrderJson).size - 1 as c_int) as isize)
        {
            (*animation).duration
        } else {
            *((*timeline_9).frames)
                .offset(((*drawOrderJson).size - 1 as c_int) as isize)
        };
    }
    if !events.is_null() {
        let mut timeline_10: *mut spEventTimeline = spEventTimeline_create(
            (*events).size,
        );
        valueMap = (*events).child;
        frameIndex = 0 as c_int;
        while !valueMap.is_null() {
            let mut event: *mut spEvent = 0 as *mut spEvent;
            let mut stringValue: *const c_char = 0 as *const c_char;
            let mut eventData: *mut spEventData = spSkeletonData_findEvent(
                skeletonData,
                Json_getString(
                    valueMap,
                    b"name\0" as *const u8 as *const c_char,
                    0 as *const c_char,
                ),
            );
            if eventData.is_null() {
                spAnimation_dispose(animation);
                _spSkeletonJson_setError(
                    self_0,
                    0 as *mut Json,
                    b"Event not found: \0" as *const u8 as *const c_char,
                    Json_getString(
                        valueMap,
                        b"name\0" as *const u8 as *const c_char,
                        0 as *const c_char,
                    ),
                );
                return 0 as *mut spAnimation;
            }
            event = spEvent_create(
                Json_getFloat(
                    valueMap,
                    b"time\0" as *const u8 as *const c_char,
                    0 as c_int as c_float,
                ),
                eventData,
            );
            (*event)
                .intValue = Json_getInt(
                valueMap,
                b"int\0" as *const u8 as *const c_char,
                (*eventData).intValue,
            );
            (*event)
                .floatValue = Json_getFloat(
                valueMap,
                b"float\0" as *const u8 as *const c_char,
                (*eventData).floatValue,
            );
            stringValue = Json_getString(
                valueMap,
                b"string\0" as *const u8 as *const c_char,
                (*eventData).stringValue,
            );
            if !stringValue.is_null() {
                let ref mut fresh170 = *(&mut (*event).stringValue
                    as *mut *const c_char as *mut *mut c_char);
                *fresh170 = _spMalloc(
                    (::core::mem::size_of::<c_char>() as c_ulong)
                        .wrapping_mul(
                            (spine_strlen(stringValue))
                                .wrapping_add(1 as c_int as c_ulong),
                        ),
                    b"spine.c\0" as *const u8 as *const c_char,
                    9016 as c_int,
                ) as *mut c_char;
                spine_strcpy(*fresh170, stringValue);
            }
            if !((*eventData).audioPath).is_null() {
                (*event)
                    .volume = Json_getFloat(
                    valueMap,
                    b"volume\0" as *const u8 as *const c_char,
                    1 as c_int as c_float,
                );
                (*event)
                    .balance = Json_getFloat(
                    valueMap,
                    b"volume\0" as *const u8 as *const c_char,
                    0 as c_int as c_float,
                );
            }
            spEventTimeline_setFrame(timeline_10, frameIndex, event);
            valueMap = (*valueMap).next;
            frameIndex += 1;
        }
        let fresh171 = (*animation).timelinesCount;
        (*animation).timelinesCount = (*animation).timelinesCount + 1;
        let ref mut fresh172 = *((*animation).timelines).offset(fresh171 as isize);
        *fresh172 = timeline_10 as *mut spTimeline;
        (*animation)
            .duration = if (*animation).duration
            > *((*timeline_10).frames)
                .offset(((*events).size - 1 as c_int) as isize)
        {
            (*animation).duration
        } else {
            *((*timeline_10).frames).offset(((*events).size - 1 as c_int) as isize)
        };
    }
    return animation;
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
    entry = Json_getItem(
        attachmentMap,
        b"vertices\0" as *const u8 as *const c_char,
    );
    entrySize = (*entry).size;
    vertices = _spMalloc(
        (::core::mem::size_of::<c_float>() as c_ulong)
            .wrapping_mul(entrySize as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        9041 as c_int,
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
        let fresh173 = i;
        i = i + 1;
        let mut boneCount: c_int = *vertices.offset(fresh173 as isize)
            as c_int;
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
    let ref mut fresh174 = *(&(*self_0).error as *const *const c_char
        as *mut *mut c_char);
    *fresh174 = 0 as *mut c_char;
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
        let ref mut fresh175 = *(&mut (*skeletonData).hash as *mut *const c_char
            as *mut *mut c_char);
        *fresh175 = _spMalloc(
            (::core::mem::size_of::<c_char>() as c_ulong)
                .wrapping_mul(
                    (spine_strlen(
                        Json_getString(
                            skeleton,
                            b"hash\0" as *const u8 as *const c_char,
                            0 as *const c_char,
                        ),
                    ))
                        .wrapping_add(1 as c_int as c_ulong),
                ),
            b"spine.c\0" as *const u8 as *const c_char,
            9115 as c_int,
        ) as *mut c_char;
        spine_strcpy(
            *fresh175,
            Json_getString(
                skeleton,
                b"hash\0" as *const u8 as *const c_char,
                0 as *const c_char,
            ),
        );
        let ref mut fresh176 = *(&mut (*skeletonData).version as *mut *const c_char
            as *mut *mut c_char);
        *fresh176 = _spMalloc(
            (::core::mem::size_of::<c_char>() as c_ulong)
                .wrapping_mul(
                    (spine_strlen(
                        Json_getString(
                            skeleton,
                            b"spine\0" as *const u8 as *const c_char,
                            0 as *const c_char,
                        ),
                    ))
                        .wrapping_add(1 as c_int as c_ulong),
                ),
            b"spine.c\0" as *const u8 as *const c_char,
            9116 as c_int,
        ) as *mut c_char;
        spine_strcpy(
            *fresh176,
            Json_getString(
                skeleton,
                b"spine\0" as *const u8 as *const c_char,
                0 as *const c_char,
            ),
        );
        if spine_strcmp(
            (*skeletonData).version,
            b"3.8.75\0" as *const u8 as *const c_char,
        ) == 0 as c_int
        {
            spSkeletonData_dispose(skeletonData);
            _spSkeletonJson_setError(
                self_0,
                root,
                b"Unsupported skeleton data, please export with a newer version of Spine.\0"
                    as *const u8 as *const c_char,
                b"\0" as *const u8 as *const c_char,
            );
            return 0 as *mut spSkeletonData;
        }
        (*skeletonData)
            .x = Json_getFloat(
            skeleton,
            b"x\0" as *const u8 as *const c_char,
            0 as c_int as c_float,
        );
        (*skeletonData)
            .y = Json_getFloat(
            skeleton,
            b"y\0" as *const u8 as *const c_char,
            0 as c_int as c_float,
        );
        (*skeletonData)
            .width = Json_getFloat(
            skeleton,
            b"width\0" as *const u8 as *const c_char,
            0 as c_int as c_float,
        );
        (*skeletonData)
            .height = Json_getFloat(
            skeleton,
            b"height\0" as *const u8 as *const c_char,
            0 as c_int as c_float,
        );
    }
    bones = Json_getItem(root, b"bones\0" as *const u8 as *const c_char);
    (*skeletonData)
        .bones = _spMalloc(
        (::core::mem::size_of::<*mut spBoneData>() as c_ulong)
            .wrapping_mul((*bones).size as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        9130 as c_int,
    ) as *mut *mut spBoneData;
    boneMap = (*bones).child;
    i = 0 as c_int;
    while !boneMap.is_null() {
        let mut data: *mut spBoneData = 0 as *mut spBoneData;
        let mut transformMode: *const c_char = 0 as *const c_char;
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
        (*data)
            .length = Json_getFloat(
            boneMap,
            b"length\0" as *const u8 as *const c_char,
            0 as c_int as c_float,
        ) * (*self_0).scale;
        (*data)
            .x = Json_getFloat(
            boneMap,
            b"x\0" as *const u8 as *const c_char,
            0 as c_int as c_float,
        ) * (*self_0).scale;
        (*data)
            .y = Json_getFloat(
            boneMap,
            b"y\0" as *const u8 as *const c_char,
            0 as c_int as c_float,
        ) * (*self_0).scale;
        (*data)
            .rotation = Json_getFloat(
            boneMap,
            b"rotation\0" as *const u8 as *const c_char,
            0 as c_int as c_float,
        );
        (*data)
            .scaleX = Json_getFloat(
            boneMap,
            b"scaleX\0" as *const u8 as *const c_char,
            1 as c_int as c_float,
        );
        (*data)
            .scaleY = Json_getFloat(
            boneMap,
            b"scaleY\0" as *const u8 as *const c_char,
            1 as c_int as c_float,
        );
        (*data)
            .shearX = Json_getFloat(
            boneMap,
            b"shearX\0" as *const u8 as *const c_char,
            0 as c_int as c_float,
        );
        (*data)
            .shearY = Json_getFloat(
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
        if spine_strcmp(transformMode, b"normal\0" as *const u8 as *const c_char)
            == 0 as c_int
        {
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
        } else if spine_strcmp(
                transformMode,
                b"noScale\0" as *const u8 as *const c_char,
            ) == 0 as c_int
            {
            (*data).transformMode = SP_TRANSFORMMODE_NOSCALE;
        } else if spine_strcmp(
                transformMode,
                b"noScaleOrReflection\0" as *const u8 as *const c_char,
            ) == 0 as c_int
            {
            (*data).transformMode = SP_TRANSFORMMODE_NOSCALEORREFLECTION;
        }
        (*data)
            .skinRequired = if Json_getInt(
            boneMap,
            b"skin\0" as *const u8 as *const c_char,
            0 as c_int,
        ) != 0
        {
            1 as c_int
        } else {
            0 as c_int
        };
        let ref mut fresh177 = *((*skeletonData).bones).offset(i as isize);
        *fresh177 = data;
        (*skeletonData).bonesCount += 1;
        boneMap = (*boneMap).next;
        i += 1;
    }
    slots = Json_getItem(root, b"slots\0" as *const u8 as *const c_char);
    if !slots.is_null() {
        let mut slotMap: *mut Json = 0 as *mut Json;
        (*skeletonData).slotsCount = (*slots).size;
        (*skeletonData)
            .slots = _spMalloc(
            (::core::mem::size_of::<*mut spSlotData>() as c_ulong)
                .wrapping_mul((*slots).size as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            9173 as c_int,
        ) as *mut *mut spSlotData;
        slotMap = (*slots).child;
        i = 0 as c_int;
        while !slotMap.is_null() {
            let mut data_0: *mut spSlotData = 0 as *mut spSlotData;
            let mut color: *const c_char = 0 as *const c_char;
            let mut dark: *const c_char = 0 as *const c_char;
            let mut item: *mut Json = 0 as *mut Json;
            let mut boneName: *const c_char = Json_getString(
                slotMap,
                b"bone\0" as *const u8 as *const c_char,
                0 as *const c_char,
            );
            let mut boneData: *mut spBoneData = spSkeletonData_findBone(
                skeletonData,
                boneName,
            );
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
            color = Json_getString(
                slotMap,
                b"color\0" as *const u8 as *const c_char,
                0 as *const c_char,
            );
            if !color.is_null() {
                spColor_setFromFloats(
                    &mut (*data_0).color,
                    toColor(color, 0 as c_int),
                    toColor(color, 1 as c_int),
                    toColor(color, 2 as c_int),
                    toColor(color, 3 as c_int),
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
            item = Json_getItem(
                slotMap,
                b"attachment\0" as *const u8 as *const c_char,
            );
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
            let ref mut fresh178 = *((*skeletonData).slots).offset(i as isize);
            *fresh178 = data_0;
            slotMap = (*slotMap).next;
            i += 1;
        }
    }
    ik = Json_getItem(root, b"ik\0" as *const u8 as *const c_char);
    if !ik.is_null() {
        let mut constraintMap: *mut Json = 0 as *mut Json;
        (*skeletonData).ikConstraintsCount = (*ik).size;
        (*skeletonData)
            .ikConstraints = _spMalloc(
            (::core::mem::size_of::<*mut spIkConstraintData>() as c_ulong)
                .wrapping_mul((*ik).size as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            9231 as c_int,
        ) as *mut *mut spIkConstraintData;
        constraintMap = (*ik).child;
        i = 0 as c_int;
        while !constraintMap.is_null() {
            let mut targetName: *const c_char = 0 as *const c_char;
            let mut data_1: *mut spIkConstraintData = spIkConstraintData_create(
                Json_getString(
                    constraintMap,
                    b"name\0" as *const u8 as *const c_char,
                    0 as *const c_char,
                ),
            );
            (*data_1)
                .order = Json_getInt(
                constraintMap,
                b"order\0" as *const u8 as *const c_char,
                0 as c_int,
            );
            (*data_1)
                .skinRequired = if Json_getInt(
                constraintMap,
                b"skin\0" as *const u8 as *const c_char,
                0 as c_int,
            ) != 0
            {
                1 as c_int
            } else {
                0 as c_int
            };
            boneMap = Json_getItem(
                constraintMap,
                b"bones\0" as *const u8 as *const c_char,
            );
            (*data_1).bonesCount = (*boneMap).size;
            (*data_1)
                .bones = _spMalloc(
                (::core::mem::size_of::<*mut spBoneData>() as c_ulong)
                    .wrapping_mul((*boneMap).size as c_ulong),
                b"spine.c\0" as *const u8 as *const c_char,
                9241 as c_int,
            ) as *mut *mut spBoneData;
            boneMap = (*boneMap).child;
            ii = 0 as c_int;
            while !boneMap.is_null() {
                let ref mut fresh179 = *((*data_1).bones).offset(ii as isize);
                *fresh179 = spSkeletonData_findBone(
                    skeletonData,
                    (*boneMap).valueString,
                );
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
            (*data_1)
                .bendDirection = if Json_getInt(
                constraintMap,
                b"bendPositive\0" as *const u8 as *const c_char,
                1 as c_int,
            ) != 0
            {
                1 as c_int
            } else {
                -(1 as c_int)
            };
            (*data_1)
                .compress = if Json_getInt(
                constraintMap,
                b"compress\0" as *const u8 as *const c_char,
                0 as c_int,
            ) != 0
            {
                1 as c_int
            } else {
                0 as c_int
            };
            (*data_1)
                .stretch = if Json_getInt(
                constraintMap,
                b"stretch\0" as *const u8 as *const c_char,
                0 as c_int,
            ) != 0
            {
                1 as c_int
            } else {
                0 as c_int
            };
            (*data_1)
                .uniform = if Json_getInt(
                constraintMap,
                b"uniform\0" as *const u8 as *const c_char,
                0 as c_int,
            ) != 0
            {
                1 as c_int
            } else {
                0 as c_int
            };
            (*data_1)
                .mix = Json_getFloat(
                constraintMap,
                b"mix\0" as *const u8 as *const c_char,
                1 as c_int as c_float,
            );
            (*data_1)
                .softness = Json_getFloat(
                constraintMap,
                b"softness\0" as *const u8 as *const c_char,
                0 as c_int as c_float,
            ) * (*self_0).scale;
            let ref mut fresh180 = *((*skeletonData).ikConstraints).offset(i as isize);
            *fresh180 = data_1;
            constraintMap = (*constraintMap).next;
            i += 1;
        }
    }
    transform = Json_getItem(root, b"transform\0" as *const u8 as *const c_char);
    if !transform.is_null() {
        let mut constraintMap_0: *mut Json = 0 as *mut Json;
        (*skeletonData).transformConstraintsCount = (*transform).size;
        (*skeletonData)
            .transformConstraints = _spMalloc(
            (::core::mem::size_of::<*mut spTransformConstraintData>() as c_ulong)
                .wrapping_mul((*transform).size as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            9275 as c_int,
        ) as *mut *mut spTransformConstraintData;
        constraintMap_0 = (*transform).child;
        i = 0 as c_int;
        while !constraintMap_0.is_null() {
            let mut name: *const c_char = 0 as *const c_char;
            let mut data_2: *mut spTransformConstraintData = spTransformConstraintData_create(
                Json_getString(
                    constraintMap_0,
                    b"name\0" as *const u8 as *const c_char,
                    0 as *const c_char,
                ),
            );
            (*data_2)
                .order = Json_getInt(
                constraintMap_0,
                b"order\0" as *const u8 as *const c_char,
                0 as c_int,
            );
            (*data_2)
                .skinRequired = if Json_getInt(
                constraintMap_0,
                b"skin\0" as *const u8 as *const c_char,
                0 as c_int,
            ) != 0
            {
                1 as c_int
            } else {
                0 as c_int
            };
            boneMap = Json_getItem(
                constraintMap_0,
                b"bones\0" as *const u8 as *const c_char,
            );
            (*data_2).bonesCount = (*boneMap).size;
            let ref mut fresh181 = *(&(*data_2).bones as *const *mut *mut spBoneData
                as *mut *mut *mut spBoneData);
            *fresh181 = _spMalloc(
                (::core::mem::size_of::<*mut spBoneData>() as c_ulong)
                    .wrapping_mul((*boneMap).size as c_ulong),
                b"spine.c\0" as *const u8 as *const c_char,
                9285 as c_int,
            ) as *mut *mut spBoneData;
            boneMap = (*boneMap).child;
            ii = 0 as c_int;
            while !boneMap.is_null() {
                let ref mut fresh182 = *((*data_2).bones).offset(ii as isize);
                *fresh182 = spSkeletonData_findBone(
                    skeletonData,
                    (*boneMap).valueString,
                );
                if (*((*data_2).bones).offset(ii as isize)).is_null() {
                    spSkeletonData_dispose(skeletonData);
                    _spSkeletonJson_setError(
                        self_0,
                        root,
                        b"Transform bone not found: \0" as *const u8
                            as *const c_char,
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
            (*data_2)
                .local = Json_getInt(
                constraintMap_0,
                b"local\0" as *const u8 as *const c_char,
                0 as c_int,
            );
            (*data_2)
                .relative = Json_getInt(
                constraintMap_0,
                b"relative\0" as *const u8 as *const c_char,
                0 as c_int,
            );
            (*data_2)
                .offsetRotation = Json_getFloat(
                constraintMap_0,
                b"rotation\0" as *const u8 as *const c_char,
                0 as c_int as c_float,
            );
            (*data_2)
                .offsetX = Json_getFloat(
                constraintMap_0,
                b"x\0" as *const u8 as *const c_char,
                0 as c_int as c_float,
            ) * (*self_0).scale;
            (*data_2)
                .offsetY = Json_getFloat(
                constraintMap_0,
                b"y\0" as *const u8 as *const c_char,
                0 as c_int as c_float,
            ) * (*self_0).scale;
            (*data_2)
                .offsetScaleX = Json_getFloat(
                constraintMap_0,
                b"scaleX\0" as *const u8 as *const c_char,
                0 as c_int as c_float,
            );
            (*data_2)
                .offsetScaleY = Json_getFloat(
                constraintMap_0,
                b"scaleY\0" as *const u8 as *const c_char,
                0 as c_int as c_float,
            );
            (*data_2)
                .offsetShearY = Json_getFloat(
                constraintMap_0,
                b"shearY\0" as *const u8 as *const c_char,
                0 as c_int as c_float,
            );
            (*data_2)
                .rotateMix = Json_getFloat(
                constraintMap_0,
                b"rotateMix\0" as *const u8 as *const c_char,
                1 as c_int as c_float,
            );
            (*data_2)
                .translateMix = Json_getFloat(
                constraintMap_0,
                b"translateMix\0" as *const u8 as *const c_char,
                1 as c_int as c_float,
            );
            (*data_2)
                .scaleMix = Json_getFloat(
                constraintMap_0,
                b"scaleMix\0" as *const u8 as *const c_char,
                1 as c_int as c_float,
            );
            (*data_2)
                .shearMix = Json_getFloat(
                constraintMap_0,
                b"shearMix\0" as *const u8 as *const c_char,
                1 as c_int as c_float,
            );
            let ref mut fresh183 = *((*skeletonData).transformConstraints)
                .offset(i as isize);
            *fresh183 = data_2;
            constraintMap_0 = (*constraintMap_0).next;
            i += 1;
        }
    }
    pathJson = Json_getItem(root, b"path\0" as *const u8 as *const c_char);
    if !pathJson.is_null() {
        let mut constraintMap_1: *mut Json = 0 as *mut Json;
        (*skeletonData).pathConstraintsCount = (*pathJson).size;
        (*skeletonData)
            .pathConstraints = _spMalloc(
            (::core::mem::size_of::<*mut spPathConstraintData>() as c_ulong)
                .wrapping_mul((*pathJson).size as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            9326 as c_int,
        ) as *mut *mut spPathConstraintData;
        constraintMap_1 = (*pathJson).child;
        i = 0 as c_int;
        while !constraintMap_1.is_null() {
            let mut name_0: *const c_char = 0 as *const c_char;
            let mut item_0: *const c_char = 0 as *const c_char;
            let mut data_3: *mut spPathConstraintData = spPathConstraintData_create(
                Json_getString(
                    constraintMap_1,
                    b"name\0" as *const u8 as *const c_char,
                    0 as *const c_char,
                ),
            );
            (*data_3)
                .order = Json_getInt(
                constraintMap_1,
                b"order\0" as *const u8 as *const c_char,
                0 as c_int,
            );
            (*data_3)
                .skinRequired = if Json_getInt(
                constraintMap_1,
                b"skin\0" as *const u8 as *const c_char,
                0 as c_int,
            ) != 0
            {
                1 as c_int
            } else {
                0 as c_int
            };
            boneMap = Json_getItem(
                constraintMap_1,
                b"bones\0" as *const u8 as *const c_char,
            );
            (*data_3).bonesCount = (*boneMap).size;
            let ref mut fresh184 = *(&(*data_3).bones as *const *mut *mut spBoneData
                as *mut *mut *mut spBoneData);
            *fresh184 = _spMalloc(
                (::core::mem::size_of::<*mut spBoneData>() as c_ulong)
                    .wrapping_mul((*boneMap).size as c_ulong),
                b"spine.c\0" as *const u8 as *const c_char,
                9337 as c_int,
            ) as *mut *mut spBoneData;
            boneMap = (*boneMap).child;
            ii = 0 as c_int;
            while !boneMap.is_null() {
                let ref mut fresh185 = *((*data_3).bones).offset(ii as isize);
                *fresh185 = spSkeletonData_findBone(
                    skeletonData,
                    (*boneMap).valueString,
                );
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
            if spine_strcmp(item_0, b"fixed\0" as *const u8 as *const c_char)
                == 0 as c_int
            {
                (*data_3).positionMode = SP_POSITION_MODE_FIXED;
            } else if spine_strcmp(
                    item_0,
                    b"percent\0" as *const u8 as *const c_char,
                ) == 0 as c_int
                {
                (*data_3).positionMode = SP_POSITION_MODE_PERCENT;
            }
            item_0 = Json_getString(
                constraintMap_1,
                b"spacingMode\0" as *const u8 as *const c_char,
                b"length\0" as *const u8 as *const c_char,
            );
            if spine_strcmp(item_0, b"length\0" as *const u8 as *const c_char)
                == 0 as c_int
            {
                (*data_3).spacingMode = SP_SPACING_MODE_LENGTH;
            } else if spine_strcmp(
                    item_0,
                    b"fixed\0" as *const u8 as *const c_char,
                ) == 0 as c_int
                {
                (*data_3).spacingMode = SP_SPACING_MODE_FIXED;
            } else if spine_strcmp(
                    item_0,
                    b"percent\0" as *const u8 as *const c_char,
                ) == 0 as c_int
                {
                (*data_3).spacingMode = SP_SPACING_MODE_PERCENT;
            }
            item_0 = Json_getString(
                constraintMap_1,
                b"rotateMode\0" as *const u8 as *const c_char,
                b"tangent\0" as *const u8 as *const c_char,
            );
            if spine_strcmp(item_0, b"tangent\0" as *const u8 as *const c_char)
                == 0 as c_int
            {
                (*data_3).rotateMode = SP_ROTATE_MODE_TANGENT;
            } else if spine_strcmp(
                    item_0,
                    b"chain\0" as *const u8 as *const c_char,
                ) == 0 as c_int
                {
                (*data_3).rotateMode = SP_ROTATE_MODE_CHAIN;
            } else if spine_strcmp(
                    item_0,
                    b"chainScale\0" as *const u8 as *const c_char,
                ) == 0 as c_int
                {
                (*data_3).rotateMode = SP_ROTATE_MODE_CHAIN_SCALE;
            }
            (*data_3)
                .offsetRotation = Json_getFloat(
                constraintMap_1,
                b"rotation\0" as *const u8 as *const c_char,
                0 as c_int as c_float,
            );
            (*data_3)
                .position = Json_getFloat(
                constraintMap_1,
                b"position\0" as *const u8 as *const c_char,
                0 as c_int as c_float,
            );
            if (*data_3).positionMode as c_uint
                == SP_POSITION_MODE_FIXED as c_int as c_uint
            {
                (*data_3).position *= (*self_0).scale;
            }
            (*data_3)
                .spacing = Json_getFloat(
                constraintMap_1,
                b"spacing\0" as *const u8 as *const c_char,
                0 as c_int as c_float,
            );
            if (*data_3).spacingMode as c_uint
                == SP_SPACING_MODE_LENGTH as c_int as c_uint
                || (*data_3).spacingMode as c_uint
                    == SP_SPACING_MODE_FIXED as c_int as c_uint
            {
                (*data_3).spacing *= (*self_0).scale;
            }
            (*data_3)
                .rotateMix = Json_getFloat(
                constraintMap_1,
                b"rotateMix\0" as *const u8 as *const c_char,
                1 as c_int as c_float,
            );
            (*data_3)
                .translateMix = Json_getFloat(
                constraintMap_1,
                b"translateMix\0" as *const u8 as *const c_char,
                1 as c_int as c_float,
            );
            let ref mut fresh186 = *((*skeletonData).pathConstraints).offset(i as isize);
            *fresh186 = data_3;
            constraintMap_1 = (*constraintMap_1).next;
            i += 1;
        }
    }
    skins = Json_getItem(root, b"skins\0" as *const u8 as *const c_char);
    if !skins.is_null() {
        let mut skinMap: *mut Json = 0 as *mut Json;
        (*skeletonData)
            .skins = _spMalloc(
            (::core::mem::size_of::<*mut spSkin>() as c_ulong)
                .wrapping_mul((*skins).size as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            9385 as c_int,
        ) as *mut *mut spSkin;
        skinMap = (*skins).child;
        i = 0 as c_int;
        while !skinMap.is_null() {
            let mut attachmentsMap: *mut Json = 0 as *mut Json;
            let mut curves: *mut Json = 0 as *mut Json;
            let mut skinPart: *mut Json = 0 as *mut Json;
            let mut skin: *mut spSkin = spSkin_create(
                Json_getString(
                    skinMap,
                    b"name\0" as *const u8 as *const c_char,
                    b"\0" as *const u8 as *const c_char,
                ),
            );
            skinPart = Json_getItem(
                skinMap,
                b"bones\0" as *const u8 as *const c_char,
            );
            if !skinPart.is_null() {
                skinPart = (*skinPart).child;
                while !skinPart.is_null() {
                    let mut bone: *mut spBoneData = spSkeletonData_findBone(
                        skeletonData,
                        (*skinPart).valueString,
                    );
                    if bone.is_null() {
                        spSkeletonData_dispose(skeletonData);
                        _spSkeletonJson_setError(
                            self_0,
                            root,
                            b"Skin bone constraint not found: \0" as *const u8
                                as *const c_char,
                            (*skinPart).valueString,
                        );
                        return 0 as *mut spSkeletonData;
                    }
                    spBoneDataArray_add((*skin).bones, bone);
                    skinPart = (*skinPart).next;
                }
            }
            skinPart = Json_getItem(
                skinMap,
                b"ik\0" as *const u8 as *const c_char,
            );
            if !skinPart.is_null() {
                skinPart = (*skinPart).child;
                while !skinPart.is_null() {
                    let mut constraint: *mut spIkConstraintData = spSkeletonData_findIkConstraint(
                        skeletonData,
                        (*skinPart).valueString,
                    );
                    if constraint.is_null() {
                        spSkeletonData_dispose(skeletonData);
                        _spSkeletonJson_setError(
                            self_0,
                            root,
                            b"Skin IK constraint not found: \0" as *const u8
                                as *const c_char,
                            (*skinPart).valueString,
                        );
                        return 0 as *mut spSkeletonData;
                    }
                    spIkConstraintDataArray_add((*skin).ikConstraints, constraint);
                    skinPart = (*skinPart).next;
                }
            }
            skinPart = Json_getItem(
                skinMap,
                b"path\0" as *const u8 as *const c_char,
            );
            if !skinPart.is_null() {
                skinPart = (*skinPart).child;
                while !skinPart.is_null() {
                    let mut constraint_0: *mut spPathConstraintData = spSkeletonData_findPathConstraint(
                        skeletonData,
                        (*skinPart).valueString,
                    );
                    if constraint_0.is_null() {
                        spSkeletonData_dispose(skeletonData);
                        _spSkeletonJson_setError(
                            self_0,
                            root,
                            b"Skin path constraint not found: \0" as *const u8
                                as *const c_char,
                            (*skinPart).valueString,
                        );
                        return 0 as *mut spSkeletonData;
                    }
                    spPathConstraintDataArray_add((*skin).pathConstraints, constraint_0);
                    skinPart = (*skinPart).next;
                }
            }
            skinPart = Json_getItem(
                skinMap,
                b"transform\0" as *const u8 as *const c_char,
            );
            if !skinPart.is_null() {
                skinPart = (*skinPart).child;
                while !skinPart.is_null() {
                    let mut constraint_1: *mut spTransformConstraintData = spSkeletonData_findTransformConstraint(
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
                    spTransformConstraintDataArray_add(
                        (*skin).transformConstraints,
                        constraint_1,
                    );
                    skinPart = (*skinPart).next;
                }
            }
            let fresh187 = (*skeletonData).skinsCount;
            (*skeletonData).skinsCount = (*skeletonData).skinsCount + 1;
            let ref mut fresh188 = *((*skeletonData).skins).offset(fresh187 as isize);
            *fresh188 = skin;
            if spine_strcmp(
                (*skin).name,
                b"default\0" as *const u8 as *const c_char,
            ) == 0 as c_int
            {
                (*skeletonData).defaultSkin = skin;
            }
            attachmentsMap = (*Json_getItem(
                skinMap,
                b"attachments\0" as *const u8 as *const c_char,
            ))
                .child;
            while !attachmentsMap.is_null() {
                let mut slot: *mut spSlotData = spSkeletonData_findSlot(
                    skeletonData,
                    (*attachmentsMap).name,
                );
                let mut attachmentMap: *mut Json = 0 as *mut Json;
                attachmentMap = (*attachmentsMap).child;
                while !attachmentMap.is_null() {
                    let mut attachment: *mut spAttachment = 0 as *mut spAttachment;
                    let mut skinAttachmentName: *const c_char = (*attachmentMap)
                        .name;
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
                    let mut color_0: *const c_char = 0 as *const c_char;
                    let mut entry: *mut Json = 0 as *mut Json;
                    let mut typeString: *const c_char = Json_getString(
                        attachmentMap,
                        b"type\0" as *const u8 as *const c_char,
                        b"region\0" as *const u8 as *const c_char,
                    );
                    let mut type_0: spAttachmentType = SP_ATTACHMENT_REGION;
                    if spine_strcmp(
                        typeString,
                        b"region\0" as *const u8 as *const c_char,
                    ) == 0 as c_int
                    {
                        type_0 = SP_ATTACHMENT_REGION;
                    } else if spine_strcmp(
                            typeString,
                            b"mesh\0" as *const u8 as *const c_char,
                        ) == 0 as c_int
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
                    } else if spine_strcmp(
                            typeString,
                            b"path\0" as *const u8 as *const c_char,
                        ) == 0 as c_int
                        {
                        type_0 = SP_ATTACHMENT_PATH;
                    } else if spine_strcmp(
                            typeString,
                            b"clipping\0" as *const u8 as *const c_char,
                        ) == 0 as c_int
                        {
                        type_0 = SP_ATTACHMENT_CLIPPING;
                    } else if spine_strcmp(
                            typeString,
                            b"point\0" as *const u8 as *const c_char,
                        ) == 0 as c_int
                        {
                        type_0 = SP_ATTACHMENT_POINT;
                    } else {
                        spSkeletonData_dispose(skeletonData);
                        _spSkeletonJson_setError(
                            self_0,
                            root,
                            b"Unknown attachment type: \0" as *const u8
                                as *const c_char,
                            typeString,
                        );
                        return 0 as *mut spSkeletonData;
                    }
                    attachment = spAttachmentLoader_createAttachment(
                        (*self_0).attachmentLoader,
                        skin,
                        type_0,
                        attachmentName,
                        path,
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
                                let mut region: *mut spRegionAttachment = attachment
                                    as *mut spRegionAttachment;
                                if !path.is_null() {
                                    let ref mut fresh189 = *(&mut (*region).path
                                        as *mut *const c_char as *mut *mut c_char);
                                    *fresh189 = _spMalloc(
                                        (::core::mem::size_of::<c_char>() as c_ulong)
                                            .wrapping_mul(
                                                (spine_strlen(path))
                                                    .wrapping_add(1 as c_int as c_ulong),
                                            ),
                                        b"spine.c\0" as *const u8 as *const c_char,
                                        9487 as c_int,
                                    ) as *mut c_char;
                                    spine_strcpy(*fresh189, path);
                                }
                                (*region)
                                    .x = Json_getFloat(
                                    attachmentMap,
                                    b"x\0" as *const u8 as *const c_char,
                                    0 as c_int as c_float,
                                ) * (*self_0).scale;
                                (*region)
                                    .y = Json_getFloat(
                                    attachmentMap,
                                    b"y\0" as *const u8 as *const c_char,
                                    0 as c_int as c_float,
                                ) * (*self_0).scale;
                                (*region)
                                    .scaleX = Json_getFloat(
                                    attachmentMap,
                                    b"scaleX\0" as *const u8 as *const c_char,
                                    1 as c_int as c_float,
                                );
                                (*region)
                                    .scaleY = Json_getFloat(
                                    attachmentMap,
                                    b"scaleY\0" as *const u8 as *const c_char,
                                    1 as c_int as c_float,
                                );
                                (*region)
                                    .rotation = Json_getFloat(
                                    attachmentMap,
                                    b"rotation\0" as *const u8 as *const c_char,
                                    0 as c_int as c_float,
                                );
                                (*region)
                                    .width = Json_getFloat(
                                    attachmentMap,
                                    b"width\0" as *const u8 as *const c_char,
                                    32 as c_int as c_float,
                                ) * (*self_0).scale;
                                (*region)
                                    .height = Json_getFloat(
                                    attachmentMap,
                                    b"height\0" as *const u8 as *const c_char,
                                    32 as c_int as c_float,
                                ) * (*self_0).scale;
                                color_0 = Json_getString(
                                    attachmentMap,
                                    b"color\0" as *const u8 as *const c_char,
                                    0 as *const c_char,
                                );
                                if !color_0.is_null() {
                                    spColor_setFromFloats(
                                        &mut (*region).color,
                                        toColor(color_0, 0 as c_int),
                                        toColor(color_0, 1 as c_int),
                                        toColor(color_0, 2 as c_int),
                                        toColor(color_0, 3 as c_int),
                                    );
                                }
                                spRegionAttachment_updateOffset(region);
                                spAttachmentLoader_configureAttachment(
                                    (*self_0).attachmentLoader,
                                    attachment,
                                );
                            }
                            2 | 3 => {
                                let mut mesh: *mut spMeshAttachment = attachment
                                    as *mut spMeshAttachment;
                                let ref mut fresh190 = *(&mut (*mesh).path
                                    as *mut *const c_char as *mut *mut c_char);
                                *fresh190 = _spMalloc(
                                    (::core::mem::size_of::<c_char>() as c_ulong)
                                        .wrapping_mul(
                                            (spine_strlen(path))
                                                .wrapping_add(1 as c_int as c_ulong),
                                        ),
                                    b"spine.c\0" as *const u8 as *const c_char,
                                    9514 as c_int,
                                ) as *mut c_char;
                                spine_strcpy(*fresh190, path);
                                color_0 = Json_getString(
                                    attachmentMap,
                                    b"color\0" as *const u8 as *const c_char,
                                    0 as *const c_char,
                                );
                                if !color_0.is_null() {
                                    spColor_setFromFloats(
                                        &mut (*mesh).color,
                                        toColor(color_0, 0 as c_int),
                                        toColor(color_0, 1 as c_int),
                                        toColor(color_0, 2 as c_int),
                                        toColor(color_0, 3 as c_int),
                                    );
                                }
                                (*mesh)
                                    .width = Json_getFloat(
                                    attachmentMap,
                                    b"width\0" as *const u8 as *const c_char,
                                    32 as c_int as c_float,
                                ) * (*self_0).scale;
                                (*mesh)
                                    .height = Json_getFloat(
                                    attachmentMap,
                                    b"height\0" as *const u8 as *const c_char,
                                    32 as c_int as c_float,
                                ) * (*self_0).scale;
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
                                    (*mesh)
                                        .triangles = _spMalloc(
                                        (::core::mem::size_of::<c_ushort>() as c_ulong)
                                            .wrapping_mul((*entry).size as c_ulong),
                                        b"spine.c\0" as *const u8 as *const c_char,
                                        9533 as c_int,
                                    ) as *mut c_ushort;
                                    entry = (*entry).child;
                                    ii = 0 as c_int;
                                    while !entry.is_null() {
                                        *((*mesh).triangles)
                                            .offset(ii as isize) = (*entry).valueInt as c_ushort;
                                        entry = (*entry).next;
                                        ii += 1;
                                    }
                                    entry = Json_getItem(
                                        attachmentMap,
                                        b"uvs\0" as *const u8 as *const c_char,
                                    );
                                    verticesLength = (*entry).size;
                                    (*mesh)
                                        .regionUVs = _spMalloc(
                                        (::core::mem::size_of::<c_float>() as c_ulong)
                                            .wrapping_mul(verticesLength as c_ulong),
                                        b"spine.c\0" as *const u8 as *const c_char,
                                        9539 as c_int,
                                    ) as *mut c_float;
                                    entry = (*entry).child;
                                    ii = 0 as c_int;
                                    while !entry.is_null() {
                                        *((*mesh).regionUVs)
                                            .offset(ii as isize) = (*entry).valueFloat;
                                        entry = (*entry).next;
                                        ii += 1;
                                    }
                                    _readVerticesJson(
                                        self_0,
                                        attachmentMap,
                                        &mut (*mesh).super_0,
                                        verticesLength,
                                    );
                                    spMeshAttachment_updateUVs(mesh);
                                    (*mesh)
                                        .hullLength = Json_getInt(
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
                                        (*mesh)
                                            .edges = _spMalloc(
                                            (::core::mem::size_of::<c_int>() as c_ulong)
                                                .wrapping_mul((*entry).size as c_ulong),
                                            b"spine.c\0" as *const u8 as *const c_char,
                                            9552 as c_int,
                                        ) as *mut c_int;
                                        entry = (*entry).child;
                                        ii = 0 as c_int;
                                        while !entry.is_null() {
                                            *((*mesh).edges).offset(ii as isize) = (*entry).valueInt;
                                            entry = (*entry).next;
                                            ii += 1;
                                        }
                                    }
                                    spAttachmentLoader_configureAttachment(
                                        (*self_0).attachmentLoader,
                                        attachment,
                                    );
                                } else {
                                    let mut inheritDeform: c_int = Json_getInt(
                                        attachmentMap,
                                        b"deform\0" as *const u8 as *const c_char,
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
                                        inheritDeform,
                                    );
                                }
                            }
                            1 => {
                                let mut box_0: *mut spBoundingBoxAttachment = attachment
                                    as *mut spBoundingBoxAttachment;
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
                                spAttachmentLoader_configureAttachment(
                                    (*self_0).attachmentLoader,
                                    attachment,
                                );
                            }
                            4 => {
                                let mut pathAttachment: *mut spPathAttachment = attachment
                                    as *mut spPathAttachment;
                                let mut vertexCount_0: c_int = 0 as c_int;
                                (*pathAttachment)
                                    .closed = Json_getInt(
                                    attachmentMap,
                                    b"closed\0" as *const u8 as *const c_char,
                                    0 as c_int,
                                );
                                (*pathAttachment)
                                    .constantSpeed = Json_getInt(
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
                                (*pathAttachment)
                                    .lengthsLength = vertexCount_0 / 3 as c_int;
                                (*pathAttachment)
                                    .lengths = _spMalloc(
                                    (::core::mem::size_of::<c_float>() as c_ulong)
                                        .wrapping_mul(
                                            (*pathAttachment).lengthsLength as c_ulong,
                                        ),
                                    b"spine.c\0" as *const u8 as *const c_char,
                                    9582 as c_int,
                                ) as *mut c_float;
                                curves = Json_getItem(
                                    attachmentMap,
                                    b"lengths\0" as *const u8 as *const c_char,
                                );
                                curves = (*curves).child;
                                ii = 0 as c_int;
                                while !curves.is_null() {
                                    *((*pathAttachment).lengths)
                                        .offset(
                                            ii as isize,
                                        ) = (*curves).valueFloat * (*self_0).scale;
                                    curves = (*curves).next;
                                    ii += 1;
                                }
                            }
                            5 => {
                                let mut point: *mut spPointAttachment = attachment
                                    as *mut spPointAttachment;
                                (*point)
                                    .x = Json_getFloat(
                                    attachmentMap,
                                    b"x\0" as *const u8 as *const c_char,
                                    0 as c_int as c_float,
                                ) * (*self_0).scale;
                                (*point)
                                    .y = Json_getFloat(
                                    attachmentMap,
                                    b"y\0" as *const u8 as *const c_char,
                                    0 as c_int as c_float,
                                ) * (*self_0).scale;
                                (*point)
                                    .rotation = Json_getFloat(
                                    attachmentMap,
                                    b"rotation\0" as *const u8 as *const c_char,
                                    0 as c_int as c_float,
                                );
                                color_0 = Json_getString(
                                    attachmentMap,
                                    b"color\0" as *const u8 as *const c_char,
                                    0 as *const c_char,
                                );
                                if !color_0.is_null() {
                                    spColor_setFromFloats(
                                        &mut (*point).color,
                                        toColor(color_0, 0 as c_int),
                                        toColor(color_0, 1 as c_int),
                                        toColor(color_0, 2 as c_int),
                                        toColor(color_0, 3 as c_int),
                                    );
                                }
                            }
                            6 => {
                                let mut clip: *mut spClippingAttachment = attachment
                                    as *mut spClippingAttachment;
                                let mut vertexCount_1: c_int = 0 as c_int;
                                let mut end: *const c_char = Json_getString(
                                    attachmentMap,
                                    b"end\0" as *const u8 as *const c_char,
                                    0 as *const c_char,
                                );
                                if !end.is_null() {
                                    let mut endSlot: *mut spSlotData = spSkeletonData_findSlot(
                                        skeletonData,
                                        end,
                                    );
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
                                spAttachmentLoader_configureAttachment(
                                    (*self_0).attachmentLoader,
                                    attachment,
                                );
                            }
                            _ => {}
                        }
                        spSkin_setAttachment(
                            skin,
                            (*slot).index,
                            skinAttachmentName,
                            attachment,
                        );
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
        let mut linkedMesh: *mut _spLinkedMeshJson = ((*internal).linkedMeshes)
            .offset(i as isize);
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
        parent_0 = spSkin_getAttachment(
            skin_0,
            (*linkedMesh).slotIndex,
            (*linkedMesh).parent,
        );
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
        (*(*linkedMesh).mesh)
            .super_0
            .deformAttachment = if (*linkedMesh).inheritDeform != 0 {
            parent_0 as *mut spVertexAttachment
        } else {
            (*linkedMesh).mesh as *mut spVertexAttachment
        };
        spMeshAttachment_setParentMesh(
            (*linkedMesh).mesh,
            parent_0 as *mut spMeshAttachment,
        );
        spMeshAttachment_updateUVs((*linkedMesh).mesh);
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
        (*skeletonData)
            .events = _spMalloc(
            (::core::mem::size_of::<*mut spEventData>() as c_ulong)
                .wrapping_mul((*events).size as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            9655 as c_int,
        ) as *mut *mut spEventData;
        eventMap = (*events).child;
        i = 0 as c_int;
        while !eventMap.is_null() {
            let mut eventData: *mut spEventData = spEventData_create((*eventMap).name);
            (*eventData)
                .intValue = Json_getInt(
                eventMap,
                b"int\0" as *const u8 as *const c_char,
                0 as c_int,
            );
            (*eventData)
                .floatValue = Json_getFloat(
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
                let ref mut fresh191 = *(&mut (*eventData).stringValue
                    as *mut *const c_char as *mut *mut c_char);
                *fresh191 = _spMalloc(
                    (::core::mem::size_of::<c_char>() as c_ulong)
                        .wrapping_mul(
                            (spine_strlen(stringValue))
                                .wrapping_add(1 as c_int as c_ulong),
                        ),
                    b"spine.c\0" as *const u8 as *const c_char,
                    9661 as c_int,
                ) as *mut c_char;
                spine_strcpy(*fresh191, stringValue);
            }
            audioPath = Json_getString(
                eventMap,
                b"audio\0" as *const u8 as *const c_char,
                0 as *const c_char,
            );
            if !audioPath.is_null() {
                let ref mut fresh192 = *(&mut (*eventData).audioPath
                    as *mut *const c_char as *mut *mut c_char);
                *fresh192 = _spMalloc(
                    (::core::mem::size_of::<c_char>() as c_ulong)
                        .wrapping_mul(
                            (spine_strlen(audioPath))
                                .wrapping_add(1 as c_int as c_ulong),
                        ),
                    b"spine.c\0" as *const u8 as *const c_char,
                    9664 as c_int,
                ) as *mut c_char;
                spine_strcpy(*fresh192, audioPath);
                (*eventData)
                    .volume = Json_getFloat(
                    eventMap,
                    b"volume\0" as *const u8 as *const c_char,
                    1 as c_int as c_float,
                );
                (*eventData)
                    .balance = Json_getFloat(
                    eventMap,
                    b"balance\0" as *const u8 as *const c_char,
                    0 as c_int as c_float,
                );
            }
            let ref mut fresh193 = *((*skeletonData).events).offset(i as isize);
            *fresh193 = eventData;
            eventMap = (*eventMap).next;
            i += 1;
        }
    }
    animations = Json_getItem(root, b"animations\0" as *const u8 as *const c_char);
    if !animations.is_null() {
        let mut animationMap: *mut Json = 0 as *mut Json;
        (*skeletonData)
            .animations = _spMalloc(
            (::core::mem::size_of::<*mut spAnimation>() as c_ulong)
                .wrapping_mul((*animations).size as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            9676 as c_int,
        ) as *mut *mut spAnimation;
        animationMap = (*animations).child;
        while !animationMap.is_null() {
            let mut animation: *mut spAnimation = _spSkeletonJson_readAnimation(
                self_0,
                animationMap,
                skeletonData,
            );
            if animation.is_null() {
                spSkeletonData_dispose(skeletonData);
                return 0 as *mut spSkeletonData;
            }
            let fresh194 = (*skeletonData).animationsCount;
            (*skeletonData).animationsCount = (*skeletonData).animationsCount + 1;
            let ref mut fresh195 = *((*skeletonData).animations)
                .offset(fresh194 as isize);
            *fresh195 = animation;
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
        9723 as c_int,
    ) as *mut spBoneDataArray;
    (*array).size = 0 as c_int;
    (*array).capacity = initialCapacity;
    (*array)
        .items = _spCalloc(
        initialCapacity as size_t,
        ::core::mem::size_of::<*mut spBoneData>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        9723 as c_int,
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
        (*self_0)
            .capacity = if 8 as c_int
            > ((*self_0).size as c_float * 1.75f32) as c_int
        {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0)
            .items = _spRealloc(
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
    (*self_0)
        .items = _spRealloc(
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
        (*self_0)
            .capacity = if 8 as c_int
            > ((*self_0).size as c_float * 1.75f32) as c_int
        {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0)
            .items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<*mut spBoneData>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut *mut spBoneData;
    }
    let fresh196 = (*self_0).size;
    (*self_0).size = (*self_0).size + 1;
    let ref mut fresh197 = *((*self_0).items).offset(fresh196 as isize);
    *fresh197 = value;
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
        ((*self_0).items).offset(index as isize).offset(1 as c_int as isize)
            as *const c_void,
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
pub unsafe extern "C" fn spBoneDataArray_pop(
    mut self_0: *mut spBoneDataArray,
) -> *mut spBoneData {
    (*self_0).size -= 1;
    let mut item: *mut spBoneData = *((*self_0).items).offset((*self_0).size as isize);
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn spBoneDataArray_peek(
    mut self_0: *mut spBoneDataArray,
) -> *mut spBoneData {
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
        9724 as c_int,
    ) as *mut spIkConstraintDataArray;
    (*array).size = 0 as c_int;
    (*array).capacity = initialCapacity;
    (*array)
        .items = _spCalloc(
        initialCapacity as size_t,
        ::core::mem::size_of::<*mut spIkConstraintData>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        9724 as c_int,
    ) as *mut *mut spIkConstraintData;
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn spIkConstraintDataArray_dispose(
    mut self_0: *mut spIkConstraintDataArray,
) {
    _spFree((*self_0).items as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spIkConstraintDataArray_clear(
    mut self_0: *mut spIkConstraintDataArray,
) {
    (*self_0).size = 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spIkConstraintDataArray_setSize(
    mut self_0: *mut spIkConstraintDataArray,
    mut newSize: c_int,
) -> *mut spIkConstraintDataArray {
    (*self_0).size = newSize;
    if (*self_0).capacity < newSize {
        (*self_0)
            .capacity = if 8 as c_int
            > ((*self_0).size as c_float * 1.75f32) as c_int
        {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0)
            .items = _spRealloc(
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
    (*self_0)
        .items = _spRealloc(
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
        (*self_0)
            .capacity = if 8 as c_int
            > ((*self_0).size as c_float * 1.75f32) as c_int
        {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0)
            .items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<*mut spIkConstraintData>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut *mut spIkConstraintData;
    }
    let fresh198 = (*self_0).size;
    (*self_0).size = (*self_0).size + 1;
    let ref mut fresh199 = *((*self_0).items).offset(fresh198 as isize);
    *fresh199 = value;
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
        ((*self_0).items).offset(index as isize).offset(1 as c_int as isize)
            as *const c_void,
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
    let mut item: *mut spIkConstraintData = *((*self_0).items)
        .offset((*self_0).size as isize);
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
        9725 as c_int,
    ) as *mut spTransformConstraintDataArray;
    (*array).size = 0 as c_int;
    (*array).capacity = initialCapacity;
    (*array)
        .items = _spCalloc(
        initialCapacity as size_t,
        ::core::mem::size_of::<*mut spTransformConstraintData>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        9725 as c_int,
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
        (*self_0)
            .capacity = if 8 as c_int
            > ((*self_0).size as c_float * 1.75f32) as c_int
        {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0)
            .items = _spRealloc(
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
    (*self_0)
        .items = _spRealloc(
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
        (*self_0)
            .capacity = if 8 as c_int
            > ((*self_0).size as c_float * 1.75f32) as c_int
        {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0)
            .items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<*mut spTransformConstraintData>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut *mut spTransformConstraintData;
    }
    let fresh200 = (*self_0).size;
    (*self_0).size = (*self_0).size + 1;
    let ref mut fresh201 = *((*self_0).items).offset(fresh200 as isize);
    *fresh201 = value;
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
        ((*self_0).items).offset(index as isize).offset(1 as c_int as isize)
            as *const c_void,
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
    let mut item: *mut spTransformConstraintData = *((*self_0).items)
        .offset((*self_0).size as isize);
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
        9726 as c_int,
    ) as *mut spPathConstraintDataArray;
    (*array).size = 0 as c_int;
    (*array).capacity = initialCapacity;
    (*array)
        .items = _spCalloc(
        initialCapacity as size_t,
        ::core::mem::size_of::<*mut spPathConstraintData>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        9726 as c_int,
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
        (*self_0)
            .capacity = if 8 as c_int
            > ((*self_0).size as c_float * 1.75f32) as c_int
        {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0)
            .items = _spRealloc(
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
    (*self_0)
        .items = _spRealloc(
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
        (*self_0)
            .capacity = if 8 as c_int
            > ((*self_0).size as c_float * 1.75f32) as c_int
        {
            8 as c_int
        } else {
            ((*self_0).size as c_float * 1.75f32) as c_int
        };
        (*self_0)
            .items = _spRealloc(
            (*self_0).items as *mut c_void,
            (::core::mem::size_of::<*mut spPathConstraintData>() as c_ulong)
                .wrapping_mul((*self_0).capacity as c_ulong),
        ) as *mut *mut spPathConstraintData;
    }
    let fresh202 = (*self_0).size;
    (*self_0).size = (*self_0).size + 1;
    let ref mut fresh203 = *((*self_0).items).offset(fresh202 as isize);
    *fresh203 = value;
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
        ((*self_0).items).offset(index as isize).offset(1 as c_int as isize)
            as *const c_void,
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
    let mut item: *mut spPathConstraintData = *((*self_0).items)
        .offset((*self_0).size as isize);
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
        9729 as c_int,
    ) as *mut _Entry;
    (*self_0).slotIndex = slotIndex;
    let ref mut fresh204 = *(&mut (*self_0).name as *mut *const c_char
        as *mut *mut c_char);
    *fresh204 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul(
                (spine_strlen(name)).wrapping_add(1 as c_int as c_ulong),
            ),
        b"spine.c\0" as *const u8 as *const c_char,
        9731 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh204, name);
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
        9743 as c_int,
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
        as unsafe extern "C" fn(
            size_t,
            size_t,
            *const c_char,
            c_int,
        ) -> *mut c_void)(
        1 as c_int as size_t,
        ::core::mem::size_of::<_spSkin>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        9755 as c_int,
    ) as *mut _spSkin))
        .super_0;
    let ref mut fresh205 = *(&(*self_0).name as *const *const c_char
        as *mut *mut c_char);
    *fresh205 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul(
                (spine_strlen(name)).wrapping_add(1 as c_int as c_ulong),
            ),
        b"spine.c\0" as *const u8 as *const c_char,
        9756 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh205, name);
    (*self_0).bones = spBoneDataArray_create(4 as c_int);
    (*self_0).ikConstraints = spIkConstraintDataArray_create(4 as c_int);
    (*self_0)
        .transformConstraints = spTransformConstraintDataArray_create(4 as c_int);
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
    let mut currentHashtableEntry: *mut *mut _SkinHashTableEntry = ((*(self_0
        as *mut _spSkin))
        .entriesHashTable)
        .as_mut_ptr();
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
    let mut hashEntry: *mut _SkinHashTableEntry = (*(self_0 as *mut _spSkin))
        .entriesHashTable[(slotIndex as c_uint)
        .wrapping_rem(100 as c_int as c_uint) as usize];
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
        let ref mut fresh206 = (*(self_0 as *mut _spSkin)).entries;
        *fresh206 = newEntry;
        let mut hashTableIndex: c_uint = (slotIndex as c_uint)
            .wrapping_rem(100 as c_int as c_uint);
        let mut hashTable: *mut *mut _SkinHashTableEntry = ((*(self_0 as *mut _spSkin))
            .entriesHashTable)
            .as_mut_ptr();
        let mut newHashEntry: *mut _SkinHashTableEntry = _SkinHashTableEntry_create(
            newEntry,
        );
        (*newHashEntry).next = *hashTable.offset(hashTableIndex as isize);
        let ref mut fresh207 = (*(self_0 as *mut _spSkin))
            .entriesHashTable[hashTableIndex as usize];
        *fresh207 = newHashEntry;
    };
}
#[no_mangle]
pub unsafe extern "C" fn spSkin_getAttachment(
    mut self_0: *const spSkin,
    mut slotIndex: c_int,
    mut name: *const c_char,
) -> *mut spAttachment {
    let mut hashEntry: *const _SkinHashTableEntry = (*(self_0 as *mut _spSkin))
        .entriesHashTable[(slotIndex as c_uint)
        .wrapping_rem(100 as c_int as c_uint) as usize];
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
        let mut slot: *mut spSlot = *((*skeleton).slots)
            .offset((*entry).slotIndex as isize);
        if (*slot).attachment == (*entry).attachment {
            let mut attachment: *mut spAttachment = spSkin_getAttachment(
                self_0,
                (*entry).slotIndex,
                (*entry).name,
            );
            if !attachment.is_null() {
                spSlot_setAttachment(slot, attachment);
            }
        }
        entry = (*entry).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spSkin_addSkin(
    mut self_0: *mut spSkin,
    mut other: *const spSkin,
) {
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
pub unsafe extern "C" fn spSkin_copySkin(
    mut self_0: *mut spSkin,
    mut other: *const spSkin,
) {
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
        if (*(*entry).attachment).type_0 as c_uint
            == SP_ATTACHMENT_MESH as c_int as c_uint
        {
            let mut attachment: *mut spMeshAttachment = spMeshAttachment_newLinkedMesh(
                (*entry).attachment as *mut spMeshAttachment,
            );
            spSkin_setAttachment(
                self_0,
                (*entry).slotIndex,
                (*entry).name,
                &mut (*attachment).super_0.super_0,
            );
        } else {
            let mut attachment_0: *mut spAttachment = if !((*entry).attachment).is_null()
            {
                spAttachment_copy((*entry).attachment)
            } else {
                0 as *mut spAttachment
            };
            spSkin_setAttachment(
                self_0,
                (*entry).slotIndex,
                (*entry).name,
                attachment_0,
            );
        }
        entry = (*entry).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spSkin_getAttachments(
    mut self_0: *const spSkin,
) -> *mut spSkinEntry {
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
    let ref mut fresh208 = (*(self_0 as *mut _spSkin)).entries;
    *fresh208 = 0 as *mut _Entry;
    let mut currentHashtableEntry: *mut *mut _SkinHashTableEntry = ((*(self_0
        as *mut _spSkin))
        .entriesHashTable)
        .as_mut_ptr();
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < 100 as c_int {
        let mut hashtableEntry: *mut _SkinHashTableEntry = *currentHashtableEntry;
        while !hashtableEntry.is_null() {
            let mut nextEntry_0: *mut _SkinHashTableEntry = (*hashtableEntry).next;
            _SkinHashTableEntry_dispose(hashtableEntry);
            hashtableEntry = nextEntry_0;
        }
        let ref mut fresh209 = (*(self_0 as *mut _spSkin)).entriesHashTable[i as usize];
        *fresh209 = 0 as *mut _SkinHashTableEntry;
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
    let mut self_0: *mut spSlot = &mut (*((_spCalloc
        as unsafe extern "C" fn(
            size_t,
            size_t,
            *const c_char,
            c_int,
        ) -> *mut c_void)(
        1 as c_int as size_t,
        ::core::mem::size_of::<_spSlot>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        10004 as c_int,
    ) as *mut _spSlot))
        .super_0;
    let ref mut fresh210 = *(&(*self_0).data as *const *mut spSlotData
        as *mut *mut spSlotData);
    *fresh210 = data;
    let ref mut fresh211 = *(&(*self_0).bone as *const *mut spBone as *mut *mut spBone);
    *fresh211 = bone;
    spColor_setFromFloats(
        &mut (*self_0).color,
        1 as c_int as c_float,
        1 as c_int as c_float,
        1 as c_int as c_float,
        1 as c_int as c_float,
    );
    (*self_0)
        .darkColor = if ((*data).darkColor).is_null() {
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
#[no_mangle]
pub unsafe extern "C" fn spSlot_setAttachment(
    mut self_0: *mut spSlot,
    mut attachment: *mut spAttachment,
) {
    if attachment == (*self_0).attachment {
        return;
    }
    let ref mut fresh212 = *(&mut (*self_0).attachment as *mut *mut spAttachment);
    *fresh212 = attachment;
    (*(self_0 as *mut _spSlot)).attachmentTime = (*(*(*self_0).bone).skeleton).time;
    (*self_0).deformCount = 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spSlot_setAttachmentTime(
    mut self_0: *mut spSlot,
    mut time: c_float,
) {
    (*(self_0 as *mut _spSlot))
        .attachmentTime = (*(*(*self_0).bone).skeleton).time - time;
}
#[no_mangle]
pub unsafe extern "C" fn spSlot_getAttachmentTime(
    mut self_0: *const spSlot,
) -> c_float {
    return (*(*(*self_0).bone).skeleton).time
        - (*(self_0 as *mut _spSlot)).attachmentTime;
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
        let ref mut fresh213 = *(&mut (*self_0).attachment as *mut *mut spAttachment);
        *fresh213 = 0 as *mut spAttachment;
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
        10080 as c_int,
    ) as *mut spSlotData;
    *(&(*self_0).index as *const c_int as *mut c_int) = index;
    let ref mut fresh214 = *(&(*self_0).name as *const *const c_char
        as *mut *mut c_char);
    *fresh214 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul(
                (spine_strlen(name)).wrapping_add(1 as c_int as c_ulong),
            ),
        b"spine.c\0" as *const u8 as *const c_char,
        10082 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh214, name);
    let ref mut fresh215 = *(&(*self_0).boneData as *const *const spBoneData
        as *mut *mut spBoneData);
    *fresh215 = boneData;
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
        let ref mut fresh216 = *(&mut (*self_0).attachmentName
            as *mut *const c_char as *mut *mut c_char);
        *fresh216 = _spMalloc(
            (::core::mem::size_of::<c_char>() as c_ulong)
                .wrapping_mul(
                    (spine_strlen(attachmentName))
                        .wrapping_add(1 as c_int as c_ulong),
                ),
            b"spine.c\0" as *const u8 as *const c_char,
            10098 as c_int,
        ) as *mut c_char;
        spine_strcpy(*fresh216, attachmentName);
    } else {
        let ref mut fresh217 = *(&mut (*self_0).attachmentName
            as *mut *const c_char as *mut *mut c_char);
        *fresh217 = 0 as *mut c_char;
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
        10137 as c_int,
    ) as *mut spTransformConstraint;
    let ref mut fresh218 = *(&(*self_0).data as *const *mut spTransformConstraintData
        as *mut *mut spTransformConstraintData);
    *fresh218 = data;
    (*self_0).rotateMix = (*data).rotateMix;
    (*self_0).translateMix = (*data).translateMix;
    (*self_0).scaleMix = (*data).scaleMix;
    (*self_0).shearMix = (*data).shearMix;
    (*self_0).bonesCount = (*data).bonesCount;
    let ref mut fresh219 = *(&(*self_0).bones as *const *mut *mut spBone
        as *mut *mut *mut spBone);
    *fresh219 = _spMalloc(
        (::core::mem::size_of::<*mut spBone>() as c_ulong)
            .wrapping_mul((*self_0).bonesCount as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        10144 as c_int,
    ) as *mut *mut spBone;
    i = 0 as c_int;
    while i < (*self_0).bonesCount {
        let ref mut fresh220 = *((*self_0).bones).offset(i as isize);
        *fresh220 = spSkeleton_findBone(
            skeleton,
            (**((*(*self_0).data).bones).offset(i as isize)).name,
        );
        i += 1;
    }
    (*self_0).target = spSkeleton_findBone(skeleton, (*(*(*self_0).data).target).name);
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn spTransformConstraint_dispose(
    mut self_0: *mut spTransformConstraint,
) {
    _spFree((*self_0).bones as *mut c_void);
    _spFree(self_0 as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _spTransformConstraint_applyAbsoluteWorld(
    mut self_0: *mut spTransformConstraint,
) {
    let mut rotateMix: c_float = (*self_0).rotateMix;
    let mut translateMix: c_float = (*self_0).translateMix;
    let mut scaleMix: c_float = (*self_0).scaleMix;
    let mut shearMix: c_float = (*self_0).shearMix;
    let mut target: *mut spBone = (*self_0).target;
    let mut ta: c_float = (*target).a;
    let mut tb: c_float = (*target).b;
    let mut tc: c_float = (*target).c;
    let mut td: c_float = (*target).d;
    let mut degRadReflect: c_float = if ta * td - tb * tc
        > 0 as c_int as c_float
    {
        3.1415926535897932385f32 / 180 as c_int as c_float
    } else {
        -(3.1415926535897932385f32 / 180 as c_int as c_float)
    };
    let mut offsetRotation: c_float = (*(*self_0).data).offsetRotation
        * degRadReflect;
    let mut offsetShearY: c_float = (*(*self_0).data).offsetShearY * degRadReflect;
    let mut modified: c_int = 0;
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
    let mut ts: c_float = 0.;
    let mut by: c_float = 0.;
    i = 0 as c_int;
    while i < (*self_0).bonesCount {
        let mut bone: *mut spBone = *((*self_0).bones).offset(i as isize);
        modified = 0 as c_int;
        if rotateMix != 0 as c_int as c_float {
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
            r *= rotateMix;
            cosine = cosf(r);
            sine = sinf(r);
            *(&(*bone).a as *const c_float
                as *mut c_float) = cosine * a - sine * c;
            *(&(*bone).b as *const c_float
                as *mut c_float) = cosine * b - sine * d;
            *(&(*bone).c as *const c_float
                as *mut c_float) = sine * a + cosine * c;
            *(&(*bone).d as *const c_float
                as *mut c_float) = sine * b + cosine * d;
            modified = 1 as c_int;
        }
        if translateMix != 0 as c_int as c_float {
            spBone_localToWorld(
                target,
                (*(*self_0).data).offsetX,
                (*(*self_0).data).offsetY,
                &mut x,
                &mut y,
            );
            *(&(*bone).worldX as *const c_float as *mut c_float)
                += (x - (*bone).worldX) * translateMix;
            *(&(*bone).worldY as *const c_float as *mut c_float)
                += (y - (*bone).worldY) * translateMix;
            modified = 1 as c_int;
        }
        if scaleMix > 0 as c_int as c_float {
            s = spine_sqrtf((*bone).a * (*bone).a + (*bone).c * (*bone).c);
            ts = spine_sqrtf(ta * ta + tc * tc);
            if s > 0.00001f32 {
                s = (s + (ts - s + (*(*self_0).data).offsetScaleX) * scaleMix) / s;
            }
            *(&(*bone).a as *const c_float as *mut c_float) *= s;
            *(&(*bone).c as *const c_float as *mut c_float) *= s;
            s = spine_sqrtf((*bone).b * (*bone).b + (*bone).d * (*bone).d);
            ts = spine_sqrtf(tb * tb + td * td);
            if s > 0.00001f32 {
                s = (s + (ts - s + (*(*self_0).data).offsetScaleY) * scaleMix) / s;
            }
            *(&(*bone).b as *const c_float as *mut c_float) *= s;
            *(&(*bone).d as *const c_float as *mut c_float) *= s;
            modified = 1 as c_int;
        }
        if shearMix > 0 as c_int as c_float {
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
            r = by + (r + offsetShearY) * shearMix;
            *(&(*bone).b as *const c_float as *mut c_float) = cosf(r) * s;
            *(&(*bone).d as *const c_float as *mut c_float) = sinf(r) * s;
            modified = 1 as c_int;
        }
        if modified != 0 {
            *(&mut (*bone).appliedValid as *mut c_int) = 0 as c_int;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _spTransformConstraint_applyRelativeWorld(
    mut self_0: *mut spTransformConstraint,
) {
    let mut rotateMix: c_float = (*self_0).rotateMix;
    let mut translateMix: c_float = (*self_0).translateMix;
    let mut scaleMix: c_float = (*self_0).scaleMix;
    let mut shearMix: c_float = (*self_0).shearMix;
    let mut target: *mut spBone = (*self_0).target;
    let mut ta: c_float = (*target).a;
    let mut tb: c_float = (*target).b;
    let mut tc: c_float = (*target).c;
    let mut td: c_float = (*target).d;
    let mut degRadReflect: c_float = if ta * td - tb * tc
        > 0 as c_int as c_float
    {
        3.1415926535897932385f32 / 180 as c_int as c_float
    } else {
        -(3.1415926535897932385f32 / 180 as c_int as c_float)
    };
    let mut offsetRotation: c_float = (*(*self_0).data).offsetRotation
        * degRadReflect;
    let mut offsetShearY: c_float = (*(*self_0).data).offsetShearY * degRadReflect;
    let mut modified: c_int = 0;
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
        modified = 0 as c_int;
        if rotateMix != 0 as c_int as c_float {
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
            r *= rotateMix;
            cosine = cosf(r);
            sine = sinf(r);
            *(&(*bone).a as *const c_float
                as *mut c_float) = cosine * a - sine * c;
            *(&(*bone).b as *const c_float
                as *mut c_float) = cosine * b - sine * d;
            *(&(*bone).c as *const c_float
                as *mut c_float) = sine * a + cosine * c;
            *(&(*bone).d as *const c_float
                as *mut c_float) = sine * b + cosine * d;
            modified = 1 as c_int;
        }
        if translateMix != 0 as c_int as c_float {
            spBone_localToWorld(
                target,
                (*(*self_0).data).offsetX,
                (*(*self_0).data).offsetY,
                &mut x,
                &mut y,
            );
            *(&(*bone).worldX as *const c_float as *mut c_float)
                += x * translateMix;
            *(&(*bone).worldY as *const c_float as *mut c_float)
                += y * translateMix;
            modified = 1 as c_int;
        }
        if scaleMix > 0 as c_int as c_float {
            s = (spine_sqrtf(ta * ta + tc * tc) - 1 as c_int as c_float
                + (*(*self_0).data).offsetScaleX) * scaleMix
                + 1 as c_int as c_float;
            *(&(*bone).a as *const c_float as *mut c_float) *= s;
            *(&(*bone).c as *const c_float as *mut c_float) *= s;
            s = (spine_sqrtf(tb * tb + td * td) - 1 as c_int as c_float
                + (*(*self_0).data).offsetScaleY) * scaleMix
                + 1 as c_int as c_float;
            *(&(*bone).b as *const c_float as *mut c_float) *= s;
            *(&(*bone).d as *const c_float as *mut c_float) *= s;
            modified = 1 as c_int;
        }
        if shearMix > 0 as c_int as c_float {
            r = atan2f(td, tb) - atan2f(tc, ta);
            if r > 3.1415926535897932385f32 {
                r -= 3.1415926535897932385f32 * 2 as c_int as c_float;
            } else if r < -3.1415926535897932385f32 {
                r += 3.1415926535897932385f32 * 2 as c_int as c_float;
            }
            b = (*bone).b;
            d = (*bone).d;
            r = atan2f(d, b)
                + (r - 3.1415926535897932385f32 / 2 as c_int as c_float
                    + offsetShearY) * shearMix;
            s = spine_sqrtf(b * b + d * d);
            *(&(*bone).b as *const c_float as *mut c_float) = cosf(r) * s;
            *(&(*bone).d as *const c_float as *mut c_float) = sinf(r) * s;
            modified = 1 as c_int;
        }
        if modified != 0 {
            *(&mut (*bone).appliedValid as *mut c_int) = 0 as c_int;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _spTransformConstraint_applyAbsoluteLocal(
    mut self_0: *mut spTransformConstraint,
) {
    let mut rotateMix: c_float = (*self_0).rotateMix;
    let mut translateMix: c_float = (*self_0).translateMix;
    let mut scaleMix: c_float = (*self_0).scaleMix;
    let mut shearMix: c_float = (*self_0).shearMix;
    let mut target: *mut spBone = (*self_0).target;
    let mut i: c_int = 0;
    let mut rotation: c_float = 0.;
    let mut r: c_float = 0.;
    let mut x: c_float = 0.;
    let mut y: c_float = 0.;
    let mut scaleX: c_float = 0.;
    let mut scaleY: c_float = 0.;
    let mut shearY: c_float = 0.;
    if (*target).appliedValid == 0 {
        spBone_updateAppliedTransform(target);
    }
    i = 0 as c_int;
    while i < (*self_0).bonesCount {
        let mut bone: *mut spBone = *((*self_0).bones).offset(i as isize);
        if (*bone).appliedValid == 0 {
            spBone_updateAppliedTransform(bone);
        }
        rotation = (*bone).arotation;
        if rotateMix != 0 as c_int as c_float {
            r = (*target).arotation - rotation + (*(*self_0).data).offsetRotation;
            r
                -= ((16384 as c_int
                    - (16384.499999999996f64
                        - (r / 360 as c_int as c_float) as c_double)
                        as c_int) * 360 as c_int) as c_float;
            rotation += r * rotateMix;
        }
        x = (*bone).ax;
        y = (*bone).ay;
        if translateMix != 0 as c_int as c_float {
            x += ((*target).ax - x + (*(*self_0).data).offsetX) * translateMix;
            y += ((*target).ay - y + (*(*self_0).data).offsetY) * translateMix;
        }
        scaleX = (*bone).ascaleX;
        scaleY = (*bone).ascaleY;
        if scaleMix != 0 as c_int as c_float {
            if scaleX as c_double > 0.00001f64 {
                scaleX = (scaleX
                    + ((*target).ascaleX - scaleX + (*(*self_0).data).offsetScaleX)
                        * scaleMix) / scaleX;
            }
            if scaleY as c_double > 0.00001f64 {
                scaleY = (scaleY
                    + ((*target).ascaleY - scaleY + (*(*self_0).data).offsetScaleY)
                        * scaleMix) / scaleY;
            }
        }
        shearY = (*bone).ashearY;
        if shearMix != 0 as c_int as c_float {
            r = (*target).ashearY - shearY + (*(*self_0).data).offsetShearY;
            r
                -= ((16384 as c_int
                    - (16384.499999999996f64
                        - (r / 360 as c_int as c_float) as c_double)
                        as c_int) * 360 as c_int) as c_float;
            (*bone).shearY += r * shearMix;
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
    let mut rotateMix: c_float = (*self_0).rotateMix;
    let mut translateMix: c_float = (*self_0).translateMix;
    let mut scaleMix: c_float = (*self_0).scaleMix;
    let mut shearMix: c_float = (*self_0).shearMix;
    let mut target: *mut spBone = (*self_0).target;
    let mut i: c_int = 0;
    let mut rotation: c_float = 0.;
    let mut x: c_float = 0.;
    let mut y: c_float = 0.;
    let mut scaleX: c_float = 0.;
    let mut scaleY: c_float = 0.;
    let mut shearY: c_float = 0.;
    if (*target).appliedValid == 0 {
        spBone_updateAppliedTransform(target);
    }
    i = 0 as c_int;
    while i < (*self_0).bonesCount {
        let mut bone: *mut spBone = *((*self_0).bones).offset(i as isize);
        if (*bone).appliedValid == 0 {
            spBone_updateAppliedTransform(bone);
        }
        rotation = (*bone).arotation;
        if rotateMix != 0 as c_int as c_float {
            rotation
                += ((*target).arotation + (*(*self_0).data).offsetRotation) * rotateMix;
        }
        x = (*bone).ax;
        y = (*bone).ay;
        if translateMix != 0 as c_int as c_float {
            x += ((*target).ax + (*(*self_0).data).offsetX) * translateMix;
            y += ((*target).ay + (*(*self_0).data).offsetY) * translateMix;
        }
        scaleX = (*bone).ascaleX;
        scaleY = (*bone).ascaleY;
        if scaleMix != 0 as c_int as c_float {
            if scaleX > 0.00001f32 {
                scaleX
                    *= ((*target).ascaleX - 1 as c_int as c_float
                        + (*(*self_0).data).offsetScaleX) * scaleMix
                        + 1 as c_int as c_float;
            }
            if scaleY > 0.00001f32 {
                scaleY
                    *= ((*target).ascaleY - 1 as c_int as c_float
                        + (*(*self_0).data).offsetScaleY) * scaleMix
                        + 1 as c_int as c_float;
            }
        }
        shearY = (*bone).ashearY;
        if shearMix != 0 as c_int as c_float {
            shearY += ((*target).ashearY + (*(*self_0).data).offsetShearY) * shearMix;
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
pub unsafe extern "C" fn spTransformConstraint_apply(
    mut self_0: *mut spTransformConstraint,
) {
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
        10407 as c_int,
    ) as *mut spTransformConstraintData;
    let ref mut fresh221 = *(&(*self_0).name as *const *const c_char
        as *mut *mut c_char);
    *fresh221 = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul(
                (spine_strlen(name)).wrapping_add(1 as c_int as c_ulong),
            ),
        b"spine.c\0" as *const u8 as *const c_char,
        10408 as c_int,
    ) as *mut c_char;
    spine_strcpy(*fresh221, name);
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
        10451 as c_int,
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
        spShortArray_dispose(
            *((*(*self_0).convexPolygonsIndices).items).offset(i as isize),
        );
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
        spShortArray_dispose(
            *((*(*self_0).polygonIndicesPool).items).offset(i as isize),
        );
        i += 1;
    }
    spArrayShortArray_dispose((*self_0).polygonIndicesPool);
    _spFree(self_0 as *mut c_void);
}
unsafe extern "C" fn _obtainPolygon(
    mut self_0: *mut spTriangulator,
) -> *mut spFloatArray {
    if (*(*self_0).polygonPool).size == 0 as c_int {
        return spFloatArray_create(16 as c_int)
    } else {
        return spArrayFloatArray_pop((*self_0).polygonPool)
    };
}
unsafe extern "C" fn _freePolygon(
    mut self_0: *mut spTriangulator,
    mut polygon: *mut spFloatArray,
) {
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
unsafe extern "C" fn _obtainPolygonIndices(
    mut self_0: *mut spTriangulator,
) -> *mut spShortArray {
    if (*(*self_0).polygonIndicesPool).size == 0 as c_int {
        return spShortArray_create(16 as c_int)
    } else {
        return spArrayShortArray_pop((*self_0).polygonIndicesPool)
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
    return (p1x * (p3y - p2y) + p2x * (p1y - p3y) + p3x * (p2y - p1y)
        >= 0 as c_int as c_float) as c_int;
}
unsafe extern "C" fn _isConcave(
    mut index: c_int,
    mut vertexCount: c_int,
    mut vertices: *mut c_float,
    mut indices: *mut c_short,
) -> c_int {
    let mut previous: c_int = (*indices
        .offset(((vertexCount + index - 1 as c_int) % vertexCount) as isize)
        as c_int) << 1 as c_int;
    let mut current: c_int = (*indices.offset(index as isize) as c_int)
        << 1 as c_int;
    let mut next: c_int = (*indices
        .offset(((index + 1 as c_int) % vertexCount) as isize) as c_int)
        << 1 as c_int;
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
    return if p3x * py - p3y * px + px * p1y - p1x * py
        >= 0 as c_int as c_float
    {
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
                let mut p1: c_int = (*indices.offset(previous as isize)
                    as c_int) << 1 as c_int;
                let mut p2: c_int = (*indices.offset(i as isize) as c_int)
                    << 1 as c_int;
                let mut p3: c_int = (*indices.offset(next as isize) as c_int)
                    << 1 as c_int;
                let mut p1x: c_float = *vertices.offset(p1 as isize);
                let mut p1y: c_float = *vertices
                    .offset((p1 + 1 as c_int) as isize);
                let mut p2x: c_float = *vertices.offset(p2 as isize);
                let mut p2y: c_float = *vertices
                    .offset((p2 + 1 as c_int) as isize);
                let mut p3x: c_float = *vertices.offset(p3 as isize);
                let mut p3y: c_float = *vertices
                    .offset((p3 + 1 as c_int) as isize);
                ii = (next + 1 as c_int) % vertexCount;
                loop {
                    if !(ii != previous) {
                        break 's_80;
                    }
                    let mut v: c_int = 0;
                    let mut vx: c_float = 0.;
                    let mut vy: c_float = 0.;
                    if !(*isConcave.offset(ii as isize) == 0) {
                        v = (*indices.offset(ii as isize) as c_int)
                            << 1 as c_int;
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
            *indices
                .offset(((vertexCount + i - 1 as c_int) % vertexCount) as isize),
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
        *isConcave
            .offset(
                previousIndex as isize,
            ) = _isConcave(previousIndex, vertexCount, vertices, indices);
        *isConcave
            .offset(
                nextIndex as isize,
            ) = _isConcave(nextIndex, vertexCount, vertices, indices);
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
        let mut t1: c_int = (*trianglesItems.offset(i as isize) as c_int)
            << 1 as c_int;
        let mut t2: c_int = (*trianglesItems
            .offset((i + 1 as c_int) as isize) as c_int) << 1 as c_int;
        let mut t3: c_int = (*trianglesItems
            .offset((i + 2 as c_int) as isize) as c_int) << 1 as c_int;
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
            firstIndex = *((*polygonIndices).items).offset(0 as c_int as isize)
                as c_int;
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
                        otherFirstIndex = *((*otherIndices).items)
                            .offset(0 as c_int as isize) as c_int;
                        otherSecondIndex = *((*otherIndices).items)
                            .offset(1 as c_int as isize) as c_int;
                        otherLastIndex = *((*otherIndices).items)
                            .offset(2 as c_int as isize) as c_int;
                        otherPoly = *((*convexPolygons).items).offset(ii as isize);
                        x3_0 = *((*otherPoly).items)
                            .offset(((*otherPoly).size - 2 as c_int) as isize);
                        y3_0 = *((*otherPoly).items)
                            .offset(((*otherPoly).size - 1 as c_int) as isize);
                        if !(otherFirstIndex != firstIndex
                            || otherSecondIndex != lastIndex)
                        {
                            winding1_0 = _winding(
                                prevPrevX,
                                prevPrevY,
                                prevX,
                                prevY,
                                x3_0,
                                y3_0,
                            );
                            winding2_0 = _winding(
                                x3_0,
                                y3_0,
                                firstX,
                                firstY,
                                secondX,
                                secondY,
                            );
                            if winding1_0 == winding && winding2_0 == winding {
                                spFloatArray_clear(otherPoly);
                                spShortArray_clear(otherIndices);
                                spFloatArray_add(polygon, x3_0);
                                spFloatArray_add(polygon, y3_0);
                                spShortArray_add(
                                    polygonIndices,
                                    otherLastIndex as c_short,
                                );
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
pub unsafe extern "C" fn _spVertexAttachment_init(
    mut attachment: *mut spVertexAttachment,
) {
    let fresh222 = nextID;
    nextID = nextID + 1;
    (*attachment).id = (fresh222 & 65535 as c_int) << 11 as c_int;
    (*attachment).deformAttachment = attachment;
}
#[no_mangle]
pub unsafe extern "C" fn _spVertexAttachment_deinit(
    mut attachment: *mut spVertexAttachment,
) {
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
            let mut vy: c_float = *vertices
                .offset((v + 1 as c_int) as isize);
            *worldVertices.offset(w as isize) = vx * (*bone).a + vy * (*bone).b + x;
            *worldVertices
                .offset(
                    (w + 1 as c_int) as isize,
                ) = vx * (*bone).c + vy * (*bone).d + y;
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
                let fresh223 = v_0;
                v_0 = v_0 + 1;
                let mut n_0: c_int = *bones.offset(fresh223 as isize);
                n_0 += v_0;
                while v_0 < n_0 {
                    let mut bone_0: *mut spBone = *skeletonBones
                        .offset(*bones.offset(v_0 as isize) as isize);
                    let mut vx_0: c_float = *vertices.offset(b as isize);
                    let mut vy_0: c_float = *vertices
                        .offset((b + 1 as c_int) as isize);
                    let mut weight: c_float = *vertices
                        .offset((b + 2 as c_int) as isize);
                    wx
                        += (vx_0 * (*bone_0).a + vy_0 * (*bone_0).b + (*bone_0).worldX)
                            * weight;
                    wy
                        += (vx_0 * (*bone_0).c + vy_0 * (*bone_0).d + (*bone_0).worldY)
                            * weight;
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
                let fresh224 = v_0;
                v_0 = v_0 + 1;
                let mut n_1: c_int = *bones.offset(fresh224 as isize);
                n_1 += v_0;
                while v_0 < n_1 {
                    let mut bone_1: *mut spBone = *skeletonBones
                        .offset(*bones.offset(v_0 as isize) as isize);
                    let mut vx_1: c_float = *vertices.offset(b_0 as isize)
                        + *deformArray.offset(f as isize);
                    let mut vy_1: c_float = *vertices
                        .offset((b_0 + 1 as c_int) as isize)
                        + *deformArray.offset((f + 1 as c_int) as isize);
                    let mut weight_0: c_float = *vertices
                        .offset((b_0 + 2 as c_int) as isize);
                    wx_0
                        += (vx_1 * (*bone_1).a + vy_1 * (*bone_1).b + (*bone_1).worldX)
                            * weight_0;
                    wy_0
                        += (vx_1 * (*bone_1).c + vy_1 * (*bone_1).d + (*bone_1).worldY)
                            * weight_0;
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
        (*to)
            .bones = _spMalloc(
            (::core::mem::size_of::<c_int>() as c_ulong)
                .wrapping_mul((*from).bonesCount as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            10896 as c_int,
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
        (*to)
            .vertices = _spMalloc(
            (::core::mem::size_of::<c_float>() as c_ulong)
                .wrapping_mul((*from).verticesCount as c_ulong),
            b"spine.c\0" as *const u8 as *const c_char,
            10908 as c_int,
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
pub unsafe extern "C" fn _spJitterVertexEffect_begin(
    mut _self_0: *mut spVertexEffect,
    mut _skeleton: *mut spSkeleton,
) {}
#[no_mangle]
pub unsafe extern "C" fn _spJitterVertexEffect_transform(
    mut self_0: *mut spVertexEffect,
    mut x: *mut c_float,
    mut y: *mut c_float,
    mut _u: *mut c_float,
    mut _v: *mut c_float,
    mut _light: *mut spColor,
    mut _dark: *mut spColor,
) {
    let mut internal: *mut spJitterVertexEffect = self_0 as *mut spJitterVertexEffect;
    let mut jitterX: c_float = (*internal).jitterX;
    let mut jitterY: c_float = (*internal).jitterY;
    *x += _spMath_randomTriangular(-jitterX, jitterY);
    *y += _spMath_randomTriangular(-jitterX, jitterY);
}
#[no_mangle]
pub unsafe extern "C" fn _spJitterVertexEffect_end(mut _self_0: *mut spVertexEffect) {}
#[no_mangle]
pub unsafe extern "C" fn spJitterVertexEffect_create(
    mut jitterX: c_float,
    mut jitterY: c_float,
) -> *mut spJitterVertexEffect {
    let mut effect: *mut spJitterVertexEffect = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spJitterVertexEffect>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        10973 as c_int,
    ) as *mut spJitterVertexEffect;
    (*effect)
        .super_0
        .begin = Some(
        _spJitterVertexEffect_begin
            as unsafe extern "C" fn(*mut spVertexEffect, *mut spSkeleton) -> (),
    );
    (*effect)
        .super_0
        .transform = Some(
        _spJitterVertexEffect_transform
            as unsafe extern "C" fn(
                *mut spVertexEffect,
                *mut c_float,
                *mut c_float,
                *mut c_float,
                *mut c_float,
                *mut spColor,
                *mut spColor,
            ) -> (),
    );
    (*effect)
        .super_0
        .end = Some(
        _spJitterVertexEffect_end as unsafe extern "C" fn(*mut spVertexEffect) -> (),
    );
    (*effect).jitterX = jitterX;
    (*effect).jitterY = jitterY;
    return effect;
}
#[no_mangle]
pub unsafe extern "C" fn spJitterVertexEffect_dispose(
    mut effect: *mut spJitterVertexEffect,
) {
    _spFree(effect as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _spSwirlVertexEffect_begin(
    mut self_0: *mut spVertexEffect,
    mut skeleton: *mut spSkeleton,
) {
    let mut internal: *mut spSwirlVertexEffect = self_0 as *mut spSwirlVertexEffect;
    (*internal).worldX = (*skeleton).x + (*internal).centerX;
    (*internal).worldY = (*skeleton).y + (*internal).centerY;
}
#[no_mangle]
pub unsafe extern "C" fn _spSwirlVertexEffect_transform(
    mut self_0: *mut spVertexEffect,
    mut positionX: *mut c_float,
    mut positionY: *mut c_float,
    mut _u: *mut c_float,
    mut _v: *mut c_float,
    mut _light: *mut spColor,
    mut _dark: *mut spColor,
) {
    let mut internal: *mut spSwirlVertexEffect = self_0 as *mut spSwirlVertexEffect;
    let mut radAngle: c_float = (*internal).angle
        * (3.1415926535897932385f32 / 180 as c_int as c_float);
    let mut x: c_float = *positionX - (*internal).worldX;
    let mut y: c_float = *positionY - (*internal).worldY;
    let mut dist: c_float = spine_sqrtf(x * x + y * y);
    if dist < (*internal).radius {
        let mut theta: c_float = _spMath_interpolate(
            Some(
                _spMath_pow2_apply
                    as unsafe extern "C" fn(c_float) -> c_float,
            ),
            0 as c_int as c_float,
            radAngle,
            ((*internal).radius - dist) / (*internal).radius,
        );
        let mut cosine: c_float = cosf(theta);
        let mut sine: c_float = sinf(theta);
        *positionX = cosine * x - sine * y + (*internal).worldX;
        *positionY = sine * x + cosine * y + (*internal).worldY;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _spSwirlVertexEffect_end(mut _self_0: *mut spVertexEffect) {}
#[no_mangle]
pub unsafe extern "C" fn spSwirlVertexEffect_create(
    mut radius: c_float,
) -> *mut spSwirlVertexEffect {
    let mut effect: *mut spSwirlVertexEffect = _spCalloc(
        1 as c_int as size_t,
        ::core::mem::size_of::<spSwirlVertexEffect>() as c_ulong,
        b"spine.c\0" as *const u8 as *const c_char,
        11017 as c_int,
    ) as *mut spSwirlVertexEffect;
    (*effect)
        .super_0
        .begin = Some(
        _spSwirlVertexEffect_begin
            as unsafe extern "C" fn(*mut spVertexEffect, *mut spSkeleton) -> (),
    );
    (*effect)
        .super_0
        .transform = Some(
        _spSwirlVertexEffect_transform
            as unsafe extern "C" fn(
                *mut spVertexEffect,
                *mut c_float,
                *mut c_float,
                *mut c_float,
                *mut c_float,
                *mut spColor,
                *mut spColor,
            ) -> (),
    );
    (*effect)
        .super_0
        .end = Some(
        _spSwirlVertexEffect_end as unsafe extern "C" fn(*mut spVertexEffect) -> (),
    );
    (*effect).radius = radius;
    return effect;
}
#[no_mangle]
pub unsafe extern "C" fn spSwirlVertexEffect_dispose(
    mut effect: *mut spSwirlVertexEffect,
) {
    _spFree(effect as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _spInternalRandom() -> c_float {
    return spine_rand() as c_float / 2147483647 as c_int as c_float;
}
static mut mallocFunc: Option::<unsafe extern "C" fn(size_t) -> *mut c_void> = {
    Some(spine_malloc as unsafe extern "C" fn(size_t) -> *mut c_void)
};
static mut reallocFunc: Option::<
    unsafe extern "C" fn(*mut c_void, size_t) -> *mut c_void,
> = {
    Some(
        spine_realloc
            as unsafe extern "C" fn(*mut c_void, size_t) -> *mut c_void,
    )
};
static mut debugMallocFunc: Option::<
    unsafe extern "C" fn(size_t, *const c_char, c_int) -> *mut c_void,
> = None;
static mut freeFunc: Option::<unsafe extern "C" fn(*mut c_void) -> ()> = {
    Some(spine_free as unsafe extern "C" fn(*mut c_void) -> ())
};
static mut randomFunc: Option::<unsafe extern "C" fn() -> c_float> = unsafe {
    Some(
        ::core::mem::transmute::<
            unsafe extern "C" fn() -> c_float,
            unsafe extern "C" fn() -> c_float,
        >(_spInternalRandom),
    )
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
pub unsafe extern "C" fn _spRealloc(
    mut ptr: *mut c_void,
    mut size: size_t,
) -> *mut c_void {
    return reallocFunc.expect("non-null function pointer")(ptr, size);
}
#[no_mangle]
pub unsafe extern "C" fn _spFree(mut ptr: *mut c_void) {
    freeFunc.expect("non-null function pointer")(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn _spRandom() -> c_float {
    return ::core::mem::transmute::<
        _,
        fn() -> c_float,
    >(randomFunc.expect("non-null function pointer"))();
}
#[no_mangle]
pub unsafe extern "C" fn _spSetDebugMalloc(
    mut spine_malloc_0: Option::<
        unsafe extern "C" fn(
            size_t,
            *const c_char,
            c_int,
        ) -> *mut c_void,
    >,
) {
    debugMallocFunc = spine_malloc_0;
}
#[no_mangle]
pub unsafe extern "C" fn _spSetMalloc(
    mut spine_malloc_0: Option::<unsafe extern "C" fn(size_t) -> *mut c_void>,
) {
    mallocFunc = spine_malloc_0;
}
#[no_mangle]
pub unsafe extern "C" fn _spSetRealloc(
    mut spine_realloc_0: Option::<
        unsafe extern "C" fn(*mut c_void, size_t) -> *mut c_void,
    >,
) {
    reallocFunc = spine_realloc_0;
}
#[no_mangle]
pub unsafe extern "C" fn _spSetFree(
    mut spine_free_0: Option::<unsafe extern "C" fn(*mut c_void) -> ()>,
) {
    freeFunc = spine_free_0;
}
#[no_mangle]
pub unsafe extern "C" fn _spSetRandom(
    mut random: Option::<unsafe extern "C" fn() -> c_float>,
) {
    randomFunc = random;
}
#[no_mangle]
pub unsafe extern "C" fn _spReadFile(
    mut path: *const c_char,
    mut length: *mut c_int,
) -> *mut c_char {
    let mut data: *mut c_char = 0 as *mut c_char;
    #[allow(unused_variables)]
    let mut result: size_t;
    let mut file: *mut FILE = spine_fopen(
        path,
        b"rb\0" as *const u8 as *const c_char,
    );
    if file.is_null() {
        return 0 as *mut c_char;
    }
    spine_fseek(file, 0 as c_int as c_long, 2 as c_int);
    *length = spine_ftell(file) as c_int;
    spine_fseek(file, 0 as c_int as c_long, 0 as c_int);
    data = _spMalloc(
        (::core::mem::size_of::<c_char>() as c_ulong)
            .wrapping_mul(*length as c_ulong),
        b"spine.c\0" as *const u8 as *const c_char,
        11122 as c_int,
    ) as *mut c_char;
    result = spine_fread(
        data as *mut c_void,
        1 as c_int as size_t,
        *length as size_t,
        file,
    );
    spine_fclose(file);
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn _spMath_random(
    mut min: c_float,
    mut max: c_float,
) -> c_float {
    return min + (max - min) * _spRandom();
}
#[no_mangle]
pub unsafe extern "C" fn _spMath_randomTriangular(
    mut min: c_float,
    mut max: c_float,
) -> c_float {
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
    mut apply: Option::<unsafe extern "C" fn(c_float) -> c_float>,
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
        ((a - 1 as c_int as c_float) * 2 as c_int as c_float)
            as c_double,
        2 as c_int as c_double,
    ) / -(2 as c_int) as c_double + 1 as c_int as c_double)
        as c_float;
}
#[no_mangle]
pub unsafe extern "C" fn _spMath_pow2out_apply(mut a: c_float) -> c_float {
    return (pow(
        (a - 1 as c_int as c_float) as c_double,
        2 as c_int as c_double,
    ) * -(1 as c_int) as c_double + 1 as c_int as c_double)
        as c_float;
}


type _IO_wide_data = u8;
type _IO_codecvt = u8;
type _IO_marker = u8;
pub use crate::c::environment::types::*;
