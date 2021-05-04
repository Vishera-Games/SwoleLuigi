use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};
use crate::luigieffect::_HIT_COUNT;
use crate::luigi::{_IS_RAGE, _GFXNAME, _IS_BURST};

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_13",
    animcmd = "sound_attack13")]
pub fn swole_luigi_sound_attack_13(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=6)
        if(is_excute){
            PLAY_SE(hash40("se_common_swing_03"))
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_dash",
    animcmd = "sound_attackdash")]
pub fn swole_luigi_sound_attack_dash(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=1)
        if(is_excute){
            PLAY_SE(hash40("se_comming_swing_10"))
            PLAY_SE(hash40("se_luigi_step_left_m"))
            PLAY_SEQUENCE(hash40("seq_luigi_rnd_attack"))
        }
        frame(Frame=6)
        if(is_excute){
            PLAY_SE(hash40("se_luigi_step_right_m"))
        }
        frame(Frame=10)
        if(is_excute){
            PLAY_SE(hash40("se_luigi_step_left_m"))
        }
        frame(Frame=13)
        if(is_excute){
            PLAY_SE(hash40("se_luigi_step_right_m"))
        }
        frame(Frame=16)
        if(is_excute){
            PLAY_SE(hash40("se_luigi_step_left_m"))
        }
        frame(Frame=20)
        if(is_excute){
            PLAY_SE(hash40("se_luigi_step_right_m"))
        }
        frame(Frame=23)
        if(is_excute){
            PLAY_SE(hash40("se_luigi_step_left_m"))
        }
        frame(Frame=26)
        if(is_excute){
            PLAY_SE(hash40("se_luigi_step_right_m"))
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_s3_hi",
    animcmd = "sound_attacks3hi")]
pub fn swole_luigi_sound_attack_s3_hi(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        frame(Frame=3)
        if(is_excute){
            PLAY_SE(hash40("se_common_punch_kick_swing_l"))
        }
        frame(Frame=11)
        if(is_excute){
            PLAY_SE(hash40("se_common_punch_kick_swing_l"))
        }
        frame(Frame=15)
        if(is_excute){
            if(_IS_BURST[entry_id]){
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
    animation = "attack_s3_s",
    animcmd = "sound_attacks3")]
pub fn swole_luigi_sound_attack_s3(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        frame(Frame=3)
        if(is_excute){
            PLAY_SE(hash40("se_common_punch_kick_swing_l"))
        }
        frame(Frame=11)
        if(is_excute){
            PLAY_SE(hash40("se_common_punch_kick_swing_l"))
        }
        frame(Frame=15)
        if(is_excute){
            if(_IS_BURST[entry_id]){
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
    animation = "attack_s3_lw",
    animcmd = "sound_attacks3lw")]
pub fn swole_luigi_sound_attack_s3_lw(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        frame(Frame=3)
        if(is_excute){
            PLAY_SE(hash40("se_common_punch_kick_swing_l"))
        }
        frame(Frame=11)
        if(is_excute){
            PLAY_SE(hash40("se_common_punch_kick_swing_l"))
        }
        frame(Frame=15)
        if(is_excute){
            if(_IS_BURST[entry_id]){
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
    animation = "attack_hi3",
    animcmd = "sound_attackhi3")]
pub fn swole_luigi_sound_attack_hi3(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_lw3",
    animcmd = "sound_attacklw3")]
pub fn swole_luigi_sound_attack_lw3(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=9)
        if(is_excute){
            PLAY_SE(hash40("se_common_punch_kick_swing_s"))
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_s4_hi",
    animcmd = "sound_attacks4hi")]
pub fn swole_luigi_sound_attack_s4hi(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        frame(Frame=6)
        if(is_excute){
            STOP_SE(hash40("se_common_smash_start"))
        }
        frame(Frame=13)
        if(is_excute){
            PLAY_SE(hash40("se_common_smashswing_04"))
        }
        frame(Frame=14)
        if(is_excute){
            PLAY_SE(hash40("vc_luigi_attack07"))
            PLAY_SE(hash40("se_luigi_smash_s01"))
        }
        frame(Frame=15)
        if(is_excute){
            PLAY_SE(hash40("se_common_spirits_floor_elec_spark2"))
        }
        frame(Frame=22)
        if(is_excute){
            if(_IS_BURST[entry_id]){
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
    animation = "attack_s4_s",
    animcmd = "sound_attacks4")]
pub fn swole_luigi_sound_attack_s4(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        frame(Frame=6)
        if(is_excute){
            STOP_SE(hash40("se_common_smash_start"))
        }
        frame(Frame=13)
        if(is_excute){
            PLAY_SE(hash40("se_common_smashswing_04"))
        }
        frame(Frame=14)
        if(is_excute){
            PLAY_SE(hash40("vc_luigi_attack07"))
            PLAY_SE(hash40("se_luigi_smash_s01"))
        }
        frame(Frame=15)
        if(is_excute){
            PLAY_SE(hash40("se_common_spirits_floor_elec_spark2"))
        }
        frame(Frame=22)
        if(is_excute){
            if(_IS_BURST[entry_id]){
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
    animation = "attack_s4_lw",
    animcmd = "sound_attacks4lw")]
pub fn swole_luigi_sound_attack_s4lw(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        frame(Frame=6)
        if(is_excute){
            STOP_SE(hash40("se_common_smash_start"))
        }
        frame(Frame=13)
        if(is_excute){
            PLAY_SE(hash40("se_common_smashswing_04"))
        }
        frame(Frame=14)
        if(is_excute){
            PLAY_SE(hash40("vc_luigi_attack07"))
            PLAY_SE(hash40("se_luigi_smash_s01"))
        }
        frame(Frame=15)
        if(is_excute){
            PLAY_SE(hash40("se_common_spirits_floor_elec_spark2"))
        }
        frame(Frame=22)
        if(is_excute){
            if(_IS_BURST[entry_id]){
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
    animation = "attack_hi4",
    animcmd = "sound_attackhi4")]
pub fn swole_luigi_sound_attack_hi4(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_lw4",
    animcmd = "sound_attacklw4")]
pub fn swole_luigi_sound_attack_lw4(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=7)
        if(is_excute){
            PLAY_SE(hash40("vc_luigi_attack06"))
            PLAY_SE(hash40("se_common_punch_kick_swing_l"))
	        PLAY_SE(hash40("se_common_punch_kick_swing_s"))
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_air_n",
    animcmd = "sound_attackairn")]
pub fn swole_luigi_sound_attack_air_n(fighter: &mut L2CFighterCommon) {
    acmd!({
        
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_air_f",
    animcmd = "sound_attackairf")]
pub fn swole_luigi_sound_attack_air_f(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        frame(Frame=16)
        if(is_excute){
            PLAY_SEQUENCE(hash40("seq_luigi_rnd_attack"))
            PLAY_SE(hash40("se_common_swing_04"))
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_air_b",
    animcmd = "sound_attackairb")]
pub fn swole_luigi_sound_attack_air_b(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        frame(Frame=16)
        if(is_excute){
            PLAY_SEQUENCE(hash40("seq_luigi_rnd_attack"))
            PLAY_SE(hash40("se_common_swing_09"))
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_air_hi",
    animcmd = "sound_attackairhi")]
pub fn swole_luigi_sound_attack_air_hi(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_air_lw",
    animcmd = "sound_attackairlw")]
pub fn swole_luigi_sound_attack_air_lw(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        frame(Frame=14)
        if(is_excute){
            PLAY_SEQUENCE(hash40("seq_luigi_rnd_attack"))
            PLAY_SE(hash40("se_common_swing_05"))
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "final_start",
    animcmd = "sound_finalstart")]
pub fn swole_luigi_sound_final_start(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=10)
        if(is_excute){
            PLAY_SE(hash40("se_cloud_attackdash"))
            PLAY_SE(hash40("se_common_spirits_floor_elec_loop"))
            PLAY_SE(hash40("se_common_spirits_wind_loop"))
            PLAY_SE(hash40("se_item_blackball_loop"))
        }
        frame(Frame=67)
        if(is_excute){
            STOP_SE(hash40("se_common_spirits_floor_elec_loop"))
            STOP_SE(hash40("se_common_spirits_wind_loop"))
            STOP_SE(hash40("se_item_blackball_loop"))
            PLAY_SE(hash40("se_common_smash_start_03"))
        }
        frame(Frame=68)
        if(is_excute){
            PLAY_SE(hash40("se_common_spirits_quake_loop"))
        }
        frame(Frame=72)
        if(is_excute){
            PLAY_SE(95102347375 as u64)
            PLAY_SE(hash40("se_common_boss_down"))
        }
        frame(Frame=103)
        if(is_excute){
            STOP_SE(hash40("se_common_spirits_quake_loop"))
            STOP_SE(95102347375 as u64)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "final_start_l",
    animcmd = "sound_finalstartl")]
pub fn swole_luigi_sound_final_startl(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=10)
        if(is_excute){
            PLAY_SE(hash40("se_cloud_attackdash"))
            PLAY_SE(hash40("se_common_spirits_floor_elec_loop"))
            PLAY_SE(hash40("se_common_spirits_wind_loop"))
            PLAY_SE(hash40("se_item_blackball_loop"))
        }
        frame(Frame=67)
        if(is_excute){
            STOP_SE(hash40("se_common_spirits_floor_elec_loop"))
            STOP_SE(hash40("se_common_spirits_wind_loop"))
            STOP_SE(hash40("se_item_blackball_loop"))
            PLAY_SE(hash40("se_common_smash_start_03"))
        }
        frame(Frame=68)
        if(is_excute){
            PLAY_SE(hash40("se_common_spirits_quake_loop"))
        }
        frame(Frame=72)
        if(is_excute){
            PLAY_SE(95102347375 as u64)
            PLAY_SE(hash40("se_common_boss_down"))
        }
        frame(Frame=103)
        if(is_excute){
            STOP_SE(hash40("se_common_spirits_quake_loop"))
            STOP_SE(95102347375 as u64)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "final_air_start",
    animcmd = "sound_finalairstart")]
pub fn swole_luigi_sound_final_start_air(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=10)
        if(is_excute){
            PLAY_SE(hash40("se_cloud_attackdash"))
            PLAY_SE(hash40("se_common_spirits_floor_elec_loop"))
            PLAY_SE(hash40("se_common_spirits_wind_loop"))
            PLAY_SE(hash40("se_item_blackball_loop"))
        }
        frame(Frame=67)
        if(is_excute){
            STOP_SE(hash40("se_common_spirits_floor_elec_loop"))
            STOP_SE(hash40("se_common_spirits_wind_loop"))
            STOP_SE(hash40("se_item_blackball_loop"))
            PLAY_SE(hash40("se_common_smash_start_03"))
        }
        frame(Frame=68)
        if(is_excute){
            PLAY_SE(hash40("se_common_spirits_quake_loop"))
        }
        frame(Frame=72)
        if(is_excute){
            PLAY_SE(95102347375 as u64)
            PLAY_SE(hash40("se_common_boss_down"))
        }
        frame(Frame=103)
        if(is_excute){
            STOP_SE(hash40("se_common_spirits_quake_loop"))
            STOP_SE(95102347375 as u64)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "final_air_start_l",
    animcmd = "sound_finalairstartl")]
pub fn swole_luigi_sound_final_start_airl(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=10)
        if(is_excute){
            PLAY_SE(hash40("se_cloud_attackdash"))
            PLAY_SE(hash40("se_common_spirits_floor_elec_loop"))
            PLAY_SE(hash40("se_common_spirits_wind_loop"))
            PLAY_SE(hash40("se_item_blackball_loop"))
        }
        frame(Frame=67)
        if(is_excute){
            STOP_SE(hash40("se_common_spirits_floor_elec_loop"))
            STOP_SE(hash40("se_common_spirits_wind_loop"))
            STOP_SE(hash40("se_item_blackball_loop"))
            PLAY_SE(hash40("se_common_smash_start_03"))
        }
        frame(Frame=68)
        if(is_excute){
            PLAY_SE(hash40("se_common_spirits_quake_loop"))
        }
        frame(Frame=72)
        if(is_excute){
            PLAY_SE(95102347375 as u64)
            PLAY_SE(hash40("se_common_boss_down"))
        }
        frame(Frame=103)
        if(is_excute){
            STOP_SE(hash40("se_common_spirits_quake_loop"))
            STOP_SE(95102347375 as u64)
        }
    });
}

pub fn install() {
    acmd::add_hooks!(
        //swole_luigi_sound_attack_11,
        //swole_luigi_sound_attack_12,
        swole_luigi_sound_attack_13,
        swole_luigi_sound_attack_dash,
        swole_luigi_sound_attack_s3,
        swole_luigi_sound_attack_s3_hi,
        swole_luigi_sound_attack_s3_lw,
        //swole_luigi_sound_attack_hi3,
        swole_luigi_sound_attack_lw3,
        swole_luigi_sound_attack_s4hi,
        swole_luigi_sound_attack_s4,
        swole_luigi_sound_attack_s4lw,
        //swole_luigi_sound_attack_hi4,
        swole_luigi_sound_attack_lw4,
        //swole_luigi_sound_attack_air_n,
        swole_luigi_sound_attack_air_f,
        swole_luigi_sound_attack_air_b,
        //swole_luigi_sound_attack_air_hi,
        swole_luigi_sound_attack_air_lw,
        swole_luigi_sound_final_start,
        swole_luigi_sound_final_startl,
        swole_luigi_sound_final_start_air,
        swole_luigi_sound_final_start_airl
    );
}
