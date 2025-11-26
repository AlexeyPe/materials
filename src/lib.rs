#![allow(dead_code)]
//! <h2>Materials - is a library of material data</h2>
//!
//! 1 material = 1 struct
//!
//! Macros are used to create structures.<br>
//! Materials support localization.
//!
//! Для создания структур используются макросы.<br>
//! Материалы поддерживают локализацию.


macro_rules! new_rock {
    (
        $struct_name:ident,
        names: [$($field_name:path : $lang_literal:literal),*]
    ) => {
        #[derive(Debug)]
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

/// Number of material structures <br>
/// Количество структур материалов
pub const COUNT_MATERIALS:u32 = 2;
/// Number of rock structures <br>
/// Количество структур горной породы
pub const COUNT_ROCKS:u32 = 2;
/// Количество поддерживаемых языков
pub const COUNT_SUPPORTED_LANGUAGES:u16 = 2;
pub const ALL_MATERIALS: &[&dyn Material] = &[
    &Basalt,
    &Granite,
];

/// SLang = Supported Language. ISO 639-1
pub enum SLang {
    RU = 570,
    EN = 45,
}

/// Еденица температуры, >0
/// unit for temperature
type Kelvin = f32;
/// Еденица температуры, °C
type Celsius = f32;

#[derive(Debug)]
/// Группы горных пород (По образованию)
pub enum RockGroup {
    /// Осадочные породы. Образуются в процессе осаждения, со временем уплотняются под тяжестью накапливающегося материала, в результате чего становятся плотными и превращаются в горную породу.
    Sedimentary,
    /// Магматические породы. Образуется из расплавленной магмы при её охлаждении и затвердевании.
    Igneous,
    /// Метаморфическая порода. Образуется в результате изменения других пород под воздействием высокого давления и температуры.
    Metamorphic,
}

/// Базовое свойство, даёт локализованное название
pub trait Material {
    /// Возвращает прогресс локализации. <br>
    /// Увеличивается на 1 за каждое переведенное SLang. <br>
    /// В идеале должно быть равно количеству SLang.
    fn get_progress_locale_name(&self) -> u16 {0}
    /// Возвращает локализованное название материала
    fn get_name(&self, _lang:SLang) -> &'static str {""}
}

/// Density, g/cm3 and kg/m3 <br>
/// Плотность, г/см3 и кг/м3
pub trait Density {
    const DENSITY_MIN_G_CM3: f32;
    const DENSITY_MAX_G_CM3: f32;
    const DENSITY_AVG_G_CM3: f32 = (Self::DENSITY_MIN_G_CM3 + Self::DENSITY_MAX_G_CM3) * 0.5;

    const DENSITY_MIN_KG_M3: f32 = Self::DENSITY_MIN_G_CM3 * 1000.0;
    const DENSITY_MAX_KG_M3: f32 = Self::DENSITY_MAX_G_CM3 * 1000.0;
    const DENSITY_AVG_KG_M3: f32 = Self::DENSITY_AVG_G_CM3 * 1000.0;
}

/// Melting point, Kelvin and Celsius °C <br>
/// Температура плавления, Кельвины и градусы Цельсия °C
pub trait Melting {
    const MELTING_MIN_C: Celsius;
    const MELTING_MAX_C: Celsius;
    const MELTING_AVG_C: Celsius = (Self::MELTING_MIN_C + Self::MELTING_MAX_C) * 0.5;

    const MELTING_MIN_K: Kelvin = Self::MELTING_MIN_C + 273.15;
    const MELTING_MAX_K: Kelvin = Self::MELTING_MAX_C + 273.15;
    const MELTING_AVG_K: Kelvin = Self::MELTING_AVG_C + 273.15;
}

/// Горная порода
pub trait Rock {
    const GROUP: RockGroup;
}

new_rock!(Basalt, names:[SLang::RU:"Базальт", SLang::EN:"Basalt"]);
new_rock!(Granite, names:[SLang::RU:"Гранит", SLang::EN:"Granite"]);

#[cfg(test)]
mod localization_tests {
    use super::*;
    #[test]
    fn localization_name() {
        println!("COUNT_SUPPORTED_LANGUAGES:{}",COUNT_SUPPORTED_LANGUAGES);
        let mut need_translate:u32 = 0;
        for material in ALL_MATERIALS {
            let mut add: &str = "";
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