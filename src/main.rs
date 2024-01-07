// i2c_as7341.rs - Interface to AS7341 via i2c.

use std::thread;
use std::time::Duration;
use anyhow::Result;

use rppal::i2c::I2c;

fn main() -> Result<()> {

    println!("Initialize AS3741");

    // Get access to Raspberry Pi's I2C peripheral
    let mut i2c: I2c = I2c::new()?;

    // Inialize AS7341 for I2C communication
    i2c_as7341::as7341_init(&mut i2c)?;


    /* 
    println!("Turn on AS3741 LEDs"); 

    // Enable LEDs
    i2c_as7341::as7341_enable_leds(&i2c, true)?;

    // Turn on LEDs
    for n in 0..20_u8 {
        println!("LED level = {}", n);
        i2c_as7341::as7341_control_leds(&i2c, true, n)?;
        thread::sleep(Duration::from_millis(1000));
    }
    
    println!("Turn off AS3741 LEDs"); 

    // Turn off LEDs
    i2c_as7341::as7341_control_leds(&i2c, false, 0)?;

    // Disable LED
    i2c_as7341::as7341_enable_leds(&i2c, false)?;    
    

    println!("I2C bus ID is {}", i2c.bus());
    println!("I2C clock speed is {}", i2c.clock_speed()?);
    println!("I2C capabilities {:#?}", i2c.capabilities());

    thread::sleep(Duration::from_millis(1000));
    */
   
    // AS7341 ATIME Config
    i2c_as7341::as7341_atime_config(&i2c, 100)?;

    // AS7341 ASTEP Config
    i2c_as7341::as7341_astep_config(&i2c, 999)?;

    // AS7341 AGAIN Config
    i2c_as7341::as7341_again_config(&i2c, 0x06)?;

    // Enable AS7341
    i2c_as7341::as7341_enable(&i2c, true)?;    

    // Read channel data
    for _ in 0..10 {
        i2c_as7341::as7341_start_measure(&i2c, 0)?;
        i2c_as7341::as7341_read_spectral_data_one(&i2c)?;
        i2c_as7341::as7341_start_measure(&i2c, 1)?;
        i2c_as7341::as7341_read_spectral_data_two(&i2c)?;
        println!("");
        thread::sleep(Duration::from_millis(1000));
    }

    Ok(())

}