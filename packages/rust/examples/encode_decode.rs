use global_digital_address::{get_digi_pin, get_lat_lng_from_digipin, approx_cell_size_meters};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lat = 28.6139;
    let lon = 77.2090;

    let code = get_digi_pin(lat, lon, 10)?;
    println!("DIGIPIN: {}", code);

    let center = get_lat_lng_from_digipin(&code)?;
    println!("Center: {}, {}", center.latitude, center.longitude);

    println!("Approx cell size @10: {:.3} m", approx_cell_size_meters(10));
    Ok(())
}
