use std::fmt;

macro_rules! vehicles {
    ($($variant:ident, $id:expr, $cost:expr,)*) => {
        #[derive(Clone, Copy)]
        #[allow(clippy::upper_case_acronyms)]
        pub enum Vehicle {
            Random,
            Slow,
            Normal,
            Fast,
            Bike,
            PedalBike,
            FighterJet,
            JetSki,
            Tank,
            Sub,
            $($variant,)*
        }

        impl Vehicle {
            /// Human-readable display of this vehicle.
            pub fn display(&self) -> String {
                use self::Vehicle::*;

                match *self {
                    Random => format!("a random vehicle BlessRNG"),
                    Slow => format!("something slow PepeHands"),
                    Normal => format!("a normal car"),
                    Fast => format!("something fast monkaSpeed"),
                    Bike => format!("a bike!"),
                    PedalBike => format!("a... bike?"),
                    FighterJet => format!("a fighter jet DansRage"),
                    JetSki => format!("a jet ski DansRage"),
                    Tank => format!("a tank!"),
                    Sub => format!("a submarine!"),
                    $($variant => format!("a {}!", $id),)*
                }
            }

            /// Map an id to a vehicle.
            pub fn from_id(id: impl AsRef<str>) -> Option<Vehicle> {
                use self::Vehicle::*;

                match id.as_ref() {
                    "random" => Some(Random),
                    "slow" => Some(Slow),
                    "normal" => Some(Normal),
                    "fast" => Some(Fast),
                    "bike" => Some(Bike),
                    "pedalbike" => Some(PedalBike),
                    "fighter-jet" => Some(FighterJet),
                    "jet-ski" => Some(JetSki),
                    "tank" => Some(Tank),
                    "sub" => Some(Sub),
                    $($id => Some($variant),)*
                    _ => None,
                }
            }

            /**
             * Get the cost of a vehicle.
             */
            pub fn cost(&self) -> u32 {
                use self::Vehicle::*;

                match *self {
                    Random => 10,
                    Slow => 20,
                    Normal => 20,
                    Fast => 30,
                    Bike => 50,
                    PedalBike => 10,
                    FighterJet => 50,
                    JetSki => 10,
                    Tank => 50,
                    Sub => 20,
                    $($variant => $cost,)*
                }
            }

            /// Get a list of all cars.
            pub fn cars() -> Vec<Vehicle> {
                use self::Vehicle::*;
                vec![Slow, Normal, Fast]
            }

            /// Get a list of all vehicles.
            pub fn categories() -> Vec<Vehicle> {
                use self::Vehicle::*;

                vec![
                    Random, Slow, Normal, Fast, Bike, PedalBike, FighterJet, Blimp, JetSki, Tank,
                    Sub,
                ]
            }

            /// Get a random car.
            pub fn random_car() -> Vehicle {
                use rand::Rng as _;
                let mut rng = rand::thread_rng();
                let cars = Self::cars();
                cars[rng.gen_range(0..cars.len())]
            }
        }

        impl fmt::Display for Vehicle {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                use self::Vehicle::*;

                match *self {
                    Random => "random".fmt(fmt),
                    Slow => "slow".fmt(fmt),
                    Normal => "normal".fmt(fmt),
                    Fast => "fast".fmt(fmt),
                    Bike => "bike".fmt(fmt),
                    PedalBike => "pedalbike".fmt(fmt),
                    FighterJet => "fighter-jet".fmt(fmt),
                    JetSki => "jet-ski".fmt(fmt),
                    Tank => "tank".fmt(fmt),
                    Sub => "sub".fmt(fmt),
                    $($variant => $id.fmt(fmt),)*
                }
            }
        }
    }
}

vehicles! {
    Adder, "adder", 50,
    Airbus, "airbus", 50,
    Airtug, "airtug", 50,
    Akula, "akula", 50,
    Akuma, "akuma", 50,
    Alpha, "alpha", 50,
    AlphaZ1, "alphaz1", 50,
    Ambulance, "ambulance", 50,
    Annihilator, "annihilator", 50,
    APC, "apc", 50,
    Ardent, "ardent", 50,
    ArmyTanker, "armytanker", 50,
    ArmyTrailer, "armytrailer", 50,
    ArmyTrailer2, "armytrailer2", 50,
    Asea, "asea", 50,
    Asea2, "asea2", 50,
    Asterope, "asterope", 50,
    Autarch, "autarch", 50,
    Avarus, "avarus", 50,
    Avenger, "avenger", 50,
    Avenger2, "avenger2", 50,
    Bagger, "bagger", 50,
    BaleTrailer, "baletrailer", 50,
    Baller, "baller", 50,
    Baller2, "baller2", 50,
    Baller3, "baller3", 50,
    Baller4, "baller4", 50,
    Baller5, "baller5", 50,
    Baller6, "baller6", 50,
    Banshee, "banshee", 50,
    Banshee2, "banshee2", 50,
    Barracks, "barracks", 50,
    Barracks2, "barracks2", 50,
    Barracks3, "barracks3", 50,
    Barrage, "barrage", 50,
    Bati, "bati", 50,
    Bati2, "bati2", 50,
    Benson, "benson", 50,
    Besra, "besra", 50,
    BestiaGTS, "bestiagts", 50,
    BF400, "bf400", 50,
    BfInjection, "bfinjection", 50,
    Biff, "biff", 50,
    Bifta, "bifta", 50,
    Bison, "bison", 50,
    Bison2, "bison2", 50,
    Bison3, "bison3", 50,
    BJXL, "bjxl", 50,
    Blade, "blade", 50,
    Blazer, "blazer", 50,
    Blazer2, "blazer2", 50,
    Blazer3, "blazer3", 50,
    Blazer4, "blazer4", 50,
    Blazer5, "blazer5", 50,
    Blimp, "blimp", 50,
    Blimp2, "blimp2", 50,
    Blimp3, "blimp3", 50,
    Blista, "blista", 50,
    Blista2, "blista2", 50,
    Blista3, "blista3", 50,
    Bmx, "bmx", 50,
    BoatTrailer, "boattrailer", 50,
    BobcatXL, "bobcatxl", 50,
    Bodhi2, "bodhi2", 50,
    Bombushka, "bombushka", 50,
    Boxville, "boxville", 50,
    Boxville2, "boxville2", 50,
    Boxville3, "boxville3", 50,
    Boxville4, "boxville4", 50,
    Boxville5, "boxville5", 50,
    Brawler, "brawler", 50,
    Brickade, "brickade", 50,
    Brioso, "brioso", 50,
    Bruiser, "bruiser", 50,
    Bruiser2, "bruiser2", 50,
    Bruiser3, "bruiser3", 50,
    Brutus, "brutus", 50,
    Brutus2, "brutus2", 50,
    Brutus3, "brutus3", 50,
    BType, "btype", 50,
    BType2, "btype2", 50,
    BType3, "btype3", 50,
    Buccaneer, "buccaneer", 50,
    Buccaneer2, "buccaneer2", 50,
    Buffalo, "buffalo", 50,
    Buffalo2, "buffalo2", 50,
    Buffalo3, "buffalo3", 50,
    Bulldozer, "bulldozer", 50,
    Bullet, "bullet", 50,
    Burrito, "burrito", 50,
    Burrito2, "burrito2", 50,
    Burrito3, "burrito3", 50,
    Burrito4, "burrito4", 50,
    Burrito5, "burrito5", 50,
    Bus, "bus", 50,
    Buzzard, "buzzard", 50,
    Buzzard2, "buzzard2", 50,
    CableCar, "cablecar", 50,
    Caddy, "caddy", 50,
    Caddy2, "caddy2", 50,
    Caddy3, "caddy3", 50,
    Camper, "camper", 50,
    Caracara, "caracara", 50,
    Carbonizzare, "carbonizzare", 50,
    CarbonRS, "carbonrs", 50,
    Cargobob, "cargobob", 50,
    Cargobob2, "cargobob2", 50,
    Cargobob3, "cargobob3", 50,
    Cargobob4, "cargobob4", 50,
    CargoPlane, "cargoplane", 50,
    Casco, "casco", 50,
    Cavalcade, "cavalcade", 50,
    Cavalcade2, "cavalcade2", 50,
    Cerberus, "cerberus", 50,
    Cerberus2, "cerberus2", 50,
    Cerberus3, "cerberus3", 50,
    Cheburek, "cheburek", 50,
    Cheetah, "cheetah", 50,
    Cheetah2, "cheetah2", 50,
    Chernobog, "chernobog", 50,
    Chimera, "chimera", 50,
    Chino, "chino", 50,
    Chino2, "chino2", 50,
    Cliffhanger, "cliffhanger", 50,
    Clique, "clique", 50,
    Coach, "coach", 50,
    Cog55, "cog55", 50,
    Cog552, "cog552", 50,
    CogCabrio, "cogcabrio", 50,
    Cognoscenti, "cognoscenti", 50,
    Cognoscenti2, "cognoscenti2", 50,
    Comet2, "comet2", 50,
    Comet3, "comet3", 50,
    Comet4, "comet4", 50,
    Comet5, "comet5", 50,
    Contender, "contender", 50,
    Coquette, "coquette", 50,
    Coquette2, "coquette2", 50,
    Coquette3, "coquette3", 50,
    Cruiser, "cruiser", 50,
    Crusader, "crusader", 50,
    Cuban800, "cuban800", 50,
    Cutter, "cutter", 50,
    Cyclone, "cyclone", 50,
    Daemon, "daemon", 50,
    Daemon2, "daemon2", 50,
    Deathbike, "deathbike", 50,
    Deathbike2, "deathbike2", 50,
    Deathbike3, "deathbike3", 50,
    Defiler, "defiler", 50,
    Deluxo, "deluxo", 50,
    Deveste, "deveste", 50,
    Deviant, "deviant", 50,
    Diablous, "diablous", 50,
    Diablous2, "diablous2", 50,
    Dilettante, "dilettante", 50,
    Dilettante2, "dilettante2", 50,
    Dinghy, "dinghy", 50,
    Dinghy2, "dinghy2", 50,
    Dinghy3, "dinghy3", 50,
    Dinghy4, "dinghy4", 50,
    DLoader, "dloader", 50,
    DockTrailer, "docktrailer", 50,
    Docktug, "docktug", 50,
    Dodo, "dodo", 50,
    Dominator, "dominator", 50,
    Dominator2, "dominator2", 50,
    Dominator3, "dominator3", 50,
    Dominator4, "dominator4", 50,
    Dominator5, "dominator5", 50,
    Dominator6, "dominator6", 50,
    Double, "double", 50,
    Dubsta, "dubsta", 50,
    Dubsta2, "dubsta2", 50,
    Dubsta3, "dubsta3", 50,
    Dukes, "dukes", 50,
    Dukes2, "dukes2", 50,
    Dump, "dump", 50,
    Dune, "dune", 50,
    Dune2, "dune2", 50,
    Dune3, "dune3", 50,
    Dune4, "dune4", 50,
    Dune5, "dune5", 50,
    Duster, "duster", 50,
    Elegy, "elegy", 50,
    Elegy2, "elegy2", 50,
    Ellie, "ellie", 50,
    Emperor, "emperor", 50,
    Emperor2, "emperor2", 50,
    Emperor3, "emperor3", 50,
    Enduro, "enduro", 50,
    EntityXF, "entityxf", 50,
    EntityXXR, "entityxxr", 50,
    Esskey, "esskey", 50,
    Exemplar, "exemplar", 50,
    F620, "f620", 50,
    Faction, "faction", 50,
    Faction2, "faction2", 50,
    Faction3, "faction3", 50,
    Fagaloa, "fagaloa", 50,
    Faggio, "faggio", 50,
    Faggio2, "faggio2", 50,
    Faggio3, "faggio3", 50,
    FBI, "fbi", 50,
    FBI2, "fbi2", 50,
    FCR, "fcr", 50,
    FCR2, "fcr2", 50,
    Felon, "felon", 50,
    Felon2, "felon2", 50,
    Feltzer2, "feltzer2", 50,
    Feltzer3, "feltzer3", 50,
    FireTruck, "firetruck", 50,
    Fixter, "fixter", 50,
    FlashGT, "flashgt", 50,
    Flatbed, "flatbed", 50,
    FMJ, "fmj", 50,
    Forklift, "forklift", 50,
    FQ2, "fq2", 50,
    Freecrawler, "freecrawler", 50,
    Freight, "freight", 50,
    FreightCar, "freightcar", 50,
    FreightCont1, "freightcont1", 50,
    FreightCont2, "freightcont2", 50,
    FreightGrain, "freightgrain", 50,
    FreightTrailer, "freighttrailer", 50,
    Frogger, "frogger", 50,
    Frogger2, "frogger2", 50,
    Fugitive, "fugitive", 50,
    Furoregt, "furoregt", 50,
    Fusilade, "fusilade", 50,
    Futo, "futo", 50,
    Gargoyle, "gargoyle", 50,
    Gauntlet, "gauntlet", 30,
    Gauntlet2, "gauntlet2", 50,
    GB200, "gb200", 50,
    GBurrito, "gburrito", 50,
    GBurrito2, "gburrito2", 50,
    Glendale, "glendale", 50,
    GP1, "gp1", 50,
    GrainTrailer, "graintrailer", 50,
    Granger, "granger", 50,
    Gresley, "gresley", 50,
    GT500, "gt500", 50,
    Guardian, "guardian", 50,
    Habanero, "habanero", 50,
    Hakuchou, "hakuchou", 50,
    Hakuchou2, "hakuchou2", 50,
    HalfTrack, "halftrack", 50,
    Handler, "handler", 50,
    Hauler, "hauler", 50,
    Hauler2, "hauler2", 50,
    Havok, "havok", 50,
    Hermes, "hermes", 50,
    Hexer, "hexer", 50,
    Hotknife, "hotknife", 50,
    HotringSabre, "hotringsabre", 50,
    Howard, "howard", 50,
    Hunter, "hunter", 50,
    Huntley, "huntley", 50,
    Hustler, "hustler", 50,
    Hydra, "hydra", 60,
    Impaler, "impaler", 50,
    Impaler2, "impaler2", 50,
    Impaler3, "impaler3", 50,
    Impaler4, "impaler4", 50,
    Imperator, "imperator", 50,
    Imperator2, "imperator2", 50,
    Imperator3, "imperator3", 50,
    Infernus, "infernus", 50,
    Infernus2, "infernus2", 50,
    Ingot, "ingot", 50,
    Innovation, "innovation", 50,
    Insurgent, "insurgent", 50,
    Insurgent2, "insurgent2", 50,
    Insurgent3, "insurgent3", 50,
    Intruder, "intruder", 50,
    Issi2, "issi2", 50,
    Issi3, "issi3", 50,
    Issi4, "issi4", 50,
    Issi5, "issi5", 50,
    Issi6, "issi6", 50,
    ItaliGTB, "italigtb", 50,
    ItaliGTB2, "italigtb2", 50,
    ItaliGTO, "italigto", 50,
    Jackal, "jackal", 50,
    JB700, "jb700", 50,
    Jester, "jester", 50,
    Jester2, "jester2", 50,
    Jester3, "jester3", 50,
    Jet, "jet", 50,
    Jetmax, "jetmax", 50,
    Journey, "journey", 50,
    Kalahari, "kalahari", 50,
    Kamacho, "kamacho", 50,
    Khamelion, "khamelion", 50,
    Khanjari, "khanjari", 50,
    Kuruma, "kuruma", 50,
    Kuruma2, "kuruma2", 50,
    Landstalker, "landstalker", 50,
    Lazer, "lazer", 50,
    LE7B, "le7b", 50,
    Lectro, "lectro", 50,
    Lguard, "lguard", 50,
    Limo2, "limo2", 50,
    Lurcher, "lurcher", 50,
    Luxor, "luxor", 50,
    Luxor2, "luxor2", 50,
    Lynx, "lynx", 50,
    Mamba, "mamba", 50,
    Mammatus, "mammatus", 50,
    Manana, "manana", 50,
    Manchez, "manchez", 50,
    Marquis, "marquis", 50,
    Marshall, "marshall", 50,
    Massacro, "massacro", 50,
    Massacro2, "massacro2", 50,
    Maverick, "maverick", 50,
    Menacer, "menacer", 50,
    Mesa, "mesa", 50,
    Mesa2, "mesa2", 50,
    Mesa3, "mesa3", 50,
    MetroTrain, "metrotrain", 50,
    Michelli, "michelli", 50,
    Microlight, "microlight", 50,
    Miljet, "miljet", 50,
    Minivan, "minivan", 50,
    Minivan2, "minivan2", 50,
    Mixer, "mixer", 50,
    Mixer2, "mixer2", 50,
    Mogul, "mogul", 50,
    Molotok, "molotok", 50,
    Monroe, "monroe", 50,
    Monster, "monster", 50,
    Monster3, "monster3", 50,
    Monster4, "monster4", 50,
    Monster5, "monster5", 50,
    Moonbeam, "moonbeam", 50,
    Moonbeam2, "moonbeam2", 50,
    Mower, "mower", 50,
    Mule, "mule", 50,
    Mule2, "mule2", 50,
    Mule3, "mule3", 50,
    Mule4, "mule4", 50,
    Nemesis, "nemesis", 50,
    Neon, "neon", 50,
    Nero, "nero", 50,
    Nero2, "nero2", 50,
    Nightblade, "nightblade", 50,
    Nightshade, "nightshade", 50,
    NightShark, "nightshark", 50,
    Nimbus, "nimbus", 50,
    Ninef, "ninef", 50,
    Ninef2, "ninef2", 50,
    Nokota, "nokota", 50,
    Omnis, "omnis", 50,
    Oppressor, "oppressor", 50,
    Oppressor2, "oppressor2", 50,
    Oracle, "oracle", 50,
    Oracle2, "oracle2", 50,
    Osiris, "osiris", 50,
    Packer, "packer", 50,
    Panto, "panto", 50,
    Paradise, "paradise", 50,
    Pariah, "pariah", 50,
    Patriot, "patriot", 50,
    PBus, "pbus", 50,
    PBus2, "pbus2", 50,
    PCJ, "pcj", 50,
    Penetrator, "penetrator", 50,
    Penumbra, "penumbra", 50,
    Peyote, "peyote", 50,
    Pfister811, "pfister811", 50,
    Phantom, "phantom", 50,
    Phantom2, "phantom2", 50,
    Phantom3, "phantom3", 50,
    Phoenix, "phoenix", 50,
    Picador, "picador", 50,
    Pigalle, "pigalle", 50,
    Police, "police", 50,
    Police2, "police2", 50,
    Police3, "police3", 50,
    Police4, "police4", 50,
    Policeb, "policeb", 50,
    PoliceOld1, "policeold1", 50,
    PoliceOld2, "policeold2", 50,
    PoliceT, "policet", 50,
    Polmav, "polmav", 50,
    Pony, "pony", 50,
    Pony2, "pony2", 50,
    Pounder, "pounder", 50,
    Pounder2, "pounder2", 50,
    Prairie, "prairie", 50,
    Pranger, "pranger", 50,
    Predator, "predator", 50,
    Premier, "premier", 50,
    Primo, "primo", 50,
    Primo2, "primo2", 50,
    PropTrailer, "proptrailer", 50,
    Prototipo, "prototipo", 50,
    Pyro, "pyro", 50,
    Radi, "radi", 50,
    Raiden, "raiden", 50,
    RakeTrailer, "raketrailer", 50,
    RallyTruck, "rallytruck", 50,
    RancherXL, "rancherxl", 50,
    RancherXL2, "rancherxl2", 50,
    RapidGT, "rapidgt", 50,
    RapidGT2, "rapidgt2", 50,
    RapidGT3, "rapidgt3", 50,
    Raptor, "raptor", 50,
    RatBike, "ratbike", 50,
    RatLoader, "ratloader", 50,
    RatLoader2, "ratloader2", 50,
    RCBandito, "rcbandito", 50,
    Reaper, "reaper", 50,
    Rebel, "rebel", 50,
    Rebel2, "rebel2", 50,
    Regina, "regina", 50,
    RentalBus, "rentalbus", 50,
    Retinue, "retinue", 50,
    Revolter, "revolter", 50,
    Rhapsody, "rhapsody", 50,
    Rhino, "rhino", 50,
    Riata, "riata", 50,
    Riot, "riot", 50,
    Riot2, "riot2", 50,
    Ripley, "ripley", 50,
    Rocoto, "rocoto", 50,
    Rogue, "rogue", 50,
    Romero, "romero", 50,
    Rubble, "rubble", 50,
    Ruffian, "ruffian", 50,
    Ruiner, "ruiner", 50,
    Ruiner2, "ruiner2", 50,
    Ruiner3, "ruiner3", 50,
    Rumpo, "rumpo", 50,
    Rumpo2, "rumpo2", 50,
    Rumpo3, "rumpo3", 50,
    Ruston, "ruston", 50,
    SabreGT, "sabregt", 50,
    SabreGT2, "sabregt2", 50,
    Sadler, "sadler", 50,
    Sadler2, "sadler2", 50,
    Sanchez, "sanchez", 50,
    Sanchez2, "sanchez2", 50,
    Sanctus, "sanctus", 50,
    Sandking, "sandking", 50,
    Sandking2, "sandking2", 50,
    Savage, "savage", 50,
    Savestra, "savestra", 50,
    SC1, "sc1", 50,
    Scarab, "scarab", 50,
    Scarab2, "scarab2", 50,
    Scarab3, "scarab3", 50,
    Schafter2, "schafter2", 50,
    Schafter3, "schafter3", 50,
    Schafter4, "schafter4", 50,
    Schafter5, "schafter5", 50,
    Schafter6, "schafter6", 50,
    Schlagen, "schlagen", 50,
    Schwarzer, "schwarzer", 50,
    Scorcher, "scorcher", 50,
    Scramjet, "scramjet", 50,
    Scrap, "scrap", 50,
    Seabreeze, "seabreeze", 50,
    Seashark, "seashark", 50,
    Seashark2, "seashark2", 50,
    Seashark3, "seashark3", 50,
    SeaSparrow, "seasparrow", 50,
    Seminole, "seminole", 50,
    Sentinel, "sentinel", 50,
    Sentinel2, "sentinel2", 50,
    Sentinel3, "sentinel3", 50,
    Serrano, "serrano", 50,
    Seven70, "seven70", 50,
    Shamal, "shamal", 50,
    Sheava, "sheava", 50,
    Sheriff, "sheriff", 50,
    Sheriff2, "sheriff2", 50,
    Shotaro, "shotaro", 50,
    Skylift, "skylift", 50,
    SlamVan, "slamvan", 50,
    SlamVan2, "slamvan2", 50,
    SlamVan3, "slamvan3", 50,
    SlamVan4, "slamvan4", 50,
    SlamVan5, "slamvan5", 50,
    SlamVan6, "slamvan6", 50,
    Sovereign, "sovereign", 50,
    Specter, "specter", 50,
    Specter2, "specter2", 50,
    Speeder, "speeder", 50,
    Speeder2, "speeder2", 50,
    Speedo, "speedo", 50,
    Speedo2, "speedo2", 50,
    Speedo4, "speedo4", 50,
    Squalo, "squalo", 50,
    Stafford, "stafford", 50,
    Stalion, "stalion", 50,
    Stalion2, "stalion2", 50,
    Stanier, "stanier", 50,
    Starling, "starling", 50,
    Stinger, "stinger", 50,
    StingerGT, "stingergt", 50,
    Stockade, "stockade", 50,
    Stockade3, "stockade3", 50,
    Stratum, "stratum", 50,
    Streiter, "streiter", 50,
    Stretch, "stretch", 50,
    Strikeforce, "strikeforce", 50,
    Stromberg, "stromberg", 50,
    Stunt, "stunt", 50,
    Submersible, "submersible", 50,
    Submersible2, "submersible2", 50,
    Sultan, "sultan", 50,
    SultanRS, "sultanrs", 50,
    Suntrap, "suntrap", 50,
    Superd, "superd", 50,
    Supervolito, "supervolito", 50,
    Supervolito2, "supervolito2", 50,
    Surano, "surano", 50,
    Surfer, "surfer", 50,
    Surfer2, "surfer2", 50,
    Surge, "surge", 50,
    Swift, "swift", 50,
    Swift2, "swift2", 50,
    Swinger, "swinger", 50,
    T20, "t20", 50,
    Taco, "taco", 50,
    Tailgater, "tailgater", 50,
    Taipan, "taipan", 50,
    Tampa, "tampa", 50,
    Tampa2, "tampa2", 50,
    Tampa3, "tampa3", 50,
    Tanker, "tanker", 50,
    Tanker2, "tanker2", 50,
    TankerCar, "tankercar", 50,
    Taxi, "taxi", 50,
    Technical, "technical", 50,
    Technical2, "technical2", 50,
    Technical3, "technical3", 50,
    Tempesta, "tempesta", 50,
    Terrorbyte, "terrorbyte", 50,
    Tezeract, "tezeract", 50,
    Thrust, "thrust", 50,
    Thruster, "thruster", 50,
    TipTruck, "tiptruck", 50,
    TipTruck2, "tiptruck2", 50,
    Titan, "titan", 50,
    Torero, "torero", 50,
    Tornado, "tornado", 50,
    Tornado2, "tornado2", 50,
    Tornado3, "tornado3", 50,
    Tornado4, "tornado4", 50,
    Tornado5, "tornado5", 50,
    Tornado6, "tornado6", 50,
    Toro, "toro", 50,
    Toro2, "toro2", 50,
    Toros, "toros", 50,
    Tourbus, "tourbus", 50,
    TowTruck, "towtruck", 50,
    TowTruck2, "towtruck2", 50,
    TR2, "tr2", 50,
    TR3, "tr3", 50,
    TR4, "tr4", 50,
    Tractor, "tractor", 50,
    Tractor2, "tractor2", 50,
    Tractor3, "tractor3", 50,
    TrailerLarge, "trailerlarge", 50,
    TrailerLogs, "trailerlogs", 50,
    Trailers, "trailers", 50,
    Trailers2, "trailers2", 50,
    Trailers3, "trailers3", 50,
    Trailers4, "trailers4", 50,
    TrailerSmall, "trailersmall", 50,
    TrailerSmall2, "trailersmall2", 50,
    Trash, "trash", 50,
    Trash2, "trash2", 50,
    TRFlat, "trflat", 50,
    TriBike, "tribike", 50,
    TriBike2, "tribike2", 50,
    TriBike3, "tribike3", 50,
    TrophyTruck, "trophytruck", 50,
    TrophyTruck2, "trophytruck2", 50,
    Tropic, "tropic", 50,
    Tropic2, "tropic2", 50,
    Tropos, "tropos", 50,
    Tug, "tug", 50,
    Tula, "tula", 50,
    Tulip, "tulip", 50,
    Turismo2, "turismo2", 50,
    Turismor, "turismor", 50,
    TVTrailer, "tvtrailer", 50,
    Tyrant, "tyrant", 50,
    Tyrus, "tyrus", 50,
    UtilityTruck, "utilitytruck", 50,
    UtilityTruck2, "utilitytruck2", 50,
    UtilityTruck3, "utilitytruck3", 50,
    UtilliTruck, "utillitruck", 50,
    UtilliTruck2, "utillitruck2", 50,
    UtilliTruck3, "utillitruck3", 50,
    Vacca, "vacca", 50,
    Vader, "vader", 50,
    Vagner, "vagner", 50,
    Valkyrie, "valkyrie", 50,
    Valkyrie2, "valkyrie2", 50,
    Vamos, "vamos", 50,
    Velum, "velum", 50,
    Velum2, "velum2", 50,
    Verlierer2, "verlierer2", 50,
    Vestra, "vestra", 50,
    Vigero, "vigero", 50,
    Vigilante, "vigilante", 50,
    Vindicator, "vindicator", 50,
    Virgo, "virgo", 50,
    Virgo2, "virgo2", 50,
    Virgo3, "virgo3", 50,
    Viseris, "viseris", 50,
    Visione, "visione", 50,
    Volatol, "volatol", 50,
    Volatus, "volatus", 50,
    Voltic, "voltic", 50,
    Voltic2, "voltic2", 50,
    Voodoo, "voodoo", 50,
    Voodoo2, "voodoo2", 50,
    Vortex, "vortex", 50,
    Warrener, "warrener", 50,
    Washington, "washington", 50,
    Wastelander, "wastelander", 50,
    Windsor, "windsor", 50,
    Windsor2, "windsor2", 50,
    Wolfsbane, "wolfsbane", 50,
    XA21, "xa21", 50,
    XLS, "xls", 50,
    XLS2, "xls2", 50,
    Yosemite, "yosemite", 50,
    Youga, "youga", 50,
    Youga2, "youga2", 50,
    Z190, "z190", 50,
    Zentorno, "zentorno", 50,
    Zion, "zion", 50,
    Zion2, "zion2", 50,
    ZombieA, "zombiea", 50,
    ZombieB, "zombieb", 50,
    ZR380, "zr380", 50,
    ZR3802, "zr3802", 50,
    ZR3803, "zr3803", 50,
    ZType, "ztype", 50,
}
