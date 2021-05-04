use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};
use crate::luigieffect::{_HIT_COUNT, _COUNTING};

pub static mut _GFXNAME: [&str; 8] = ["collision_attr_normal"; 8];
pub static mut _IS_RAGE: [bool; 8] = [false; 8]; //Is the variable for if the player is in burst mode
pub static mut _IS_BURST: [bool; 8] = [false; 8]; //Is the variable for if the player is using an actual burst attack

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_13",
    animcmd = "game_attack13")]
pub fn swole_luigi_attack_13(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        if(is_excute){
            JostleModule::set_status(false)
        }
        frame(Frame=7)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=361, KBG=90, FKB=0, BKB=30, Size=4.5, X=0.0, Y=7.0, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40(_GFXNAME[entry_id]), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=6.0, Angle=361, KBG=90, FKB=0, BKB=30, Size=3.0, X=0.0, Y=6.5, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40(_GFXNAME[entry_id]), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        wait(Frames=1)
        if(is_excute){
            JostleModule::set_status(true)
        }
        frame(Frame=47)
        if(is_excute){
            StatusModule::change_status_request_from_script(*FIGHTER_STATUS_KIND_WAIT, true)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_dash",
    animcmd = "game_attackdash")]
pub fn swole_luigi_attack_dash(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=5)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=30, KBG=60, FKB=0, BKB=60, Size=6.0, X=0.0, Y=6.5, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=5)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.0, Angle=30, KBG=60, FKB=0, BKB=60, Size=6.0, X=0.0, Y=6.5, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=14)
        if(is_excute){
            AttackModule::clear_all()
            if(AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT)){
                CancelModule::enable_cancel() //Made it so you can cancel the move early if you hit someone with it
            }
        }
        frame(Frame=52)
        if(is_excute){
            StatusModule::change_status_request_from_script(*FIGHTER_STATUS_KIND_WAIT, true)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_s3_hi",
    animcmd = "game_attacks3hi")]
pub fn swole_luigi_attack_s3_hi(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        frame(Frame=4)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=30, KBG=100, FKB=25, BKB=0, Size=4.0, X=0.0, Y=6.5, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=60, KBG=100, FKB=25, BKB=0, Size=4.0, X=0.0, Y=6.5, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=4.0, Angle=1, KBG=100, FKB=30, BKB=0, Size=4.0, X=0.0, Y=6.5, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=4.0, Angle=40, KBG=100, FKB=12, BKB=0, Size=4.0, X=0.0, Y=6.5, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=12)
        if(is_excute){
            if(_IS_RAGE[entry_id] == false || _IS_RAGE[entry_id] && ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_ATTACK)){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=361, KBG=130, FKB=0, BKB=30, Size=3.0, X=0.0, Y=5.5, Z=5.0, X2=0.0, Y2=7.5, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40(_GFXNAME[entry_id]), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=9.5, Angle=361, KBG=128, FKB=0, BKB=30, Size=0.5, X=0.0, Y=6.5, Z=11.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=130, FKB=0, BKB=30, Size=4.0, X=0.0, Y=6.5, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40(_GFXNAME[entry_id]), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
            if (_IS_RAGE[entry_id] && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT)) {
                HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_XLU), 0)
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=30, FKB=0, BKB=30, Size=3.0, X=0.0, Y=5.5, Z=5.0, X2=0.0, Y2=7.5, Z2=5.0, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=30, FKB=0, BKB=30, Size=0.5, X=0.0, Y=6.5, Z=11.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=30, FKB=0, BKB=30, Size=4.0, X=0.0, Y=6.5, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
                CAM_ZOOM_IN_arg5(10, 0, 2.0, -0.25, -1.0 * PostureModule::lr(module_accessor)) //Camera zoom in function
                rust{
                    _IS_BURST[entry_id] = true; //If the player is holding A by frame 12, the attack does a burst mode with alternate hitboxes + an explosion
                }
            }
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
            if(_IS_BURST[entry_id]){
                SlowModule::set_whole(10, 0) //Slow time for dramatic effect if you're bursting
            }
        }
        wait(Frames=1)
        if(is_excute){
            if(_IS_BURST[entry_id] && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT)){ //If the player actually landed the burst attack, this is the followup explosion
                SlowModule::clear_whole()
                AttackModule::set_damage_effect_mul_scale(0.3) //I tried to make the hit explosion smaller so you could see the green gfx better but idk if it did anything
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=17.0, Angle=361, KBG=95, FKB=0, BKB=30, Size=3.0, X=0.0, Y=5.5, Z=5.0, X2=0.0, Y2=7.5, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40(_GFXNAME[entry_id]), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=17.0, Angle=361, KBG=95, FKB=0, BKB=30, Size=7.0, X=0.0, Y=6.5, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40(_GFXNAME[entry_id]), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
            HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_NORMAL), 0)
            CAM_ZOOM_OUT()
            if(_IS_BURST[entry_id] && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT)){ 
                rust{
                    _IS_RAGE[entry_id] = false;
                    _HIT_COUNT[entry_id] = 0;
                    _IS_BURST[entry_id] = false; //All rage mode and burst related stuff is set to non burst if the player used a burst attack and landed it
                }
            }
        }
        frame(Frame=50)
        if(is_excute){
            StatusModule::change_status_request_from_script(*FIGHTER_STATUS_KIND_WAIT, true)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_s3_s",
    animcmd = "game_attacks3")]
pub fn swole_luigi_attack_s3(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        frame(Frame=4)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=30, KBG=100, FKB=25, BKB=0, Size=4.0, X=0.0, Y=6.5, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=60, KBG=100, FKB=25, BKB=0, Size=4.0, X=0.0, Y=6.5, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=4.0, Angle=1, KBG=100, FKB=30, BKB=0, Size=4.0, X=0.0, Y=6.5, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=4.0, Angle=40, KBG=100, FKB=12, BKB=0, Size=4.0, X=0.0, Y=6.5, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=12)
        if(is_excute){
            if(_IS_RAGE[entry_id] == false || _IS_RAGE[entry_id] && ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_ATTACK)){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=361, KBG=130, FKB=0, BKB=30, Size=3.0, X=0.0, Y=5.5, Z=5.0, X2=0.0, Y2=7.5, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40(_GFXNAME[entry_id]), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=9.5, Angle=361, KBG=128, FKB=0, BKB=30, Size=0.5, X=0.0, Y=6.5, Z=11.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=130, FKB=0, BKB=30, Size=4.0, X=0.0, Y=6.5, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40(_GFXNAME[entry_id]), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
            if (_IS_RAGE[entry_id] && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT)) {
                HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_XLU), 0)
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=30, FKB=0, BKB=30, Size=3.0, X=0.0, Y=5.5, Z=5.0, X2=0.0, Y2=7.5, Z2=5.0, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=30, FKB=0, BKB=30, Size=0.5, X=0.0, Y=6.5, Z=11.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=30, FKB=0, BKB=30, Size=4.0, X=0.0, Y=6.5, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
                CAM_ZOOM_IN_arg5(10, 0, 2.0, -0.25, -1.0 * PostureModule::lr(module_accessor))
                rust{
                    _IS_BURST[entry_id] = true;
                }
            }
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
            if(_IS_BURST[entry_id]){
                SlowModule::set_whole(10, 0)
            }
        }
        wait(Frames=1)
        if(is_excute){
            if(_IS_BURST[entry_id] && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT)){
                SlowModule::clear_whole()
                AttackModule::set_damage_effect_mul_scale(0.3)
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=17.0, Angle=361, KBG=95, FKB=0, BKB=30, Size=3.0, X=0.0, Y=5.5, Z=5.0, X2=0.0, Y2=7.5, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40(_GFXNAME[entry_id]), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=17.0, Angle=361, KBG=95, FKB=0, BKB=30, Size=7.0, X=0.0, Y=6.5, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40(_GFXNAME[entry_id]), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
            HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_NORMAL), 0)
            CAM_ZOOM_OUT()
            if(_IS_BURST[entry_id] && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT)){
                rust{
                    _IS_RAGE[entry_id] = false;
                    _HIT_COUNT[entry_id] = 0;
                    _IS_BURST[entry_id] = false;
                }
            }
        }
        frame(Frame=50)
        if(is_excute){
            StatusModule::change_status_request_from_script(*FIGHTER_STATUS_KIND_WAIT, true)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_s3_lw",
    animcmd = "game_attacks3lw")]
pub fn swole_luigi_attack_s3_lw(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        frame(Frame=4)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=30, KBG=100, FKB=25, BKB=0, Size=4.0, X=0.0, Y=6.5, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=60, KBG=100, FKB=25, BKB=0, Size=4.0, X=0.0, Y=6.5, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=4.0, Angle=1, KBG=100, FKB=30, BKB=0, Size=4.0, X=0.0, Y=6.5, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=4.0, Angle=40, KBG=100, FKB=12, BKB=0, Size=4.0, X=0.0, Y=6.5, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=12)
        if(is_excute){
            if(_IS_RAGE[entry_id] == false || _IS_RAGE[entry_id] && ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_ATTACK)){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=361, KBG=130, FKB=0, BKB=30, Size=3.0, X=0.0, Y=5.5, Z=5.0, X2=0.0, Y2=7.5, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40(_GFXNAME[entry_id]), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=9.5, Angle=361, KBG=128, FKB=0, BKB=30, Size=0.5, X=0.0, Y=6.5, Z=11.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=130, FKB=0, BKB=30, Size=4.0, X=0.0, Y=6.5, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40(_GFXNAME[entry_id]), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
            if (_IS_RAGE[entry_id] && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT)) {
                HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_XLU), 0)
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=30, FKB=0, BKB=30, Size=3.0, X=0.0, Y=5.5, Z=5.0, X2=0.0, Y2=7.5, Z2=5.0, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=30, FKB=0, BKB=30, Size=0.5, X=0.0, Y=6.5, Z=11.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=30, FKB=0, BKB=30, Size=4.0, X=0.0, Y=6.5, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
                CAM_ZOOM_IN_arg5(10, 0, 2.0, -0.25, -1.0 * PostureModule::lr(module_accessor))
                rust{
                    _IS_BURST[entry_id] = true;
                }
            }
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
            if(_IS_BURST[entry_id]){
                SlowModule::set_whole(10, 0)
            }
        }
        wait(Frames=1)
        if(is_excute){
            if(_IS_BURST[entry_id] && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT)){
                SlowModule::clear_whole()
                AttackModule::set_damage_effect_mul_scale(0.3)
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=17.0, Angle=361, KBG=95, FKB=0, BKB=30, Size=3.0, X=0.0, Y=5.5, Z=5.0, X2=0.0, Y2=7.5, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40(_GFXNAME[entry_id]), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=17.0, Angle=361, KBG=95, FKB=0, BKB=30, Size=7.0, X=0.0, Y=6.5, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40(_GFXNAME[entry_id]), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
            HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_NORMAL), 0)
            CAM_ZOOM_OUT()
            if(_IS_BURST[entry_id] && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT)){
                rust{
                    _IS_RAGE[entry_id] = false;
                    _HIT_COUNT[entry_id] = 0;
                    _IS_BURST[entry_id] = false;
                }
            }
        }
        frame(Frame=50)
        if(is_excute){
            StatusModule::change_status_request_from_script(*FIGHTER_STATUS_KIND_WAIT, true)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_hi3",
    animcmd = "game_attackhi3")]
pub fn swole_luigi_attack_hi3(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        frame(Frame=5)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("neck"), Damage=6.0, Angle=100, KBG=150, FKB=0, BKB=8, Size=3.3, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40(_GFXNAME[entry_id]), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=6.0, Angle=100, KBG=150, FKB=0, BKB=8, Size=4.3, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40(_GFXNAME[entry_id]), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("handl"), Damage=6.0, Angle=100, KBG=150, FKB=0, BKB=8, Size=4.8, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40(_GFXNAME[entry_id]), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=6)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_lw3",
    animcmd = "game_attacklw3")]
pub fn swole_luigi_attack_lw3(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        frame(Frame=11)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.0, Angle=75, KBG=65, FKB=0, BKB=45, Size=3.0, X=0.0, Y=3.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40(_GFXNAME[entry_id]), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.0, Angle=75, KBG=65, FKB=0, BKB=45, Size=4.8, X=0.0, Y=3.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40(_GFXNAME[entry_id]), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        wait(Frames=3)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_s4_hi",
    animcmd = "game_attacks4hi")]
pub fn swole_luigi_attack_s4hi(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        frame(Frame=5)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=15)
        if(is_excute){
            if(_IS_RAGE[entry_id]){
                HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_XLU), 0)
                ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=3.0, Angle=361, KBG=30, FKB=0, BKB=30, Size=3.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=6.0, Angle=361, KBG=30, FKB=0, BKB=30, Size=1.0, X=0.0, Y=12.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_ENERGY)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=4.0, Angle=361, KBG=30, FKB=0, BKB=30, Size=5.0, X=0.0, Y=12.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_ENERGY)
                rust{
                    _IS_BURST[entry_id] = true;
                }
            }
            if(_IS_RAGE[entry_id] == false){
                ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=18.0, Angle=361, KBG=70, FKB=0, BKB=30, Size=3.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=20.0, Angle=361, KBG=70, FKB=0, BKB=30, Size=1.0, X=0.0, Y=12.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_ENERGY)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=18.0, Angle=361, KBG=70, FKB=0, BKB=30, Size=5.0, X=0.0, Y=12.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_ENERGY)
            }
        }
        wait(Frames=1)
        if(is_excute){
            if(_IS_RAGE[entry_id]){
                if(AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT)){
                    SlowModule::set_whole(2, 0)
                    CAM_ZOOM_IN_arg5(5, 0, 2.0, -0.25, -1.0 * PostureModule::lr(module_accessor))
                }
            }
        }
        wait(Frames=1)
        if(is_excute){
            if(_IS_RAGE[entry_id] == false){
                AttackModule::clear(ID=0, false)
            }
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=22)
        if(is_excute){
            SlowModule::clear_whole()
            if(AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) && _IS_RAGE[entry_id]){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=24.0, Angle=361, KBG=65, FKB=0, BKB=30, Size=1.0, X=0.0, Y=12.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=3.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_ENERGY)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=20.0, Angle=361, KBG=65, FKB=0, BKB=30, Size=7.0, X=0.0, Y=12.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_ENERGY)
            }
        }
        wait(Frames=2)
        if(is_excute){
            CAM_ZOOM_OUT()
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        wait(Frames=1)
        if(is_excute){
            HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_NORMAL), 0)
            rust{
                if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) && _IS_BURST[entry_id] {
                    _IS_RAGE[entry_id] = false;
                    _HIT_COUNT[entry_id] = 0;
                    _IS_BURST[entry_id] = false;
                }
            }
        }
        frame(Frame=55)
        if(is_excute){
            StatusModule::change_status_request_from_script(*FIGHTER_STATUS_KIND_WAIT, true)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_s4_s",
    animcmd = "game_attacks4")]
pub fn swole_luigi_attack_s4(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        frame(Frame=5)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=15)
        if(is_excute){
            if(_IS_RAGE[entry_id]){
                HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_XLU), 0)
                ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=3.0, Angle=361, KBG=30, FKB=0, BKB=30, Size=3.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=6.0, Angle=361, KBG=30, FKB=0, BKB=30, Size=1.0, X=0.0, Y=8.9, Z=14.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_ENERGY)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=4.0, Angle=361, KBG=30, FKB=0, BKB=30, Size=5.0, X=0.0, Y=8.9, Z=14.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_ENERGY)
                rust{
                    _IS_BURST[entry_id] = true;
                }
            }
            if(_IS_RAGE[entry_id] == false){
                ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=18.0, Angle=361, KBG=70, FKB=0, BKB=30, Size=3.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=20.0, Angle=361, KBG=70, FKB=0, BKB=30, Size=1.0, X=0.0, Y=8.9, Z=14.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_ENERGY)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=18.0, Angle=361, KBG=70, FKB=0, BKB=30, Size=5.0, X=0.0, Y=8.9, Z=14.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_ENERGY)
            }
        }
        wait(Frames=1)
        if(is_excute){
            if(_IS_RAGE[entry_id]){
                if(AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT)){
                    SlowModule::set_whole(2, 0)
                    CAM_ZOOM_IN_arg5(5, 0, 2.0, -0.25, -1.0 * PostureModule::lr(module_accessor))
                }
            }
        }
        wait(Frames=1)
        if(is_excute){
            if(_IS_RAGE[entry_id] == false){
                AttackModule::clear(ID=0, false)
            }
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=22)
        if(is_excute){
            SlowModule::clear_whole()
            if(AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) && _IS_RAGE[entry_id]){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=24.0, Angle=361, KBG=65, FKB=0, BKB=30, Size=1.0, X=0.0, Y=8.9, Z=14.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=3.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_ENERGY)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=20.0, Angle=361, KBG=65, FKB=0, BKB=30, Size=7.0, X=0.0, Y=8.9, Z=14.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_ENERGY)
            }
        }
        wait(Frames=2)
        if(is_excute){
            CAM_ZOOM_OUT()
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        wait(Frames=1)
        if(is_excute){
            HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_NORMAL), 0)
            rust{
                if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) && _IS_BURST[entry_id] {
                    _IS_RAGE[entry_id] = false;
                    _HIT_COUNT[entry_id] = 0;
                    _IS_BURST[entry_id] = false;
                }
            }
        }
        frame(Frame=55)
        if(is_excute){
            StatusModule::change_status_request_from_script(*FIGHTER_STATUS_KIND_WAIT, true)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_s4_lw",
    animcmd = "game_attacks4lw")]
pub fn swole_luigi_attack_s4lw(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        frame(Frame=5)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=15)
        if(is_excute){
            if(_IS_RAGE[entry_id]){
                HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_XLU), 0)
                ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=3.0, Angle=361, KBG=30, FKB=0, BKB=30, Size=3.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=6.0, Angle=361, KBG=30, FKB=0, BKB=30, Size=1.0, X=0.0, Y=4.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_ENERGY)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=4.0, Angle=361, KBG=30, FKB=0, BKB=30, Size=5.0, X=0.0, Y=4.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_ENERGY)
                rust{
                    _IS_BURST[entry_id] = true;
                }
            }
            if(_IS_RAGE[entry_id] == false){
                ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=18.0, Angle=361, KBG=70, FKB=0, BKB=30, Size=3.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=20.0, Angle=361, KBG=70, FKB=0, BKB=30, Size=1.0, X=0.0, Y=4.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_ENERGY)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=18.0, Angle=361, KBG=70, FKB=0, BKB=30, Size=5.0, X=0.0, Y=4.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_ENERGY)
            }
        }
        wait(Frames=1)
        if(is_excute){
            if(_IS_RAGE[entry_id]){
                if(AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT)){
                    SlowModule::set_whole(2, 0)
                    CAM_ZOOM_IN_arg5(5, 0, 2.0, -0.25, -1.0 * PostureModule::lr(module_accessor))
                }
            }
        }
        wait(Frames=1)
        if(is_excute){
            if(_IS_RAGE[entry_id] == false){
                AttackModule::clear(ID=0, false)
            }
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=22)
        if(is_excute){
            SlowModule::clear_whole()
            if(AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) && _IS_BURST[entry_id]){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=24.0, Angle=361, KBG=65, FKB=0, BKB=30, Size=1.0, X=0.0, Y=4.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=3.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_ENERGY)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=20.0, Angle=361, KBG=65, FKB=0, BKB=30, Size=7.0, X=0.0, Y=4.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_ENERGY)
            }
        }
        wait(Frames=2)
        if(is_excute){
            CAM_ZOOM_OUT()
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        wait(Frames=1)
        if(is_excute){
            HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_NORMAL), 0)
            rust{
                if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) && _IS_RAGE[entry_id] {
                    _IS_RAGE[entry_id] = false;
                    _HIT_COUNT[entry_id] = 0;
                    _IS_BURST[entry_id] = false;
                }
            }
        }
        frame(Frame=55)
        if(is_excute){
            StatusModule::change_status_request_from_script(*FIGHTER_STATUS_KIND_WAIT, true)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_hi4",
    animcmd = "game_attackhi4")]
pub fn swole_luigi_attack_hi4(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_lw4",
    animcmd = "game_attacklw4")]
pub fn swole_luigi_attack_lw4(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        frame(Frame=3)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=9)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=20.0, Angle=35, KBG=58, FKB=0, BKB=50, Size=4.5, X=0.0, Y=6.0, Z=1.0, X2=0.0, Y2=6.0, Z2=13.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40(_GFXNAME[entry_id]), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=18.0, Angle=35, KBG=58, FKB=0, BKB=50, Size=4.5, X=0.0, Y=6.0, Z=-1.0, X2=0.0, Y2=6.0, Z2=-8.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40(_GFXNAME[entry_id]), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        wait(Frames=3)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_air_n",
    animcmd = "game_attackairn")]
pub fn swole_luigi_attack_air_n(fighter: &mut L2CFighterCommon) {
    acmd!({
        
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_air_f",
    animcmd = "game_attackairf")]
pub fn swole_luigi_attack_air_f(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        FT_MOTION_RATE(FSM=0.75)
        frame(Frame=14)
        FT_MOTION_RATE(FSM=1.0)
        frame(Frame=16)
        if(is_excute){
            if(_IS_RAGE[entry_id] && ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) || _IS_RAGE[entry_id] == false) {
                ATTACK(ID=0, Part=0, Bone=hash40("shoulderl"), Damage=10.0, Angle=75, KBG=50, FKB=0, BKB=30, Size=2.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40(_GFXNAME[entry_id]), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("handl"), Damage=15.0, Angle=361, KBG=70, FKB=0, BKB=30, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40(_GFXNAME[entry_id]), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
            if(_IS_RAGE[entry_id] && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK)) {
                HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_XLU), 0)
                ATTACK(ID=0, Part=0, Bone=hash40("shoulderl"), Damage=25.0, Angle=361, KBG=80, FKB=0, BKB=40, Size=2.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("handl"), Damage=25.0, Angle=361, KBG=80, FKB=0, BKB=40, Size=7.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_PUNCH)
                rust{
                    _IS_BURST[entry_id] = true;
                }
            }
        }
        frame(Frame=23)
        if(is_excute){
            AttackModule::clear_all()
            HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_NORMAL), 0)
            rust{
                if _IS_BURST[entry_id]{
                    if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) && _IS_RAGE[entry_id] {
                        _HIT_COUNT[entry_id] = 0;
                        _IS_RAGE[entry_id] = false;
                    }
                    _IS_BURST[entry_id] = false;
                }
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_air_b",
    animcmd = "game_attackairb")]
pub fn swole_luigi_attack_air_b(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        FT_MOTION_RATE(FSM=0.75)
        frame(Frame=15)
        FT_MOTION_RATE(FSM=1.0)
        frame(Frame=17)
        if(is_excute){
            if(_IS_RAGE[entry_id] == false || _IS_RAGE[entry_id] && ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_ATTACK)) {
                ATTACK(ID=0, Part=0, Bone=hash40("shoulderr"), Damage=15.0, Angle=361, KBG=50, FKB=0, BKB=30, Size=2.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40(_GFXNAME[entry_id]), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("handr"), Damage=19.0, Angle=361, KBG=70, FKB=0, BKB=30, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40(_GFXNAME[entry_id]), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
            if(_IS_RAGE[entry_id] && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK)) {
                HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_XLU), 0)
                ATTACK(ID=0, Part=0, Bone=hash40("shoulderr"), Damage=23.0, Angle=361, KBG=80, FKB=0, BKB=20, Size=2.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("handr"), Damage=23.0, Angle=361, KBG=80, FKB=0, BKB=20, Size=7.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_PUNCH)
                rust{
                    _IS_BURST[entry_id] = true;
                }
            }
        }
        frame(Frame=18)
        if(is_excute){
            if(_IS_BURST[entry_id] == false){
                ATTACK(ID=1, Part=0, Bone=hash40("handr"), Damage=19.0, Angle=270, KBG=80, FKB=0, BKB=15, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40(_GFXNAME[entry_id]), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
            if(_IS_BURST[entry_id]){
                ATTACK(ID=1, Part=0, Bone=hash40("handr"), Damage=23.0, Angle=270, KBG=80, FKB=0, BKB=25, Size=7.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
        }
        frame(Frame=19)
        if(is_excute){
            if(_IS_BURST[entry_id] == false){
                ATTACK(ID=0, Part=0, Bone=hash40("shoulderr"), Damage=10.0, Angle=361, KBG=50, FKB=0, BKB=30, Size=2.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40(_GFXNAME[entry_id]), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
        }
        frame(Frame=21)
        if(is_excute){
            if(_IS_BURST[entry_id] == false){
                ATTACK(ID=0, Part=0, Bone=hash40("shoulderr"), Damage=6.0, Angle=361, KBG=50, FKB=0, BKB=30, Size=2.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40(_GFXNAME[entry_id]), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("handr"), Damage=6.0, Angle=361, KBG=70, FKB=0, BKB=30, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40(_GFXNAME[entry_id]), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            }
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
            HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_NORMAL), 0)
            rust{
                if _IS_BURST[entry_id]{
                    if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) && _IS_RAGE[entry_id] {
                        _HIT_COUNT[entry_id] = 0;
                        _IS_RAGE[entry_id] = false;
                    }
                    _IS_BURST[entry_id] = false;
                }
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_air_hi",
    animcmd = "game_attackairhi")]
pub fn swole_luigi_attack_air_hi(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        frame(Frame=2)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=5)
        if(is_excute){
            if(_IS_RAGE[entry_id] == false) {
                ATTACK(ID=0, Part=0, Bone=hash40("legr"), Damage=11.0, Angle=55, KBG=100, FKB=0, BKB=0, Size=4.0, X=1.2, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=11.0, Angle=55, KBG=100, FKB=0, BKB=0, Size=4.0, X=3.2, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            }
            if(_IS_RAGE[entry_id]){
                ATTACK(ID=0, Part=0, Bone=hash40("legr"), Damage=11.0, Angle=85, KBG=90, FKB=0, BKB=0, Size=4.0, X=1.2, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=11.0, Angle=85, KBG=90, FKB=0, BKB=0, Size=4.0, X=3.2, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            }
        }
        frame(Frame=8)
        if(is_excute){
            ATK_POWER(0, 7)
            ATK_POWER(1, 7)
        }
        frame(Frame=12)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=19)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_air_lw",
    animcmd = "game_attackairlw")]
pub fn swole_luigi_attack_air_lw(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        if(is_excute){
            MotionModule::set_rate(2.0)
        }
        frame(Frame=10)
        if(is_excute){
            MotionModule::set_rate(1.0)
        }
        frame(Frame=16)
        if(is_excute){
            if(_IS_RAGE[entry_id] == false || _IS_RAGE[entry_id] && ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_ATTACK)) {
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=270, KBG=100, FKB=0, BKB=20, Size=5.5, X=0.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40(_GFXNAME[entry_id]), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            }
            if(_IS_RAGE[entry_id] && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK)) {
                HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_XLU), 0)
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=20.0, Angle=270, KBG=100, FKB=0, BKB=20, Size=5.5, X=0.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                rust{
                    _IS_BURST[entry_id] = true;
                }
            }
        }
        wait(Frames=3)
        if(is_excute){
            AttackModule::clear_all()
            HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_NORMAL), 0)
            rust{
                if _IS_BURST[entry_id]{
                    if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) && _IS_RAGE[entry_id] {
                        _HIT_COUNT[entry_id] = 0;
                        _IS_RAGE[entry_id] = false;
                    }
                    _IS_BURST[entry_id] = false;
                }
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "final_start",
    animcmd = "game_finalstart")]
pub fn swole_luigi_final_start(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        if(is_excute){
            SLOW_OPPONENT(10, 70)
            SlowModule::set_whole(2, 0)
            HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_XLU), 0)
        }
        frame(Frame=10)
        if(is_excute){
            FT_SET_FINAL_FEAR_FACE(90)
            CAM_ZOOM_IN_arg5(20, 0, 2.0, -0.25, 1.0 * PostureModule::lr(module_accessor))
            //REQ_FINAL_START_CAMERA_arg3(hash40("d04finalstart02.nuanmb"), false, false)
            FT_START_CUTIN()
        }
        frame(Frame=64)
        if(is_excute){
            SlowModule::clear_whole()
        }
        frame(Frame=66)
        if(is_excute){
            SlowModule::set_whole(4, 0)
            CAM_ZOOM_OUT()
        }
        frame(Frame=68)
        FT_MOTION_RATE(FSM=2.0)
        if(is_excute){
            CameraModule::set_camera_type(4) //Zooms the camera out to the max
        }
        frame(Frame=70)
        FT_MOTION_RATE(FSM=4.5)
        if(is_excute){
            SlowModule::clear_whole()
        }
        frame(Frame=72.3)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.2, Angle=361, KBG=5, FKB=12, BKB=10, Size=8.0, X=0.0, Y=7.0, Z=12.0, X2=0.0, Y2=7.0, Z2=60.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_ENERGY)
        }
        frame(Frame=73)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.2, Angle=361, KBG=5, FKB=12, BKB=10, Size=8.0, X=0.0, Y=7.0, Z=12.0, X2=0.0, Y2=7.0, Z2=400.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_ENERGY)
        }
        frame(Frame=102)
        if(is_excute){
            AttackModule::clear_all()
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=361, KBG=170, FKB=0, BKB=80, Size=8.0, X=0.0, Y=7.0, Z=12.0, X2=0.0, Y2=7.0, Z2=400.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_ENERGY)
        }
        frame(Frame=106)
        FT_MOTION_RATE(FSM=1.0)
        if(is_excute){
            AttackModule::clear_all()
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=106)
        FT_MOTION_RATE(FSM=1.0)
        frame(Frame=129)
        if(is_excute){
            HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_NORMAL), 0)
            CameraModule::set_camera_type(1) //Resets the camera to normal type
            StatusModule::change_status_request_from_script(*FIGHTER_STATUS_KIND_WAIT, true)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "final_start_l",
    animcmd = "game_finalstartl")]
pub fn swole_luigi_final_startl(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        if(is_excute){
            SLOW_OPPONENT(10, 70)
            SlowModule::set_whole(2, 0)
            HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_XLU), 0)
        }
        frame(Frame=10)
        if(is_excute){
            FT_SET_FINAL_FEAR_FACE(90)
            CAM_ZOOM_IN_arg5(20, 0, 2.0, -0.25, 1.0 * PostureModule::lr(module_accessor))
            //REQ_FINAL_START_CAMERA_arg3(hash40("d04finalstart02.nuanmb"), false, false)
            FT_START_CUTIN()
        }
        frame(Frame=64)
        if(is_excute){
            SlowModule::clear_whole()
        }
        frame(Frame=66)
        if(is_excute){
            SlowModule::set_whole(4, 0)
            CAM_ZOOM_OUT()
        }
        frame(Frame=68)
        FT_MOTION_RATE(FSM=2.0)
        if(is_excute){
            CameraModule::set_camera_type(4)
        }
        frame(Frame=70)
        FT_MOTION_RATE(FSM=4.5)
        if(is_excute){
            SlowModule::clear_whole()
        }
        frame(Frame=72.3)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.2, Angle=361, KBG=5, FKB=12, BKB=10, Size=8.0, X=0.0, Y=7.0, Z=12.0, X2=0.0, Y2=7.0, Z2=60.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_ENERGY)
        }
        frame(Frame=73)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.2, Angle=361, KBG=5, FKB=12, BKB=10, Size=8.0, X=0.0, Y=7.0, Z=12.0, X2=0.0, Y2=7.0, Z2=400.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_ENERGY)
        }
        frame(Frame=102)
        if(is_excute){
            AttackModule::clear_all()
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=361, KBG=170, FKB=0, BKB=80, Size=8.0, X=0.0, Y=7.0, Z=12.0, X2=0.0, Y2=7.0, Z2=400.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_ENERGY)
        }
        frame(Frame=106)
        FT_MOTION_RATE(FSM=1.0)
        if(is_excute){
            AttackModule::clear_all()
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=106)
        FT_MOTION_RATE(FSM=1.0)
        frame(Frame=129)
        if(is_excute){
            HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_NORMAL), 0)
            CameraModule::set_camera_type(1)
            StatusModule::change_status_request_from_script(*FIGHTER_STATUS_KIND_WAIT, true)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "final_air_start",
    animcmd = "game_finalairstart")]
pub fn swole_luigi_final_start_air(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        if(is_excute){
            SLOW_OPPONENT(10, 70)
            SlowModule::set_whole(2, 0)
            HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_XLU), 0)
        }
        frame(Frame=10)
        if(is_excute){
            FT_SET_FINAL_FEAR_FACE(90)
            CAM_ZOOM_IN_arg5(20, 0, 2.0, -0.25, 1.0 * PostureModule::lr(module_accessor))
            //REQ_FINAL_START_CAMERA_arg3(hash40("d04finalstart02.nuanmb"), false, false)
            FT_START_CUTIN()
        }
        frame(Frame=64)
        if(is_excute){
            SlowModule::clear_whole()
        }
        frame(Frame=66)
        if(is_excute){
            SlowModule::set_whole(4, 0)
            CAM_ZOOM_OUT()
        }
        frame(Frame=68)
        FT_MOTION_RATE(FSM=2.0)
        if(is_excute){
            CameraModule::set_camera_type(4)
        }
        frame(Frame=70)
        FT_MOTION_RATE(FSM=4.5)
        if(is_excute){
            SlowModule::clear_whole()
        }
        frame(Frame=72.3)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.2, Angle=361, KBG=5, FKB=12, BKB=10, Size=8.0, X=0.0, Y=7.0, Z=12.0, X2=0.0, Y2=7.0, Z2=60.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_ENERGY)
        }
        frame(Frame=73)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.2, Angle=361, KBG=5, FKB=12, BKB=10, Size=8.0, X=0.0, Y=7.0, Z=12.0, X2=0.0, Y2=7.0, Z2=400.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_ENERGY)
        }
        frame(Frame=102)
        if(is_excute){
            AttackModule::clear_all()
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=361, KBG=170, FKB=0, BKB=80, Size=8.0, X=0.0, Y=7.0, Z=12.0, X2=0.0, Y2=7.0, Z2=400.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_ENERGY)
        }
        frame(Frame=106)
        FT_MOTION_RATE(FSM=1.0)
        if(is_excute){
            AttackModule::clear_all()
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=106)
        FT_MOTION_RATE(FSM=1.0)
        frame(Frame=120)
        if(is_excute){
            HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_NORMAL), 0)
            CameraModule::set_camera_type(1)
            StatusModule::change_status_request_from_script(*FIGHTER_STATUS_KIND_WAIT, true)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "final_air_start_l",
    animcmd = "game_finalairstartl")]
pub fn swole_luigi_final_start_airl(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        if(is_excute){
            SLOW_OPPONENT(10, 70)
            SlowModule::set_whole(2, 0)
            HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_XLU), 0)
        }
        frame(Frame=10)
        if(is_excute){
            FT_SET_FINAL_FEAR_FACE(90)
            CAM_ZOOM_IN_arg5(20, 0, 2.0, -0.25, 1.0 * PostureModule::lr(module_accessor))
            //REQ_FINAL_START_CAMERA_arg3(hash40("d04finalstart02.nuanmb"), false, false)
            FT_START_CUTIN()
        }
        frame(Frame=64)
        if(is_excute){
            SlowModule::clear_whole()
        }
        frame(Frame=66)
        if(is_excute){
            SlowModule::set_whole(4, 0)
            CAM_ZOOM_OUT()
        }
        frame(Frame=68)
        FT_MOTION_RATE(FSM=2.0)
        if(is_excute){
            CameraModule::set_camera_type(4)
        }
        frame(Frame=70)
        FT_MOTION_RATE(FSM=4.5)
        if(is_excute){
            SlowModule::clear_whole()
        }
        frame(Frame=72.3)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.2, Angle=361, KBG=5, FKB=12, BKB=10, Size=8.0, X=0.0, Y=7.0, Z=12.0, X2=0.0, Y2=7.0, Z2=60.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_ENERGY)
        }
        frame(Frame=73)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.2, Angle=361, KBG=5, FKB=12, BKB=10, Size=8.0, X=0.0, Y=7.0, Z=12.0, X2=0.0, Y2=7.0, Z2=400.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_ENERGY)
        }
        frame(Frame=102)
        if(is_excute){
            AttackModule::clear_all()
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=361, KBG=170, FKB=0, BKB=80, Size=8.0, X=0.0, Y=7.0, Z=12.0, X2=0.0, Y2=7.0, Z2=400.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_ENERGY)
        }
        frame(Frame=106)
        FT_MOTION_RATE(FSM=1.0)
        if(is_excute){
            AttackModule::clear_all()
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=106)
        FT_MOTION_RATE(FSM=1.0)
        frame(Frame=120)
        if(is_excute){
            HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_NORMAL), 0)
            CameraModule::set_camera_type(1)
            StatusModule::change_status_request_from_script(*FIGHTER_STATUS_KIND_WAIT, true)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "special_hi",
    animcmd = "game_specialhi")]
pub fn swole_luigi_special_hi(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        if(is_excute){
            if(_IS_RAGE[entry_id]) {
                HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_XLU), 0)
            }
            rust{
                _COUNTING[entry_id] = 0;
            }
        }
        frame(Frame=8)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR)
            if(_IS_RAGE[entry_id] == false) {
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=25.0, Angle=88, KBG=88, FKB=0, BKB=50, Size=2.2, X=1.2, Y=6.0, Z=7.0, X2=-1.2, Y2=6.0, Z2=7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_BAT, Type=ATTACK_REGION_PUNCH)
                WorkModule::on_flag(Flag=FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT)
            }
            if(_IS_RAGE[entry_id]) {
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=90, KBG=100, FKB=140, BKB=50, Size=6.0, X=0.0, Y=4.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
                rust{
                    _IS_BURST[entry_id] = true;
                }
            }
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
            if(_IS_BURST[entry_id]) {
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=90, KBG=100, FKB=80, BKB=50, Size=6.0, X=0.0, Y=17.5, Z=5.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=90, KBG=100, FKB=100, BKB=50, Size=6.0, X=0.0, Y=10.0, Z=5.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=0.0, Angle=366, KBG=100, FKB=100, BKB=50, Size=8.0, X=0.0, Y=17.5, Z=5.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_none"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            }
        }
        wait(Frames=1)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS)
        }
        wait(Frames=1)
        if(is_excute){
            SA_SET(State=SITUATION_KIND_AIR)
            if(_IS_BURST[entry_id]) {
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=90, KBG=100, FKB=70, BKB=50, Size=6.0, X=0.0, Y=16.0, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=0.3, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=90, KBG=100, FKB=90, BKB=50, Size=6.0, X=0.0, Y=10.0, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=0.3, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=0.0, Angle=366, KBG=100, FKB=100, BKB=50, Size=8.0, X=0.0, Y=16.0, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_none"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            }
        }
        frame(Frame=12)
        if(is_excute){
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07 as u64, GROUND_CLIFF_CHECK_KIND_ALWAYS)
        }
        frame(Frame=21)
        if(is_excute){
            if(_IS_BURST[entry_id] && AttackModule::is_infliction(module_accessor, *COLLISION_KIND_MASK_HIT)){
                SlowModule::set_whole(3, 0)
            }
        }
        frame(Frame=23)
        if(is_excute){
            if(_IS_BURST[entry_id]){
                AttackModule::clear_all()
                WorkModule::on_flag(Flag=FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT)
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=75, KBG=110, FKB=0, BKB=55, Size=8.0, X=0.0, Y=16.0, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_BAT, Type=ATTACK_REGION_PUNCH)
            }
        }
        frame(Frame=24)
        if(is_excute){
            if(_IS_RAGE[entry_id] == false){
                AttackModule::clear_all()
            }
            if(_IS_RAGE[entry_id]) {
                HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_NORMAL), 0)
                rust{
                    _IS_BURST[entry_id] = false;
                    if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
                        _IS_RAGE[entry_id] = false;
                        _HIT_COUNT[entry_id] = 0;
                        SlowModule::clear_whole(module_accessor);
                    }
                }
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "special_air_hi",
    animcmd = "game_specialairhi")]
pub fn swole_luigi_special_air_hi(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!({
        if(is_excute){
            if(_IS_RAGE[entry_id]) {
                HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_XLU), 0)
            }
            rust{
                _COUNTING[entry_id] = 0;
            }
        }
        frame(Frame=6)
        if(is_excute){
            if(_IS_RAGE[entry_id] == false) {
                WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR)
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=20.0, Angle=90, KBG=80, FKB=0, BKB=40, Size=2.7, X=0.0, Y=6.0, Z=5.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
            }
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS)
            if(_HIT_COUNT[entry_id] >= 11) {
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=90, KBG=100, FKB=100, BKB=50, Size=6.0, X=0.0, Y=4.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
                rust{
                    _IS_BURST[entry_id] = true;
                }
            }
        }
        wait(Frames=1)
        if(is_excute){
            SA_SET(State=SITUATION_KIND_AIR)
            if(_IS_BURST[entry_id]) {
                AttackModule::clear_all()
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=90, KBG=100, FKB=100, BKB=50, Size=6.0, X=0.0, Y=25.0, Z=5.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=90, KBG=100, FKB=160, BKB=50, Size=8.0, X=0.0, Y=18.0, Z=4.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=0.0, Angle=366, KBG=100, FKB=100, BKB=50, Size=8.0, X=0.0, Y=25.0, Z=5.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_none"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            }
        }
        frame(Frame=10)
        if(is_excute){
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07 as u64, GROUND_CLIFF_CHECK_KIND_ALWAYS)
            if(_IS_BURST[entry_id]) {
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=90, KBG=100, FKB=80, BKB=50, Size=6.0, X=0.0, Y=34.0, Z=5.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=90, KBG=100, FKB=100, BKB=50, Size=8.0, X=0.0, Y=27.0, Z=4.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=0.0, Angle=366, KBG=100, FKB=100, BKB=50, Size=8.0, X=0.0, Y=34.0, Z=5.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_none"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            }
        }
        frame(Frame=11)
        if(is_excute){
            if(_IS_BURST[entry_id]) {
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=90, KBG=100, FKB=60, BKB=50, Size=6.0, X=0.0, Y=20.0, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=90, KBG=100, FKB=80, BKB=50, Size=6.0, X=0.0, Y=13.0, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=0.0, Angle=366, KBG=100, FKB=100, BKB=50, Size=8.0, X=0.0, Y=13.0, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_none"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            }
        }
        frame(Frame=12)
        if(is_excute){
            if(_IS_BURST[entry_id]) {
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=366, KBG=100, FKB=60, BKB=50, Size=6.0, X=0.0, Y=17.5, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=90, KBG=100, FKB=80, BKB=50, Size=6.0, X=0.0, Y=10.5, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=0.0, Angle=366, KBG=100, FKB=100, BKB=50, Size=8.0, X=0.0, Y=17.5, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_none"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            }
        }
        frame(Frame=21)
        if(is_excute){
            if(_IS_BURST[entry_id] && AttackModule::is_infliction(module_accessor, *COLLISION_KIND_MASK_HIT)){
                SlowModule::set_whole(3, 0)
            }
        }
        frame(Frame=23)
        if(is_excute){
            if(_IS_BURST[entry_id]){
                AttackModule::clear_all()
                WorkModule::on_flag(Flag=FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT)
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=75, KBG=92, FKB=0, BKB=60, Size=8.0, X=0.0, Y=16.0, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_BAT, Type=ATTACK_REGION_PUNCH)
            }
        }
        frame(Frame=24)
        if(is_excute){
            if(_IS_RAGE[entry_id] == false){
                AttackModule::clear_all()
            }
            if(_IS_RAGE[entry_id]) {
                HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_NORMAL), 0)
                rust{
                    _IS_BURST[entry_id] = false;
                    if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
                        _IS_RAGE[entry_id] = false;
                        _HIT_COUNT[entry_id] = 0;
                        SlowModule::clear_whole(module_accessor);
                    }
                }
            }
        }
    });
}

pub fn install() {
    acmd::add_hooks!(
        //swole_luigi_attack_11,
        //swole_luigi_attack_12,
        swole_luigi_attack_13,
        swole_luigi_attack_dash,
        swole_luigi_attack_s3,
        swole_luigi_attack_s3_hi,
        swole_luigi_attack_s3_lw,
        swole_luigi_attack_hi3,
        swole_luigi_attack_lw3,
        swole_luigi_attack_s4hi,
        swole_luigi_attack_s4,
        swole_luigi_attack_s4lw,
        //swole_luigi_attack_hi4,
        swole_luigi_attack_lw4,
        //swole_luigi_attack_air_n,
        swole_luigi_attack_air_f,
        swole_luigi_attack_air_b,
        //swole_luigi_attack_air_hi,
        swole_luigi_attack_air_lw,
        swole_luigi_final_start,
        swole_luigi_final_startl,
        swole_luigi_final_start_air,
        swole_luigi_final_start_airl,
        swole_luigi_special_hi,
        swole_luigi_special_air_hi
    );
}
