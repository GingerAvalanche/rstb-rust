use super::*;

#[repr(C)]
pub struct Physics<T> {
    base:       ParamIO<T>,     // ParamIO
    base2:      Resource<T>,    // Resource
    mParamSet:  ParamSet<T>,    // phys::ParamSet
}

#[repr(C)]
pub struct ParamSet<T> {
    base:                               agl::ParameterList<T>,      // agl::utl::ParameterList
    rigidBodySets:                      sead::Buffer<T>,            // sead::Buffer<RigidBodySetParam>
    character_controller:               T,                          // CharacterControllerParam*
    cloth_set:                          T,                          // ClothSetParam*
    ragdoll:                            T,                          // RagdollParam*
    support_bone:                       T,                          // SupportBoneParam*
    context_info:                       T,                          // ContactInfoParam*
    edge_rigid_body_set:                T,                          // EdgeRigidBodySetParam*
    obj:                                agl::ParameterObj<T>,       // agl::utl::ParameterObj
    use_rigid_body_set_num:             agl::Parameter<T, i32>,     // agl::utl::Parameter<int>
    use_character_controller:           agl::Parameter<T, bool>,    // agl::utl::Parameter<bool>
    use_ragdoll:                        agl::Parameter<T, bool>,    // agl::utl::Parameter<bool>
    use_support_bone:                   agl::Parameter<T, bool>,    // agl::utl::Parameter<bool>
    use_cloth:                          agl::Parameter<T, bool>,    // agl::utl::Parameter<bool>
    use_contact_info:                   agl::Parameter<T, bool>,    // agl::utl::Parameter<bool>
    use_system_group_handler:           agl::Parameter<T, bool>,    // agl::utl::Parameter<bool>
    user_edge_rigid_body_num:           agl::Parameter<T, i32>,     // agl::utl::Parameter<int>
    num_rigid_bodies_with_link_matrix:  i32,                        // int
}

#[repr(C)]
pub struct RigidBodySetParam<T> {
    base:               agl::ParameterList<T>,                  // agl::utl::ParameterList
    type_val:           u32,                                    // Type - enum
    obj:                agl::ParameterObj<T>,                   // agl::utl::ParameterObj
    set_name:           agl::Parameter<T, sead::SafeString<T>>, // agl::utl::Parameter<sead::SafeString>
    type_:              agl::Parameter<T, sead::SafeString<T>>, // agl::utl::Parameter<sead::SafeString>
    num:                agl::Parameter<T, i32>,                 // agl::utl::Parameter<int>
    setup_file_path:    agl::Parameter<T, sead::SafeString<T>>, // agl::utl::Parameter<sead::SafeString>
    rigid_bodies:       sead::Buffer<T>,                        // sead::Buffer<RigidBodyParam>
}

#[repr(C)]
pub struct CharacterControllerParam<T> {
    base:                                       agl::ParameterList<T>,
    base2:                                      ICharacterControllerParam<T>,
    obj:                                        agl::ParameterObj<T>,
    mass:                                       agl::Parameter<T, f32>,
    volume:                                     agl::Parameter<T, f32>,
    max_force:                                  agl::Parameter<T, f32>,
    form_num:                                   agl::Parameter<T, i32>,
    layer:                                      agl::Parameter<T, sead::FixedSafeString<T, 32>>,
    groundhit:                                  agl::Parameter<T, sead::FixedSafeString<T, 32>>,
    initial_state:                              agl::Parameter<T, sead::FixedSafeString<T, 32>>,
    initial_form:                               agl::Parameter<T, sead::FixedSafeString<T, 32>>,
    max_impulse:                                agl::Parameter<T, f32>,
    contact_point_info:                         agl::Parameter<T, sead::FixedSafeString<T, 32>>,
    collision_info:                             agl::Parameter<T, sead::FixedSafeString<T, 32>>,
    use_nav_mesh_character:                     agl::Parameter<T, bool>,
    nav_mesh_character_radius:                  agl::Parameter<T, f32>,
    nav_mesh_character_height:                  agl::Parameter<T, f32>,
    nav_mesh_character_avoidance_priority:      agl::Parameter<T, u32>,
    nav_mesh_character_max_speed:               agl::Parameter<T, f32>,
    nav_mesh_character_max_acceleration:        agl::Parameter<T, f32>,
    nav_mesh_character_max_angular_velocity:    agl::Parameter<T, f32>,
    nav_mesh_character_type:                    agl::Parameter<T, sead::SafeString<T>>,
    enable_water_effect:                        agl::Parameter<T, bool>,
    enable_force_fall_cliff_edge:               agl::Parameter<T, bool>,
    water_effective_height:                     agl::Parameter<T, f32>,
    water_flow_effective_rate:                  agl::Parameter<T, f32>,
    water_attn_effective_rate:                  agl::Parameter<T, f32>,
    max_force_scale_NPC:                        agl::Parameter<T, f32>,
    water_buoyancy_scale:                       agl::Parameter<T, f32>,
    magne_mass_scaling_factor:                  agl::Parameter<T, f32>,
    height_enable_hitting_wall:                 agl::Parameter<T, f32>,
    forms:                                      sead::Buffer<T>,
}

#[repr(C)]
pub struct ICharacterControllerParam<T> {
    vfptr:  T,  // vfptr*
}

#[repr(C)]
pub struct Form<T> {
    base:               agl::ParameterList<T>,
    form_header_obj:    agl::ParameterObj<T>,
    shape_num:          agl::Parameter<T, i32>,
    form_type:          agl::Parameter<T, sead::FixedSafeString<T, 32>>,
    shape_params:       sead::Buffer<T>,
}

#[repr(C)]
pub struct ClothSetParam<T> {
    base:                   agl::ParameterList<T>,
    cloth_header_obj:       agl::ParameterObj<T>,
    cloth_setup_file_path:  agl::Parameter<T, sead::SafeString<T>>,
    cloth_num:              agl::Parameter<T, i32>,
    cloth_setup_file_name:  sead::FixedSafeString<T, 64>,
    sub_wind:               ClothSubWindParam<T>,
    cloths:                 sead::Buffer<T>,
}

#[repr(C)]
pub struct RagdollParam<T> {
    base:                       agl::ParameterObj<T>,
    contact_point_info:         agl::Parameter<T, sead::SafeString<T>>,
    collision_info:             agl::Parameter<T, sead::SafeString<T>>,
    ragdoll_setup_file_path:    agl::Parameter<T, sead::SafeString<T>>,
    ragdoll_setup_file_stem:    sead::FixedSafeString<T, 32>,
}

#[repr(C)]
pub struct SupportBoneParam<T> {
    base:                           agl::ParameterObj<T>,
    support_bone_setup_file_path:   agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct ContactInfoParam<T> {
    base:                   agl::ParameterList<T>,
    contact_point_info:     sead::Buffer<T>,
    collision_info:         sead::Buffer<T>,
    obj:                    agl::ParameterObj<T>,
    contact_point_info_num: agl::Parameter<T, i32>,
    collision_info_num:     agl::Parameter<T, i32>,
}

#[repr(C)]
pub struct EdgeRigidBodySetParam<T> {
    base:               agl::ParameterList<T>,
    edge_rigid_bodies:  sead::Buffer<T>,
}

#[repr(C)]
pub struct Info<T> {
    base:                           agl::ParameterObj<T>,
    rigid_body_name:                agl::Parameter<T, sead::SafeString<T>>,
    mass:                           agl::Parameter<T, f32>,
    volume:                         agl::Parameter<T, f32>,
    toi:                            agl::Parameter<T, bool>,
    mEnableAutoAddWorld:            agl::Parameter<T, bool>,
    navmesh:                        agl::Parameter<T, sead::SafeString<T>>,
    navmesh_sub_material:           agl::Parameter<T, sead::SafeString<T>>,
    inertia:                        agl::Parameter<T, sead::Vector3f>,
    center_of_mass:                 agl::Parameter<T, sead::Vector3f>,
    bounding_center:                agl::Parameter<T, sead::Vector3f>,
    bounding_extents:               agl::Parameter<T, sead::Vector3f>,
    contact_point_info:             agl::Parameter<T, sead::SafeString<T>>,
    collision_info:                 agl::Parameter<T, sead::SafeString<T>>,
    max_linear_velocity:            agl::Parameter<T, f32>,
    linear_damping:                 agl::Parameter<T, f32>,
    max_angular_velocity_rad:       agl::Parameter<T, f32>,
    angular_damping:                agl::Parameter<T, f32>,
    max_impulse:                    agl::Parameter<T, f32>,
    col_impulse_scale:              agl::Parameter<T, f32>,
    ignore_normal_for_impulse:      agl::Parameter<T, bool>,
    always_character_mass_scaling:  agl::Parameter<T, bool>,
    friction_scale:                 agl::Parameter<T, f32>,
    restitution_scale:              agl::Parameter<T, f32>,
    water_buoyancy_scale:           agl::Parameter<T, f32>,
    water_flow_effective_rate:      agl::Parameter<T, f32>,
    magne_mass_scaling_factor:      agl::Parameter<T, f32>,
    motion_type:                    agl::Parameter<T, sead::SafeString<T>>,
    layer:                          agl::Parameter<T, sead::SafeString<T>>,
    groundhit:                      agl::Parameter<T, sead::SafeString<T>>,
    use_ground_hit_type_mask:       agl::Parameter<T, bool>,
    ground_hit_type_mask:           agl::Parameter<T, sead::SafeString<T>>,
    receiver_type:                  agl::Parameter<T, sead::SafeString<T>>,
    no_hit_ground:                  agl::Parameter<T, bool>,
    no_hit_water:                   agl::Parameter<T, bool>,
    no_char_standing_on:            agl::Parameter<T, bool>,
    contact_mask:                   agl::Parameter<T, u32>,
    link_matrix:                    agl::Parameter<T, sead::SafeString<T>>,
    link_entity_set:                agl::Parameter<T, sead::SafeString<T>>,
    link_entity_body:               agl::Parameter<T, sead::SafeString<T>>,
    use_entity_shape:               agl::Parameter<T, bool>,
    shape_num:                      agl::Parameter<T, i32>,
    navmesh_val:                    u32, // enum
    navmesh_sub_material_val:       u32, // enum
    ground_hit_mask:                u32,
}

#[repr(C)]
pub struct RigidBodyParam<T> {
    base:   agl::ParameterList<T>,
    info:   Info<T>,
    shapes: sead::Buffer<T>,
}

#[repr(C)]
pub struct ClothSubWindParam<T> {
    base:               agl::ParameterObj<T>,
    sub_wind_direction: agl::Parameter<T, sead::Vector3f>,
    sub_wind_frequency: agl::Parameter<T, f32>,
    sub_wind_speed:     agl::Parameter<T, f32>,
}

#[repr(C)]
pub struct ClothParam<T> {
    base:                   agl::ParameterObj<T>,
    wind_drag:              agl::Parameter<T, f32>,
    wind_frequency:         agl::Parameter<T, f32>,
    wind_min_speed:         agl::Parameter<T, f32>,
    wind_max_speed:         agl::Parameter<T, f32>,
    sub_wind_factor_main:   agl::Parameter<T, f32>,
    sub_wind_factor_add:    agl::Parameter<T, f32>,
    wind_enable:            agl::Parameter<T, bool>,
    writeback_to_local:     agl::Parameter<T, bool>,
    name:                   agl::Parameter<T, sead::SafeString<T>>,
    base_bone:              agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct ContactPointInfoParam<T> {
    base:           agl::ParameterObj<T>,
    name:           agl::Parameter<T, sead::FixedSafeString<T, 32>>,
    contact_type:   agl::Parameter<T, sead::FixedSafeString<T, 32>>,
    num:            agl::Parameter<T, i32>,
}

#[repr(C)]
pub struct CollisionInfoParam<T> {
    base:           agl::ParameterObj<T>,
    name:           agl::Parameter<T, sead::FixedSafeString<T, 32>>,
    collision_type: agl::Parameter<T, sead::FixedSafeString<T, 32>>,
}

#[repr(C)]
pub struct EdgeRigidBodyParam<T> {
    base:       agl::ParameterObj<T>,
    set_name:   agl::Parameter<T, sead::SafeString<T>>,
    body_name:  agl::Parameter<T, sead::SafeString<T>>,
    edge_type:  agl::Parameter<T, sead::SafeString<T>>,
}

#[repr(C)]
pub struct ShapeParamObj<T> {
    base:                       agl::ParameterObj<T>,
    shape_type:                 agl::Parameter<T, sead::FixedSafeString<T, 32>>,
    radius:                     agl::Parameter<T, f32>,
    convex_radius:              agl::Parameter<T, f32>,
    translate_0:                agl::Parameter<T, sead::Vector3f>,
    translate_1:                agl::Parameter<T, sead::Vector3f>,
    rotate:                     agl::Parameter<T, sead::Vector3f>,
    vertex_num:                 agl::Parameter<T, i32>,
    vertices:                   sead::Buffer<T>,
    material:                   agl::Parameter<T, sead::FixedSafeString<T, 32>>,
    sub_material:               agl::Parameter<T, sead::FixedSafeString<T, 32>>,
    wall_code:                  agl::Parameter<T, sead::FixedSafeString<T, 32>>,
    floor_code:                 agl::Parameter<T, sead::FixedSafeString<T, 32>>,
    item_code_disable_stick:    agl::Parameter<T, bool>,
}
