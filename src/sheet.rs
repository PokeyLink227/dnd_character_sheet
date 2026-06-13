use crate::{templates::*, user::*};

use serde::{Deserialize, Serialize};

use regex::Regex;

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
    acrobatics,
    appraise,
    bluff,
    climb,
    craft,
    diplomacy,
    disabledevice,
    disguise,
    escapeArtist,
    fly,
    handleAnimal,
    heal,
    knowledge_arcana,
    knowledge_Dungeoneering,
    knowledge_Engineering,
    knowledge_Geography,
    knowledge_history,
    knowledge_local,
    knowledge_nature,
    knowledge_nobility,
    knowledge_planes,
    knowledge_religion,
    linguistics,
    perception,
    perform_1,
    perform_2,
    profession_1,
    profession_2,
    ride,
    sense_motive,
    sleight_of_hand,
    spellcraft,
    stealth,
    survival,
    swim,
    use_magic_device,
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
            "acrobatics" => Some(Self::acrobatics),
            "appraise" => Some(Self::appraise),
            "bluff" => Some(Self::bluff),
            "climb" => Some(Self::climb),
            "craft" => Some(Self::craft),
            "diplomacy" => Some(Self::diplomacy),
            "disabledevice" => Some(Self::disabledevice),
            "disguise" => Some(Self::disguise),
            "escapeArtist" => Some(Self::escapeArtist),
            "fly" => Some(Self::fly),
            "handleAnimal" => Some(Self::handleAnimal),
            "heal" => Some(Self::heal),
            "knowledge_arcana" => Some(Self::knowledge_arcana),
            "knowledge_Dungeoneering" => Some(Self::knowledge_Dungeoneering),
            "knowledge_Engineering" => Some(Self::knowledge_Engineering),
            "knowledge_Geography" => Some(Self::knowledge_Geography),
            "knowledge_history" => Some(Self::knowledge_history),
            "knowledge_local" => Some(Self::knowledge_local),
            "knowledge_nature" => Some(Self::knowledge_nature),
            "knowledge_nobility" => Some(Self::knowledge_nobility),
            "knowledge_planes" => Some(Self::knowledge_planes),
            "knowledge_religion" => Some(Self::knowledge_religion),
            "linguistics" => Some(Self::linguistics),
            "perception" => Some(Self::perception),
            "perform_1" => Some(Self::perform_1),
            "perform_2" => Some(Self::perform_1),
            "profession_1" => Some(Self::profession_2),
            "profession_2" => Some(Self::profession_2),
            "ride" => Some(Self::ride),
            "sense_motive" => Some(Self::sense_motive),
            "sleight_of_hand" => Some(Self::sleight_of_hand),
            "spellcraft" => Some(Self::spellcraft),
            "stealth" => Some(Self::stealth),
            "survival" => Some(Self::survival),
            "swim" => Some(Self::swim),
            "use_magic_device" => Some(Self::use_magic_device),
            _ => None,
        }
    }
}

pub const SHEET_NUM_STATS: usize = 57;

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
    "acrobatics",
    "appraise",
    "bluff",
    "climb",
    "craft",
    "diplomacy",
    "disabledevice",
    "disguise",
    "escapeArtist",
    "fly",
    "handleAnimal",
    "heal",
    "knowledge_arcana",
    "knowledge_Dungeoneering",
    "knowledge_Engineering",
    "knowledge_Geography",
    "knowledge_history",
    "knowledge_local",
    "knowledge_nature",
    "knowledge_nobility",
    "knowledge_planes",
    "knowledge_religion",
    "linguistics",
    "perception",
    "perform_1",
    "perform_2",
    "profession_1",
    "profession_2",
    "ride",
    "sense_motive",
    "sleight_of_hand",
    "spellcraft",
    "stealth",
    "survival",
    "swim",
    "use_magic_device",
];

static SHEET_DEP_MAP: [&'static [SheetField]; SHEET_NUM_STATS] = [
    &[SheetField::Strength],
    &[SheetField::StrengthMod],
    &[SheetField::Attack, SheetField::climb, SheetField::swim],
    &[SheetField::Dexterity],
    &[SheetField::DexterityMod],
    &[
        SheetField::AC,
        SheetField::acrobatics,
        SheetField::disabledevice,
        SheetField::escapeArtist,
        SheetField::fly,
        SheetField::ride,
        SheetField::sleight_of_hand,
        SheetField::stealth,
    ],
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
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
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
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
    |vals| vals[SheetField::DexterityMod as usize] + 10,
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

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(untagged)]
pub enum Formula {
    #[default]
    #[serde(rename = "static")]
    Static,
    Single(String),
    Multiple(Vec<String>),
}

impl Formula {
    fn get_deps(&self) -> Vec<String> {
        match self {
            Self::Static => vec![],
            Self::Single(s) => {
                let re = Regex::new(r"[a-z_]+\[(?P<stat_mod>[a-z_]+)\]|(?P<stat>[a-z_]+)").unwrap();
                let mut deps = vec![];
                re.captures_iter(s)
                    .map(|c| c.name("stat"))
                    .for_each(|mat| {

                        if let Some(stat) = mat {
                            deps.push(stat.as_str().to_string());
                        }
                    });
                deps
            }
            Self::Multiple(_) => vec![],
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Stat {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub formula: Formula,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Class {
    pub uuid: String,
    pub id: String,
    pub name: String,
    pub stats: Vec<Stat>,
    pub bab: f32,
}

mod test {
    use super::*;

    #[test]
    fn parse_base_class() {
        let f = std::fs::read_to_string("./data/classes/base.yaml").unwrap();
        println!("{:?}", f);

        let mut class: Class = yaml_serde::from_str(&f).unwrap();
        println!("{:?}", class);

        assert_eq!(
            class.stats[2].formula.get_deps(),
            vec!["temp_strength", "base_strength"]
        );

        panic!();
    }
}
