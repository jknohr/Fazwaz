use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {


    // Europe
   
    pub albania: (String, String, String),      // ("Lek", "ALL", "L")
    pub andorra: (String, String, String),      // ("Euro", "EUR", "€")
    pub austria: (String, String, String),      // ("Euro", "EUR", "€")
    pub belgium: (String, String, String),      // ("Euro", "EUR", "€")
    pub belarus: (String, String, String),      // ("Ruble", "BYN", "Br")
    pub bosnia: (String, String, String),       // ("Mark", "BAM", "KM")
    pub bulgaria: (String, String, String),     // ("Lev", "BGN", "лв")
    pub croatia: (String, String, String),      // ("Euro", "EUR", "€")
    pub cyprus: (String, String, String),       // ("Euro", "EUR", "€")
    pub czech_republic: (String, String, String), // ("Koruna", "CZK", "Kč")
    pub denmark: (String, String, String),      // ("Krone", "DKK", "kr")
    pub estonia: (String, String, String),      // ("Euro", "EUR", "€")
    pub finland: (String, String, String),      // ("Euro", "EUR", "€")
    pub france: (String, String, String),       // ("Euro", "EUR", "€")
    pub germany: (String, String, String),      // ("Euro", "EUR", "€")
    pub greece: (String, String, String),       // ("Euro", "EUR", "€")
    pub hungary: (String, String, String),      // ("Forint", "HUF", "Ft")
    pub iceland: (String, String, String),      // ("Króna", "ISK", "kr")
    pub ireland: (String, String, String),      // ("Euro", "EUR", "€")
    pub italy: (String, String, String),        // ("Euro", "EUR", "€")
    pub latvia: (String, String, String),       // ("Euro", "EUR", "€")
    pub liechtenstein: (String, String, String), // ("Franc", "CHF", "Fr")
    pub lithuania: (String, String, String),    // ("Euro", "EUR", "€")
    pub luxembourg: (String, String, String),   // ("Euro", "EUR", "€")
    pub malta: (String, String, String),        // ("Euro", "EUR", "€")
    pub moldova: (String, String, String),      // ("Leu", "MDL", "L")
    pub monaco: (String, String, String),       // ("Euro", "EUR", "€")
    pub montenegro: (String, String, String),   // ("Euro", "EUR", "€")
    pub netherlands: (String, String, String),  // ("Euro", "EUR", "€")
    pub north_macedonia: (String, String, String), // ("Denar", "MKD", "ден")
    pub norway: (String, String, String),       // ("Krone", "NOK", "kr")
    pub poland: (String, String, String),       // ("Złoty", "PLN", "zł")
    pub portugal: (String, String, String),     // ("Euro", "EUR", "€")
    pub romania: (String, String, String),      // ("Leu", "RON", "lei")
    pub russia: (String, String, String),       // ("Ruble", "RUB", "₽")
    pub san_marino: (String, String, String),   // ("Euro", "EUR", "€")
    pub serbia: (String, String, String),       // ("Dinar", "RSD", "дин")
    pub slovakia: (String, String, String),     // ("Euro", "EUR", "€")
    pub slovenia: (String, String, String),     // ("Euro", "EUR", "€")
    pub spain: (String, String, String),        // ("Euro", "EUR", "€")
    pub sweden: (String, String, String),       // ("Krona", "SEK", "kr")
    pub switzerland: (String, String, String),  // ("Franc", "CHF", "Fr")
    pub ukraine: (String, String, String),      // ("Hryvnia", "UAH", "₴")
    pub united_kingdom: (String, String, String), // ("Pound", "GBP", "£")
    pub vatican_city: (String, String, String), // ("Euro", "EUR", "€")

  // Asia
      // Asia
   
    pub afghanistan: (String, String, String),  // ("Afghani", "AFN", "؋")
    pub armenia: (String, String, String),      // ("Dram", "AMD", "֏")
    pub azerbaijan: (String, String, String),   // ("Manat", "AZN", "₼")
    pub bahrain: (String, String, String),      // ("Dinar", "BHD", ".د.ب")
    pub bangladesh: (String, String, String),   // ("Taka", "BDT", "৳")
    pub bhutan: (String, String, String),       // ("Ngultrum", "BTN", "Nu.")
    pub brunei: (String, String, String),       // ("Dollar", "BND", "$")
    pub cambodia: (String, String, String),     // ("Riel", "KHR", "៛")
    pub china: (String, String, String),        // ("Yuan", "CNY", "¥")
    pub east_timor: (String, String, String),   // ("Dollar", "USD", "$")
    pub georgia: (String, String, String),      // ("Lari", "GEL", "₾")
    pub hong_kong: (String, String, String),    // ("Dollar", "HKD", "HK$")
    pub india: (String, String, String),        // ("Rupee", "INR", "₹")
    pub indonesia: (String, String, String),    // ("Rupiah", "IDR", "Rp")
    pub iran: (String, String, String),         // ("Rial", "IRR", "﷼")
    pub iraq: (String, String, String),         // ("Dinar", "IQD", "ع.د")
    pub israel: (String, String, String),       // ("Shekel", "ILS", "₪")
    pub japan: (String, String, String),        // ("Yen", "JPY", "¥")
    pub jordan: (String, String, String),       // ("Dinar", "JOD", "د.ا")
    pub kazakhstan: (String, String, String),   // ("Tenge", "KZT", "₸")
    pub kuwait: (String, String, String),       // ("Dinar", "KWD", "د.ك")
    pub kyrgyzstan: (String, String, String),   // ("Som", "KGS", "с")
    pub laos: (String, String, String),         // ("Kip", "LAK", "₭")
    pub lebanon: (String, String, String),      // ("Pound", "LBP", "ل.ل")
    pub macau: (String, String, String),        // ("Pataca", "MOP", "MOP$")
    pub malaysia: (String, String, String),     // ("Ringgit", "MYR", "RM")
    pub maldives: (String, String, String),     // ("Rufiyaa", "MVR", "Rf")
    pub mongolia: (String, String, String),     // ("Tugrik", "MNT", "₮")
    pub myanmar: (String, String, String),      // ("Kyat", "MMK", "K")
    pub nepal: (String, String, String),        // ("Rupee", "NPR", "रू")
    pub north_korea: (String, String, String),  // ("Won", "KPW", "₩")
    pub oman: (String, String, String),         // ("Rial", "OMR", "ر.ع.")
    pub pakistan: (String, String, String),     // ("Rupee", "PKR", "₨")
    pub philippines: (String, String, String),  // ("Peso", "PHP", "₱")
    pub qatar: (String, String, String),        // ("Riyal", "QAR", "ر.ق")
    pub saudi_arabia: (String, String, String), // ("Riyal", "SAR", "ر.س")
    pub singapore: (String, String, String),    // ("Dollar", "SGD", "S$")
    pub south_korea: (String, String, String),  // ("Won", "KRW", "₩")
    pub sri_lanka: (String, String, String),    // ("Rupee", "LKR", "රු")
    pub syria: (String, String, String),        // ("Pound", "SYP", "£S")
    pub taiwan: (String, String, String),       // ("Dollar", "TWD", "NT$")
    pub tajikistan: (String, String, String),   // ("Somoni", "TJS", "ЅМ")
    pub thailand: (String, String, String),     // ("Baht", "THB", "฿")
    pub turkey: (String, String, String),       // ("Lira", "TRY", "₺")
    pub turkmenistan: (String, String, String), // ("Manat", "TMT", "T")
    pub uae: (String, String, String),          // ("Dirham", "AED", "د.إ")
    pub uzbekistan: (String, String, String),   // ("Som", "UZS", "so'm")
    pub vietnam: (String, String, String),      // ("Dong", "VND", "₫")
    pub yemen: (String, String, String),        // ("Rial", "YER", "﷼")

    // Africa
    pub algeria: (String, String, String),      // ("Dinar", "DZD", "د.ج")
    pub angola: (String, String, String),       // ("Kwanza", "AOA", "Kz")
    pub benin: (String, String, String),        // ("CFA Franc", "XOF", "CFA")
    pub botswana: (String, String, String),     // ("Pula", "BWP", "P")
    pub burkina_faso: (String, String, String), // ("CFA Franc", "XOF", "CFA")
    pub burundi: (String, String, String),      // ("Franc", "BIF", "FBu")
    pub cameroon: (String, String, String),     // ("CFA Franc", "XAF", "FCFA")
    pub cape_verde: (String, String, String),   // ("Escudo", "CVE", "$")
    pub car: (String, String, String),          // ("CFA Franc", "XAF", "FCFA")
    pub chad: (String, String, String),         // ("CFA Franc", "XAF", "FCFA")
    pub comoros: (String, String, String),      // ("Franc", "KMF", "CF")
    pub congo: (String, String, String),        // ("CFA Franc", "XAF", "FCFA")
    pub drc: (String, String, String),          // ("Franc", "CDF", "FC")
    pub djibouti: (String, String, String),     // ("Franc", "DJF", "Fdj")
    pub egypt: (String, String, String),        // ("Pound", "EGP", "£")
    pub eq_guinea: (String, String, String),    // ("CFA Franc", "XAF", "FCFA")
    pub eritrea: (String, String, String),      // ("Nakfa", "ERN", "Nfk")
    pub ethiopia: (String, String, String),     // ("Birr", "ETB", "Br")
    pub gabon: (String, String, String),        // ("CFA Franc", "XAF", "FCFA")
    pub gambia: (String, String, String),       // ("Dalasi", "GMD", "D")
    pub ghana: (String, String, String),        // ("Cedi", "GHS", "₵")
    pub guinea: (String, String, String),       // ("Franc", "GNF", "FG")
    pub guinea_bissau: (String, String, String),// ("CFA Franc", "XOF", "CFA")
    pub ivory_coast: (String, String, String),  // ("CFA Franc", "XOF", "CFA")
    pub kenya: (String, String, String),        // ("Shilling", "KES", "KSh")
    pub lesotho: (String, String, String),      // ("Loti", "LSL", "L")
    pub liberia: (String, String, String),      // ("Dollar", "LRD", "$")
    pub libya: (String, String, String),        // ("Dinar", "LYD", "ل.د")
    pub madagascar: (String, String, String),   // ("Ariary", "MGA", "Ar")
    pub malawi: (String, String, String),       // ("Kwacha", "MWK", "MK")
    pub mali: (String, String, String),         // ("CFA Franc", "XOF", "CFA")
    pub mauritania: (String, String, String),   // ("Ouguiya", "MRU", "UM")
    pub mauritius: (String, String, String),    // ("Rupee", "MUR", "₨")
    pub morocco: (String, String, String),      // ("Dirham", "MAD", "د.م.")
    pub mozambique: (String, String, String),   // ("Metical", "MZN", "MT")
    pub namibia: (String, String, String),      // ("Dollar", "NAD", "$")
    pub niger: (String, String, String),        // ("CFA Franc", "XOF", "CFA")
    pub nigeria: (String, String, String),      // ("Naira", "NGN", "₦")
    pub rwanda: (String, String, String),       // ("Franc", "RWF", "FRw")
    pub sao_tome: (String, String, String),     // ("Dobra", "STN", "Db")
    pub senegal: (String, String, String),      // ("CFA Franc", "XOF", "CFA")
    pub seychelles: (String, String, String),   // ("Rupee", "SCR", "₨")
    pub sierra_leone: (String, String, String), // ("Leone", "SLL", "Le")
    pub somalia: (String, String, String),      // ("Shilling", "SOS", "Sh")
    pub south_africa: (String, String, String), // ("Rand", "ZAR", "R")
    pub south_sudan: (String, String, String),  // ("Pound", "SSP", "£")
    pub sudan: (String, String, String),        // ("Pound", "SDG", "ج.س.")
    pub swaziland: (String, String, String),    // ("Lilangeni", "SZL", "L")
    pub tanzania: (String, String, String),     // ("Shilling", "TZS", "TSh")
    pub togo: (String, String, String),         // ("CFA Franc", "XOF", "CFA")
    pub tunisia: (String, String, String),      // ("Dinar", "TND", "د.ت")
    pub uganda: (String, String, String),       // ("Shilling", "UGX", "USh")
    pub zambia: (String, String, String),       // ("Kwacha", "ZMW", "ZK")
    pub zimbabwe: (String, String, String),     // ("Dollar", "USD", "$")

    // North America
    pub antigua: (String, String, String),      // ("Dollar", "XCD", "$")
    pub bahamas: (String, String, String),      // ("Dollar", "BSD", "$")
    pub barbados: (String, String, String),     // ("Dollar", "BBD", "$")
    pub belize: (String, String, String),       // ("Dollar", "BZD", "BZ$")
    pub canada: (String, String, String),       // ("Dollar", "CAD", "$")
    pub costa_rica: (String, String, String),   // ("Colón", "CRC", "₡")
    pub cuba: (String, String, String),         // ("Peso", "CUP", "₱")
    pub dominica: (String, String, String),     // ("Dollar", "XCD", "$")
    pub dominican_rep: (String, String, String),// ("Peso", "DOP", "RD$")
    pub el_salvador: (String, String, String),  // ("Dollar", "USD", "$")
    pub grenada: (String, String, String),      // ("Dollar", "XCD", "$")
    pub guatemala: (String, String, String),    // ("Quetzal", "GTQ", "Q")
    pub haiti: (String, String, String),        // ("Gourde", "HTG", "G")
    pub honduras: (String, String, String),     // ("Lempira", "HNL", "L")
    pub jamaica: (String, String, String),      // ("Dollar", "JMD", "J$")
    pub mexico: (String, String, String),       // ("Peso", "MXN", "$")
    pub nicaragua: (String, String, String),    // ("Córdoba", "NIO", "C$")
    pub panama: (String, String, String),       // ("Balboa", "PAB", "B/.")
    pub st_kitts: (String, String, String),     // ("Dollar", "XCD", "$")
    pub st_lucia: (String, String, String),     // ("Dollar", "XCD", "$")
    pub st_vincent: (String, String, String),   // ("Dollar", "XCD", "$")
    pub trinidad: (String, String, String),     // ("Dollar", "TTD", "TT$")
    pub usa: (String, String, String),          // ("Dollar", "USD", "$")

    // South America
    pub argentina: (String, String, String),    // ("Peso", "ARS", "$")
    pub bolivia: (String, String, String),      // ("Boliviano", "BOB", "Bs.")
    pub brazil: (String, String, String),       // ("Real", "BRL", "R$")
    pub chile: (String, String, String),        // ("Peso", "CLP", "$")
    pub colombia: (String, String, String),     // ("Peso", "COP", "$")
    pub ecuador: (String, String, String),      // ("Dollar", "USD", "$")
    pub guyana: (String, String, String),       // ("Dollar", "GYD", "$")
    pub paraguay: (String, String, String),     // ("Guaraní", "PYG", "₲")
    pub peru: (String, String, String),         // ("Sol", "PEN", "S/")
    pub suriname: (String, String, String),     // ("Dollar", "SRD", "$")
    pub uruguay: (String, String, String),      // ("Peso", "UYU", "$U")
    pub venezuela: (String, String, String),    // ("Bolívar", "VES", "Bs.")

    // Oceania
    pub australia: (String, String, String),    // ("Dollar", "AUD", "$")
    pub fiji: (String, String, String),         // ("Dollar", "FJD", "$")
    pub kiribati: (String, String, String),     // ("Dollar", "AUD", "$")
    pub marshall_islands: (String, String, String), // ("Dollar", "USD", "$")
    pub micronesia: (String, String, String),   // ("Dollar", "USD", "$")
    pub nauru: (String, String, String),        // ("Dollar", "AUD", "$")
    pub new_zealand: (String, String, String),  // ("Dollar", "NZD", "$")
    pub palau: (String, String, String),        // ("Dollar", "USD", "$")
    pub papua_new_guinea: (String, String, String), // ("Kina", "PGK", "K")
    pub samoa: (String, String, String),        // ("Tala", "WST", "T")
    pub solomon_islands: (String, String, String), // ("Dollar", "SBD", "$")
    pub tonga: (String, String, String),        // ("Paʻanga", "TOP", "T$")
    pub tuvalu: (String, String, String),       // ("Dollar", "AUD", "$")
    pub vanuatu: (String, String, String),      // ("Vatu", "VUV", "VT")
}

impl Currency {
    pub fn new() -> Self {
        Self {      
            afghanistan: ("Afghani".to_string(), "AFN".to_string(), "؋".to_string()),
            armenia: ("Dram".to_string(), "AMD".to_string(), "֏".to_string()),
            azerbaijan: ("Manat".to_string(), "AZN".to_string(), "₼".to_string()),
            bahrain: ("Dinar".to_string(), "BHD".to_string(), ".د.ب".to_string()),
            bangladesh: ("Taka".to_string(), "BDT".to_string(), "৳".to_string()),
            bhutan: ("Ngultrum".to_string(), "BTN".to_string(), "Nu.".to_string()),
            brunei: ("Dollar".to_string(), "BND".to_string(), "$".to_string()),
            cambodia: ("Riel".to_string(), "KHR".to_string(), "៛".to_string()),
            china: ("Yuan".to_string(), "CNY".to_string(), "¥".to_string()),
            east_timor: ("Dollar".to_string(), "USD".to_string(), "$".to_string()),
            georgia: ("Lari".to_string(), "GEL".to_string(), "₾".to_string()),
            hong_kong: ("Dollar".to_string(), "HKD".to_string(), "HK$".to_string()),
            india: ("Rupee".to_string(), "INR".to_string(), "₹".to_string()),
            indonesia: ("Rupiah".to_string(), "IDR".to_string(), "Rp".to_string()),
            iran: ("Rial".to_string(), "IRR".to_string(), "﷼".to_string()),
            iraq: ("Dinar".to_string(), "IQD".to_string(), "ع.د".to_string()),
            israel: ("Shekel".to_string(), "ILS".to_string(), "₪".to_string()),
            japan: ("Yen".to_string(), "JPY".to_string(), "¥".to_string()),
            jordan: ("Dinar".to_string(), "JOD".to_string(), "د.ا".to_string()),
            kazakhstan: ("Tenge".to_string(), "KZT".to_string(), "₸".to_string()),
            kuwait: ("Dinar".to_string(), "KWD".to_string(), "د.ك".to_string()),
            kyrgyzstan: ("Som".to_string(), "KGS".to_string(), "с".to_string()),
            laos: ("Kip".to_string(), "LAK".to_string(), "₭".to_string()),
            lebanon: ("Pound".to_string(), "LBP".to_string(), "ل.ل".to_string()),
            macau: ("Pataca".to_string(), "MOP".to_string(), "MOP$".to_string()),
            malaysia: ("Ringgit".to_string(), "MYR".to_string(), "RM".to_string()),
            maldives: ("Rufiyaa".to_string(), "MVR".to_string(), "Rf".to_string()),
            mongolia: ("Tugrik".to_string(), "MNT".to_string(), "₮".to_string()),
            myanmar: ("Kyat".to_string(), "MMK".to_string(), "K".to_string()),
            nepal: ("Rupee".to_string(), "NPR".to_string(), "रू".to_string()),
            north_korea: ("Won".to_string(), "KPW".to_string(), "₩".to_string()),
            oman: ("Rial".to_string(), "OMR".to_string(), "ر.ع.".to_string()),
            pakistan: ("Rupee".to_string(), "PKR".to_string(), "₨".to_string()),
            philippines: ("Peso".to_string(), "PHP".to_string(), "₱".to_string()),
            qatar: ("Riyal".to_string(), "QAR".to_string(), "ر.ق".to_string()),
            saudi_arabia: ("Riyal".to_string(), "SAR".to_string(), "ر.س".to_string()),
            singapore: ("Dollar".to_string(), "SGD".to_string(), "S$".to_string()),
            south_korea: ("Won".to_string(), "KRW".to_string(), "₩".to_string()),
            sri_lanka: ("Rupee".to_string(), "LKR".to_string(), "රු".to_string()),
            syria: ("Pound".to_string(), "SYP".to_string(), "£S".to_string()),
            taiwan: ("Dollar".to_string(), "TWD".to_string(), "NT$".to_string()),
            tajikistan: ("Somoni".to_string(), "TJS".to_string(), "ЅМ".to_string()),
            thailand: ("Baht".to_string(), "THB".to_string(), "฿".to_string()),
            turkey: ("Lira".to_string(), "TRY".to_string(), "₺".to_string()),
            turkmenistan: ("Manat".to_string(), "TMT".to_string(), "T".to_string()),
            uae: ("Dirham".to_string(), "AED".to_string(), "د.إ".to_string()),
            uzbekistan: ("Som".to_string(), "UZS".to_string(), "so'm".to_string()),
            vietnam: ("Dong".to_string(), "VND".to_string(), "₫".to_string()),
            yemen: ("Rial".to_string(), "YER".to_string(), "﷼".to_string()),
            albania: ("Lek".to_string(), "ALL".to_string(), "Lek".to_string()),
            algeria: ("Dinar".to_string(), "DZD".to_string(), "د.ج".to_string()),
            andorra: ("Euro".to_string(), "EUR".to_string(), "€".to_string()),
            austria: ("Euro".to_string(), "EUR".to_string(), "€".to_string()),
            belarus: ("Ruble".to_string(), "BYN".to_string(), "Br".to_string()),
            belgium: ("Euro".to_string(), "EUR".to_string(), "€".to_string()),
            bulgaria: ("Lev".to_string(), "BGN".to_string(), "лв".to_string()),
            bosnia: ("Mark".to_string(), "BAM".to_string(), "KM".to_string()),

            croatia: ("Kuna".to_string(), "HRK".to_string(), "kn".to_string()),
            cyprus: ("Euro".to_string(), "EUR".to_string(), "€".to_string()),
            czech_republic: ("Koruna".to_string(), "CZK".to_string(), "Kč".to_string()),
            denmark: ("Krone".to_string(), "DKK".to_string(), "kr".to_string()),
            estonia: ("Euro".to_string(), "EUR".to_string(), "€".to_string()),
            finland: ("Euro".to_string(), "EUR".to_string(), "€".to_string()),
            france: ("Euro".to_string(), "EUR".to_string(), "€".to_string()),
            germany: ("Euro".to_string(), "EUR".to_string(), "€".to_string()),
            greece: ("Euro".to_string(), "EUR".to_string(), "€".to_string()),
            hungary: ("Forint".to_string(), "HUF".to_string(), "Ft".to_string()),
            iceland: ("Króna".to_string(), "ISK".to_string(), "kr".to_string()),
            ireland: ("Euro".to_string(), "EUR".to_string(), "€".to_string()),
            italy: ("Euro".to_string(), "EUR".to_string(), "€".to_string()),
            latvia: ("Euro".to_string(), "EUR".to_string(), "€".to_string()),
            liechtenstein: ("Franc".to_string(), "CHF".to_string(), "Fr".to_string()),
            lithuania: ("Euro".to_string(), "EUR".to_string(), "€".to_string()),
            luxembourg: ("Euro".to_string(), "EUR".to_string(), "€".to_string()),
            malta: ("Euro".to_string(), "EUR".to_string(), "€".to_string()),
            moldova: ("Leu".to_string(), "MDL".to_string(), "L".to_string()),
            monaco: ("Euro".to_string(), "EUR".to_string(), "€".to_string()),
            montenegro: ("Euro".to_string(), "EUR".to_string(), "€".to_string()),
            netherlands: ("Euro".to_string(), "EUR".to_string(), "€".to_string()),
            north_macedonia: ("Denar".to_string(), "MKD".to_string(), "ден".to_string()),
            norway: ("Krone".to_string(), "NOK".to_string(), "kr".to_string()),
            poland: ("Złoty".to_string(), "PLN".to_string(), "zł".to_string()),
            portugal: ("Euro".to_string(), "EUR".to_string(), "€".to_string()),
            romania: ("Leu".to_string(), "RON".to_string(), "lei".to_string()),
            russia: ("Ruble".to_string(), "RUB".to_string(), "₽".to_string()),
            san_marino: ("Euro".to_string(), "EUR".to_string(), "€".to_string()),
            serbia: ("Dinar".to_string(), "RSD".to_string(), "дин".to_string()),
            slovakia: ("Euro".to_string(), "EUR".to_string(), "€".to_string()),
            slovenia: ("Euro".to_string(), "EUR".to_string(), "€".to_string()),
            spain: ("Euro".to_string(), "EUR".to_string(), "€".to_string()),
            sweden: ("Krona".to_string(), "SEK".to_string(), "kr".to_string()),
            switzerland: ("Franc".to_string(), "CHF".to_string(), "Fr".to_string()),
            ukraine: ("Hryvnia".to_string(), "UAH".to_string(), "₴".to_string()),
            united_kingdom: ("Pound".to_string(), "GBP".to_string(), "£".to_string()),
            vatican_city: ("Euro".to_string(), "EUR".to_string(), "€".to_string()),
            angola: ("Kwanza".to_string(), "AOA".to_string(), "Kz".to_string()),
            benin: ("Franc".to_string(), "XOF".to_string(), "CFA".to_string()),
            botswana: ("Pula".to_string(), "BWP".to_string(), "P".to_string()),
            burkina_faso: ("Franc".to_string(), "XOF".to_string(), "CFA".to_string()),
            burundi: ("Franc".to_string(), "BIF".to_string(), "FBu".to_string()),
            cameroon: ("Franc".to_string(), "XAF".to_string(), "FCFA".to_string()),
            cape_verde: ("Escudo".to_string(), "CVE".to_string(), "Esc".to_string()),
            car: ("Franc".to_string(), "XAF".to_string(), "FCFA".to_string()),
            chad: ("Franc".to_string(), "XAF".to_string(), "FCFA".to_string()),
            comoros: ("Franc".to_string(), "KMF".to_string(), "CF".to_string()),
            congo: ("Franc".to_string(), "XAF".to_string(), "FCFA".to_string()),
            drc: ("Franc".to_string(), "XAF".to_string(), "FCFA".to_string()),
            djibouti: ("Franc".to_string(), "DJF".to_string(), "Fdj".to_string()),
            egypt: ("Pound".to_string(), "EGP".to_string(), "£".to_string()),
            eq_guinea: ("Franc".to_string(), "XOF".to_string(), "CFA".to_string()),
            eritrea: ("Nakfa".to_string(), "ERN".to_string(), "Nfk".to_string()),
            ethiopia: ("Birr".to_string(), "ETB".to_string(), "Br".to_string()),
            gabon: ("CFA Franc".to_string(), "XAF".to_string(), "FCFA".to_string()),
            gambia: ("Dalasi".to_string(), "GMD".to_string(), "D".to_string()),
            ghana: ("Cedi".to_string(), "GHS".to_string(), "₵".to_string()),
            guinea: ("Franc".to_string(), "GNF".to_string(), "FG".to_string()),
            guinea_bissau: ("CFA Franc".to_string(), "XOF".to_string(), "CFA".to_string()),
            ivory_coast: ("CFA Franc".to_string(), "XOF".to_string(), "CFA".to_string()),
            kenya: ("Shilling".to_string(), "KES".to_string(), "KSh".to_string()),
            lesotho: ("Loti".to_string(), "LSL".to_string(), "L".to_string()),
            liberia: ("Dollar".to_string(), "LRD".to_string(), "$".to_string()),
            libya: ("Dinar".to_string(), "LYD".to_string(), "ل.د".to_string()),
            madagascar: ("Ariary".to_string(), "MGA".to_string(), "Ar".to_string()),
            malawi: ("Kwacha".to_string(), "MWK".to_string(), "MK".to_string()),
            mali: ("CFA Franc".to_string(), "XOF".to_string(), "CFA".to_string()),
            mauritania: ("Ouguiya".to_string(), "MRU".to_string(), "UM".to_string()),
            mauritius: ("Rupee".to_string(), "MUR".to_string(), "₨".to_string()),
            morocco: ("Dirham".to_string(), "MAD".to_string(), "د.م.".to_string()),
            mozambique: ("Metical".to_string(), "MZN".to_string(), "MT".to_string()),
            namibia: ("Dollar".to_string(), "NAD".to_string(), "$".to_string()),
            niger: ("CFA Franc".to_string(), "XOF".to_string(), "CFA".to_string()),
            nigeria: ("Naira".to_string(), "NGN".to_string(), "₦".to_string()),
            rwanda: ("Franc".to_string(), "RWF".to_string(), "FRw".to_string()),
            sao_tome: ("Dobra".to_string(), "STN".to_string(), "Db".to_string()),
            senegal: ("CFA Franc".to_string(), "XOF".to_string(), "CFA".to_string()),
            seychelles: ("Rupee".to_string(), "SCR".to_string(), "₨".to_string()),
            sierra_leone: ("Leone".to_string(), "SLL".to_string(), "Le".to_string()),
            somalia: ("Shilling".to_string(), "SOS".to_string(), "Sh".to_string()),
            south_africa: ("Rand".to_string(), "ZAR".to_string(), "R".to_string()),
            south_sudan: ("Pound".to_string(), "SSP".to_string(), "£".to_string()),
            sudan: ("Pound".to_string(), "SDG".to_string(), "ج.س.".to_string()),
            swaziland: ("Lilangeni".to_string(), "SZL".to_string(), "L".to_string()),
            tanzania: ("Shilling".to_string(), "TZS".to_string(), "TSh".to_string()),
            togo: ("CFA Franc".to_string(), "XOF".to_string(), "CFA".to_string()),
            tunisia: ("Dinar".to_string(), "TND".to_string(), "د.ت".to_string()),
            uganda: ("Shilling".to_string(), "UGX".to_string(), "USh".to_string()),
            zambia: ("Kwacha".to_string(), "ZMW".to_string(), "ZK".to_string()),
            zimbabwe: ("Dollar".to_string(), "USD".to_string(), "$".to_string()),

            // Would you like me to continue with North America next?
            antigua: ("Dollar".to_string(), "XCD".to_string(), "$".to_string()),
            bahamas: ("Dollar".to_string(), "BSD".to_string(), "$".to_string()),
            barbados: ("Dollar".to_string(), "BBD".to_string(), "$".to_string()),
            belize: ("Dollar".to_string(), "BZD".to_string(), "BZ$".to_string()),
            canada: ("Dollar".to_string(), "CAD".to_string(), "$".to_string()),
            costa_rica: ("Colón".to_string(), "CRC".to_string(), "₡".to_string()),
            cuba: ("Peso".to_string(), "CUP".to_string(), "₱".to_string()),
            dominica: ("Dollar".to_string(), "XCD".to_string(), "$".to_string()),
            dominican_rep: ("Peso".to_string(), "DOP".to_string(), "RD$".to_string()),
            el_salvador: ("Dollar".to_string(), "USD".to_string(), "$".to_string()),
            grenada: ("Dollar".to_string(), "XCD".to_string(), "$".to_string()),
            guatemala: ("Quetzal".to_string(), "GTQ".to_string(), "Q".to_string()),
            haiti: ("Gourde".to_string(), "HTG".to_string(), "G".to_string()),
            honduras: ("Lempira".to_string(), "HNL".to_string(), "L".to_string()),
            jamaica: ("Dollar".to_string(), "JMD".to_string(), "J$".to_string()),
            mexico: ("Peso".to_string(), "MXN".to_string(), "$".to_string()),
            nicaragua: ("Córdoba".to_string(), "NIO".to_string(), "C$".to_string()),
            panama: ("Balboa".to_string(), "PAB".to_string(), "B/.".to_string()),
            st_kitts: ("Dollar".to_string(), "XCD".to_string(), "$".to_string()),
            st_lucia: ("Dollar".to_string(), "XCD".to_string(), "$".to_string()),
            st_vincent: ("Dollar".to_string(), "XCD".to_string(), "$".to_string()),
            trinidad: ("Dollar".to_string(), "TTD".to_string(), "TT$".to_string()),
            usa: ("Dollar".to_string(), "USD".to_string(), "$".to_string()),

            // South America
            argentina: ("Peso".to_string(), "ARS".to_string(), "$".to_string()),
            bolivia: ("Boliviano".to_string(), "BOB".to_string(), "Bs.".to_string()),
            brazil: ("Real".to_string(), "BRL".to_string(), "R$".to_string()),
            chile: ("Peso".to_string(), "CLP".to_string(), "$".to_string()),
            colombia: ("Peso".to_string(), "COP".to_string(), "$".to_string()),
            ecuador: ("Dollar".to_string(), "USD".to_string(), "$".to_string()),
            guyana: ("Dollar".to_string(), "GYD".to_string(), "$".to_string()),
            paraguay: ("Guaraní".to_string(), "PYG".to_string(), "₲".to_string()),
            peru: ("Sol".to_string(), "PEN".to_string(), "S/".to_string()),
            suriname: ("Dollar".to_string(), "SRD".to_string(), "$".to_string()),
            uruguay: ("Peso".to_string(), "UYU".to_string(), "$U".to_string()),
            venezuela: ("Bolívar".to_string(), "VES".to_string(), "Bs.".to_string()),

            // Would you like me to finish with Oceania?
            australia: ("Dollar".to_string(), "AUD".to_string(), "$".to_string()),
            fiji: ("Dollar".to_string(), "FJD".to_string(), "$".to_string()),
            kiribati: ("Dollar".to_string(), "AUD".to_string(), "$".to_string()),
            marshall_islands: ("Dollar".to_string(), "USD".to_string(), "$".to_string()),
            micronesia: ("Dollar".to_string(), "USD".to_string(), "$".to_string()),
            nauru: ("Dollar".to_string(), "AUD".to_string(), "$".to_string()),
            new_zealand: ("Dollar".to_string(), "NZD".to_string(), "$".to_string()),
            palau: ("Dollar".to_string(), "USD".to_string(), "$".to_string()),
            papua_new_guinea: ("Kina".to_string(), "PGK".to_string(), "K".to_string()),
            samoa: ("Tala".to_string(), "WST".to_string(), "T".to_string()),
            solomon_islands: ("Dollar".to_string(), "SBD".to_string(), "$".to_string()),
            tonga: ("Paʻanga".to_string(), "TOP".to_string(), "T$".to_string()),
            tuvalu: ("Dollar".to_string(), "AUD".to_string(), "$".to_string()),
            vanuatu: ("Vatu".to_string(), "VUV".to_string(), "VT".to_string()),
        }
    }
}