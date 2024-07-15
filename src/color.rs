use serde::{Deserialize, Deserializer};
use std::fmt;

#[derive(Clone, Debug)]
pub enum Color {
    None,
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
    Orange,
    Color0,
    Color1,
    Color2,
    Color3,
    Color4,
    Color5,
    Color6,
    Color7,
    Color8,
    Color9,
    Color10,
    Color11,
    Color12,
    Color13,
    Color14,
    Color15,
    Color16,
    Color17,
    Color18,
    Color19,
    Color20,
    Color21,
    Color22,
    Color23,
    Color24,
    Color25,
    Color26,
    Color27,
    Color28,
    Color29,
    Color30,
    Color31,
    Color32,
    Color33,
    Color34,
    Color35,
    Color36,
    Color37,
    Color38,
    Color39,
    Color40,
    Color41,
    Color42,
    Color43,
    Color44,
    Color45,
    Color46,
    Color47,
    Color48,
    Color49,
    Color50,
    Color51,
    Color52,
    Color53,
    Color54,
    Color55,
    Color56,
    Color57,
    Color58,
    Color59,
    Color60,
    Color61,
    Color62,
    Color63,
    Color64,
    Color65,
    Color66,
    Color67,
    Color68,
    Color69,
    Color70,
    Color71,
    Color72,
    Color73,
    Color74,
    Color75,
    Color76,
    Color77,
    Color78,
    Color79,
    Color80,
    Color81,
    Color82,
    Color83,
    Color84,
    Color85,
    Color86,
    Color87,
    Color88,
    Color89,
    Color90,
    Color91,
    Color92,
    Color93,
    Color94,
    Color95,
    Color96,
    Color97,
    Color98,
    Color99,
    Color100,
    Color101,
    Color102,
    Color103,
    Color104,
    Color105,
    Color106,
    Color107,
    Color108,
    Color109,
    Color110,
    Color111,
    Color112,
    Color113,
    Color114,
    Color115,
    Color116,
    Color117,
    Color118,
    Color119,
    Color120,
    Color121,
    Color122,
    Color123,
    Color124,
    Color125,
    Color126,
    Color127,
    Color128,
    Color129,
    Color130,
    Color131,
    Color132,
    Color133,
    Color134,
    Color135,
    Color136,
    Color137,
    Color138,
    Color139,
    Color140,
    Color141,
    Color142,
    Color143,
    Color144,
    Color145,
    Color146,
    Color147,
    Color148,
    Color149,
    Color150,
    Color151,
    Color152,
    Color153,
    Color154,
    Color155,
    Color156,
    Color157,
    Color158,
    Color159,
    Color160,
    Color161,
    Color162,
    Color163,
    Color164,
    Color165,
    Color166,
    Color167,
    Color168,
    Color169,
    Color170,
    Color171,
    Color172,
    Color173,
    Color174,
    Color175,
    Color176,
    Color177,
    Color178,
    Color179,
    Color180,
    Color181,
    Color182,
    Color183,
    Color184,
    Color185,
    Color186,
    Color187,
    Color188,
    Color189,
    Color190,
    Color191,
    Color192,
    Color193,
    Color194,
    Color195,
    Color196,
    Color197,
    Color198,
    Color199,
    Color200,
    Color201,
    Color202,
    Color203,
    Color204,
    Color205,
    Color206,
    Color207,
    Color208,
    Color209,
    Color210,
    Color211,
    Color212,
    Color213,
    Color214,
    Color215,
    Color216,
    Color217,
    Color218,
    Color219,
    Color220,
    Color221,
    Color222,
    Color223,
    Color224,
    Color225,
    Color226,
    Color227,
    Color228,
    Color229,
    Color230,
    Color231,
    Color232,
    Color233,
    Color234,
    Color235,
    Color236,
    Color237,
    Color238,
    Color239,
    Color240,
    Color241,
    Color242,
    Color243,
    Color244,
    Color245,
    Color246,
    Color247,
    Color248,
    Color249,
    Color250,
    Color251,
    Color252,
    Color253,
    Color254,
    Color255,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Color::None => write!(f, "default"),
            Color::Red => write!(f, "Red"),
            Color::Green => write!(f, "Green"),
            Color::Blue => write!(f, "Blue"),
            Color::Yellow => write!(f, "Yellow"),
            Color::Magenta => write!(f, "Magenta"),
            Color::Cyan => write!(f, "Cyan"),
            Color::Orange => write!(f, "color214"),
            Color::Color0 => write!(f, "color0"),
            Color::Color1 => write!(f, "color1"),
            Color::Color2 => write!(f, "color2"),
            Color::Color3 => write!(f, "color3"),
            Color::Color4 => write!(f, "color4"),
            Color::Color5 => write!(f, "color5"),
            Color::Color6 => write!(f, "color6"),
            Color::Color7 => write!(f, "color7"),
            Color::Color8 => write!(f, "color8"),
            Color::Color9 => write!(f, "color9"),
            Color::Color10 => write!(f, "color10"),
            Color::Color11 => write!(f, "color11"),
            Color::Color12 => write!(f, "color12"),
            Color::Color13 => write!(f, "color13"),
            Color::Color14 => write!(f, "color14"),
            Color::Color15 => write!(f, "color15"),
            Color::Color16 => write!(f, "color16"),
            Color::Color17 => write!(f, "color17"),
            Color::Color18 => write!(f, "color18"),
            Color::Color19 => write!(f, "color19"),
            Color::Color20 => write!(f, "color20"),
            Color::Color21 => write!(f, "color21"),
            Color::Color22 => write!(f, "color22"),
            Color::Color23 => write!(f, "color23"),
            Color::Color24 => write!(f, "color24"),
            Color::Color25 => write!(f, "color25"),
            Color::Color26 => write!(f, "color26"),
            Color::Color27 => write!(f, "color27"),
            Color::Color28 => write!(f, "color28"),
            Color::Color29 => write!(f, "color29"),
            Color::Color30 => write!(f, "color30"),
            Color::Color31 => write!(f, "color31"),
            Color::Color32 => write!(f, "color32"),
            Color::Color33 => write!(f, "color33"),
            Color::Color34 => write!(f, "color34"),
            Color::Color35 => write!(f, "color35"),
            Color::Color36 => write!(f, "color36"),
            Color::Color37 => write!(f, "color37"),
            Color::Color38 => write!(f, "color38"),
            Color::Color39 => write!(f, "color39"),
            Color::Color40 => write!(f, "color40"),
            Color::Color41 => write!(f, "color41"),
            Color::Color42 => write!(f, "color42"),
            Color::Color43 => write!(f, "color43"),
            Color::Color44 => write!(f, "color44"),
            Color::Color45 => write!(f, "color45"),
            Color::Color46 => write!(f, "color46"),
            Color::Color47 => write!(f, "color47"),
            Color::Color48 => write!(f, "color48"),
            Color::Color49 => write!(f, "color49"),
            Color::Color50 => write!(f, "color50"),
            Color::Color51 => write!(f, "color51"),
            Color::Color52 => write!(f, "color52"),
            Color::Color53 => write!(f, "color53"),
            Color::Color54 => write!(f, "color54"),
            Color::Color55 => write!(f, "color55"),
            Color::Color56 => write!(f, "color56"),
            Color::Color57 => write!(f, "color57"),
            Color::Color58 => write!(f, "color58"),
            Color::Color59 => write!(f, "color59"),
            Color::Color60 => write!(f, "color60"),
            Color::Color61 => write!(f, "color61"),
            Color::Color62 => write!(f, "color62"),
            Color::Color63 => write!(f, "color63"),
            Color::Color64 => write!(f, "color64"),
            Color::Color65 => write!(f, "color65"),
            Color::Color66 => write!(f, "color66"),
            Color::Color67 => write!(f, "color67"),
            Color::Color68 => write!(f, "color68"),
            Color::Color69 => write!(f, "color69"),
            Color::Color70 => write!(f, "color70"),
            Color::Color71 => write!(f, "color71"),
            Color::Color72 => write!(f, "color72"),
            Color::Color73 => write!(f, "color73"),
            Color::Color74 => write!(f, "color74"),
            Color::Color75 => write!(f, "color75"),
            Color::Color76 => write!(f, "color76"),
            Color::Color77 => write!(f, "color77"),
            Color::Color78 => write!(f, "color78"),
            Color::Color79 => write!(f, "color79"),
            Color::Color80 => write!(f, "color80"),
            Color::Color81 => write!(f, "color81"),
            Color::Color82 => write!(f, "color82"),
            Color::Color83 => write!(f, "color83"),
            Color::Color84 => write!(f, "color84"),
            Color::Color85 => write!(f, "color85"),
            Color::Color86 => write!(f, "color86"),
            Color::Color87 => write!(f, "color87"),
            Color::Color88 => write!(f, "color88"),
            Color::Color89 => write!(f, "color89"),
            Color::Color90 => write!(f, "color90"),
            Color::Color91 => write!(f, "color91"),
            Color::Color92 => write!(f, "color92"),
            Color::Color93 => write!(f, "color93"),
            Color::Color94 => write!(f, "color94"),
            Color::Color95 => write!(f, "color95"),
            Color::Color96 => write!(f, "color96"),
            Color::Color97 => write!(f, "color97"),
            Color::Color98 => write!(f, "color98"),
            Color::Color99 => write!(f, "color99"),
            Color::Color100 => write!(f, "color100"),
            Color::Color101 => write!(f, "color101"),
            Color::Color102 => write!(f, "color102"),
            Color::Color103 => write!(f, "color103"),
            Color::Color104 => write!(f, "color104"),
            Color::Color105 => write!(f, "color105"),
            Color::Color106 => write!(f, "color106"),
            Color::Color107 => write!(f, "color107"),
            Color::Color108 => write!(f, "color108"),
            Color::Color109 => write!(f, "color109"),
            Color::Color110 => write!(f, "color110"),
            Color::Color111 => write!(f, "color111"),
            Color::Color112 => write!(f, "color112"),
            Color::Color113 => write!(f, "color113"),
            Color::Color114 => write!(f, "color114"),
            Color::Color115 => write!(f, "color115"),
            Color::Color116 => write!(f, "color116"),
            Color::Color117 => write!(f, "color117"),
            Color::Color118 => write!(f, "color118"),
            Color::Color119 => write!(f, "color119"),
            Color::Color120 => write!(f, "color120"),
            Color::Color121 => write!(f, "color121"),
            Color::Color122 => write!(f, "color122"),
            Color::Color123 => write!(f, "color123"),
            Color::Color124 => write!(f, "color124"),
            Color::Color125 => write!(f, "color125"),
            Color::Color126 => write!(f, "color126"),
            Color::Color127 => write!(f, "color127"),
            Color::Color128 => write!(f, "color128"),
            Color::Color129 => write!(f, "color129"),
            Color::Color130 => write!(f, "color130"),
            Color::Color131 => write!(f, "color131"),
            Color::Color132 => write!(f, "color132"),
            Color::Color133 => write!(f, "color133"),
            Color::Color134 => write!(f, "color134"),
            Color::Color135 => write!(f, "color135"),
            Color::Color136 => write!(f, "color136"),
            Color::Color137 => write!(f, "color137"),
            Color::Color138 => write!(f, "color138"),
            Color::Color139 => write!(f, "color139"),
            Color::Color140 => write!(f, "color140"),
            Color::Color141 => write!(f, "color141"),
            Color::Color142 => write!(f, "color142"),
            Color::Color143 => write!(f, "color143"),
            Color::Color144 => write!(f, "color144"),
            Color::Color145 => write!(f, "color145"),
            Color::Color146 => write!(f, "color146"),
            Color::Color147 => write!(f, "color147"),
            Color::Color148 => write!(f, "color148"),
            Color::Color149 => write!(f, "color149"),
            Color::Color150 => write!(f, "color150"),
            Color::Color151 => write!(f, "color151"),
            Color::Color152 => write!(f, "color152"),
            Color::Color153 => write!(f, "color153"),
            Color::Color154 => write!(f, "color154"),
            Color::Color155 => write!(f, "color155"),
            Color::Color156 => write!(f, "color156"),
            Color::Color157 => write!(f, "color157"),
            Color::Color158 => write!(f, "color158"),
            Color::Color159 => write!(f, "color159"),
            Color::Color160 => write!(f, "color160"),
            Color::Color161 => write!(f, "color161"),
            Color::Color162 => write!(f, "color162"),
            Color::Color163 => write!(f, "color163"),
            Color::Color164 => write!(f, "color164"),
            Color::Color165 => write!(f, "color165"),
            Color::Color166 => write!(f, "color166"),
            Color::Color167 => write!(f, "color167"),
            Color::Color168 => write!(f, "color168"),
            Color::Color169 => write!(f, "color169"),
            Color::Color170 => write!(f, "color170"),
            Color::Color171 => write!(f, "color171"),
            Color::Color172 => write!(f, "color172"),
            Color::Color173 => write!(f, "color173"),
            Color::Color174 => write!(f, "color174"),
            Color::Color175 => write!(f, "color175"),
            Color::Color176 => write!(f, "color176"),
            Color::Color177 => write!(f, "color177"),
            Color::Color178 => write!(f, "color178"),
            Color::Color179 => write!(f, "color179"),
            Color::Color180 => write!(f, "color180"),
            Color::Color181 => write!(f, "color181"),
            Color::Color182 => write!(f, "color182"),
            Color::Color183 => write!(f, "color183"),
            Color::Color184 => write!(f, "color184"),
            Color::Color185 => write!(f, "color185"),
            Color::Color186 => write!(f, "color186"),
            Color::Color187 => write!(f, "color187"),
            Color::Color188 => write!(f, "color188"),
            Color::Color189 => write!(f, "color189"),
            Color::Color190 => write!(f, "color190"),
            Color::Color191 => write!(f, "color191"),
            Color::Color192 => write!(f, "color192"),
            Color::Color193 => write!(f, "color193"),
            Color::Color194 => write!(f, "color194"),
            Color::Color195 => write!(f, "color195"),
            Color::Color196 => write!(f, "color196"),
            Color::Color197 => write!(f, "color197"),
            Color::Color198 => write!(f, "color198"),
            Color::Color199 => write!(f, "color199"),
            Color::Color200 => write!(f, "color200"),
            Color::Color201 => write!(f, "color201"),
            Color::Color202 => write!(f, "color202"),
            Color::Color203 => write!(f, "color203"),
            Color::Color204 => write!(f, "color204"),
            Color::Color205 => write!(f, "color205"),
            Color::Color206 => write!(f, "color206"),
            Color::Color207 => write!(f, "color207"),
            Color::Color208 => write!(f, "color208"),
            Color::Color209 => write!(f, "color209"),
            Color::Color210 => write!(f, "color210"),
            Color::Color211 => write!(f, "color211"),
            Color::Color212 => write!(f, "color212"),
            Color::Color213 => write!(f, "color213"),
            Color::Color214 => write!(f, "color214"),
            Color::Color215 => write!(f, "color215"),
            Color::Color216 => write!(f, "color216"),
            Color::Color217 => write!(f, "color217"),
            Color::Color218 => write!(f, "color218"),
            Color::Color219 => write!(f, "color219"),
            Color::Color220 => write!(f, "color220"),
            Color::Color221 => write!(f, "color221"),
            Color::Color222 => write!(f, "color222"),
            Color::Color223 => write!(f, "color223"),
            Color::Color224 => write!(f, "color224"),
            Color::Color225 => write!(f, "color225"),
            Color::Color226 => write!(f, "color226"),
            Color::Color227 => write!(f, "color227"),
            Color::Color228 => write!(f, "color228"),
            Color::Color229 => write!(f, "color229"),
            Color::Color230 => write!(f, "color230"),
            Color::Color231 => write!(f, "color231"),
            Color::Color232 => write!(f, "color232"),
            Color::Color233 => write!(f, "color233"),
            Color::Color234 => write!(f, "color234"),
            Color::Color235 => write!(f, "color235"),
            Color::Color236 => write!(f, "color236"),
            Color::Color237 => write!(f, "color237"),
            Color::Color238 => write!(f, "color238"),
            Color::Color239 => write!(f, "color239"),
            Color::Color240 => write!(f, "color240"),
            Color::Color241 => write!(f, "color241"),
            Color::Color242 => write!(f, "color242"),
            Color::Color243 => write!(f, "color243"),
            Color::Color244 => write!(f, "color244"),
            Color::Color245 => write!(f, "color245"),
            Color::Color246 => write!(f, "color246"),
            Color::Color247 => write!(f, "color247"),
            Color::Color248 => write!(f, "color248"),
            Color::Color249 => write!(f, "color249"),
            Color::Color250 => write!(f, "color250"),
            Color::Color251 => write!(f, "color251"),
            Color::Color252 => write!(f, "color252"),
            Color::Color253 => write!(f, "color253"),
            Color::Color254 => write!(f, "color254"),
            Color::Color255 => write!(f, "color255"),
        }
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let color_str = String::deserialize(deserializer)?;
        match color_str.as_str() {
            "none" => Ok(Color::None),
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            "yellow" => Ok(Color::Yellow),
            "magenta" => Ok(Color::Magenta),
            "cyan" => Ok(Color::Cyan),
            "orange" => Ok(Color::Orange),
            "color0" => Ok(Color::Color0),
            "color1" => Ok(Color::Color1),
            "color2" => Ok(Color::Color2),
            "color3" => Ok(Color::Color3),
            "color4" => Ok(Color::Color4),
            "color5" => Ok(Color::Color5),
            "color6" => Ok(Color::Color6),
            "color7" => Ok(Color::Color7),
            "color8" => Ok(Color::Color8),
            "color9" => Ok(Color::Color9),
            "color10" => Ok(Color::Color10),
            "color11" => Ok(Color::Color11),
            "color12" => Ok(Color::Color12),
            "color13" => Ok(Color::Color13),
            "color14" => Ok(Color::Color14),
            "color15" => Ok(Color::Color15),
            "color16" => Ok(Color::Color16),
            "color17" => Ok(Color::Color17),
            "color18" => Ok(Color::Color18),
            "color19" => Ok(Color::Color19),
            "color20" => Ok(Color::Color20),
            "color21" => Ok(Color::Color21),
            "color22" => Ok(Color::Color22),
            "color23" => Ok(Color::Color23),
            "color24" => Ok(Color::Color24),
            "color25" => Ok(Color::Color25),
            "color26" => Ok(Color::Color26),
            "color27" => Ok(Color::Color27),
            "color28" => Ok(Color::Color28),
            "color29" => Ok(Color::Color29),
            "color30" => Ok(Color::Color30),
            "color31" => Ok(Color::Color31),
            "color32" => Ok(Color::Color32),
            "color33" => Ok(Color::Color33),
            "color34" => Ok(Color::Color34),
            "color35" => Ok(Color::Color35),
            "color36" => Ok(Color::Color36),
            "color37" => Ok(Color::Color37),
            "color38" => Ok(Color::Color38),
            "color39" => Ok(Color::Color39),
            "color40" => Ok(Color::Color40),
            "color41" => Ok(Color::Color41),
            "color42" => Ok(Color::Color42),
            "color43" => Ok(Color::Color43),
            "color44" => Ok(Color::Color44),
            "color45" => Ok(Color::Color45),
            "color46" => Ok(Color::Color46),
            "color47" => Ok(Color::Color47),
            "color48" => Ok(Color::Color48),
            "color49" => Ok(Color::Color49),
            "color50" => Ok(Color::Color50),
            "color51" => Ok(Color::Color51),
            "color52" => Ok(Color::Color52),
            "color53" => Ok(Color::Color53),
            "color54" => Ok(Color::Color54),
            "color55" => Ok(Color::Color55),
            "color56" => Ok(Color::Color56),
            "color57" => Ok(Color::Color57),
            "color58" => Ok(Color::Color58),
            "color59" => Ok(Color::Color59),
            "color60" => Ok(Color::Color60),
            "color61" => Ok(Color::Color61),
            "color62" => Ok(Color::Color62),
            "color63" => Ok(Color::Color63),
            "color64" => Ok(Color::Color64),
            "color65" => Ok(Color::Color65),
            "color66" => Ok(Color::Color66),
            "color67" => Ok(Color::Color67),
            "color68" => Ok(Color::Color68),
            "color69" => Ok(Color::Color69),
            "color70" => Ok(Color::Color70),
            "color71" => Ok(Color::Color71),
            "color72" => Ok(Color::Color72),
            "color73" => Ok(Color::Color73),
            "color74" => Ok(Color::Color74),
            "color75" => Ok(Color::Color75),
            "color76" => Ok(Color::Color76),
            "color77" => Ok(Color::Color77),
            "color78" => Ok(Color::Color78),
            "color79" => Ok(Color::Color79),
            "color80" => Ok(Color::Color80),
            "color81" => Ok(Color::Color81),
            "color82" => Ok(Color::Color82),
            "color83" => Ok(Color::Color83),
            "color84" => Ok(Color::Color84),
            "color85" => Ok(Color::Color85),
            "color86" => Ok(Color::Color86),
            "color87" => Ok(Color::Color87),
            "color88" => Ok(Color::Color88),
            "color89" => Ok(Color::Color89),
            "color90" => Ok(Color::Color90),
            "color91" => Ok(Color::Color91),
            "color92" => Ok(Color::Color92),
            "color93" => Ok(Color::Color93),
            "color94" => Ok(Color::Color94),
            "color95" => Ok(Color::Color95),
            "color96" => Ok(Color::Color96),
            "color97" => Ok(Color::Color97),
            "color98" => Ok(Color::Color98),
            "color99" => Ok(Color::Color99),
            "color100" => Ok(Color::Color100),
            "color101" => Ok(Color::Color101),
            "color102" => Ok(Color::Color102),
            "color103" => Ok(Color::Color103),
            "color104" => Ok(Color::Color104),
            "color105" => Ok(Color::Color105),
            "color106" => Ok(Color::Color106),
            "color107" => Ok(Color::Color107),
            "color108" => Ok(Color::Color108),
            "color109" => Ok(Color::Color109),
            "color110" => Ok(Color::Color110),
            "color111" => Ok(Color::Color111),
            "color112" => Ok(Color::Color112),
            "color113" => Ok(Color::Color113),
            "color114" => Ok(Color::Color114),
            "color115" => Ok(Color::Color115),
            "color116" => Ok(Color::Color116),
            "color117" => Ok(Color::Color117),
            "color118" => Ok(Color::Color118),
            "color119" => Ok(Color::Color119),
            "color120" => Ok(Color::Color120),
            "color121" => Ok(Color::Color121),
            "color122" => Ok(Color::Color122),
            "color123" => Ok(Color::Color123),
            "color124" => Ok(Color::Color124),
            "color125" => Ok(Color::Color125),
            "color126" => Ok(Color::Color126),
            "color127" => Ok(Color::Color127),
            "color128" => Ok(Color::Color128),
            "color129" => Ok(Color::Color129),
            "color130" => Ok(Color::Color130),
            "color131" => Ok(Color::Color131),
            "color132" => Ok(Color::Color132),
            "color133" => Ok(Color::Color133),
            "color134" => Ok(Color::Color134),
            "color135" => Ok(Color::Color135),
            "color136" => Ok(Color::Color136),
            "color137" => Ok(Color::Color137),
            "color138" => Ok(Color::Color138),
            "color139" => Ok(Color::Color139),
            "color140" => Ok(Color::Color140),
            "color141" => Ok(Color::Color141),
            "color142" => Ok(Color::Color142),
            "color143" => Ok(Color::Color143),
            "color144" => Ok(Color::Color144),
            "color145" => Ok(Color::Color145),
            "color146" => Ok(Color::Color146),
            "color147" => Ok(Color::Color147),
            "color148" => Ok(Color::Color148),
            "color149" => Ok(Color::Color149),
            "color150" => Ok(Color::Color150),
            "color151" => Ok(Color::Color151),
            "color152" => Ok(Color::Color152),
            "color153" => Ok(Color::Color153),
            "color154" => Ok(Color::Color154),
            "color155" => Ok(Color::Color155),
            "color156" => Ok(Color::Color156),
            "color157" => Ok(Color::Color157),
            "color158" => Ok(Color::Color158),
            "color159" => Ok(Color::Color159),
            "color160" => Ok(Color::Color160),
            "color161" => Ok(Color::Color161),
            "color162" => Ok(Color::Color162),
            "color163" => Ok(Color::Color163),
            "color164" => Ok(Color::Color164),
            "color165" => Ok(Color::Color165),
            "color166" => Ok(Color::Color166),
            "color167" => Ok(Color::Color167),
            "color168" => Ok(Color::Color168),
            "color169" => Ok(Color::Color169),
            "color170" => Ok(Color::Color170),
            "color171" => Ok(Color::Color171),
            "color172" => Ok(Color::Color172),
            "color173" => Ok(Color::Color173),
            "color174" => Ok(Color::Color174),
            "color175" => Ok(Color::Color175),
            "color176" => Ok(Color::Color176),
            "color177" => Ok(Color::Color177),
            "color178" => Ok(Color::Color178),
            "color179" => Ok(Color::Color179),
            "color180" => Ok(Color::Color180),
            "color181" => Ok(Color::Color181),
            "color182" => Ok(Color::Color182),
            "color183" => Ok(Color::Color183),
            "color184" => Ok(Color::Color184),
            "color185" => Ok(Color::Color185),
            "color186" => Ok(Color::Color186),
            "color187" => Ok(Color::Color187),
            "color188" => Ok(Color::Color188),
            "color189" => Ok(Color::Color189),
            "color190" => Ok(Color::Color190),
            "color191" => Ok(Color::Color191),
            "color192" => Ok(Color::Color192),
            "color193" => Ok(Color::Color193),
            "color194" => Ok(Color::Color194),
            "color195" => Ok(Color::Color195),
            "color196" => Ok(Color::Color196),
            "color197" => Ok(Color::Color197),
            "color198" => Ok(Color::Color198),
            "color199" => Ok(Color::Color199),
            "color200" => Ok(Color::Color200),
            "color201" => Ok(Color::Color201),
            "color202" => Ok(Color::Color202),
            "color203" => Ok(Color::Color203),
            "color204" => Ok(Color::Color204),
            "color205" => Ok(Color::Color205),
            "color206" => Ok(Color::Color206),
            "color207" => Ok(Color::Color207),
            "color208" => Ok(Color::Color208),
            "color209" => Ok(Color::Color209),
            "color210" => Ok(Color::Color210),
            "color211" => Ok(Color::Color211),
            "color212" => Ok(Color::Color212),
            "color213" => Ok(Color::Color213),
            "color214" => Ok(Color::Color214),
            "color215" => Ok(Color::Color215),
            "color216" => Ok(Color::Color216),
            "color217" => Ok(Color::Color217),
            "color218" => Ok(Color::Color218),
            "color219" => Ok(Color::Color219),
            "color220" => Ok(Color::Color220),
            "color221" => Ok(Color::Color221),
            "color222" => Ok(Color::Color222),
            "color223" => Ok(Color::Color223),
            "color224" => Ok(Color::Color224),
            "color225" => Ok(Color::Color225),
            "color226" => Ok(Color::Color226),
            "color227" => Ok(Color::Color227),
            "color228" => Ok(Color::Color228),
            "color229" => Ok(Color::Color229),
            "color230" => Ok(Color::Color230),
            "color231" => Ok(Color::Color231),
            "color232" => Ok(Color::Color232),
            "color233" => Ok(Color::Color233),
            "color234" => Ok(Color::Color234),
            "color235" => Ok(Color::Color235),
            "color236" => Ok(Color::Color236),
            "color237" => Ok(Color::Color237),
            "color238" => Ok(Color::Color238),
            "color239" => Ok(Color::Color239),
            "color240" => Ok(Color::Color240),
            "color241" => Ok(Color::Color241),
            "color242" => Ok(Color::Color242),
            "color243" => Ok(Color::Color243),
            "color244" => Ok(Color::Color244),
            "color245" => Ok(Color::Color245),
            "color246" => Ok(Color::Color246),
            "color247" => Ok(Color::Color247),
            "color248" => Ok(Color::Color248),
            "color249" => Ok(Color::Color249),
            "color250" => Ok(Color::Color250),
            "color251" => Ok(Color::Color251),
            "color252" => Ok(Color::Color252),
            "color253" => Ok(Color::Color253),
            "color254" => Ok(Color::Color254),
            "color255" => Ok(Color::Color255),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid color: {}",
                color_str
            ))),
        }
    }
}
