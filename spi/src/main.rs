use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

fn main() {
    spi_rppal();

    //rs_ws281x_out(18);

    //rs_ws281x_out(10);
}

fn rs_ws281x_out(pin: i32) {
    use rs_ws281x::{ControllerBuilder, ChannelBuilder, StripType};
    println!("Started");

    let mut controller = ControllerBuilder::new()
        .channel(
            0,
            ChannelBuilder::new()
                .pin(pin)
                .count(300)
                .strip_type(StripType::Ws2812)
                .brightness(255)
                .build(),
        )
        .build()
        .unwrap();

    let led = controller.leds_mut(0);
    for i in 0..300 {
        let i = i as usize;
        if i % 4 < 2 {
            led[i] = [216, 216, 216, 0];
        } else {
            led[i] = [12, 0, 179, 0];
        }
    }

    controller.render().unwrap();
    controller.wait().unwrap();

    println!("Done.");

}

fn spi_rppal(){
    println!("Started");

    let mut spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 800 * 1000, Mode::Mode0).unwrap();

    let colors = [
        ColorRGB(255, 0, 0),
        ColorRGB(0, 255, 0),
        ColorRGB(0, 0, 255),
        ColorRGB(0, 0, 0),
        ColorRGB(0, 0, 1),
        ColorRGB(0, 1, 0),
        ColorRGB(1, 0, 0),
        ColorRGB(0, 0, 0),
        ColorRGB(255, 255, 255)
    ];

    let mut buffer = Vec::new();
    colors
        .iter()
        .for_each(|c| {
            // swapping RGB to the GRB expected by LED
            buffer.extend_from_slice(&[c.1, c.0, c.2]);
        });

    let output = buffer
        .drain(..)
        .flat_map(|val| byte_to_spi_bytes(val).to_vec())
        .collect::<Vec<u8>>();

    spi.write(&output).unwrap();

    println!("Done.");
}

/// Stores color as a tuple of (Red, Green, Blue)
pub struct ColorRGB(pub u8, pub u8, pub u8);

// Convert panel bits into their SPI counterparts
// 0 -> 001
// 1 -> 011
pub fn byte_to_spi_bytes(input: u8) -> [u8; 3] {
    // first convert the u8 to 24 bits
    let mut bool_array = [false; 24];
    for bit_index in 0..8 {
        let bit = input & (1 << bit_index) != 0;
        let out_index = bit_index * 3;

        // first bit is always 0
        // this could be omitted because the array is initialized to false
        bool_array[out_index] = false;

        bool_array[out_index + 1] = bit;

        // last bit is always 1
        bool_array[out_index + 2] = true;
    }

    // then convert the 24 bits to three u8
    [
        bool_slice_to_u8(&bool_array[0..8]),
        bool_slice_to_u8(&bool_array[8..16]),
        bool_slice_to_u8(&bool_array[16..24]),
    ]
}

fn bool_slice_to_u8(input: &[bool]) -> u8 {
    if input.len() != 8 { panic!("bool to u8 conversion requires exactly 8 booleans") }

    let mut out = 0b0000_0000u8;

    for (carry_bit, flag) in input.iter().enumerate() {
        if *flag { out += 0b0000_0001u8 << carry_bit }
    }

    out
}