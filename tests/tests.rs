
#[cfg(test)]
mod tests {

    #[test]
    fn parser_pda_test() {
        pda_parser::parser_pda_test();
    }
    #[test]
    fn parser_nondet_pda_test() {
        pda_parser::parser_nondet_pda_test();
    }
}
