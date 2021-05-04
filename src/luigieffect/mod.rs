use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::sv_animcmd::*;
use acmd::{acmd, acmd_func};
use crate::luigi::{_IS_RAGE, _GFXNAME, _IS_BURST}; //I needed some of the variables I created in /src/luigi/mod.src

static mut _GFXSIZE: [f32; 8] = [0.1; 8];
pub static mut _HIT_COUNT: [i32; 8] = [0; 8];
static mut _FCOUNTER: [i32; 8] = [0; 8];
pub static mut _COUNTING : [i32; 8] = [0; 8];
static mut _COUNTING2 : [i32; 8] = [0; 8];
static mut _CURRENTFRAME : [f32; 8] = [0.0; 8];

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_11",
    animcmd = "effect_attack11")]
pub fn swole_luigi_effect_attack_11(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        frame(Frame=1)
        if(is_excute){
            EFFECT_FLIP(hash40("sys_attack_line"), hash40("sys_attack_line"), hash40("top"), 0, 6.7, -3.8, 0, 0, 0, 0.95, 0, 1, 0, 0, 0, 0, true, EF_FLIP_YZ)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
            if(_IS_RAGE[entry_id]){
                EFFECT_FOLLOW(0x0edd56991a as u64, hash40("havel"), 0, 0, 0, -30, 0, 0, 1.0, true)
            }
        }
        frame(Frame=2)
        if(is_excute){
            EFFECT(hash40("sys_attack_impact"), hash40("top"), 0, 6.7, 12.3, 0, 0, 0, 1, 0, 1, 0, 0, 0, 360, true)
            LAST_EFFECT_SET_ALPHA(0.7)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_12",
    animcmd = "effect_attack12")]
pub fn swole_luigi_effect_attack_12(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        if(is_excute){
            if(_IS_RAGE[entry_id]){ //This is on every single punching, and most kicking attacks for when Luigi meets the requirements for rage mode
                EFFECT_OFF_KIND(0x0edd56991a as u64, false, false)
                EFFECT_FOLLOW(0x0edd56991a as u64, hash40("haver"), 0, 0, 0, -30, 0, 0, 1.0, true)
            }
        }
        frame(Frame=2)
        if(is_excute){
            EFFECT_FLIP(hash40("sys_attack_line"), hash40("sys_attack_line"), hash40("top"), 0, 6.5, 0.5, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, true, EF_FLIP_YZ)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
        }
        frame(Frame=4)
        if(is_excute){
            EFFECT(hash40("sys_attack_impact"), hash40("top"), 0, 6.5, 15.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, true)
            LAST_EFFECT_SET_ALPHA(0.7)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_13",
    animcmd = "effect_attack13")]
pub fn swole_luigi_effect_attack_13(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        if(is_excute){
            if(_IS_RAGE[entry_id]){
                EFFECT_OFF_KIND(0x0edd56991a as u64, false, false)
                EFFECT_FOLLOW(0x0edd56991a as u64, hash40("havel"), 0, 0, 0, -30, 0, 0, 1.0, true)
            }
        }
        frame(Frame=6)
        if(is_excute){
            EFFECT_FLIP(hash40("sys_attack_line"), hash40("sys_attack_line"), hash40("top"), 0, 7.0, -6.5, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true, EF_FLIP_YZ)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
        }
        frame(Frame=7)
        if(is_excute){
            EFFECT(hash40("sys_attack_impact"), hash40("top"), 0, 7.0, 11.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, true)
            LAST_EFFECT_SET_ALPHA(0.7)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
        }
        frame(Frame=20)
        if(is_excute){
            if(_IS_RAGE[entry_id]){
                EFFECT_OFF_KIND(0x0edd56991a as u64, false, false)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_dash",
    animcmd = "effect_attackdash")]
pub fn swole_luigi_effect_attack_dash(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=5)
        if(is_excute){
            EFFECT_FOLLOW(0x11f62f23d0 as u64, hash40("top"), 0, 4, 16, 0, 0, 0, 1, true)
            LANDING_EFFECT(hash40("sys_atk_smoke"), hash40("top"), 9, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false)
            LAST_EFFECT_SET_RATE(0.6)
        }
        frame(Frame=6)
        for(3 Iterations){
            if(is_excute){
                EFFECT_FOLLOW(hash40("sys_damage_fire"), hash40("arml"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true)
                LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 1.0, /*B*/ 0.1)
                LAST_EFFECT_SET_ALPHA(0.7)
                EFFECT_FOLLOW(hash40("sys_damage_fire"), hash40("shoulderl"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true)
                LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 1.0, /*B*/ 0.1)
                LAST_EFFECT_SET_ALPHA(0.7)
                EFFECT_FOLLOW(hash40("sys_damage_fire"), hash40("shoulderr"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true)
                LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 1.0, /*B*/ 0.1)
                LAST_EFFECT_SET_ALPHA(0.7)
            }
            wait(5)
        }
        frame(Frame=9)
        if(is_excute){
            FOOT_EFFECT(hash40("sys_turn_smoke"), hash40("top"), 9, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false)
        }
        frame(Frame=14)
        if(is_excute){
            FOOT_EFFECT(hash40("sys_turn_smoke"), hash40("top"), 9, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_s3_hi",
    animcmd = "effect_attacks3hi")]
pub fn swole_luigi_effect_attack_s3_hi(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        frame(Frame=3)
        if(is_excute){
            EFFECT_FLIP(hash40("sys_attack_line"), hash40("sys_attack_line"), hash40("top"), 0, 6.5, -4.5, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true, EF_FLIP_YZ)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
        }
        frame(Frame=4)
        if(is_excute){
            EFFECT(hash40("sys_attack_impact"), hash40("top"), 0, 5.5, 13.2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, true)
            LAST_EFFECT_SET_ALPHA(0.7)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.2, /*B*/ 0.1)
        }
        frame(Frame=5)
        if(is_excute){
            if(_IS_RAGE[entry_id]){
                EFFECT_FOLLOW(0x0edd56991a as u64, hash40("haver"), 0, 0, 0, -30, 0, 0, 1.0, true)
            }
        }
        frame(Frame=11)
        if(is_excute){
            EFFECT_FLIP(hash40("sys_attack_line"), hash40("sys_attack_line"), hash40("top"), 0, 6.5, -7.5, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true, EF_FLIP_YZ)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
        }
        frame(Frame=12)
        if(is_excute){
            EFFECT(hash40("sys_attack_impact"), hash40("top"), 0, 5.5, 10.2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, true)
            LAST_EFFECT_SET_ALPHA(0.7)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.2, /*B*/ 0.1)
            if(_IS_BURST[entry_id]){ //This GFX will display if the player is using their burst in side tilt, but also only if the character connects the first part of the move which is handled in the game section
                EFFECT(hash40("sys_hit_elec"), hash40("haver"), 0, 0, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, true)
                LAST_EFFECT_SET_COLOR(/*R*/ 1.0, /*G*/ 0.5, /*B*/ 0.0)
                EffectModule::req_screen(smash::phx::Hash40::new("bg_mewtwo_final"), false, false, false) //How to use a final smash screen
            }
        }
        wait(Frames=3)
        if(is_excute){
            if(_IS_BURST[entry_id]){
                EFFECT(hash40("sys_hit_aura"), hash40("haver"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true)
                LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
                EFFECT(hash40("sys_hit_elec"), hash40("haver"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true)
                LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
            }
        }
        wait(Frames=1)
        if(is_excute){
            if(_IS_BURST[entry_id]){
                EffectModule::remove_screen(smash::phx::Hash40::new("bg_mewtwo_final"), 0) // How to remove a final smash screen
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_s3_s",
    animcmd = "effect_attacks3")]
pub fn swole_luigi_effect_attack_s3(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        frame(Frame=3)
        if(is_excute){
            EFFECT_FLIP(hash40("sys_attack_line"), hash40("sys_attack_line"), hash40("top"), 0, 6.5, -4.5, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true, EF_FLIP_YZ)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
        }
        frame(Frame=4)
        if(is_excute){
            EFFECT(hash40("sys_attack_impact"), hash40("top"), 0, 5.5, 13.2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, true)
            LAST_EFFECT_SET_ALPHA(0.7)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.2, /*B*/ 0.1)
        }
        frame(Frame=5)
        if(is_excute){
            if(_IS_RAGE[entry_id]){
                EFFECT_FOLLOW(0x0edd56991a as u64, hash40("haver"), 0, 0, 0, -30, 0, 0, 1.0, true)
            }
        }
        frame(Frame=11)
        if(is_excute){
            EFFECT_FLIP(hash40("sys_attack_line"), hash40("sys_attack_line"), hash40("top"), 0, 6.5, -7.5, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true, EF_FLIP_YZ)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
        }
        frame(Frame=12)
        if(is_excute){
            EFFECT(hash40("sys_attack_impact"), hash40("top"), 0, 5.5, 10.2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, true)
            LAST_EFFECT_SET_ALPHA(0.7)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.2, /*B*/ 0.1)
            if(_IS_BURST[entry_id]){
                EFFECT(hash40("sys_hit_elec"), hash40("haver"), 0, 0, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, true)
                LAST_EFFECT_SET_COLOR(/*R*/ 1.0, /*G*/ 0.5, /*B*/ 0.0)
                EffectModule::req_screen(smash::phx::Hash40::new("bg_mewtwo_final"), false, false, false)
            }
        }
        wait(Frames=3)
        if(is_excute){
            if(_IS_BURST[entry_id]){
                EFFECT(hash40("sys_hit_aura"), hash40("haver"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true)
                LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
                EFFECT(hash40("sys_hit_elec"), hash40("haver"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true)
                LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
            }
        }
        wait(Frames=1)
        if(is_excute){
            if(_IS_BURST[entry_id]){
                EffectModule::remove_screen(smash::phx::Hash40::new("bg_mewtwo_final"), 0)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_s3_lw",
    animcmd = "effect_attacks3lw")]
pub fn swole_luigi_effect_attack_s3_lw(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        frame(Frame=3)
        if(is_excute){
            EFFECT_FLIP(hash40("sys_attack_line"), hash40("sys_attack_line"), hash40("top"), 0, 6.5, -4.5, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true, EF_FLIP_YZ)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
        }
        frame(Frame=4)
        if(is_excute){
            EFFECT(hash40("sys_attack_impact"), hash40("top"), 0, 5.5, 13.2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, true)
            LAST_EFFECT_SET_ALPHA(0.7)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.2, /*B*/ 0.1)
        }
        frame(Frame=5)
        if(is_excute){
            if(_IS_RAGE[entry_id]){
                EFFECT_FOLLOW(0x0edd56991a as u64, hash40("haver"), 0, 0, 0, -30, 0, 0, 1.0, true)
            }
        }
        frame(Frame=11)
        if(is_excute){
            EFFECT_FLIP(hash40("sys_attack_line"), hash40("sys_attack_line"), hash40("top"), 0, 6.5, -7.5, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true, EF_FLIP_YZ)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
        }
        frame(Frame=12)
        if(is_excute){
            EFFECT(hash40("sys_attack_impact"), hash40("top"), 0, 5.5, 10.2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, true)
            LAST_EFFECT_SET_ALPHA(0.7)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.2, /*B*/ 0.1)
            if(_IS_BURST[entry_id]){
                EFFECT(hash40("sys_hit_elec"), hash40("haver"), 0, 0, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, true)
                LAST_EFFECT_SET_COLOR(/*R*/ 1.0, /*G*/ 0.5, /*B*/ 0.0)
                EffectModule::req_screen(smash::phx::Hash40::new("bg_mewtwo_final"), false, false, false)
            }
        }
        wait(Frames=3)
        if(is_excute){
            if(_IS_BURST[entry_id]){
                EFFECT(hash40("sys_hit_aura"), hash40("haver"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true)
                LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
                EFFECT(hash40("sys_hit_elec"), hash40("haver"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true)
                LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
            }
        }
        wait(Frames=1)
        if(is_excute){
            if(_IS_BURST[entry_id]){
                EffectModule::remove_screen(smash::phx::Hash40::new("bg_mewtwo_final"), 0)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_lw3",
    animcmd = "effect_attacklw3")]
pub fn swole_luigi_effect_attack_lw3(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        if(is_excute){
            if(_IS_RAGE[entry_id]){
                EFFECT_FOLLOW(0x0edd56991a as u64, hash40("toel"), 0, 0, 0, -30, 0, 0, 1.0, true)
            }
        }
        frame(Frame=8)
        if(is_excute){
            EFFECT_FOLLOW_FLIP(hash40("sys_attack_arc_c"), hash40("sys_attack_arc_c"), hash40("top"), -4, 3, 0.5, -9, -36, 162, 1.0, true, EF_FLIP_YZ)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
        }
        frame(frame=35)
        if(is_excute){
            EFFECT_OFF_KIND(0x0edd56991a as u64, false, false)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_hi3",
    animcmd = "effect_attackhi3")]
pub fn swole_luigi_effect_attack_hi3(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        if(is_excute){
            if(_IS_RAGE[entry_id]){
                EFFECT_FOLLOW(0x0edd56991a as u64, hash40("havel"), 0, 0, 0, -30, 0, 0, 1.0, true)
            }
        }
        frame(Frame=5)
        if(is_excute){
            EFFECT_FOLLOW_FLIP(hash40("sys_attack_arc_c"), hash40("sys_attack_arc_c"), hash40("top"), 2, 11, 2, -12, 16, 95, 0.9, true, EF_FLIP_YZ)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1) //Literally only added this script to turn the default stuff green
        }
        frame(Frame=15)
        if(is_excute){
            LANDING_EFFECT(hash40("sys_landing_smoke_s"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_s4_hold",
    animcmd = "effect_attacks4charge")]
pub fn swole_luigi_effect_attack_s4hold(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        frame(Frame=1)
        for(60 Iterations){
            if(is_excute){
                if(_IS_RAGE[entry_id] == true){ //If in rage mode burst is automatic on forward smash, and is indicated by this green fire gfx which will grow in size the longer you charge the smash.
                    EFFECT_FOLLOW(0x0edd56991a as u64, hash40("haver"), 0, 0, 0, -30, 0, 0, _GFXSIZE[entry_id] / 1.3, true) //The gfx scaling is handled in the once_per_fighter_frame
                }
            }
            wait(1)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_s4_hi",
    animcmd = "effect_attacks4hi")]
pub fn swole_luigi_effect_attack_s4hi(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        if(is_excute){
            if(_IS_RAGE[entry_id] == true){
                EFFECT_FOLLOW(0x0edd56991a as u64, hash40("havel"), 0, 0, 0, -30, 0, 0, _GFXSIZE[entry_id] / 1.3, true)
            }
        }
        frame(Frame=3)
        if(is_excute){
            EFFECT(hash40("sys_smash_flash"), hash40("top"), 0, 7, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
        }
        frame(Frame=14)
        if(is_excute){
            LANDING_EFFECT(hash40("sys_atk_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
            EFFECT_FLIP(hash40("sys_attack_line"), hash40("sys_attack_line"), hash40("top"), 0.5, 5.5, 0, -25, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true, EF_FLIP_NONE)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
        }
        frame(Frame=16)
        if(is_excute){
            EFFECT(hash40("sys_hit_aura"), hash40("havel"), 0, 0, 0, 0, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, true)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.6, /*B*/ 0.12)
            EFFECT(hash40("sys_hit_elec"), hash40("havel"), 0, 0, 0, 0, 0, 0, _GFXSIZE[entry_id] / 4.0, 0, 0, 0, 0, 0, 0, true)
            if(_IS_BURST[entry_id] && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT)){
                EffectModule::req_screen(smash::phx::Hash40::new("bg_mewtwo_final"), false, false, false)
            }
        }
        frame(Frame=22)
        if(is_excute){
            if(AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) && _IS_RAGE[entry_id]){ //If the move connects and Luigi was in rage mode
                EFFECT(hash40("sys_hit_aura"), hash40("top"), 0, 12, 11, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true)
                LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.4, /*B*/ 0.08)
                EFFECT(hash40("sys_bomb_b"), hash40("top"), 0, 12, 11, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true)
                LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.4, /*B*/ 0.08)
                EFFECT_OFF_KIND(0x0edd56991a, false, false)
                EFFECT_FOLLOW(0x15880548be, hash40("top"), 0, 12, 11, 0, 0, 0, 1, true)
            }
        }
        frame(Frame=24)
        if(is_excute){
            if(_IS_RAGE[entry_id] && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT)){
                EffectModule::remove_screen(smash::phx::Hash40::new("bg_mewtwo_final"), 0)
            }
        }
        frame(Frame=25)
        if(is_excute){
            rust{
                _GFXSIZE[entry_id] = 0.1; //Rest the gfx size 
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_s4_s",
    animcmd = "effect_attacks4")]
pub fn swole_luigi_effect_attack_s4(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        if(is_excute){
            if(_IS_RAGE[entry_id] == true){
                EFFECT_FOLLOW(0x0edd56991a as u64, hash40("havel"), 0, 0, 0, -30, 0, 0, _GFXSIZE[entry_id] / 1.3, true)
            }
        }
        frame(Frame=3)
        if(is_excute){
            EFFECT(hash40("sys_smash_flash"), hash40("top"), 0, 7, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
        }
        frame(Frame=14)
        if(is_excute){
            LANDING_EFFECT(hash40("sys_atk_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
            EFFECT_FLIP(hash40("sys_attack_line"), hash40("sys_attack_line"), hash40("top"), 0.5, 8, 0, 0, -5, 0, 1.0, 0, 0, 0, 0, 0, 0, true, EF_FLIP_NONE)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
        }
        frame(Frame=16)
        if(is_excute){
            EFFECT(hash40("sys_hit_aura"), hash40("havel"), 0, 0, 0, 0, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, true)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.6, /*B*/ 0.12)
            EFFECT(hash40("sys_hit_elec"), hash40("havel"), 0, 0, 0, 0, 0, 0, _GFXSIZE[entry_id] / 4.0, 0, 0, 0, 0, 0, 0, true)
            if(_IS_BURST[entry_id] && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT)){
                EffectModule::req_screen(smash::phx::Hash40::new("bg_mewtwo_final"), false, false, false)
            }
        }
        frame(Frame=22)
        if(is_excute){
            if(AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) && _IS_RAGE[entry_id]){
                EFFECT(hash40("sys_hit_aura"), hash40("top"), 0, 8.9, 14, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true)
                LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.4, /*B*/ 0.08)
                EFFECT(hash40("sys_bomb_b"), hash40("top"), 0, 8.9, 14, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true)
                LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.4, /*B*/ 0.08)
                EFFECT_OFF_KIND(0x0edd56991a, false, false)
                EFFECT_FOLLOW(0x15880548be, hash40("top"), 0, 8.9, 14, 0, 0, 0, 1, true)
            }
        }
        frame(Frame=24)
        if(is_excute){
            if(_IS_RAGE[entry_id] && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT)){
                EffectModule::remove_screen(smash::phx::Hash40::new("bg_mewtwo_final"), 0)
            }
        }
        frame(Frame=25)
        if(is_excute){
            rust{
                _GFXSIZE[entry_id] = 0.1;
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_s4_lw",
    animcmd = "effect_attacks4lw")]
pub fn swole_luigi_effect_attack_s4lw(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        if(is_excute){
            if(_IS_RAGE[entry_id] == true){
                EFFECT_FOLLOW(0x0edd56991a as u64, hash40("havel"), 0, 0, 0, -30, 0, 0, _GFXSIZE[entry_id] / 1.3, true)
            }
        }
        frame(Frame=3)
        if(is_excute){
            EFFECT(hash40("sys_smash_flash"), hash40("top"), 0, 7, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
        }
        frame(Frame=14)
        if(is_excute){
            LANDING_EFFECT(hash40("sys_atk_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
            EFFECT_FLIP(hash40("sys_attack_line"), hash40("sys_attack_line"), hash40("top"), 0, 9.7, 0, 24, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true, EF_FLIP_NONE)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
        }
        frame(Frame=16)
        if(is_excute){
            EFFECT(hash40("sys_hit_aura"), hash40("havel"), 0, 0, 0, 0, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, true)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.6, /*B*/ 0.12)
            EFFECT(hash40("sys_hit_elec"), hash40("havel"), 0, 0, 0, 0, 0, 0, _GFXSIZE[entry_id] / 4.0, 0, 0, 0, 0, 0, 0, true)
            if(_IS_BURST[entry_id] && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT)){
                EffectModule::req_screen(smash::phx::Hash40::new("bg_mewtwo_final"), false, false, false)
            }
        }
        frame(Frame=22)
        if(is_excute){
            if(AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) && _IS_RAGE[entry_id]){
                EFFECT(hash40("sys_hit_aura"), hash40("top"), 0, 4, 11, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true)
                LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.4, /*B*/ 0.08)
                EFFECT(hash40("sys_bomb_b"), hash40("top"), 0, 4, 11, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true)
                LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.4, /*B*/ 0.08)
                EFFECT_OFF_KIND(0x0edd56991a, false, false)
                EFFECT_FOLLOW(0x15880548be, hash40("top"), 0, 4, 11, 0, 0, 0, 1, true)
            }
        }
        frame(Frame=24)
        if(is_excute){
            if(_IS_RAGE[entry_id] && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT)){
                EffectModule::remove_screen(smash::phx::Hash40::new("bg_mewtwo_final"), 0)
            }
        }
        frame(Frame=25)
        if(is_excute){
            rust{
                _GFXSIZE[entry_id] = 0.1;
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_hi4",
    animcmd = "effect_attackhi4")]
pub fn swole_luigi_effect_attack_hi4(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=4)
        if(is_excute){
            EFFECT(hash40("sys_smash_flash"), hash40("head"), 4.0, 0, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
        }
        frame(Frame=10)
        if(is_excute){
            EFFECT_FOLLOW_FLIP(0x0f90c2c93c as u64, 0x0f90c2c93c as u64, hash40("throw"), 0, 8, 2, -15, -90, -90, 0.95, true, EF_FLIP_YZ)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1) //Only added this to turn stuff green
        }
        frame(Frame=13)
        if(is_excute){
            LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_lw4_hold",
    animcmd = "effect_attacklw4charge")]
pub fn swole_luigi_effect_attack_lw4hold(fighter: &mut L2CFighterCommon) {
    acmd!({
        //not used
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_lw4",
    animcmd = "effect_attacklw4")]
pub fn swole_luigi_effect_attack_lw4(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        if(is_excute){ //Nothing really special here just custom script with standard gfx
            if(_IS_RAGE[entry_id]){
                EFFECT_FOLLOW(0x0edd56991a as u64, hash40("havel"), 0, 0, 0, -30, 0, 0, 1.0, true)
                EFFECT_FOLLOW(0x0edd56991a as u64, hash40("toer"), 0, 0, 0, -30, 0, 0, 1.0, true)
            }
        }
        frame(Frame=2)
        if(is_excute){
            EFFECT(hash40("sys_smash_flash"), hash40("top"), 0, 8, 7.5, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, true)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
        }
        frame(Frame=8)
        if(is_excute){
            //EFFECT_FLIP(hash40("sys_attack_line"), hash40("sys_attack_line"), hash40("top"), 0.5, 5.5, 0, -25, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true, EF_FLIP_NONE)
            EFFECT_FOLLOW_FLIP(hash40("sys_attack_line"), hash40("sys_attack_line"), hash40("top"), 0, 6, -3, 0, 0, 0, 1.2, true, EF_FLIP_YZ)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
            EFFECT_FOLLOW_FLIP(hash40("sys_attack_line"), hash40("sys_attack_line"), hash40("top"), 0, 7, 5.5, 180, -10, 0, 1.2, true, EF_FLIP_YZ)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
            LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false)
        }
        frame(Frame=10)
        if(is_excute){
            EFFECT_ALPHA(hash40("sys_attack_impact"), hash40("top"), 0, 6.5, 9, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 360, true, 1)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
            EFFECT_ALPHA(hash40("sys_attack_impact"), hash40("top"), 0, 7.5, -10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, true, 1)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_air_f",
    animcmd = "effect_attackairf")]
pub fn swole_luigi_effect_attack_air_f(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        if(is_excute){
            if(_IS_RAGE[entry_id] == true){
                EFFECT_FOLLOW(0x0edd56991a as u64, hash40("havel"), 0, 0, 0, -30, 0, 0, 1.0, true)
            }
        }
        frame(Frame=16)
        if(is_excute){
            EFFECT_FOLLOW(hash40("sys_attack_arc_d"), hash40("top"), -1, 8, 0.8, 180, -200, 90, 0.9, false) //These things are kinda really awful to line up from scratch 
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
            LAST_EFFECT_SET_RATE(1.2)
        }  
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_air_b",
    animcmd = "effect_attackairb")]
pub fn swole_luigi_effect_attack_air_b(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        if(is_excute){
            if(_IS_RAGE[entry_id] == true){
                EFFECT_FOLLOW(0x0edd56991a as u64, hash40("haver"), 0, 0, 0, -30, 0, 0, 1.0, true)
            }
        }
        frame(Frame=17)
        if(is_excute){
            EFFECT_FOLLOW(hash40("sys_attack_arc_d"), hash40("top"), -1, 7, -1.5, -180, -30, -90, 0.9, false) //Nothing special standard custom gfx
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
            LAST_EFFECT_SET_RATE(1.27)
        } 
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_air_hi",
    animcmd = "effect_attackairhi")]
pub fn swole_luigi_effect_attack_air_hi(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=5)
        if(is_excute){
            EFFECT_FOLLOW_FLIP(hash40("sys_attack_arc"), hash40("sys_attack_arc"), hash40("top"), -1, 11, 1, 0, 70, 90, 0.9, true, EF_FLIP_YZ)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1) //Literally just turned the arc green
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_air_lw",
    animcmd = "effect_attackairlw")]
pub fn swole_luigi_effect_attack_air_lw(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        if(is_excute){
            if(_IS_RAGE[entry_id] == true){
                EFFECT_FOLLOW(0x0edd56991a as u64, hash40("toel"), 0, 0, 0, -30, 0, 0, 1.0, true)
                EFFECT_FOLLOW(0x0edd56991a as u64, hash40("toer"), 0, 0, 0, -30, 0, 0, 1.0, true)
            }
        }
        frame(Frame=15)
        if(is_excute){
            EFFECT_FOLLOW_NO_STOP(hash40("sys_attack_speedline"), hash40("top"), 0, -1.0, 0, -90, 0, 0, 1.3, true) //Standard gfx using adjusted coordinates and colors from Ganon's script
            LAST_PARTICLE_SET_COLOR(0.0, 0.8, 0.1)
        }
        frame(Frame=16)
        if(is_excute){
            EFFECT_FOLLOW_ALPHA(hash40("sys_attack_impact"), hash40("top"), 0, -3, 0, 0, 0, 0, 1.5, true, 2)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "final_start",
    animcmd = "effect_finalstart")]
pub fn swole_luigi_effect_final_start(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        frame(Frame=1)
        rust{
            _COUNTING[entry_id] = 0;
            _GFXSIZE[entry_id] = 0.0001;
        }
        frame(Frame=2)
        if(is_excute){
            EffectModule::req_screen(smash::phx::Hash40::new_raw(0x1210319015), false, false, false) //I forget which final screen this is
        }
        frame(Frame=11)
        for(55 Iterations){
            if(is_excute){
                if(_COUNTING[entry_id] == 5){ //If the frame counting variable reaches 5
                    EFFECT_FOLLOW(hash40("sys_landing_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true) //Display the smoke shockwave thing. If I did not make a counter it would spawn so much the switch will lag
                }
                    EFFECT_OFF_KIND(hash40("luigi_final_muzzle"), false, false) //Kill the previous gfx for the green shock looking ball
                    EFFECT_FOLLOW(hash40("luigi_final_muzzle"), hash40("haver"), 3, 0, 0, 0, 0, 0, _GFXSIZE[entry_id], 0, 0, 0, 0, 0, 0, true) //Display a gfx I found from Luigi's final smash that looked really poggers
                    LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.0) //Make it green
            }
            wait(1)
            rust{
                _COUNTING[entry_id] += 1; //Frame counter adds 1
                _GFXSIZE[entry_id] += 0.02; //Graphical effect size for the shock ball increases
                if _COUNTING[entry_id] > 5 {
                    _COUNTING[entry_id] = 0; //If the counter reaches greater than 5, reset it to 0 so that the smoke effect can display again
                }
            }
        }
        frame(Frame=67)
        if(is_excute){
            EffectModule::remove_screen(smash::phx::Hash40::new_raw(0x1210319015), 0) //Remove previous final screen
            EFFECT(hash40("sys_hit_elec"), hash40("haver"), 3, 0, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, true) //Kinda used to make a shockwave effect before the beam spawns
            EFFECT_FOLLOW(0x0edd56991a as u64, hash40("rot"), 0, -3, 0, -30, 0, 0, 4.5, true) //Spawns a large green fire around Luigi like an aura
            EFFECT_OFF_KIND(86106501629 as u64, false, false)
            EFFECT_OFF_KIND(hash40("luigi_final_muzzle"), false, false)
            EffectModule::req_screen(smash::phx::Hash40::new("bg_mewtwo_final"), false, false, false) //Mewtwo's final screen us mostly just plain black so it's useful for scenes
        }
        frame(Frame=72)
        if(is_excute){
            EFFECT_FOLLOW(hash40("luigi_final_muzzle"), hash40("haver"), 3, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, true)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
            EFFECT(hash40("sys_genesis_beam"), hash40("top"), 0, 7, 20, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false) //The daybreak cannon graphical effect
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.7, /*B*/ 0.0)
            LAST_EFFECT_SET_RATE(0.9)
            EFFECT(72705012482 as u64, hash40("top"), 0, 7, 20, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false) //The shockwave gfx at the base of daybreak
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.7, /*B*/ 0.0) //Turn it all green tbh
            LAST_EFFECT_SET_RATE(0.9)
        }
        frame(Frame=95)
        if(is_excute){
            EFFECT_DETACH_KIND(hash40("sys_genesis_beam"), 18446744073709551615 as u64) //Idk what this does but it's used in the daybreak script
        }
        frame(Frame=105)
        if(is_excute){
            EFFECT_OFF_KIND(72705012482 as u64, false, false) //Trying to manually end daybreak didn't seem to work, neither did adjusting its rate so it seems to be a set length
            EFFECT_OFF_KIND(hash40("luigi_final_muzzle"), false, false)
            EFFECT_OFF_KIND(0x0edd56991a as u64, false, false)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "final_start_l",
    animcmd = "effect_finalstartl")]
pub fn swole_luigi_effect_final_startl(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        frame(Frame=1)
        rust{
            _COUNTING[entry_id] = 0;
            _GFXSIZE[entry_id] = 0.0001;
        }
        frame(Frame=2)
        if(is_excute){
            EffectModule::req_screen(smash::phx::Hash40::new_raw(0x1210319015), false, false, false)
        }
        frame(Frame=11)
        for(55 Iterations){
            if(is_excute){
                if(_COUNTING[entry_id] == 5){
                    EFFECT_FOLLOW(hash40("sys_landing_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true)
                }
                    //EFFECT_OFF_KIND(86106501629 as u64, false, false)
                    EFFECT_OFF_KIND(hash40("luigi_final_muzzle"), false, false)
                    //EFFECT_FOLLOW(86106501629 as u64, hash40("haver"), 3, 0, 0, 0, 0, 0, _GFXSIZE[entry_id] / 2.0, 0, 0, 0, 0, 0, 0, true)
                    //LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
                    EFFECT_FOLLOW(hash40("luigi_final_muzzle"), hash40("haver"), 3, 0, 0, 0, 0, 0, _GFXSIZE[entry_id], 0, 0, 0, 0, 0, 0, true)
                    LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.0)
            }
            wait(1)
            rust{
                _COUNTING[entry_id] += 1;
                _GFXSIZE[entry_id] += 0.02;
                if _COUNTING[entry_id] > 3 {
                    _COUNTING[entry_id] = 0;
                }
            }
        }
        frame(Frame=67)
        if(is_excute){
            EffectModule::remove_screen(smash::phx::Hash40::new_raw(0x1210319015), 0)
            EFFECT(hash40("sys_hit_elec"), hash40("haver"), 3, 0, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, true)
            EFFECT_FOLLOW(0x0edd56991a as u64, hash40("rot"), 0, -3, 0, -30, 0, 0, 4.5, true)
            EFFECT_OFF_KIND(86106501629 as u64, false, false)
            EFFECT_OFF_KIND(hash40("luigi_final_muzzle"), false, false)
            EffectModule::req_screen(smash::phx::Hash40::new("bg_mewtwo_final"), false, false, false)
        }
        frame(Frame=72)
        if(is_excute){
            EFFECT_FOLLOW(hash40("luigi_final_muzzle"), hash40("haver"), 3, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, true)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
            EFFECT(hash40("sys_genesis_beam"), hash40("top"), 0, 7, 20, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.7, /*B*/ 0.0)
            LAST_EFFECT_SET_RATE(0.9)
            EFFECT(72705012482 as u64, hash40("top"), 0, 7, 20, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.7, /*B*/ 0.0)
            LAST_EFFECT_SET_RATE(0.9)
        }
        frame(Frame=95)
        if(is_excute){
            EFFECT_DETACH_KIND(hash40("sys_genesis_beam"), 18446744073709551615 as u64)
        }
        frame(Frame=105)
        if(is_excute){
            EFFECT_OFF_KIND(72705012482 as u64, false, false)
            EFFECT_OFF_KIND(hash40("luigi_final_muzzle"), false, false)
            EFFECT_OFF_KIND(0x0edd56991a as u64, false, false)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "final_air_start",
    animcmd = "effect_finalairstart")]
pub fn swole_luigi_effect_final_start_air(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        frame(Frame=1)
        rust{
            _GFXSIZE[entry_id] = 0.0001;
        }
        frame(Frame=2)
        if(is_excute){
            EffectModule::req_screen(smash::phx::Hash40::new_raw(0x1210319015), false, false, false)
        }
        frame(Frame=11)
        for(55 Iterations){
            if(is_excute){
                    //EFFECT_OFF_KIND(86106501629 as u64, false, false)
                    EFFECT_OFF_KIND(hash40("luigi_final_muzzle"), false, false)
                    //EFFECT_FOLLOW(86106501629 as u64, hash40("haver"), 3, 0, 0, 0, 0, 0, _GFXSIZE[entry_id] / 2.0, 0, 0, 0, 0, 0, 0, true)
                    //LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
                    EFFECT_FOLLOW(hash40("luigi_final_muzzle"), hash40("haver"), 3, 0, 0, 0, 0, 0, _GFXSIZE[entry_id], 0, 0, 0, 0, 0, 0, true)
                    LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.0)
            }
            wait(1)
            rust{
                _GFXSIZE[entry_id] += 0.02;
            }
        }
        frame(Frame=67)
        if(is_excute){
            EffectModule::remove_screen(smash::phx::Hash40::new_raw(0x1210319015), 0)
            EFFECT(hash40("sys_hit_elec"), hash40("haver"), 3, 0, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, true)
            EFFECT_FOLLOW(0x0edd56991a as u64, hash40("rot"), 0, -3, 0, -30, 0, 0, 4.5, true)
            EFFECT_OFF_KIND(86106501629 as u64, false, false)
            EFFECT_OFF_KIND(hash40("luigi_final_muzzle"), false, false)
            EffectModule::req_screen(smash::phx::Hash40::new("bg_mewtwo_final"), false, false, false)
        }
        frame(Frame=72)
        if(is_excute){
            EFFECT_FOLLOW(hash40("luigi_final_muzzle"), hash40("haver"), 3, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, true)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
            EFFECT(hash40("sys_genesis_beam"), hash40("top"), 0, 7, 20, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.7, /*B*/ 0.0)
            LAST_EFFECT_SET_RATE(0.9)
            EFFECT(72705012482 as u64, hash40("top"), 0, 7, 20, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.7, /*B*/ 0.0)
            LAST_EFFECT_SET_RATE(0.9)
        }
        frame(Frame=95)
        if(is_excute){
            EFFECT_DETACH_KIND(hash40("sys_genesis_beam"), 18446744073709551615 as u64)
        }
        frame(Frame=105)
        if(is_excute){
            EFFECT_OFF_KIND(72705012482 as u64, false, false)
            EFFECT_OFF_KIND(hash40("luigi_final_muzzle"), false, false)
            EFFECT_OFF_KIND(0x0edd56991a as u64, false, false)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "final_air_start_l",
    animcmd = "effect_finalairstartl")]
pub fn swole_luigi_effect_final_start_airl(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        frame(Frame=1)
        rust{
            _GFXSIZE[entry_id] = 0.0001;
        }
        frame(Frame=2)
        if(is_excute){
            EffectModule::req_screen(smash::phx::Hash40::new_raw(0x1210319015), false, false, false)
        }
        frame(Frame=11)
        for(55 Iterations){
            if(is_excute){
                    //EFFECT_OFF_KIND(86106501629 as u64, false, false)
                    EFFECT_OFF_KIND(hash40("luigi_final_muzzle"), false, false)
                    //EFFECT_FOLLOW(86106501629 as u64, hash40("haver"), 3, 0, 0, 0, 0, 0, _GFXSIZE[entry_id] / 2.0, 0, 0, 0, 0, 0, 0, true)
                    //LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
                    EFFECT_FOLLOW(hash40("luigi_final_muzzle"), hash40("haver"), 3, 0, 0, 0, 0, 0, _GFXSIZE[entry_id], 0, 0, 0, 0, 0, 0, true)
                    LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.0)
            }
            wait(1)
            rust{
                _GFXSIZE[entry_id] += 0.02;
            }
        }
        frame(Frame=67)
        if(is_excute){
            EffectModule::remove_screen(smash::phx::Hash40::new_raw(0x1210319015), 0)
            EFFECT(hash40("sys_hit_elec"), hash40("haver"), 3, 0, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, true)
            EFFECT_FOLLOW(0x0edd56991a as u64, hash40("rot"), 0, -3, 0, -30, 0, 0, 4.5, true)
            EFFECT_OFF_KIND(86106501629 as u64, false, false)
            EFFECT_OFF_KIND(hash40("luigi_final_muzzle"), false, false)
            EffectModule::req_screen(smash::phx::Hash40::new("bg_mewtwo_final"), false, false, false)
        }
        frame(Frame=72)
        if(is_excute){
            EFFECT_FOLLOW(hash40("luigi_final_muzzle"), hash40("haver"), 3, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, true)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.8, /*B*/ 0.1)
            EFFECT(hash40("sys_genesis_beam"), hash40("top"), 0, 7, 20, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.7, /*B*/ 0.0)
            LAST_EFFECT_SET_RATE(0.9)
            EFFECT(72705012482 as u64, hash40("top"), 0, 7, 20, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false)
            LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.7, /*B*/ 0.0)
            LAST_EFFECT_SET_RATE(0.9)
        }
        frame(Frame=95)
        if(is_excute){
            EFFECT_DETACH_KIND(hash40("sys_genesis_beam"), 18446744073709551615 as u64)
        }
        frame(Frame=105)
        if(is_excute){
            EFFECT_OFF_KIND(72705012482 as u64, false, false)
            EFFECT_OFF_KIND(hash40("luigi_final_muzzle"), false, false)
            EFFECT_OFF_KIND(0x0edd56991a as u64, false, false)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "special_hi",
    animcmd = "effect_specialhi")]
pub fn swole_luigi_effect_special_hi(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        if(is_excute){
            LANDING_EFFECT(hash40("sys_action_smoke_h"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
        }
        frame(Frame=2)
        if(is_excute){
            EFFECT_COLOR(hash40("sys_smash_flash"), hash40("top"), 0, 4, 8, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true, 2.008, 0.698, 0.698)
        }
        frame(Frame=5)
        if(is_excute){
            EFFECT_FOLLOW(0x15880548be, hash40("handl"), 1, 0, 0, 0, 0, 0, 1, true)
        }
        frame(Frame=7)
        if(is_excute){
            if(_IS_RAGE[entry_id]){
                EFFECT_FOLLOW(0x0edd56991a as u64, hash40("havel"), 0, 0, 0, -30, 0, 0, 3.0, true)
            }
        }
        frame(Frame=8)
        if(is_excute){
            EFFECT_FOLLOW(0x181d41ff6f, hash40("handl"), 1, 0, 0, 0, 0, 0, 1, true)
        }
        frame(Frame=23)
        if(is_excute){
            EFFECT_OFF_KIND(0x0edd56991a as u64, false, false)
            if(_IS_BURST[entry_id] && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT)){
                EFFECT(hash40("sys_hit_aura"), hash40("havel"), 0, 0.0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true)
                LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.4, /*B*/ 0.08)
                EFFECT(hash40("sys_bomb_b"), hash40("havel"), 0, 0.0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true)
                LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.4, /*B*/ 0.08)
                EFFECT_OFF_KIND(0x0edd56991a, false, false)
                EFFECT_FOLLOW(0x15880548be, hash40("havel"), 0, 0.0, 0, 0, 0, 0, 1, true)
                PLAY_SE(0x13c2ff2d51 as u64)
                PLAY_SE(hash40("se_item_magicball_warpin"))
                PLAY_SE(hash40("se_item_killsword_flash"))
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "special_air_hi",
    animcmd = "effect_specialairhi")]
pub fn swole_luigi_effect_special_air_hi(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        frame(Frame=2)
        if(is_excute){
            EFFECT_COLOR(hash40("sys_smash_flash"), hash40("top"), 0, 4, 8, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true, 2.008, 0.698, 0.698)
        }
        frame(Frame=5)
        if(is_excute){
            EFFECT_FOLLOW(0x15880548be, hash40("handl"), 1, 0, 0, 0, 0, 0, 1, true)
        }
        frame(Frame=7)
        if(is_excute){
            if(_IS_RAGE[entry_id]){
                EFFECT_FOLLOW(0x0edd56991a as u64, hash40("havel"), 0, 0, 0, -30, 0, 0, 3.0, true)
            }
        }
        frame(Frame=8)
        if(is_excute){
            EFFECT_FOLLOW(0x181d41ff6f, hash40("handl"), 1, 0, 0, 0, 0, 0, 1, true)
        }
        frame(Frame=23)
        if(is_excute){
            EFFECT_OFF_KIND(0x0edd56991a as u64, false, false)
            if(_IS_BURST[entry_id] && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT)){
                EFFECT(hash40("sys_hit_aura"), hash40("havel"), 0, 0.0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true)
                LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.4, /*B*/ 0.08)
                EFFECT(hash40("sys_bomb_b"), hash40("havel"), 0, 0.0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true)
                LAST_EFFECT_SET_COLOR(/*R*/ 0.0, /*G*/ 0.4, /*B*/ 0.08)
                EFFECT_OFF_KIND(0x0edd56991a, false, false)
                EFFECT_FOLLOW(0x15880548be, hash40("havel"), 0, 0.0, 0, 0, 0, 0, 1, true)
                PLAY_SE(0x13c2ff2d51 as u64)
                PLAY_SE(hash40("se_item_magicball_warpin"))
                PLAY_SE(hash40("se_item_killsword_flash"))
            }
        }
    });
}

pub fn once_per_luigi_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let kind = smash::app::utility::get_kind(module_accessor);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            
        if kind == *FIGHTER_KIND_LUIGI {
			//println!("Current Status Kind: {}", StatusModule::status_kind(module_accessor));
			//println!("Current Motion Kind: {}", MotionModule::motion_kind(module_accessor));
			//println!("Current Motion Frame: {}", MotionModule::frame(module_accessor));
            //println!("GFX Size: {}", _GFXSIZE[entry_id]);

            //RAGE MODE 
            if AttackModule::is_infliction(module_accessor, *COLLISION_KIND_MASK_HIT){
                if _FCOUNTER[entry_id] < 1 { //If frame counter is 0
                    _HIT_COUNT[entry_id] += 1; //Each hit counts towards getting rage mode
                }
                _FCOUNTER[entry_id] += 1; //Pretty sure it would count every single frame of hitlag as a hit so I had to make a frame counter
            }
            if AttackModule::is_infliction(module_accessor, *COLLISION_KIND_NONE){
                _FCOUNTER[entry_id] = 0; //If you're not in hitlag your frame counter resets to 0
            }
            if _HIT_COUNT[entry_id] >= 10 { //If your hit count is over 10 you are in rage mode and most attacks get a fire effect on them
                _IS_RAGE[entry_id] = true;
                _GFXNAME[entry_id] = "collision_attr_fire";
            }
            if _HIT_COUNT[entry_id] < 10 { //If your hit count is under 10 you are not in rage mode and your attacks have their normal hit effects
                _IS_RAGE[entry_id] = false;
                _GFXNAME[entry_id] = "collision_attr_normal";
            }

            //Forward Smash
            if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD {
                if MotionModule::frame(module_accessor) <= 1.0 {
                    _GFXSIZE[entry_id] = 0.1;
                }
                if _IS_RAGE[entry_id] == false { //If not in rage mode
                    _COUNTING[entry_id] += 1;
                    if _COUNTING[entry_id] >= 5 { //Frame counting variable ofc
                        let lua_state = fighter.lua_state_agent;
                        acmd!(lua_state, {
                            // acmd code goes here
                            EFFECT_FOLLOW(hash40("sys_hit_elec"), hash40("havel"), 0, 0, 0, -30, 0, 0, _GFXSIZE[entry_id] / 6.0, true) //Spawns an electric effect if you are not in rage mode. Wouldn't work in the main script for some reason when I tried
                        });
                        _COUNTING[entry_id] = 0;
                    }
                    if _GFXSIZE[entry_id] <= 1.7 {
                        _GFXSIZE[entry_id] += 0.025;
                    }
                }
                if _IS_RAGE[entry_id] {
                    if _GFXSIZE[entry_id] <= 3.4 {
                        _GFXSIZE[entry_id] += 0.05; //Max GFX size is bigger during rage but it doesn't matter because I got lazy and just started adjusting the gfx with multipliers instead so I don't have to do math
                    }
                }
            }

            //Forward Air
            if MotionModule::motion_kind(module_accessor) == hash40("attack_air_f"){
                if _IS_RAGE[entry_id] && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) || _IS_BURST[entry_id] {
                    _COUNTING[entry_id] += 1;
                    if _COUNTING[entry_id] >= 5 {
                        let lua_state = fighter.lua_state_agent;
                        acmd!(lua_state, {
                            // acmd code goes here
                            EFFECT_FOLLOW(hash40("sys_hit_elec"), hash40("havel"), 0, 0, 0, -30, 0, 0, 0.3, true) //This gfx is spawned on every single aerial when you are trying to use burst mode with it, an orange electric shockwave
                            LAST_EFFECT_SET_COLOR(/*R*/ 0.8, /*G*/ 0.4, /*B*/ 0.0)
                        });
                        _COUNTING[entry_id] = 0;
                    }
                }
            }
            //Back Air
            if MotionModule::motion_kind(module_accessor) == hash40("attack_air_b"){
                if _IS_RAGE[entry_id] && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) || _IS_BURST[entry_id] {
                    _COUNTING[entry_id] += 1;
                    if _COUNTING[entry_id] >= 5 {
                        let lua_state = fighter.lua_state_agent;
                        acmd!(lua_state, {
                            // acmd code goes here
                            EFFECT_FOLLOW(hash40("sys_hit_elec"), hash40("haver"), 0, 0, 0, -30, 0, 0, 0.3, true)
                            LAST_EFFECT_SET_COLOR(/*R*/ 0.8, /*G*/ 0.4, /*B*/ 0.0)
                        });
                        _COUNTING[entry_id] = 0;
                    }
                }
            }
            //Down Air
            if MotionModule::motion_kind(module_accessor) == hash40("attack_air_lw"){
                if _IS_RAGE[entry_id] && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) || _IS_BURST[entry_id] {
                    _COUNTING[entry_id] += 1;
                    if _COUNTING[entry_id] >= 5 {
                        let lua_state = fighter.lua_state_agent;
                        acmd!(lua_state, {
                            // acmd code goes here
                            EFFECT_FOLLOW(hash40("sys_hit_elec"), hash40("toer"), 0, 0, 0, -30, 0, 0, 0.2, true)
                            LAST_EFFECT_SET_COLOR(/*R*/ 0.8, /*G*/ 0.4, /*B*/ 0.0)
                            EFFECT_FOLLOW(hash40("sys_hit_elec"), hash40("toel"), 0, 0, 0, -30, 0, 0, 0.2, true)
                            LAST_EFFECT_SET_COLOR(/*R*/ 0.8, /*G*/ 0.4, /*B*/ 0.0)
                        });
                        _COUNTING[entry_id] = 0;
                    }
                }
            }

            //Final Smash
            if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_FINAL {
                ArticleModule::remove_exist(module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)); 
            } //The article is not actually spawned in his ACMD scripts so I just removed it every frame lmao

            //Death and/or Respawn
            if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ENTRY || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DEAD || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH {
                _IS_RAGE[entry_id] = false;
                _HIT_COUNT[entry_id] = 0;
                //If you die or are just entering a match your hit count is set to 0 and you do not have rage
            }
		}
    }
}

pub fn install() {
    acmd::add_custom_hooks!(once_per_luigi_frame);
    acmd::add_hooks!(
        swole_luigi_effect_attack_11,
        swole_luigi_effect_attack_12,
        swole_luigi_effect_attack_13,
        swole_luigi_effect_attack_dash,
        swole_luigi_effect_attack_s3,
        swole_luigi_effect_attack_s3_hi,
        swole_luigi_effect_attack_s3_lw,
        swole_luigi_effect_attack_hi3,
        swole_luigi_effect_attack_lw3,
        swole_luigi_effect_attack_s4hold,
        swole_luigi_effect_attack_s4hi,
        swole_luigi_effect_attack_s4,
        swole_luigi_effect_attack_s4lw,
        swole_luigi_effect_attack_hi4,
        swole_luigi_effect_attack_lw4hold,
        swole_luigi_effect_attack_lw4,
        swole_luigi_effect_attack_air_f,
        swole_luigi_effect_attack_air_b,
        swole_luigi_effect_attack_air_hi,
        swole_luigi_effect_attack_air_lw,
        swole_luigi_effect_final_start,
        swole_luigi_effect_final_startl,
        swole_luigi_effect_final_start_air,
        swole_luigi_effect_final_start_airl,
        swole_luigi_effect_special_hi,
        swole_luigi_effect_special_air_hi
    );
}
