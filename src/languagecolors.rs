use std::collections::HashMap;

pub fn colors() -> HashMap<String, String> {
    let mut lang_colors: HashMap<&str, &str> = HashMap::new();
    lang_colors.insert("1C Enterprise", "#814CCC");
    lang_colors.insert("2-Dimensional Array", "#38761D");
    lang_colors.insert("4D", "#004289");
    lang_colors.insert("ABAP", "#E8274B");
    lang_colors.insert("ABAP CDS", "#555e25");
    lang_colors.insert("AGS Script", "#B9D9FF");
    lang_colors.insert("AIDL", "#34EB6B");
    lang_colors.insert("AL", "#3AA2B5");
    lang_colors.insert("AMPL", "#E6EFBB");
    lang_colors.insert("ANTLR", "#9DC3FF");
    lang_colors.insert("API Blueprint", "#2ACCA8");
    lang_colors.insert("APL", "#5A8164");
    lang_colors.insert("ASP.NET", "#9400ff");
    lang_colors.insert("ATS", "#1ac620");
    lang_colors.insert("ActionScript", "#882B0F");
    lang_colors.insert("Ada", "#02f88c");
    lang_colors.insert("Adblock Filter List", "#800000");
    lang_colors.insert("Adobe Font Metrics", "#fa0f00");
    lang_colors.insert("Agda", "#315665");
    lang_colors.insert("Alloy", "#64C800");
    lang_colors.insert("Alpine Abuild", "#0D597F");
    lang_colors.insert("Altium Designer", "#A89663");
    lang_colors.insert("AngelScript", "#C7D7DC");
    lang_colors.insert("Ant Build System", "#A9157E");
    lang_colors.insert("Antlers", "#ff269e");
    lang_colors.insert("ApacheConf", "#d12127");
    lang_colors.insert("Apex", "#1797c0");
    lang_colors.insert("Apollo Guidance Computer", "#0B3D91");
    lang_colors.insert("AppleScript", "#101F1F");
    lang_colors.insert("Arc", "#aa2afe");
    lang_colors.insert("AsciiDoc", "#73a0c5");
    lang_colors.insert("AspectJ", "#a957b0");
    lang_colors.insert("Assembly", "#6E4C13");
    lang_colors.insert("Astro", "#ff5a03");
    lang_colors.insert("Asymptote", "#ff0000");
    lang_colors.insert("Augeas", "#9CC134");
    lang_colors.insert("AutoHotkey", "#6594b9");
    lang_colors.insert("AutoIt", "#1C3552");
    lang_colors.insert("Avro IDL", "#0040FF");
    lang_colors.insert("Awk", "#c30e9b");
    lang_colors.insert("BASIC", "#ff0000");
    lang_colors.insert("BQN", "#2b7067");
    lang_colors.insert("Ballerina", "#FF5000");
    lang_colors.insert("Batchfile", "#C1F12E");
    lang_colors.insert("Beef", "#a52f4e");
    lang_colors.insert("Berry", "#15A13C");
    lang_colors.insert("BibTeX", "#778899");
    lang_colors.insert("Bicep", "#519aba");
    lang_colors.insert("Bikeshed", "#5562ac");
    lang_colors.insert("Bison", "#6A463F");
    lang_colors.insert("BitBake", "#00bce4");
    lang_colors.insert("Blade", "#f7523f");
    lang_colors.insert("BlitzBasic", "#00FFAE");
    lang_colors.insert("BlitzMax", "#cd6400");
    lang_colors.insert("Bluespec", "#12223c");
    lang_colors.insert("Bluespec BH", "#12223c");
    lang_colors.insert("Boo", "#d4bec1");
    lang_colors.insert("Boogie", "#c80fa0");
    lang_colors.insert("Brainfuck", "#2F2530");
    lang_colors.insert("BrighterScript", "#66AABB");
    lang_colors.insert("Brightscript", "#662D91");
    lang_colors.insert("Browserslist", "#ffd539");
    lang_colors.insert("C", "#555555");
    lang_colors.insert("C#", "#178600");
    lang_colors.insert("C++", "#f34b7d");
    lang_colors.insert("CAP CDS", "#0092d1");
    lang_colors.insert("CLIPS", "#00A300");
    lang_colors.insert("CMake", "#DA3434");
    lang_colors.insert("COLLADA", "#F1A42B");
    lang_colors.insert("CSON", "#244776");
    lang_colors.insert("CSS", "#563d7c");
    lang_colors.insert("CSV", "#237346");
    lang_colors.insert("CUE", "#5886E1");
    lang_colors.insert("CWeb", "#00007a");
    lang_colors.insert("Cabal Config", "#483465");
    lang_colors.insert("Caddyfile", "#22b638");
    lang_colors.insert("Cadence", "#00ef8b");
    lang_colors.insert("Cairo", "#ff4a48");
    lang_colors.insert("CameLIGO", "#3be133");
    lang_colors.insert("Cap'n Proto", "#c42727");
    lang_colors.insert("Ceylon", "#dfa535");
    lang_colors.insert("Chapel", "#8dc63f");
    lang_colors.insert("ChucK", "#3f8000");
    lang_colors.insert("Circom", "#707575");
    lang_colors.insert("Cirru", "#ccccff");
    lang_colors.insert("Clarion", "#db901e");
    lang_colors.insert("Clarity", "#5546ff");
    lang_colors.insert("Classic ASP", "#6a40fd");
    lang_colors.insert("Clean", "#3F85AF");
    lang_colors.insert("Click", "#E4E6F3");
    lang_colors.insert("Clojure", "#db5855");
    lang_colors.insert("Closure Templates", "#0d948f");
    lang_colors.insert("Cloud Firestore Security Rules", "#FFA000");
    lang_colors.insert("CodeQL", "#140f46");
    lang_colors.insert("CoffeeScript", "#244776");
    lang_colors.insert("ColdFusion", "#ed2cd6");
    lang_colors.insert("ColdFusion CFC", "#ed2cd6");
    lang_colors.insert("Common Lisp", "#3fb68b");
    lang_colors.insert("Common Workflow Language", "#B5314C");
    lang_colors.insert("Component Pascal", "#B0CE4E");
    lang_colors.insert("Coq", "#d0b68c");
    lang_colors.insert("Crystal", "#000100");
    lang_colors.insert("Csound", "#1a1a1a");
    lang_colors.insert("Csound Document", "#1a1a1a");
    lang_colors.insert("Csound Score", "#1a1a1a");
    lang_colors.insert("Cuda", "#3A4E3A");
    lang_colors.insert("Curry", "#531242");
    lang_colors.insert("Cypher", "#34c0eb");
    lang_colors.insert("Cython", "#fedf5b");
    lang_colors.insert("D", "#ba595e");
    lang_colors.insert("D2", "#526ee8");
    lang_colors.insert("DM", "#447265");
    lang_colors.insert("Dafny", "#FFEC25");
    lang_colors.insert("Darcs Patch", "#8eff23");
    lang_colors.insert("Dart", "#00B4AB");
    lang_colors.insert("DataWeave", "#003a52");
    lang_colors.insert("Debian Package Control File", "#D70751");
    lang_colors.insert("DenizenScript", "#FBEE96");
    lang_colors.insert("Dhall", "#dfafff");
    lang_colors.insert("DirectX 3D File", "#aace60");
    lang_colors.insert("Dockerfile", "#384d54");
    lang_colors.insert("Dogescript", "#cca760");
    lang_colors.insert("Dotenv", "#e5d559");
    lang_colors.insert("Dylan", "#6c616e");
    lang_colors.insert("E", "#ccce35");
    lang_colors.insert("ECL", "#8a1267");
    lang_colors.insert("ECLiPSe", "#001d9d");
    lang_colors.insert("EJS", "#a91e50");
    lang_colors.insert("EQ", "#a78649");
    lang_colors.insert("Earthly", "#2af0ff");
    lang_colors.insert("Easybuild", "#069406");
    lang_colors.insert("Ecere Projects", "#913960");
    lang_colors.insert("Ecmarkup", "#eb8131");
    lang_colors.insert("Edge", "#0dffe0");
    lang_colors.insert("EdgeQL", "#31A7FF");
    lang_colors.insert("EditorConfig", "#fff1f2");
    lang_colors.insert("Eiffel", "#4d6977");
    lang_colors.insert("Elixir", "#6e4a7e");
    lang_colors.insert("Elm", "#60B5CC");
    lang_colors.insert("Elvish", "#55BB55");
    lang_colors.insert("Elvish Transcript", "#55BB55");
    lang_colors.insert("Emacs Lisp", "#c065db");
    lang_colors.insert("EmberScript", "#FFF4F3");
    lang_colors.insert("Erlang", "#B83998");
    lang_colors.insert("Euphoria", "#FF790B");
    lang_colors.insert("F#", "#b845fc");
    lang_colors.insert("F*", "#572e30");
    lang_colors.insert("FIGlet Font", "#FFDDBB");
    lang_colors.insert("FIRRTL", "#2f632f");
    lang_colors.insert("FLUX", "#88ccff");
    lang_colors.insert("Factor", "#636746");
    lang_colors.insert("Fancy", "#7b9db4");
    lang_colors.insert("Fantom", "#14253c");
    lang_colors.insert("Faust", "#c37240");
    lang_colors.insert("Fennel", "#fff3d7");
    lang_colors.insert("Filebench WML", "#F6B900");
    lang_colors.insert("Fluent", "#ffcc33");
    lang_colors.insert("Forth", "#341708");
    lang_colors.insert("Fortran", "#4d41b1");
    lang_colors.insert("Fortran Free Form", "#4d41b1");
    lang_colors.insert("FreeBasic", "#141AC9");
    lang_colors.insert("FreeMarker", "#0050b2");
    lang_colors.insert("Frege", "#00cafe");
    lang_colors.insert("Futhark", "#5f021f");
    lang_colors.insert("G-code", "#D08CF2");
    lang_colors.insert("GAML", "#FFC766");
    lang_colors.insert("GAMS", "#f49a22");
    lang_colors.insert("GAP", "#0000cc");
    lang_colors.insert("GCC Machine Description", "#FFCFAB");
    lang_colors.insert("GDScript", "#355570");
    lang_colors.insert("GEDCOM", "#003058");
    lang_colors.insert("GLSL", "#5686a5");
    lang_colors.insert("GSC", "#FF6800");
    lang_colors.insert("Game Maker Language", "#71b417");
    lang_colors.insert("Gemfile.lock", "#701516");
    lang_colors.insert("Gemini", "#ff6900");
    lang_colors.insert("Genero 4gl", "#63408e");
    lang_colors.insert("Genero per", "#d8df39");
    lang_colors.insert("Genie", "#fb855d");
    lang_colors.insert("Genshi", "#951531");
    lang_colors.insert("Gentoo Ebuild", "#9400ff");
    lang_colors.insert("Gentoo Eclass", "#9400ff");
    lang_colors.insert("Gerber Image", "#d20b00");
    lang_colors.insert("Gherkin", "#5B2063");
    lang_colors.insert("Git Attributes", "#F44D27");
    lang_colors.insert("Git Config", "#F44D27");
    lang_colors.insert("Git Revision List", "#F44D27");
    lang_colors.insert("Gleam", "#ffaff3");
    lang_colors.insert("Glimmer JS", "#F5835F");
    lang_colors.insert("Glimmer TS", "#3178c6");
    lang_colors.insert("Glyph", "#c1ac7f");
    lang_colors.insert("Gnuplot", "#f0a9f0");
    lang_colors.insert("Go", "#00ADD8");
    lang_colors.insert("Go Checksums", "#00ADD8");
    lang_colors.insert("Go Module", "#00ADD8");
    lang_colors.insert("Go Workspace", "#00ADD8");
    lang_colors.insert("Godot Resource", "#355570");
    lang_colors.insert("Golo", "#88562A");
    lang_colors.insert("Gosu", "#82937f");
    lang_colors.insert("Grace", "#615f8b");
    lang_colors.insert("Gradle", "#02303a");
    lang_colors.insert("Gradle Kotlin DSL", "#02303a");
    lang_colors.insert("Grammatical Framework", "#ff0000");
    lang_colors.insert("GraphQL", "#e10098");
    lang_colors.insert("Graphviz (DOT)", "#2596be");
    lang_colors.insert("Groovy", "#4298b8");
    lang_colors.insert("Groovy Server Pages", "#4298b8");
    lang_colors.insert("HAProxy", "#106da9");
    lang_colors.insert("HCL", "#844FBA");
    lang_colors.insert("HLSL", "#aace60");
    lang_colors.insert("HOCON", "#9ff8ee");
    lang_colors.insert("HTML", "#e34c26");
    lang_colors.insert("HTML+ECR", "#2e1052");
    lang_colors.insert("HTML+EEX", "#6e4a7e");
    lang_colors.insert("HTML+ERB", "#701516");
    lang_colors.insert("HTML+PHP", "#4f5d95");
    lang_colors.insert("HTML+Razor", "#512be4");
    lang_colors.insert("HTTP", "#005C9C");
    lang_colors.insert("HXML", "#f68712");
    lang_colors.insert("Hack", "#878787");
    lang_colors.insert("Haml", "#ece2a9");
    lang_colors.insert("Handlebars", "#f7931e");
    lang_colors.insert("Harbour", "#0e60e3");
    lang_colors.insert("Haskell", "#5e5086");
    lang_colors.insert("Haxe", "#df7900");
    lang_colors.insert("HiveQL", "#dce200");
    lang_colors.insert("HolyC", "#ffefaf");
    lang_colors.insert("Hosts File", "#308888");
    lang_colors.insert("Hy", "#7790B2");
    lang_colors.insert("IDL", "#a3522f");
    lang_colors.insert("IGOR Pro", "#0000cc");
    lang_colors.insert("INI", "#d1dbe0");
    lang_colors.insert("Idris", "#b30000");
    lang_colors.insert("Ignore List", "#000000");
    lang_colors.insert("ImageJ Macro", "#99AAFF");
    lang_colors.insert("Imba", "#16cec6");
    lang_colors.insert("Inno Setup", "#264b99");
    lang_colors.insert("Io", "#a9188d");
    lang_colors.insert("Ioke", "#078193");
    lang_colors.insert("Isabelle", "#FEFE00");
    lang_colors.insert("Isabelle ROOT", "#FEFE00");
    lang_colors.insert("J", "#9EEDFF");
    lang_colors.insert("JAR Manifest", "#b07219");
    lang_colors.insert("JCL", "#d90e09");
    lang_colors.insert("JFlex", "#DBCA00");
    lang_colors.insert("JSON", "#292929");
    lang_colors.insert("JSON with Comments", "#292929");
    lang_colors.insert("JSON5", "#267CB9");
    lang_colors.insert("JSONLD", "#0c479c");
    lang_colors.insert("JSONiq", "#40d47e");
    lang_colors.insert("Janet", "#0886a5");
    lang_colors.insert("Jasmin", "#d03600");
    lang_colors.insert("Java", "#b07219");
    lang_colors.insert("Java Properties", "#2A6277");
    lang_colors.insert("Java Server Pages", "#2A6277");
    lang_colors.insert("JavaScript", "#f1e05a");
    lang_colors.insert("JavaScript+ERB", "#f1e05a");
    lang_colors.insert("Jest Snapshot", "#15c213");
    lang_colors.insert("JetBrains MPS", "#21D789");
    lang_colors.insert("Jinja", "#a52a22");
    lang_colors.insert("Jison", "#56b3cb");
    lang_colors.insert("Jison Lex", "#56b3cb");
    lang_colors.insert("Jolie", "#843179");
    lang_colors.insert("Jsonnet", "#0064bd");
    lang_colors.insert("Julia", "#a270ba");
    lang_colors.insert("Julia REPL", "#a270ba");
    lang_colors.insert("Jupyter Notebook", "#DA5B0B");
    lang_colors.insert("Just", "#384d54");
    lang_colors.insert("KRL", "#28430A");
    lang_colors.insert("Kaitai Struct", "#773b37");
    lang_colors.insert("KakouneScript", "#6f8042");
    lang_colors.insert("KerboScript", "#41adf0");
    lang_colors.insert("KiCad Layout", "#2f4aab");
    lang_colors.insert("KiCad Legacy Layout", "#2f4aab");
    lang_colors.insert("KiCad Schematic", "#2f4aab");
    lang_colors.insert("Kotlin", "#A97BFF");
    lang_colors.insert("LFE", "#4C3023");
    lang_colors.insert("LLVM", "#185619");
    lang_colors.insert("LOLCODE", "#cc9900");
    lang_colors.insert("LSL", "#3d9970");
    lang_colors.insert("LabVIEW", "#fede06");
    lang_colors.insert("Lark", "#2980B9");
    lang_colors.insert("Lasso", "#999999");
    lang_colors.insert("Latte", "#f2a542");
    lang_colors.insert("Less", "#1d365d");
    lang_colors.insert("Lex", "#DBCA00");
    lang_colors.insert("LigoLANG", "#0e74ff");
    lang_colors.insert("LilyPond", "#9ccc7c");
    lang_colors.insert("Liquid", "#67b8de");
    lang_colors.insert("Literate Agda", "#315665");
    lang_colors.insert("Literate CoffeeScript", "#244776");
    lang_colors.insert("Literate Haskell", "#5e5086");
    lang_colors.insert("LiveScript", "#499886");
    lang_colors.insert("Logtalk", "#295b9a");
    lang_colors.insert("LookML", "#652B81");
    lang_colors.insert("Lua", "#000080");
    lang_colors.insert("Luau", "#00A2FF");
    lang_colors.insert("MATLAB", "#e16737");
    lang_colors.insert("MAXScript", "#00a6a6");
    lang_colors.insert("MDX", "#fcb32c");
    lang_colors.insert("MLIR", "#5EC8DB");
    lang_colors.insert("MQL4", "#62A8D6");
    lang_colors.insert("MQL5", "#4A76B8");
    lang_colors.insert("MTML", "#b7e1f4");
    lang_colors.insert("Macaulay2", "#d8ffff");
    lang_colors.insert("Makefile", "#427819");
    lang_colors.insert("Mako", "#7e858d");
    lang_colors.insert("Markdown", "#083fa1");
    lang_colors.insert("Marko", "#42bff2");
    lang_colors.insert("Mask", "#f97732");
    lang_colors.insert("Mathematica", "#dd1100");
    lang_colors.insert("Max", "#c4a79c");
    lang_colors.insert("Mercury", "#ff2b2b");
    lang_colors.insert("Mermaid", "#ff3670");
    lang_colors.insert("Meson", "#007800");
    lang_colors.insert("Metal", "#8f14e9");
    lang_colors.insert("MiniYAML", "#ff1111");
    lang_colors.insert("Mint", "#02b046");
    lang_colors.insert("Mirah", "#c7a938");
    lang_colors.insert("Modelica", "#de1d31");
    lang_colors.insert("Modula-2", "#10253f");
    lang_colors.insert("Modula-3", "#223388");
    lang_colors.insert("Mojo", "#ff4c1f");
    lang_colors.insert("Monkey C", "#8D6747");
    lang_colors.insert("MoonScript", "#ff4585");
    lang_colors.insert("Motoko", "#fbb03b");
    lang_colors.insert("Motorola 68K Assembly", "#005daa");
    lang_colors.insert("Move", "#4a137a");
    lang_colors.insert("Mustache", "#724b3b");
    lang_colors.insert("NCL", "#28431f");
    lang_colors.insert("NMODL", "#00356B");
    lang_colors.insert("NPM Config", "#cb3837");
    lang_colors.insert("NWScript", "#111522");
    lang_colors.insert("Nasal", "#1d2c4e");
    lang_colors.insert("Nearley", "#990000");
    lang_colors.insert("Nemerle", "#3d3c6e");
    lang_colors.insert("NetLinx", "#0aa0ff");
    lang_colors.insert("NetLinx+ERB", "#747faa");
    lang_colors.insert("NetLogo", "#ff6375");
    lang_colors.insert("NewLisp", "#87AED7");
    lang_colors.insert("Nextflow", "#3ac486");
    lang_colors.insert("Nginx", "#009639");
    lang_colors.insert("Nim", "#ffc200");
    lang_colors.insert("Nit", "#009917");
    lang_colors.insert("Nix", "#7e7eff");
    lang_colors.insert("Nu", "#c9df40");
    lang_colors.insert("NumPy", "#9C8AF9");
    lang_colors.insert("Nunjucks", "#3d8137");
    lang_colors.insert("Nushell", "#4E9906");
    lang_colors.insert("OASv2-json", "#85ea2d");
    lang_colors.insert("OASv2-yaml", "#85ea2d");
    lang_colors.insert("OASv3-json", "#85ea2d");
    lang_colors.insert("OASv3-yaml", "#85ea2d");
    lang_colors.insert("OCaml", "#ef7a08");
    lang_colors.insert("ObjectScript", "#424893");
    lang_colors.insert("Objective-C", "#438eff");
    lang_colors.insert("Objective-C++", "#6866fb");
    lang_colors.insert("Objective-J", "#ff0c5a");
    lang_colors.insert("Odin", "#60AFFE");
    lang_colors.insert("Omgrofl", "#cabbff");
    lang_colors.insert("Opal", "#f7ede0");
    lang_colors.insert("Open Policy Agent", "#7d9199");
    lang_colors.insert("OpenAPI Specification v2", "#85ea2d");
    lang_colors.insert("OpenAPI Specification v3", "#85ea2d");
    lang_colors.insert("OpenCL", "#ed2e2d");
    lang_colors.insert("OpenEdge ABL", "#5ce600");
    lang_colors.insert("OpenQASM", "#AA70FF");
    lang_colors.insert("OpenSCAD", "#e5cd45");
    lang_colors.insert("Option List", "#476732");
    lang_colors.insert("Org", "#77aa99");
    lang_colors.insert("Oxygene", "#cdd0e3");
    lang_colors.insert("Oz", "#fab738");
    lang_colors.insert("P4", "#7055b5");
    lang_colors.insert("PDDL", "#0d00ff");
    lang_colors.insert("PEG.js", "#234d6b");
    lang_colors.insert("PHP", "#4F5D95");
    lang_colors.insert("PLSQL", "#dad8d8");
    lang_colors.insert("PLpgSQL", "#336790");
    lang_colors.insert("POV-Ray SDL", "#6bac65");
    lang_colors.insert("Pact", "#F7A8B8");
    lang_colors.insert("Pan", "#cc0000");
    lang_colors.insert("Papyrus", "#6600cc");
    lang_colors.insert("Parrot", "#f3ca0a");
    lang_colors.insert("Pascal", "#E3F171");
    lang_colors.insert("Pawn", "#dbb284");
    lang_colors.insert("Pep8", "#C76F5B");
    lang_colors.insert("Perl", "#0298c3");
    lang_colors.insert("PicoLisp", "#6067af");
    lang_colors.insert("PigLatin", "#fcd7de");
    lang_colors.insert("Pike", "#005390");
    lang_colors.insert("Pip Requirements", "#FFD343");
    lang_colors.insert("Pkl", "#6b9543");
    lang_colors.insert("PlantUML", "#fbbd16");
    lang_colors.insert("PogoScript", "#d80074");
    lang_colors.insert("Polar", "#ae81ff");
    lang_colors.insert("Portugol", "#f8bd00");
    lang_colors.insert("PostCSS", "#dc3a0c");
    lang_colors.insert("PostScript", "#da291c");
    lang_colors.insert("PowerBuilder", "#8f0f8d");
    lang_colors.insert("PowerShell", "#012456");
    lang_colors.insert("Praat", "#c8506d");
    lang_colors.insert("Prisma", "#0c344b");
    lang_colors.insert("Processing", "#0096D8");
    lang_colors.insert("Procfile", "#3B2F63");
    lang_colors.insert("Prolog", "#74283c");
    lang_colors.insert("Promela", "#de0000");
    lang_colors.insert("Propeller Spin", "#7fa2a7");
    lang_colors.insert("Pug", "#a86454");
    lang_colors.insert("Puppet", "#302B6D");
    lang_colors.insert("PureBasic", "#5a6986");
    lang_colors.insert("PureScript", "#1D222D");
    lang_colors.insert("Pyret", "#ee1e10");
    lang_colors.insert("Python", "#3572A5");
    lang_colors.insert("Python console", "#3572A5");
    lang_colors.insert("Python traceback", "#3572A5");
    lang_colors.insert("Q#", "#fed659");
    lang_colors.insert("QML", "#44a51c");
    lang_colors.insert("Qt Script", "#00b841");
    lang_colors.insert("Quake", "#882233");
    lang_colors.insert("R", "#198CE7");
    lang_colors.insert("RAML", "#77d9fb");
    lang_colors.insert("RBS", "#701516");
    lang_colors.insert("RDoc", "#701516");
    lang_colors.insert("REXX", "#d90e09");
    lang_colors.insert("RMarkdown", "#198ce7");
    lang_colors.insert("RON", "#a62c00");
    lang_colors.insert("RPGLE", "#2BDE21");
    lang_colors.insert("RUNOFF", "#665a4e");
    lang_colors.insert("Racket", "#3c5caa");
    lang_colors.insert("Ragel", "#9d5200");
    lang_colors.insert("Raku", "#0000fb");
    lang_colors.insert("Rascal", "#fffaa0");
    lang_colors.insert("ReScript", "#ed5051");
    lang_colors.insert("Reason", "#ff5847");
    lang_colors.insert("ReasonLIGO", "#ff5847");
    lang_colors.insert("Rebol", "#358a5b");
    lang_colors.insert("Record Jar", "#0673ba");
    lang_colors.insert("Red", "#f50000");
    lang_colors.insert("Regular Expression", "#009a00");
    lang_colors.insert("Ren'Py", "#ff7f7f");
    lang_colors.insert("Rez", "#FFDAB3");
    lang_colors.insert("Ring", "#2D54CB");
    lang_colors.insert("Riot", "#A71E49");
    lang_colors.insert("RobotFramework", "#00c0b5");
    lang_colors.insert("Roc", "#7c38f5");
    lang_colors.insert("Roff", "#ecdebe");
    lang_colors.insert("Roff Manpage", "#ecdebe");
    lang_colors.insert("Rouge", "#cc0088");
    lang_colors.insert("RouterOS Script", "#DE3941");
    lang_colors.insert("Ruby", "#701516");
    lang_colors.insert("Rust", "#dea584");
    lang_colors.insert("SAS", "#B34936");
    lang_colors.insert("SCSS", "#c6538c");
    lang_colors.insert("SPARQL", "#0C4597");
    lang_colors.insert("SQF", "#3F3F3F");
    lang_colors.insert("SQL", "#e38c00");
    lang_colors.insert("SQLPL", "#e38c00");
    lang_colors.insert("SRecode Template", "#348a34");
    lang_colors.insert("STL", "#373b5e");
    lang_colors.insert("SVG", "#ff9900");
    lang_colors.insert("SaltStack", "#646464");
    lang_colors.insert("Sass", "#a53b70");
    lang_colors.insert("Scala", "#c22d40");
    lang_colors.insert("Scaml", "#bd181a");
    lang_colors.insert("Scenic", "#fdc700");
    lang_colors.insert("Scheme", "#1e4aec");
    lang_colors.insert("Scilab", "#ca0f21");
    lang_colors.insert("Self", "#0579aa");
    lang_colors.insert("ShaderLab", "#222c37");
    lang_colors.insert("Shell", "#89e051");
    lang_colors.insert("ShellCheck Config", "#cecfcb");
    lang_colors.insert("Shen", "#120F14");
    lang_colors.insert("Simple File Verification", "#C9BFED");
    lang_colors.insert("Singularity", "#64E6AD");
    lang_colors.insert("Slash", "#007eff");
    lang_colors.insert("Slice", "#003fa2");
    lang_colors.insert("Slim", "#2b2b2b");
    lang_colors.insert("Slint", "#2379F4");
    lang_colors.insert("SmPL", "#c94949");
    lang_colors.insert("Smalltalk", "#596706");
    lang_colors.insert("Smarty", "#f0c040");
    lang_colors.insert("Smithy", "#c44536");
    lang_colors.insert("Snakemake", "#419179");
    lang_colors.insert("Solidity", "#AA6746");
    lang_colors.insert("SourcePawn", "#f69e1d");
    lang_colors.insert("Squirrel", "#800000");
    lang_colors.insert("Stan", "#b2011d");
    lang_colors.insert("Standard ML", "#dc566d");
    lang_colors.insert("Starlark", "#76d275");
    lang_colors.insert("Stata", "#1a5f91");
    lang_colors.insert("StringTemplate", "#3fb34f");
    lang_colors.insert("Stylus", "#ff6347");
    lang_colors.insert("SubRip Text", "#9e0101");
    lang_colors.insert("SugarSS", "#2fcc9f");
    lang_colors.insert("SuperCollider", "#46390b");
    lang_colors.insert("Svelte", "#ff3e00");
    lang_colors.insert("Sway", "#00F58C");
    lang_colors.insert("Sweave", "#198ce7");
    lang_colors.insert("Swift", "#F05138");
    lang_colors.insert("SystemVerilog", "#DAE1C2");
    lang_colors.insert("TI Program", "#A0AA87");
    lang_colors.insert("TL-Verilog", "#C40023");
    lang_colors.insert("TLA", "#4b0079");
    lang_colors.insert("TOML", "#9c4221");
    lang_colors.insert("TSQL", "#e38c00");
    lang_colors.insert("TSV", "#237346");
    lang_colors.insert("TSX", "#3178c6");
    lang_colors.insert("TXL", "#0178b8");
    lang_colors.insert("Talon", "#333333");
    lang_colors.insert("Tcl", "#e4cc98");
    lang_colors.insert("TeX", "#3D6117");
    lang_colors.insert("Terra", "#00004c");
    lang_colors.insert("Terraform Template", "#7b42bb");
    lang_colors.insert("TextGrid", "#c8506d");
    lang_colors.insert("TextMate Properties", "#df66e4");
    lang_colors.insert("Textile", "#ffe7ac");
    lang_colors.insert("Thrift", "#D12127");
    lang_colors.insert("Toit", "#c2c9fb");
    lang_colors.insert("Turing", "#cf142b");
    lang_colors.insert("Twig", "#c1d026");
    lang_colors.insert("TypeScript", "#3178c6");
    lang_colors.insert("Typst", "#239dad");
    lang_colors.insert("Unified Parallel C", "#4e3617");
    lang_colors.insert("Unity3D Asset", "#222c37");
    lang_colors.insert("Uno", "#9933cc");
    lang_colors.insert("UnrealScript", "#a54c4d");
    lang_colors.insert("UrWeb", "#ccccee");
    lang_colors.insert("V", "#4f87c4");
    lang_colors.insert("VBA", "#867db1");
    lang_colors.insert("VBScript", "#15dcdc");
    lang_colors.insert("VCL", "#148AA8");
    lang_colors.insert("VHDL", "#adb2cb");
    lang_colors.insert("Vala", "#a56de2");
    lang_colors.insert("Valve Data Format", "#f26025");
    lang_colors.insert("Velocity Template Language", "#507cff");
    lang_colors.insert("Verilog", "#b2b7f8");
    lang_colors.insert("Vim Help File", "#199f4b");
    lang_colors.insert("Vim Script", "#199f4b");
    lang_colors.insert("Vim Snippet", "#199f4b");
    lang_colors.insert("Visual Basic .NET", "#945db7");
    lang_colors.insert("Visual Basic 6.0", "#2c6353");
    lang_colors.insert("Volt", "#1F1F1F");
    lang_colors.insert("Vue", "#41b883");
    lang_colors.insert("Vyper", "#2980b9");
    lang_colors.insert("WDL", "#42f1f4");
    lang_colors.insert("WGSL", "#1a5e9a");
    lang_colors.insert("Web Ontology Language", "#5b70bd");
    lang_colors.insert("WebAssembly", "#04133b");
    lang_colors.insert("WebAssembly Interface Type", "#6250e7");
    lang_colors.insert("Whiley", "#d5c397");
    lang_colors.insert("Wikitext", "#fc5757");
    lang_colors.insert("Windows Registry Entries", "#52d5ff");
    lang_colors.insert("Witcher Script", "#ff0000");
    lang_colors.insert("Wollok", "#a23738");
    lang_colors.insert("World of Warcraft Addon Data", "#f7e43f");
    lang_colors.insert("Wren", "#383838");
    lang_colors.insert("X10", "#4B6BEF");
    lang_colors.insert("XC", "#99DA07");
    lang_colors.insert("XML", "#0060ac");
    lang_colors.insert("XML Property List", "#0060ac");
    lang_colors.insert("XQuery", "#5232e7");
    lang_colors.insert("XSLT", "#EB8CEB");
    lang_colors.insert("Xojo", "#81bd41");
    lang_colors.insert("Xonsh", "#285EEF");
    lang_colors.insert("Xtend", "#24255d");
    lang_colors.insert("YAML", "#cb171e");
    lang_colors.insert("YARA", "#220000");
    lang_colors.insert("YASnippet", "#32AB90");
    lang_colors.insert("Yacc", "#4B6C4B");
    lang_colors.insert("Yul", "#794932");
    lang_colors.insert("ZAP", "#0d665e");
    lang_colors.insert("ZIL", "#dc75e5");
    lang_colors.insert("ZenScript", "#00BCD1");
    lang_colors.insert("Zephir", "#118f9e");
    lang_colors.insert("Zig", "#ec915c");
    lang_colors.insert("Zimpl", "#d67711");
    lang_colors.insert("crontab", "#ead7ac");
    lang_colors.insert("eC", "#913960");
    lang_colors.insert("fish", "#4aae47");
    lang_colors.insert("hoon", "#00b171");
    lang_colors.insert("jq", "#c7254e");
    lang_colors.insert("kvlang", "#1da6e0");
    lang_colors.insert("mIRC Script", "#3d57c3");
    lang_colors.insert("mcfunction", "#E22837");
    lang_colors.insert("mupad", "#244963");
    lang_colors.insert("nanorc", "#2d004d");
    lang_colors.insert("nesC", "#94B0C7");
    lang_colors.insert("ooc", "#b0b77e");
    lang_colors.insert("q", "#0040cd");
    lang_colors.insert("reStructuredText", "#141414");
    lang_colors.insert("sed", "#64b970");
    lang_colors.insert("templ", "#66D0DD");
    lang_colors.insert("wisp", "#7582D1");
    lang_colors.insert("xBase", "#403a40");
    
    return lang_colors.into_iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
}