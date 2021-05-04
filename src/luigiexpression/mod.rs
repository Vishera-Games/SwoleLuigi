use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};
use crate::luigieffect::_HIT_COUNT;

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_13",
    animcmd = "expression_attack13")]
pub fn swole_luigi_expression_attack_13(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_dash",
    animcmd = "expression_attackdash")]
pub fn swole_luigi_expression_attack_dash(fighter: &mut L2CFighterCommon) {
    acmd!({
        
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_s3_hi",
    animcmd = "expression_attacks3hi")]
pub fn swole_luigi_expression_attack_s3_hi(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_s3_s",
    animcmd = "expression_attacks3")]
pub fn swole_luigi_expression_attack_s3(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_s3_lw",
    animcmd = "expression_attacks3lw")]
pub fn swole_luigi_expression_attack_s3_lw(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_hi3",
    animcmd = "expression_attackhi3")]
pub fn swole_luigi_expression_attack_hi3(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_lw3",
    animcmd = "expression_attacklw3")]
pub fn swole_luigi_expression_attack_lw3(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_s4_hi",
    animcmd = "expression_attacks4hi")]
pub fn swole_luigi_expression_attack_s4hi(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_s4_s",
    animcmd = "expression_attacks4")]
pub fn swole_luigi_expression_attack_s4(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_s4_lw",
    animcmd = "expression_attacks4lw")]
pub fn swole_luigi_expression_attack_s4lw(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_hi4",
    animcmd = "expression_attackhi4")]
pub fn swole_luigi_expression_attack_hi4(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_lw4",
    animcmd = "expression_attacklw4")]
pub fn swole_luigi_expression_attack_lw4(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_air_n",
    animcmd = "expression_attackairn")]
pub fn swole_luigi_expression_attack_air_n(fighter: &mut L2CFighterCommon) {
    acmd!({
        
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_air_f",
    animcmd = "expression_attackairf")]
pub fn swole_luigi_expression_attack_air_f(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_air_b",
    animcmd = "expression_attackairb")]
pub fn swole_luigi_expression_attack_air_b(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_air_hi",
    animcmd = "expression_attackairhi")]
pub fn swole_luigi_expression_attack_air_hi(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_air_lw",
    animcmd = "expression_attackairlw")]
pub fn swole_luigi_expression_attack_air_lw(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "final_start",
    animcmd = "expression_finalstart")]
pub fn swole_luigi_expression_final_start(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=11)
        for(11 Iterations){
            if(is_excute){
                QUAKE(CAMERA_QUAKE_KIND_S)
            }
            wait(Frames=5)
        }
        frame(Frame=72)
        for(8 Iterations){
            if(is_excute){
                QUAKE(CAMERA_QUAKE_KIND_XL)
            }
            wait(Frames=4)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "final_start_l",
    animcmd = "expression_finalstartl")]
pub fn swole_luigi_expression_final_startl(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=11)
        for(11 Iterations){
            if(is_excute){
                QUAKE(CAMERA_QUAKE_KIND_S)
            }
            wait(Frames=5)
        }
        frame(Frame=72)
        for(8 Iterations){
            if(is_excute){
                QUAKE(CAMERA_QUAKE_KIND_XL)
            }
            wait(Frames=4)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "final_air_start",
    animcmd = "expression_finalairstart")]
pub fn swole_luigi_expression_final_start_air(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=11)
        for(11 Iterations){
            if(is_excute){
                QUAKE(CAMERA_QUAKE_KIND_S)
            }
            wait(Frames=5)
        }
        frame(Frame=72)
        for(8 Iterations){
            if(is_excute){
                QUAKE(CAMERA_QUAKE_KIND_XL)
            }
            wait(Frames=4)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "final_air_start_l",
    animcmd = "expression_finalairstartl")]
pub fn swole_luigi_expression_final_start_airl(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=11)
        for(11 Iterations){
            if(is_excute){
                QUAKE(CAMERA_QUAKE_KIND_S)
            }
            wait(Frames=5)
        }
        frame(Frame=72)
        for(8 Iterations){
            if(is_excute){
                QUAKE(CAMERA_QUAKE_KIND_XL)
            }
            wait(Frames=4)
        }
    });
}

pub fn install() {
    acmd::add_hooks!(
        //swole_luigi_expression_attack_11,
        //swole_luigi_expression_attack_12,
        //swole_luigi_expression_attack_13,
        //swole_luigi_expression_attack_dash,
        swole_luigi_expression_attack_s3,
        swole_luigi_expression_attack_s3_hi,
        swole_luigi_expression_attack_s3_lw,
        //swole_luigi_expression_attack_hi3,
        //swole_luigi_expression_attack_lw3,
        swole_luigi_expression_attack_s4hi,
        swole_luigi_expression_attack_s4,
        swole_luigi_expression_attack_s4lw,
        //swole_luigi_expression_attack_hi4,
        swole_luigi_expression_attack_lw4,
        //swole_luigi_expression_attack_air_n,
        //swole_luigi_expression_attack_air_f,
        //swole_luigi_expression_attack_air_b,
        //swole_luigi_expression_attack_air_hi,
        swole_luigi_expression_attack_air_lw,
        swole_luigi_expression_final_start,
        swole_luigi_expression_final_startl,
        swole_luigi_expression_final_start_air,
        swole_luigi_expression_final_start_airl
    );
}
