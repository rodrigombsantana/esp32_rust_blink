#![no_std]
//desabilita a biblioteca standard no rust, reduzindo bastante o codigo compilado pois focamos aqui em bare-metal
#![no_main] // desabilita a utilização mandatoria de uma função chamada main

use esp_backtrace as _; //importa o backtrace do esp para gerir error (panic na linguagem Rust)
use esp_hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay, IO}; //importa metodos esp_hal
use esp_println::println; // importa um println especifico do esp

#[entry] //configuracao dizendo que a função a seguir é o ponto de entrada
fn main() -> ! {
    let peripherals = Peripherals::take(); //obtem acesso aos periféricos do esp32
    let system = peripherals.SYSTEM.split(); //recebe as informacao dos periféricos de sistema
    let clocks = ClockControl::max(system.clock_control).freeze(); //obtem acesso ao Clock
    let mut delay = Delay::new(&clocks); // obtem acesso ao struct Delay para exe

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX); // cria uma intancia do GPIO
    let mut led = io.pins.gpio2.into_push_pull_output(); // variavel led com GPIO2, push&pull output

    println!("Hello world e Blink!"); // imprime Hello World no serial/console do esp32
    loop {
        //looping infinito
        led.set_high().unwrap(); //liga LED
        println!("ON");
        delay.delay_ms(1000u32); //configura o delay para 1segundo do tipo unsigned int 32 bits
        led.set_low().unwrap(); //desliga LED
        println!("OFF");
        delay.delay_ms(1000u32);
    }
}
