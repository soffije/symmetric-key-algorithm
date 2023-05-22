// Функция для прямого преобразования по алгоритму S-блока
fn s_block(input: u8) -> u8 {
    // Таблица констант для S-блока
    let s_table: [u8; 16] = [
        0b1110, 0b0100, 0b1101, 0b0001, 0b0010, 0b1111, 0b1011, 0b1000,
        0b0011, 0b1010, 0b0110, 0b1100, 0b0101, 0b1001, 0b0000, 0b0111,
    ];

    let row = (input & 0b1100) >> 2; // Получаем значение строки
    let column = input & 0b0011; // Получаем значение столбца

    s_table[(row * 4 + column) as usize] // Возвращаем выходное значение из таблицы
}

// Функция для обратного преобразования по алгоритму S-блока
fn s_block_inverse(input: u8) -> u8 {
    // Таблица констант для обратного преобразования
    let s_table_inverse: [u8; 16] = [
        0b1110, 0b0110, 0b0010, 0b1100, 0b0100, 0b1010, 0b0000, 0b1001,
        0b1000, 0b0101, 0b1101, 0b0001, 0b0111, 0b1011, 0b0011, 0b1111,
    ];

    let row = (input & 0b1100) >> 2; // Получаем значение строки
    let column = input & 0b0011; // Получаем значение столбца

    s_table_inverse[(row * 4 + column) as usize] // Возвращаем выходное значение из таблицы
}

// Функция для реализации P-блока
fn p_block(input: u8) -> u8 {
    let permutation_table: [u8; 8] = [2, 6, 3, 1, 4, 8, 5, 7]; // Таблица перестановки

    let mut output = 0;

    for i in 0..8 {
        let bit = (input >> (8 - permutation_table[i])) & 1; // Получаем бит с новой позиции
        output |= bit << i; // Записываем бит на позицию i в выходном числе
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s_block() {
        let input: u8 = 6; // Входное значение
        let expected_output: u8 = 11; // Ожидаемое выходное значение

        let output = s_block(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_s_block_inverse() {
        let input: u8 = 0b1010; // Входное значение
        let expected_output: u8 = 0b1101; // Ожидаемое выходное значение

        let output = s_block_inverse(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_p_block() {
        let input: u8 = 0b01010101; // Входное значение
        let expected_output: u8 = 0b00110011; // Ожидаемое выходное значение

        let output = p_block(input);
        assert_eq!(output, expected_output);
    }
}

