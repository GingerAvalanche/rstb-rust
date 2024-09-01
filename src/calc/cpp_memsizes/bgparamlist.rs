use std::mem::size_of;

use phf::{phf_map, Map};
use roead::aamp::ParameterIO;

use super::cpp_classes::GParamList::*;
use crate::Endian;

const CLASS_SIZE_WIIU: usize = size_of::<GParamList<u32>>();
const CLASS_SIZE_NX: usize = size_of::<GParamList<u64>>();

const OVERHEAD_WIIU: usize = 0xEC;
const OVERHEAD_NX: usize = 0x1C0;

static OBJ_SIZES_WIIU: Map<&'static str, usize> = phf_map! {
    "AirWall" => size_of::<GParamListObjectAirWall<u32>>(),
    "AnimalFollowOffset" => size_of::<GParamListObjectAnimalFollowOffset<u32>>(),
    "AnimalUnit" => size_of::<GParamListObjectAnimalUnit<u32>>(),
    "Armor" => size_of::<GParamListObjectArmor<u32>>(),
    "ArmorEffect" => size_of::<GParamListObjectArmorEffect<u32>>(),
    "ArmorHead" => size_of::<GParamListObjectArmorHead<u32>>(),
    "ArmorUpper" => size_of::<GParamListObjectArmorUpper<u32>>(),
    "Arrow" => size_of::<GParamListObjectArrow<u32>>(),
    "Attack" => size_of::<GParamListObjectAttack<u32>>(),
    "AttackInterval" => size_of::<GParamListObjectAttackInterval<u32>>(),
    "AutoGen" => size_of::<GParamListObjectAutoGen<u32>>(),
    "Beam" => size_of::<GParamListObjectBeam<u32>>(),
    "BindActor" => size_of::<GParamListObjectBindActor<u32>>(),
    "BindBone" => size_of::<GParamListObjectBindBone<u32>>(),
    "Bow" => size_of::<GParamListObjectBow<u32>>(),
    "Bullet" => size_of::<GParamListObjectBullet<u32>>(),
    "Camera" => size_of::<GParamListObjectCamera<u32>>(),
    "ChemicalType" => size_of::<GParamListObjectChemicalType<u32>>(),
    "ClothReaction" => size_of::<GParamListObjectClothReaction<u32>>(),
    "CookSpice" => size_of::<GParamListObjectCookSpice<u32>>(),
    "CureItem" => size_of::<GParamListObjectCureItem<u32>>(),
    "EatTarget" => size_of::<GParamListObjectEatTarget<u32>>(),
    "Enemy" => size_of::<GParamListObjectEnemy<u32>>(),
    "EnemyLevel" => size_of::<GParamListObjectEnemyLevel<u32>>(),
    "EnemyRace" => size_of::<GParamListObjectEnemyRace<u32>>(),
    "EnemyShown" => size_of::<GParamListObjectEnemyShown<u32>>(),
    "Event" => size_of::<GParamListObjectEvent<u32>>(),
    "ExtendedEntity" => size_of::<GParamListObjectExtendedEntity<u32>>(),
    "Fish" => size_of::<GParamListObjectFish<u32>>(),
    "GelEnemy" => size_of::<GParamListObjectGelEnemy<u32>>(),
    "General" => size_of::<GParamListObjectGeneral<u32>>(),
    "GiantArmor" => size_of::<GParamListObjectGiantArmor<u32>>(),
    "GiantArmorSlot" => size_of::<GParamListObjectGiantArmorSlot<u32>>(),
    "Global" => size_of::<GParamListObjectGlobal<u32>>(),
    "Golem" => size_of::<GParamListObjectGolem<u32>>(),
    "GolemIK" => size_of::<GParamListObjectGolemIK<u32>>(),
    "Grab" => size_of::<GParamListObjectGrab<u32>>(),
    "Guardian" => size_of::<GParamListObjectGuardian<u32>>(),
    "GuardianMini" => size_of::<GParamListObjectGuardianMini<u32>>(),
    "GuardianMiniWeapon" => size_of::<GParamListObjectGuardianMiniWeapon<u32>>(),
    "Horse" => size_of::<GParamListObjectHorse<u32>>(),
    "HorseCreator" => size_of::<GParamListObjectHorseCreator<u32>>(),
    "HorseObject" => size_of::<GParamListObjectHorseObject<u32>>(),
    "HorseRider" => size_of::<GParamListObjectHorseRider<u32>>(),
    "HorseTargetedInfo" => size_of::<GParamListObjectHorseTargetedInfo<u32>>(),
    "HorseUnit" => size_of::<GParamListObjectHorseUnit<u32>>(),
    "Insect" => size_of::<GParamListObjectInsect<u32>>(),
    "Item" => size_of::<GParamListObjectItem<u32>>(),
    "LargeSword" => size_of::<GParamListObjectLargeSword<u32>>(),
    "Liftable" => size_of::<GParamListObjectLiftable<u32>>(),
    "LumberjackTree" => size_of::<GParamListObjectLumberjackTree<u32>>(),
    "MasterSword" => size_of::<GParamListObjectMasterSword<u32>>(),
    "MonsterShop" => size_of::<GParamListObjectMonsterShop<u32>>(),
    "Motorcycle" => size_of::<GParamListObjectMotorcycle<u32>>(),
    "Nest" => size_of::<GParamListObjectNest<u32>>(),
    "Npc" => size_of::<GParamListObjectNpc<u32>>(),
    "NpcEquipment" => size_of::<GParamListObjectNpcEquipment<u32>>(),
    "PictureBook" => size_of::<GParamListObjectPictureBook<u32>>(),
    "Player" => size_of::<GParamListObjectPlayer<u32>>(),
    "Prey" => size_of::<GParamListObjectPrey<u32>>(),
    "Rod" => size_of::<GParamListObjectRod<u32>>(),
    "Rope" => size_of::<GParamListObjectRope<u32>>(),
    "Rupee" => size_of::<GParamListObjectRupee<u32>>(),
    "Sandworm" => size_of::<GParamListObjectSandworm<u32>>(),
    "SeriesArmor" => size_of::<GParamListObjectSeriesArmor<u32>>(),
    "ShiekerStone" => size_of::<GParamListObjectShiekerStone<u32>>(),
    "Shield" => size_of::<GParamListObjectShield<u32>>(),
    "SmallSword" => size_of::<GParamListObjectSmallSword<u32>>(),
    "Spear" => size_of::<GParamListObjectSpear<u32>>(),
    "StalEnemy" => size_of::<GParamListObjectStalEnemy<u32>>(),
    "Swarm" => size_of::<GParamListObjectSwarm<u32>>(),
    "System" => size_of::<GParamListObjectSystem<u32>>(),
    "Traveler" => size_of::<GParamListObjectTraveler<u32>>(),
    "WeaponCommon" => size_of::<GParamListObjectWeaponCommon<u32>>(),
    "WeaponOption" => size_of::<GParamListObjectWeaponOption<u32>>(),
    "WeaponThrow" => size_of::<GParamListObjectWeaponThrow<u32>>(),
    "WizzRobe" => size_of::<GParamListObjectWizzrobe<u32>>(),
    "WolfLink" => size_of::<GParamListObjectWolfLink<u32>>(),
    "Zora" => size_of::<GParamListObjectZora<u32>>(),
};
static OBJ_SIZES_NX: Map<&'static str, usize> = phf_map! {
    "AirWall" => size_of::<GParamListObjectAirWall<u64>>(),
    "AnimalFollowOffset" => size_of::<GParamListObjectAnimalFollowOffset<u64>>(),
    "AnimalUnit" => size_of::<GParamListObjectAnimalUnit<u64>>(),
    "Armor" => size_of::<GParamListObjectArmor<u64>>(),
    "ArmorEffect" => size_of::<GParamListObjectArmorEffect<u64>>(),
    "ArmorHead" => size_of::<GParamListObjectArmorHead<u64>>(),
    "ArmorUpper" => size_of::<GParamListObjectArmorUpper<u64>>(),
    "Arrow" => size_of::<GParamListObjectArrow<u64>>(),
    "Attack" => size_of::<GParamListObjectAttack<u64>>(),
    "AttackInterval" => size_of::<GParamListObjectAttackInterval<u64>>(),
    "AutoGen" => size_of::<GParamListObjectAutoGen<u64>>(),
    "Beam" => size_of::<GParamListObjectBeam<u64>>(),
    "BindActor" => size_of::<GParamListObjectBindActor<u64>>(),
    "BindBone" => size_of::<GParamListObjectBindBone<u64>>(),
    "Bow" => size_of::<GParamListObjectBow<u64>>(),
    "Bullet" => size_of::<GParamListObjectBullet<u64>>(),
    "Camera" => size_of::<GParamListObjectCamera<u64>>(),
    "ChemicalType" => size_of::<GParamListObjectChemicalType<u64>>(),
    "ClothReaction" => size_of::<GParamListObjectClothReaction<u64>>(),
    "CookSpice" => size_of::<GParamListObjectCookSpice<u64>>(),
    "CureItem" => size_of::<GParamListObjectCureItem<u64>>(),
    "EatTarget" => size_of::<GParamListObjectEatTarget<u64>>(),
    "Enemy" => size_of::<GParamListObjectEnemy<u64>>(),
    "EnemyLevel" => size_of::<GParamListObjectEnemyLevel<u64>>(),
    "EnemyRace" => size_of::<GParamListObjectEnemyRace<u64>>(),
    "EnemyShown" => size_of::<GParamListObjectEnemyShown<u64>>(),
    "Event" => size_of::<GParamListObjectEvent<u64>>(),
    "ExtendedEntity" => size_of::<GParamListObjectExtendedEntity<u64>>(),
    "Fish" => size_of::<GParamListObjectFish<u64>>(),
    "GelEnemy" => size_of::<GParamListObjectGelEnemy<u64>>(),
    "General" => size_of::<GParamListObjectGeneral<u64>>(),
    "GiantArmor" => size_of::<GParamListObjectGiantArmor<u64>>(),
    "GiantArmorSlot" => size_of::<GParamListObjectGiantArmorSlot<u64>>(),
    "Global" => size_of::<GParamListObjectGlobal<u64>>(),
    "Golem" => size_of::<GParamListObjectGolem<u64>>(),
    "GolemIK" => size_of::<GParamListObjectGolemIK<u64>>(),
    "Grab" => size_of::<GParamListObjectGrab<u64>>(),
    "Guardian" => size_of::<GParamListObjectGuardian<u64>>(),
    "GuardianMini" => size_of::<GParamListObjectGuardianMini<u64>>(),
    "GuardianMiniWeapon" => size_of::<GParamListObjectGuardianMiniWeapon<u64>>(),
    "Horse" => size_of::<GParamListObjectHorse<u64>>(),
    "HorseCreator" => size_of::<GParamListObjectHorseCreator<u64>>(),
    "HorseObject" => size_of::<GParamListObjectHorseObject<u64>>(),
    "HorseRider" => size_of::<GParamListObjectHorseRider<u64>>(),
    "HorseTargetedInfo" => size_of::<GParamListObjectHorseTargetedInfo<u64>>(),
    "HorseUnit" => size_of::<GParamListObjectHorseUnit<u64>>(),
    "Insect" => size_of::<GParamListObjectInsect<u64>>(),
    "Item" => size_of::<GParamListObjectItem<u64>>(),
    "LargeSword" => size_of::<GParamListObjectLargeSword<u64>>(),
    "Liftable" => size_of::<GParamListObjectLiftable<u64>>(),
    "LumberjackTree" => size_of::<GParamListObjectLumberjackTree<u64>>(),
    "MasterSword" => size_of::<GParamListObjectMasterSword<u64>>(),
    "MonsterShop" => size_of::<GParamListObjectMonsterShop<u64>>(),
    "Motorcycle" => size_of::<GParamListObjectMotorcycle<u64>>(),
    "Nest" => size_of::<GParamListObjectNest<u64>>(),
    "Npc" => size_of::<GParamListObjectNpc<u64>>(),
    "NpcEquipment" => size_of::<GParamListObjectNpcEquipment<u64>>(),
    "PictureBook" => size_of::<GParamListObjectPictureBook<u64>>(),
    "Player" => size_of::<GParamListObjectPlayer<u64>>(),
    "Prey" => size_of::<GParamListObjectPrey<u64>>(),
    "Rod" => size_of::<GParamListObjectRod<u64>>(),
    "Rope" => size_of::<GParamListObjectRope<u64>>(),
    "Rupee" => size_of::<GParamListObjectRupee<u64>>(),
    "Sandworm" => size_of::<GParamListObjectSandworm<u64>>(),
    "SeriesArmor" => size_of::<GParamListObjectSeriesArmor<u64>>(),
    "ShiekerStone" => size_of::<GParamListObjectShiekerStone<u64>>(),
    "Shield" => size_of::<GParamListObjectShield<u64>>(),
    "SmallSword" => size_of::<GParamListObjectSmallSword<u64>>(),
    "Spear" => size_of::<GParamListObjectSpear<u64>>(),
    "StalEnemy" => size_of::<GParamListObjectStalEnemy<u64>>(),
    "Swarm" => size_of::<GParamListObjectSwarm<u64>>(),
    "System" => size_of::<GParamListObjectSystem<u64>>(),
    "Traveler" => size_of::<GParamListObjectTraveler<u64>>(),
    "WeaponCommon" => size_of::<GParamListObjectWeaponCommon<u64>>(),
    "WeaponOption" => size_of::<GParamListObjectWeaponOption<u64>>(),
    "WeaponThrow" => size_of::<GParamListObjectWeaponThrow<u64>>(),
    "WizzRobe" => size_of::<GParamListObjectWizzrobe<u64>>(),
    "WolfLink" => size_of::<GParamListObjectWolfLink<u64>>(),
    "Zora" => size_of::<GParamListObjectZora<u64>>(),
};

pub fn parse_size(bytes: &[u8], endian: Endian) -> Option<u32> {
    let mut total_size = match endian {
        Endian::Big => super::PARSE_CONST_WIIU + CLASS_SIZE_WIIU + OVERHEAD_WIIU,
        Endian::Little => super::PARSE_CONST_NX + CLASS_SIZE_NX + OVERHEAD_NX,
    };

    let a = ParameterIO::from_binary(bytes).ok()?;
    let obj_map: &Map<&'static str, usize> = match endian {
        Endian::Big => &OBJ_SIZES_WIIU,
        Endian::Little => &OBJ_SIZES_NX,
    };
    let (
        iter_size,
        size_t,
    );
    match endian {
        Endian::Big => {
            iter_size = super::ITER_CONST_WIIU;
            size_t = size_of::<u32>();
        }
        Endian::Little => {
            iter_size = super::ITER_CONST_NX;
            size_t = size_of::<u64>();
        }
    };

    //constexpr size_t NumGParamListObjTypes = 1 + 0x4E;
    //mObjects.allocBufferAssert(NumGParamListObjTypes, heap);
    // I dunno what else to say. I guess we just alloc pointers
    // for every damn thing.
    total_size += 0x4F * size_t;

    // This is a big hint, if I'm smart enough to figure out what it's
    // hinting at: iter_size works here, but there *is no iterator* in
    // the parse code. What is the connection, instead? addObj()?
    for (name, size) in (*obj_map).into_iter() {
        if a.param_root.objects.get(*name).is_some() {
            // Don't add size_t here, because the buffer is of pointers,
            // which are trivially destructible
            total_size += iter_size + size;
        }
    }

    Some(total_size as u32)
}
