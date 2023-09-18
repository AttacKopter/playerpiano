use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Address, Channel, Pca9685};

static CONTROLLERS: Vec<(bool, bool, bool, bool, bool, bool)> = vec![
    (false, false, false, false, false, false),
    (false, false, false, false, false, true),
    (false, false, false, false, true, false),
    (false, false, false, false, true, true),
    (false, false, false, true, false, false),
    (false, false, false, true, false, true),
    (false, false, false, true, true, false),
];

fn test() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let (a5, a4, a3, a2, a1, a0) = (false, false, false, false, false, false);
    let address = (a5, a4, a3, a2, a1, a0);
    let mut pwm = Pca9685::new(dev, address).unwrap();
    pwm.set_prescale(100).unwrap();
    pwm.enable().unwrap();

    // Turn on channel 0 at 0 and off at 2047, which is 50% in the range `[0..4095]`.
    pwm.set_channel_on_off(Channel::C0, 0, 2047).unwrap();
}

fn zero() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    for controller in CONTROLLERS {
        let mut pwm = Pca9685::new(dev, controller).unwrap();
        pwm.set_prescale(100).unwrap();
        pwm.enable().unwrap();
        for i in 0..11 {
            move_motor(pwm, i, 90);
        }
    }

    fn move_motor(pwm: Pca9685, channel: u64, angle: u64) {
        let off = (35/9*angle)+100; 
        match channel {
            0 => pwm.set_channel_on_off(Channel::C0, 0, off),
            1 => pwm.set_channel_on_off(Channel::C1, 0, off),
            2 => pwm.set_channel_on_off(Channel::C2, 0, off),
            3 => pwm.set_channel_on_off(Channel::C3, 0, off),
            4 => pwm.set_channel_on_off(Channel::C4, 0, off),
            5 => pwm.set_channel_on_off(Channel::C5, 0, off),
            6 => pwm.set_channel_on_off(Channel::C6, 0, off),
            7 => pwm.set_channel_on_off(Channel::C7, 0, off),
            8 => pwm.set_channel_on_off(Channel::C8, 0, off),
            9 => pwm.set_channel_on_off(Channel::C9, 0, off),
            10 => pwm.set_channel_on_off(Channel::C10, 0, off),
            11 => pwm.set_channel_on_off(Channel::C11, 0, off),
            _ => panic!()
        }
    }
}