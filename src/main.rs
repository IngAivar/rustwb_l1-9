fn set_bit(mut number: i64, index: usize, value: bool) -> i64 {
    // Создаем маску для установки бита
    let mask = 1 << index;

    // Если нужно установить бит в 1, делаем побитовое ИЛИ с маской
    if value {
        number |= mask;
    } else {
        // Если нужно установить бит в 0, инвертируем маску и делаем побитовое И
        number &= !mask;
    }

    number
}

fn main() {
    let num = 49; // Представим, что это наше число
    let bit_index = 3; // Хотим установить 3-й бит в 1
    let new_num = set_bit(num, bit_index, true);

    println!("Исходное число: {}", num);
    println!("Новое число: {}", new_num);
}