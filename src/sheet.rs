use crate::{templates::*, user::*};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum SheetField {
    BaseStrength = 0,
    Strength = 1,
    StrengthMod = 2,
    BaseDexterity = 3,
    Dexterity = 4,
    DexterityMod = 5,
    BaseWisdom = 6,
    Wisdom = 7,
    WisdomMod = 8,
    BaseCharisma = 9,
    Charisma = 10,
    CharismaMod = 11,
    BaseConstitution = 12,
    Constitution = 13,
    ConstitutionMod = 14,
    BaseIntelligence = 15,
    Intelligence = 16,
    IntelligenceMod = 17,
    BAB = 18,
    Attack = 19,
    AC = 20,
}

impl SheetField {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "base_strength" => Some(Self::BaseStrength),
            "strength" => Some(Self::Strength),
            "strength_modifier" => Some(Self::StrengthMod),
            "base_dexterity" => Some(Self::BaseDexterity),
            "dexterity" => Some(Self::Dexterity),
            "dexterity_modifier" => Some(Self::DexterityMod),
            "base_wisdom" => Some(Self::BaseWisdom),
            "wisdom" => Some(Self::Wisdom),
            "wisdom_modifier" => Some(Self::WisdomMod),
            "base_charisma" => Some(Self::BaseCharisma),
            "charisma" => Some(Self::Charisma),
            "charisma_modifier" => Some(Self::CharismaMod),
            "base_constitution" => Some(Self::BaseConstitution),
            "constitution" => Some(Self::Constitution),
            "constitution_modifier" => Some(Self::ConstitutionMod),
            "base_intelligence" => Some(Self::BaseIntelligence),
            "intelligence" => Some(Self::Intelligence),
            "intelligence_modifier" => Some(Self::IntelligenceMod),
            "bab" => Some(Self::BAB),
            "attack" => Some(Self::Attack),
            "ac" => Some(Self::AC),
            _ => None,
        }
    }
}

pub const SHEET_NUM_STATS: usize = 21;

static SHEET_STR_MAP: [&'static str; SHEET_NUM_STATS] = [
    "base_strength",
    "strength",
    "strength_modifier",
    "base_dexterity",
    "dexterity",
    "dexterity_modifier",
    "base_wisdom",
    "wisdom",
    "wisdom_modifier",
    "base_charisma",
    "charisma",
    "charisma_modifier",
    "base_constitution",
    "constitution",
    "constitution_modifier",
    "base_intelligence",
    "intelligence",
    "intelligence_modifier",
    "bab",
    "attack",
    "ac",
];

static SHEET_DEP_MAP: [&'static [SheetField]; SHEET_NUM_STATS] = [
    &[SheetField::Strength],
    &[SheetField::StrengthMod],
    &[SheetField::Attack],
    &[SheetField::Dexterity],
    &[SheetField::DexterityMod],
    &[SheetField::AC],
    &[SheetField::Wisdom],
    &[SheetField::WisdomMod],
    &[],
    &[SheetField::Charisma],
    &[SheetField::CharismaMod],
    &[],
    &[SheetField::Constitution],
    &[SheetField::ConstitutionMod],
    &[],
    &[SheetField::Intelligence],
    &[SheetField::IntelligenceMod],
    &[],
    &[SheetField::Attack],
    &[],
    &[],
];

static SHEET_CALC_MAP: [fn(&[i32]) -> i32; SHEET_NUM_STATS] = [
    |vals| vals[SheetField::BaseStrength as usize],
    |vals| vals[SheetField::BaseStrength as usize],
    |vals| vals[SheetField::Strength as usize] / 2 - 5,
    |vals| vals[SheetField::BaseDexterity as usize],
    |vals| vals[SheetField::BaseDexterity as usize],
    |vals| vals[SheetField::Dexterity as usize] / 2 - 5,
    |vals| vals[SheetField::BaseWisdom as usize],
    |vals| vals[SheetField::BaseWisdom as usize],
    |vals| vals[SheetField::Wisdom as usize] / 2 - 5,
    |vals| vals[SheetField::BaseCharisma as usize],
    |vals| vals[SheetField::BaseCharisma as usize],
    |vals| vals[SheetField::Charisma as usize] / 2 - 5,
    |vals| vals[SheetField::BaseConstitution as usize],
    |vals| vals[SheetField::BaseConstitution as usize],
    |vals| vals[SheetField::Constitution as usize] / 2 - 5,
    |vals| vals[SheetField::BaseIntelligence as usize],
    |vals| vals[SheetField::BaseIntelligence as usize],
    |vals| vals[SheetField::Intelligence as usize] / 2 - 5,
    |vals| vals[SheetField::BAB as usize],
    |vals| vals[SheetField::BAB as usize] + vals[SheetField::StrengthMod as usize],
    |vals| vals[SheetField::DexterityMod as usize] + 10,
];

pub fn calc_route(val_map: &mut [i32], field: SheetField, new_val: i32) -> Vec<StatField> {
    let mut updated_fields = Vec::new();

    val_map[field as usize] = new_val;
    let mut to_update_a = Vec::from(SHEET_DEP_MAP[field as usize]);
    let mut to_update_b = vec![];

    while !to_update_a.is_empty() {
        to_update_b.clear();

        for &field in &to_update_a {
            let new_val = SHEET_CALC_MAP[field as usize](&val_map);
            val_map[field as usize] = new_val;
            updated_fields.push(StatField {
                id: SHEET_STR_MAP[field as usize].to_string(),
                value: new_val,
            });
            to_update_b.extend_from_slice(SHEET_DEP_MAP[field as usize]);
        }

        std::mem::swap(&mut to_update_a, &mut to_update_b);
    }

    updated_fields
}
