/// ISO 639-1
pub enum SupportedLanguage {
    RU = 570,
    EN = 45,
}

/// Еденица температуры, >0
/// unit for temperature
type Kelvin = f32;
/// Еденица температуры
type Celsius = f32;

#[allow(dead_code)]
pub struct Rock {
    pub name_ru: &'static str,
    pub name_en: &'static str,
    group: GroupOfRocks,
    /// Средняя плотность, г/см3
    density_g_cm3: f32,
    /// Средняя плотность, г/м3
    density_kg_m3: f32,
    /// Средняя температура плавления в Цельсиях(°C)
    melting_avg_c:Option<Celsius>,
    melting_min_c:Option<Celsius>,
    melting_max_c:Option<Celsius>,
    /// Средняя температура плавления в Кельвинах(К)
    melting_avg_k:Option<Kelvin>,
    melting_min_k:Option<Kelvin>,
    melting_max_k:Option<Kelvin>,
    /// Средняя температура воспламенения в Цельсиях(°C)
    /// Это минимальная температура для начала горения.
    ignition_avg_c:Option<Celsius>,
    /// Средняя температура воспламенения в Кельвинах(К)
    /// Это минимальная температура для начала горения.
    ignition_avg_k:Option<Kelvin>,
    /// Средняя температура горения в Цельсиях(°C)
    /// Это температура, до которой нагреваются продукты горения (сгорания какого-либо топлива)
    /// Метан в чистом кислороде может гореть при температуре до 2800 °C, тогда как в воздухе — не более 1950 °C.
    /// Древесина в условиях хорошей тяги разогревается до 1100 °C, а при недостатке воздуха — всего до 600 °C
    burning_avg_c:Option<Celsius>,
    /// Средняя температура горения в Кельвинах(К)
    /// Это температура, до которой нагреваются продукты горения (сгорания какого-либо топлива)
    /// Метан в чистом кислороде может гореть при температуре до 2800 °C, тогда как в воздухе — не более 1950 °C.
    /// Древесина в условиях хорошей тяги разогревается до 1100 °C, а при недостатке воздуха — всего до 600 °C
    burning_avg_k:Option<Kelvin>,
}

#[allow(dead_code)]
/// Элемент из периодической системы химических элементов
pub struct Element {
    pub name_ru: &'static str,
    pub name_en: &'static str,
    /// Средняя плотность, г/см3
    density_g_cm3: f32,
    /// Средняя плотность, г/м3
    density_kg_m3: f32,
    /// Температура плавления
    melting_point: Kelvin,
    /// Температура кипения
    boiling_point: Kelvin,
}

#[allow(dead_code)]
pub struct Metall {
    element: Element,
    /// Твёрдость по шкале Мооса, от 0.0 до 10.0
    hardness_mohs: i8,
}

#[allow(dead_code)]  
pub struct Mineral {
    pub name: &'static str, 
    /// Твёрдость по шкале Мооса
    hardness_mohs: i8,
    /// Средняя плотность, г/см3
    density_g_cm3: f32,
    /// Средняя плотность, г/м3
    density_kg_m3: f32,
}

#[derive(Debug)]
/// Группы горных пород (По образованию)
pub enum GroupOfRocks {
    /// Осадочные породы. Образуются в процессе осаждения, со временем уплотняются под тяжестью накапливающегося материала, в результате чего становятся плотными и превращаются в горную породу.
    Sedimentary,
    /// Магматические породы. Образуется из расплавленной магмы при её охлаждении и затвердевании.
    Igneous,
    /// Метаморфическая порода. Образуется в результате изменения других пород под воздействием высокого давления и температуры.
    Metamorphic,
}

impl Element {
    pub const fn new(
        name_ru: &'static str,
        name_en:&'static str,
        density_g_cm3:f32,
        melting_point:f32,
        boiling_point:f32,

    ) -> Self {
        Element {
            name_ru,
            name_en,
            density_g_cm3,
            density_kg_m3: density_g_cm3*1000.0,
            // hardness_mohs: 0,
            melting_point,
            boiling_point,
        }
    }
}

const fn avg_c(arr: &[Option<Celsius>; 2]) -> Option<Celsius> {
    match arr {
        [Some(a), Some(b)] => Some((*a + *b) * 0.5),
        _ => None,
    }
}
const fn celsius_to_kelvin(celsius: Option<Celsius>) -> Option<Kelvin> {
    match celsius {
        Some(temp) => Some(temp + 273.15),
        None => None,
    }
}


impl Rock {
    pub const fn new(
        name_ru: &'static str,
        name_en:&'static str,
        group: GroupOfRocks,
        density_g_cm3:f32,
        melting_c:[Option<Celsius>; 2],
        ignition_c:Option<Celsius>,
        burning_c:Option<Celsius>,
    ) -> Self {
        Rock {
            name_ru,
            name_en,
            group,
            density_g_cm3,
            density_kg_m3: density_g_cm3*1000.0,
            melting_avg_c: avg_c(&melting_c),
            melting_min_c: melting_c[0],
            melting_max_c: melting_c[1],
            melting_avg_k: celsius_to_kelvin(avg_c(&melting_c)),
            melting_min_k: celsius_to_kelvin(melting_c[0]),
            melting_max_k: celsius_to_kelvin(melting_c[1]),
            ignition_avg_c: ignition_c,
            ignition_avg_k: celsius_to_kelvin(ignition_c),
            burning_avg_c: burning_c,
            burning_avg_k: celsius_to_kelvin(burning_c),
        }
    }
}

pub trait Material {
    fn get_name(&self, lang:SupportedLanguage) -> &'static str;
    fn print(&self);
}
impl Material for Rock {
    fn get_name(&self, lang:SupportedLanguage) -> &'static str {
        match lang {
            SupportedLanguage::RU => self.name_ru,
            SupportedLanguage::EN => self.name_en,
        }
    }
    fn print(&self) {
        println!("Material(Rock):{}, density_g_cm3:{}, density_kg_m3:{} melting_avg_c:{:?} burning_avg_c:{:?} ignition_avg_c:{:?}", self.name_en, self.density_g_cm3, self.density_kg_m3, self.melting_avg_c, self.burning_avg_c, self.ignition_avg_c)
    }
}
impl Material for Element {
    fn get_name(&self, lang:SupportedLanguage) -> &'static str {
        match lang {
            SupportedLanguage::RU => self.name_ru,
            SupportedLanguage::EN => self.name_en,
        }
    }
    fn print(&self) {
        println!("Material(Element):{}, density_g_cm3:{}, density_kg_m3:{}", self.name_en, self.density_g_cm3, self.density_kg_m3)
    }
}

pub const COPPER:Element = Element::new("Медь","Copper", 8.92, 1356.6, 2840.0);
pub const GOLD:Element = Element::new("Золото","Gold", 19.31, 1337.33, 3129.0);
pub const SERA:Element = Element::new("Сера","Sera", 2.07, 386.0, 717.824);
pub const OXYGEN:Element = Element::new("Кислород","Oxygen", 0.00142897, 	54.8, 90.19);

pub const GRANITE:Rock = Rock::new(
    "Гранит","Granite", GroupOfRocks::Igneous, 2.7,
    [Some(1215.0), Some(1260.0)], None, None,
);
pub const BASALT:Rock = Rock::new(
    "Базальт","Basalat", GroupOfRocks::Igneous, 2.85, 
    [Some(1100.0), Some(1250.0)], None, None,
);
pub const OBSIDIAN:Rock = Rock::new(
    "Обсидиан","Obsidian", GroupOfRocks::Igneous, 2.55, 
    [Some(1200.0), Some(1500.0)], None, None,
);
pub const SANDSTONE:Rock = Rock::new(
    "Песчаник","Sandstone", GroupOfRocks::Sedimentary, 2.44, 
    [Some(1500.), Some(1700.0)], None, None,
);
/// Торф, предшественник угля
pub const PEAT:Rock = Rock::new(
    "Торф","Peat", GroupOfRocks::Sedimentary, 2.44, 
    [None, None], Some(60.0), Some(500.0),
);
/// Твёрдый ископаемый уголь, образовавшийся из торфа, содержит 65—70 % углерода, имеет бурый цвет, наиболее молодой из ископаемых углей.
/// Значительная часть бурых углей залегает на небольших глубинах в угольных залежах глубинах 10—60 м, что позволяет отрабатывать их открытым способом. На отдельных месторождениях глубины залежей 100—200 м.ё
pub const BROWN_COAL:Rock = Rock::new(
    "Бурый уголь","Brown coal", GroupOfRocks::Sedimentary, 1.0, 
    [None, None], Some(250.0), Some(1900.0),
);
/// Образуется из бурого угля на глубинах порядка трёх километров.
/// В английском такой уголь называют иначе, возможно просто coal, не уверен. Возможно hard coal относится к антрациту.
pub const STONE_COAL:Rock = Rock::new(
    "Каменный уголь","Stone coal", GroupOfRocks::Sedimentary, 2.44, 
    [None, None], Some(400.0), Some(2100.0),
);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        BASALT.print();
        COPPER.print();
        GOLD.print();
        OXYGEN.print();
        BROWN_COAL.print();
    }
}