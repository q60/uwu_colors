//! Named colors for completions building.

type ColorCollection = &'static [(&'static str, &'static str)];

// css named colors
const CSS: ColorCollection = &[
    ("aliceblue", "f0f8ff"),
    ("antiquewhite", "faebd7"),
    ("aqua", "00ffff"),
    ("aquamarine", "7fffd4"),
    ("azure", "f0ffff"),
    ("beige", "f5f5dc"),
    ("bisque", "ffe4c4"),
    ("black", "000000"),
    ("blanchedalmond", "ffebcd"),
    ("blue", "0000ff"),
    ("blueviolet", "8a2be2"),
    ("brown", "a52a2a"),
    ("burlywood", "deb887"),
    ("cadetblue", "5f9ea0"),
    ("chartreuse", "7fff00"),
    ("chocolate", "d2691e"),
    ("coral", "ff7f50"),
    ("cornflowerblue", "6495ed"),
    ("cornsilk", "fff8dc"),
    ("crimson", "dc143c"),
    ("cyan", "00ffff"),
    ("darkblue", "00008b"),
    ("darkcyan", "008b8b"),
    ("darkgoldenrod", "b8860b"),
    ("darkgray", "a9a9a9"),
    ("darkgreen", "006400"),
    ("darkgrey", "a9a9a9"),
    ("darkkhaki", "bdb76b"),
    ("darkmagenta", "8b008b"),
    ("darkolivegreen", "556b2f"),
    ("darkorange", "ff8c00"),
    ("darkorchid", "9932cc"),
    ("darkred", "8b0000"),
    ("darksalmon", "e9967a"),
    ("darkseagreen", "8fbc8f"),
    ("darkslateblue", "483d8b"),
    ("darkslategray", "2f4f4f"),
    ("darkslategrey", "2f4f4f"),
    ("darkturquoise", "00ced1"),
    ("darkviolet", "9400d3"),
    ("deeppink", "ff1493"),
    ("deepskyblue", "00bfff"),
    ("dimgray", "696969"),
    ("dimgrey", "696969"),
    ("dodgerblue", "1e90ff"),
    ("firebrick", "b22222"),
    ("floralwhite", "fffaf0"),
    ("forestgreen", "228b22"),
    ("fuchsia", "ff00ff"),
    ("gainsboro", "dcdcdc"),
    ("ghostwhite", "f8f8ff"),
    ("gold", "ffd700"),
    ("goldenrod", "daa520"),
    ("gray", "808080"),
    ("green", "008000"),
    ("greenyellow", "adff2f"),
    ("grey", "808080"),
    ("honeydew", "f0fff0"),
    ("hotpink", "ff69b4"),
    ("indianred", "cd5c5c"),
    ("indigo", "4b0082"),
    ("ivory", "fffff0"),
    ("khaki", "f0e68c"),
    ("lavender", "e6e6fa"),
    ("lavenderblush", "fff0f5"),
    ("lawngreen", "7cfc00"),
    ("lemonchiffon", "fffacd"),
    ("lightblue", "add8e6"),
    ("lightcoral", "f08080"),
    ("lightcyan", "e0ffff"),
    ("lightgoldenrodyellow", "fafad2"),
    ("lightgray", "d3d3d3"),
    ("lightgreen", "90ee90"),
    ("lightgrey", "d3d3d3"),
    ("lightpink", "ffb6c1"),
    ("lightsalmon", "ffa07a"),
    ("lightseagreen", "20b2aa"),
    ("lightskyblue", "87cefa"),
    ("lightslategray", "778899"),
    ("lightslategrey", "778899"),
    ("lightsteelblue", "b0c4de"),
    ("lightyellow", "ffffe0"),
    ("lime", "00ff00"),
    ("limegreen", "32cd32"),
    ("linen", "faf0e6"),
    ("magenta", "ff00ff"),
    ("maroon", "800000"),
    ("mediumaquamarine", "66cdaa"),
    ("mediumblue", "0000cd"),
    ("mediumorchid", "ba55d3"),
    ("mediumpurple", "9370db"),
    ("mediumseagreen", "3cb371"),
    ("mediumslateblue", "7b68ee"),
    ("mediumspringgreen", "00fa9a"),
    ("mediumturquoise", "48d1cc"),
    ("mediumvioletred", "c71585"),
    ("midnightblue", "191970"),
    ("mintcream", "f5fffa"),
    ("mistyrose", "ffe4e1"),
    ("moccasin", "ffe4b5"),
    ("navajowhite", "ffdead"),
    ("navy", "000080"),
    ("oldlace", "fdf5e6"),
    ("olive", "808000"),
    ("olivedrab", "6b8e23"),
    ("orange", "ffa500"),
    ("orangered", "ff4500"),
    ("orchid", "da70d6"),
    ("palegoldenrod", "eee8aa"),
    ("palegreen", "98fb98"),
    ("paleturquoise", "afeeee"),
    ("palevioletred", "db7093"),
    ("papayawhip", "ffefd5"),
    ("peachpuff", "ffdab9"),
    ("peru", "cd853f"),
    ("pink", "ffc0cb"),
    ("plum", "dda0dd"),
    ("powderblue", "b0e0e6"),
    ("purple", "800080"),
    ("rebeccapurple", "663399"),
    ("red", "ff0000"),
    ("rosybrown", "bc8f8f"),
    ("royalblue", "4169e1"),
    ("saddlebrown", "8b4513"),
    ("salmon", "fa8072"),
    ("sandybrown", "f4a460"),
    ("seagreen", "2e8b57"),
    ("seashell", "fff5ee"),
    ("sienna", "a0522d"),
    ("silver", "c0c0c0"),
    ("skyblue", "87ceeb"),
    ("slateblue", "6a5acd"),
    ("slategray", "708090"),
    ("slategrey", "708090"),
    ("snow", "fffafa"),
    ("springgreen", "00ff7f"),
    ("steelblue", "4682b4"),
    ("tan", "d2b48c"),
    ("teal", "008080"),
    ("thistle", "d8bfd8"),
    ("tomato", "ff6347"),
    ("transparent", "00000000"),
    ("turquoise", "40e0d0"),
    ("violet", "ee82ee"),
    ("wheat", "f5deb3"),
    ("white", "ffffff"),
    ("whitesmoke", "f5f5f5"),
    ("yellow", "ffff00"),
    ("yellowgreen", "9acd32"),
];

// colorhexa colors by name
const COLOR_HEXA: ColorCollection = &[
    ("Air Force blue", "5d8aa8"),
    ("Alice blue", "f0f8ff"),
    ("Alizarin crimson", "e32636"),
    ("Almond", "efdecd"),
    ("Amaranth", "e52b50"),
    ("Amber", "ffbf00"),
    ("American rose", "ff033e"),
    ("Amethyst", "9966cc"),
    ("Android Green", "a4c639"),
    ("Anti-flash white", "f2f3f4"),
    ("Antique brass", "cd9575"),
    ("Antique fuchsia", "915c83"),
    ("Antique white", "faebd7"),
    ("Ao", "008000"),
    ("Apple green", "8db600"),
    ("Apricot", "fbceb1"),
    ("Aqua", "00ffff"),
    ("Aquamarine", "7fffd4"),
    ("Army green", "4b5320"),
    ("Arylide yellow", "e9d66b"),
    ("Ash grey", "b2beb5"),
    ("Asparagus", "87a96b"),
    ("Atomic tangerine", "ff9966"),
    ("Auburn", "a52a2a"),
    ("Aureolin", "fdee00"),
    ("AuroMetalSaurus", "6e7f80"),
    ("Awesome", "ff2052"),
    ("Azure", "007fff"),
    ("Azure mist/web", "f0ffff"),
    ("Baby blue", "89cff0"),
    ("Baby blue eyes", "a1caf1"),
    ("Baby pink", "f4c2c2"),
    ("Ball Blue", "21abcd"),
    ("Banana Mania", "fae7b5"),
    ("Banana yellow", "ffe135"),
    ("Battleship grey", "848482"),
    ("Bazaar", "98777b"),
    ("Beau blue", "bcd4e6"),
    ("Beaver", "9f8170"),
    ("Beige", "f5f5dc"),
    ("Bisque", "ffe4c4"),
    ("Bistre", "3d2b1f"),
    ("Bittersweet", "fe6f5e"),
    ("Black", "000000"),
    ("Blanched Almond", "ffebcd"),
    ("Bleu de France", "318ce7"),
    ("Blizzard Blue", "ace5ee"),
    ("Blond", "faf0be"),
    ("Blue", "0000ff"),
    ("Blue Bell", "a2a2d0"),
    ("Blue Gray", "6699cc"),
    ("Blue green", "0d98ba"),
    ("Blue purple", "8a2be2"),
    ("Blue violet", "8a2be2"),
    ("Blush", "de5d83"),
    ("Bole", "79443b"),
    ("Bondi blue", "0095b6"),
    ("Bone", "e3dac9"),
    ("Boston University Red", "cc0000"),
    ("Bottle green", "006a4e"),
    ("Boysenberry", "873260"),
    ("Brandeis blue", "0070ff"),
    ("Brass", "b5a642"),
    ("Brick red", "cb4154"),
    ("Bright cerulean", "1dacd6"),
    ("Bright green", "66ff00"),
    ("Bright lavender", "bf94e4"),
    ("Bright maroon", "c32148"),
    ("Bright pink", "ff007f"),
    ("Bright turquoise", "08e8de"),
    ("Bright ube", "d19fe8"),
    ("Brilliant lavender", "f4bbff"),
    ("Brilliant rose", "ff55a3"),
    ("Brink pink", "fb607f"),
    ("British racing green", "004225"),
    ("Bronze", "cd7f32"),
    ("Brown", "a52a2a"),
    ("Bubble gum", "ffc1cc"),
    ("Bubbles", "e7feff"),
    ("Buff", "f0dc82"),
    ("Bulgarian rose", "480607"),
    ("Burgundy", "800020"),
    ("Burlywood", "deb887"),
    ("Burnt orange", "cc5500"),
    ("Burnt sienna", "e97451"),
    ("Burnt umber", "8a3324"),
    ("Byzantine", "bd33a4"),
    ("Byzantium", "702963"),
    ("CG Blue", "007aa5"),
    ("CG Red", "e03c31"),
    ("Cadet", "536872"),
    ("Cadet blue", "5f9ea0"),
    ("Cadet grey", "91a3b0"),
    ("Cadmium green", "006b3c"),
    ("Cadmium orange", "ed872d"),
    ("Cadmium red", "e30022"),
    ("Cadmium yellow", "fff600"),
    ("Café au lait", "a67b5b"),
    ("Café noir", "4b3621"),
    ("Cal Poly Pomona green", "1e4d2b"),
    ("Cambridge Blue", "a3c1ad"),
    ("Camel", "c19a6b"),
    ("Camouflage green", "78866b"),
    ("Canary", "ffff99"),
    ("Canary yellow", "ffef00"),
    ("Candy apple red", "ff0800"),
    ("Candy pink", "e4717a"),
    ("Capri", "00bfff"),
    ("Caput mortuum", "592720"),
    ("Cardinal", "c41e3a"),
    ("Caribbean green", "00cc99"),
    ("Carmine", "ff0040"),
    ("Carmine pink", "eb4c42"),
    ("Carmine red", "ff0038"),
    ("Carnation pink", "ffa6c9"),
    ("Carnelian", "b31b1b"),
    ("Carolina blue", "99badd"),
    ("Carrot orange", "ed9121"),
    ("Celadon", "ace1af"),
    ("Celeste", "b2ffff"),
    ("Celestial blue", "4997d0"),
    ("Cerise", "de3163"),
    ("Cerise pink", "ec3b83"),
    ("Cerulean", "007ba7"),
    ("Cerulean blue", "2a52be"),
    ("Chamoisee", "a0785a"),
    ("Champagne", "fad6a5"),
    ("Charcoal", "36454f"),
    ("Chartreuse", "7fff00"),
    ("Cherry", "de3163"),
    ("Cherry blossom pink", "ffb7c5"),
    ("Chestnut", "cd5c5c"),
    ("Chocolate", "d2691e"),
    ("Chrome yellow", "ffa700"),
    ("Cinereous", "98817b"),
    ("Cinnabar", "e34234"),
    ("Cinnamon", "d2691e"),
    ("Citrine", "e4d00a"),
    ("Classic rose", "fbcce7"),
    ("Cobalt", "0047ab"),
    ("Cocoa brown", "d2691e"),
    ("Coffee", "6f4e37"),
    ("Columbia blue", "9bddff"),
    ("Cool black", "002e63"),
    ("Cool grey", "8c92ac"),
    ("Copper", "b87333"),
    ("Copper rose", "996666"),
    ("Coquelicot", "ff3800"),
    ("Coral", "ff7f50"),
    ("Coral pink", "f88379"),
    ("Coral red", "ff4040"),
    ("Cordovan", "893f45"),
    ("Corn", "fbec5d"),
    ("Cornell Red", "b31b1b"),
    ("Cornflower", "9aceeb"),
    ("Cornflower blue", "6495ed"),
    ("Cornsilk", "fff8dc"),
    ("Cosmic latte", "fff8e7"),
    ("Cotton candy", "ffbcd9"),
    ("Cream", "fffdd0"),
    ("Crimson", "dc143c"),
    ("Crimson Red", "990000"),
    ("Crimson glory", "be0032"),
    ("Cyan", "00ffff"),
    ("Daffodil", "ffff31"),
    ("Dandelion", "f0e130"),
    ("Dark blue", "00008b"),
    ("Dark brown", "654321"),
    ("Dark byzantium", "5d3954"),
    ("Dark candy apple red", "a40000"),
    ("Dark cerulean", "08457e"),
    ("Dark chestnut", "986960"),
    ("Dark coral", "cd5b45"),
    ("Dark cyan", "008b8b"),
    ("Dark electric blue", "536878"),
    ("Dark goldenrod", "b8860b"),
    ("Dark gray", "a9a9a9"),
    ("Dark green", "013220"),
    ("Dark jungle green", "1a2421"),
    ("Dark khaki", "bdb76b"),
    ("Dark lava", "483c32"),
    ("Dark lavender", "734f96"),
    ("Dark magenta", "8b008b"),
    ("Dark midnight blue", "003366"),
    ("Dark olive green", "556b2f"),
    ("Dark orange", "ff8c00"),
    ("Dark orchid", "9932cc"),
    ("Dark pastel blue", "779ecb"),
    ("Dark pastel green", "03c03c"),
    ("Dark pastel purple", "966fd6"),
    ("Dark pastel red", "c23b22"),
    ("Dark pink", "e75480"),
    ("Dark powder blue", "003399"),
    ("Dark raspberry", "872657"),
    ("Dark red", "8b0000"),
    ("Dark salmon", "e9967a"),
    ("Dark scarlet", "560319"),
    ("Dark sea green", "8fbc8f"),
    ("Dark sienna", "3c1414"),
    ("Dark slate blue", "483d8b"),
    ("Dark slate gray", "2f4f4f"),
    ("Dark spring green", "177245"),
    ("Dark tan", "918151"),
    ("Dark tangerine", "ffa812"),
    ("Dark taupe", "483c32"),
    ("Dark terra cotta", "cc4e5c"),
    ("Dark turquoise", "00ced1"),
    ("Dark violet", "9400d3"),
    ("Dartmouth green", "00693e"),
    ("Davy grey", "555555"),
    ("Debian red", "d70a53"),
    ("Deep carmine", "a9203e"),
    ("Deep carmine pink", "ef3038"),
    ("Deep carrot orange", "e9692c"),
    ("Deep cerise", "da3287"),
    ("Deep champagne", "fad6a5"),
    ("Deep chestnut", "b94e48"),
    ("Deep coffee", "704241"),
    ("Deep fuchsia", "c154c1"),
    ("Deep jungle green", "004b49"),
    ("Deep lilac", "9955bb"),
    ("Deep magenta", "cc00cc"),
    ("Deep peach", "ffcba4"),
    ("Deep pink", "ff1493"),
    ("Deep saffron", "ff9933"),
    ("Deep sky blue", "00bfff"),
    ("Denim", "1560bd"),
    ("Desert", "c19a6b"),
    ("Desert sand", "edc9af"),
    ("Dim gray", "696969"),
    ("Dodger blue", "1e90ff"),
    ("Dogwood rose", "d71868"),
    ("Dollar bill", "85bb65"),
    ("Drab", "967117"),
    ("Duke blue", "00009c"),
    ("Earth yellow", "e1a95f"),
    ("Ecru", "c2b280"),
    ("Eggplant", "614051"),
    ("Eggshell", "f0ead6"),
    ("Egyptian blue", "1034a6"),
    ("Electric blue", "7df9ff"),
    ("Electric crimson", "ff003f"),
    ("Electric cyan", "00ffff"),
    ("Electric green", "00ff00"),
    ("Electric indigo", "6f00ff"),
    ("Electric lavender", "f4bbff"),
    ("Electric lime", "ccff00"),
    ("Electric purple", "bf00ff"),
    ("Electric ultramarine", "3f00ff"),
    ("Electric violet", "8f00ff"),
    ("Electric yellow", "ffff00"),
    ("Emerald", "50c878"),
    ("Eton blue", "96c8a2"),
    ("Fallow", "c19a6b"),
    ("Falu red", "801818"),
    ("Famous", "ff00ff"),
    ("Fandango", "b53389"),
    ("Fashion fuchsia", "f400a1"),
    ("Fawn", "e5aa70"),
    ("Feldgrau", "4d5d53"),
    ("Fern", "71bc78"),
    ("Fern green", "4f7942"),
    ("Ferrari Red", "ff2800"),
    ("Field drab", "6c541e"),
    ("Fire engine red", "ce2029"),
    ("Firebrick", "b22222"),
    ("Flame", "e25822"),
    ("Flamingo pink", "fc8eac"),
    ("Flavescent", "f7e98e"),
    ("Flax", "eedc82"),
    ("Floral white", "fffaf0"),
    ("Fluorescent orange", "ffbf00"),
    ("Fluorescent pink", "ff1493"),
    ("Fluorescent yellow", "ccff00"),
    ("Folly", "ff004f"),
    ("Forest green", "228b22"),
    ("French beige", "a67b5b"),
    ("French blue", "0072bb"),
    ("French lilac", "86608e"),
    ("French rose", "f64a8a"),
    ("Fuchsia", "ff00ff"),
    ("Fuchsia pink", "ff77ff"),
    ("Fulvous", "e48400"),
    ("Fuzzy Wuzzy", "cc6666"),
    ("Gainsboro", "dcdcdc"),
    ("Gamboge", "e49b0f"),
    ("Ghost white", "f8f8ff"),
    ("Ginger", "b06500"),
    ("Glaucous", "6082b6"),
    ("Glitter", "e6e8fa"),
    ("Gold", "ffd700"),
    ("Golden brown", "996515"),
    ("Golden poppy", "fcc200"),
    ("Golden yellow", "ffdf00"),
    ("Goldenrod", "daa520"),
    ("Granny Smith Apple", "a8e4a0"),
    ("Gray", "808080"),
    ("Gray asparagus", "465945"),
    ("Green", "00ff00"),
    ("Green Blue", "1164b4"),
    ("Green yellow", "adff2f"),
    ("Grullo", "a99a86"),
    ("Guppie green", "00ff7f"),
    ("Halayà úbe", "663854"),
    ("Han blue", "446ccf"),
    ("Han purple", "5218fa"),
    ("Hansa yellow", "e9d66b"),
    ("Harlequin", "3fff00"),
    ("Harvard crimson", "c90016"),
    ("Harvest Gold", "da9100"),
    ("Heart Gold", "808000"),
    ("Heliotrope", "df73ff"),
    ("Hollywood cerise", "f400a1"),
    ("Honeydew", "f0fff0"),
    ("Hooker green", "49796b"),
    ("Hot magenta", "ff1dce"),
    ("Hot pink", "ff69b4"),
    ("Hunter green", "355e3b"),
    ("Icterine", "fcf75e"),
    ("Inchworm", "b2ec5d"),
    ("India green", "138808"),
    ("Indian red", "cd5c5c"),
    ("Indian yellow", "e3a857"),
    ("Indigo", "4b0082"),
    ("International Klein Blue", "002fa7"),
    ("International orange", "ff4f00"),
    ("Iris", "5a4fcf"),
    ("Isabelline", "f4f0ec"),
    ("Islamic green", "009000"),
    ("Ivory", "fffff0"),
    ("Jade", "00a86b"),
    ("Jasmine", "f8de7e"),
    ("Jasper", "d73b3e"),
    ("Jazzberry jam", "a50b5e"),
    ("Jonquil", "fada5e"),
    ("June bud", "bdda57"),
    ("Jungle green", "29ab87"),
    ("KU Crimson", "e8000d"),
    ("Kelly green", "4cbb17"),
    ("Khaki", "c3b091"),
    ("La Salle Green", "087830"),
    ("Languid lavender", "d6cadd"),
    ("Lapis lazuli", "26619c"),
    ("Laser Lemon", "fefe22"),
    ("Laurel green", "a9ba9d"),
    ("Lava", "cf1020"),
    ("Lavender", "e6e6fa"),
    ("Lavender blue", "ccccff"),
    ("Lavender blush", "fff0f5"),
    ("Lavender gray", "c4c3d0"),
    ("Lavender indigo", "9457eb"),
    ("Lavender magenta", "ee82ee"),
    ("Lavender mist", "e6e6fa"),
    ("Lavender pink", "fbaed2"),
    ("Lavender purple", "967bb6"),
    ("Lavender rose", "fba0e3"),
    ("Lawn green", "7cfc00"),
    ("Lemon", "fff700"),
    ("Lemon Yellow", "fff44f"),
    ("Lemon chiffon", "fffacd"),
    ("Lemon lime", "bfff00"),
    ("Light Crimson", "f56991"),
    ("Light Thulian pink", "e68fac"),
    ("Light apricot", "fdd5b1"),
    ("Light blue", "add8e6"),
    ("Light brown", "b5651d"),
    ("Light carmine pink", "e66771"),
    ("Light coral", "f08080"),
    ("Light cornflower blue", "93ccea"),
    ("Light cyan", "e0ffff"),
    ("Light fuchsia pink", "f984ef"),
    ("Light goldenrod yellow", "fafad2"),
    ("Light gray", "d3d3d3"),
    ("Light green", "90ee90"),
    ("Light khaki", "f0e68c"),
    ("Light pastel purple", "b19cd9"),
    ("Light pink", "ffb6c1"),
    ("Light salmon", "ffa07a"),
    ("Light salmon pink", "ff9999"),
    ("Light sea green", "20b2aa"),
    ("Light sky blue", "87cefa"),
    ("Light slate gray", "778899"),
    ("Light taupe", "b38b6d"),
    ("Light yellow", "ffffed"),
    ("Lilac", "c8a2c8"),
    ("Lime", "bfff00"),
    ("Lime green", "32cd32"),
    ("Lincoln green", "195905"),
    ("Linen", "faf0e6"),
    ("Lion", "c19a6b"),
    ("Liver", "534b4f"),
    ("Lust", "e62020"),
    ("MSU Green", "18453b"),
    ("Macaroni and Cheese", "ffbd88"),
    ("Magenta", "ff00ff"),
    ("Magic mint", "aaf0d1"),
    ("Magnolia", "f8f4ff"),
    ("Mahogany", "c04000"),
    ("Maize", "fbec5d"),
    ("Majorelle Blue", "6050dc"),
    ("Malachite", "0bda51"),
    ("Manatee", "979aaa"),
    ("Mango Tango", "ff8243"),
    ("Mantis", "74c365"),
    ("Maroon", "800000"),
    ("Mauve", "e0b0ff"),
    ("Mauve taupe", "915f6d"),
    ("Mauvelous", "ef98aa"),
    ("Maya blue", "73c2fb"),
    ("Meat brown", "e5b73b"),
    ("Medium Persian blue", "0067a5"),
    ("Medium aquamarine", "66ddaa"),
    ("Medium blue", "0000cd"),
    ("Medium candy apple red", "e2062c"),
    ("Medium carmine", "af4035"),
    ("Medium champagne", "f3e5ab"),
    ("Medium electric blue", "035096"),
    ("Medium jungle green", "1c352d"),
    ("Medium lavender magenta", "dda0dd"),
    ("Medium orchid", "ba55d3"),
    ("Medium purple", "9370db"),
    ("Medium red violet", "bb3385"),
    ("Medium sea green", "3cb371"),
    ("Medium slate blue", "7b68ee"),
    ("Medium spring bud", "c9dc87"),
    ("Medium spring green", "00fa9a"),
    ("Medium taupe", "674c47"),
    ("Medium teal blue", "0054b4"),
    ("Medium turquoise", "48d1cc"),
    ("Medium violet red", "c71585"),
    ("Melon", "fdbcb4"),
    ("Midnight blue", "191970"),
    ("Midnight green", "004953"),
    ("Mikado yellow", "ffc40c"),
    ("Mint", "3eb489"),
    ("Mint cream", "f5fffa"),
    ("Mint green", "98ff98"),
    ("Misty rose", "ffe4e1"),
    ("Moccasin", "faebd7"),
    ("Mode beige", "967117"),
    ("Moonstone blue", "73a9c2"),
    ("Mordant red 19", "ae0c00"),
    ("Moss green", "addfad"),
    ("Mountain Meadow", "30ba8f"),
    ("Mountbatten pink", "997a8d"),
    ("Mulberry", "c54b8c"),
    ("Munsell", "f2f3f4"),
    ("Mustard", "ffdb58"),
    ("Myrtle", "21421e"),
    ("Nadeshiko pink", "f6adc6"),
    ("Napier green", "2a8000"),
    ("Naples yellow", "fada5e"),
    ("Navajo white", "ffdead"),
    ("Navy blue", "000080"),
    ("Neon Carrot", "ffa343"),
    ("Neon fuchsia", "fe59c2"),
    ("Neon green", "39ff14"),
    ("Non-photo blue", "a4dded"),
    ("North Texas Green", "059033"),
    ("Ocean Boat Blue", "0077be"),
    ("Ochre", "cc7722"),
    ("Office green", "008000"),
    ("Old gold", "cfb53b"),
    ("Old lace", "fdf5e6"),
    ("Old lavender", "796878"),
    ("Old mauve", "673147"),
    ("Old rose", "c08081"),
    ("Olive", "808000"),
    ("Olive Drab", "6b8e23"),
    ("Olive Green", "bab86c"),
    ("Olivine", "9ab973"),
    ("Onyx", "0f0f0f"),
    ("Opera mauve", "b784a7"),
    ("Orange", "ffa500"),
    ("Orange Yellow", "f8d568"),
    ("Orange peel", "ff9f00"),
    ("Orange red", "ff4500"),
    ("Orchid", "da70d6"),
    ("Otter brown", "654321"),
    ("Outer Space", "414a4c"),
    ("Outrageous Orange", "ff6e4a"),
    ("Oxford Blue", "002147"),
    ("Pacific Blue", "1ca9c9"),
    ("Pakistan green", "006600"),
    ("Palatinate blue", "273be2"),
    ("Palatinate purple", "682860"),
    ("Pale aqua", "bcd4e6"),
    ("Pale blue", "afeeee"),
    ("Pale brown", "987654"),
    ("Pale carmine", "af4035"),
    ("Pale cerulean", "9bc4e2"),
    ("Pale chestnut", "ddadaf"),
    ("Pale copper", "da8a67"),
    ("Pale cornflower blue", "abcdef"),
    ("Pale gold", "e6be8a"),
    ("Pale goldenrod", "eee8aa"),
    ("Pale green", "98fb98"),
    ("Pale lavender", "dcd0ff"),
    ("Pale magenta", "f984e5"),
    ("Pale pink", "fadadd"),
    ("Pale plum", "dda0dd"),
    ("Pale red violet", "db7093"),
    ("Pale robin egg blue", "96ded1"),
    ("Pale silver", "c9c0bb"),
    ("Pale spring bud", "ecebbd"),
    ("Pale taupe", "bc987e"),
    ("Pale violet red", "db7093"),
    ("Pansy purple", "78184a"),
    ("Papaya whip", "ffefd5"),
    ("Paris Green", "50c878"),
    ("Pastel blue", "aec6cf"),
    ("Pastel brown", "836953"),
    ("Pastel gray", "cfcfc4"),
    ("Pastel green", "77dd77"),
    ("Pastel magenta", "f49ac2"),
    ("Pastel orange", "ffb347"),
    ("Pastel pink", "ffd1dc"),
    ("Pastel purple", "b39eb5"),
    ("Pastel red", "ff6961"),
    ("Pastel violet", "cb99c9"),
    ("Pastel yellow", "fdfd96"),
    ("Patriarch", "800080"),
    ("Payne grey", "536878"),
    ("Peach", "ffe5b4"),
    ("Peach puff", "ffdab9"),
    ("Peach yellow", "fadfad"),
    ("Pear", "d1e231"),
    ("Pearl", "eae0c8"),
    ("Pearl Aqua", "88d8c0"),
    ("Peridot", "e6e200"),
    ("Periwinkle", "ccccff"),
    ("Persian blue", "1c39bb"),
    ("Persian indigo", "32127a"),
    ("Persian orange", "d99058"),
    ("Persian pink", "f77fbe"),
    ("Persian plum", "701c1c"),
    ("Persian red", "cc3333"),
    ("Persian rose", "fe28a2"),
    ("Phlox", "df00ff"),
    ("Phthalo blue", "000f89"),
    ("Phthalo green", "123524"),
    ("Piggy pink", "fddde6"),
    ("Pine green", "01796f"),
    ("Pink", "ffc0cb"),
    ("Pink Flamingo", "fc74fd"),
    ("Pink Sherbet", "f78fa7"),
    ("Pink pearl", "e7accf"),
    ("Pistachio", "93c572"),
    ("Platinum", "e5e4e2"),
    ("Plum", "dda0dd"),
    ("Portland Orange", "ff5a36"),
    ("Powder blue", "b0e0e6"),
    ("Princeton orange", "ff8f00"),
    ("Prussian blue", "003153"),
    ("Psychedelic purple", "df00ff"),
    ("Puce", "cc8899"),
    ("Pumpkin", "ff7518"),
    ("Purple", "800080"),
    ("Purple Heart", "69359c"),
    ("Purple Mountain's Majesty", "9d81ba"),
    ("Purple mountain majesty", "9678b6"),
    ("Purple pizzazz", "fe4eda"),
    ("Purple taupe", "50404d"),
    ("Rackley", "5d8aa8"),
    ("Radical Red", "ff355e"),
    ("Raspberry", "e30b5d"),
    ("Raspberry glace", "915f6d"),
    ("Raspberry pink", "e25098"),
    ("Raspberry rose", "b3446c"),
    ("Raw Sienna", "d68a59"),
    ("Razzle dazzle rose", "ff33cc"),
    ("Razzmatazz", "e3256b"),
    ("Red", "ff0000"),
    ("Red Orange", "ff5349"),
    ("Red brown", "a52a2a"),
    ("Red violet", "c71585"),
    ("Rich black", "004040"),
    ("Rich carmine", "d70040"),
    ("Rich electric blue", "0892d0"),
    ("Rich lilac", "b666d2"),
    ("Rich maroon", "b03060"),
    ("Rifle green", "414833"),
    ("Robin's Egg Blue", "1fcecb"),
    ("Rose", "ff007f"),
    ("Rose bonbon", "f9429e"),
    ("Rose ebony", "674846"),
    ("Rose gold", "b76e79"),
    ("Rose madder", "e32636"),
    ("Rose pink", "ff66cc"),
    ("Rose quartz", "aa98a9"),
    ("Rose taupe", "905d5d"),
    ("Rose vale", "ab4e52"),
    ("Rosewood", "65000b"),
    ("Rosso corsa", "d40000"),
    ("Rosy brown", "bc8f8f"),
    ("Royal azure", "0038a8"),
    ("Royal blue", "4169e1"),
    ("Royal fuchsia", "ca2c92"),
    ("Royal purple", "7851a9"),
    ("Ruby", "e0115f"),
    ("Ruddy", "ff0028"),
    ("Ruddy brown", "bb6528"),
    ("Ruddy pink", "e18e96"),
    ("Rufous", "a81c07"),
    ("Russet", "80461b"),
    ("Rust", "b7410e"),
    ("Sacramento State green", "00563f"),
    ("Saddle brown", "8b4513"),
    ("Safety orange", "ff6700"),
    ("Saffron", "f4c430"),
    ("Saint Patrick Blue", "23297a"),
    ("Salmon", "ff8c69"),
    ("Salmon pink", "ff91a4"),
    ("Sand", "c2b280"),
    ("Sand dune", "967117"),
    ("Sandstorm", "ecd540"),
    ("Sandy brown", "f4a460"),
    ("Sandy taupe", "967117"),
    ("Sap green", "507d2a"),
    ("Sapphire", "0f52ba"),
    ("Satin sheen gold", "cba135"),
    ("Scarlet", "ff2400"),
    ("School bus yellow", "ffd800"),
    ("Screamin Green", "76ff7a"),
    ("Sea blue", "006994"),
    ("Sea green", "2e8b57"),
    ("Seal brown", "321414"),
    ("Seashell", "fff5ee"),
    ("Selective yellow", "ffba00"),
    ("Sepia", "704214"),
    ("Shadow", "8a795d"),
    ("Shamrock", "45cea2"),
    ("Shamrock green", "009e60"),
    ("Shocking pink", "fc0fc0"),
    ("Sienna", "882d17"),
    ("Silver", "c0c0c0"),
    ("Sinopia", "cb410b"),
    ("Skobeloff", "007474"),
    ("Sky blue", "87ceeb"),
    ("Sky magenta", "cf71af"),
    ("Slate blue", "6a5acd"),
    ("Slate gray", "708090"),
    ("Smalt", "003399"),
    ("Smokey topaz", "933d41"),
    ("Smoky black", "100c08"),
    ("Snow", "fffafa"),
    ("Spiro Disco Ball", "0fc0fc"),
    ("Spring bud", "a7fc00"),
    ("Spring green", "00ff7f"),
    ("Steel blue", "4682b4"),
    ("Stil de grain yellow", "fada5e"),
    ("Stizza", "990000"),
    ("Stormcloud", "008080"),
    ("Straw", "e4d96f"),
    ("Sunglow", "ffcc33"),
    ("Sunset", "fad6a5"),
    ("Sunset Orange", "fd5e53"),
    ("Tan", "d2b48c"),
    ("Tangelo", "f94d00"),
    ("Tangerine", "f28500"),
    ("Tangerine yellow", "ffcc00"),
    ("Taupe", "483c32"),
    ("Taupe gray", "8b8589"),
    ("Tawny", "cd5700"),
    ("Tea green", "d0f0c0"),
    ("Tea rose", "f4c2c2"),
    ("Teal", "008080"),
    ("Teal blue", "367588"),
    ("Teal green", "006d5b"),
    ("Terra cotta", "e2725b"),
    ("Thistle", "d8bfd8"),
    ("Thulian pink", "de6fa1"),
    ("Tickle Me Pink", "fc89ac"),
    ("Tiffany Blue", "0abab5"),
    ("Tiger eye", "e08d3c"),
    ("Timberwolf", "dbd7d2"),
    ("Titanium yellow", "eee600"),
    ("Tomato", "ff6347"),
    ("Toolbox", "746cc0"),
    ("Topaz", "ffc87c"),
    ("Tractor red", "fd0e35"),
    ("Trolley Grey", "808080"),
    ("Tropical rain forest", "00755e"),
    ("True Blue", "0073cf"),
    ("Tufts Blue", "417dc1"),
    ("Tumbleweed", "deaa88"),
    ("Turkish rose", "b57281"),
    ("Turquoise", "30d5c8"),
    ("Turquoise blue", "00ffef"),
    ("Turquoise green", "a0d6b4"),
    ("Tuscan red", "66424d"),
    ("Twilight lavender", "8a496b"),
    ("Tyrian purple", "66023c"),
    ("UA blue", "0033aa"),
    ("UA red", "d9004c"),
    ("UCLA Blue", "536895"),
    ("UCLA Gold", "ffb300"),
    ("UFO Green", "3cd070"),
    ("UP Forest green", "014421"),
    ("UP Maroon", "7b1113"),
    ("USC Cardinal", "990000"),
    ("USC Gold", "ffcc00"),
    ("Ube", "8878c3"),
    ("Ultra pink", "ff6fff"),
    ("Ultramarine", "120a8f"),
    ("Ultramarine blue", "4166f5"),
    ("Umber", "635147"),
    ("United Nations blue", "5b92e5"),
    ("University of California Gold", "b78727"),
    ("Unmellow Yellow", "ffff66"),
    ("Upsdell red", "ae2029"),
    ("Urobilin", "e1ad21"),
    ("Utah Crimson", "d3003f"),
    ("Vanilla", "f3e5ab"),
    ("Vegas gold", "c5b358"),
    ("Venetian red", "c80815"),
    ("Verdigris", "43b3ae"),
    ("Vermilion", "e34234"),
    ("Veronica", "a020f0"),
    ("Violet", "ee82ee"),
    ("Violet Blue", "324ab2"),
    ("Violet Red", "f75394"),
    ("Viridian", "40826d"),
    ("Vivid auburn", "922724"),
    ("Vivid burgundy", "9f1d35"),
    ("Vivid cerise", "da1d81"),
    ("Vivid tangerine", "ffa089"),
    ("Vivid violet", "9f00ff"),
    ("Warm black", "004242"),
    ("Waterspout", "00ffff"),
    ("Wenge", "645452"),
    ("Wheat", "f5deb3"),
    ("White", "ffffff"),
    ("White smoke", "f5f5f5"),
    ("Wild Strawberry", "ff43a4"),
    ("Wild Watermelon", "fc6c85"),
    ("Wild blue yonder", "a2add0"),
    ("Wine", "722f37"),
    ("Wisteria", "c9a0dc"),
    ("Xanadu", "738678"),
    ("Yale Blue", "0f4d92"),
    ("Yellow", "ffff00"),
    ("Yellow Orange", "ffae42"),
    ("Yellow green", "9acd32"),
    ("Zaffre", "0014a8"),
    ("Zinnwaldite brown", "2c1608"),
];

/// Named colors variants.
pub enum NamedColors {
    /// CSS named colors.
    Css,

    /// colorhexa colors by name.
    ColorHexa,
}

impl NamedColors {
    /// Returns a reference to one of color slices according to [`NamedColors`].
    pub fn get(&self) -> ColorCollection {
        match self {
            Self::Css => CSS,
            Self::ColorHexa => COLOR_HEXA,
        }
    }
}
