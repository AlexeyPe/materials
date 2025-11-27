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
/// Grams per cm3, g/cm³ <br>
/// Грамм на см3, г/см³
type Gsm3 = f32;
/// Kilogram per m3, kg/m³ <br>
/// Килограмм на м3, кг/м³
type KGm3 = f32;

macro_rules! new_rock {
    // new rock material with names + group
    (
        // example_1 - Basalt
        $struct_name:ident,
        // example_1 - names: [RU:"Базальт", EN:"Basalt"]
        // example_2 - names: [EN:"Basalt"]
        names: [$($field_name:path : $lang_literal:literal),*],
        // example_1 - group: [Igneous, GroupRock::Igneous]
        // example_2 - group: [Igneous, GroupRock::Igneous, SubgroupIgneous, SubgroupIgneous::Extrusive]
        group: [$group_trait:ident, $group:path $(, $subgroup:ident, $subgroup_val:path)?],
    ) => {
        new_mat!($struct_name, names:[$($field_name : $lang_literal),*]);
        mat_add!($struct_name, $group_trait, $group, $($subgroup, $subgroup_val)?);
    };
    // new rock material with names + group + melting
    (
        $struct_name:ident,
        names: [$($field_name:path : $lang_literal:literal),*],
        group: [$group_trait:ident, $group:path $(, $subgroup:ident, $subgroup_val:path)?],
        melting_c: [$melting_min_c:expr, $melting_max_c:expr],
    ) => {
        new_mat!($struct_name, names:[$($field_name : $lang_literal),*]);
        mat_add!($struct_name, $group_trait, $group, $($subgroup, $subgroup_val)?);
        mat_add!($struct_name, melting_min:$melting_min_c, melting_max:$melting_max_c);
    };
    // new rock material with names + group + melting + density
    (
        $struct_name:ident,
        names: [$($field_name:path : $lang_literal:literal),*],
        group: [$group_trait:ident, $group:path $(, $subgroup:ident, $subgroup_val:path)?],
        melting_c: [$melting_min_c:expr, $melting_max_c:expr],
        dencity_gsm3: [$density_min_gcm3:expr, $density_max_gcm3:expr],
    ) => {
        new_mat!($struct_name, names:[$($field_name : $lang_literal),*]);
        mat_add!($struct_name, $group_trait, $group, $($subgroup, $subgroup_val)?);
        mat_add!($struct_name, melting_min:$melting_min_c, melting_max:$melting_max_c);
        mat_add!($struct_name, density_min:$density_min_gcm3, density_max:$density_max_gcm3);
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
    ($struct_name:ident, melting_min:$melting_min_c:expr, melting_max:$melting_max_c:expr) => {
        impl Melting for $struct_name {
            fn get_melting_avg_c(&self) -> Celsius {
                ($melting_min_c + $melting_max_c) * 0.5
            }
            fn get_melting_avg_k(&self) -> Kelvin {
                (($melting_min_c + $melting_max_c) * 0.5) + 273.15
            }
        }
    };
    // Add Density for material
    ($struct_name:ident, density_min:$density_min_gcm3:expr, density_max:$density_max_gcm3:expr) => {
        impl Density for $struct_name {
            fn get_density_avg_gcm3(&self) -> Gsm3 {
                ($density_min_gcm3 + $density_max_gcm3) * 0.5
            }
            fn get_density_avg_kgm3(&self) -> KGm3 {
                (($density_min_gcm3 + $density_max_gcm3) * 0.5) * 1000.0
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
}

/// Number of material structures
pub const COUNT_MATERIALS:u32 = ALL_MATERIALS.len() as u32;
/// SLang size
pub const COUNT_SUPPORTED_LANGUAGES:u16 = 2;
pub const ALL_MATERIALS: &[&dyn Material] = &[
    &Basalt,
    &Granite,
    &Obsidian,
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

new_rock!(
    Basalt,
    names: [SLang::RU:"Базальт", SLang::EN:"Basalt"],
    group: [Igneous, GroupRock::Igneous, SubgroupIgneous, SubgroupIgneous::Extrusive],
    melting_c: [1100.0, 1250.0],
    dencity_gsm3:[2.6, 3.1],
);
new_rock!(
    Granite,
    names: [SLang::RU:"Гранит", SLang::EN:"Granite"],
    group: [Igneous, GroupRock::Igneous, SubgroupIgneous, SubgroupIgneous::Intrusive],
    melting_c: [1215.0, 1260.0],
    dencity_gsm3:[2.6, 3.0],
);
new_rock!(
    Obsidian,
    names:[SLang::RU:"Обсидиан", SLang::EN:"Obsidian"],
    group: [Igneous, GroupRock::Igneous, SubgroupIgneous, SubgroupIgneous::Extrusive],
    melting_c: [1200.0, 1500.0],
    dencity_gsm3:[2.5, 2.6],
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
        println!(
            "{} density:{}g/cm³ melting:{}°C",
            Obsidian.get_name(SLang::RU),
            Obsidian.get_density_avg_gcm3(),
            Obsidian.get_melting_avg_c()
        )
    }
}