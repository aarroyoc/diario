use rand::prelude::*;

const fn get_table() -> [&'static str; 16] {
    [
        "cero", "uno", "dos", "tres", "cuatro", "cinco", "seis", "siete", "ocho", "nueve", "diez",
        "once", "doce", "trece", "catorce", "quince",
    ]
}

pub fn get_captcha() -> (&'static str, u8) {
    let table = get_table();
    let n = random::<u8>() % 16;
    (table[n as usize], n)
}
