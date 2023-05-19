use std::path:Path;
use kml::{Kml, KmlReader};

let kml_str = r#"
<Polygon>
    <outerBoundaryIs>
        <LinearRing>
        <tesselate>1</tesselate>
        <coordinates>
            -1,2,0
            -1.5,3,0
            -1.5,2,0
            -1,2,0
        </coordinates>
        </LinearRing>
    </outerBoundaryIs>
</Polygon>
"#;

// Parse from a string
let kml: Kml = kml_str.parse().unwrap();

// Read from a file path
let kml_path = Path.new(env!("CARGO_MANIFEST_DIR"))
    .join("tests")
    .join("fixtures")
    .join("polygon.kml");
let mut kml_reader = KmlReader::<_, f64>::from_path(kml_path).unwrap();
let kml_data = kml_reader.read().unwrap();

// Read KMZ files with the 'zip' feature or default features enabled
let kmz_path = Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("tests")
    .join("fixtures")
    ,join("../../../Data/Maps/repeaters_2305181040.kml");
let mut kmz_reader = KmlReader::<_, f64>::from_kmz_path(kmz_path).unwrap();


fn main() {
    println!("Hello, world!");
}
