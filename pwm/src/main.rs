use rs_ws281x::{ControllerBuilder, ChannelBuilder, StripType};

fn main() {
    println!("Started");

    let mut controller = ControllerBuilder::new()
        .channel(
            0,
            ChannelBuilder::new()
                .pin(18)
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