use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Address, Channel, Pca9685};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let (a5, a4, a3, a2, a1, a0) = (false, false, false, false, false, false);
    let address = (a5, a4, a3, a2, a1, a0);
    let mut pwm = Pca9685::new(dev, address).unwrap();
    pwm.set_prescale(100).unwrap();
    pwm.enable().unwrap();

    // Turn on channel 0 at 0 and off at 2047, which is 50% in the range `[0..4095]`.
    pwm.set_channel_on_off(Channel::C0, 0, 2047).unwrap();
}