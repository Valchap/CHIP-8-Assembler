#![cfg(test)]

use crate::parser::parse;

#[test]
fn test_cls() {
    assert_eq!(parse("CLS"), Ok(vec![0x00E0]));
}

#[test]
fn test_ret() {
    assert_eq!(parse("RET"), Ok(vec![0x00EE]));
}

#[test]
fn test_jmp() {
    assert_eq!(parse("JMP 1234A"), Ok(vec![0x14D2]));
}

#[test]
fn test_call() {
    assert_eq!(parse("CALL 1234A"), Ok(vec![0x24D2]));
}

#[test]
fn test_seq_v_nn() {
    assert_eq!(parse("SEQ V12 42B"), Ok(vec![0x3C2A]));
}

#[test]
fn test_sne_v_nn() {
    assert_eq!(parse("SNE V12 42B"), Ok(vec![0x4C2A]));
}

#[test]
fn test_seq_v_v() {
    assert_eq!(parse("SEQ V6 V12"), Ok(vec![0x56C0]));
}

#[test]
fn test_ld_v_nn() {
    assert_eq!(parse("LD V12 42B"), Ok(vec![0x6C2A]));
}

#[test]
fn test_add_v_nn() {
    assert_eq!(parse("ADD V12 42B"), Ok(vec![0x7C2A]));
}

#[test]
fn test_ld_v_v() {
    assert_eq!(parse("LD V6 V12"), Ok(vec![0x86C0]));
}

#[test]
fn test_or_v_v() {
    assert_eq!(parse("OR V6 V12"), Ok(vec![0x86C1]));
}

#[test]
fn test_and_v_v() {
    assert_eq!(parse("AND V6 V12"), Ok(vec![0x86C2]));
}

#[test]
fn test_xor_v_v() {
    assert_eq!(parse("XOR V6 V12"), Ok(vec![0x86C3]));
}

#[test]
fn test_add_v_v() {
    assert_eq!(parse("ADD V6 V12"), Ok(vec![0x86C4]));
}

#[test]
fn test_sub_v_v() {
    assert_eq!(parse("SUB V6 V12"), Ok(vec![0x86C5]));
}

#[test]
fn test_shr_v_v() {
    assert_eq!(parse("SHR V6 V12"), Ok(vec![0x86C6]));
}

#[test]
fn test_subn_v_v() {
    assert_eq!(parse("SUBN V6 V12"), Ok(vec![0x86C7]));
}

#[test]
fn test_shl_v_v() {
    assert_eq!(parse("SHL V6 V12"), Ok(vec![0x86CE]));
}

#[test]
fn test_sne_v_v() {
    assert_eq!(parse("SNE V6 V12"), Ok(vec![0x96C0]));
}

#[test]
fn test_ld_i_nnn() {
    assert_eq!(parse("LD I 1234A"), Ok(vec![0xA4D2]));
}

#[test]
fn test_jmpo_nnn() {
    assert_eq!(parse("JMPO 1234A"), Ok(vec![0xB4D2]));
}

#[test]
fn test_rnd_v_nn() {
    assert_eq!(parse("RND V12 42B"), Ok(vec![0xCC2A]));
}

#[test]
fn test_drw_v_v_n() {
    assert_eq!(parse("DRW V6 V12 15N"), Ok(vec![0xD6CF]));
}

#[test]
fn test_skp_v() {
    assert_eq!(parse("SKP V12"), Ok(vec![0xEC9E]));
}

#[test]
fn test_sknp_v() {
    assert_eq!(parse("SKNP V12"), Ok(vec![0xECA1]));
}

#[test]
fn test_ld_v_dt() {
    assert_eq!(parse("LD V12 DT"), Ok(vec![0xFC07]));
}

#[test]
fn test_ldk_v() {
    assert_eq!(parse("LDK V12"), Ok(vec![0xFC0A]));
}

#[test]
fn test_ld_dt_v() {
    assert_eq!(parse("LD DT V12"), Ok(vec![0xFC15]));
}

#[test]
fn test_ld_st_v() {
    assert_eq!(parse("LD ST V12"), Ok(vec![0xFC18]));
}

#[test]
fn test_add_i_v() {
    assert_eq!(parse("ADD I V12"), Ok(vec![0xFC1E]));
}

#[test]
fn test_spr_v() {
    assert_eq!(parse("SPR V12"), Ok(vec![0xFC29]));
}

#[test]
fn test_bcd_v() {
    assert_eq!(parse("BCD V12"), Ok(vec![0xFC33]));
}

#[test]
fn test_stn_v() {
    assert_eq!(parse("STN V12"), Ok(vec![0xFC55]));
}

#[test]
fn test_ldn_v() {
    assert_eq!(parse("LDN V12"), Ok(vec![0xFC65]));
}
