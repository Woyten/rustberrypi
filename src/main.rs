extern crate sysfs_gpio;

use std::thread;
use std::time::Duration;
use sysfs_gpio::Direction;
use sysfs_gpio::Pin;
use sysfs_gpio::Result;

fn main() -> Result<()> {
    let my_led = Pin::new(17);
    my_led.export()?;
    loop {
        my_led.set_direction(Direction::Low)?;
        my_led.set_value(0)?;
        thread::sleep(Duration::from_millis(200));
        my_led.set_value(1)?;
        thread::sleep(Duration::from_millis(200));
    }
}
