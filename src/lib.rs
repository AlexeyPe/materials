#![allow(dead_code)]
//! <h2>Materials - is a library of material data</h2>
//!
//! 1 material = 1 struct
//!
//! Macros are used to create structures.<br>
//! Materials support localization.

/// unit for temperature <br>
/// Еденица температуры, >0
type Kelvin = f32;
/// unit for temperature, °C <br>
/// Еденица температуры, °C
type Celsius = f32;
/// Grams per cm³, g/cm³ <br>
/// Грамм на см³, г/см³
type Gsm3 = f32;
/// Kilogram per m³, kg/m³ <br>
/// Килограмм на м³, кг/м³
type KGm3 = f32;
/// MJ/kg
/// МДж/кг
#[allow(non_camel_case_types)]
type MJ_kg = f32;
/// kkal/kg
/// ккал/кг
#[allow(non_camel_case_types)]
type kkal_kg = f32;

macro_rules! new_elem {
    (
        // example_1 - Gold
        $struct_name:ident,
        // example_1 - names: [RU:"Золото", EN:"Gold"]
        // example_2 - names: [EN:"Gold"]
        names: [$($field_name:path : $lang_literal:literal),*],
        // example_1 - element: [1, "H"]
        element: [$number:expr, $symbol:literal],
        // example_1 - dencity_gsm3: [1.1, 1.5]
        dencity_gsm3: [$density_min:expr, $density_max:expr],
        // example_1 - melting_c: [1450.0, 1500.0]
        $(melting_c:[$melting_min_c:expr, $melting_max_c:expr],)?
        // example_1 - ignition_c: [100.0, 110.0]
        $(ignition_c:[$ignition_min:expr, $ignition_max:expr],)?
        // example_1 - burning_c: [500.0, 610.0]
        $(burning_c:[$burning_min:expr, $burning_max:expr],)?
        // example_1 - heat_value_mj: [14.0, 15.5]
        $(heat_value_mj:[$heat_value_min:expr, $heat_value_max:expr],)?
        $(metal:$metal:path,)?
    ) => {
        new_mat!($struct_name, names:[$($field_name : $lang_literal),*]);
        mat_add!($struct_name, dencity_gsm3:[$density_min, $density_max]);
        mat_add!($struct_name, $number, $symbol);
        $(
            mat_add!($struct_name, melting:[$melting_min_c, $melting_max_c]);
        )?
        $(
            mat_add!($struct_name, ignition_c:[$ignition_min, $ignition_max]);
        )?
        $(
            mat_add!($struct_name, burning_c:[$burning_min, $burning_max]);
        )?
        $(
            mat_add!($struct_name, heat_value_mj:[$heat_value_min, $heat_value_max]);
        )?
        $(
            mat_add!($struct_name, metal: $metal);
        )?
    };
}
macro_rules! new_rock {
    (
        // example_1 - Basalt
        $struct_name:ident,
        // example_1 - names: [RU:"Базальт", EN:"Basalt"]
        // example_2 - names: [EN:"Basalt"]
        names: [$($field_name:path : $lang_literal:literal),*],
        // example_1 - group: [Igneous, GroupRock::Igneous]
        // example_2 - group: [Igneous, GroupRock::Igneous, SubgroupIgneous, SubgroupIgneous::Extrusive]
        group: [$group_trait:ident, $group:path $(, $subgroup:ident, $subgroup_val:path)?],
        // example_1 - dencity_gsm3: [1.1, 1.5]
        dencity_gsm3: [$density_min:expr, $density_max:expr],
        // example_1 - melting_c: [1450.0, 1500.0]
        $(melting_c:[$melting_min_c:expr, $melting_max_c:expr],)?
        // example_1 - ignition_c: [100.0, 110.0]
        $(ignition_c:[$ignition_min:expr, $ignition_max:expr],)?
        // example_1 - burning_c: [500.0, 610.0]
        $(burning_c:[$burning_min:expr, $burning_max:expr],)?
        // example_1 - heat_value_mj: [14.0, 15.5]
        $(heat_value_mj:[$heat_value_min:expr, $heat_value_max:expr],)?
    ) => {
        new_mat!($struct_name, names:[$($field_name : $lang_literal),*]);
        mat_add!($struct_name, dencity_gsm3:[$density_min, $density_max]);
        mat_add!($struct_name, $group_trait, $group $(, $subgroup, $subgroup_val)?);
        $(
            mat_add!($struct_name, melting:[$melting_min_c, $melting_max_c]);
        )?
        $(
            mat_add!($struct_name, ignition_c:[$ignition_min, $ignition_max]);
        )?
        $(
            mat_add!($struct_name, burning_c:[$burning_min, $burning_max]);
        )?
        $(
            mat_add!($struct_name, heat_value_mj:[$heat_value_min, $heat_value_max]);
        )?
    };
}
macro_rules! new_alloy {
    (
        // example_1 - Gold
        $struct_name:ident,
        // example_1 - names: [RU:"Золото", EN:"Gold"]
        // example_2 - names: [EN:"Gold"]
        names: [$($field_name:path : $lang_literal:literal),*],
        chemical_composition: [$([$element:ident, $element_min:expr, $element_max:expr],)*],
        // example_1 - dencity_gsm3: [1.1, 1.5]
        dencity_gsm3: [$density_min:expr, $density_max:expr],
        // example_1 - melting_c: [1450.0, 1500.0]
        melting_c:[$melting_min_c:expr, $melting_max_c:expr],
    ) => {
        new_mat!($struct_name, names:[$($field_name : $lang_literal),*]);
        mat_add!($struct_name, dencity_gsm3:[$density_min, $density_max]);
        mat_add!($struct_name, melting:[$melting_min_c, $melting_max_c]);
        mat_add!($struct_name, alloy: {
            chemical_composition: [
                $(
                    [$element, $element_min, $element_max],
                )*
            ],
        });
    };
}
macro_rules! new_mat {
    // new material with names
    ( $struct_name:ident, names: [$($field_name:path : $lang_literal:literal),*]) => {
        pub struct $struct_name;
        
        impl Material for $struct_name {
            fn get_progress_locale_name(&self) -> u16 {
                [$($lang_literal),*].len() as u16
            }
            fn get_name(&self, lang:SLang) -> &'static str {
                #[allow(unreachable_patterns)]
                match lang {
                    $(
                        $field_name => $lang_literal,
                    )*
                    _ => "",
                }
            }
        }
    };

}
macro_rules! mat_add {
    // Add Melting for material
    ($struct_name:ident, melting:[$min_c:expr, $max_c:expr]) => {
        impl Melting for $struct_name {
            fn get_melting_avg_c(&self) -> Celsius {
                ($min_c + $max_c) * 0.5
            }
            fn get_melting_avg_k(&self) -> Kelvin {
                (($min_c + $max_c) * 0.5) + 273.15
            }
        }
    };
    // Add Density for material
    ($struct_name:ident, dencity_gsm3:[$min_gsm3:expr, $max_gsm3:expr]) => {
        impl Density for $struct_name {
            fn get_density_avg_gcm3(&self) -> Gsm3 {
                ($min_gsm3 + $max_gsm3) * 0.5
            }
            fn get_density_avg_kgm3(&self) -> KGm3 {
                (($min_gsm3 + $max_gsm3) * 0.5) * 1000.0
            }
        }
    };
    // Add Rock for material with subgroup
    ($struct_name:ident, $group_trait:ident, $group:path, $subgroup:ident, $subgroup_val:path) => {
        impl Rock for $struct_name {
            fn get_group(&self) -> GroupRock {
                $group
            }
        }
        impl $group_trait for $struct_name {
            fn get_subgroup(&self) -> $subgroup {
                $subgroup_val
            }
        }
    };
    // Add Rock for material without subgroup
    ($struct_name:ident, $group_trait:ident, $group:path) => {
        impl Rock for $struct_name {
            fn get_group(&self) -> GroupRock {
                $group
            }
        }
    };
    // Add Ignition for material without subgroup
    ($struct_name:ident, ignition_c:[$min_c:expr, $max_c:expr]) => {
        impl Ignition for $struct_name {
            fn get_ignition_avg_c(&self) -> Celsius {
                ($min_c + $max_c) * 0.5
            }
            fn get_ignition_avg_k(&self) -> Kelvin {
                (($min_c + $max_c) * 0.5) + 273.15
            }
        }
    };
    // Add Burning for material without subgroup
    ($struct_name:ident, burning_c:[$min_c:expr, $max_c:expr]) => {
        impl Burning for $struct_name {
            fn get_burning_avg_c(&self) -> Celsius {
                ($min_c + $max_c) * 0.5
            }
            fn get_burning_avg_k(&self) -> Kelvin {
                (($min_c + $max_c) * 0.5) + 273.15
            }
        }
    };
    // Add HeatValue for material
    ($struct_name:ident, heat_value_mj:[$min_mj:expr, $max_mj:expr]) => {
        impl HeatValue for $struct_name {
            fn get_heat_value_avg_kkal(&self) -> kkal_kg {
                ($min_mj + $max_mj) * 0.5
            }
            fn get_heat_value_avg_mj(&self) -> MJ_kg {
                (($min_mj + $max_mj) * 0.5) * 238.8458966275
            }
        }
    };
    // Add Element for material
    ($struct_name:ident, $number:expr, $symbol:literal) => {
        impl Element for $struct_name {
            fn get_symbol(&self) -> &'static str {$symbol}
            fn get_number(&self) -> u8 {$number}
        }
    };
    // Add Metal for material
    ($struct_name:ident, metal:$metal:path) => {
        impl Metal for $struct_name {
            const GROUP_METAL: GroupMetal = $metal; 
        }
    };
    // Add Alloy for material
    ($struct_name:ident, alloy:{
        chemical_composition: [$([$element:ident, $element_min:expr, $element_max:expr],)*],
    }) => {
        impl Alloy for $struct_name {
            const CHEMICAL_COMPOSITION: &'static [(&'static dyn Element, f32, f32)] = &[
                $(
                    (&$element, $element_min, $element_max),
                )*
            ];
        }
    }
}

/// Number of material structures
pub const COUNT_MATERIALS:u32 = ALL_MATERIALS.len() as u32;
/// SLang size
pub const COUNT_SUPPORTED_LANGUAGES:u16 = 2;
pub const ALL_MATERIALS: &[&dyn Material] = &[
    &Basalt,
    &Granite,
    &Obsidian,
    &BrownCoal,
    &Eclogite,

    &Hydrogen,
    &Gold,
    &Iron,
    &Aluminium,
    &Copper,
    &Zinc,
    &Magnesium,
    &Manganese,

    &Brass,
    &Dural,
];

/// SLang = Supported Language. ISO 639-1
pub enum SLang {
    RU = 570,
    EN = 45,
}

#[derive(Debug)]
/// Rock groups by formation
pub enum GroupRock {
    /// Result of compaction of sediment on the bottom of rivers, seas and oceans, and the destruction/weathering of various rocks on land.<br>
    /// Осадочная порода. Результат уплотнения осадка на дне рек, морей и океанов, а также разрушения/выветривания различных горных пород на суше.
    Sedimentary,
    /// It is formed from magmatic melt during its cooling and solidification.<br>
    /// Магматическая порода. Образуется из магматического расплава при её охлаждении и затвердевании.<br>
    Igneous,
    /// Formed from other rocks under the influence of temperature, pressure and fluids.<br>
    /// Метаморфическая порода. Образуется из других пород под действием температуры, давления и флюидов.
    Metamorphic,
}

pub enum SubgroupIgneous {
    /// Intrusive/Plutonic -Formed as a result of the solidification of magma in the depths of the earth.<br>
    /// Интрузивные/Плутонические - образовались в результате застывания магмы в глубинах земли.
    Intrusive,
    /// Extrusive/Effusive/Volcanic - Formed on the surface as a result of the outpouring of lava.<br>
    /// Экструзивные/Эффузивные/Вулканические - образовались на поверхности в результате излияния лав.
    Extrusive,
}

pub enum SubgroupSedimentary {
    /// Formed as a result of the vital activity of animals and plants.<br>
    /// Биогенные - образуются в результате жизнедеятельности животных и растительных организмов.
    Biogenic,
    /// Formed by chemical precipitation from aqueous solutions or by evaporation of water.<br>
    /// Хемогенные - образуются из химического осаждения из водных растворов или при испарении воды.
    Chemogenic,
    /// Formed as a result of weathering processes, volcanism, tectonic and man-made activity.<br>
    /// Обломочные/Терригенные - образуются как результат процессов выветривания, вулканизма, тектонической и техногенной активности.
    Clastic,
}

pub enum GroupMetal {
    // Щелочные
    Alkali,
    // Щёлочноземельные
    AlkalineEarth,
    // Переходные
    Transition,
    // Лёгкие
    PostTransition,
}

pub trait Material {
    /// Возвращает прогресс локализации. <br>
    /// Увеличивается на 1 за каждое переведенное SLang. <br>
    /// В идеале должно быть равно количеству SLang.
    fn get_progress_locale_name(&self) -> u16 {0}
    /// Возвращает локализованное название материала
    fn get_name(&self, _lang:SLang) -> &'static str {""}
}

/// Density, g/cm3 and kg/m3
pub trait Density {
    fn get_density_avg_gcm3(&self) -> Gsm3 {0.0}
    fn get_density_avg_kgm3(&self) -> KGm3 {0.0}
}

/// Melting point, Kelvin and Celsius °C
pub trait Melting {
    fn get_melting_avg_c(&self) -> Celsius;
    fn get_melting_avg_k(&self) -> Kelvin;
}

pub trait Rock {
    fn get_group(&self) -> GroupRock;
}

/// Sedimentary rock
pub trait Sedimentary {
    fn get_subgroup(&self) -> SubgroupSedimentary;
}
/// Igneous rock
pub trait Igneous {
    fn get_subgroup(&self) -> SubgroupIgneous;
}
/// Metamorphic rock
pub trait Metamorphic {

}
/// Ignition temperature
pub trait Ignition {
    fn get_ignition_avg_c(&self) -> Celsius {0.0}
    fn get_ignition_avg_k(&self) -> Kelvin {0.0}
}
/// Burning, max temperature
pub trait Burning {
    fn get_burning_avg_c(&self) -> Celsius {0.0}
    fn get_burning_avg_k(&self) -> Kelvin {0.0}
}
pub trait HeatValue {
    fn get_heat_value_avg_kkal(&self) -> kkal_kg {0.0}
    fn get_heat_value_avg_mj(&self) -> MJ_kg {0.0}
}
/// This is an element of the periodic table of chemical elements.
pub trait Element {
    fn get_symbol(&self) -> &'static str {""}
    fn get_number(&self) -> u8;
}

pub trait Metal {
    const GROUP_METAL: GroupMetal;
}

pub trait Alloy {
    // fn get_chemical_composition() -> &'static [(&'static dyn Element, f32, f32)];
    const CHEMICAL_COMPOSITION: &'static [(&'static dyn Element, f32, f32)];
}

// impl Alloy for Brass {
//     // fn get_chemical_composition() -> &'static [(&'static dyn Element, f32, f32)] {
//     //     &[(&Copper, 50.0, 90.0)]
//     // }
//     const CHEMICAL_COMPOSITION: &'static [(&'static dyn Element, f32, f32)] = &[(&Copper, 50.0, 90.0)];
// }

new_rock!( Basalt,
    names: [SLang::RU:"Базальт", SLang::EN:"Basalt"],
    group: [Igneous, GroupRock::Igneous, SubgroupIgneous, SubgroupIgneous::Extrusive],
    dencity_gsm3:[2.6, 3.1],
    melting_c: [1100.0, 1250.0],
);
new_rock!( Granite,
    names: [SLang::RU:"Гранит", SLang::EN:"Granite"],
    group: [Igneous, GroupRock::Igneous, SubgroupIgneous, SubgroupIgneous::Intrusive],
    dencity_gsm3:[2.6, 3.0],
    melting_c: [1215.0, 1260.0],
);
new_rock!( Obsidian,
    names:[SLang::RU:"Обсидиан", SLang::EN:"Obsidian"],
    group: [Igneous, GroupRock::Igneous, SubgroupIgneous, SubgroupIgneous::Extrusive],
    dencity_gsm3:[2.5, 2.6],
    melting_c: [1200.0, 1500.0],
);
new_rock!( BrownCoal,
    names: [SLang::RU:"Бурый уголь", SLang::EN:"Brown Coal"],
    group: [Sedimentary, GroupRock::Sedimentary, SubgroupSedimentary, SubgroupSedimentary::Biogenic],
    dencity_gsm3:[1.2, 1.5],
    ignition_c: [250.0, 250.0],
    burning_c: [1900.0, 1900.0],
    heat_value_mj: [14.0, 16.0],
);
new_rock!( Eclogite,
    names: [SLang::RU:"Эклогит", SLang::EN:"Eclogite"],
    group: [Metamorphic, GroupRock::Metamorphic],
    dencity_gsm3: [3.3, 3.7],
);

new_elem!( Gold,
    names: [SLang::RU:"Золото", SLang::EN:"Gold"],
    element: [79, "Au"],
    dencity_gsm3: [19.3, 19.32],
    melting_c: [1064.18, 1064.18],
    metal: GroupMetal::Transition,
);
new_elem!( Iron,
    names: [SLang::RU:"Железо", SLang::EN:"Iron"],
    element: [26, "Fe"],
    dencity_gsm3: [7.874, 7.874],
    melting_c: [1538.85, 1538.85],
    metal: GroupMetal::Transition,
);
new_elem!( Aluminium,
    names: [SLang::RU:"Алюминий", SLang::EN:"Aluminium"],
    element: [13, "Al"],
    dencity_gsm3: [2.6989, 2.6989],
    melting_c: [933.5, 933.5],
    metal: GroupMetal::PostTransition,
);
new_elem!( Copper,
    names: [SLang::RU:"Медь", SLang::EN:"Copper"],
    element: [29, "Cu"],
    dencity_gsm3: [8.92, 8.92],
    melting_c: [1083.4, 1083.4],
    metal: GroupMetal::Transition,
);
new_elem!( Zinc,
    names: [SLang::RU:"Цинк", SLang::EN:"Zinc"],
    element: [30, "Zn"],
    dencity_gsm3: [7.13, 7.13],
    melting_c: [419.55, 419.55],
    metal: GroupMetal::Transition,
);
new_elem!( Magnesium,
    names: [SLang::RU:"Магний", SLang::EN:"Magnesium"],
    element: [12, "Mg"],
    dencity_gsm3: [1.738, 1.738],
    melting_c: [650.0, 650.0],
    metal: GroupMetal::AlkalineEarth,
);
new_elem!( Manganese,
    names: [SLang::RU:"Марганец", SLang::EN:"Manganese"],
    element: [25, "Mn"],
    dencity_gsm3: [7.21, 7.21],
    melting_c: [1243.0, 1243.0],
    metal: GroupMetal::Transition,
);
new_elem!( Hydrogen,
    names: [SLang::RU:"Водород", SLang::EN:"Hydrogen"],
    element: [1, "H"],
    dencity_gsm3: [0.0000899, 0.0000899],
    ignition_c: [510.0, 590.0],
    burning_c: [2600.0, 2900.0],
    heat_value_mj: [141.865, 141.865],
);

new_alloy!( Brass,
    names: [SLang::RU:"Латунь", SLang::EN:"Brass"],
    chemical_composition: [
        [Copper, 50.0, 90.0],
        [Zinc, 10.0, 50.0],
    ],
    dencity_gsm3: [8.5, 8.7],
    melting_c: [900.0, 950.0],
);
new_alloy!( Dural,
    names: [SLang::RU:"Дюраль", SLang::EN:"Dural"],
    chemical_composition: [
        [Aluminium, 91.0, 95.0],
        [Copper, 3.8, 4.9],
        [Magnesium, 1.2, 1.8],
        [Manganese, 0.3, 0.9],
    ],
    dencity_gsm3: [2.79, 2.77],
    melting_c: [515.0, 640.0],
);

#[cfg(test)]
mod localization_tests {
    use super::*;
    #[test]
    fn localization_name() {
        println!("COUNT_SUPPORTED_LANGUAGES:{}",COUNT_SUPPORTED_LANGUAGES);
        println!("COUNT_MATERIALS:{}",COUNT_MATERIALS);
        let mut need_translate:u32 = 0;
        for material in ALL_MATERIALS {
            let mut add: &str = "✔";
            if material.get_progress_locale_name() < COUNT_SUPPORTED_LANGUAGES {
                add = "🔥";
                need_translate += (COUNT_SUPPORTED_LANGUAGES - material.get_progress_locale_name()) as u32;
            }
            println!(
                "RU:{}, EN:{}, progress: {}/{} {}",
                material.get_name(SLang::RU),
                material.get_name(SLang::EN),
                material.get_progress_locale_name(),
                COUNT_SUPPORTED_LANGUAGES,
                add,
            );
        }
        assert!(need_translate == 0, "Need translate {} materials", need_translate)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        println!("COUNT_MATERIALS: {}",COUNT_MATERIALS);
        println!(
            "{} density: {}g/cm³ melting: {}°C",
            Obsidian.get_name(SLang::EN),
            Obsidian.get_density_avg_gcm3(),
            Obsidian.get_melting_avg_c()
        );
        println!(
            "{} number: {}, density: {}g/cm³ melting: {}°C",
            Gold.get_name(SLang::EN),
            Gold.get_number(),
            Gold.get_density_avg_gcm3(),
            Gold.get_melting_avg_c(),
        );
        println!(
            "{} number: {}, density: {}g/cm³, ignition: {}°C, burning_c: {}°C",
            Hydrogen.get_name(SLang::EN),
            Hydrogen.get_number(),
            Hydrogen.get_density_avg_gcm3(),
            Hydrogen.get_ignition_avg_c(),
            Hydrogen.get_burning_avg_c(),
        );
    }
}