use serde_json::Value;

pub(super) trait Cmp {
    fn cmp_bool(&self, v1: bool, v2: bool) -> bool;

    fn cmp_f64(&self, v1: f64, v2: f64) -> bool;

    fn cmp_string(&self, v1: &str, v2: &str) -> bool;

    fn cmp_json<'a>(&self, v1: &[&'a Value], v2: &[&'a Value]) -> Vec<&'a Value>;

    fn default(&self) -> bool {
        false
    }
}

pub(super) struct CmpEq;

impl Cmp for CmpEq {
    fn cmp_bool(&self, v1: bool, v2: bool) -> bool {
        v1 == v2
    }

    fn cmp_f64(&self, v1: f64, v2: f64) -> bool {
        (v1 - v2).abs() == 0_f64
    }

    fn cmp_string(&self, v1: &str, v2: &str) -> bool {
        v1 == v2
    }

    fn cmp_json<'a>(&self, v1: &[&'a Value], v2: &[&'a Value]) -> Vec<&'a Value> {
        let mut ret = vec![];

        for a in v1 {
            for b in v2 {
                if a == b {
                    ret.push(*a);
                }
            }
        }

        ret
    }
}

pub(super) struct CmpNe;

impl Cmp for CmpNe {
    fn cmp_bool(&self, v1: bool, v2: bool) -> bool {
        v1 != v2
    }

    fn cmp_f64(&self, v1: f64, v2: f64) -> bool {
        (v1 - v2).abs() != 0_f64
    }

    fn cmp_string(&self, v1: &str, v2: &str) -> bool {
        v1 != v2
    }

    fn cmp_json<'a>(&self, v1: &[&'a Value], v2: &[&'a Value]) -> Vec<&'a Value> {
        let mut ret = vec![];

        for a in v1 {
            for b in v2 {
                if a != b {
                    ret.push(*a);
                }
            }
        }

        ret
    }
}

pub(super) struct CmpGt;

impl Cmp for CmpGt {
    fn cmp_bool(&self, v1: bool, v2: bool) -> bool {
        v1 & !v2
    }

    fn cmp_f64(&self, v1: f64, v2: f64) -> bool {
        v1 > v2
    }

    fn cmp_string(&self, v1: &str, v2: &str) -> bool {
        v1 > v2
    }

    fn cmp_json<'a>(&self, _: &[&'a Value], _: &[&'a Value]) -> Vec<&'a Value> {
        Vec::new()
    }
}

pub(super) struct CmpGe;

impl Cmp for CmpGe {
    fn cmp_bool(&self, v1: bool, v2: bool) -> bool {
        v1 >= v2
    }

    fn cmp_f64(&self, v1: f64, v2: f64) -> bool {
        v1 >= v2
    }

    fn cmp_string(&self, v1: &str, v2: &str) -> bool {
        v1 >= v2
    }

    fn cmp_json<'a>(&self, _: &[&'a Value], _: &[&'a Value]) -> Vec<&'a Value> {
        Vec::new()
    }
}

pub(super) struct CmpLt;

impl Cmp for CmpLt {
    fn cmp_bool(&self, v1: bool, v2: bool) -> bool {
        !v1 & v2
    }

    fn cmp_f64(&self, v1: f64, v2: f64) -> bool {
        v1 < v2
    }

    fn cmp_string(&self, v1: &str, v2: &str) -> bool {
        v1 < v2
    }

    fn cmp_json<'a>(&self, _: &[&'a Value], _: &[&'a Value]) -> Vec<&'a Value> {
        Vec::new()
    }
}

pub(super) struct CmpLe;

impl Cmp for CmpLe {
    fn cmp_bool(&self, v1: bool, v2: bool) -> bool {
        v1 <= v2
    }

    fn cmp_f64(&self, v1: f64, v2: f64) -> bool {
        v1 <= v2
    }

    fn cmp_string(&self, v1: &str, v2: &str) -> bool {
        v1 <= v2
    }

    fn cmp_json<'a>(&self, _: &[&'a Value], _: &[&'a Value]) -> Vec<&'a Value> {
        Vec::new()
    }
}

pub(super) struct CmpAnd;

impl Cmp for CmpAnd {
    fn cmp_bool(&self, v1: bool, v2: bool) -> bool {
        v1 && v2
    }

    fn cmp_f64(&self, _v1: f64, _v2: f64) -> bool {
        true
    }

    fn cmp_string(&self, v1: &str, v2: &str) -> bool {
        !v1.is_empty() && !v2.is_empty()
    }

    fn cmp_json<'a>(&self, v1: &[&'a Value], v2: &[&'a Value]) -> Vec<&'a Value> {
        CmpEq.cmp_json(v1, v2)
    }
}

pub(super) struct CmpOr;

impl Cmp for CmpOr {
    fn cmp_bool(&self, v1: bool, v2: bool) -> bool {
        v1 || v2
    }

    fn cmp_f64(&self, _v1: f64, _v2: f64) -> bool {
        true
    }

    fn cmp_string(&self, v1: &str, v2: &str) -> bool {
        !v1.is_empty() || !v2.is_empty()
    }

    fn cmp_json<'a>(&self, v1: &[&'a Value], v2: &[&'a Value]) -> Vec<&'a Value> {
        let mut ret = [v1, v2].concat();

        for x in (0..ret.len()).rev() {
            for y in (x+1..ret.len()).rev() {
                if ret[x] == ret[y] {
                    ret.remove(y);
                }
            }
        }

        ret
    }
}


#[cfg(test)]
mod cmp_inner_tests {
    use serde_json::Value;

    use select::cmp::*;

    #[test]
    fn cmp_eq() {
        let cmp_fn = CmpEq;
        assert_eq!(cmp_fn.default(), false);
        assert_eq!(cmp_fn.cmp_bool(true, false), false);
        assert_eq!(cmp_fn.cmp_bool(true, true), true);
        assert_eq!(cmp_fn.cmp_f64(0.1, 0.1), true);
        assert_eq!(cmp_fn.cmp_f64(0.1, 0.2), false);
        assert_eq!(cmp_fn.cmp_string("1", "1"), true);
        assert_eq!(cmp_fn.cmp_string("1", "2"), false);
    }

    #[test]
    fn cmp_ne() {
        let cmp_fn = CmpNe;
        assert_eq!(cmp_fn.default(), false);
        assert_eq!(cmp_fn.cmp_bool(true, false), true);
        assert_eq!(cmp_fn.cmp_bool(true, true), false);
        assert_eq!(cmp_fn.cmp_f64(0.1, 0.1), false);
        assert_eq!(cmp_fn.cmp_f64(0.1, 0.2), true);
        assert_eq!(cmp_fn.cmp_string("1", "1"), false);
        assert_eq!(cmp_fn.cmp_string("1", "2"), true);
    }

    #[test]
    fn cmp_gt() {
        let cmp_fn = CmpGt;
        assert_eq!(cmp_fn.default(), false);
        assert_eq!(cmp_fn.cmp_bool(true, false), true);
        assert_eq!(cmp_fn.cmp_bool(true, true), false);
        assert_eq!(cmp_fn.cmp_f64(0.2, 0.1), true);
        assert_eq!(cmp_fn.cmp_f64(0.1, 0.2), false);
        assert_eq!(cmp_fn.cmp_string("a", "a"), false);
        assert_eq!(cmp_fn.cmp_string("b", "a"), true);
        assert_eq!(cmp_fn.cmp_string("1", "2"), false);
    }

    #[test]
    fn cmp_ge() {
        let cmp_fn = CmpGe;
        assert_eq!(cmp_fn.default(), false);
        assert_eq!(cmp_fn.cmp_bool(true, false), true);
        assert_eq!(cmp_fn.cmp_bool(true, true), true);
        assert_eq!(cmp_fn.cmp_f64(0.2, 0.1), true);
        assert_eq!(cmp_fn.cmp_f64(0.1, 0.1), true);
        assert_eq!(cmp_fn.cmp_f64(0.1, 0.2), false);
        assert_eq!(cmp_fn.cmp_string("1", "1"), true);
        assert_eq!(cmp_fn.cmp_string("ab", "a"), true);
        assert_eq!(cmp_fn.cmp_string("1", "2"), false);
    }

    #[test]
    fn cmp_lt() {
        let cmp_fn = CmpLt;
        assert_eq!(cmp_fn.default(), false);
        assert_eq!(cmp_fn.cmp_bool(true, false), false);
        assert_eq!(cmp_fn.cmp_bool(false, true), true);
        assert_eq!(cmp_fn.cmp_bool(true, true), false);
        assert_eq!(cmp_fn.cmp_bool(false, false), false);
        assert_eq!(cmp_fn.cmp_f64(0.1, 0.2), true);
        assert_eq!(cmp_fn.cmp_f64(0.1, 0.1), false);
        assert_eq!(cmp_fn.cmp_f64(0.2, 0.1), false);
        assert_eq!(cmp_fn.cmp_string("a", "a"), false);
        assert_eq!(cmp_fn.cmp_string("ab", "b"), true);
        assert_eq!(cmp_fn.cmp_string("1", "2"), true);
    }

    #[test]
    fn cmp_le() {
        let cmp_fn = CmpLe;
        assert_eq!(cmp_fn.default(), false);
        assert_eq!(cmp_fn.cmp_bool(true, false), false);
        assert_eq!(cmp_fn.cmp_bool(false, true), true);
        assert_eq!(cmp_fn.cmp_bool(true, true), true);
        assert_eq!(cmp_fn.cmp_bool(false, false), true);
        assert_eq!(cmp_fn.cmp_f64(0.1, 0.2), true);
        assert_eq!(cmp_fn.cmp_f64(0.1, 0.1), true);
        assert_eq!(cmp_fn.cmp_f64(0.2, 0.1), false);
        assert_eq!(cmp_fn.cmp_string("a", "a"), true);
        assert_eq!(cmp_fn.cmp_string("ab", "b"), true);
        assert_eq!(cmp_fn.cmp_string("abd", "abc"), false);
        assert_eq!(cmp_fn.cmp_string("1", "2"), true);
    }

    #[test]
    fn cmp_and() {
        let cmp_fn = CmpAnd;
        assert_eq!(cmp_fn.default(), false);
        assert_eq!(cmp_fn.cmp_bool(true, false), false);
        assert_eq!(cmp_fn.cmp_bool(false, true), false);
        assert_eq!(cmp_fn.cmp_bool(true, true), true);
        assert_eq!(cmp_fn.cmp_bool(false, false), false);
        assert_eq!(cmp_fn.cmp_f64(0.0, 0.0), true);
        assert_eq!(cmp_fn.cmp_string("a", "a"), true);
    }

    #[test]
    fn cmp_or() {
        let cmp_fn = CmpOr;
        assert_eq!(cmp_fn.default(), false);
        assert_eq!(cmp_fn.cmp_bool(true, false), true);
        assert_eq!(cmp_fn.cmp_bool(false, true), true);
        assert_eq!(cmp_fn.cmp_bool(true, true), true);
        assert_eq!(cmp_fn.cmp_bool(false, false), false);
        assert_eq!(cmp_fn.cmp_f64(0.0, 0.0), true);
        assert_eq!(cmp_fn.cmp_string("a", "a"), true);
    }

    #[test]
    fn cmp_json() {
        let v1 = Value::Bool(true);
        let v2 = Value::String("1".to_string());
        let left = [&v1, &v2];
        let right = [&v1, &v2];
        let empty: Vec<&Value> = Vec::new();

        assert_eq!(CmpEq.cmp_json(&left, &right), left.to_vec());
        assert_eq!(CmpNe.cmp_json(&left, &right), left.to_vec());
        assert_eq!(CmpGt.cmp_json(&left, &right), empty);
        assert_eq!(CmpGe.cmp_json(&left, &right), empty);
        assert_eq!(CmpLt.cmp_json(&left, &right), empty);
        assert_eq!(CmpLe.cmp_json(&left, &right), empty);
        assert_eq!(CmpAnd.cmp_json(&left, &right), left.to_vec());
        assert_eq!(CmpOr.cmp_json(&left, &right), left.to_vec());

        assert_eq!(
            CmpEq.cmp_json(&[&Value::Bool(true)], &[&Value::Bool(true)]),
            vec![&Value::Bool(true)]
        );
        assert_eq!(
            CmpEq.cmp_json(&[&Value::Bool(true)], &[&Value::Bool(false)]),
            empty
        );
        assert_eq!(
            CmpNe.cmp_json(&[&Value::Bool(true)], &[&Value::Bool(true)]),
            empty
        );
        assert_eq!(
            CmpNe.cmp_json(&[&Value::Bool(false)], &[&Value::Bool(true)]),
            vec![&Value::Bool(false)]
        );
        assert_eq!(
            CmpAnd.cmp_json(&[&Value::Bool(true)], &[&Value::Bool(true)]),
            vec![&Value::Bool(true)]
        );
        assert_eq!(
            CmpOr.cmp_json(&[&Value::Bool(true)], &[&Value::Bool(false)]),
            vec![&Value::Bool(true), &Value::Bool(false)]
        );
    }
}
