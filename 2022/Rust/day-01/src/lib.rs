pub fn process_part1(input: &str) -> String {
    // Chunked up the input by new lines
    let result = input
        .split("\n\n")
        .map(|elf_load| {
            // elf_load is the chunk of the listed numbers
            elf_load
                .lines() // On new line
                .map(|item| item.parse::<u32>().unwrap()) // Mapping each line, parsing each. -> ::<u32> turbofish syntax. Telling parse to return a u32 -> Then we have to unwrap that
                .sum::<u32>()
        }) // We are summing the values during this map
        .max() // Returns a result itself
        .unwrap(); // Unwrapping that result
    result.to_string()
}

// This is how you test in Rust
#[cfg(test)] // Config test
mod tests {
    // Module tests
    use super::*;

    #[test] // Test Macro
    fn it_works() {
        let input: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let result = process_part1(input);
        assert_eq!(result, "24000");
    }
}
